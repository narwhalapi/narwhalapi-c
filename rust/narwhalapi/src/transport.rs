//! Optional transport and clock injection. The default transport uses reqwest on the caller's runtime.
use crate::TransportError;
use reqwest::header::HeaderMap;
use std::{future::Future, pin::Pin, time::Duration};

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
/// Headers arrive before the body; the client deadline includes both futures.
pub struct Response {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: BoxFuture<Result<Vec<u8>, TransportError>>,
}
pub trait Transport: Send + Sync {
    fn execute(&self, request: reqwest::Request) -> BoxFuture<Result<Response, TransportError>>;
}
#[derive(Clone)]
pub struct HttpTransport(pub reqwest::Client);
impl Transport for HttpTransport {
    fn execute(&self, request: reqwest::Request) -> BoxFuture<Result<Response, TransportError>> {
        let http = self.0.clone();
        Box::pin(async move {
            let response = http
                .execute(request)
                .await
                .map_err(|e| TransportError::new(e.without_url(), false))?;
            Ok(Response {
                status: response.status().as_u16(),
                headers: response.headers().clone(),
                body: Box::pin(async move {
                    response
                        .bytes()
                        .await
                        .map(|b| b.to_vec())
                        .map_err(|e| TransportError::new(e.without_url(), true))
                }),
            })
        })
    }
}
/// Monotonic time, retry sleeping and full jitter. Injectable for deterministic tests.
pub trait Clock: Send + Sync {
    fn now(&self) -> tokio::time::Instant {
        tokio::time::Instant::now()
    }
    fn sleep(&self, wait: Duration) -> BoxFuture<()> {
        Box::pin(tokio::time::sleep(wait))
    }
    fn jitter(&self, ceiling: Duration) -> Duration {
        use std::hash::BuildHasher;
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let random = std::collections::hash_map::RandomState::new()
            .hash_one(COUNTER.fetch_add(1, Ordering::Relaxed));
        let nanos = ceiling.as_nanos().min(u64::MAX.into());
        Duration::from_nanos((nanos * u128::from(random) / u128::from(u64::MAX)) as u64)
    }
}
pub(crate) struct SystemClock;
impl Clock for SystemClock {}
