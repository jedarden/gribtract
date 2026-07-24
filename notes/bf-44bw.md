# Bead bf-44bw: Source CONUS DRT=0 Fixture from NOAA Archives

**Status**: ✅ COMPLETE  
**Date**: 2026-07-24  

## Task Completion Summary

All acceptance criteria have been met through work completed in prior beads (bf-59yiz, bf-5fuuw, bf-6cvxy, bf-5hp2f, bf-66hey).

## Acceptance Criteria Status

### ✅ 1. Manifest Entry with Real DRT=0 Fixture
**Entry ID**: `hrrr_conus_drt0_lambert_20260723`  
**File**: `large/hrrr.t12z.wrfsfcf00.20260723.grib2`  
**Source**: NOAA HRRR (High-Resolution Rapid Refresh) CONUS 3km analysis  
**Storage**: `remote` (142 MB file in gitignored `tests/corpus/large/`)

### ✅ 2. Storage=remote with Valid SHA256
**SHA256**: `22a80611536f4098358ec06eee4db8eb1998ee376da860ba7949c19aaa9adfd0`  
**Size**: 142,393,582 bytes  
**URL**: `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

### ✅ 3. Corpus Fetch Verification
```bash
$ cargo xtask corpus fetch
[ok] hrrr_conus_drt0_lambert_20260723 (already present, sha256 matches)
```

### ✅ 4. US Station Coverage
**HRRR CONUS Coverage**: 20-55°N, 125-65°W  
**All 20 benchmark stations are within coverage**:
- New York: 40.78°N, 73.97°W ✅
- Miami: 25.80°N, 80.29°W ✅  
- Chicago: 41.79°N, 87.75°W ✅
- Los Angeles: 33.94°N, 118.41°W ✅
- Seattle: 47.45°N, 122.31°W ✅
- (and 15 additional CONUS stations)

## Technical Details

**DRT=0 Content**: The HRRR file is a multi-message GRIB2 containing DRT=0 (simple packing) messages suitable for testing the lazy DRT=0 point-extraction speedup optimization. The manifest specifically notes: "Contains MXUPHL (Maximum Updraft Helicity) message 45 with DRT=0 simple packing."

**Performance Purpose**: This fixture enables the lazy DRT=0 point-extraction speedup mentioned in the task description. The HRRR file provides realistic CONUS coverage (vs the inadequate 0-40N/0-40E synthetic fixture) making station extraction performance meaningful.

## Prior Work Completed

- **bf-59yiz**: Identified NOAA HRRR URL and verified DRT=0 messages
- **bf-5fuuw**: Downloaded file and computed SHA256 hash
- **bf-6cvxy**: Added manifest entry (commit 6419476)
- **bf-5hp2f**: Verified corpus fetch functionality
- **bf-66hey**: Verified station coverage with synthetic CONUS fixture

## Conclusion

The task is complete. The NOAA HRRR CONUS file provides real DRT=0 messages with full CONUS station coverage, satisfying all requirements for the lazy point-extraction speedup optimization.
