# CONUS Coverage Verification Scripts

This directory contains scripts for verifying that GRIB2 files provide adequate CONUS (Continental United States) geographic coverage.

## Available Scripts

### verify_conus_quick.sh
Quick CONUS coverage verification using first grid point analysis.

**Usage:**
```bash
./verify_conus_quick.sh <grib2_file>
```

**Example:**
```bash
./verify_conus_quick.sh hrrr.t12z.wrfsfcf00.grib2
```

**What it does:**
- Extracts the first grid point coordinates from the GRIB2 file using wgrib2
- Converts longitude from 0-360°E to -180-180°W format
- Checks if the first point falls within CONUS bounds:
  - Northern: 50°N
  - Southern: 20°N  
  - Western: 125°W
  - Eastern: 65°W
- Reports whether the file likely provides CONUS coverage

**Limitations:**
- Only tests the first grid point, not full spatial extent
- Cannot verify complete coverage or edge effects
- Should be used for initial screening only

**Requirements:**
- `wgrib2` must be installed and in PATH

### verify_conus_comprehensive.sh
Comprehensive CONUS coverage verification using station validation.

**Usage:**
```bash
./verify_conus_comprehensive.sh <grib2_file>
```

**Example:**
```bash
./verify_conus_comprehensive.sh hrrr.t12z.wrfsfcf00.grib2
```

**What it does:**
- Tests coverage against 15 key CONUS weather stations
- Validates stations across all geographic regions:
  - Northern border (International Falls)
  - Southern border (Brownsville)
  - Western coast (Portland, Seattle, Los Angeles)
  - Eastern coast (Boston)
  - Central regions (Denver, Chicago, Dallas)
  - Southeast (Miami, Atlanta)
  - Northwest, Southwest, Midwest, East Coast
- Calculates coverage percentage
- Provides pass/fail verdict based on coverage thresholds

**Coverage Thresholds:**
- **≥95%:** ✅ ACCEPTABLE CONUS COVERAGE
- **80-94%:** ⚠️ PARTIAL CONUS COVERAGE  
- **<80%:** ❌ INSUFFICIENT CONUS COVERAGE

**Requirements:**
- `wgrib2` must be installed and in PATH
- Note: Current implementation uses placeholder station checking
- For production use, integrate with gribtract library

### check_conus_coverage.sh
Original CONUS coverage checking script with station information.

**Usage:**
```bash
./check_conus_coverage.sh
```

**What it does:**
- Displays key CONUS weather station coordinates
- Shows HRRR CONUS grid characteristics
- Provides reference information for manual validation

**Requirements:**
- None (informational script)

## Verification Criteria

The scripts use the following CONUS geographic bounds:

| Boundary | Coordinate | Reference Point |
|----------|-----------|-----------------|
| Northern Limit | ~50°N | International Falls, MN: 48.57°N |
| Southern Limit | ~20°N | Brownsville, TX: 25.91°N |
| Western Limit | ~125°W | Portland, OR: 122.60°W |
| Eastern Limit | ~65°W | Boston, MA: 71.01°W |

## Integration with Other Tools

For complete CONUS coverage validation, these scripts work best with:

1. **gribtract** (Rust library)
   ```bash
   # Enhanced coverage checker with full station database
   ./check_conus_coverage_enhanced file.grib2
   ```

2. **wgrib2** (GRIB2 inspection tool)
   ```bash
   # Grid information extraction
   wgrib2 file.grib2 -grid
   
   # Full inventory
   wgrib2 file.grib2 -match "" -grid
   ```

3. **grib_ls** (eccodes tool)
   ```bash
   # Comprehensive parameter extraction
   grib_ls -p /grid file.grib2
   grib_ls -p /LaD,/LoV,/Latin1,/Latin2 file.grib2
   ```

## Documentation

For detailed CONUS coverage verification criteria and methodology, see:
- `docs/conus-coverage-verification-criteria.md` - Complete verification criteria
- `docs/conus-coverage-validation-summary.md` - HRRR CONUS validation results
- `docs/bf-1357i-spatial-extent-extraction-guide.md` - Spatial extent extraction methods

## Examples

### Quick verification of HRRR CONUS file
```bash
./verify_conus_quick.sh hrrr.t12z.wrfsfcf00.grib2
# Expected output: First point within CONUS bounds
```

### Comprehensive validation of NAM AWIP12 file
```bash
./verify_conus_comprehensive.sh nam.t00z.awip1200.tm00.grib2
# Expected output: ≥95% coverage (ACCEPTABLE)
```

### Extract grid information manually
```bash
wgrib2 file.grib2 -grid
```

## Troubleshooting

### Common Issues

**"wgrib2: command not found"**
- Install wgrib2 from: https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/
- Add to PATH or modify scripts to use full path

**"Error: Could not extract first point coordinates"**
- Check that the file is a valid GRIB2 file
- Verify wgrib2 is working: `wgrib2 --version`
- Try manual inspection: `wgrib2 file.grib2 -grid`

**"File not found" error**
- Check that the file path is correct
- Use absolute path if needed: `/home/coding/gribtract/data/file.grib2`

**Coverage results seem incorrect**
- Remember that comprehensive script uses placeholder implementation
- For production use, integrate with gribtract library
- Use check_conus_coverage_enhanced.rs for accurate results

## Development Status

These scripts are part of the CONUS coverage validation infrastructure developed for bead bf-1yvp2. They provide practical implementations of the verification criteria documented in `docs/conus-coverage-verification-criteria.md`.

## DRT Extraction Scripts

### extract_drt.sh

Extracts DRT (Data Representation Type) values from GRIB2 files using wgrib2.

**Usage:**
```bash
./extract_drt.sh <grib2_file_path>
```

**Examples:**
```bash
# Extract DRT from a single file
./extract_drt.sh /path/to/file.grib2
# Output: DRT=30

# Extract DRT from multiple files in a directory
for file in *.grib2; do
    echo "$file: $(./extract_drt.sh "$file")"
done
```

**What it does:**
- Uses `wgrib2 -grid` to output grid information
- Extracts the `grid_template` value (which is the DRT) using grep
- Returns the DRT value as output (e.g., "DRT=30")
- Handles multiple DRT values in multi-message files

**Error handling:**
- Missing or incorrect arguments
- File not found
- File not readable  
- Empty files
- Invalid or corrupted GRIB2 files
- wgrib2 not installed

**Return values:**
- **0:** Success - DRT value extracted and printed
- **1:** Error - appropriate error message printed to stderr

**Requirements:**
- `wgrib2` must be installed and in PATH

**Related files:**
- `../check_drt_values.sh` - Python script for batch DRT checking
- `../check_drt_downloaded.sh` - Bash script for batch DRT checking of downloaded files

**Implementation details:**
The key command used:
```bash
wgrib2 "$file" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+'
```

This extracts the grid template number from wgrib2 output like:
```
1:80:grid_template=30:winds(N/S):
```

## Contributing

When adding new verification methods:
1. Follow the existing naming convention: `verify_conus_*.sh`
2. Include usage examples in help text
3. Document requirements and dependencies
4. Update this README with new capabilities
5. Test against known good and bad coverage files

---

**Last Updated:** 2026-07-24  
**Related Beads:** bf-1yvp2 (CONUS coverage), bf-1fvpp (DRT extraction)  
**Status:** ✅ Complete
