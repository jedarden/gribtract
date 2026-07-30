# CONUS Coverage Verification Summary (Bead bf-1evex)

**Date:** 2026-07-24  
**Bead:** bf-1evex  
**Status:** ✅ COMPLETE

## Task Completion Summary

Successfully verified CONUS geographic coverage for all 7 DRT=0 candidates using wgrib2 grid analysis and manual CONUS grid point calculations.

## Results

### Verification Results
- **Total Candidates Analyzed:** 7 DRT=0 files
- **Candidates Passing CONUS Filter:** 7 (100%)
- **Candidates Failing CONUS Filter:** 0
- **Filter Criteria:** ≥50% CONUS coverage

### Key Findings
✅ **All 7 DRT=0 candidates provide COMPLETE CONUS coverage**
✅ **Grid Template 0 (Regular Latitude-Longitude) confirmed for all files**
✅ **Global grid coverage naturally includes CONUS as subset**
✅ **No geographic filtering required - 100% pass rate**
✅ **CONUS bounds verified: 24°N-50°N, 125°W-67°W (235°E-293°E)**

### CONUS Coverage by Resolution

**0.25° Resolution (1 file):**
- Global grid: 1,440×721 = 1,038,240 points
- CONUS points: 24,465 (105 lat × 233 lon)
- Coverage: 2.36% of global grid
- File: gfs_0p25_20260723_f000.grib2

**0.50° Resolution (4 files):**
- Global grid: 720×361 = 259,920 points
- CONUS points: 6,201 (53 lat × 117 lon)
- Coverage: 2.38% of global grid
- Files: gfs_0p50_20260724_f000.grib2, gefs_0p50_f000.grib2, gefs_0p50_f003.grib2, gefs_0p50_f006.grib2

**1.00° Resolution (2 files):**
- Global grid: 360×181 = 65,160 points
- CONUS points: 1,593 (27 lat × 59 lon)
- Coverage: 2.44% of global grid
- Files: gfs_1p00_20260724_f000.grib2, gfs_1p00_20260723_f000.grib2

## Files Generated

1. **drt_verification/conus_coverage_report.md** - Comprehensive verification report
2. **drt_verification/conus_coverage_summary.json** - Machine-readable results summary
3. **notes/bf-1evex-conus-verification-summary.md** - This summary document

## Methodology

### Tools Used
- **wgrib2** - Grid definition extraction
- **Manual calculation** - CONUS grid point computation
- **CONUS bounds** - 24°N-50°N, 125°W-67°W (235°E-293°E)

### Process
1. Extracted grid definitions using wgrib2 -grid
2. Parsed grid dimensions, lat/lon ranges, and spacing
3. Calculated CONUS grid points based on resolution
4. Computed coverage percentages
5. Verified geographic bounds inclusion
6. Filtered and documented results

## Acceptance Criteria Status

✅ **Run wgrib2 -grid on each DRT=0 header file** - Completed
✅ **Extract grid definition parameters** - Completed
✅ **Verify coverage includes CONUS extent** - Completed
✅ **Filter candidates covering full/majority CONUS** - Completed
✅ **Document geographic bounds and coverage percentage** - Completed

## Recommendations

**For Production:** Use GFS 0.50° files (6,201 CONUS points, optimal balance)
**For Development:** Use GFS 1.00° files (1,593 CONUS points, fastest downloads)
**For Research:** Use GFS 0.25° files (24,465 CONUS points, maximum detail)

## Next Steps

All 7 DRT=0 candidates are ready for CONUS DRT=0 tool development. No geographic exclusions needed. The verification confirms that global GRIB2 grids provide complete CONUS coverage naturally.

---

**Bead bf-1evex completed successfully on 2026-07-24**
**CONUS coverage verification: 100% complete, all candidates pass**