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

## Contributing

When adding new verification methods:
1. Follow the existing naming convention: `verify_conus_*.sh`
2. Include usage examples in help text
3. Document requirements and dependencies
4. Update this README with new capabilities
5. Test against known good and bad coverage files

---

**Last Updated:** 2026-07-24  
**Bead ID:** bf-1yvp2  
**Status:** ✅ Complete
