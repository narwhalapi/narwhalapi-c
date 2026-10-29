//! C ABI over the blocking Rust client. `c/include/narwhalapi.h` documents the contract;
//! its ownership rules are the safety requirements of every `unsafe` export.
#![allow(non_camel_case_types, clippy::missing_safety_doc)]

use std::{
    any::Any,
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::{null, null_mut},
    time::Duration,
};

/// Read one field of a caller-allocated struct, or `None` when the caller's struct (sized
/// by its `size` field) predates it. Uses raw places only: no reference to the struct is made.
macro_rules! field {
    ($p:expr, $size:expr, $T:ty, $f:ident, $F:ty) => {
        if $size >= ::std::mem::offset_of!($T, $f) + ::std::mem::size_of::<$F>() {
            Some(::std::ptr::addr_of!((*$p).$f).read())
        } else {
            None
        }
    };
}

mod generated;
pub use generated::*;
#[cfg(test)]
mod tests;

/// Result of every export that can fail.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum nw_status {
    NW_OK = 0,
    NW_ERR_API,
    NW_ERR_TRANSPORT,
    NW_ERR_DECODE,
    NW_ERR_INVALID_ARGUMENT,
    NW_ERR_WEBHOOK,
    NW_ERR_INTERNAL,
}

/// Opaque, thread-safe client. Create with `nw_client_new`, free with `nw_client_free`.
pub struct nw_client {
    inner: sdk::blocking::Client,
}

/// Opaque error. Read it with the `nw_error_*` accessors and free it with `nw_error_free`.
pub struct nw_error {
    http_status: c_int,
    code: Option<CString>,
    title: Option<CString>,
    detail: Option<CString>,
    request_id: Option<CString>,
    instance: Option<CString>,
    message: CString,
    retry_after: i64,
    raw_body: Vec<u8>,
}

/// One extra request header. Authorization cannot be overridden. Its layout is frozen
/// because it is passed by value in arrays.
#[repr(C)]
pub struct nw_header {
    pub name: *const c_char,
    pub value: *const c_char,
}

/// Client configuration. Zero-initialize, set `size` to `sizeof(nw_config)`, then set fields;
/// a NULL `config` uses every default.
/// `api_key` NULL reads `NARWHAL_API_KEY`; `base_url` NULL reads `NARWHAL_BASE_URL`, else
/// `https://api.narwhalapi.com`. Unset `has_` flags keep the defaults: a 30 s timeout per
/// attempt, 2 retries and a 30 s maximum `Retry-After` wait.
#[repr(C)]
pub struct nw_config {
    pub size: usize,
    pub api_key: *const c_char,
    pub base_url: *const c_char,
    pub has_timeout_ms: bool,
    pub timeout_ms: u64,
    pub has_max_retries: bool,
    pub max_retries: u32,
    pub has_max_retry_wait_ms: bool,
    pub max_retry_wait_ms: u64,
    pub extra_headers: *const nw_header,
    pub extra_headers_len: usize,
}

/// One rate-limit window from response headers. `reset` is ISO-8601 UTC text, or NULL.
#[repr(C)]
pub struct nw_rate_limit {
    pub has_limit: bool,
    pub limit: i64,
    pub has_remaining: bool,
    pub remaining: i64,
    pub reset: *const c_char,
}

/// Header metadata of a top-level response. Absent parts are NULL. `short_limit` is the
/// short window (`X-RateLimit-Short-*`); `quota` is the plan quota (`X-RateLimit-*`).
#[repr(C)]
pub struct nw_response_meta {
    pub request_id: *const c_char,
    pub quota: *const nw_rate_limit,
    pub short_limit: *const nw_rate_limit,
}
/// A decoded value the C mirror cannot hold.
pub(crate) enum Unrepresentable {
    /// A string contains U+0000, which a NUL-terminated C string cannot hold.
    Nul,
    /// A union variant this library version has no C mirror for.
    Variant,
}

/// Owned storage behind one response: its C strings and mirror arrays.
#[derive(Default)]
pub(crate) struct Store {
    strings: Vec<CString>,
    blocks: Vec<Box<dyn Any>>,
}
impl Store {
    pub(crate) fn str(&mut self, value: &str) -> Result<*const c_char, Unrepresentable> {
        let value = CString::new(value).map_err(|_| Unrepresentable::Nul)?;
        let pointer = value.as_ptr();
        self.strings.push(value);
        Ok(pointer)
    }
    pub(crate) fn boxed<T: 'static>(&mut self, value: T) -> *const T {
        let value = Box::new(value);
        let pointer: *const T = &*value;
        self.blocks.push(value);
        pointer
    }
    pub(crate) fn array<T: 'static>(&mut self, items: Vec<T>) -> (*const T, usize) {
        let (pointer, len) = (items.as_ptr(), items.len());
        self.blocks.push(Box::new(items));
        (pointer, len)
    }
    fn rate(
        &mut self,
        rate: Option<&sdk::RateLimit>,
    ) -> Result<*const nw_rate_limit, Unrepresentable> {
        let Some(rate) = rate else {
            return Ok(null());
        };
        let reset = match &rate.reset {
            Some(reset) => self.str(reset)?,
            None => null(),
        };
        Ok(self.boxed(nw_rate_limit {
            has_limit: rate.limit.is_some(),
            limit: rate.limit.unwrap_or_default(),
            has_remaining: rate.remaining.is_some(),
            remaining: rate.remaining.unwrap_or_default(),
            reset,
        }))
    }
    pub(crate) fn meta(
        &mut self,
        meta: &sdk::ResponseMeta,
    ) -> Result<nw_response_meta, Unrepresentable> {
        Ok(nw_response_meta {
            request_id: match &meta.request_id {
                Some(id) => self.str(id)?,
                None => null(),
            },
            quota: self.rate(meta.quota.as_ref())?,
            short_limit: self.rate(meta.short.as_ref())?,
        })
    }
}

/// One response: the `#[repr(C)]` head first, so a head pointer is the arena pointer.
#[repr(C)]
struct Owned<H> {
    head: H,
    store: Store,
}

pub(crate) enum Failure {
    Sdk(sdk::Error),
    /// The value decoded, but the mirror failed; keeps the original response bytes.
    Unrepresentable(Unrepresentable, Vec<u8>),
}
impl From<sdk::Error> for Failure {
    fn from(error: sdk::Error) -> Self {
        Self::Sdk(error)
    }
}
fn invalid(reason: &'static str) -> Failure {
    Failure::Sdk(sdk::Error::InvalidArgument(reason))
}

#[cfg(nw_c_tests)]
static PANIC_HOOK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Test-only: while enabled, every export panics on entry. Not part of the header.
#[cfg(nw_c_tests)]
#[no_mangle]
pub extern "C" fn nw_test_panic_hook(enabled: bool) {
    PANIC_HOOK.store(enabled, std::sync::atomic::Ordering::SeqCst);
}

#[inline]
fn test_hook() {
    #[cfg(nw_c_tests)]
    if PANIC_HOOK.load(std::sync::atomic::Ordering::SeqCst) {
        // resume_unwind skips the panic message; the unwind is still a real panic.
        std::panic::resume_unwind(Box::new("test panic hook"));
    }
}

/// Run a non-status export: panics and NULL inputs produce `fallback`.
fn guard<T>(fallback: T, run: impl FnOnce() -> Option<T>) -> T {
    catch_unwind(AssertUnwindSafe(|| {
        test_hook();
        run()
    }))
    .ok()
    .flatten()
    .unwrap_or(fallback)
}

fn text(value: &str) -> CString {
    // Error text is diagnostic; an embedded NUL becomes U+FFFD. Raw bytes stay exact.
    CString::new(value.replace('\0', "\u{fffd}")).expect("NUL-free text")
}

fn error_of(failure: Option<Failure>) -> (nw_status, nw_error) {
    let mut error = nw_error {
        http_status: 0,
        code: None,
        title: None,
        detail: None,
        request_id: None,
        instance: None,
        message: text("Internal error: a panic was caught at the C boundary"),
        retry_after: -1,
        raw_body: Vec::new(),
    };
    let Some(failure) = failure else {
        return (nw_status::NW_ERR_INTERNAL, error);
    };
    let error_text = match &failure {
        Failure::Sdk(e) => e.to_string(),
        Failure::Unrepresentable(Unrepresentable::Nul, _) => {
            "Response did not match the contract: a string contains U+0000, which a C string \
             cannot hold"
                .into()
        }
        Failure::Unrepresentable(Unrepresentable::Variant, _) => {
            "Response has a union variant this library version cannot represent".into()
        }
    };
    error.message = text(&error_text);
    let status = match failure {
        Failure::Unrepresentable(_, raw_body) => {
            error.raw_body = raw_body;
            nw_status::NW_ERR_DECODE
        }
        Failure::Sdk(sdk::Error::Api(e)) => {
            error.http_status = c_int::from(e.status);
            error.code = Some(text(&e.code));
            error.title = Some(text(&e.title));
            error.detail = Some(text(&e.detail));
            error.request_id = e.request_id.as_deref().map(text);
            error.instance = e.instance.as_deref().map(text);
            error.retry_after = e
                .retry_after
                .map_or(-1, |s| i64::try_from(s).unwrap_or(i64::MAX));
            error.raw_body = e.raw_body;
            nw_status::NW_ERR_API
        }
        Failure::Sdk(sdk::Error::Transport(_)) => nw_status::NW_ERR_TRANSPORT,
        Failure::Sdk(sdk::Error::Decode(e)) => {
            error.raw_body = e.raw_body;
            nw_status::NW_ERR_DECODE
        }
        Failure::Sdk(sdk::Error::InvalidArgument(_)) => nw_status::NW_ERR_INVALID_ARGUMENT,
        Failure::Sdk(sdk::Error::Webhook(e)) => {
            error.code = Some(text(e.reason));
            nw_status::NW_ERR_WEBHOOK
        }
        Failure::Sdk(_) => nw_status::NW_ERR_INTERNAL,
    };
    (status, error)
}

/// Run a status export that hands the caller one owned pointer.
unsafe fn status<T>(
    out: *mut *mut T,
    err: *mut *mut nw_error,
    run: impl FnOnce() -> Result<*mut T, Failure>,
) -> nw_status {
    if !err.is_null() {
        *err = null_mut();
    }
    if !out.is_null() {
        *out = null_mut();
    }
    let result = catch_unwind(AssertUnwindSafe(|| {
        test_hook();
        if out.is_null() {
            return Err(invalid("out must not be NULL"));
        }
        run()
    }));
    let failure = match result {
        Ok(Ok(value)) => {
            *out = value;
            return nw_status::NW_OK;
        }
        Ok(Err(failure)) => Some(failure),
        Err(_) => None,
    };
    let reported = catch_unwind(AssertUnwindSafe(|| {
        let (code, error) = error_of(failure);
        if !err.is_null() {
            *err = Box::into_raw(Box::new(error));
        }
        code
    }));
    reported.unwrap_or(nw_status::NW_ERR_INTERNAL)
}

/// Run an operation and mirror its value into one owned response.
pub(crate) unsafe fn respond<V, H>(
    out: *mut *mut H,
    err: *mut *mut nw_error,
    run: impl FnOnce() -> Result<(V, Vec<u8>), Failure>,
    mirror: fn(&V, &mut Store) -> Result<H, Unrepresentable>,
) -> nw_status {
    status(out, err, || {
        let (value, raw_body) = run()?;
        let mut store = Store::default();
        let head = mirror(&value, &mut store).map_err(|e| Failure::Unrepresentable(e, raw_body))?;
        Ok(Box::into_raw(Box::new(Owned { head, store })).cast::<H>())
    })
}

/// Free a response head created by `respond`.
pub(crate) unsafe fn release<H>(value: *mut H) {
    guard((), || {
        if !value.is_null() {
            drop(Box::from_raw(value.cast::<Owned<H>>()));
        }
        Some(())
    })
}

/// The `size` of a non-NULL caller-allocated struct; 0 means it was never set.
pub(crate) unsafe fn params_size<T>(params: *const T) -> Result<usize, Failure> {
    match params.cast::<usize>().read() {
        0 => Err(invalid("size must be set to sizeof the struct")),
        size => Ok(size),
    }
}

pub(crate) unsafe fn client_ref<'a>(
    client: *mut nw_client,
) -> Result<&'a sdk::blocking::Client, Failure> {
    client
        .as_ref()
        .map(|c| &c.inner)
        .ok_or_else(|| invalid("client must not be NULL"))
}

pub(crate) unsafe fn opt<'a>(value: *const c_char) -> Result<Option<&'a str>, Failure> {
    if value.is_null() {
        return Ok(None);
    }
    CStr::from_ptr(value)
        .to_str()
        .map(Some)
        .map_err(|_| invalid("strings must be UTF-8"))
}

pub(crate) unsafe fn arg<'a>(value: *const c_char) -> Result<&'a str, Failure> {
    opt(value)?.ok_or_else(|| invalid("required argument must not be NULL"))
}

pub(crate) unsafe fn opt_list<'a>(
    items: *const *const c_char,
    len: usize,
) -> Result<Option<Vec<&'a str>>, Failure> {
    if items.is_null() {
        return if len == 0 {
            Ok(None)
        } else {
            Err(invalid("array must not be NULL when its length is nonzero"))
        };
    }
    std::slice::from_raw_parts(items, len)
        .iter()
        .map(|item| arg(*item))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(crate) unsafe fn list<'a>(
    items: *const *const c_char,
    len: usize,
) -> Result<Vec<&'a str>, Failure> {
    Ok(opt_list(items, len)?.unwrap_or_default())
}

unsafe fn bytes<'a>(data: *const u8, len: usize) -> Result<&'a [u8], Failure> {
    match (data.is_null(), len) {
        (true, 0) => Ok(&[]),
        (true, _) => Err(invalid(
            "buffer must not be NULL when its length is nonzero",
        )),
        (false, _) => Ok(std::slice::from_raw_parts(data, len)),
    }
}

/// Create a client. `config` may be NULL. Free the client with `nw_client_free`.
#[no_mangle]
pub unsafe extern "C" fn nw_client_new(
    config: *const nw_config,
    out: *mut *mut nw_client,
    err: *mut *mut nw_error,
) -> nw_status {
    status(out, err, || {
        let c = config;
        let size = if c.is_null() { 0 } else { params_size(c)? };
        let read_str = |value: Option<*const c_char>| opt(value.unwrap_or(null()));
        let key = match read_str(field!(c, size, nw_config, api_key, *const c_char))? {
            Some(key) => key.to_owned(),
            None => std::env::var("NARWHAL_API_KEY").unwrap_or_default(),
        };
        let mut builder = sdk::blocking::Client::builder(key);
        if !c.is_null() {
            if let Some(url) = read_str(field!(c, size, nw_config, base_url, *const c_char))? {
                builder = builder.base_url(url);
            }
            if field!(c, size, nw_config, has_timeout_ms, bool) == Some(true) {
                if let Some(ms) = field!(c, size, nw_config, timeout_ms, u64) {
                    builder = builder.timeout(Duration::from_millis(ms));
                }
            }
            if field!(c, size, nw_config, has_max_retries, bool) == Some(true) {
                if let Some(retries) = field!(c, size, nw_config, max_retries, u32) {
                    builder = builder.max_retries(retries);
                }
            }
            if field!(c, size, nw_config, has_max_retry_wait_ms, bool) == Some(true) {
                if let Some(ms) = field!(c, size, nw_config, max_retry_wait_ms, u64) {
                    builder = builder.max_retry_wait(Duration::from_millis(ms));
                }
            }
            let headers =
                field!(c, size, nw_config, extra_headers, *const nw_header).unwrap_or(null());
            let len = field!(c, size, nw_config, extra_headers_len, usize).unwrap_or(0);
            let headers = if headers.is_null() {
                if len != 0 {
                    return Err(invalid("extra_headers must not be NULL"));
                }
                &[][..]
            } else {
                std::slice::from_raw_parts(headers, len)
            };
            let headers = headers
                .iter()
                .map(|h| Ok((arg(h.name)?, arg(h.value)?)))
                .collect::<Result<Vec<_>, Failure>>()?;
            builder = builder.extra_headers(headers);
        }
        let inner = builder.build_blocking()?;
        Ok(Box::into_raw(Box::new(nw_client { inner })))
    })
}

/// Free a client after every call on it has returned. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_client_free(client: *mut nw_client) {
    guard((), || {
        if !client.is_null() {
            drop(Box::from_raw(client));
        }
        Some(())
    })
}

/// Free an error. NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn nw_error_free(error: *mut nw_error) {
    guard((), || {
        if !error.is_null() {
            drop(Box::from_raw(error));
        }
        Some(())
    })
}

unsafe fn error_text(
    error: *const nw_error,
    field: impl FnOnce(&nw_error) -> Option<&CString>,
) -> *const c_char {
    guard(null(), || field(error.as_ref()?).map(|s| s.as_ptr()))
}

/// HTTP status of an API error; 0 for other errors.
#[no_mangle]
pub unsafe extern "C" fn nw_error_status(error: *const nw_error) -> c_int {
    guard(0, || Some(error.as_ref()?.http_status))
}

/// API problem code, or the webhook verification reason; NULL for other errors.
#[no_mangle]
pub unsafe extern "C" fn nw_error_code(error: *const nw_error) -> *const c_char {
    error_text(error, |e| e.code.as_ref())
}

/// API problem title; NULL for other errors.
#[no_mangle]
pub unsafe extern "C" fn nw_error_title(error: *const nw_error) -> *const c_char {
    error_text(error, |e| e.title.as_ref())
}

/// API problem detail; NULL for other errors.
#[no_mangle]
pub unsafe extern "C" fn nw_error_detail(error: *const nw_error) -> *const c_char {
    error_text(error, |e| e.detail.as_ref())
}

/// Request ID of an API error; NULL when absent or empty.
#[no_mangle]
pub unsafe extern "C" fn nw_error_request_id(error: *const nw_error) -> *const c_char {
    error_text(error, |e| e.request_id.as_ref())
}

/// Problem instance of an API error; NULL when absent.
#[no_mangle]
pub unsafe extern "C" fn nw_error_instance(error: *const nw_error) -> *const c_char {
    error_text(error, |e| e.instance.as_ref())
}

/// Error text. API errors read `HTTP <status> <code>: <title>`, plus
/// ` (request_id <id>)` only when a request ID is present.
#[no_mangle]
pub unsafe extern "C" fn nw_error_message(error: *const nw_error) -> *const c_char {
    error_text(error, |e| Some(&e.message))
}

/// `Retry-After` seconds of an API error; -1 if absent.
#[no_mangle]
pub unsafe extern "C" fn nw_error_retry_after(error: *const nw_error) -> i64 {
    guard(-1, || Some(error.as_ref()?.retry_after))
}

/// Original response bytes of an API or decode error, owned by the error. Sets `*len`
/// when `len` is non-NULL. NULL with length 0 when there is no body.
#[no_mangle]
pub unsafe extern "C" fn nw_error_raw_body(error: *const nw_error, len: *mut usize) -> *const u8 {
    if !len.is_null() {
        *len = 0;
    }
    guard(null(), || {
        let body = &error.as_ref()?.raw_body;
        if body.is_empty() {
            return None;
        }
        if !len.is_null() {
            *len = body.len();
        }
        Some(body.as_ptr())
    })
}

static VERSION: &CStr =
    match CStr::from_bytes_with_nul(concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes()) {
        Ok(version) => version,
        Err(_) => panic!("version contains NUL"),
    };

/// Library version, a static string. Never NULL.
#[no_mangle]
pub extern "C" fn nw_version() -> *const c_char {
    guard(VERSION.as_ptr(), || Some(VERSION.as_ptr()))
}

/// Whether this library can serve a program compiled against header version
/// `major.minor`: pass `NW_VERSION_MAJOR` and `NW_VERSION_MINOR`. Before 1.0 the minor
/// version must match (a minor release may break); from 1.0 the major must match and the
/// library minor must be at least `minor`. Returns false on panic.
#[no_mangle]
pub extern "C" fn nw_version_check(major: u32, minor: u32) -> bool {
    guard(false, || {
        Some(if NW_VERSION_MAJOR == 0 {
            major == 0 && minor == NW_VERSION_MINOR
        } else {
            major == NW_VERSION_MAJOR && minor <= NW_VERSION_MINOR
        })
    })
}

/// Verify a webhook delivery over the exact raw body bytes, then decode it.
/// A NULL header is missing (`missing_header`). `tolerance_seconds` must be >= 0;
/// `now_unix` 0 uses the system clock. Verification failures return `NW_ERR_WEBHOOK`
/// with the reason as `nw_error_code`. Free the event with `nw_webhook_event_free`.
#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn nw_webhook_verify(
    body: *const u8,
    body_len: usize,
    timestamp_header: *const c_char,
    signature_header: *const c_char,
    secrets: *const *const c_char,
    secrets_len: usize,
    tolerance_seconds: i64,
    now_unix: i64,
    out: *mut *mut nw_webhook_event,
    err: *mut *mut nw_error,
) -> nw_status {
    let run = || {
        let body = bytes(body, body_len)?;
        let mut headers = std::collections::BTreeMap::new();
        if let Some(value) = opt(timestamp_header)? {
            headers.insert("x-narwhal-timestamp", value);
        }
        if let Some(value) = opt(signature_header)? {
            headers.insert("x-narwhal-signature", value);
        }
        let secrets = opt_list(secrets, secrets_len)?.unwrap_or_default();
        let tolerance = u64::try_from(tolerance_seconds)
            .map_err(|_| invalid("tolerance_seconds must not be negative"))?;
        let mut options = sdk::webhooks::VerifyOptions::default().tolerance_seconds(tolerance);
        match u64::try_from(now_unix) {
            Ok(0) => {}
            Ok(now) => options = options.now(now),
            Err(_) => return Err(invalid("now_unix must not be negative")),
        }
        let event = sdk::webhooks::verify(body, &headers, secrets.as_slice(), options)?;
        Ok((event, body.to_vec()))
    };
    respond(out, err, run, generated::mirror_webhook_event)
}
