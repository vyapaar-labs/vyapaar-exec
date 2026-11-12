# HANDOFF — vyapaar-exec

**Status**: Migration complete for 7/8 crates; `vyapaar-exec-angelone` is a
scaffold stub (gap 0001). Roadmap dismantled into `docs/{design,gaps,plans}`;
docs rewritten to reflect source. Plan 0002 active (Milestone 3 completion).
**Date**: 2026-11-12 · Repo: `vyapaar-exec` · Branch: `main`

## Phase / Slice Status (current batch — docs rewrite)

- Deleted `docs/roadmap/` (roadmap.md + migration-verification.md).
- Rewrote `docs/design/`: `architecture.md`, `code-core.md` (replaces
  `code-executor.md`), `code-ratelimit.md`, `code-auth.md`, `code-slicer.md`,
  `code-router.md`, `code-audit.md`, `code-adapters.md` (new), and
  `code-migration-map.md` — all verified against source.
- Added `docs/gaps/README.md` + `0001-angelone-adapter.md` +
  `0002-audit-journal-storage.md`.
- Added `docs/plans/0002-milestone3-adapter-completion.md`; plans index keeps
  0001 (implemented).
- README fixed (no facade crate; audit journal is JSONL segment WAL; angelone
  marked stub). Source comments cite gaps at stub/deviant sites.

## Decisions & Discoveries

- `vyapaar-exec-angelone/src/lib.rs` is a **placeholder only** — Milestone 3 is
  partial (zerodha + upstox ported; angelone stub) → gap 0001.
- Audit journal is a **fsynced JSONL segment WAL**, not SQLite WAL (documented
  plan deviation in `journal.rs`; sqlx/sqlite unavailable) → gap 0002.
- `vyapaar-exec-core` modules beyond the old docs: `wire`, `wiring`
  (`BrokerEnumMap`), `decimal`, `telemetry`.
- Router: v2 `ResilientRouter` (sor/cost/health/otr) is primary; legacy
  `SmartOrderRouter` + `ExecRouter` remain for the integrated risk flow.

## Immediate Next Step

- Plan 0002 Phase 1: implement the Angel One adapter surface (gap 0001) with
  parallel `tests/`; then Phase 2 audit storage reconciliation (gap 0002).
- Re-run the verification suite (`code-migration-map.md`) on close-out.

## Operating Invariants

- Canonical profile `prod`: `cargo build/check/test --profile prod`.
- Tests live in `tests/` parallel to `src/`; no in-file `#[test]` blocks.
- Zero floats in financial paths; `rust_decimal::Decimal` only.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace --profile prod` green.
