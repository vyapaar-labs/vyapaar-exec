# `vyapaar-exec` — Token-Bucket Rate Limiter Reference

Documents `crates/vyapaar-exec-core/src/ratelimit.rs`: adaptive token-bucket rate
limiter with strict-priority wait queues shielding every broker adapter from HTTP 429 bans.

## `RequestPriority` (urgency class)

| Priority | Level | Class |
|----------|-------|-------|
| `Immediate` | 0 | Emergency kill-switch cancellations, manual panic halts |
| `High` | 1 | Stop-loss trigger adjustments, limit order cancellations |
| `Normal` | 2 | Passive order placements, modifications |
| `Low` | 3 | Background order-book, position, margin polling |

## `RateLimiterConfig`

| Field | Meaning |
|-------|---------|
| `capacity_tokens` | Maximum burst size (stored tokens) |
| `refill_rate_per_sec` | Steady-state refill rate |
| `initial_tokens` | Token balance after construction |

## `BrokerRateCatalog` — Documented per-broker presets

| Broker | Endpoint | Capacity | Refill |
|--------|----------|----------|--------|
| Zerodha | Order mutations | 3 /s | 3 /s |
| Zerodha | General API | 10 /s | 10 /s |
| Upstox | v2 API | 25 /s | 25 /s |
| Angel One | SmartAPI | 10 /s | 10 /s |

## `TokenBucketRateLimiter`

Correctness invariants:
1. A waiter is granted only by `dispatch()`, which runs under the bucket lock, atomically deducting tokens before marking granted and popping from its queue.
2. A queue contains only waiters that have NOT been granted.
3. Token accounting is exact `Decimal` math — no floats, deterministic grant cadence.

### Methods

| Method | Purpose |
|--------|---------|
| `new(config)` / `for_broker(name)` | Create validated limiter (or from preset) |
| `acquire_for(priority, cost, max_wait)` | Consume tokens, queue behind higher-priority waiters |
| `try_acquire(priority, cost)` | Try consuming without queueing |
| `available_tokens()` / `queued_counts()` | Observability |

## `RateLimitedBroker<E>`

Generic wrapper metering every call through a per-broker token bucket:

| Operation | Priority |
|-----------|----------|
| place / modify | Normal |
| cancel | High |
| `cancel_order_now` (kill-switch) | Immediate |
| order-book / positions / margins / status / health | Low |

Fetch-class traffic is background-prioritized so portfolio polls can never delay order placements or cancellations.

## Zero-Floats & Determinism

Token math uses `rust_decimal::Decimal` with `RoundingStrategy::MidpointAwayFromZero`. No `f32`/`f64`.

## Related

- `vyapaar-exec-core/src/executor.rs` — `BrokerExecutor` trait (wrapped by `RateLimitedBroker`).
- `vyapaar-exec-router` — SOR uses rate-limited executors.
