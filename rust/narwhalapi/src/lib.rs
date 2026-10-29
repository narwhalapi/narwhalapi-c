//! Typed Rust clients for the [Narwhal API](https://narwhalapi.com): economics,
//! foreign exchange, calendars, palm oil and spot prices, and delivery (events,
//! webhooks and calendar subscriptions). Decimal values retain their exact wire text.
//!
//! ```no_run
//! use narwhalapi::{Client, Error};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Error> {
//!     let nw = Client::from_env()?; // reads NARWHAL_API_KEY
//!     let cpi = nw.economics().cpi("USA").send().await?;
//!     println!("{}", cpi.value.as_str());
//!     Ok(())
//! }
//! ```
//!
//! Operations are grouped by namespace (`economics()`, `fx()`, `calendars()`,
//! `commodities()`, `events()`, `webhooks()`, `calendar_subscriptions()`); optional
//! parameters are builder methods and `send()` makes the request. Paginated requests
//! also offer `iter()`, a lazy stream.
//!
//! Features: `rustls` (default), `native-roots`, `blocking` (a synchronous client
//! with the same builders), `rust_decimal` (exact conversion), and `unstable`
//! (operations not yet in the live API; none today).
//! The README covers configuration, retries, errors, concurrency and webhooks.
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;
mod client;
pub mod codes;
mod decimal;
mod error;
mod generated;
mod meta;
mod pagination;
mod request;
#[doc(hidden)]
pub mod transport;
pub mod webhooks;

pub use client::{Client, ClientBuilder};
pub use decimal::Decimal;
pub use error::{
    ApiError, ApiErrorKind, DecodeError, Error, TransportError, WebhookVerificationError,
};
pub use generated::{models, operations};
pub use meta::{RateLimit, ResponseMeta};
pub use pagination::PageStream;
pub use request::Request;

/// Package version used in the User-Agent header.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[doc(hidden)]
pub use meta::Model;
#[doc(hidden)]
pub use pagination::Page;
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub use pagination::PageIterator;
