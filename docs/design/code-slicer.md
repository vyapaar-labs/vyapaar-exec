# `vyapaar-exec` — Order Slicer Reference

Documents `crates/vyapaar-exec-slicer/src/`: order slicing engine for SEBI exchange
freeze-quantity limits and algorithmic child-order schedulers.

## Slicers

| Slicer | File | Purpose |
|--------|------|---------|
| Freeze quantity slicer | `src/freeze.rs` | Slice orders exceeding exchange freeze limits (e.g. 5,000 NIFTY → 1,800 + 1,800 + 1,400) |
| Iceberg slicer | `src/iceberg.rs` | Synthetic iceberg child orders for venues without native iceberg |
| TWAP scheduler | `src/twap.rs` | Time-weighted average price execution over a window |
| VWAP scheduler | `src/slice.rs` | Volume-weighted average price execution |

## Key Types

| Type | Purpose |
|------|---------|
| `SlicePlan` | Result of slicing: ordered list of child orders with quantities/prices |
| `FreezeCatalog` | Exchange freeze-quantity limits per instrument |
| `SliceError` | Slicing errors (invalid quantity, freeze limit exceeded, etc.) |

## Zero-Floats Policy

All order quantities and prices use `rust_decimal::Decimal`. Freeze-limit comparisons
are exact decimal comparisons — no float rounding.

## Testing

- `tests/freeze_catalog_tests.rs` — freeze limit catalog lookups.
- `tests/iceberg_tests.rs` — iceberg child-order generation.
- `tests/slice_plan_tests.rs` — slice plan correctness.
- `tests/twap_tests.rs` — TWAP schedule correctness.
- `tests/mutation_tests.rs` — mutation testing for slicer logic.
- `slice_proptests.rs` — property-based tests.

## Related

- `vyapaar-exec-core/src/executor.rs` — `BrokerExecutor` (slicers feed into executor).
- `vyapaar-exec-router/src/sor.rs` — SOR routes sliced child orders.
