# Plans Index — `vyapaar-exec`

Per workspace rule `.agents/rules/gap-tracking.md`:
- **Gaps** are individual features (`docs/gaps/XXXX-*.md`).
- **Plans** are collections of gaps or multi-phase architectural features
  (`docs/plans/XXXX-*.md`).
- When implemented, plan documents are cleaned up from `docs/plans/` and
  recorded in this index.

## Active Plans

| # | Title | Target Scope | Status |
|---|-------|--------------|--------|
| 0002 | Milestone 3 Adapter Completion (Angel One adapter, audit storage reconciliation) | `vyapaar-exec-angelone`, `vyapaar-exec-audit` | In progress — Phase 1 (gap 0001) open, Phase 2 (gap 0002) open |

## Implemented Plans

| # | Title | Target Scope | Implemented In |
|---|-------|--------------|----------------|
| 0001 | Crypto Execution Adapters & Broker Wire-Format Mapping Verification | `vyapaar-exec-core`, `vyapaar-exec-zerodha`, `vyapaar-exec-upstox` | Phase 1 wire-mapping verified in `2675c07`; Phase 2 crypto resolved NO-GO in `d529bc9` |
