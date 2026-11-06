# 0001 — Crypto Execution Adapters & Broker Wire-Format Mapping Verification

> **Status**: Partially implemented — Phase 1 (broker wire-mapping verification/refactor) done in `2675c07`; Phase 2 (crypto scope decision) still open; Phase 3 conditional on Phase 2.
> **Target Crates**: new `vyapaar-exec-{exchange}` crates (if pursued), existing `vyapaar-exec-angelone` / `vyapaar-exec-upstox` / `vyapaar-exec-zerodha` (verification only)
> **Depends on reading**:
> - /home/pmallapp/Devel/Finance/vyapaar-labs/vyapaar-market/docs/plans/0001-technicals-breadth-and-derivatives-pricing.md

---

## 0. Executive Summary & Objective

Fincept's `crypto` toolset (76 tools spanning venues, derivatives, custody, compliance, and DeFi) has zero Vyapaar equivalent — no crypto exchange adapter exists in this repo or in `vyapaar-market`. This is framed explicitly as a scope decision (does Vyapaar want crypto execution at all?), paired with the same open broker-wire-mapping verification question raised in `vyapaar-market/docs/plans/0001-technicals-breadth-and-derivatives-pricing.md`, since this repo's broker execution crates are the execution-side counterpart to `vyapaar-market`'s data-side broker crates and the two should be checked together.

---

## 1. Context

- `vyapaar-exec` currently implements broker execution REST clients for three brokers: `vyapaar-exec-angelone`, `vyapaar-exec-upstox`, `vyapaar-exec-zerodha` (confirmed using `reqwest` 0.12), plus `vyapaar-exec-audit`, `vyapaar-exec-auth` (broker credential/TOTP login flows), `vyapaar-exec-core`, `vyapaar-exec-router`, and `vyapaar-exec-slicer`.
- No crypto exchange execution adapter exists anywhere in this repo, mirroring the same absence on the data side in `vyapaar-market` (no crypto market-data adapter either).
- An open, unverified question (raised in `vyapaar-market/docs/plans/0001-technicals-breadth-and-derivatives-pricing.md` §2.4) asks whether `vyapaar-market`'s 5 broker crates use a table-driven wire-enum mapping or ad-hoc if-chains for broker-specific format translation. The equivalent question has not been checked for this repo's `vyapaar-exec-angelone`/`-upstox`/`-zerodha` crates either, and since these are the execution-side counterpart to the same brokers (data vs. execution bounded contexts for the same wire protocols), the two verifications should be done together rather than as separate efforts.
- A different open-source trading terminal product's broker abstraction, examined in an earlier research pass, used a `BrokerEnumMap<T>` pattern — a data-table mapping of broker-specific wire enums to canonical enums — explicitly as a retrofit replacing earlier per-broker if-chains (14 of 16 brokers migrated at the time it was examined). This is cited as a reference pattern only; whether it fits this workspace has not been determined.

---

## 2. Component Specifications

### 2.1 Broker wire-mapping verification (do this first)

Check `vyapaar-exec-zerodha`, `vyapaar-exec-upstox`, and `vyapaar-exec-angelone`'s existing code for how each translates broker-specific order-status, order-side, and order-type enums into this repo's canonical enums:
- If a table-driven approach is already in use, document it as confirmed and move on.
- If ad-hoc if-chains are found, this is a shared refactor opportunity spanning both this repo and `vyapaar-market`'s broker crates — same underlying pattern (wire-enum-to-canonical-enum translation), two different bounded contexts (execution vs. data). Coordinate the refactor scope with `vyapaar-market/docs/plans/0001-technicals-breadth-and-derivatives-pricing.md` §2.4/Phase 3 rather than duplicating the investigation.

#### Verified (Phase 1 complete, exec side)

**Finding: ad-hoc match chains confirmed.** `vyapaar-exec-zerodha` (`lib.rs`, `ticker.rs`, `postback.rs`) and `vyapaar-exec-upstox` (`lib.rs`, `ws.rs`) both translated broker wire tokens with hand-rolled `match` expressions and silent per-call-site fallbacks (`_ => PendingNew`, `_ => Buy`, `_ => Nse`, `_ => Market`), each path re-deriving the same vocabulary. `vyapaar-exec-angelone` is still a placeholder with no wire translation code.

**Refactor applied** (`2675c07`): the shared `BrokerEnumMap<T>` table-driven lookup now lives in `vyapaar-exec-core` (`wiring` module, re-exported at the crate root), with per-broker data tables in `vyapaar-exec-zerodha::mapping` and `vyapaar-exec-upstox::mapping`:
- order status (`KITE_ORDER_STATUS`, `UPSTOX_ORDER_STATUS` — the Upstox table merges the REST and portfolio-feed vocabularies, so the WS ack status is table-driven too)
- order side (`KITE_ORDER_SIDE`, `UPSTOX_ORDER_SIDE`)
- exchange segment (`KITE_EXCHANGE`, `UPSTOX_EXCHANGE`)
- order type (`KITE_ORDER_TYPE`, `UPSTOX_ORDER_TYPE`) mapped to an `OrderTypeKind` discriminant whose price/trigger payloads are applied at the call site, since they ride on the wire record, not the token
- Ticker/postback ack statuses (`KITE_TICKER_STATUS`)

Lookups are ASCII case-insensitive (brokers emit both `COMPLETE` and `complete`) and zero-allocation. Existing conformance/rest/ticker/postback tests were unchanged and stayed green; new `tests/wire_mapping_tests.rs` in each broker crate pin the tables. The canonical→wire direction (`build_order_payload` etc.) was left as exhaustive matches — those have no unmapped-token fallback and are not the pattern the doc's question targets.

**Market-side coordination**: the equivalent question for `vyapaar-market` was checked jointly (this doc cross-references it). `vyapaar-market`'s five broker crates have no wire translation code yet — four are single-line placeholders and `vyapaar-market-dhan` is a stub feed — so there was nothing to refactor there. When real market-data wire translation lands, the same `BrokerEnumMap<T>` pattern should be adopted (mirrored into `vyapaar-market-core`). See `vyapaar-market/docs/plans/0001-technicals-breadth-and-derivatives-pricing.md` §2.4/Phase 3.

### 2.2 Crypto execution scope decision (conditional, no code committed)

State plainly: whether Vyapaar pursues crypto exchange execution at all is an open scope decision, not a committed direction. This document does not resolve it.

### 2.3 If pursued: first crypto exchange adapter shape (conditional on §2.2 being a "go")

IF crypto execution is pursued, the shape would mirror the existing broker-adapter pattern already used in this repo:
- One new crate per venue, `vyapaar-exec-{exchange}`, rather than attempting multi-venue coverage at once.
- Start with a single, well-documented exchange offering a stable REST + WebSocket API, as a pilot — not a broad rollout.
- The new crate would implement whatever shared execution adapter trait/interface `vyapaar-exec-core` already defines. This document explicitly does NOT assume that trait's current shape, since it has not been verified this session. **Action required before design work begins: verify `vyapaar-exec-core`'s existing adapter trait signature first**, rather than inventing one here.

---

## 3. Implementation Phases

### Phase 1: Broker wire-mapping verification (shared with `vyapaar-market`)
Verify §2.1 against `vyapaar-exec-zerodha`/`-upstox`/`-angelone`. Do this first — it is cheap, and its outcome (table-driven vs. ad-hoc) informs whether the same pattern should be used for any new crypto adapter designed in Phase 3.

**DONE** — ad-hoc match chains confirmed and refactored to `BrokerEnumMap<T>` tables; see the verified result in §2.1. `vyapaar-exec-angelone` had nothing to refactor (placeholder).

### Phase 2: Crypto execution scope decision
A go/no-go checkpoint on whether Vyapaar pursues crypto exchange execution at all — explicitly not an assumed yes.

### Phase 3 (conditional on Phase 2 being a "go"): First crypto exchange adapter pilot
Verify `vyapaar-exec-core`'s adapter trait, then implement a single `vyapaar-exec-{exchange}` crate for one well-documented venue as a pilot, following the pattern in §2.3.
