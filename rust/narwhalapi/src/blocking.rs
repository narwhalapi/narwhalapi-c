//! Synchronous operations using a lazy, shared current-thread runtime and HTTP pool.
//! Calls from many threads run concurrently: the lock guards only the cache, never I/O.
//! With a Tokio handle present, a scoped worker drives the cached runtime outside that context. A quiescent client rebuilds its
//! runtime and default pool after fork; custom transports and clocks own their fork safety.
pub use crate::generated::blocking::*;
use crate::{meta::Model, request::RequestData, ClientBuilder, Error};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

struct State {
    pid: u32,
    runtime: Option<tokio::runtime::Runtime>,
    client: Option<crate::Client>,
}
impl Drop for State {
    fn drop(&mut self) {
        let client = self.client.take();
        let runtime = self.runtime.take();
        if self.pid != std::process::id() {
            // Never touch inherited runtime/pool locks in a fork child. The child's
            // copy is abandoned; the parent continues to own and shut down its state.
            std::mem::forget(client);
            std::mem::forget(runtime);
        } else {
            drop(client);
            if let Some(runtime) = runtime {
                // OS resolver work can outlive a deadline. Dropping a client must not
                // wait for it, including when the last clone is dropped in async code.
                runtime.shutdown_background();
            }
        }
    }
}
#[derive(Clone)]
#[non_exhaustive]
pub struct Client {
    builder: ClientBuilder,
    state: Arc<Mutex<Option<Arc<State>>>>,
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
        ClientBuilder::new(api_key).build_blocking()
    }
    pub fn from_env() -> Result<Self, Error> {
        Self::new(std::env::var("NARWHAL_API_KEY").unwrap_or_default())
    }
    /// Finish configuration with `build_blocking()`.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }
    pub(crate) fn from_builder(builder: ClientBuilder) -> Self {
        Self {
            builder,
            state: Arc::new(Mutex::new(None)),
        }
    }
    #[allow(dead_code)] // Generated uses can be gated by optional features.
    pub(crate) fn feed_url(&self, path: &str, name: &str, value: &str) -> Result<String, Error> {
        self.builder.feed_url(path, name, value)
    }
    pub(crate) fn send<T: Model>(&self, data: RequestData) -> Result<T, Error> {
        self.send_keeping_body(data).map(|(value, _)| value)
    }
    pub(crate) fn send_keeping_body<T: Model>(
        &self,
        data: RequestData,
    ) -> Result<(T, Vec<u8>), Error> {
        if tokio::runtime::Handle::try_current().is_ok() {
            std::thread::scope(|scope| {
                scope
                    .spawn(|| self.send_on_thread(data))
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
        } else {
            self.send_on_thread(data)
        }
    }
    fn send_on_thread<T: Model>(&self, data: RequestData) -> Result<(T, Vec<u8>), Error> {
        let state = self.state()?;
        // Current-thread block_on may run on many threads at once: the thread holding the
        // core drives I/O, and every other caller polls its own future when woken.
        state.runtime.as_ref().expect("live runtime").block_on(
            state
                .client
                .as_ref()
                .expect("live client")
                .send_keeping_body(data),
        )
    }
    /// Return the cached runtime and pool, building them first if needed. Only this cheap
    /// check-and-clone runs under the lock.
    fn state(&self) -> Result<Arc<State>, Error> {
        let mut cache = self
            .state
            .lock()
            .map_err(|_| Error::Internal("blocking client state is poisoned"))?;
        let pid = std::process::id();
        if cache.as_ref().is_some_and(|s| s.pid != pid) {
            // State::drop discards inherited resources without shutting them down.
            *cache = None;
        }
        if cache.is_none() {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| crate::TransportError::new(e, false))?;
            let client = {
                let _guard = runtime.enter();
                self.builder.clone().build()?
            };
            *cache = Some(Arc::new(State {
                pid,
                runtime: Some(runtime),
                client: Some(client),
            }));
        }
        Ok(cache.as_ref().expect("initialized cache").clone())
    }
}
