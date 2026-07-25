# GFS Gaussian-grid Fixture Integration — Verification

## Task: Add GFS Gaussian-grid fixture to differential.rs

**Status:** ✅ **ALREADY INTEGRATED** — No code changes required

## Verification Summary

The GFS Gaussian-grid fixture (`core_gaussian_gdt40`) is **fully integrated** into the differential testing suite. All components are in place:

### 1. Manifest Entry ✓
- **ID:** `core_gaussian_gdt40`
- **Path:** `large/flx.2024011500.grib2`
- **Storage:** `remote` (fetched from NOAA CORe archive)
- **Manifest:** `tests/corpus/manifest.json`

### 2. Golden Reference File ✓
- **Path:** `tests/corpus/golden/core_gaussian_gdt40.json`
- **Size:** 361 MB
- **Generated:** `scripts/gen_golden.py` (bead bf-5lybk)
- **Status:** Valid JSON matching golden schema

### 3. Raw GRIB Fixture File ✓
- **Path:** `tests/corpus/large/flx.2024011500.grib2`
- **Size:** 11 MB
- **SHA256:** `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397`
- **Source:** NOAA NWS NCEP CORe archive (Google Cloud Storage)

### 4. Diagnostic Test ✓
- **Path:** `crates/gribtract/tests/diagnose_gfs_gaussian.rs`
- **Function:** `diagnose_core_gaussian_gdt40()`
- **Purpose:** Detailed mismatch diagnostics for Gaussian grid fixture
- **Pattern:** Follows same structure as PDT1 ensemble diagnostic

### 5. Differential Test Integration ✓
- **Path:** `crates/gribtract/tests/differential.rs`
- **Mechanism:** `corpus::list_fixtures()` automatically includes ALL fixtures from manifest
- **Test Output:** `[decode-err] core_gaussian_gdt40 — decode not implemented`
- **Confirmation:** Fixture is properly participating in differential harness

### 6. Code Compilation ✓
- **Command:** `cargo check --tests`
- **Result:** Passes without errors

## How It Works

The differential.rs test uses a **manifest-driven approach**:

```rust
let fixtures = corpus::list_fixtures().expect("corpus manifest must load");
for entry in &fixtures {
    // Automatically tests ALL fixtures in manifest
}
```

This means:
- **No individual test cases** need to be added to differential.rs
- **All fixtures** in manifest.json are automatically tested
- **New fixtures** are added via manifest.json, not code changes

## Current State

The `core_gaussian_gdt40` fixture is:
- ✅ Registered in manifest
- ✅ Golden reference generated
- ✅ Raw GRIB file fetched locally
- ✅ Participating in differential test harness
- ✅ Diagnostic test available

**Note:** The fixture currently shows `[decode-err]` because GDT 3.40 (Gaussian Latitude/Longitude grid) decoding is not yet implemented in gribtract. This is expected — the integration is complete, but the decoder support is pending.

## Acceptance Criteria — All Met ✅

- ✅ GFS Gaussian-grid fixture integrated into differential test suite
- ✅ Follows same pattern as existing fixtures (manifest-based, like PDT1 ensemble)
- ✅ Proper test naming (`core_gaussian_gdt40`) and organization
- ✅ Code compiles without errors

## Related Beads

- **bf-5lybk:** Generate GFS Gaussian-grid golden outputs (COMPLETED)
- **bf-5ysjo:** Add GFS Gaussian-grid fixture to differential.rs (THIS TASK)

## Conclusion

**Integration is complete.** The GFS Gaussian-grid fixture is fully integrated into the differential testing suite via the existing manifest-driven architecture. No code changes to differential.rs were required — the fixture was already properly configured and is actively participating in the test harness.
