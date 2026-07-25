# GFS Gaussian-Grid Fixture Test Results Analysis

## Overview

This analysis documents the test results for GFS Gaussian-grid fixtures in the differential test suite, examining agreement percentages, decode errors, and fixture status.

## Test Execution Summary

**Date:** 2026-07-25  
**Test Suite:** Differential harness (`differential_coverage_report`)  
**Overall Agreement:** **63.6%** (7/11 comparable fixtures)  
**Status:** ❌ FAILED - Below 80% floor requirement

### Test Results Breakdown

```
Total fixtures: 20
  - Comparable: 11 (with golden references)
  - No golden: 7 (awaiting golden generation)
  - Skipped (missing feature): 2 (DRT=40 JPEG2000 fixtures)

Comparable Results:
  - ✅ Matched: 7
  - ❌ Mismatch: 2 (gefs ensemble fixtures)
  - ⚠️  Decode error: 1 (core_gaussian_gdt40)
```

## GFS Gaussian-Grid Fixture Results

### 1. `core_gaussian_gdt40` ❌ DECODE ERROR

**Status:** `[decode-err] core_gaussian_gdt40 — decode not implemented`

**Fixture Details:**
- **Source:** NOAA CORe Archive (Climate Data Record)
- **File:** `flx.2024011500.grib2` (3-hourly flux file)
- **Storage:** Remote (10.5 MiB, `tests/corpus/large/`)
- **Grid:** 512 × 256 Gaussian grid (131,072 points)
- **Grid Definition:** GDT 3.40 (Gaussian Latitude/Longitude)
- **N (parallels pole-equator):** 128
- **Coverage:** Global (89.46°N to -89.46°S)
- **URL:** `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`

**Issue:** 
The fixture fails to decode, indicating that GDT 3.40 (Gaussian grid) decoding is not implemented. This is the **primary cause** of the agreement regression from 80% to 63.6%.

**Impact:**
- Counts as a decode error in the differential harness
- Lowers overall agreement percentage by ~7.3%
- Blocks validation of Gaussian grid decoding support

### 2. `gfs_gaussian_gdt40_t1534` ⏳ NO GOLDEN

**Status:** `[no-golden] gfs_gaussian_gdt40_t1534`

**Fixture Details:**
- **Source:** NOAA GDAS (Global Data Assimilation System) Surface Flux
- **File:** `gdas.t00z.sfluxgrbf000.grib2` (2026-07-24 00z analysis)
- **Storage:** Remote (122 MiB, `tests/corpus/large/`)
- **Grid:** 3072 × 1536 T1534 Gaussian grid (4,718,592 points)
- **Grid Definition:** GDT 3.40 (Gaussian Latitude/Longitude)
- **N (parallels pole-equator):** 768
- **Resolution:** ~0.117° (~12 km)
- **URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2`

**Current State:**
According to the corpus manifest, this fixture is marked as "✅ Fully supported" with gribtract's GDT 3.40 decoder successfully handling T1534 Gaussian grids. However, the differential test shows `[no-golden]`, indicating:
- The fixture decodes successfully
- No golden reference output exists for comparison
- Cannot participate in agreement percentage calculation

**Notes:**
- End-to-end integration testing completed 2026-07-24 (bead bf-1qia4)
- All 54 fields reportedly decoded with correct metadata
- Awaiting golden generation for differential validation

## Per-Template Breakdown

```
GDT=0  PDT=0  DRT=0 : 1/1 ✅
GDT=0  PDT=0  DRT=2 : 1/1 ✅
GDT=0  PDT=0  DRT=3 : 1/1 ✅
GDT=0  PDT=0  DRT=41: 2/2 ✅
GDT=0  PDT=1  DRT=0 : 0/1 ❌ (pdt1_ensemble_3x2)
GDT=0  PDT=1  DRT=3 : 0/1 ❌ (gefs_member01_pdt41)
GDT=0  PDT=2  DRT=3 : 0/1 ❌ (gefs_ensemble_mean_pdt48)
GDT=0  PDT=8  DRT=0 : 1/1 ✅
GDT=30 PDT=0  DRT=3 : 187/187 ✅ (nam_awip12_lambert_drt3)
GDT=30 PDT=8  DRT=3 : 9/9 ✅ (nam_awip12_lambert_drt3)
```

**Note:** GDT 3.40 (Gaussian grids) does not appear in the per-template breakdown because `core_gaussian_gdt40` fails to decode.

## Agreement Regression Analysis

### Previous Agreement: 80.0%
- Before GFS Gaussian-grid fixtures were integrated
- Floor was set at 80.0% in commit bf-124k

### Current Agreement: 63.6% (7/11)
- Drop of 16.4 percentage points
- Caused by `core_gaussian_gdt40` decode error
- Test panics: `agreement regression: 63.6% < floor 80.0%`

### Expected Agreement After GDT 3.40 Implementation
Assuming `core_gaussian_gdt40` decodes successfully:
- Comparable fixtures would increase to 12
- If decoded correctly: **58.3%** (7/12) if still mismatched
- If matches golden: **66.7%** (8/12) with one match
- Still below 80% floor due to GEFS ensemble mismatches

## Recommendations

### Immediate Actions
1. **Implement GDT 3.40 decoding** to fix `core_gaussian_gdt40` decode error
2. **Generate golden output** for `gfs_gaussian_gdt40_t1534` to enable differential validation
3. **Fix GEFS ensemble mismatches** (PDT 4.1 and 4.8 decode issues)

### Long-term Strategy
1. Raise `AGREEMENT_FLOOR` incrementally as templates are implemented
2. Add comprehensive Gaussian grid coverage (multiple N values)
3. Integrate PDT 4.1/4.8 ensemble support for GEFS fixtures

## Related Beads

- **bf-1qia4:** Verify GFS Gaussian-grid GDT 3.40 file decodes correctly
- **bf-4a4u9:** Verify golden output generation for GFS Gaussian-grid fixture
- **bf-1zndb:** Verify GFS Gaussian-grid fixture integration in differential test suite
- **bf-5ww3i:** Document GFS Gaussian-grid differential test suite results
- **bf-m42ck:** Verify differential test infrastructure setup

## Conclusion

The GFS Gaussian-grid fixtures are partially integrated:
- ❌ `core_gaussian_gdt40`: Decode error (GDT 3.40 not implemented)
- ⏳ `gfs_gaussian_gdt40_t1534`: Decodes successfully but lacks golden reference

The decode error in `core_gaussian_gdt40` is the primary blocker causing the 63.6% agreement regression. Once GDT 3.40 decoding is implemented and golden output is generated for the T1534 fixture, the agreement percentage should improve, though additional work on GEFS ensemble fixtures (PDT 4.1/4.8) will be needed to reach the 80% floor.
