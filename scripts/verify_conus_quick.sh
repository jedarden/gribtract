#!/bin/bash
# Quick CONUS coverage verification using first grid point analysis
# Usage: ./verify_conus_quick.sh <grib2_file>

set -e

grib_file="$1"

if [ -z "$grib_file" ]; then
    echo "Usage: $0 <grib2_file>"
    echo "Example: $0 hrrr.t12z.wrfsfcf00.grib2"
    exit 1
fi

if [ ! -f "$grib_file" ]; then
    echo "Error: File not found: $grib_file"
    exit 1
fi

echo "========================================"
echo "Quick CONUS Coverage Verification"
echo "========================================"
echo "File: $grib_file"
echo ""

# Extract first point coordinates using wgrib2
echo "Extracting grid information..."
wgrib2 "$grib_file" -grid 2>/dev/null > temp_grid_$$.txt || {
    echo "Error: Failed to extract grid information with wgrib2"
    rm -f temp_grid_$$.txt
    exit 1
}

# Parse first point coordinates - Lat1 and Lon1 are on the same line in wgrib2 output
# Format: "Lat1 21.138123 Lon1 237.280472 LoV 262.500000"
grid_line=$(grep "Lat1.*Lon1.*LoV" temp_grid_$$.txt | head -1)
lat1=$(echo "$grid_line" | awk '{print $2}')
lon1=$(echo "$grid_line" | awk '{print $4}')

# Clean up temp file
rm -f temp_grid_$$.txt

if [ -z "$lat1" ] || [ -z "$lon1" ]; then
    echo "Error: Could not extract first point coordinates"
    exit 1
fi

# Convert longitude from 0-360°E to -180-180°W format
# GRIB2 typically stores longitude as 0-360°E
# Example: 237.28°E = 122.72°W (237.28 - 360 = -122.72)
lon_decimal=$(echo "$lon1" | awk '{printf "%.2f", $1}')
lon_w=$(echo "$lon1" | awk '{if ($1 > 180) printf "%.2f", $1 - 360; else printf "%.2f", $1}')

echo "First grid point coordinates:"
echo "  Latitude: ${lat1}°N"
echo "  Longitude: ${lon1}°E (${lon_w}°W)"
echo ""

# Define CONUS bounds
CONUS_NORTH=50.0
CONUS_SOUTH=20.0
CONUS_WEST=-125.0
CONUS_EAST=-65.0

echo "CONUS coverage bounds:"
echo "  Northern: ${CONUS_NORTH}°N"
echo "  Southern: ${CONUS_SOUTH}°N"
echo "  Western: ${CONUS_WEST}°W"
echo "  Eastern: ${CONUS_EAST}°W"
echo ""

# Check if first point is within CONUS bounds
# Use awk for floating point comparison instead of bc
lat_in_range=$(awk "BEGIN {print ($lat1 >= $CONUS_SOUTH && $lat1 <= $CONUS_NORTH) ? 1 : 0}")
lon_in_range=$(awk "BEGIN {print ($lon_w >= $CONUS_WEST && $lon_w <= $CONUS_EAST) ? 1 : 0}")

echo "Coverage analysis:"
if [ "$lat_in_range" = "1" ]; then
    echo "  ✓ Latitude ${lat1}°N is within CONUS range (${CONUS_SOUTH}-${CONUS_NORTH}°N)"
else
    echo "  ✗ Latitude ${lat1}°N is outside CONUS range (${CONUS_SOUTH}-${CONUS_NORTH}°N)"
fi

if [ "$lon_in_range" = "1" ]; then
    echo "  ✓ Longitude ${lon_w}°W is within CONUS range (${CONUS_WEST}-${CONUS_EAST}°W)"
else
    echo "  ✗ Longitude ${lon_w}°W is outside CONUS range (${CONUS_WEST}-${CONUS_EAST}°W)"
fi
echo ""

# Final verdict
if [ "$lat_in_range" = "1" ] && [ "$lon_in_range" = "1" ]; then
    echo "=========================================="
    echo "✅ RESULT: First point within CONUS bounds"
    echo "=========================================="
    echo "This file likely provides CONUS coverage."
    echo "For comprehensive validation, use station-based testing."
    exit 0
else
    echo "=========================================="
    echo "❌ RESULT: First point outside CONUS bounds"
    echo "=========================================="
    echo "This file may not provide adequate CONUS coverage."
    echo "Note: Some grids may start outside CONUS but still cover it fully."
    echo "For comprehensive validation, use station-based testing."
    exit 0
fi
