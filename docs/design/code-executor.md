# `vyapaar-exec` — Broker Executor Trait Reference

Documents `crates/vyapaar-exec-core/src/executor.rs`: the canonical `BrokerExecutor`
trait and the `RiskGatedExecutor` wrapper enforcing mandatory pre-trade risk checks.

## `BrokerExecutor` Trait (v2)

Implemented by broker execution adapters (Zerodha, Upstox, Angel One, Paper/Sim):

| Method | Purpose |
|--------|---------|
| `name()` | Broker identifier (`"zerodha"`, `"upstox"`, `"angelone"`, `"sim"`) |
| `health_check()` | Session/connection probing |
| `place_order(req)` | Place a new order |
| `cancel_order(order_id)` | Cancel an active order |
| `modify_order(order_id, new_price, new_quantity)` | Modify an open order |
| `fetch_order_book()` | Full active order book snapshot |
| `fetch_positions()` | Demat and intraday positions |
| `fetch_margins()` | Real-time cash and margin balances |
| `get_order_status(order_id)` | Polling fallback for order lifecycle |

## `IcebergOrderExecutor` Trait (optional)

Brokers with native iceberg support (Zerodha Kite Connect v3) implement this:
- `place_iceberg_order(parent, display_quantity)` — broker-native iceberg legging
- `native_iceberg_legs(parent_qty, display_qty)` — validated leg count (2..=50, exact division)

## `RiskGatedExecutor<E>`

Wrapper around any `BrokerExecutor` that **strictly enforces pre-trade risk checks**
via `vyapaar_risk::RiskEngine` before forwarding to the broker:

```rust
pub struct RiskGatedExecutor<E: BrokerExecutor> {
    inner: E,
    risk_engine: Arc<Mutex<vyapaar_risk::RiskEngine>>,
}
```

| Method | Behavior |
|--------|----------|
| `place_order(req)` | Evaluates against SEBI OTR limits, circuit bands, max notional, daily loss, kill switches — then dispatches. Refused orders never touch the broker network. |
| `cancel_order(order_id, algo_id)` | Cancels then records cancellation in risk engine for OTR tracking. |
| `on_fill(fill)` | Notifies risk engine of fill to update OTR and positions. |

## `MockBrokerExecutor`

Thread-safe mock for integration testing/deterministic simulation:
- `new(auto_fill)` — auto-fill orders immediately or leave `New`.
- `with_margins()`, `with_positions()`, `with_session_down()` — configure simulated state.
- Simulated cash: INR 1,000,000.00.

## `validate_native_legs`

Validates broker-native iceberg leg count: `parent / display` must divide exactly into 2..=50 legs (Kite Connect v3 documented caps).

## Zero-Floats Policy

All prices, quantities, margins use `rust_decimal::Decimal`. No floats in financial paths.

## Related

- `vyapaar-exec-core/src/ratelimit.rs` — `RateLimitedBroker` wraps any executor with per-broker token bucket.
- `vyapaar-exec-router/src/sor.rs` — `SmartOrderRouter` uses `RiskGatedExecutor`.
- `vyapaar_risk::RiskEngine` — pre-trade gate (in `vyapaar-risk`).
