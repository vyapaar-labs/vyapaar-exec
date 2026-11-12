# व्यापार・执行 vyapaar-exec

> *Execution Gateway: First-party broker executors, headless TOTP auth, freeze slicer, Smart Order Router (SOR), and hash-chained audit journal.*

Production order routing, smart slicing, and broker protocol adapters.

## Workspace Subcrates

- `vyapaar-exec-core` — Canonical `BrokerExecutor` ports, execution stream, exact wire/decimal helpers, and strict-priority token-bucket rate limiter
- `vyapaar-exec-router` — Smart Order Router (SOR) with latency & fee minimization, health circuit breakers, and SEBI OTR balancing
- `vyapaar-exec-slicer` — SEBI freeze quantity slicer and Iceberg/TWAP chunkers
- `vyapaar-exec-auth` — Headless TOTP multi-factor authentication manager
- `vyapaar-exec-audit` — Fsynced, SHA-256 hash-chained JSONL segment audit journal with SEBI Annexure export
- `vyapaar-exec-zerodha` — Zerodha Kite execution adapter (REST + postback + ticker stream)
- `vyapaar-exec-upstox` — Upstox execution adapter (REST + portfolio WebSocket feed)
- `vyapaar-exec-angelone` — Angel One SmartAPI execution adapter (scaffold stub — see `docs/gaps/0001-angelone-adapter.md`)

## Build & Test

```bash
make prod
make check
make test
```

## Docs

- `docs/design/` — architecture + per-crate code references reflecting the source.
- `docs/gaps/` — open gaps (Angel One adapter stub, audit storage reconciliation).
- `docs/plans/` — active and implemented engineering plans.

License: AGPL-3.0.
