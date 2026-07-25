# Test Results for Bead bf-e9ifx - Differential Test Verification

## Test Run Summary
**Date:** 2026-07-25 (re-verified)
**Test:** `cargo test -p gribtract differential`
**Test Duration:** 45.25 seconds
**Result:** ❌ FAILED
**Agreement:** 7/11 (63.6%)
**Floor:** 80.0%
**Gap:** -16.4 percentage points

## Overall Test Results
```
Fixtures : 20 total (11 comparable, 7 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 7
  decode errors: 1
Agreement: 7/11 (63.6%)
```

## GFS Gaussian-grid Fixture Status
**Fixture ID:** `core_gaussian_gdt40`  
**Path:** `large/flx.2024011500.grib2`  
**Status:** ❌ **DECODE ERROR** - "decode not implemented"

### Details:
- This is a NOAA CORe Archive file (Gaussian Latitude/Longitude grid, GDT 3.40)
- Grid: 512 x 256 Gaussian grid, 131,072 points
- Storage: remote (10.5 MiB)
- **Issue:** The gribtract library does not yet implement decoding for this grid type

## Test Failure Details

### Panic Information
```
thread 'differential_coverage_report' (1652095) panicked at crates/gribtract/tests/differential.rs:82:5:
agreement regression: 63.6% < floor 80.0%
```

The test panicked because the agreement percentage (63.6%) is below the minimum floor (80.0%).

### Comparison Results

**✅ Matched (7 fixtures):**
- gfs_anl_t2m_5x5
- drt2_simple_3x2
- gfs_tmp2m_1deg_anl
- drt41_png_3x2
- pdt8_accum_3x2
- mrms_carib_refl_drt41
- nam_awip12_lambert_drt3

**❌ Decode Errors (1 fixture):**
- core_gaussian_gdt40 (GFS Gaussian-grid fixture - decode not implemented)

**❌ Mismatches (2 fixtures):**
- pdt1_ensemble_3x2
- gefs_ensemble_mean_pdt48
- gefs_member01_pdt41

**⚠️ No Golden Reference (7 fixtures):**
- conus_drt0
- rotated_latlon_5x5
- nam_awip12_lambert_drt3_20250120
- hrrr_conus_drt0_lambert_20260723
- hrrr_conus_drt3_lambert
- gfs_gaussian_gdt40_t1534
- ecmwf_ensemble_pdt41_enso

**⏭️ Skipped (2 fixtures - JPEG2000 feature disabled):**
- drt40_j2k_3x2
- gfswave_arctic_wind_drt40

## Acceptance Criteria Status

### Bead bf-e9ifx Acceptance Criteria
| Criteria | Status | Notes |
|----------|--------|-------|
| 100% agreement for GFS Gaussian-grid fixture | ❌ FAILED | Fixture has decode error |
| No test failures or panics | ❌ FAILED | Test panicked at 63.6% < 80.0% |
| Test results documented and saved | ✅ COMPLETE | This document |
| All acceptance criteria from parent bead bf-4rrq1 satisfied | ❌ FAILED | Parent criteria not met |

### Parent Bead bf-4rrq1 Acceptance Criteria
| Criteria | Status | Notes |
|----------|--------|-------|
| cargo test completes successfully | ❌ FAILED | Test panicked |
| Test output shows 100% agreement for GFS fixture | ❌ FAILED | GFS fixture has decode error |
| No test failures or panics | ❌ FAILED | Test panicked with agreement regression |

## Per-Template Breakdown
```
GDT=0 PDT=0 DRT=0: 1/1
GDT=0 PDT=0 DRT=2: 1/1
GDT=0 PDT=0 DRT=3: 1/1
GDT=0 PDT=0 DRT=41: 2/2
GDT=0 PDT=1 DRT=0: 0/1
GDT=0 PDT=1 DRT=3: 0/1
GDT=0 PDT=2 DRT=3: 0/1
GDT=0 PDT=8 DRT=0: 1/1
GDT=30 PDT=0 DRT=3: 187/187
GDT=30 PDT=8 DRT=3: 9/9
```

## Recommendations

To meet the acceptance criteria, the following work is needed:

1. **Implement GDT 3.40 decoding** - The GFS Gaussian-grid fixture requires Gaussian Latitude/Longitude grid support (GDT 3.40)
2. **Fix GEFS ensemble mismatches** - Two GEFS fixtures have template mismatches
3. **Fix PDT=1 ensemble template** - The `pdt1_ensemble_3x2` fixture has a mismatch

## Conclusion
The acceptance criteria for bead bf-e9ifx are **NOT satisfied**. The test run failed due to:
- Decode error for the GFS Gaussian-grid fixture
- Overall agreement percentage below the minimum floor
- Test panic due to agreement regression

**This bead cannot be closed until the acceptance criteria are met.**
