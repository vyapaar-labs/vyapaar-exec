# `vyapaar-exec-auth` — Headless Authentication Reference

Documents `crates/vyapaar-exec-auth/src/`: headless authentication for the
execution subsystem — RFC 6238 TOTP, credential management, automated broker
session login, token caching with 08:30 IST pre-market refresh, and CDSL eDIS
pre-authorization.

## Module Map

| Module | Purpose |
|--------|---------|
| `totp.rs` | Zero-dependency RFC 6238 TOTP over Base32 seeds (SHA-1 / SHA-256) |
| `crypto.rs` | Hand-rolled SHA-1/SHA-256/HMAC/Base32/Base64/hex |
| `credentials.rs` | `BrokerCredentials` + env/memory/keyring stores |
| `login.rs` | Headless login flows (Zerodha, Upstox, Angel One) |
| `tokens.rs` | `SessionTokens`, cache, `SessionManager`, IST 08:30 refresh scheduler |
| `edis.rs` | CDSL TPIN batch pre-authorization |

## TOTP

| Type | Purpose |
|------|---------|
| `Totp` | TOTP generator for one Base32 secret |
| `TotpAlgorithm` | `Sha1` (industry default) or `Sha256` |
| `TotpConfig` | Algorithm, `step_secs` (standard 30), `digits` (standard 6) |

`TotpConfig::STANDARD` is the standard broker configuration (SHA-1, 30 s step,
6 digits). `Totp::from_base32` / `with_config` construct from RFC 4648 Base32
seeds (case-insensitive, optional padding, spaces tolerated). `base32_encode` /
`base32_decode` are public helpers.

## Credentials

| Type | Purpose |
|------|---------|
| `BrokerName` | `Zerodha` / `Upstox` / `AngelOne` (`as_str()` → `"zerodha"` / `"upstox"` / `"angelone"`) |
| `BrokerCredentials` | API key, access token, client ID, API secret, TOTP seed; `totp_code_at(unix_seconds)` derives the live OTP |
| `CredentialStore` | Trait for credential retrieval |
| `EnvCredentialStore` | Environment-variable backed |
| `MemoryCredentialStore` | In-memory (tests/local) |
| `KeyringCredentialStore` | OS keyring (behind the `keyring` feature) |

## Login Flows

Each broker implements `LoginFlow` (an HTTP state machine over `AuthHttpClient`,
so WireMock conformance harnesses can point flows at local mock servers):

| Broker | Flow | Sequence |
|--------|------|----------|
| Zerodha | `ZerodhaHeadlessLogin` | `user_id` login → TOTP 2FA → `request_token` → checksum session exchange |
| Upstox | `UpstoxOauthLogin` | TOTP headless authenticate → authorization code → OAuth2 token exchange |
| Angel One | `AngelOneSmartLogin` | SmartAPI `loginByPassword` (SHA-256 password + TOTP) |

`AuthHttpClient` / `AuthHttpRequest` / `HttpReply` abstract the HTTP surface;
`ReqwestAuthHttpClient` is the default implementation.
`extract_code_from_redirect` recovers the OAuth authorization code.

## Tokens & Refresh Scheduling

| Type | Purpose |
|------|---------|
| `SessionTokens` | `access_token`, optional `refresh_token` / `feed_token`, `issued_at`, `expires_at`; `ttl`, `is_expired`, `expires_within` |
| `TokenCache` | In-memory per-broker cache with `store`/`peek`/`valid`/`invalidate` |
| `SessionManager` | Login-and-cache orchestration (per-broker) |
| `DailyRefreshScheduler` | IST-scheduled daily token refresh (pre-market) |
| `IST_OFFSET_SECS` | Fixed +05:30 offset (no DST) |
| `next_ist_run(now, hour_ist, minute_ist)` | Next `HH:MM` IST instant in UTC — anchors the 08:30 IST bootstrap and broker token expiry roll-offs (e.g. SmartAPI 03:30 IST) |

## eDIS

| Type | Purpose |
|------|---------|
| `EdisEntry` | One ISIN + quantity pair to authorize for delivery sale |
| `EdisRequest` | A batch for one client (status-tracked) |
| `EdisStatus` | `Draft` / `Submitted` / `Authorized` / `Rejected{reason}` / `Errored{detail}` |
| `EdisPreAuthorizer` | CDSL TPIN batch pre-authorization pipeline |
| `CdslGateway` / `HttpCdslGateway` | Transport abstraction for CDSL calls |

SEBI requires CDSL eDIS (TPIN) authorization before equity delivery
(`CncDelivery`) sells; the pre-authorizer batches the day's holdings in the
pre-market window so intraday sells never block on TPIN entry during market
hours.

## Zero-Floats Policy

No financial math in auth — all values are strings, tokens, or timestamps
(`EdisEntry.quantity` is exact `Decimal`).

## Related

* `code-core.md` — `BrokerExecutor` adapters use authenticated sessions.
* `code-router.md` — SOR health checks depend on valid sessions.
* `code-adapters.md` — adapters consume `SessionTokens`.
