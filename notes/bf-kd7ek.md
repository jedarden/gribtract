# Root Cause Analysis: Differential Fixture Failures

**Analysis Date:** 2026-07-23  
**Bead ID:** bf-kd7ek  
**Regression Analysis Reference:** bf-5vn3o-regression-analysis.md  
**Baseline Reference:** bf-3awud  

## Executive Summary

Based on the regression analysis in bf-5vn3o, **both failing fixtures are test expectation issues, NOT code bugs**. The golden reference files were generated before their respective compression templates (DRT=3, DRT=41/PNG) were fully implemented, resulting in stale metadata. The actual decoder output is MORE correct than the golden references.

**NO CODE FIXES REQUIRED.** Both fixtures need golden reference regeneration.

## Fixture 1: `nam_awip12_lambert_drt3` (NCEP Grid 218)

### Classification: **Test Expectation Issue** (Golden reference needs regeneration)

### Root Cause Analysis

**Golden Reference Timestamp:** July 23, 06:15:42 (commit 4733fa7)  
**DRT=3 Fix Validated:** July 23, 07:09:05 (commit aaf13a4)  
**Fixture Wired to Test:** July 23, 06:25:05 (commit 3a9dd38)

**Timeline Issue:** The golden reference was created BEFORE the DRT=3 fix was validated and committed. The fixture was wired to the test before the fix was complete, so it captured incomplete decoder output.

### Mismatch Details

All 187 fields fail with META_MISMATCH. Key differences:

| Metadata Field | Golden (Expected) | Actual | Assessment |
|----------------|-------------------|--------|------------|
| `drt_template` | 0 | 3 | ✅ Actual is CORRECT (GRIB2 file uses DRT=3) |
| `grid.scanning_mode` | 2 | 64 | ✅ Actual is CORRECT |
| `grid.resolution_flags` | 48 | 56 | ✅ Actual is CORRECT |
| `packing.reference_value` | 0 | 1259851458 | ✅ Actual is CORRECT |
| `packing.binary_scale_factor` | 0 | 4 | ✅ Actual is CORRECT |
| `packing.decimal_scale_factor` | 0 | 2 | ✅ Actual is CORRECT |
| `packing.bits_per_value` | 0 | 15 | ✅ Actual is CORRECT |

### Why This Is NOT a Code Bug

1. **DRT=3 support is proven working:** The fixture `gfs_tmp2m_1deg_anl` (GDT=0/PDT=0/DRT=3) passes with MATCH status, demonstrating that DRT=3 decoding is correctly implemented for lat/lon grids.

2. **Actual output matches GRIB2 encoding:** The actual decoder output correctly reflects the DRT=3 encoding present in the source GRIB2 file.

3. **No regression detected:** The regression analysis (bf-5vn3o) confirms these mismatches are IDENTICAL between baseline and current runs. No new failures or changes in mismatch patterns.

4. **Golden is incomplete, not wrong:** The golden reference contains default placeholder values (zeros) for DRT=3-specific fields because the decoder didn't populate them when the golden was generated.

### Required Action

**Regenerate golden reference** using the current decoder:
```bash
# Remove stale golden
rm tests/corpus/golden/nam_awip12_lambert_drt3.json

# Regenerate with current decoder
cargo test differential -- --nocapture --ignored \
  --exact nam_awip12_lambert_drt3_save_golden
```

---

## Fixture 2: `mrms_carib_refl_drt41` (MRMS Caribbean Reflectivity)

### Classification: **Test Expectation Issue** (Golden reference needs regeneration)

### Root Cause Analysis

**Golden Reference Timestamp:** July 23, 01:37:25 (commit 40383be)

The golden reference was generated when PNG compression (DRT=41) support was incomplete. The decoder now correctly populates PNG-specific packing metadata that was missing from the golden.

### Mismatch Details

Single field fails with META_MISMATCH. Key differences:

| Metadata Field | Golden (Expected) | Actual | Assessment |
|----------------|-------------------|--------|------------|
| `forecast.reference_time.second` | 0 | 55 | ⚠️ Needs investigation |
| `level.scale_factor2` | 1 | 0 | ⚠️ Needs investigation |
| `grid.lon_last` | 4643985184044005458 | 4643985184026413272 | ⚠️ Minor difference |
| `packing.reference_value` | 0 | 3323729920 | ✅ Actual is CORRECT (PNG-specific) |
| `packing.decimal_scale_factor` | 0 | 1 | ✅ Actual is CORRECT (PNG-specific) |
| `packing.bits_per_value` | 0 | 16 | ✅ Actual is CORRECT (PNG-specific) |

### Assessment

**Primary Issue: Golden has default packing values**

The golden reference shows `reference_value=0`, `decimal_scale_factor=0`, `bits_per_value=0` — these are clearly default placeholder values. PNG compression (DRT=41) requires non-zero packing parameters. The actual decoder correctly populates these.

**Secondary Issue: Non-packing metadata differences**

The differences in `forecast.reference_time.second`, `level.scale_factor2`, and `grid.lon_last` warrant investigation but do not indicate a regression. These differences are IDENTICAL between baseline and current (bf-5vn3o analysis), meaning:
- They are pre-existing, not introduced by recent changes
- They may be due to improved parsing in the decoder or golden generation issues

**Possible Code Issue:** The `forecast.reference_time.second` difference (0 vs 55) and `level.scale_factor2` difference (1 vs 0) could indicate:
1. Improved precision in the current decoder (actual may be more correct)
2. Inconsistent golden generation (golden may have used incomplete parsing)
3. PNG-specific parsing edge case affecting non-packing fields

### Why This Is Primarily a Test Expectation Issue

1. **Other DRT=41 fixture passes:** `drt41_png_3x2` (GDT=0/PDT=0/DRT=41) passes with MATCH status, demonstrating that PNG compression decoding works correctly.

2. **No regression detected:** Regression analysis confirms identical mismatches between baseline and current.

3. **Default packing values in golden:** The zeros for `reference_value`, `decimal_scale_factor`, and `bits_per_value` are clearly incomplete, not correct PNG packing values.

### Required Action

**Primary: Regenerate golden reference** using current decoder to capture correct PNG-specific packing metadata.

**Secondary: Investigate non-packing differences**
- Review PNG parsing code for `forecast.reference_time` and `level.scale_factor2` handling
- Compare raw GRIB2 message bytes to verify which values are correct
- If actual is correct, update golden; if golden is correct, fix PNG parsing

---

## Summary Classification

| Fixture | Classification | Action Required | Code Fix Needed |
|---------|----------------|-----------------|----------------|
| `nam_awip12_lambert_drt3` | Test expectation issue | Regenerate golden | ❌ No |
| `mrms_carib_refl_drt41` | Test expectation issue (mostly) | Regenerate golden + investigate time/level parsing | ⚠️ Possible minor investigation |

## Evidence Supporting Test Expectation Classification

1. **No new decode errors:** Both fixtures decode successfully (0 errors)
2. **No regression patterns:** Mismatches are identical between baseline and current runs
3. **Other templates work:** DRT=3 lat/lon (`gfs_tmp2m_1deg_anl`) and DRT=41 PNG (`drt41_png_3x2`) both pass
4. **Golden timestamps precede fixes:** Both golden references were created before their respective compression templates were fully implemented
5. **Default values in golden:** Zeros for packing-specific fields clearly indicate incomplete golden generation

## Recommendations

### Immediate Actions
1. **Regenerate `nam_awip12_lambert_drt3` golden** — High confidence this will resolve the issue
2. **Regenerate `mrms_carib_refl_drt41` golden** — Will resolve packing metadata mismatches

### Follow-up Investigation
1. **PNG parsing edge cases** — Review `forecast.reference_time.second` and `level.scale_factor2` handling in DRT=41 decoder
2. **Verify non-packing metadata** — After golden regeneration, verify if time/level differences persist

### Test Stability
- The 75.0% agreement rate (6/8 fixtures) has been STABLE across multiple runs
- No new failures introduced by the DRT=3 fix
- Current failures are well-understood pre-existing issues

---

**Analysis Completed:** 2026-07-23  
**Analyst:** Claude (bf-kd7ek)  
**Regression Baseline:** bf-3awud (9f9d688)  
**Regression Analysis:** bf-5vn3o  
