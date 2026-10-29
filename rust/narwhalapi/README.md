# narwhalapi

Typed Rust clients for the [Narwhal API](https://narwhalapi.com): economics
(including the equity risk premium), foreign exchange, calendars, stocks, funds, crypto,
company calendars, company filings and financials, palm oil and spot prices, and webhook
delivery. Decimal
values retain their exact wire text. Rust 1.82 or later is required.

```toml
[dependencies]
narwhalapi = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Set `NARWHAL_API_KEY` in your server environment. Keep API keys out of browser
code, logs and source control.

```rust
use narwhalapi::{Client, Error};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let nw = Client::from_env()?;
    let cpi = nw.economics().cpi("USA").send().await?;
    println!("{}", cpi.value.as_str());
    if let Some(quota) = cpi.meta.quota {
        println!("Remaining quota: {:?}", quota.remaining);
    }
    Ok(())
}
```

Required path and query parameters are function arguments. Optional query and body
fields have builder methods; only fields you set are sent. Required body fields are
checked by `send()`. Updating a webhook destination requires every field,
including `enabled(false)` to preserve a paused destination. Dot-only path
parameters (`.` and `..`) are rejected before any request.

```rust
let rates = nw.fx().rates("USD").currencies(["IDR", "SGD"]).send().await?;
let converted = nw.fx().convert_currency("1.2300", "USD", "IDR").send().await?;
```

`Decimal` accepts decimal strings or parses with `str::parse`. It does not perform
implicit arithmetic or numeric conversion. With the `rust_decimal` feature,
`to_rust_decimal()` performs an exact conversion and returns an error for more
than 28 significant digits or an unrepresentable scale. Dates and times stay strings.

For pagination, add `futures-util = "0.3"` and use `StreamExt`:

```rust
use futures_util::StreamExt;
let mut observations = nw.economics().cpi_history("USA").start("2025-01").iter();
while let Some(item) = observations.next().await {
    println!("{}", item?.value.as_str());
}
```

Streams fetch pages lazily, preserve filters and stop at an absent or null cursor.
A present empty cursor is a decode error; supplied cursors are sent unchanged.
Cursor expiry returns an error; iteration never silently restarts.

```rust
use narwhalapi::{Error, codes};
match nw.fx().rates("USD").send().await {
    Err(Error::Api(problem)) if problem.code == codes::QUOTA_EXHAUSTED => {
        println!("Quota exhausted; retry after {:?} seconds", problem.retry_after);
    }
    other => { let _ = other?; }
}
```

API errors expose status, open-string code, title, detail, request ID, instance,
retry delay and raw body. `ApiError::kind()` distinguishes bad requests,
unauthorized requests, missing resources, rate limits, unavailable services and
expired cursors. Display uses `HTTP <status> <code>: <title>` and appends
` (request_id <id>)` only when an ID is present. Decode failures retain the exact
response bytes in `DecodeError::raw_body`; transport errors retain their cause.
`TransportError::is_timeout()` and `is_connect()` identify timeouts and connection
failures; Display reports the cause kind without exposing sensitive source text.

Top-level response `meta` is populated exclusively from headers and is excluded
from JSON serialization. Models and runtime structs are non-exhaustive, allowing
new fields in future releases. Open enums retain new strings as `Unknown(String)`;
equality and hashing compare their wire strings, including known and unknown forms.
The documented wire value `"unknown"` in `SchedulePrecision` is `KnownUnknown`.
Palm oil union `kind()` strings are the exact variant model names, or `"unknown"`.
Unknown variants retain exact JSON source text in `raw: Box<serde_json::value::RawValue>`
(access it with `raw.get()`) and original response bytes in `raw_body`, including
unknown page items. Unknown numbers never lose precision or their original spelling.

## Configuration and execution

```rust
use std::time::Duration;
let nw = narwhalapi::Client::builder("your-api-key")
    .timeout(Duration::from_secs(30))
    .extra_headers([("x-application", "reporting")])
    .max_retries(2)
    .max_retry_wait(Duration::from_secs(30))
    .build()?;
```

Defaults are a 30-second total deadline per attempt, a 10-second connect deadline,
and two retries. The total deadline includes response-body reads. Only short-window
429 responses, GET 503 responses, and GET transport failures before response headers
are retried. Retry-After is honored up to `max_retry_wait`; otherwise retries use
full jitter starting at 0.5 seconds. Monthly quota errors are returned immediately.

`base_url()` overrides `NARWHAL_BASE_URL`, which defaults to
`https://api.narwhalapi.com`. `extra_headers()` accepts an iterator of pairs with
keys and values convertible to `String`, such as a string map or an array of pairs;
Authorization always uses the configured key. Clients and builders redact keys in
Debug, and clients redact them in Display. Invalid header names or values fail
client construction without echoing the supplied text.

The async `Client` is `Clone + Send + Sync` and uses the caller's Tokio runtime.
It creates no background runtime or worker thread per client. Connections are pooled;
drop an in-flight future or stream to cancel it. The default HTTP transport honors
standard proxy environment variables. TLS uses Rustls with bundled webpki roots by
default. For corporate CAs installed in the platform certificate store, enable
`native-roots`. To use only platform roots, disable defaults:

```toml
narwhalapi = { version = "0.2", default-features = false, features = ["native-roots"] }
```

Enabling both root features trusts the combined stores. Platform roots are loaded
at client construction and retained when a blocking client rebuilds after fork.
Construct a new client to pick up certificate-store changes. Platform certificate
APIs may initialize shared OS workers; the SDK creates no per-client runtime workers.

Enable `blocking` for synchronous calls with the same operation builders:

```rust
let nw = narwhalapi::blocking::Client::from_env()?;
let cpi = nw.economics().cpi("USA").send()?;
for item in nw.economics().cpi_history("USA").limit(100).iter() {
    println!("{}", item?.value.as_str());
}
```

Configure it with `blocking::Client::builder(key).build_blocking()`. The first send
creates one current-thread runtime and HTTP pool, reused by the client and its clones.
Calls from many threads run concurrently on that runtime and pool, and no dedicated
runtime thread stays running.
When a Tokio handle is present (including `spawn_blocking` or `handle.enter()`),
a scoped worker drives the same cached runtime outside that context. Use the async
client for async tasks to avoid blocking their executor thread.

The cache is keyed by process ID and rebuilt after fork. Fork only with the client
quiescent and its locks unheld; inherited runtime and pool state is abandoned in
the child without shutdown. Custom transports and clocks must supply their own
fork guarantees. An operating-system DNS lookup may finish after a timeout;
client drop shuts down the runtime without waiting for that work.

## Optional surfaces

Features are `rustls` (default webpki roots), `native-roots` (platform roots),
`blocking`, `rust_decimal`, and `unstable`.
`unstable` gates operations that are not yet in the live API; there are none today.
The calendar subscription `feed_url(token)` helper builds a URL without making a request.

To verify webhooks with axum, pass the exact `Bytes` body and
`http::HeaderMap` without parsing or re-serializing the JSON:

```rust
use axum::{body::Bytes, http::{HeaderMap, StatusCode}};
use narwhalapi::webhooks;

async fn webhook(headers: HeaderMap, body: Bytes) -> Result<StatusCode, StatusCode> {
    let secret = std::env::var("NARWHAL_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let event = webhooks::verify(&body, &headers, &[secret], Default::default())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    // Persist event.event_id as the idempotency key before processing.
    Ok(StatusCode::NO_CONTENT)
}
```

Verification also accepts `http::HeaderMap`, `HashMap` and `BTreeMap` headers,
case-insensitively. Plain map values may be strings, string vectors or slices.
All matching keys are sorted before joining their values with commas; vector and
slice values retain their order.
Pass both secrets during rotation. Empty secrets are rejected before verification.
The default timestamp tolerance is 300 seconds; `VerifyOptions` accepts an explicit
clock value and tolerance through
`VerifyOptions::default().now(seconds).tolerance_seconds(300)`. Authenticate the exact raw
bytes, and use `event_id` as the idempotency key.
