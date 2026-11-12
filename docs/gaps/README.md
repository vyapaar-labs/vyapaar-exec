# Gaps Index — `vyapaar-exec`

Per workspace rule `.agents/rules/gap-tracking.md`:
- **Gaps** (`docs/gaps/XXXX-*.md`) are individual missing features, endpoints, or stubs.
- **Plans** (`docs/plans/XXXX-*.md`) are collections of gaps or multi-phase architectural features.
- When a gap is implemented and verified, remove the document from this
  directory and record it in the README "Closed" table below.

## Active Gaps

| # | Title | Scope | Status |
|---|-------|-------|--------|
| 0001 | Angel One SmartAPI 2.0 execution adapter | `vyapaar-exec-angelone` | Stub — placeholder module only |

## Closed Gaps

| # | Title | Closed In |
|---|-------|-----------|
| 0002 | Audit journal storage reconciliation (SQLite WAL naming vs. JSONL segment WAL) | Closed — JSONL segment WAL is canonical for `vyapaar-exec` (tamper-evident journal + SEBI Annexure export); the runtime's `SqliteAuditLedger` is the complementary operational SQLite store. See `docs/design/code-audit.md` |
