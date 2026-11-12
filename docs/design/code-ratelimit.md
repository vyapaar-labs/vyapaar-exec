# `vyapaar-exec-core` — Token-Bucket Rate Limiter Reference

Documents `crates/vyapaar-exec-core/src/ratelimit.rs`: an adaptive token-bucket
rate limiter with strict-priority wait queues shielding every broker adapter
from HTTP 429 bans.

## `RequestPriority` (urgency class)

| Priority | Level | Class |
|----------|-------|-------|
| `Immediate` | 0 | Emergency kill-switch cancellations, manual panic halts |
| `High` | 1 | Stop-loss trigger adjustments, limit order cancellations |
| `Normal` | 2 | Passive order placements, modifications |
| `Low` | 3 | Background order-book, position and margin polling |

## `RateLimiterConfig`

| Field | Meaning |
|-------|---------|
| `capacity_tokens` | Maximum burst size (stored tokens) |
| `refill_rate_per_sec` | Steady-state refill rate |
| `initial_tokens` | Token balance after construction |

`RateLimiterConfig::validated()` enforces positive capacity, non-negative
refill, and `initial_tokens ∈ [0, capacity]`.

## `BrokerRateCatalog` — Documented per-broker presets

| Broker | Endpoint class | Capacity | Refill |
|--------|----------------|----------|--------|
| Zerodha | Order mutations (`zerodha_orders`) | 3 /s | 3 /s |
| Zerodha | General API (`zerodha_general`) | 10 /s | 10 /s |
| Upstox | v2 API (`upstox`) | 25 /s | 25 /s |
| Angel One | SmartAPI (`angelone`) | 10 /s | 10 /s |
| Unknown | `for_broker` default | 10 /s | 10 /s |

`BrokerRateCatalog::for_broker(name)` selects the preset for the adapter name;
`TokenBucketRateLimiter::for_broker(name)` builds a limiter from it.

## `TokenBucketRateLimiter`

Correctness invariants:
1. A waiter is granted only by `dispatch()`, which runs under the bucket lock,
   atomically deducting tokens before marking the waiter granted and popping it
   from its queue.
2. A queue therefore contains only waiters that have NOT been granted; a waiter
   checks its own shared grant flag — no queue scans.
3. Token accounting is exact `Decimal` math (`RoundingStrategy::MidpointAwayFromZero`
   in duration conversion) — no floats, deterministic grant cadence.

### Fast path & queue jumping

A new requester skips queueing entirely when tokens are available, no strictly
higher-priority waiter is queued, and no same-priority waiter precedes it —
Immediate/kill-switch grants are sub-millisecond and never sit behind passive
order flow. Otherwise the waiter subscribes to its priority FIFO
(`BTreeMap<level, VecDeque>`), parks until notified or a capped budget-accrual
timer wakes it, and re-checks on timeout against a race-grant flag.

### Methods

| Method | Purpose |
|--------|---------|
| `new(config)` / `for_broker(name)` | Validated limiter (or from preset) |
| `with_broker(name)` | Attach broker label for telemetry |
| `set_limits(capacity, refill)` | Adaptive reconfigure, clamping the balance |
| `acquire_for(priority, cost, max_wait)` | Consume tokens, queue behind higher priorities; bounded wait |
| `acquire(priority, cost)` | 24 h default budget (production prefers `acquire_for`) |
| `try_acquire(priority, cost)` | Non-queueing fast-path attempt |
| `available_tokens()` / `queued_counts()` | Observability |

## `RateLimitedBroker<E>`

Generic wrapper metering every call through the broker's preset token bucket:

| Operation | Priority |
|-----------|----------|
| place / modify | Normal |
| cancel | High |
| `cancel_order_now` (kill-switch) | Immediate |
| order-book / positions / margins / status / health | Low |

`RateLimitedBroker::preset(inner)` picks the bucket from `inner.name()`;
`with_config(inner, config)` accepts an explicit config; `max_wait` defaults to
2 s. Fetch-class traffic is background-prioritized so portfolio polls can never
delay order placements or cancellations.

## Related

* `code-core.md` — `BrokerExecutor` trait (wrapped by `RateLimitedBroker`).
* `code-router.md` — the SOR routes through rate-limited executors.
* `code-adapters.md` — each adapter exposes `rate_limited()`.
