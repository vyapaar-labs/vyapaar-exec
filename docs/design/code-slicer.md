# `vyapaar-exec-slicer` — Order Slicing Engine Reference

Documents `crates/vyapaar-exec-slicer/src/`: SEBI scrip-freeze-limit slicing and
execution algorithms. Every quantity path is exact integer/`Decimal` arithmetic
— no floating point, ever.

## Module Map

| Module | Purpose |
|--------|---------|
| `freeze.rs` | `FreezeLimitCatalog` of per-scrip freeze quantities (CSV/JSON master-data loaders + builtin index constants) |
| `slice.rs` | `FreezeLimitSlicer`: parent → `N = ⌈Q/Q_freeze⌉` children with synchronized client IDs + the placement orchestrator |
| `iceberg.rs` | Native (Kite variety `iceberg`) and synthetic iceberg engines with visible-quantity legging |
| `twap.rs` | Time-weighted schedule with deterministic zero-float Gaussian jitter and limit-chase re-pricing |
| `jitter.rs` | Deterministic integer jitter source (mulberry32 / Irwin–Hall) |

## Domain purity

Planning functions (`FreezeLimitSlicer::plan`, `iceberg::native_leg_count`,
`TwapPlan::build`) are **pure**: they take an explicit `start` timestamp and a
jitter `seed`, never query the clock, and mutate nothing. Runtime orchestrators
(`slice::place_slice_plan`, `iceberg::run_synthetic_iceberg`,
`twap::run_twap`) drive a `BrokerExecutor` + `ExecutionStream` pair and are the
only I/O layer.

## Freeze Catalog (`freeze.rs`)

* `FreezeLimitRecord` — `symbol`, `exchange`, optional broker `token`,
  `freeze_quantity` (u64), optional `lot_size`.
* `FreezeLimit` — a lookup result (limit + lot multiplier).
* `FreezeLimitCatalog` — `new()`, `insert(record)`, `record_for(&instrument)`,
  `slicing_limit(&instrument)`, `from_csv(text)`, `from_json(text)`,
  `with_default_freeze`, `len`/`is_empty`/`iter`. Lookups prefer the broker
  instrument token (exact) and fall back to the canonical upper-case symbol on
  the instrument's exchange.

Builtin regulatory index constants (`builtin_index_freezes()`):

| Symbol | Exchange | Freeze quantity | Lot size |
|--------|----------|-----------------|----------|
| NIFTY | NSE | 1,800 | 50 |
| BANKNIFTY | NSE | 900 | 30 |
| FINNIFTY | NSE | 1,800 | 25 |
| MIDCPNIFTY | NSE | 1,800 | 75 |
| NIFTYNXT50 | NSE | 1,800 | 25 |

## Freeze Slicer (`slice.rs`)

`FreezeLimitSlicer::plan(parent, now)` decomposes a parent into
`N = ⌈Q/Q_freeze⌉` greedy children of size `min(remaining, freeze)` (the
documented split, e.g. 5,000 NIFTY → 1,800 + 1,800 + 1,400). Children inherit
every parent leg; each gets a fresh `client_order_id` UUID and the parent's
UUID in `correlation_id`, so all legs group into a single audit chain.

| Type | Purpose |
|------|---------|
| `SliceLeg` | `index`, `quantity`, fresh `client_order_id` |
| `SlicePlan` | `parent_client_order_id`, applicable `freeze_quantity`, ordered `legs`; `total_quantity()` reconstructs the parent |
| `plan_against_limit` | Lower-level plan given a known freeze limit |
| `place_slice_plan` | Orchestrates submission of all child requests through a `BrokerExecutor` |

`SlicePlan::into_order_requests(&parent)` materializes submission-ready
`OrderRequest`s. When the parent fits under the limit it passes through as a
single child.

## Iceberg (`iceberg.rs`)

| Type | Purpose |
|------|---------|
| `IcebergLegging` | `Native` (broker-metered) or `Synthetic` (client-managed) |
| `MIN_NATIVE_ICEBERG_LEGS` / `MAX_NATIVE_ICEBERG_LEGS` | Kite Connect v3 caps: 2..=50 |
| `SyntheticIcebergOutcome` | `filled_legs`, `unfinished_legs`, aggregate `filled` quantity |
| `native_leg_count(parent, display)` | Derived leg count (exact division, 2..=50) |
| `place_native_iceberg` | Posts the native `iceberg` variety via `IcebergOrderExecutor` |
| `run_synthetic_iceberg` | Submits display-quantity children one at a time; advances only after the active child fills (cumulative `FillReport` deltas); aborts/records unfinished legs on rejection/cancellation/failure |

Every child keeps `correlation_id = parent.client_order_id` for audit-chain
synchronization.

## TWAP (`twap.rs`)

| Type | Purpose |
|------|---------|
| `TwapConfig` | `slices`, `horizon`, Gaussian `jitter` (ms, 0 = exact grid), optional `chase` policy |
| `TwapChase` | Limit-chase knobs: `max_steps`, exact `tick` (`with_tick` parses), `chase_after` quiet window |
| `TwapLeg` | `index`, `submitted_at` (nominal grid + jitter), `quantity` |
| `TwapPlan` | `parent_client_order_id` + scheduled `legs`; `build(parent, cfg, catalog, start, seed)` is pure |
| `TwapOutcome` | Result of the `run_twap` orchestrator |
| `run_twap` | I/O orchestrator executing a plan through `BrokerExecutor` + `ExecutionStream` |

`TwapPlan::build` lifts the leg count to `max(cfg.slices, ⌈Q/Q_freeze⌉)` so
every child respects the freeze invariant; the same seed produces a
byte-identical schedule.

## Jitter (`jitter.rs`)

`JitterSource::from_seed(seed)` — deterministic integer PRNG (mulberry32 with
Irwin–Hall box-Muller approximation for Gaussian-shaped integer jitter), seeded
from the parent (e.g. `client_order_id.as_u128() as u64`) for auditable
deterministic replay.

## Related

* `code-core.md` — `BrokerExecutor` / `ExecutionStream` (slicers feed into executors).
* `code-router.md` — the SOR routes sliced child orders.
* `code-adapters.md` — native iceberg rides the Zerodha `iceberg` variety.
