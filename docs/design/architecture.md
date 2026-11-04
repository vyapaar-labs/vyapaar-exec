# `vyapaar-exec` — DDD Architecture & Execution Bounded Context

## 1. Bounded Context & Responsibility
`vyapaar-exec` (Tier 2B, ported from `honba-te`) is the **Execution Hand** bridging the pre-trade risk gate (`vyapaar-risk`) and domain OMS (`vyapaar-ledger`) to live broker execution APIs.

## 2. Sub-Crate Topology

| Sub-Crate | Role | Key Domain Abstractions |
| :--- | :--- | :--- |
| `vyapaar-exec-core` | Core Ports & Rate Limiter | `BrokerExecutor`, `ExecutionStream`, per-broker token-bucket rate limiter (prioritizing cancels over new orders), `ProductType` mapping. |
| `vyapaar-exec-auth` | Session & TOTP Auth | RFC 6238 headless TOTP generator, OAuth2 token refresh, CDSL eDIS TPIN authorization. |
| `vyapaar-exec-slicer` | Order Slicing Engine | SEBI exchange freeze-quantity slicer (Nifty/BankNifty lot caps), Iceberg, TWAP, and VWAP child-order schedulers. |
| `vyapaar-exec-router` | Smart Order Router (SOR)| Multi-broker routing with mandatory `vyapaar_risk::RiskEngine` pre-trade check, health circuit breakers, and failover. |
| `vyapaar-exec-audit` | Regulatory Audit Journal| Append-only SQLite WAL journal recording UTC microsecond timestamps, SEBI `AlgoId`, raw payloads, and SHA-256 tamper-evident hash chain. |
| `vyapaar-exec-{zerodha,upstox,angelone}` | Broker Execution ACLs | First-party REST & postback order adapters with exact `Decimal` string serialization. |
