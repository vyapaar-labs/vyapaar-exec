# Gap 0002 — Audit Journal Storage Reconciliation (SQLite WAL naming vs. JSONL segment WAL)

**Repo**: `vyapaar-exec` · **Scope**: `crates/vyapaar-exec-audit/` + repo docs
**Status**: Open (documented deviation) · **Source of truth**: `src/journal.rs`.

## Problem

The execution gateway's canonical description across the workspace and repo
documentation names a **SQLite WAL audit journal**:

- Vyapaar workspace AGENTS (§2.1): "SQLite WAL audit journal".
- `vyapaar-exec/.agents/AGENTS.md` (§1): "SQLite WAL audit journal".
- `vyapaar-exec/README.md`: "SQLite WAL audit journal logging all order events".

The implemented journal — `crates/vyapaar-exec-audit/src/journal.rs` — is a
**fsynced, append-only JSONL segment WAL** (`wal-{first_seq:010}.jsonl`,
`SEGMENT_ROTATION` = 10,000 entries, `sync_data()` per append) with Apache
Parquet compaction. The source documents this as an explicit plan deviation:

> the plan text names SQLx SQLite WAL storage. sqlx/sqlite are unavailable in
> this build environment's dependency cache and would couple the audit path to
> an external C library; the fsynced JSONL segment WAL delivers the same
> append-only, fsync-per-entry durability with a human-inspectable replay
> format.

The durability properties (append-only, fsync-per-entry, crash-consistent,
replayable) are all delivered; only the storage engine differs from the
published wording.

## Options

1. **Adopt the JSONL segment WAL as canonical** (recommended): update
   `vyapaar-labs/.agents/AGENTS.md`, `vyapaar-exec/.agents/AGENTS.md`, and
   `README.md` to describe the hash-chained fsynced JSONL segment WAL; drop the
   "SQLite" qualifier. No code change; closes the doc/implementation skew.
2. **Revisit SQLx SQLite WAL**: add `sqlx` with a SQLite feature and the
   system SQLite library, port the journal to a `sqlite` WAL table, keep the
   SHA-256 hash chain and Parquet compaction, and re-verify tamper tests. Only
   worthwhile if a downstream consumer (regulator / OMS) requires SQL database
   access to the journal.

## Acceptance Criteria

- All published references to the audit storage engine agree with the
  implemented journal (either the JSONL segment WAL or a real SQLite WAL).
- Tamper-evidence tests (`verify_chain`) remain green; append durability
  unchanged.
- This gap document is removed and recorded as Closed in `docs/gaps/README.md`.

## Plan Link

- `docs/plans/0002-milestone3-adapter-completion.md` — Phase 2.
