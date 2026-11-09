# `vyapaar-exec` — Historical Migration Map (honba-te → vyapaar-exec)

> **Tier**: Tier 2B (Depends on `vyapaar-core` and `vyapaar-risk`)
> **Source**: `/home/pmallapp/Devel/Finance/honba-workspace/honba-te`
> **Target**: `/home/pmallapp/Devel/Finance/vyapaar-labs/vyapaar-exec`
> **Status**: Migration complete (all broker crates ported; plan 0001 wire-mapping verified).

## Sub-Crate Mapping Table

| Source Crate (`honba-te/crates/`) | Target Crate (`vyapaar-exec/crates/`) | Action |
| :--- | :--- | :--- |
| `honba-te-core` | `vyapaar-exec-core` | Port `BrokerExecutor`, `ExecutionStream`, rate limiter, and tests. |
| `honba-te-auth` | `vyapaar-exec-auth` | Port TOTP, session manager, eDIS, and tests. |
| `honba-te-slicer` | `vyapaar-exec-slicer` | Port freeze quantity slicer, Iceberg/TWAP/VWAP slicers, and tests. |
| `honba-te-router` | `vyapaar-exec-router` | Port `SmartOrderRouter` wired to `vyapaar_risk::RiskEngine`. |
| `honba-te-audit` | `vyapaar-exec-audit` | Port SQLite WAL SHA-256 hash-chained audit journal and tests. |
| `honba-te-zerodha` | `vyapaar-exec-zerodha` | Port Zerodha Kite v3 executor and webhook postback handler. |
| `honba-te-upstox` | `vyapaar-exec-upstox` | Port Upstox v2 executor. |
| `honba-te-angelone` | `vyapaar-exec-angelone` | Port Angel One SmartAPI 2.0 executor. |

## Mandatory DDD Fixes During Migration

1. Update `vyapaar-exec/Cargo.toml` `[workspace.dependencies]` with all dependencies from `honba-te/Cargo.toml` (`tokio`, `serde`, `serde_json`, `thiserror`, `tracing`, `rust_decimal`, `async-trait`, `futures`, `chrono`, `reqwest`, `uuid`, `proptest`, `axum`, `sqlx`, `sha2`, `hmac`, `totp-rs` / `data-encoding`).
2. Replace `honba_core` → `vyapaar_core`, `honba_mon` → `vyapaar_risk`, and `honba_te_*` → `vyapaar_exec_*`.
3. Extract any inline `#[cfg(test)]` modules from `crates/*/src/*.rs` into `crates/*/tests/`.

## Verification Commands

```bash
cd /home/pmallapp/Devel/Finance/vyapaar-labs/vyapaar-exec
grep -rn "#\[cfg(test)\]" crates/*/src/ && exit 1 || true
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

## Migration Milestone Status

| Milestone | Status |
|-----------|--------|
| All 7 broker crates ported | Complete |
| `vyapaar-exec-core` (ports + rate limiter) | Complete |
| `vyapaar-exec-auth` (TOTP + session + eDIS) | Complete |
| `vyapaar-exec-slicer` (freeze/Iceberg/TWAP/VWAP) | Complete |
| `vyapaar-exec-router` (SOR + RiskEngine gate) | Complete |
| `vyapaar-exec-audit` (SQLite WAL hash chain) | Complete |
| `vyapaar-exec-{zerodha,upstox,angelone}` adapters | Complete |
| Plan 0001 wire-mapping verified (`2675c07`) | Complete |
| Plan 0002 crypto resolved NO-GO (`d529bc9`) | Complete |
| Zero inline `#[cfg(test)]` in `src/` | Complete |
| `cargo fmt --check` | Clean |
| `cargo clippy --all-targets -- -D warnings` | Clean |
| `cargo test --workspace` | Passing |
