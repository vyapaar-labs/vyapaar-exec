# `vyapaar-exec-audit` — Hash-Chained Audit Journal Reference

Documents `crates/vyapaar-exec-audit/src/`: the tamper-evident regulatory audit
journal and SEBI Annexure exporter, plus the `vyapaar-audit-export` CLI.

## Module Map

| Module | Purpose |
|--------|---------|
| `hash_chain.rs` | SHA-256 `H_n = SHA-256(H_{n-1} | Timestamp | ClientOrdId | Payload)` chain and pure verification |
| `entry.rs` | `AuditRecord` / `AuditSpec` / `SystemMeta` / `AuditEvent` compliance schema |
| `journal.rs` | Fsynced append-only JSONL segment WAL + Parquet compaction + tamper-evidence verification |
| `system_info.rs` | Best-effort capture-host identity (IP/MAC/hostname) |
| `exporter.rs` | Fixed-column SEBI Annexure CSV rendering |
| `main.rs` | `vyapaar-audit-export` CLI (verify → export → optional Parquet compaction) |

## Storage: fsynced JSONL segment WAL

> **Plan deviation (as-written in `journal.rs`)**: the original plan text named
> SQLx SQLite WAL storage. sqlx/sqlite is unavailable in this build
> environment's dependency cache and would couple the audit path to an external
> C library; the fsynced JSONL segment WAL delivers the same append-only,
> fsync-per-entry durability with a human-inspectable replay format.
>
> **Resolution (gap `0002` closed)**: the JSONL segment WAL is canonical in
> `vyapaar-exec` (tamper-evident file journal + SEBI Annexure export). The
> runtime's `SqliteAuditLedger` (`vyapaar-runtime-persistence`) remains the
> complementary **operational** SQLite store written by the live
> `ExecutionActor` for day-to-day queries. No published reference now claims
> `vyapaar-exec` uses a SQLite WAL journal.

Each journal is a directory of append-only JSONL segments named
`wal-{first_seq:010}.jsonl`, rotated every `SEGMENT_ROTATION` (10,000) entries.
Every `append`:

1. seals the hash chain for the record,
2. line-serializes it,
3. `sync_data()`s the segment **before** the sequence head advances —

so a kill -9 or host crash may drop the in-flight record at worst, never yield
a torn or un-chained entry. Segments are replayed in file-name order on reopen
to restore sequence + chain-head continuity. Compaction writes a real Apache
Parquet snapshot (arrow 53 flat column layout), mirroring the vyapaar-market
tick-lakehouse idiom.

`append` captures time from an **injected** `DateTime<Utc>` (the caller owns
the clock — zero-clock purity).

## Hash Chain (`hash_chain.rs`)

```text
H_n = SHA-256(H_{n-1} | Timestamp | ClientOrdId | Payload)
```

The `|` separator is the plan's `‖` concat realized as an unambiguous framing:
each field is escaped (`\` → `\\`, `|` → `\|`) before joining, so the digest
input is injective. The chain root is `genesis()` = SHA-256 of the fixed ASCII
seed `"vyapaar-audit-genesis-v1"`, so the first entry can never pretend
continuity.

| Type / fn | Purpose |
|-----------|---------|
| `hash_hex(input)` | SHA-256 hex digest |
| `genesis()` | Fixed well-known chain root |
| `chain_hash(prev, timestamp, id, payload)` | Escape + hash the four canonical fields |
| `ChainLink` | Digest input + stored digest (the verifier's minimum view) |
| `ChainBreak` | `GenesisMismatch{seq, …}` or `BrokenLink{seq, …}` (1-based sequence) |
| `verify_chain(links)` | Full-slice verification; empty slices are valid |

## Entry Schema (`entry.rs`)

* `AuditEvent` — `Placed` / `Modified` / `Cancelled` / `Rejected` / `Filled`
  (stable lowercase labels via `as_label()`).
* `AuditSpec` — client order id, exchange order id, SEBI `algo_id`, client
  account id, client PAN, side/product/order-type labels, quantity/price as
  wire strings, event class, raw wire payload.
* `SystemMeta` — public/local IP, MAC address, port, hostname (accessor-only).
* `AuditRecord` — `seq`, `captured_at` (UTC µs), the mandatory SEBI fields,
  `system`, `raw_payload`, `prev_hash`, `hash`. `finalize(prev_hash)` seals the
  entry; `chain_link()` yields the pure verification view.

Records deliberately carry canonical **strings** for money/quantity — the audit
crate must journal exactly what crossed the wire, not re-interpret it.
`timestamp_rfc3339_us(ts)` is the single microsecond RFC 3339 source of truth
for both the chain hash and the CSV exporter.

## SEBI Annexure Exporter (`exporter.rs`)

`to_annexure_csv(records)` renders the fixed 17-column Annexure set
(`ANNEXURE_COLUMNS`) with RFC-4180 escaping (quote fields containing `,` `"`
newline CR; double embedded quotes). Export is a pure function of sealed
records — run `verify_chain` first so tampered journals cannot masquerade as
compliance evidence.

## CLI (`main.rs`)

`vyapaar-audit-export <journal_dir> [--parquet OUT] [--format csv|jsonl]`
opens the journal, verifies the chain, prints the verification line, optionally
compacts a Parquet snapshot, then emits the SEBI Annexure CSV (default) or raw
JSONL replay.

## Related

* `code-router.md` — routed orders/fills feed the audit journal.
* `code-core.md` — `BrokerExecutor` adapters produce the audited events.
* The runtime's `SqliteAuditLedger` is the complementary operational SQLite
  audit store (gap `0002` closed).
