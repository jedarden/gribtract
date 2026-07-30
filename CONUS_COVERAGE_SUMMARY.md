# CONUS Geographic Coverage Verification Summary

## Bead Context
- **Bead ID**: bf-1evex
- **Verification Date**: 2026-07-24
- **Purpose**: Verify CONUS geographic coverage for DRT=0 candidates

## CONUS Bounding Box Definition
- **Latitude Range**: 24.0°N to 50.0°N
- **Longitude Range**: 125.0°W to 67.0°W (235.0°E to 293.0°E in 0-360° notation)

## Verification Method
Used wgrib2 v3.1.3 to extract grid information:
```bash
wgrib2 -grid <candidate_file>
```

Extracted parameters:
- Grid dimensions (nx, ny)
- Latitude/longitude ranges and step sizes
- Grid cell counts covering CONUS extent
- Coverage percentage calculations

## Results Summary

### Overall Results
- **Total Candidates**: 7
- **Verified with CONUS Coverage**: 7 (100%)
- **Failed Verification**: 0
- **No CONUS Coverage**: 0

### Candidate Details

#### High Resolution (0.25°)
| Candidate | CONUS Cells | Total Cells | Coverage % | Lat Points | Lon Points |
|-----------|-------------|-------------|------------|------------|------------|
| gfs_0p25_20260723_f000 | 24,465 | 1,038,240 | 2.36% | 105 | 233 |

#### Medium Resolution (0.50°)
| Candidate | CONUS Cells | Total Cells | Coverage % | Lat Points | Lon Points |
|-----------|-------------|-------------|------------|------------|------------|
| gefs_0p50_20260724_f000 | 6,201 | 259,920 | 2.39% | 53 | 117 |
| gefs_0p50_20260724_f003 | 6,201 | 259,920 | 2.39% | 53 | 117 |
| gefs_0p50_20260724_f006 | 6,201 | 259,920 | 2.39% | 53 | 117 |
| gfs_0p50_20260724_f000 | 6,201 | 259,920 | 2.39% | 53 | 117 |

#### Standard Resolution (1.00°)
| Candidate | CONUS Cells | Total Cells | Coverage % | Lat Points | Lon Points |
|-----------|-------------|-------------|------------|------------|------------|
| gfs_1p00_20260724_f000 | 1,593 | 65,160 | 2.44% | 27 | 59 |
| gfs_1p00_20260723_f000 | 1,593 | 65,160 | 2.44% | 27 | 59 |

## Key Findings

### All Candidates Global Grid Coverage
- Every candidate uses a **global lat-lon grid** (90°N to 90°S, 0°E to 360°E)
- All successfully include the complete CONUS geographic extent
- Coverage percentages are consistent (2.36%-2.44%) as expected for global models

### Resolution Impact
- **0.25° resolution**: 105×233 CONUS grid points (24,465 cells)
- **0.50° resolution**: 53×117 CONUS grid points (6,201 cells)  
- **1.00° resolution**: 27×59 CONUS grid points (1,593 cells)

### Model Coverage
- **GFS (Global Forecast System)**: 4 candidates verified
- **GEFS (Global Ensemble Forecast System)**: 3 candidates verified

## Verification Status

### ✓ VERIFIED: All 7 Candidates
All DRT=0 candidates successfully verified to include full CONUS geographic coverage and are suitable for downstream processing requiring CONUS data.

## Files Generated
1. `verify_conus_coverage.py` - Verification script
2. `conus_coverage_verification.json` - Detailed verification results
3. `CONUS_COVERAGE_SUMMARY.md` - This summary document

## Next Steps
Candidates are ready for downstream processing:
- ✓ DRT=0 (Simple Packing) verified (bf-4wg4g)
- ✓ CONUS geographic coverage verified (bf-1evex)
- Ready for final integration testing and deployment
