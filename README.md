# व्यापार・执行 vyapaar-exec

> *Execution Gateway: First-party broker executors, headless TOTP auth, freeze slicer, Smart Order Router (SOR), and SQLite WAL audit journal.*

Production order routing, smart slicing, and broker protocol adapters.

## Subcrates

- `vyapaar-exec` — Unified execution gateway facade
- `vyapaar-exec-core` — Order execution traits, router contracts, and status lifecycles
- `vyapaar-exec-router` — Smart Order Router (SOR) with latency & fee minimization
- `vyapaar-exec-slicer` — SEBI freeze quantity slicer and TWAP/VWAP iceberg chunker
- `vyapaar-exec-auth` — Headless TOTP multi-factor authentication manager
- `vyapaar-exec-audit` — SQLite WAL audit journal logging all order events
- `vyapaar-exec-zerodha` — Zerodha Kite execution adapter
- `vyapaar-exec-upstox` — Upstox execution adapter
- `vyapaar-exec-angelone` — Angel One SmartAPI execution adapter

## Build & Test

```bash
make prod
make check
make test
```

License: AGPL-3.0.
