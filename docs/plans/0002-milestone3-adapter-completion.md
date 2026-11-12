# Plan 0002 — Milestone 3 Adapter Completion

**Repo**: `vyapaar-exec` · **Status**: In progress
**Source**: dismantled `docs/roadmap/` Milestone 3 — "Broker Execution Adapters (P0)"
**Collects gaps**: `docs/gaps/0001-angelone-adapter.md`,
`docs/gaps/0002-audit-journal-storage.md`

## Background

The legacy engineering roadmap (`docs/roadmap/roadmap.md`) was dismantled into
`docs/design/`, `docs/gaps/`, and `docs/plans/` (this change). Milestones 1 and
2 (core traits + rate limiter + auth; slicer + router + audit) are complete and
verification is recorded in `docs/design/code-migration-map.md` and
`code-ratelimit.md`/`code-slicer.md`/`code-router.md`/`code-audit.md`. The
remaining Milestone 3 work is:

- [x] Port `vyapaar-exec-zerodha` — complete (REST + postback + ticker + native iceberg).
- [x] Port `vyapaar-exec-upstox` — complete (REST + portfolio WS feed).
- [ ] Port `vyapaar-exec-angelone` — **stub** (gap 0001).
- [ ] Verify zero float conversions on order placement/modification wire
      payloads — covered by `wire_mapping_tests`/`payload_tests` for zerodha
      and upstox; outstanding for the Angel One adapter (gap 0001, item 7).
- [ ] Reconcile the audit storage wording (SQLite WAL vs. JSONL segment WAL) —
      gap 0002.

## Phases

### Phase 1 — Angel One SmartAPI 2.0 adapter (gap 0001)

Deliver the `vyapaar-exec-angelone` `BrokerExecutor` surface listed in gap
0001: REST executor, SmartLogin + token wiring (incl. `feed_token` and 03:30
IST roll-off), SmartStream order feed → `ExecutionStream`, `BrokerEnumMap`
wire tables, error mapping, `rate_limited()` (SmartAPI 10/s), exact-decimal
wire payloads, and the parallel `tests/` suite (wire mapping, payload
exactness, error mapping, integration).

**Exit criteria**: green `cargo test --workspace --profile prod`, clean
`cargo clippy --all-targets -- -D warnings`, clean `cargo fmt --check`,
no inline `#[cfg(test)]` in `src/`, no floats on financial paths. Remove gap
0001.

### Phase 2 — Audit journal storage reconciliation (gap 0002)

Pick one resolution from gap 0002 (recommended: adopt the fsynced JSONL
segment WAL as canonical and update the workspace/repo AGENTS + README wording;
alternative: port to SQLx SQLite WAL). Align every published reference with the
implemented journal and re-verify tamper tests. Remove gap 0002.

### Phase 3 — Milestone 3 close-out

- Re-run the verification suite (`code-migration-map.md` commands) and record
  results.
- Move plan 0002 to the Implemented table in `docs/plans/README.md` with the
  closing commit, and mark gap 0001 / gap 0002 Closed in `docs/gaps/README.md`.
- Update `.agents/HANDOFF.md` with the completion state.

## Invariants

- Zero floats: `rust_decimal::Decimal` only on financial paths.
- Parallel test directory rule: tests in `tests/`, never in `src/`.
- Canonical profile `prod` for build/check/test.
