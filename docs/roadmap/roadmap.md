# `vyapaar-exec` — TDD & Engineering Roadmap

## Milestone 1: Core Execution Traits, Rate Limiter & Auth (P0)
- [x] Port `vyapaar-exec-core` and `vyapaar-exec-auth`.
- [x] Verify token-bucket rate limiting and RFC 6238 TOTP test vectors.

## Milestone 2: Freeze Slicer, Smart Order Router & Audit Journal (P0)
- [ ] Port `vyapaar-exec-slicer`, `vyapaar-exec-router`, and `vyapaar-exec-audit`.
- [ ] Verify mandatory `vyapaar_risk::RiskEngine` invocation in `SmartOrderRouter` and SHA-256 hash chain integrity in `vyapaar-exec-audit`.

## Milestone 3: Broker Execution Adapters (P0)
- [ ] Port `vyapaar-exec-zerodha`, `vyapaar-exec-upstox`, and `vyapaar-exec-angelone`.
- [ ] Verify zero float conversions on order placement/modification wire payloads.
