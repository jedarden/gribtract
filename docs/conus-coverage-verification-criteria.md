# CONUS Geographic Coverage Verification Criteria

**Document Version:** 1.0  
**Created:** 2026-07-24  
**Bead Reference:** bf-1yvp2  
**Related:** `conus-coverage-validation-summary.md`, `bf-1357i-spatial-extent-extraction-guide.md`

## Overview

This document establishes the criteria for verifying that a GRIB2 file provides adequate CONUS (Continental United States) geographic coverage. It defines the spatial bounds, extraction methods, and verification procedures for validating CONUS coverage from GRIB2 metadata.

## CONUS Geographic Bounds Definition

### Standard CONUS Bounds

**Primary CONUS Coverage Criteria:**

| Boundary | Coordinate | Description | Reference Point |
|----------|-----------|-------------|-----------------|
| **Northern Limit** | ~50°N | Canada-USA border region | International Falls, MN: 48.57°N |
| **Southern Limit** | ~20°N | Mexico-USA border region | Brownsville, TX: 25.91°N |
| **Western Limit** | ~125°W | Pacific Coast | Portland, OR: 122.60°W |
| **Eastern Limit** | ~65°W | Atlantic Coast | Boston, MA: 71.01°W |

**Core CONUS Area:**
- **Latitude Range:** 20°N to 50°N (30° span, ~3,300 km)
- **Longitude Range:** 125°W to 65°W (60° span at mid-latitudes, ~5,300 km)
- **Coverage Area:** ~8.1 million km²

### Geographic Coverage Zones

**Primary Coverage Zone (100% required):**
- All 48 contiguous states
- Major population centers
- Key transportation hubs (airports, highways)
- Significant geographic features (mountains, rivers, coastlines)

**Buffer Zone (optional but recommended):**
- Coastal waters (50-100 km buffer)
- Border regions (50 km buffer into Canada/Mexico)
- Great Lakes region

## Extracting Geographic Coverage from GRIB2 Metadata

### Method 1: Using wgrib2 (Recommended for Quick Checks)

```bash
# Get grid definition information
wgrib2 file.grib2 -grid

# Example output for HRRR CONUS:
# Lambert Conformal: (1799 x 1059) input WE:SN output WE:SN res 8
# Lat1 21.138123 Lon1 237.280472 LoV 262.500000
# LatD 38.500000 Latin1 38.500000 Latin2 38.500000
# Dx 3000.000000 m Dy 3000.000000 m
```

**Key Parameters to Extract:**
- `Grid Template`: Projection type (30 = Lambert Conformal Conic)
- `Lat1/Lon1`: First grid point (southwest corner)
- `LoV`: Longitude of projection center
- `LaD`: Latitude of projection center
- `Latin1/Latin2`: Standard parallels
- `Dx/Dy`: Grid spacing in meters
- `Nx/Ny`: Grid dimensions

### Method 2: Using grib_ls (Full Parameter Access)

```bash
# Get comprehensive grid information
grib_ls -p gridType,latitudeOfFirstGridPointInDegrees,longitudeOfFirstGridPointInDegrees,Ni,Nj,DxInMetres,DyInMetres file.grib2

# For Lambert Conformal grids, get projection parameters
grib_ls -p LaD,LoV,Latin1,Latin2 file.grib2

# Get grid type
grib_ls -p gridType file.grib2
```

### Method 3: Using Python with pyproj (Most Accurate for Projected Grids)

```python
from pyproj import Transformer
import subprocess

def extract_grid_parameters(grib_file):
    """Extract grid parameters using wgrib2"""
    result = subprocess.run(
        ['wgrib2', grib_file, '-grid'],
        capture_output=True, text=True
    )
    # Parse output to extract parameters
    params = parse_wgrib2_output(result.stdout)
    return params

def calculate_conus_extent(params):
    """Calculate actual geographic extent for Lambert Conformal grids"""
    
    # Create Lambert Conformal CRS from GRIB parameters
    proj4_string = (
        f"+proj=lcc "
        f"+lat_1={params['Latin1']} +lat_2={params['Latin2']} "
        f"+lat_0={params['LaD']} +lon_0={params['LoV']-360} "
        f"+x_0=0 +y_0=0 "
        f"+a=6371229 +b=6371229 "
        f"+units=m +no_defs"
    )
    
    from pyproj import CRS
    crs = CRS.from_proj4(proj4_string)
    transformer = Transformer.from_crs(crs, "EPSG:4326")
    
    # Calculate corner points
    corners = {
        'sw': (0, 0),
        'se': (params['Ni']-1, 0),
        'nw': (0, params['Nj']-1),
        'ne': (params['Ni']-1, params['Nj']-1)
    }
    
    extent = {}
    for corner, (i, j) in corners.items():
        x = i * params['Dx']
        y = j * params['Dy']
        lon, lat = transformer.transform(x, y)
        extent[corner] = (lat, lon)
    
    return extent

# Usage
params = extract_grid_parameters('file.grib2')
extent = calculate_conus_extent(params)
print(f"Extent: {extent}")
```

## CONUS Coverage Verification Commands

### Quick Coverage Check (First Point Analysis)

```bash
#!/bin/bash
# Quick CONUS coverage verification using first grid point

grib_file=$1

# Extract first point coordinates
first_point=$(wgrib2 "$grib_file" -grid | grep "Lat1\|Lon1" | head -2)

# Parse coordinates (example parsing)
lat1=$(echo "$first_point" | grep Lat1 | awk '{print $2}')
lon1=$(echo "$first_point" | grep Lon1 | awk '{print $2}')

# Convert longitude from 0-360°E to -180-180°W
if [ "$lon1" -gt 180 ]; then
    lon1_w=$((lon1 - 360))
else
    lon1_w=$lon1
fi

echo "First grid point: ${lat1}°N, ${lon1_w}°W"

# Check against CONUS bounds
if (( $(echo "$lat1 >= 20 && $lat1 <= 50" | bc -l) )) && \
   (( $(echo "$lon1_w >= -125 && $lon1_w <= -65" | bc -l) )); then
    echo "✓ First point within CONUS bounds"
else
    echo "✗ First point outside CONUS bounds"
    exit 1
fi
```

### Comprehensive Coverage Check Using Station Validation

```bash
#!/bin/bash
# Station-based CONUS coverage validation

# Key CONUS verification stations
stations=(
    "48.57:-93.39:INL:Northern"     # International Falls, MN
    "25.91:-97.43:BRO:Southern"     # Brownsville, TX  
    "122.60:-45.59:PDX:Western"     # Portland, OR
    "71.01:-42.36:BOS:Eastern"      # Boston, MA
    "39.85:-104.67:DEN:Central"     # Denver, CO
    "25.79:-80.23:MIA:Southeast"    # Miami, FL
    "47.45:-122.31:SEA:Northwest"   # Seattle, WA
)

echo "Testing CONUS station coverage..."
grib_file=$1

covered=0
total=${#stations[@]}

for station in "${stations[@]}"; do
    IFS=':' read -r lat lon code region <<< "$station"
    
    # Use gribtract or custom tool to check if station is covered
    if check_station_coverage "$grib_file" "$lat" "$lon"; then
        echo "✓ $code ($region): COVERED"
        ((covered++))
    else
        echo "✗ $code ($region): NOT COVERED"
    fi
done

coverage_percent=$((covered * 100 / total))
echo ""
echo "Coverage: $covered/$total stations ($coverage_percent%)"

if [ $coverage_percent -ge 95 ]; then
    echo "✅ ACCEPTABLE CONUS COVERAGE"
    exit 0
elif [ $coverage_percent -ge 80 ]; then
    echo "⚠️  PARTIAL CONUS COVERAGE"
    exit 0
else
    echo "❌ INSUFFICIENT CONUS COVERAGE"
    exit 1
fi
```

### Grid Extent Calculation (Using wgrib2)

```bash
#!/bin/bash
# Calculate grid extent from GRIB2 metadata

grib_file=$1

# Extract grid parameters
wgrib2 "$grib_file" -grid > grid_info.txt

# Parse key parameters
grid_template=$(grep "grid_template" grid_info.txt | awk '{print $2}')
nx=$(grep "input WE" grid_info.txt | awk '{print $2}')
ny=$(grep "input WE" grid_info.txt | awk '{print $4}')

lat1=$(grep "Lat1" grid_info.txt | awk '{print $2}')
lon1=$(grep "Lon1" grid_info.txt | awk '{print $2}')

echo "Grid Template: $grid_template"
echo "Grid Dimensions: $nx x $ny"
echo "First Point: ${lat1}°N, ${lon1}°E"

# For Lambert Conformal (template 30), calculate extent
if [ "$grid_template" = "30" ]; then
    echo "Lambert Conformal grid detected"
    echo "Approximate CONUS extent calculation required"
    
    # Extract projection parameters
    lov=$(grep "LoV" grid_info.txt | awk '{print $2}')
    lad=$(grep "LatD" grid_info.txt | awk '{print $2}')
    
    echo "Projection center: ${lad}°N, ${lov}°E"
    
    # Calculate rough extent (simplified)
    # This is approximate - use pyproj for accurate results
    echo "For accurate extent, use pyproj with projection parameters"
fi
```

## Verification Criteria and Thresholds

### Coverage Standards

**Full CONUS Coverage (REQUIRED):**
- ✓ All 8 geographic regions covered (Northeast, Southeast, Midwest, South Central, Northwest, Southwest, Mountain, Central)
- ✓ ≥95% of test stations covered
- ✓ Northern coverage to ≥48°N
- ✓ Southern coverage to ≤26°N  
- ✓ Western coverage to ≥122°W
- ✓ Eastern coverage to ≤71°W

**Partial CONUS Coverage (ACCEPTABLE with caveats):**
- ⚠️ 80-94% of test stations covered
- ⚠️ Minor coverage gaps in less critical areas
- ⚠️ Coastal buffer zones may be missing

**Insufficient Coverage (REJECTED):**
- ✗ <80% of test stations covered
- ✗ Major geographic regions missing
- ✗ Central CONUS coverage gaps
- ✗ Key population centers not covered

### Grid Quality Thresholds

**Resolution Requirements:**
- **High Resolution:** ≤3 km (ideal for local applications)
- **Medium Resolution:** 3-13 km (acceptable for regional applications)
- **Low Resolution:** >13 km (may be insufficient for detailed applications)

**Coverage Completeness:**
- **Complete:** All CONUS regions + 50-100 km buffer
- **Adequate:** All CONUS regions, minimal buffer
- **Marginal:** Major coverage gaps or edge effects

## Edge Cases and Partial Coverage Scenarios

### Common Edge Cases

**1. Coastal Edge Proximity**
- **Description:** Coastal stations near grid boundaries
- **Impact:** May have reduced data quality due to interpolation
- **Acceptance:** Generally acceptable if ≤27% of stations are marginal
- **Example:** HRRR CONUS has 15 marginal coastal stations out of 56 total

**2. Lambert Conformal Grid Distortion**
- **Description:** Distance distortion increases away from standard parallels
- **Impact:** Grid spacing varies geographically
- **Acceptance:** Normal for projected grids, accounted for in GRIB2 design
- **Mitigation:** Use proper projection libraries for coordinate transformation

**3. Peninsula Coverage**
- **Description:** Coverage of Florida Peninsula and similar features
- **Impact:** May have reduced buffer on eastern/southern edges
- **Acceptance:** Acceptable if core peninsula is covered
- **Example:** HRRR covers Florida but Miami/Fort Lauderdale are marginal

**4. Border Region Coverage**
- **Description:** Northern and southern border regions
- **Impact:** May have limited buffer beyond borders
- **Acceptance:** Fully acceptable for CONUS-only applications
- **Example:** International Falls, MN (48.57°N) and Brownsville, TX (25.91°N) covered

### Partial Coverage Scenarios

**Scenario 1: Western CONUS Coverage**
- **Coverage Area:** West of 95°W (Mountain/Pacific regions)
- **Use Case:** Western weather applications, fire weather, western water resources
- **Verification Criteria:**
  - Coverage from Pacific Coast to ~95°W
  - Northern coverage to ≥48°N
  - Southern coverage to ≤30°N
  - All western stations covered

**Scenario 2: Eastern CONUS Coverage**
- **Coverage Area:** East of 95°W (Midwest/East Coast)
- **Use Case:** Eastern population centers, transportation hubs
- **Verification Criteria:**
  - Coverage from Atlantic Coast to ~95°W
  - Northern coverage to ≥48°N
  - Southern coverage to ≤25°N
  - All eastern stations covered

**Scenario 3: Central CONUS Coverage**
- **Coverage Area:** 95°W to 105°W (agricultural belt)
- **Use Case:** Agricultural applications, tornado alley, central transportation
- **Verification Criteria:**
  - Coverage focused on central states
  - All major central cities covered (Chicago, St. Louis, Dallas, Denver)
  - Adequate coverage for intended application

## Example Verification Workflows

### Example 1: Verify HRRR CONUS File

```bash
# Step 1: Get grid information
wgrib2 hrrr.t12z.wrfsfcf00.grib2 -grid > hrrr_grid.txt

# Step 2: Verify grid characteristics
grep -q "grid_template=30" hrrr_grid.txt && echo "✓ Lambert Conformal"
grep -q "1799 x 1059" hrrr_grid.txt && echo "✓ Expected dimensions"
grep -q "3000.000000 m" hrrr_grid.txt && echo "✓ 3km resolution"

# Step 3: Check first point within CONUS bounds
lat1=$(grep "Lat1" hrrr_grid.txt | awk '{print $2}')
lon1=$(grep "Lon1" hrrr_grid.txt | awk '{print $2}')

# Step 4: Run station validation
./check_conus_coverage_enhanced hrrr.t12z.wrfsfcf00.grib2

# Expected output: 56/56 stations covered (100%)
```

### Example 2: Verify NAM AWIP12 File

```bash
# Step 1: Extract grid parameters
wgrib2 nam.t00z.awip1200.tm00.grib2 -grid > nam_grid.txt

# Step 2: Verify projection
grep -q "Lambert Conformal" nam_grid.txt && echo "✓ Lambert Conformal"

# Step 3: Check coverage against CONUS bounds
# First point: 12.19°N, 133.46°W
# This is south/west of core CONUS but covers full extent

# Step 4: Validate station coverage
./check_conus_coverage_enhanced nam.t00z.awip1200.tm00.grib2

# Expected: Full CONUS coverage with larger buffer
```

### Example 3: Automated Verification Script

```bash
#!/bin/bash
# Automated CONUS coverage verification for GRIB2 files

verify_conus_coverage() {
    local grib_file=$1
    local result_file="coverage_report_$(basename $grib_file).txt"
    
    echo "Verifying CONUS coverage for: $grib_file"
    echo "=============================================="
    
    # Extract grid information
    wgrib2 "$grib_file" -grid > temp_grid.txt
    
    # Check grid template
    grid_template=$(grep "grid_template" temp_grid.txt | awk '{print $2}')
    echo "Grid Template: $grid_template"
    
    # Extract first point
    lat1=$(grep "Lat1" temp_grid.txt | awk '{print $2}')
    lon1=$(grep "Lon1" temp_grid.txt | awk '{print $2}')
    echo "First Point: ${lat1}°N, ${lon1}°E"
    
    # Run station validation
    echo ""
    echo "Station Coverage Test:"
    echo "======================"
    
    if check_conus_coverage_enhanced "$grib_file" > "$result_file" 2>&1; then
        coverage="PASS"
        echo "✅ CONUS COVERAGE VERIFIED"
    else
        coverage="FAIL"
        echo "❌ CONUS COVERAGE FAILED"
    fi
    
    echo ""
    echo "Full report saved to: $result_file"
    return $([ "$coverage" = "PASS" ] && echo 0 || echo 1)
}

# Usage
verify_conus_coverage "$1"
```

## Tools and Dependencies

### Required Tools

**wgrib2** (Primary tool for GRIB2 inspection)
```bash
# Installation
# Download from: https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/

# Basic usage
wgrib2 file.grib2 -grid     # Grid information
wgrib2 file.grib2 -match "" -grid   # All grid definitions
```

**grib_ls** (Alternative for comprehensive parameter access)
```bash
# Part of eccodes library
# Installation: sudo apt-get install eccodes

# Usage
grib_ls -p /grid file.grib2
grib_ls -p /LaD,/LoV,/Latin1,/Latin2 file.grib2
```

**pyproj** (Python library for accurate projection calculations)
```bash
# Installation
pip install pyproj

# Python script for accurate extent calculation
python calculate_extent.py file.grib2
```

### Verification Tools

**gribtract** (Rust library for GRIB2 processing)
```bash
# Enhanced coverage checker
rustc --edition=2021 check_conus_coverage_enhanced.rs \
  -L target/release/deps \
  --extern gribtract=target/release/libgribtract.rlib

./check_conus_coverage_enhanced file.grib2
```

## Troubleshooting

### Common Issues

**Issue 1: Longitude Coordinate Confusion**
- **Symptom:** Coordinates appear as 237°E instead of 122°W
- **Solution:** Convert from 0-360°E to -180-180°W (subtract 360 if >180°)
- **Command:** `lon_w=$((lon_e - 360))`

**Issue 2: Lambert Extent Calculation**
- **Symptom:** Cannot determine exact geographic bounds from grid metadata
- **Solution:** Use pyproj with proper Lambert Conformal parameters
- **Note:** First point alone is insufficient for projected grids

**Issue 3: Grid Scanning Order**
- **Symptom:** Coordinates appear reversed or unexpected
- **Solution:** Check scanning mode in grid definition (usually +i +j west-to-east, south-to-north)
- **Command:** `wgrib2 file.grib2 -grid | grep "scan"`

**Issue 4: Station Coverage Failures**
- **Symptom:** Known CONUS stations appear uncovered
- **Possible Causes:**
  - Incorrect longitude coordinate format (0-360°E vs -180-180°W)
  - Grid projection calculation errors
  - Actual coverage gaps in dataset
- **Solution:** Verify coordinate format and grid extent calculation

### Validation Checklist

Before accepting CONUS coverage:
- [ ] Grid template identified (30 = Lambert Conformal, 0 = Regular LL)
- [ ] First point coordinates extracted and converted correctly
- [ ] Grid dimensions and spacing verified
- [ ] Station coverage test run (≥95% coverage target)
- [ ] Geographic extent calculated (for projected grids)
- [ ] Edge cases documented (coastal stations, border regions)
- [ ] Coverage report generated and archived

## Conclusion

These CONUS coverage verification criteria provide a comprehensive framework for validating that GRIB2 files adequately cover the Continental United States. The combination of:

1. **Clear geographic bounds** (20-50°N, 125-65°W)
2. **Multiple extraction methods** (wgrib2, grib_ls, pyproj)
3. **Station-based validation** (56 test stations across 9 regions)
4. **Automated verification tools** (coverage check scripts)
5. **Edge case handling** (coastal proximity, partial coverage)

provides robust validation for CONUS coverage in GRIB2 files.

## Related Documentation

- **Detailed validation results:** `conus-coverage-validation-summary.md`
- **Spatial extent extraction:** `bf-1357i-spatial-extent-extraction-guide.md`
- **Grid definition reference:** `bf-1357i-grid-definition-reference.md`
- **Coverage validation tools:** `check_conus_coverage_enhanced.rs`
- **Station database:** Embedded in coverage checker scripts

---

**Document Status:** ✅ COMPLETE  
**Last Updated:** 2026-07-24  
**Next Review:** When new GRIB2 products are added to validation suite
