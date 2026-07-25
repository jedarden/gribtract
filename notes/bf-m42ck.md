# Differential Test Infrastructure Verification

**Date:** 2026-07-24  
**Bead:** bf-m42ck

## Summary

Verified that the differential test suite, GFS Gaussian-grid fixtures, and all required dependencies are properly configured and ready for testing.

## Verification Results

### ✅ Test Files Present

All required test infrastructure files exist and are properly sized:

- `crates/gribtract-testutil/src/diff.rs` (518 lines) — Differential comparator for comparing decoded `Field`s against golden references
- `crates/gribtract-testutil/src/golden.rs` (304 lines) — Golden reference loader and JSON mirror types
- `crates/gribtract/tests/regenerate_golden.rs` (136 lines) — Golden output regeneration test

### ✅ GFS Gaussian-grid Fixtures Present

Both GFS Gaussian-grid fixtures are available in the corpus:

1. **core_gaussian_gdt40** — CORe 3-hourly flux file
   - Grid: 512 x 256 Gaussian grid (131,072 points)
   - GDT 3.40 (Gaussian Latitude/Longitude)
   - Golden output: `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)
   - Source file: `tests/corpus/large/flx.2024011500.grib2`

2. **gfs_gaussian_gdt40_t1534** — GDAS surface flux analysis
   - Grid: T1534 (3072 x 1536, 4.7M points, N=768)
   - GDT 3.40 (Gaussian Latitude/Longitude)
   - Source file: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` (122 MB)
   - Fully integrated and verified (see bead bf-1qia4)

### ✅ Compilation Status

All tests compile successfully with no errors:

- `cargo check --tests` — Clean
- `cargo test --test differential --no-run` — Clean
- `cargo test --test regenerate_golden --no-run` — Clean
- No missing dependencies
- No compilation warnings

## Infrastructure Components

### Differential Test Framework

The `diff.rs` module provides:

- **FieldResult** enum for comparison outcomes (Match, MetaMismatch, ValuesMismatch, etc.)
- **CoverageReport** for per-(GDT,PDT,DRT) triple statistics
- Tolerance-based comparison derived from packing headers (half-ULP of quantization step)
- Metadata exact matching + grid value tolerance matching

### Golden Reference System

The `golden.rs` module provides:

- JSON mirror types for deserializing eccodes/wgrib2 output
- Golden reference loading from `tests/corpus/golden/<fixture-id>.json`
- Serde-based (de)serialization for offline comparison

### Test Corpus

The `tests/corpus/` directory contains:

- `manifest.json` — Complete fixture manifest with provenance metadata
- `small/` — Inline fixtures (< 1 MB)
- `large/` — Remote fixtures (fetched via `cargo xtask corpus fetch`)
- `golden/` — Golden reference JSON outputs for differential testing

## Acceptance Criteria Status

| Criterion | Status | Notes |
|-----------|--------|-------|
| Test files exist | ✅ PASS | All 3 required files present |
| GFS Gaussian-grid fixtures present | ✅ PASS | Both core_gaussian and T1534 fixtures available |
| Test compilation succeeds | ✅ PASS | No errors or warnings |
| No missing dependencies | ✅ PASS | Clean cargo check on all tests |

## Conclusion

The differential test infrastructure is fully operational and ready for testing GFS Gaussian-grid fixtures. The framework supports tolerance-based comparison, golden reference validation, and comprehensive coverage reporting across GDT/PDT/DRT combinations.
