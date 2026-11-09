# `vyapaar-exec` — Audit Journal Reference

Documents `crates/vyapaar-exec-audit/src/lib.rs`: hash-chained regulatory audit journal
and SEBI Annexure exporter.

## Module Map

| Module | Purpose |
|--------|---------|
| `hash_chain` | SHA-256 `H_n = SHA-256(H_{n-1} | Timestamp | ClientOrdId | Payload)` chain and pure verification |
| `entry` | `AuditRecord`/`AuditSpec`/`SystemMeta` compliance schema |
| `journal` | fsynced append-only JSONL segment WAL + Parquet compaction + tamper-evidence verification |
| `system_info` | Best-effort capture-host identity (IP/MAC/hostname) |
| `exporter` | Fixed-column SEBI Annexure CSV rendering |

## Key Types

| Type | Purpose |
|------|---------|
| `AuditRecord` | Immutable audit event record |
| `AuditEvent` | Type of audited event |
| `AuditSpec` | Compliance specification |
| `SystemMeta` | Host identity metadata |
| `ChainLink` / `ChainHash` | Hash-chain link with SHA-256 digest |
| `AuditJournal` | Append-only journal with fsync |

## Guarantees

- **Tamper-evident**: SHA-256 hash chain (`verify_chain` detects breaks).
- **Append-only WAL**: fsynced JSONL segments with rotation (`SEGMENT_ROTATION`).
- **SEBI Annexure export**: Fixed-column CSV rendering (`to_annexure_csv`).
- **Microsecond timestamps**: UTC microsecond precision on every entry.

## Related

- `vyapaar-exec-router` — router records orders/fills through the audit journal.
- `vyapaar-exec-core` — `BrokerExecutor` implementations feed audit events.
