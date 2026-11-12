# `vyapaar-exec-router` — Smart Order Router Reference

Documents `crates/vyapaar-exec-router/src/`: the resilient multi-broker Smart
Order Router with cost-optimized routing, health circuit breakers, SEBI OTR
balancing, and automated failover, plus the legacy risk-integrated router.

## Module Map

| Module | Purpose |
|--------|---------|
| `sor.rs` | `ResilientRouter` v2, `RoutePolicy`, `SorConfig`, `BrokerRegistration`, `BrokerStatus`, `RoutedAck` |
| `cost.rs` | `BrokerCostModel` — modeled brokerage per product/notional |
| `health.rs` | `HealthTracker`, `CircuitState`, `HealthConfig` — per-account circuit breakers |
| `otr.rs` | `OtrLedger`, `OtrBalanceConfig` — SEBI Order-to-Trade Ratio balancing |
| `lib.rs` | Legacy `SmartOrderRouter` (integrated `RiskEngine`) + `ExecRouter` (backward compat) |

## `ResilientRouter` v2

Routing decision per request:

1. **Filter** registered accounts by health (breaker `Open` → excluded) and
   OTR eligibility (ceiling not breached once enforcement has armed).
2. **Rank** by `RoutePolicy`:
   - `CostOptimized` — modeled brokerage ascending, OTR headroom as tiebreak
     (delivery → zero-brokerage, intraday/derivatives → flat-fee).
   - `LowestLatency` — rolling RTT ascending; unsampled accounts probe first
     (optimistic discovery, then measured-fast sticky ranking).
   - `Pinned(account)` — pinned account first, the rest by name.
3. **Place** through the top candidate. On `Transport`/`Timeout` failure the
   hop advances to the next-ranked account (transparent automated failover, up
   to `max_failover_hops`; plan default 1).
4. **Business-level outcomes** (broker rejections) count as gateway-reached
   OTR actions and never trigger failover — re-routing a broker-rejected order
   would risk double execution.

Modifications and cancellations are **stateful**: they always target the
account that owns the child order via the ownership ledger (broker order ID →
account); failover only applies to new orders.

### Types & methods

| Type / method | Purpose |
|---------------|---------|
| `SorConfig` | `policy`, `health`, `otr`, `max_failover_hops` (default: `CostOptimized`, 1 hop) |
| `BrokerRegistration` | `name`, executor, `cost` model, `health` tracker, OTR `ledger` |
| `BrokerStatus` | Ops snapshot: name, circuit state, OTR ratio, trades |
| `RoutedAck` | `broker` account label + `OrderAck` |
| `register(name, executor, cost)` | Register one account (duplicate names rejected) |
| `broker_states(now)` | Health/OTR snapshot of every account |
| `plan_route(req, product, now)` | **Pure** routing decision: ordered eligible account names (carries the µs SOR benchmark without touching transports) |
| `route_order(req, product, now)` | Rank, place, transparent failover within hop budget |
| `route_modify(order_id, …)` / `route_cancel(order_id)` | Stateful modify/cancel to the owning account (count toward OTR numerator) |
| `record_fill(broker)` / `record_fill_for_order(order_id)` | Credit a fill to an account's OTR denominator |

## Cost Model (`cost.rs`)

`BrokerCostModel` — static per-account brokerage model:

| Variant | Behavior |
|---------|----------|
| `ZeroDelivery { intraday_bps }` | Zero brokerage on `CncDelivery`; `notional × bps / 10_000` otherwise |
| `FlatFee { fee }` | Constant per-order INR fee (direct-access brokers) |
| `PercentOfNotional { bps }` | `notional × bps / 10_000` for every product |

`order_cost(product, notional)` is exact `Decimal` math; market orders (no
known notional) degenerate to zero-cost for percentage models, while flat-fee
models remain accurate.

## Health Circuit Breakers (`health.rs`)

`CircuitState`: `Closed` → `Open` → `HalfOpen` → `Closed`. `HealthConfig`
defaults: `max_rtt` 500 ms, 10-sample rolling window (3 min samples), 3
consecutive gateway failures, 30 s cooldown. An Open breaker re-arms to
`HalfOpen` after cooldown so a single probe request can prove recovery. All
time is injected as explicit `DateTime<Utc>`.

## OTR Balancing (`otr.rs`)

`OtrBalanceConfig` defaults: `max_otr` 50.0, `min_operations_before_enforcement`
25. `OtrLedger` counts `orders`/`modifications`/`cancellations` (numerator) and
`trades` (denominator; provisional denominator 1). `otr_ratio()`,
`breaches(&config)`, and `headroom(&config)` steer new pressure toward accounts
with ample buffer. Enforcement arms only after the minimum operations guard,
protecting brand-new accounts from cold-start rejection.

## Legacy Routers (`lib.rs`)

* `SmartOrderRouter` — integrated flow: `RiskEngine::check_order(&req, now)`
  **before** dispatch to a registered broker; `register_broker`,
  `route_order`, `record_fill`, `record_cancellation`.
* `ExecRouter` — backward-compatible OTR counter per scrip/day with exact
  `notional(p, q)` / `notional_exact(p, q)` helpers.

## Pre-Trade Risk Gating

Pre-trade SEBI risk gating stays upstream of `ResilientRouter`: wrap registry
executors in `vyapaar_exec_core::RiskGatedExecutor`, or use the legacy
`SmartOrderRouter` for the integrated risk-engine flow.

## Related

* `code-core.md` — `RiskGatedExecutor` (mandatory pre-trade gate), `RateLimitedBroker`.
* `code-slicer.md` — sliced child orders are routed here.
* `vyapaar_risk::RiskEngine` — pre-trade risk gate (in `vyapaar-risk`).
