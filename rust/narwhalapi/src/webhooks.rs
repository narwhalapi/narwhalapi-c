//! Local webhook verification. Verify exact raw bytes and deduplicate by `event_id`.
use crate::{models::WebhookEvent, Error, WebhookVerificationError};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
    time::{SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq;

/// Case-insensitive headers from an HTTP framework or a plain map.
pub trait Headers {
    fn value(&self, name: &str) -> Option<String>;
}
impl Headers for http::HeaderMap {
    fn value(&self, name: &str) -> Option<String> {
        let values: Vec<_> = self
            .get_all(name)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .collect();
        (!values.is_empty()).then(|| values.join(","))
    }
}
/// A string or an ordered list of strings in a plain header map.
pub trait HeaderValues {
    fn joined(&self) -> String;
}
impl HeaderValues for str {
    fn joined(&self) -> String {
        self.to_owned()
    }
}
impl HeaderValues for String {
    fn joined(&self) -> String {
        self.clone()
    }
}
impl<T: HeaderValues + ?Sized> HeaderValues for &T {
    fn joined(&self) -> String {
        (*self).joined()
    }
}
impl<T: AsRef<str>> HeaderValues for [T] {
    fn joined(&self) -> String {
        self.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(",")
    }
}
impl<T: AsRef<str>> HeaderValues for Vec<T> {
    fn joined(&self) -> String {
        self.as_slice().joined()
    }
}
impl<T: AsRef<str>, const N: usize> HeaderValues for [T; N] {
    fn joined(&self) -> String {
        self.as_slice().joined()
    }
}
fn map_value<'a, K: AsRef<str> + 'a, V: HeaderValues + 'a>(
    entries: impl Iterator<Item = (&'a K, &'a V)>,
    name: &str,
) -> Option<String> {
    let mut matches: Vec<_> = entries
        .filter(|(key, _)| key.as_ref().eq_ignore_ascii_case(name))
        .collect();
    matches.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
    (!matches.is_empty()).then(|| {
        matches
            .into_iter()
            .map(|(_, v)| v.joined())
            .collect::<Vec<_>>()
            .join(",")
    })
}
impl<K: AsRef<str> + Eq + Hash, V: HeaderValues, S: BuildHasher> Headers for HashMap<K, V, S> {
    fn value(&self, name: &str) -> Option<String> {
        map_value(self.iter(), name)
    }
}
impl<K: AsRef<str> + Ord, V: HeaderValues> Headers for BTreeMap<K, V> {
    fn value(&self, name: &str) -> Option<String> {
        map_value(self.iter(), name)
    }
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct VerifyOptions {
    pub tolerance_seconds: u64,
    pub now: Option<u64>,
}
impl Default for VerifyOptions {
    fn default() -> Self {
        Self {
            tolerance_seconds: 300,
            now: None,
        }
    }
}
impl VerifyOptions {
    pub fn now(mut self, now: u64) -> Self {
        self.now = Some(now);
        self
    }
    pub fn tolerance_seconds(mut self, seconds: u64) -> Self {
        self.tolerance_seconds = seconds;
        self
    }
}
/// Authenticate the exact request bytes before decoding a delivery event.
pub fn verify(
    raw_body: &[u8],
    headers: &impl Headers,
    secrets: &[impl AsRef<str>],
    options: VerifyOptions,
) -> Result<WebhookEvent, Error> {
    if secrets.is_empty() || secrets.iter().any(|s| s.as_ref().is_empty()) {
        return Err(Error::InvalidArgument("webhook secrets must not be empty"));
    }
    let fail = |reason| Error::Webhook(WebhookVerificationError { reason });
    let timestamp = headers
        .value("x-narwhal-timestamp")
        .ok_or_else(|| fail("missing_header"))?;
    let signature = headers
        .value("x-narwhal-signature")
        .ok_or_else(|| fail("missing_header"))?;
    if timestamp.is_empty() || !timestamp.bytes().all(|b| b.is_ascii_digit()) {
        return Err(fail("bad_timestamp"));
    }
    let digits = timestamp.trim_start_matches('0');
    let ts = if digits.is_empty() {
        0
    } else {
        digits
            .parse::<u64>()
            .map_err(|_| fail("timestamp_out_of_range"))?
    };
    let now = options.now.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    });
    if now.abs_diff(ts) > options.tolerance_seconds {
        return Err(fail("timestamp_out_of_range"));
    }
    let mut matched = subtle::Choice::from(0);
    for secret in secrets {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_ref().as_bytes())
            .expect("HMAC accepts any key size");
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(raw_body);
        let expected = mac.finalize().into_bytes();
        for entry in signature.split(',') {
            let Some(hex) = entry.trim().strip_prefix("sha256=") else {
                continue;
            };
            if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                continue;
            }
            let mut candidate = [0u8; 32];
            for (i, byte) in candidate.iter_mut().enumerate() {
                *byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("validated hex");
            }
            matched |= expected.as_slice().ct_eq(&candidate);
        }
    }
    if !bool::from(matched) {
        return Err(fail("no_matching_signature"));
    }
    crate::meta::decode(raw_body, Default::default())
        .or_else(|_| legacy_prediction(raw_body))
        .map_err(|_| fail("invalid_body"))
}
/// Queued retries and replays keep the body they were first sent with. A prediction body sent
/// before the 22 October 2026 lean contracts names the market `event_id`.
fn legacy_prediction(raw_body: &[u8]) -> Result<WebhookEvent, Error> {
    let mut value: serde_json::Value =
        serde_json::from_slice(raw_body).map_err(|_| crate::DecodeError {
            raw_body: raw_body.to_vec(),
        })?;
    if let Some(prediction) = value.get_mut("prediction").and_then(|p| p.as_object_mut()) {
        if !prediction.contains_key("prediction_event_id") {
            if let Some(id) = prediction.get("event_id").cloned() {
                prediction.insert("prediction_event_id".into(), id);
            }
        }
    }
    let patched = serde_json::to_vec(&value).expect("a parsed value serialises");
    crate::meta::decode(&patched, Default::default())
}
