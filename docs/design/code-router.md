# `vyapaar-exec` — Smart Order Router Reference

Documents `crates/vyapaar-exec-router/src/sor.rs`: the resilient multi-broker Smart Order
Router with cost-optimized routing, health circuit breakers, and automated failover.

## `ResilientRouter`

Routing decision per request:

1. **Filter** registered accounts by health (`Closed`/`HalfOpen`) and SEBI OTR eligibility.
2. **Rank** by `RoutePolicy`:
   - `CostOptimized` — modeled brokerage ascending, OTR ratio as tiebreak (delivery → zero-brokerage, intraday/derivatives → flat-fee).
   - `LowestLatency` — rolling RTT ascending.
   - `Pinned(account)` — pinned account first, others by name.
3. **Place** through top candidate. On `Transport`/`Timeout` failure, advance to next-ranked account (transparent automated failover, up to `max_failover_hops`).
4. **Business-level outcomes** (broker rejections) count as gateway-reached OTR actions — never trigger failover (avoids double execution).

## Modules

| Module | Purpose |
|--------|---------|
| `sor.rs` | `ResilientRouter`, `RoutePolicy` |
| `cost.rs` | `BrokerCostModel` — modeled brokerage per instrument/type |
| `health.rs` | `HealthTracker`, `CircuitState` — per-account health circuit breakers |
| `otr.rs` | `OtrLedger`, `OtrBalanceConfig` — SEBI OTR (Order-to-Trade Ratio) tracking |

## Pre-Trade Risk Gating

Pre-trade SEBI risk gating stays upstream of this layer. Wrap registry executors in
`vyapaar_exec_core::RiskGatedExecutor`, or use the legacy `SmartOrderRouter` for the integrated
risk-engine flow.

## Related

- `vyapaar-exec-core/src/executor.rs` — `RiskGatedExecutor` (mandatory pre-trade gate).
- `vyapaar-exec-core/src/ratelimit.rs` — `RateLimitedBroker` (per-broker token bucket).
- `vyapaar_risk::RiskEngine` — pre-trade risk gate (in `vyapaar-risk`).
