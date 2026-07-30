#!/bin/bash
# Script to check DRT (Data Representation Template) values for GRIB2 files
# Uses wgrib2 -Sec3 option to extract Grid Definition Template information

set -e

echo "=== DRT Value Check Report ==="
echo "Generated: $(date)"
echo "Command: wgrib2 -Sec3 <file> | grep -oP 'Grid Def Template=\K[0-9.]+(?= |$)'"
echo ""

# Function to check DRT for a single file
check_drt() {
    local file="$1"
    local basename=$(basename "$file")

    # Skip empty files (size = 0)
    if [ ! -s "$file" ]; then
        echo "SKIP: $basename (empty file)"
        return
    fi

    # Skip tiny files that are likely incomplete (less than 1KB)
    local size=$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null || echo "0")
    if [ "$size" -lt 1024 ]; then
        echo "SKIP: $basename (${size} bytes - too small, likely incomplete download)"
        return
    fi

    # Extract DRT values from all records and get unique values
    local drt_values
    drt_values=$(wgrib2 -Sec3 "$file" 2>/dev/null | grep -oP 'Grid Def Template=\K[0-9.]+' | sort -u | tr '\n' ',' | sed 's/,$//')

    if [ -z "$drt_values" ]; then
        echo "ERROR: $basename (could not extract DRT)"
    else
        echo "DRT($drt_values): $basename"
    fi
}

# Export function for use with find
export -f check_drt

echo "=== GFS Files ==="
find samples/grib2-noaa-gfs/ -type f -size +1k -name "gfs.*" -exec bash -c 'check_drt "$0"' {} \; 2>/dev/null || echo "No GFS files found"

echo ""
echo "=== HRRR Files ==="
find samples/grib2-noaa-hrrr/ -type f -size +1k -name "*.grib2" -exec bash -c 'check_drt "$0"' {} \; 2>/dev/null || echo "No HRRR files found"

echo ""
echo "=== NAM Files ==="
find samples/grib2-noaa-nam/ -type f -size +1k -name "*.grib2" -exec bash -c 'check_drt "$0"' {} \; 2>/dev/null || echo "No NAM files found"

echo ""
echo "=== RAP Files ==="
find samples/grib2-noaa-rap/ -type f -size +1k -name "*.grib2" -exec bash -c 'check_drt "$0"' {} \; 2>/dev/null || echo "No RAP files found"

echo ""
echo "=== Other GRIB2 Files in samples/ ==="
find samples/ -maxdepth 1 -type f -size +1k \( -name "*.grib2" -o -name "nam.*" -o -name "hrrr.*" \) -exec bash -c 'check_drt "$0"' {} \; 2>/dev/null || echo "No other GRIB2 files found"

echo ""
echo "=== Summary ==="
echo "DRT 0.0 = Latitude/Longitude grid"
echo "DRT 3.0 = Lambert Conformal Conic grid"
echo "DRT 3.30 = Lambert Conformal Conic grid (variant)"
echo "DRT 40.0 = Rotated Latitude/Longitude grid"
echo "Other values = specialized grid types"
echo ""

echo "Files with DRT=0.0 (simple lat/lon):"
find samples/ -type f -size +1k \( -name "*.grib2" -o -name "gfs.*" -o -name "nam.*" -o -name "hrrr.*" -o -name "rap.*" \) -exec bash -c 'wgrib2 -Sec3 "$0" 2>/dev/null | grep -q "Grid Def Template=0\.0" && echo "$0"' {} \; 2>/dev/null | xargs -r basename -a | sed 's/^/  /' || echo "  (none found)"

echo ""
echo "Files with DRT=3.0/3.30 (Lambert Conformal):"
drt3_count=$(find samples/ -type f -size +1k \( -name "*.grib2" -o -name "gfs.*" -o -name "nam.*" -o -name "hrrr.*" -o -name "rap.*" \) -exec bash -c 'wgrib2 -Sec3 "$0" 2>/dev/null | grep -q "Grid Def Template=3\." && echo "$0"' {} \; 2>/dev/null | wc -l)
find samples/ -type f -size +1k \( -name "*.grib2" -o -name "gfs.*" -o -name "nam.*" -o -name "hrrr.*" -o -name "rap.*" \) -exec bash -c 'wgrib2 -Sec3 "$0" 2>/dev/null | grep -q "Grid Def Template=3\." && echo "$0"' {} \; 2>/dev/null | xargs -r basename -a | sed 's/^/  /' | head -20
echo "  ... and $((drt3_count - 20)) more files" | sed "s/and \([0-9]*\) more files/and 0 more files/"

echo ""
echo "Total DRT Statistics:"
echo "  DRT 0.0 (Lat/Lon): $(find samples/ -type f -size +1k \( -name "*.grib2" -o -name "gfs.*" -o -name "nam.*" -o -name "hrrr.*" -o -name "rap.*" \) -exec bash -c 'wgrib2 -Sec3 "$0" 2>/dev/null | grep -q "Grid Def Template=0\.0" && echo "$0"' {} \; 2>/dev/null | wc -l) files"
echo "  DRT 3.x (Lambert): $drt3_count files"

echo ""
echo "Skipped files (empty/incomplete):"
find samples/ -type f ! -size +1k \( -name "*.grib2" -o -name "gfs.*" -o -name "nam.*" -o -name "hrrr.*" -o -name "rap.*" \) -exec basename {} \; 2>/dev/null | sed 's/^/  /' || echo "  (none found)"
