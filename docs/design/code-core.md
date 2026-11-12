# `vyapaar-exec-core` — Canonical Execution Ports & Machinery

Documents `crates/vyapaar-exec-core/src/`: the unified broker execution
interface, push-stream contract, canonical Indian product taxonomy, exact wire
helpers, table-driven wire-token mapping, error taxonomy, and telemetry.

## Module Map

| Module | Purpose |
|--------|---------|
| `executor.rs` | `BrokerExecutor` trait v2, `IcebergOrderExecutor`, `RiskGatedExecutor`, `MockBrokerExecutor`, `validate_native_legs` |
| `stream.rs` | `ExecutionStream` push trait, `BrokerExecutionEvent` taxonomy, `ExecutionEventSink` / `ChannelExecutionStream` |
| `types.rs` | `ProductType` taxonomy, `BrokerHealth`, `OrderSnapshot`, `BrokerPosition`, `BrokerMargins`, `PositionSide` |
| `ratelimit.rs` | `RequestPriority`, `TokenBucketRateLimiter`, `BrokerRateCatalog`, `RateLimitedBroker` (see `code-ratelimit.md`) |
| `wire.rs` | Exact JSON → `Decimal`/`Price`/`Quantity` decoding; IST timestamp parsing |
| `decimal.rs` | `format_fixed_exact`, `quantity_to_u32_exact` (no rounding, ever) |
| `wiring.rs` | `BrokerEnumMap<T>` data tables + `OrderTypeKind` |
| `error.rs` | `BrokerError` taxonomy (incl. boxed `RiskRefusal`) |
| `telemetry.rs` | Prometheus broker metrics + OTel `TraceContext` helpers |

The crate re-exports the canonical domain types from `vyapaar-core`
(`OrderRequest`, `OrderAck`, `OrderStatus`, `FillReport`, `Price`, `Quantity`,
`Money`, `InstrumentId`, `OrderSide`, `OrderType`, `TimeInForce`, `Exchange`,
`PriceBand`) and `vyapaar_risk` (`RiskEngine`, `RiskRefusal`).

## `BrokerExecutor` Trait (v2)

Implemented by broker adapters (Zerodha, Upstox, Angel One, mock/sim):

| Method | Purpose |
|--------|---------|
| `name()` | Broker identifier (`"zerodha"`, `"upstox"`, `"angelone"`, `"mock"`) |
| `health_check()` | Session/connection probing (default: not implemented) |
| `place_order(req)` | Place a new order (required) |
| `cancel_order(order_id)` | Cancel an active order (required) |
| `modify_order(order_id, new_price, new_quantity)` | Modify an open order (default: not implemented) |
| `fetch_order_book()` | Active order book snapshot (default: not implemented) |
| `fetch_positions()` | Demat + intraday positions (default: not implemented) |
| `fetch_margins()` | Cash and margin balances (default: not implemented) |
| `get_order_status(order_id)` | Polling fallback (required) |

Pull-based `get_order_status` is retained strictly as a fallback — lifecycles
are driven by push streams.

## `IcebergOrderExecutor` (optional)

Brokers with native iceberg support (currently Zerodha) implement
`place_iceberg_order(parent, display_quantity)`. `native_iceberg_legs` and the
free function `validate_native_legs` enforce the Kite Connect v3 gateway
contract: `display` must divide `parent` exactly into **2..=50** legs.

## `RiskGatedExecutor<E>`

Wrapper around any `BrokerExecutor` enforcing a mandatory pre-trade risk check
before any order touches the broker network:

```rust
pub struct RiskGatedExecutor<E: BrokerExecutor> {
    inner: E,
    risk_engine: Arc<Mutex<vyapaar_risk::RiskEngine>>,
}
```

| Method | Behavior |
|--------|----------|
| `place_order(req)` | `risk_engine.check_order(&req, now)` → refused orders rejected synchronously, never dispatched |
| `cancel_order(order_id, algo_id)` | Cancels at the broker, then `record_cancellation(algo_id)` for OTR tracking |
| `on_fill(fill)` | `risk_engine.record_fill(fill)` to update OTR/positions |

## `MockBrokerExecutor`

Thread-safe deterministic mock: `new(auto_fill)` (auto-fill or leave `New`),
`with_margins`, `with_positions`, `with_session_down`. Simulated default cash
INR 1,000,000.00. Name `"mock"`.

## `ExecutionStream` & Events

```rust
pub enum BrokerExecutionEvent {
    OrderAck(OrderAck),
    Fill(FillReport),
    OrderCancelled { order_id: String, reason: Option<String> },
    OrderRejected { order_id: Option<String>, reason: String },
}
```

`ChannelExecutionStream::channel(capacity)` returns an mpsc pair with an
`ExecutionEventSink` (async `send` + sync `try_send` for webhook handlers).
`BrokerExecutionEvent::order_id()` recovers the broker order id.

## `ProductType` Taxonomy & Wire Tokens

Canonical Indian product taxonomy with per-broker tokens and reverse parsers:

| Canonical | Zerodha | Upstox | Angel One |
|-----------|---------|--------|-----------|
| `CncDelivery` | `CNC` | `D` | `DELIVERY` |
| `MisIntraday` | `MIS` | `I` | `INTRADAY` |
| `NrmlDerivative` | `NRML` | `M` | `CARRYFORWARD` |
| `MtfMargin` | `MTF` | `MTF` | `MTF` |

`ProductType::broker_token(name)` / `from_zerodha` / `from_upstox` /
`from_angelone` perform the mapping.

## Exact Wire Handling

* `format_fixed_exact(value, scale)` — pads to exactly `scale` fractional
  digits (`2500.5 → "2500.50"`); **errors** on tick-misaligned values rather
  than rounding.
* `quantity_to_u32_exact(quantity)` — positive integral share/lot counts only.
* `wire::decimal_from_json` / `price_from_json` / `quantity_from_json` /
  `integer_from_json` — exact `Decimal` from JSON numbers or strings, never
  through IEEE floats.
* `wire::parse_broker_ist_datetime` / `parse_ist_time_of_day` /
  `ist_today` — broker IST timestamps normalized to UTC (+05:30 fixed offset).

## Table-Driven Wire Mapping

`BrokerEnumMap<T>::new(table)` is a `const`-constructible, zero-allocation,
WASM-portable data table mapping broker wire tokens → canonical enum values
(case-insensitive lookups). `OrderTypeKind` maps wire order-type tokens
(`LIMIT`, `SL-M`, `SL`, `MARKET`) to the canonical `OrderType` shape, with the
adapter filling price/trigger payloads via `OrderTypeKind::into_order_type`.

## `BrokerError` Taxonomy

`Auth`, `Rejected`, `RiskRefusal(Box<vyapaar_risk::RiskRefusal>)`,
`Transport`, `Timeout`, `OrderNotFound`, `InvalidRequest`, `RateLimited` —
with `From<RiskRefusal>`.

## Telemetry

`BrokerMetrics` exports Prometheus histogram/gauge/counter families
(`broker_http_request_duration_seconds`, `broker_websocket_latency_seconds`,
`broker_rate_limit_queue_depth`, `broker_rate_limit_drops_total`) plus
OpenTelemetry `TraceContext` ↔ `SpanContext` conversion and W3C `traceparent`
helpers.

## Related

* `code-ratelimit.md` — `RateLimitedBroker` wraps any executor with a token bucket.
* `code-router.md` — `ResilientRouter` wraps registry executors (optionally in `RiskGatedExecutor`).
* `code-adapters.md` — Zerodha/Upstox/Angel One adapters implement these ports.
* `vyapaar_risk::RiskEngine` — pre-trade gate (in `vyapaar-risk`).
