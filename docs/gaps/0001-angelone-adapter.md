# Gap 0001 — Angel One SmartAPI 2.0 Execution Adapter

**Repo**: `vyapaar-exec` · **Scope**: `crates/vyapaar-exec-angelone/`
**Status**: Open (stub) · **Source of truth**: the placeholder module.

## Problem

`vyapaar-exec-angelone` was scaffolded as part of the `honba-te` migration but
is **not implemented**. `src/lib.rs` contains only a placeholder doc comment
(`//! Placeholder module: populated by the Tier 2B migration in progress.`),
`tests/integration_tests.rs` is empty, and `Cargo.toml` declares only
`vyapaar-core` + `serde` + `thiserror`. The Milestone 3 roadmap item
("Port `vyapaar-exec-zerodha`, `vyapaar-exec-upstox`, and
`vyapaar-exec-angelone`") is therefore **partially** complete — zerodha and
upstox are ported and green, angelone is a stub.

## Required Surface (mirrors the ported adapters)

The adapter must implement `vyapaar_exec_core::BrokerExecutor` (and the exact
wire discipline of `vyapaar-exec-zerodha` / `vyapaar-exec-upstox`):

1. **REST executor** — SmartAPI 2.0: place (`order/place`), modify
   (`order/modify`), cancel (`order/cancel`), order book, positions, funds.
2. **Auth integration** — `vyapaar_exec_auth::AngelOneSmartLogin` /
   `loginByPassword` (SHA-256 password + TOTP) and `SessionTokens`/`feed_token`
   wiring; SmartAPI 03:30 IST token roll-off via `next_ist_run`.
3. **Order stream** — SmartStream order feed pushed into an
   `ExecutionStream` (`BrokerExecutionEvent`), mirroring the Upstox WS
   connector.
4. **Product tokens** — `ProductType::angelone()`/`from_angelone`
   (`DELIVERY` / `INTRADAY` / `CARRYFORWARD` / `MTF`) and a `BrokerEnumMap`
   wire-token table in `src/mapping.rs`.
5. **Error mapping** — SmartAPI `data` envelope + error codes →
   `BrokerError` (mirrors `map_upstox_error`).
6. **Rate limiting** — `rate_limited()` via `BrokerRateCatalog::for_broker("angelone")`
   (SmartAPI 10/s preset).
7. **Zero-float verification** — order placement/modification wire payloads
   serialize exact `Decimal` strings (`format_fixed_exact`, scale 2);
   `wire_mapping_tests` + `payload_tests` prove no float reaches the wire.
8. **Tests** in `crates/vyapaar-exec-angelone/tests/` (parallel test rule) —
   wire mapping, payload exactness, error mapping, and a MockBroker-based
   integration harness.

## Acceptance Criteria

- `cargo test --workspace --profile prod` passes with angelone tests green.
- `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt --check` clean.
- Zero inline `#[cfg(test)]` in `src/`; zero `f32`/`f64` on financial paths.
- This gap document is removed and recorded as Closed in `docs/gaps/README.md`,
  and the plan `docs/plans/0002-milestone3-adapter-completion.md` is updated.

## Plan Link

- `docs/plans/0002-milestone3-adapter-completion.md` — Phase 1.
