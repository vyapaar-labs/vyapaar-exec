# `vyapaar-exec` — Authentication Reference

Documents `crates/vyapaar-exec-auth/src/lib.rs`: headless authentication for the execution
subsystem — RFC 6238 TOTP, credential management, automated broker session login, and CDSL
eDIS pre-authorization.

## Module Map

| Module | Purpose |
|--------|---------|
| `totp` | RFC 6238 TOTP over Base32 seeds (SHA-1/SHA-256), zero-dependency |
| `crypto` | Hand-rolled SHA-1/SHA-256/HMAC/Base32/Base64/hex |
| `credentials` | `BrokerCredentials` + env/memory/keyring stores |
| `login` | Headless login flows (Zerodha, Upstox, Angel One) |
| `tokens` | `SessionTokens`, cache, IST 08:30 pre-market refresh scheduler |
| `edis` | CDSL TPIN batch pre-authorization |

## Key Types

| Type | Purpose |
|------|---------|
| `BrokerCredentials` | API key, access token, client ID, API secret |
| `CredentialStore` | Trait for credential retrieval (env/memory/keyring) |
| `SessionTokens` | Cached broker session tokens |
| `TokenCache` / `SessionManager` | Token lifecycle management |
| `DailyRefreshScheduler` | IST-scheduled daily token refresh |
| `Totp` / `TotpConfig` | TOTP generator and config |
| `EdisPreAuthorizer` | CDSL TPIN pre-authorization |

## Login Flows

| Broker | Flow Class |
|--------|------------|
| Zerodha | `ZerodhaHeadlessLogin` |
| Upstox | `UpstoxOauthLogin` |
| Angel One | `AngelOneSmartLogin` |

## Zero-Floats Policy

No financial math in auth — all values are strings, tokens, or timestamps.

## Related

- `vyapaar-exec-core` — `BrokerExecutor` adapters use authenticated sessions.
- `vyapaar-exec-router` — SOR health checks depend on valid sessions.
