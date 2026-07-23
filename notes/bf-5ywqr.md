# Decode Error Rate Analysis - Post DRT=3 Fix

**Bead ID:** bf-5ywqr
**Test Date:** 2026-07-23
**Baseline:** bf-3awud (2026-07-23 07:51:20)

## Executive Summary

**Decode Error Rate Status:** ✅ **NO REGRESSION DETECTED**
- **Baseline decode errors:** 0
- **Current decode errors:** 0
- **Change:** None (0 → 0)

**Overall Test Suite Health:** ✅ **IMPROVED**
- **Baseline agreement:** 6/8 (75.0%)
- **Current agreement:** 8/8 (100.0%)
- **Change:** +2 fixtures (+25.0 percentage points)

## Baseline vs Current Comparison

| Metric | Baseline | Current | Change |
|--------|----------|---------|--------|
| Total Fixtures | 12 | 12 | - |
| Comparable Fixtures | 8 | 8 | - |
| Decode Errors | 0 | 0 | **0** ✅ |
| Fixtures Matched | 6 | 8 | **+2** ✅ |
| Agreement Percentage | 75.0% | 100.0% | **+25.0%** ✅ |

## Template-Specific Decode Error Comparison

### All Templates: 0 Decode Errors (Baseline & Current)

| Template | Baseline Decode Errors | Current Decode Errors | Change |
|----------|------------------------|------------------------|--------|
| GDT=0 PDT=0 DRT=0 | 0/1 | 0/1 | 0 |
| GDT=0 PDT=0 DRT=2 | 0/1 | 0/1 | 0 |
| GDT=0 PDT=0 DRT=3 | 0/1 | 0/1 | 0 |
| GDT=0 PDT=0 DRT=41 | 0/2 | 0/2 | 0 |
| GDT=0 PDT=1 DRT=0 | 0/1 | 0/1 | 0 |
| GDT=0 PDT=8 DRT=0 | 0/1 | 0/1 | 0 |
| GDT=30 PDT=0 DRT=3 | 0/187 | 0/187 | 0 |
| GDT=30 PDT=8 DRT=3 | 0/9 | 0/9 | 0 |

**Total Fields:** 202 fields across 8 fixtures
**Baseline Decode Errors:** 0/202 (0.0%)
**Current Decode Errors:** 0/202 (0.0%)
**Change:** 0% (stable)

## Fixture-Level Analysis

### Previously Failing (Now Fixed)

#### 1. `nam_awip12_lambert_drt3` (187 fields)
- **Baseline:** META_MISMATCH (stale golden reference)
- **Current:** MATCH
- **Root Cause:** Golden reference regenerated with correct DRT=3 parsing
- **Impact:** +187 fields now matching

#### 2. `mrms_carib_refl_drt41` (1 field)
- **Baseline:** META_MISMATCH (metadata inconsistencies)
- **Current:** MATCH
- **Root Cause:** Golden reference regenerated with correct PNG (DRT=41) representation
- **Impact:** +1 field now matching

### Consistently Passing Fixtures (6 fixtures)

| Fixture | Template | Fields | Status |
|---------|----------|--------|--------|
| `gfs_anl_t2m_5x5` | GDT=0/PDT=0/DRT=0 | 1 | MATCH (stable) |
| `drt2_simple_3x3` | GDT=0/PDT=0/DRT=2 | 1 | MATCH (stable) |
| `gfs_tmp2m_1deg_anl` | GDT=0/PDT=0/DRT=3 | 1 | MATCH (stable) |
| `drt41_png_3x2` | GDT=0/PDT=0/DRT=41 | 1 | MATCH (stable) |
| `pdt1_ensemble_3x2` | GDT=0/PDT=1/DRT=0 | 1 | MATCH (stable) |
| `pdt8_accum_3x2` | GDT=0/PDT=8/DRT=0 | 1 | MATCH (stable) |

### Non-Comparable Fixtures

**No Golden References (2 fixtures):**
- `nam_awip12_lambert_drt3_20250120` (GDT=30/PDT=0/DRT=3) - needs golden generation
- `hrrr_conus_drt3_lambert` (GDT=30/PDT=0/DRT=3) - needs golden generation

**Skipped - Feature Disabled (2 fixtures):**
- `drt40_j2k_3x2` (DRT=40 JPEG2000)
- `gfswave_arctic_wind_drt40` (DRT=40 JPEG2000)

## Root Cause Analysis of Agreement Improvement

The +25% agreement improvement (6/8 → 8/8) is **NOT due to decoder changes** but due to **golden reference regeneration**:

1. **`nam_awip12_lambert_drt3.json`** regenerated at 2026-07-23 08:43 (1.16GB)
2. **`mrms_carib_refl_drt41.json`** regenerated at 2026-07-23 08:43 (80MB)

These golden references had stale metadata from incomplete decoder support. Regeneration with the correct DRT=3 and DRT=41 parsers eliminated the mismatches.

**Critical insight:** The decode error rate remained at 0% throughout. The "failures" were not decode failures but golden reference mismatches.

## DRT=3 Specific Validation

The DRT=3 fix (template 5.3) was the primary concern for this analysis. Results:

| Fixture Type | Baseline Status | Current Status | Change |
|--------------|-----------------|-----------------|--------|
| Lat/Lon DRT=3 (`gfs_tmp2m_1deg_anl`) | MATCH | MATCH | ✅ Stable |
| Lambert DRT=3 (`nam_awip12_lambert_drt3`) | META_MISMATCH | MATCH | ✅ Fixed (golden regenerated) |
| All DRT=3 Fields (197 total) | 0 decode errors | 0 decode errors | ✅ No regression |

## Test Execution Details

**Command:**
```bash
cargo test differential_coverage_report --test differential -- --nocapture
```

**Execution Time:** 21.47 seconds
**Output:** `/tmp/current_differential_test.txt`

**Test Environment:**
- All inline fixtures present
- No remote fixtures fetched
- JPEG2000 feature disabled (DRT=40 fixtures skipped)

## Conclusion

### Decode Error Rate Assessment
**✅ NO REGRESSION DETECTED**

The DRT=3 fix has **not introduced any decode error rate increases** across the fixture suite:
- All 202 fields across 8 comparable fixtures decode successfully
- 0 decode errors at baseline and 0 decode errors currently
- 0.0% error rate maintained across all template types

### Overall Health Assessment
**✅ IMPROVEMENT DETECTED**

The test suite shows improved health due to golden reference regeneration:
- Agreement increased from 75% to 100%
- All previously failing fixtures now pass (golden references corrected)
- No fixtures regressed from passing to failing

### Recommendations

1. **No action required** - decode error rates are stable at 0%
2. **Maintain golden reference freshness** - regenerate when decoder behavior changes
3. **Generate golden references** for 2 pending fixtures (`nam_awip12_lambert_drt3_20250120`, `hrrr_conus_drt3_lambert`)
4. **Enable JPEG2000 feature** - test DRT=40 fixtures for completeness

**Task Status:** ✅ COMPLETE - No unexplained error rate increases found
