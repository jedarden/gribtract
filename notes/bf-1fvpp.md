# DRT Extraction Script Implementation

**Bead ID:** bf-1fvpp  
**Date:** 2026-07-24  
**Status:** ✅ Complete

## Task Completed

Created a DRT (Data Representation Type) extraction script that uses wgrib2 to extract DRT values from GRIB2 files.

## Implementation

### Script Created

**File:** `scripts/extract_drt.sh`

**Features:**
- Takes a GRIB2 file path as command-line argument
- Uses `wgrib2 -grid` to extract grid information
- Parses and extracts DRT values using grep with Perl regex
- Handles multiple DRT values (for multi-message files)
- Comprehensive error handling for:
  - Missing arguments
  - File not found
  - Non-readable files
  - Empty files
  - Invalid/corrupted GRIB2 files
  - Missing wgrib2 installation

**Key command:**
```bash
wgrib2 "$file" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+' | sort -u
```

### Testing Results

Successfully tested on multiple files:

| File | DRT Value | Status |
|------|-----------|--------|
| ndfd_temp.grib2 | 30 | ✅ Success |
| gfs_20260724_00z_1p00_f000.grib2 | 0 | ✅ Success |
| test.grib2 (empty) | N/A | ✅ Error handling |
| nonexistent.grib2 | N/A | ✅ Error handling |
| nam.20260724... (corrupted) | N/A | ✅ Error handling |

### Documentation Updated

- Updated `scripts/README.md` with DRT extraction section
- Added usage examples, error handling documentation
- Listed related scripts for batch processing

## Usage Examples

### Basic usage
```bash
./scripts/extract_drt.sh /path/to/file.grib2
# Output: DRT=30
```

### Batch processing
```bash
for file in samples/*.grib2; do
    echo "$file: $(./scripts/extract_drt.sh "$file")"
done
```

## Integration with Existing Work

This script complements existing DRT checking scripts:
- `check_drt_values.sh` - Python-based batch DRT checker (direct binary parsing)
- `check_drt_downloaded.sh` - Bash-based batch checker for downloaded files

The new script is designed for single-file extraction, making it ideal for:
- Manual inspection of individual files
- Integration into other scripts
- Quick DRT verification

## Technical Notes

### DRT Value Meaning

DRT (Data Representation Type) values indicate the grid definition template number used in GRIB2 Section 3. Common values:
- **0:** Latitude/Longitude grid
- **30:** Lambert Conformal Conic projection
- Other values indicate different grid types

### Why Use wgrib2?

While direct binary parsing (as in `check_drt_values.sh`) is possible, using wgrib2 provides:
- More robust handling of various GRIB2 formats
- Built-in validation of file structure
- Simpler maintenance (no binary format changes to track)
- Consistency with other GRIB2 processing tools

## Acceptance Criteria Met

✅ Working script that extracts DRT values from GRIB2 files  
✅ Script tested on multiple sample files (both successful and error cases)  
✅ Script file saved to `scripts/extract_drt.sh`  
✅ Documentation completed in `scripts/README.md`

## Files Modified

- `scripts/extract_drt.sh` (created)
- `scripts/README.md` (updated)
- `notes/bf-1fvpp.md` (created - this file)

## Related Work

- Bead bf-1jvhe: Complete DRT value checking for all GRIB2 files
- Bead bf-21wf9: wgrib2 installation verification
- Bead bf-z1tk3: GRIB2 sample download verification
