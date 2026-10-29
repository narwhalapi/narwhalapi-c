use std::fmt;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Api(#[from] Box<ApiError>),
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error("Invalid argument: {0}")]
    InvalidArgument(&'static str),
    #[error(transparent)]
    Webhook(#[from] WebhookVerificationError),
    /// A failure inside the SDK itself, never caused by arguments or the server.
    #[error("Internal error: {0}")]
    Internal(&'static str),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApiErrorKind {
    ApiError,
    BadRequest,
    Unauthorized,
    NotFound,
    ServiceUnavailable,
    RateLimited,
    CursorExpired,
}
#[derive(Clone)]
#[non_exhaustive]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub title: String,
    pub detail: String,
    pub request_id: Option<String>,
    pub instance: Option<String>,
    pub retry_after: Option<u64>,
    pub raw_body: Vec<u8>,
}
impl ApiError {
    pub fn kind(&self) -> ApiErrorKind {
        if self.code == crate::codes::CURSOR_EXPIRED {
            return ApiErrorKind::CursorExpired;
        }
        match self.status {
            400 => ApiErrorKind::BadRequest,
            401 => ApiErrorKind::Unauthorized,
            404 => ApiErrorKind::NotFound,
            429 => ApiErrorKind::RateLimited,
            503 => ApiErrorKind::ServiceUnavailable,
            _ => ApiErrorKind::ApiError,
        }
    }
    pub fn is_quota_exhausted(&self) -> bool {
        self.code == crate::codes::QUOTA_EXHAUSTED
    }
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {} {}: {}", self.status, self.code, self.title)?;
        if let Some(id) = self.request_id.as_ref().filter(|s| !s.is_empty()) {
            write!(f, " (request_id {id})")?;
        }
        Ok(())
    }
}
impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl std::error::Error for ApiError {}
#[derive(thiserror::Error)]
#[error("Request transport failed ({})", self.cause_kind())]
#[non_exhaustive]
pub struct TransportError {
    #[source]
    pub cause: Box<dyn std::error::Error + Send + Sync>,
    pub response_received: bool,
}
impl TransportError {
    fn has_cause(&self, predicate: impl Fn(&(dyn std::error::Error + 'static)) -> bool) -> bool {
        let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(self.cause.as_ref());
        while let Some(current) = cause {
            if predicate(current) {
                return true;
            }
            cause = current.source();
        }
        false
    }
    /// Whether a connection or total-attempt deadline expired.
    pub fn is_timeout(&self) -> bool {
        self.has_cause(|e| {
            e.is::<tokio::time::error::Elapsed>()
                || e.downcast_ref::<reqwest::Error>()
                    .is_some_and(reqwest::Error::is_timeout)
                || e.downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::TimedOut)
        })
    }
    /// Whether the transport failed to establish a connection (including DNS or TLS).
    pub fn is_connect(&self) -> bool {
        self.has_cause(|e| {
            e.downcast_ref::<reqwest::Error>()
                .is_some_and(reqwest::Error::is_connect)
                || (!self.response_received
                    && e.downcast_ref::<std::io::Error>().is_some_and(|e| {
                        matches!(
                            e.kind(),
                            std::io::ErrorKind::ConnectionRefused
                                | std::io::ErrorKind::NotConnected
                                | std::io::ErrorKind::AddrNotAvailable
                        )
                    }))
        })
    }
    fn cause_kind(&self) -> &'static str {
        if self.is_timeout() {
            "timeout"
        } else if self.is_connect() {
            "connect"
        } else {
            "other"
        }
    }
    pub fn new(
        cause: impl std::error::Error + Send + Sync + 'static,
        response_received: bool,
    ) -> Self {
        Self {
            cause: Box::new(cause),
            response_received,
        }
    }
}
impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("response_received", &self.response_received)
            .finish_non_exhaustive()
    }
}
#[derive(thiserror::Error)]
#[error("Response did not match the contract")]
#[non_exhaustive]
pub struct DecodeError {
    pub raw_body: Vec<u8>,
}
impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DecodeError")
    }
}
#[derive(Debug, thiserror::Error)]
#[error("{reason}")]
#[non_exhaustive]
pub struct WebhookVerificationError {
    pub reason: &'static str,
}
