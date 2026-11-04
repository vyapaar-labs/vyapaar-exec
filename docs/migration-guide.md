# `vyapaar-exec` — Subagent Migration & DDD Refactoring Guide

> **Tier**: Tier 2B (Depends on `vyapaar-core` and `vyapaar-risk`)
> **Source**: `/home/pmallapp/Devel/Finance/honba-workspace/honba-te`
> **Target**: `/home/pmallapp/Devel/Finance/vyapaar-labs/vyapaar-exec`

---

## 1. Sub-Crate Mapping Table

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

---

## 2. Mandatory DDD Fixes During Migration
1. Update `vyapaar-exec/Cargo.toml` `[workspace.dependencies]` with all dependencies from `honba-te/Cargo.toml` (`tokio`, `serde`, `serde_json`, `thiserror`, `tracing`, `rust_decimal`, `async-trait`, `futures`, `chrono`, `reqwest`, `uuid`, `proptest`, `axum`, `sqlx`, `sha2`, `hmac`, `totp-rs` / `data-encoding` as used in `honba-te`).
2. Replace `honba_core` → `vyapaar_core`, `honba_mon` → `vyapaar_risk`, and `honba_te_*` → `vyapaar_exec_*`.
3. Extract any inline `#[cfg(test)]` modules from `crates/*/src/*.rs` into `crates/*/tests/`.

## 3. Verification Commands
```bash
cd /home/pmallapp/Devel/Finance/vyapaar-labs/vyapaar-exec
grep -rn "#\[cfg(test)\]" crates/*/src/ && exit 1 || true
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```
