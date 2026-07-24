# DRT=0 Verification Results - Major Discrepancy Found

**Date**: 2026-07-24  
**Bead**: bf-ow25s  
**Task**: Verify DRT=0 packing for candidate files

## Critical Finding: Previous Documentation Was Incorrect

The previous verification in `VERIFIED_DRT0_CONUS_FILES.md` claimed that 7 files were "DRT=0 (Simple Packing)". This verification using wgrib2 has proven that claim to be **incorrect**.

## Actual DRT Analysis of Candidate Files

### GFS Files - Mixed Packing (NOT Pure DRT=0)
| File | Total Records | DRT=0 Records | DRT=3 Records | Primary Packing |
|------|--------------|---------------|---------------|-----------------|
| gfs_1p00_20260724_f000.grib2 | 696 | 1 (0.14%) | 695 (99.86%) | Complex |
| gfs_0p25_20260723_f000.grib2 | 696 | 1 (0.14%) | 695 (99.86%) | Complex |
| gfs_1p00_20260723_f000.grib2 | 696 | 1 (0.14%) | 695 (99.86%) | Complex |
| gfs_0p50_20260724_f000.grib2 | 696 | 1 (0.14%) | 695 (99.86%) | Complex |

**The one DRT=0 record**: Record #205 is "CLMR:50 mb" (climatological moisture at 50 millibars)

### GEFS Files - 100% Complex Packing (NO DRT=0)
| File | Total Records | DRT=0 Records | DRT=3 Records | Packing Type |
|------|--------------|---------------|---------------|---------------|
| gefs_0p50_f000.grib2 | 71 | 0 (0%) | 71 (100%) | Complex |
| gefs_0p50_f003.grib2 | 85 | 0 (0%) | 85 (100%) | Complex |
| gefs_0p50_f006.grib2 | 85 | 0 (0%) | 85 (100%) | Complex |

### Failed Downloads
| File | Status |
|------|--------|
| gfs_0p25_20260724_f000.grib2 | 0 bytes - failed download |
| gfs_0p50_20260723_f000.grib2 | 0 bytes - failed download |

## Pure DRT=0 Files Found

Only **1 pure DRT=0 file** was found in the entire repository:
- `tests/corpus/small/conus_drt0.grib2` - 4KB test file with 1 record

## Technical Details

### DRT (Data Representation Template) Types Found
- **DRT=5.0**: Simple packing - no spatial differencing, easiest to decode
- **DRT=5.3**: Complex packing + spatial differencing - requires additional decoding steps

### wgrib2 Commands Used
```bash
# Check all DRT values
wgrib2 <file> -Sec5 | grep "Data Repr. Template"

# Count DRT occurrences  
wgrib2 <file> -Sec5 | grep -o "Data Repr. Template=5\.[0-9]*" | sort | uniq -c

# Packing details
wgrib2 <file> -packing
```

## Impact on Downstream Processing

If pure DRT=0 (simple packing) is a requirement:
- ❌ **NONE of the GFS/GEFS operational files are suitable**
- These files are 99.86% complex packing (DRT=5.3)
- Only 0.14% of records use simple packing
- Complex packing requires spatial differencing decode

## Recommendations

1. **Update requirements** - Can DRT=3 complex packing be acceptable?
2. **Implement DRT=3 decoder** - Required to process 99.86% of operational data
3. **Search alternative sources** - Historical datasets or different models may use pure DRT=0
4. **Consider HRRR** - 32/170 records (~19%) are DRT=0, better than GFS/GEFS

## Verification Methodology

This verification used:
- **wgrib2 v3.1.3** for DRT analysis
- **-Sec5 flag** for Data Representation Template extraction
- **-packing flag** for packing type confirmation
- **Complete file analysis** - checked ALL records, not just samples

## Conclusion

The previous claim that "7 files use DRT=0 (Simple Packing)" was **incorrect**. The actual findings:

- ❌ **0 pure DRT=0 operational files** found in candidates
- ❌ **4 GFS files** with 99.86% complex packing
- ❌ **3 GEFS files** with 100% complex packing  
- ❌ **2 failed downloads** (0-byte files)

**No pure DRT=0 CONUS candidate files exist** in the current dataset.
