# Baseline Differential Fixture Test Results (Post DRT=3 Fix)

**Test Date:** 2026-07-23
**Bead ID:** bf-3awud
**Purpose:** Capture baseline results after DRT=3 fix implementation

## Executive Summary

**Overall Agreement:** 6/8 comparable fixtures (75.0%)
**Decode Errors:** 0
**Total Fixtures:** 12 (8 comparable, 2 no-golden, 2 skipped-feature)

### Key Achievement: DRT=3 Lat/Lon Fix Working ✅
- `gfs_tmp2m_1deg_anl` (GDT=0/PDT=0/DRT=3) now **MATCHES** golden reference
- This is the first DRT=3 lat/lon fixture to pass after the template 5.3 fix

## Detailed Results by Fixture

### ✅ PASSING (6 fixtures)

| Fixture ID | GDT | PDT | DRT | Description | Status |
|------------|-----|-----|-----|-------------|--------|
| `gfs_anl_t2m_5x5` | 0 | 0 | 0 | Lat/lon, simple packing, 5x5 grid | MATCH |
| `drt2_simple_3x3` | 0 | 0 | 2 | Complex packing no spatial differencing, 3x3 | MATCH |
| `gfs_tmp2m_1deg_anl` | 0 | 0 | 3 | **Lat/lon with spatial differencing, 360x181** | **MATCH (NEW after DRT=3 fix)** |
| `drt41_png_3x2` | 0 | 0 | 41 | PNG compression, 3x2 | MATCH |
| `pdt1_ensemble_3x2` | 0 | 1 | 0 | Ensemble member forecast, 3x2 | MATCH |
| `pdt8_accum_3x2` | 0 | 8 | 0 | Time-processed accumulation, 3x2 | MATCH |

### ❌ FAILING (2 fixtures)

#### 1. `nam_awip12_lambert_drt3` (NCEP Grid 218)
**Status:** META_MISMATCH - All 187 fields fail
**Template:** GDT=30 (Lambert Conformal) / PDT=0 / DRT=3
**Issue:** Golden reference file has stale metadata
- Golden shows `drt=0`, `grid.nx=None`, `grid.ny=None`
- Actual correctly shows `drt=3`, `grid.nx=614`, `grid.ny=428`
- Also mismatched: scanning_mode (2 vs 64), resolution_flags (48 vs 56)

**Diagnostic Details (Field 0):**
```
- grid.scanning_mode: expected=2, actual=64
- grid.resolution_flags: expected=48, actual=56
- drt_template: expected=0, actual=3
- packing.reference_value: expected=0, actual=1259851458
- packing.binary_scale_factor: expected=0, actual=4
- packing.decimal_scale_factor: expected=0, actual=2
- packing.bits_per_value: expected=0, actual=15
```

**Root Cause:** Golden reference file (`tests/corpus/golden/nam_awip12_lambert_drt3.json`) was generated with incomplete decoder support. Needs regeneration with correct DRT=3 parsing.

#### 2. `mrms_carib_refl_drt41` (MRMS Caribbean Reflectivity)
**Status:** META_MISMATCH - Single field fails
**Template:** GDT=0 (lat/lon) / PDT=0 / DRT=41
**Issue:** Metadata mismatches in time, grid extent, and packing parameters

**Diagnostic Details (Field 0):**
```
- forecast.reference_time.second: expected=0, actual=55
- level.scale_factor2: expected=1, actual=0
- grid.lon_last: expected=4643985184044005458, actual=4643985184026413272
- packing.reference_value: expected=0, actual=3323729920
- packing.decimal_scale_factor: expected=0, actual=1
- packing.bits_per_value: expected=0, actual=16
```

**Root Cause:** Possible decoder bug in PNG (DRT=41) data representation or golden reference generation issue.

### ⏭️ SKIPPED (2 fixtures - feature disabled)

| Fixture ID | Template | Reason |
|------------|----------|--------|
| `drt40_j2k_3x2` | DRT=40 JPEG2000 | Skipped (jpeg2000 feature not compiled) |
| `gfswave_arctic_wind_drt40` | DRT=40 JPEG2000 | Skipped (jpeg2000 feature not compiled) |

### 📋 NO GOLDEN (2 fixtures)

| Fixture ID | Template | Status |
|------------|----------|--------|
| `nam_awip12_lambert_drt3_20250120` | GDT=30/PDT=0/DRT=3 | No golden reference (needs generation) |
| `hrrr_conus_drt3_lambert` | GDT=30/PDT=0/DRT=3 | No golden reference (needs generation) |

## Template-Specific Summary

```
GDT=0 PDT=0 DRT=0: 1/1 (100.0%)
GDT=0 PDT=0 DRT=2: 1/1 (100.0%)
GDT=0 PDT=0 DRT=3: 1/1 (100.0%) ✅ NOW WORKING
GDT=0 PDT=0 DRT=41: 1/2 (50.0%)
GDT=0 PDT=1 DRT=0: 1/1 (100.0%)
GDT=0 PDT=8 DRT=0: 1/1 (100.0%)
GDT=30 PDT=0 DRT=3: 0/187 (0.0%) ⚠️ Stale golden reference
GDT=30 PDT=8 DRT=3: 0/9 (0.0%) ⚠️ No golden reference
```

## Action Items

1. **High Priority:** Regenerate golden reference for `nam_awip12_lambert_drt3` using correct DRT=3 decoder
2. **Investigate:** `mrms_carib_refl_drt41` metadata mismatches (time, grid, packing)
3. **Generate:** Golden references for `nam_awip12_lambert_drt3_20250120` and `hrrr_conus_drt3_lambert`
4. **Future:** Enable jpeg2000 feature and test DRT=40 fixtures

## Test Execution Details

**Command:**
```bash
cargo test differential_coverage_report --test differential -- --nocapture
```

**Execution Time:** 20.64 seconds
**Output:** `/tmp/differential_test_output.txt`

**Diagnostic Tests:**
- `nam_awip12_lambert_drt3`: `/tmp/nam_drt3_diagnostic.txt` (18.41s)
- `mrms_carib_refl_drt41`: `/tmp/mrms_drt41_diagnostic.txt` (1.54s)

## Conclusion

The DRT=3 fix successfully enables decoding of lat/lon grids with spatial differencing (`gfs_tmp2m_1deg_anl`). However, Lambert conformal grids with DRT=3 (`nam_awip12_lambert_drt3`) show mismatches due to stale golden references. The PNG (DRT=41) fixture (`mrms_carib_refl_drt41`) has metadata inconsistencies that warrant investigation.

**Baseline established for future comparison.**
