use reqwest::header::HeaderMap;
use serde::Serialize;
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RateLimit {
    pub limit: Option<i64>,
    pub remaining: Option<i64>,
    pub reset: Option<String>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ResponseMeta {
    pub request_id: Option<String>,
    pub quota: Option<RateLimit>,
    pub short: Option<RateLimit>,
}
pub(crate) fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers.get(name)?.to_str().ok().map(str::to_owned)
}
impl ResponseMeta {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Self {
        let rate = |prefix: &str| {
            let r = RateLimit {
                limit: header(headers, &format!("{prefix}limit")).and_then(|s| s.parse().ok()),
                remaining: header(headers, &format!("{prefix}remaining"))
                    .and_then(|s| s.parse().ok()),
                reset: header(headers, &format!("{prefix}reset")),
            };
            (r.limit.is_some() || r.remaining.is_some() || r.reset.is_some()).then_some(r)
        };
        Self {
            request_id: header(headers, "x-request-id"),
            quota: rate("x-ratelimit-"),
            short: rate("x-ratelimit-short-"),
        }
    }
}
/// Implemented by generated models to attach headers and retain original response bytes.
#[doc(hidden)]
pub trait Model: serde::de::DeserializeOwned + Serialize + Send + Unpin + 'static {
    fn kind(&self) -> Option<&'static str> {
        None
    }
    fn unknown_raw(&self) -> Option<&serde_json::value::RawValue> {
        None
    }
    fn attach(&mut self, _meta: ResponseMeta, _raw: &[u8]) {}
    fn meta(&self) -> Option<&ResponseMeta> {
        None
    }
}
pub(crate) fn decode<T: Model>(raw: &[u8], meta: ResponseMeta) -> Result<T, crate::Error> {
    let mut value: T = serde_json::from_slice(raw).map_err(|_| crate::DecodeError {
        raw_body: raw.to_vec(),
    })?;
    value.attach(meta, raw);
    Ok(value)
}

// A present cursor is opaque but must not be empty, even for a future page variant.
pub(crate) fn cursor<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    use serde::Deserialize;
    let value = Option::<String>::deserialize(d)?;
    if value.as_deref() == Some("") {
        return Err(serde::de::Error::custom("next_cursor must not be empty"));
    }
    Ok(value)
}
// Financial items are decimal strings; a null value means "no figure" and is left out.
pub(crate) fn decimal_items<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<std::collections::BTreeMap<String, crate::Decimal>, D::Error> {
    use serde::Deserialize;
    let all = std::collections::BTreeMap::<String, Option<crate::Decimal>>::deserialize(d)?;
    Ok(all.into_iter().filter_map(|(k, v)| Some((k, v?))).collect())
}
pub(crate) fn raw_cursor(
    value: Option<&serde_json::value::RawValue>,
) -> Result<Option<String>, serde_json::Error> {
    match value {
        None => Ok(None),
        Some(raw) => cursor(&mut serde_json::Deserializer::from_str(raw.get())),
    }
}
