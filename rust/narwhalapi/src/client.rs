use crate::{
    meta::{decode, header, Model},
    request::RequestData,
    transport::{Clock, HttpTransport, SystemClock, Transport},
    ApiError, Error, ResponseMeta, TransportError,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Method, Url,
};
use std::{fmt, sync::Arc, time::Duration};

#[derive(Clone)]
#[non_exhaustive]
pub struct ClientBuilder {
    key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
    max_retry_wait: Duration,
    headers: HeaderMap,
    invalid_headers: bool,
    #[cfg(feature = "native-roots")]
    native_certificates: Option<Arc<Vec<reqwest::Certificate>>>,
    transport: Option<Arc<dyn Transport>>,
    clock: Arc<dyn Clock>,
}
impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClientBuilder(api_key=<redacted>)")
    }
}
impl ClientBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            key: api_key.into(),
            base_url: std::env::var("NARWHAL_BASE_URL")
                .unwrap_or_else(|_| "https://api.narwhalapi.com".into()),
            timeout: Duration::from_secs(30),
            max_retries: 2,
            max_retry_wait: Duration::from_secs(30),
            headers: HeaderMap::new(),
            invalid_headers: false,
            #[cfg(feature = "native-roots")]
            native_certificates: None,
            transport: None,
            clock: Arc::new(SystemClock),
        }
    }
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    pub fn max_retry_wait(mut self, wait: Duration) -> Self {
        self.max_retry_wait = wait;
        self
    }
    /// Replace extra headers. Invalid names or values fail at client construction.
    pub fn extra_headers<K: Into<String>, V: Into<String>>(
        mut self,
        headers: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.headers.clear();
        self.invalid_headers = false;
        for (key, value) in headers {
            match (
                HeaderName::from_bytes(key.into().as_bytes()),
                HeaderValue::from_str(&value.into()),
            ) {
                (Ok(key), Ok(value)) => {
                    self.headers.append(key, value);
                }
                _ => self.invalid_headers = true,
            }
        }
        self
    }
    #[doc(hidden)]
    pub fn transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }
    #[doc(hidden)]
    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }
    pub(crate) fn validate(&mut self) -> Result<(), Error> {
        if self.invalid_headers {
            return Err(Error::InvalidArgument("invalid extra headers"));
        }
        if self.key.is_empty() {
            return Err(Error::InvalidArgument("NARWHAL_API_KEY is required"));
        }
        self.base_url = self.base_url.trim_end_matches('/').to_owned();
        let url =
            Url::parse(&self.base_url).map_err(|_| Error::InvalidArgument("invalid base URL"))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::InvalidArgument("invalid base URL"));
        }
        if self.timeout.is_zero()
            || tokio::time::Instant::now()
                .checked_add(self.timeout)
                .is_none()
        {
            return Err(Error::InvalidArgument("invalid timeout"));
        }
        let mut key = HeaderValue::from_str(&format!("Bearer {}", self.key))
            .map_err(|_| Error::InvalidArgument("invalid API key"))?;
        key.set_sensitive(true);
        self.headers.insert(AUTHORIZATION, key);
        self.headers
            .entry(ACCEPT)
            .or_insert(HeaderValue::from_static(
                "application/json, application/problem+json",
            ));
        self.headers.entry(USER_AGENT).or_insert(
            HeaderValue::from_str(&format!(
                "narwhalapi-rust/{} (spec {})",
                crate::VERSION,
                &crate::generated::LIVE_SPEC_SHA[..8]
            ))
            .expect("static User-Agent"),
        );
        #[cfg(feature = "native-roots")]
        if self.transport.is_none() && self.native_certificates.is_none() {
            // Load while constructing the client, before a possible fork. Keep only
            // owned certificate bytes; rebuilding a pool must not re-enter OS stores.
            let loaded = rustls_native_certs::load_native_certs();
            let mut store = rustls::RootCertStore::empty();
            let mut certificates = Vec::new();
            let mut invalid = 0;
            for cert in loaded.certs {
                if store.add(cert.clone()).is_err() {
                    invalid += 1;
                    continue;
                }
                let cert = reqwest::Certificate::from_der(cert.as_ref())
                    .map_err(|e| TransportError::new(e.without_url(), false))?;
                certificates.push(cert);
            }
            if certificates.is_empty() && invalid > 0 {
                return Err(TransportError::new(
                    std::io::Error::other("zero valid certificates in native root store"),
                    false,
                )
                .into());
            }
            self.native_certificates = Some(Arc::new(certificates));
        }
        Ok(())
    }
    pub fn build(mut self) -> Result<Client, Error> {
        self.validate()?;
        let transport = match &self.transport {
            Some(t) => t.clone(),
            None => {
                let http = reqwest::Client::builder();
                #[cfg(feature = "native-roots")]
                let http = {
                    let mut http = http.tls_built_in_native_certs(false);
                    for cert in self
                        .native_certificates
                        .as_ref()
                        .expect("loaded certificates")
                        .iter()
                    {
                        http = http.add_root_certificate(cert.clone());
                    }
                    http
                };
                Arc::new(HttpTransport(
                    http.connect_timeout(Duration::from_secs(10))
                        .redirect(reqwest::redirect::Policy::none())
                        .build()
                        .map_err(|e| TransportError::new(e.without_url(), false))?,
                ))
            }
        };
        Ok(Client {
            config: Arc::new(self),
            transport,
        })
    }
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn build_blocking(mut self) -> Result<crate::blocking::Client, Error> {
        self.validate()?;
        Ok(crate::blocking::Client::from_builder(self))
    }
    #[allow(dead_code)] // Generated uses can be gated by optional features.
    pub(crate) fn feed_url(&self, path: &str, name: &str, value: &str) -> Result<String, Error> {
        Ok(format!(
            "{}{}",
            self.base_url,
            crate::request::path(path, &[(name, value.to_owned())])?
        ))
    }
}
/// Cloneable, thread-safe async client. Uses the caller's Tokio runtime.
#[derive(Clone)]
#[non_exhaustive]
pub struct Client {
    config: Arc<ClientBuilder>,
    transport: Arc<dyn Transport>,
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Client(api_key=<redacted>)")
    }
}
impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl Client {
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::builder(api_key).build()
    }
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }
    pub fn from_env() -> Result<Self, Error> {
        Self::new(std::env::var("NARWHAL_API_KEY").unwrap_or_default())
    }
    #[allow(dead_code)] // Generated uses can be gated by optional features.
    pub(crate) fn feed_url(&self, path: &str, name: &str, value: &str) -> Result<String, Error> {
        self.config.feed_url(path, name, value)
    }
    pub(crate) async fn send<T: Model>(&self, data: RequestData) -> Result<T, Error> {
        self.send_keeping_body(data).await.map(|(value, _)| value)
    }
    /// Decode a success and also return the exact bytes it was decoded from.
    pub(crate) async fn send_keeping_body<T: Model>(
        &self,
        data: RequestData,
    ) -> Result<(T, Vec<u8>), Error> {
        let request = data.build(&self.config.base_url, &self.config.headers)?;
        for attempt in 0..=self.config.max_retries {
            let result = self
                .attempt(request.try_clone().expect("buffered request"))
                .await;
            let failure = match result {
                Ok((status, headers, raw)) if (200..300).contains(&status) => {
                    // A 204 answer has no body; it reads as an empty object.
                    let value = if status == 204 && raw.is_empty() {
                        decode(b"{}", ResponseMeta::from_headers(&headers)).map(|mut v: T| {
                            v.attach(ResponseMeta::from_headers(&headers), &raw);
                            v
                        })
                    } else {
                        decode(&raw, ResponseMeta::from_headers(&headers))
                    }?;
                    return Ok((value, raw));
                }
                Ok((status, headers, raw)) => {
                    Error::Api(Box::new(api_error(status, &headers, raw)))
                }
                Err(error) => Error::Transport(error),
            };
            let wait = if attempt < self.config.max_retries {
                self.retry_wait(request.method(), attempt, &failure)
            } else {
                None
            };
            match wait {
                Some(wait) => self.config.clock.sleep(wait).await,
                None => return Err(failure),
            }
        }
        unreachable!("bounded retry loop always returns")
    }
    async fn attempt(
        &self,
        request: reqwest::Request,
    ) -> Result<(u16, HeaderMap, Vec<u8>), TransportError> {
        let deadline = self.config.clock.now() + self.config.timeout;
        let mut received = false;
        let result = tokio::time::timeout_at(deadline, async {
            let response = self.transport.execute(request).await?;
            received = true;
            let body = response.body.await.map_err(|mut e| {
                e.response_received = true;
                e
            })?;
            Ok((response.status, response.headers, body))
        })
        .await;
        result.unwrap_or_else(|e| Err(TransportError::new(e, received)))
    }
    fn retry_wait(&self, method: &Method, attempt: u32, error: &Error) -> Option<Duration> {
        match error {
            Error::Api(p) => {
                if !(p.status == 429 && p.code == crate::codes::RATE_LIMIT_EXCEEDED
                    || p.status == 503 && method == Method::GET)
                {
                    return None;
                }
                if let Some(seconds) = p.retry_after {
                    let wait = Duration::from_secs(seconds);
                    return (wait <= self.config.max_retry_wait).then_some(wait);
                }
            }
            Error::Transport(t) if method == Method::GET && !t.response_received => {}
            _ => return None,
        }
        let ceiling = Duration::from_millis(500)
            .checked_mul(2_u32.checked_pow(attempt).unwrap_or(u32::MAX))
            .unwrap_or(Duration::MAX);
        Some(self.config.clock.jitter(ceiling).min(ceiling))
    }
}
fn api_error(status: u16, headers: &HeaderMap, raw_body: Vec<u8>) -> ApiError {
    let data: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_slice(&raw_body).unwrap_or_default();
    let string = |key: &str| {
        data.get(key)
            .and_then(|value| serde_json::from_str::<String>(value.get()).ok())
    };
    ApiError {
        status,
        code: string("code").unwrap_or_else(|| "unknown".into()),
        title: string("title").unwrap_or_default(),
        detail: string("detail").unwrap_or_default(),
        request_id: header(headers, "x-request-id")
            .filter(|s| !s.is_empty())
            .or_else(|| string("request_id").filter(|s| !s.is_empty())),
        instance: string("instance"),
        retry_after: header(headers, "retry-after").and_then(|s| s.parse().ok()),
        raw_body,
    }
}
