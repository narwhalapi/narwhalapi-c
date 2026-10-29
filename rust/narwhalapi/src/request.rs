use crate::{meta::Model, Client, Error};
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Method, Url,
};
use serde_json::{Map, Value};
use std::{fmt, marker::PhantomData};

const SEGMENT: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');
pub(crate) fn path(template: &str, values: &[(&str, String)]) -> Result<String, Error> {
    let mut path = template.to_owned();
    for (name, value) in values {
        if value == "." || value == ".." {
            return Err(Error::InvalidArgument(
                "path parameters must not be dot-only segments",
            ));
        }
        path = path.replace(
            &format!("{{{name}}}"),
            &percent_encoding::utf8_percent_encode(value, SEGMENT).to_string(),
        );
    }
    Ok(path)
}
#[derive(Clone)]
pub(crate) struct RequestData {
    method: &'static str,
    path: Result<String, &'static str>,
    query: Vec<(&'static str, String)>,
    body: Option<Map<String, Value>>,
    required_query: &'static [&'static str],
    required_body: &'static [&'static str],
    error: Option<&'static str>,
}
impl fmt::Debug for RequestData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RequestData(..)")
    }
}
impl RequestData {
    pub(crate) fn new(
        method: &'static str,
        template: &str,
        values: &[(&str, String)],
        required_query: &'static [&'static str],
        required_body: &'static [&'static str],
        body: bool,
    ) -> Self {
        Self {
            method,
            path: path(template, values)
                .map_err(|_| "path parameters must not be dot-only segments"),
            query: Vec::new(),
            body: body.then(Map::new),
            required_query,
            required_body,
            error: None,
        }
    }
    pub(crate) fn query(
        &mut self,
        name: &'static str,
        values: Vec<String>,
        wire: &str,
        decimal: bool,
    ) {
        self.query.retain(|(k, _)| *k != name);
        if wire != "single" && values.iter().any(|s| s.is_empty() || s.contains(',')) {
            self.error = Some("query lists require nonempty strings without commas");
            return;
        }
        if decimal && values.iter().any(|s| crate::Decimal::new(s).is_err()) {
            self.error = Some("expected a decimal wire string");
            return;
        }
        if wire == "comma_join" {
            if !values.is_empty() {
                self.query.push((name, values.join(",")));
            }
        } else {
            self.query.extend(
                values
                    .into_iter()
                    .filter(|s| name == "cursor" || !s.is_empty())
                    .map(|v| (name, v)),
            );
        }
    }
    #[allow(dead_code)] // Generated uses can be gated by optional features.
    pub(crate) fn body(&mut self, name: &str, value: Value, decimal: bool) {
        if decimal
            && value
                .as_str()
                .is_none_or(|s| crate::Decimal::new(s).is_err())
        {
            self.error = Some("expected a decimal wire string");
            return;
        }
        self.body
            .get_or_insert_with(Map::new)
            .insert(name.to_owned(), value);
    }
    pub(crate) fn cursor(&mut self, cursor: &str) {
        self.query("cursor", vec![cursor.into()], "single", false);
    }
    pub(crate) fn build(&self, base: &str, headers: &HeaderMap) -> Result<reqwest::Request, Error> {
        if let Some(e) = self.error {
            return Err(Error::InvalidArgument(e));
        }
        for name in self.required_query {
            if !self.query.iter().any(|(k, _)| k == name) {
                return Err(Error::InvalidArgument("missing required query parameter"));
            }
        }
        for name in self.required_body {
            if self
                .body
                .as_ref()
                .and_then(|b| b.get(*name))
                .is_none_or(Value::is_null)
            {
                return Err(Error::InvalidArgument("missing required body field"));
            }
        }
        let mut url = Url::parse(&format!(
            "{}{}",
            base,
            self.path.as_ref().map_err(|e| Error::InvalidArgument(e))?
        ))
        .map_err(|_| Error::InvalidArgument("invalid request URL"))?;
        if !self.query.is_empty() {
            url.query_pairs_mut().extend_pairs(&self.query);
        }
        let mut request = reqwest::Request::new(
            Method::from_bytes(self.method.as_bytes()).expect("generated HTTP method"),
            url,
        );
        *request.headers_mut() = headers.clone();
        if let Some(body) = &self.body {
            request.headers_mut().insert(
                CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
            *request.body_mut() = Some(serde_json::to_vec(body).expect("JSON body").into());
        }
        Ok(request)
    }
}
/// Shared typed request storage. Operation builders expose only applicable parameters.
#[derive(Clone)]
#[non_exhaustive]
pub struct Request<T, C = Client> {
    pub(crate) client: C,
    pub(crate) data: RequestData,
    marker: PhantomData<T>,
}
impl<T, C> fmt::Debug for Request<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Request(api_key=<redacted>)")
    }
}
impl<T, C> Request<T, C> {
    pub(crate) fn new(client: C, data: RequestData) -> Self {
        Self {
            client,
            data,
            marker: PhantomData,
        }
    }
}
impl<T: Model> Request<T> {
    pub async fn send(self) -> Result<T, Error> {
        self.client.send(self.data).await
    }
}
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
impl<T: Model> Request<T, crate::blocking::Client> {
    pub fn send(self) -> Result<T, Error> {
        self.client.send(self.data)
    }
    /// For the C library: the decoded value and the exact response bytes.
    #[doc(hidden)]
    pub fn send_keeping_body(self) -> Result<(T, Vec<u8>), Error> {
        self.client.send_keeping_body(self.data)
    }
}
