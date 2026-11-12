# Broker Execution Adapters — Zerodha / Upstox / Angel One

Documents the first-party broker ACLs in `crates/vyapaar-exec-{zerodha,upstox,angelone}/`.
All adapters implement `vyapaar_exec_core::BrokerExecutor` (and, where
supported, `IcebergOrderExecutor`), translate wire payloads through the exact
`wire`/`decimal` helpers, and normalize broker errors into `BrokerError`.

## Zerodha (`vyapaar-exec-zerodha`)

Kite Connect v3 adapter (`https://api.kite.trade`, 15 s request timeout):

| Surface | Details |
|---------|---------|
| REST | `POST /orders/regular` (place), `PUT /orders/regular/{id}` (modify), `DELETE /orders/regular/{id}` (cancel), `GET /orders` (book), `GET /portfolio/positions` (positions), `GET /user/margins` (margins), `GET /user/profile` (health), `GET /orders/{id}` (status) |
| Native iceberg | `POST /orders/iceberg` with `iceberg_legs` (2..=50, exact division) + `iceberg_quantity` |
| Postback webhook | `postback.rs` — axum listener on `/postback` verifying the Kite SHA-256 checksum (`order_id + order_timestamp + api_secret`, constant-time compare), tracking cumulative fills, emitting `BrokerExecutionEvent`s over an `ExecutionStream` |
| Ticker stream | `ticker.rs` — `wss://ws.kite.trade` order-update text frames (`{"type":"order","data":[…]}`), `KiteTickerOrderStream` + `spawn_order_updates` with retry/backoff and keepalive pings |
| Rate limiting | `rate_limited()` → `RateLimitedBroker` on the Kite presets (orders 3/s, general 10/s) |
| Auth header | `token {api_key}:{access_token}`, `X-Kite-Version: 3` |

`build_order_payload` emits exact decimal strings at scale 2 (`format_fixed_exact`)
with `quantity_to_u32_exact` share counts; SEBI `algo_id` travels in the
compliance `tag`; `TimeInForce::Gtc` maps to Kite `TTL`. `map_kite_error`
translates HTTP status + `error_type` literals (429/`TooManyRequests` →
`RateLimited`, 401/403/`TokenException` → `Auth`, `InputException` → `InvalidRequest`,
`NetworkException`/`GatewayException` → `Transport`, else `Rejected`).
`mapping.rs` holds the wire-token → canonical mapping (via `BrokerEnumMap`).

## Upstox (`vyapaar-exec-upstox`)

Upstox v2 adapter (`https://api.upstox.com/v2`, OAuth2 Bearer, 15 s timeout):

| Surface | Details |
|---------|---------|
| REST | `POST /order/place` (place), `PUT /order/modify` (modify), `DELETE /order/cancel?order_id=` (cancel), `GET /order/book` (book), `GET /portfolio/short-term-positions` (positions), `GET /user/funds-and-margin` (margins), `GET /user/profile` (health), `GET /order/details/{id}` (status) |
| Modify semantics | **Read-then-patch**: the current order detail is fetched first, then patched with the new price/quantity legs |
| Order stream | `ws.rs` — portfolio RSS feed (`wss://ws.upstox.com/v2/feed/portfolio-stream-feed`); the connector follows the documented 302 redirect to the dedicated feed host (bounded hops) |
| Rate limiting | `rate_limited()` → `RateLimitedBroker` on the Upstox preset (25/s) |
| Auth header | `Authorization: Bearer {access_token}`, `Content-Type: application/json` |

`UpstoxOrderPayload` serializes `price`/`trigger_price` as exact scale-2
strings and `quantity` as a validated integral u32 — no `f64` anywhere.
Instrument keys map through `upstox_exchange_segment` (`NSE_EQ`, `BSE_EQ`,
`NSE_FO`, `NSE_CDS`, `MCX_FO`); `canonical_symbol` strips Upstox `-EQ`/`-BE`
suffixes. `map_upstox_error` handles the `{"errors":[…]}` envelope
(`UPI1001`/`RATELIMIT` → `RateLimited`, `UDC1002`/`AUTHENTICATION`/expiry →
`Auth`, `UDI*`/invalid → `InvalidRequest`, 503/unavailable → `Transport`,
else `Rejected`). `TimeInForce::Gtc` falls back to `DAY`.

## Angel One (`vyapaar-exec-angelone`)

**Stub only.** `src/lib.rs` is a placeholder doc comment and
`tests/integration_tests.rs` is empty; `Cargo.toml` declares only
`vyapaar-core` + `serde` + `thiserror`. The SmartAPI 2.0 adapter (REST
`loginByPassword`/TOTP session, order REST, SmartStream order feed, product
tokens `DELIVERY`/`INTRADAY`/`CARRYFORWARD`/`MTF`, SmartAPI 10/s preset) is not
implemented — see `docs/gaps/0001-angelone-adapter.md` and
`docs/plans/0002-milestone3-adapter-completion.md`.

## Shared Wire Discipline

* Prices/quantities decode exactly: `wire::decimal_from_json` round-trips
  through serde_json's shortest textual form; exchange floats never become
  domain values.
* Broker timestamps parse as IST (+05:30) and normalize to UTC.
* Every adapter exposes `rate_limited()` for the documented preset.

## Related

* `code-core.md` — the `BrokerExecutor` / `IcebergOrderExecutor` ports these adapters implement.
* `code-auth.md` — headless login flows supply the sessions these adapters consume.
* `docs/gaps/0001-angelone-adapter.md` — the pending Angel One surface.
