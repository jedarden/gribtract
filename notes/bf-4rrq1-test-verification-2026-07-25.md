# Differential Test Verification Report - Bead bf-4rrq1

## Test Run Summary
**Date:** 2026-07-25  
**Test:** `cargo test -p gribtract differential`  
**Test Duration:** 45.99 seconds  
**Result:** ❌ FAILED - Acceptance criteria NOT met

## Acceptance Criteria Status

| Criteria | Expected | Actual | Status |
|----------|----------|--------|--------|
| cargo test completes successfully | Pass | Panic (agreement regression) | ❌ FAILED |
| Test output shows 100% agreement for GFS fixture | 100% agreement | Decode error | ❌ FAILED |
| No test failures or panics | No panic | Test panicked | ❌ FAILED |

## Overall Test Results

```
Fixtures : 20 total (11 comparable, 7 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 7
  decode errors: 1
Agreement: 7/11 (63.6%)
Floor: 80.0%
Gap: -16.4 percentage points
```

## GFS Gaussian-grid Fixture Status

**Fixture ID:** `core_gaussian_gdt40`  
**Path:** `large/flx.2024011500.grib2`  
**Status:** ❌ **DECODE ERROR** - "decode not implemented"

### Details:
- NOAA CORe Archive file (Gaussian Latitude/Longitude grid, GDT 3.40)
- Grid: 512 x 256 Gaussian grid, 131,072 points  
- Storage: remote (10.5 MiB)
- **Blocking Issue:** gribtract library does not implement decoding for GDT 3.40

## Test Failure Details

### Panic Information
```
thread 'differential_coverage_report' (1935861) panicked at crates/gribtract/tests/differential.rs:82:5:
agreement regression: 63.6% < floor 80.0%
```

The test panicked because the current agreement percentage (63.6%) is below the minimum required floor (80.0%).

### Fixture Comparison Results

**✅ Matched (7 fixtures):**
- gfs_anl_t2m_5x5
- drt2_simple_3x2  
- gfs_tmp2m_1deg_anl
- drt41_png_3x2
- pdt8_accum_3x2
- mrms_carib_refl_drt41
- nam_awip12_lambert_drt3

**❌ Decode Errors (1 fixture):**
- core_gaussian_gdt40 (GFS Gaussian-grid fixture)

**❌ Mismatches (3 fixtures):**
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

## Per-Template Agreement Breakdown

```
GDT=0 PDT=0 DRT=0: 1/1          ✅ 100%
GDT=0 PDT=0 DRT=2: 1/1          ✅ 100%  
GDT=0 PDT=0 DRT=3: 1/1          ✅ 100%
GDT=0 PDT=0 DRT=41: 2/2         ✅ 100%
GDT=0 PDT=1 DRT=0: 0/1          ❌ 0%
GDT=0 PDT=1 DRT=3: 0/1          ❌ 0%
GDT=0 PDT=2 DRT=3: 0/1          ❌ 0%
GDT=0 PDT=8 DRT=0: 1/1          ✅ 100%
GDT=30 PDT=0 DRT=3: 187/187     ✅ 100%
GDT=30 PDT=8 DRT=3: 9/9         ✅ 100%
```

## Required Work to Meet Acceptance Criteria

To satisfy the bead bf-4rrq1 acceptance criteria, the following work is needed:

1. **Implement GDT 3.40 decoding** - The GFS Gaussian-grid fixture (core_gaussian_gdt40) requires Gaussian Latitude/Longitude grid support
2. **Fix GEFS ensemble mismatches** - Two GEFS fixtures have template decode issues
3. **Fix PDT=1 ensemble template** - The pdt1_ensemble_3x2 fixture has a mismatch
4. **Raise agreement floor** - Once fixes are implemented, the overall agreement will need to exceed 80%

## Conclusion

The acceptance criteria for bead bf-4rrq1 are **NOT satisfied**. The differential test suite cannot pass until:

1. GDT 3.40 (Gaussian Latitude/Longitude grid) decoding is implemented
2. Template mismatches in PDT 1.0 and PDT 4.8 ensembles are resolved  
3. Overall agreement reaches or exceeds 80.0%

**This bead cannot be closed until the acceptance criteria are met.**

---

**Bead ID:** bf-4rrq1  
**Verification Date:** 2026-07-25  
**Test Duration:** 45.99 seconds  
**Status:** ❌ ACCEPTANCE CRITERIA NOT MET