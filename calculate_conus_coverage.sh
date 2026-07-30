#!/bin/bash
# Calculate CONUS coverage for DRT=0 GRIB2 files
# CONUS bounds: 24°N-50°N, 125°W-67°W (235°E-293°E)

echo "CONUS Coverage Analysis for DRT=0 Candidates"
echo "============================================"
echo "CONUS Definition: 24°N-50°N, 125°W-67°W (235°E-293°E)"
echo ""

# Function to calculate CONUS grid points
calculate_conus_points() {
    local lat_start=$1  # Northernmost latitude
    local lat_end=$2    # Southernmost latitude
    local lon_start=$3  # Westernmost longitude (0-360)
    local lon_end=$4    # Easternmost longitude (0-360)
    local lat_res=$5    # Latitude resolution
    local lon_res=$6    # Longitude resolution

    # Calculate latitude points (CONUS: 24°N to 50°N = 26 degrees)
    local conus_lat_start=50
    local conus_lat_end=24
    local lat_range=$((conus_lat_start - conus_lat_end))
    local lat_points=$((lat_range / lat_res + 1))

    # Calculate longitude points (CONUS: 125°W to 67°W = 235°E to 293°E = 58 degrees)
    local conus_lon_start=235
    local conus_lon_end=293
    local lon_range=$((conus_lon_end - conus_lon_start))
    local lon_points=$((lon_range / lon_res + 1))

    local conus_points=$((lat_points * lon_points))

    echo "$lat_points $lon_points $conus_points"
}

# Function to analyze a GRIB2 file
analyze_file() {
    local file=$1
    local filename=$(basename "$file")

    echo "Analyzing: $filename"

    # Get grid information using wgrib2
    local grid_info=$(wgrib2 "$file" -grid -match "" | head -3)

    # Extract grid dimensions
    local grid_dims=$(echo "$grid_info" | grep "lat-lon grid" | sed -n 's/.*(\(.*\)).*/\1/p')
    local ni=$(echo "$grid_dims" | cut -d'x' -f1 | tr -d ' ')
    local nj=$(echo "$grid_dims" | cut -d'x' -f2 | tr -d ' ')

    # Extract latitude info
    local lat_info=$(echo "$grid_info" | grep "lat " | sed -n 's/.*lat \([^ ]*\) to \([^ ]*\) by \([^ ]*\).*/\1 \2 \3/p')
    local lat_start=$(echo "$lat_info" | cut -d' ' -f1)
    local lat_end=$(echo "$lat_info" | cut -d' ' -f2)
    local lat_res=$(echo "$lat_info" | cut -d' ' -f3)

    # Extract longitude info
    local lon_info=$(echo "$grid_info" | grep "lon " | sed -n 's/.*lon \([^ ]*\) to \([^ ]*\) by \([^ ]*\).*/\1 \2 \3/p')
    local lon_start=$(echo "$lon_info" | cut -d' ' -f1)
    local lon_end=$(echo "$lon_info" | cut -d' ' -f2)
    local lon_res=$(echo "$lon_info" | cut -d' ' -f3)

    # Calculate total global points
    local total_points=$((ni * nj))

    # Calculate CONUS coverage
    local conus_calc=$(calculate_conus_points "$lat_start" "$lat_end" "$lon_start" "$lon_end" "$lat_res" "$lon_res")
    local conus_lat_points=$(echo "$conus_calc" | cut -d' ' -f1)
    local conus_lon_points=$(echo "$conus_calc" | cut -d' ' -f2)
    local conus_points=$(echo "$conus_calc" | cut -d' ' -f3)

    # Calculate coverage percentage
    local coverage_pct=$(echo "scale=4; $conus_points * 100 / $total_points" | bc)

    echo "  Grid Template: 0 (Regular Lat-Lon)"
    echo "  Grid Dimensions: ${ni} × ${nj} = $total_points total points"
    echo "  Latitude: ${lat_start}°N to ${lat_end}°N by ${lat_res}°"
    echo "  Longitude: ${lon_start}°E to ${lon_end}°E by ${lon_res}°"
    echo "  CONUS Bounds: 24°N-50°N, 125°W-67°W (235°E-293°E)"
    echo "  CONUS Grid Points: ${conus_points} (${conus_lat_points} lat × ${conus_lon_points} lon)"
    echo "  CONUS Coverage: ${coverage_pct}% of global grid"
    echo ""

    # Output for JSON parsing
    echo "JSON_OUTPUT:$filename|${ni}|${nj}|${total_points}|${lat_start}|${lat_end}|${lat_res}|${lon_start}|${lon_end}|${lon_res}|${conus_lat_points}|${conus_lon_points}|${conus_points}|${coverage_pct}"
}

# Create results directory
mkdir -p /home/coding/gribtract/drt_verification/conus_coverage

# Analyze all 7 DRT=0 candidates
candidates=(
    "/home/coding/gribtract/drt_search_results/gfs_1p00_20260724_f000.grib2"
    "/home/coding/gribtract/drt_search_results/gfs_0p25_20260723_f000.grib2"
    "/home/coding/gribtract/drt_search_results/gefs_0p50_f000.grib2"
    "/home/coding/gribtract/drt_search_results/gefs_0p50_f003.grib2"
    "/home/coding/gribtract/drt_search_results/gfs_1p00_20260723_f000.grib2"
    "/home/coding/gribtract/drt_search_results/gfs_0p50_20260724_f000.grib2"
    "/home/coding/gribtract/drt_search_results/gefs_0p50_f006.grib2"
)

# Initialize JSON output
echo "{"
echo "  \"analysis_date\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\","
echo "  \"conus_definition\": {"
echo "    \"lat_min\": 24,"
echo "    \"lat_max\": 50,"
echo "    \"lon_min\": 235,"
echo "    \"lon_max\": 293,"
echo "    \"description\": \"24°N-50°N, 125°W-67°W (235°E-293°E)\""
echo "  },"
echo "  \"candidates\": ["

first=true
for candidate in "${candidates[@]}"; do
    if [ -f "$candidate" ]; then
        if [ "$first" = true ]; then
            first=false
        else
            echo ","
        fi
        analyze_file "$candidate" | grep "JSON_OUTPUT" | sed 's/JSON_OUTPUT:/    {/' | sed 's/|/", "/g' | sed 's/$/"}/'
    fi
done

echo ""
echo "  ]"
echo "}"
