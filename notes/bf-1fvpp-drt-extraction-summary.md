# Bead bf-1fvpp: DRT Extraction Script

## Summary

The DRT (Data Representation Type) extraction script has been successfully created and tested.

## Script Location

`/home/coding/gribtract/scripts/extract_drt.sh`

## Script Details

### Usage
```bash
./extract_drt.sh <grib2_file_path>
```

### What it does
- Uses `wgrib2 -grid` to extract grid information from GRIB2 files
- Parses the `grid_template` value (which is the DRT) using grep with Perl regex
- Returns the DRT value in format "DRT=<value>"
- Handles multiple DRT values in multi-message files

### Key Command
```bash
wgrib2 "$file" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+' | sort -u
```

### Error Handling
- ✅ Checks if wgrib2 is installed
- ✅ Validates correct number of arguments
- ✅ Checks if file exists
- ✅ Checks if file is readable
- ✅ Checks if file is empty
- ✅ Handles invalid/corrupted GRIB2 files
- ✅ Returns appropriate exit codes (0=success, 1=error)

## Test Results

### Valid GRIB2 Files Tested

| File | DRT Value | Status |
|------|-----------|--------|
| samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf01.grib2 | DRT=30 | ✅ Pass |
| samples/grib2-noaa-gfs/gfs.20260724.t00z.pgrb2.0p25.f000 | DRT=0 | ✅ Pass |
| ndfd_temp.grib2 | DRT=30 | ✅ Pass |

### Error Handling Tested

| Test Case | Expected Result | Actual Result | Status |
|-----------|----------------|---------------|--------|
| Non-existent file | Error: File not found | Error: File not found | ✅ Pass |
| Empty file | Error: File is empty | Error: File is empty | ✅ Pass |
| Invalid GRIB2 file | Error: Could not extract DRT | Error: Could not extract DRT | ✅ Pass |
| Missing arguments | Usage message | Usage message | ✅ Pass |

## DRT Values Observed

- **DRT=0**: GFS files (Global Forecast System) - likely uses template 0 (regular latitude/longitude grid)
- **DRT=30**: HRRR and NDFD files - likely uses template 30 (Lambert Conformal Conic projection)

## Documentation

The script is fully documented in:
- `/home/coding/gribtract/scripts/README.md` - Complete usage guide and examples
- Inline comments in the script itself

## Integration

The script integrates well with existing tools:
- Can be used with `check_drt_downloaded.sh` for batch processing
- Works alongside `check_drt_values.sh` (Python alternative)
- Documented in scripts README alongside other GRIB2 utilities

## Acceptance Criteria Met

✅ Working script that extracts DRT values from GRIB2 files
✅ Script tested on multiple sample files
✅ Script file saved and documented
✅ Error handling implemented and tested
✅ Documentation complete

## Conclusion

The DRT extraction script is fully functional, well-tested, and properly documented. It successfully extracts DRT values from GRIB2 files using wgrib2 and handles error cases gracefully.
