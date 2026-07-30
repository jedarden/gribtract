#!/bin/bash
# Script to search NOAA archives for DRT=0 GRIB2 files with CONUS coverage
# Bead bf-5eokv

set -e

TARGET_DATE=${1:-$(date +%Y%m%d)}
BASE_DIR="/home/coding/gribtract/drt_search_results"
mkdir -p "$BASE_DIR"

echo "Searching for DRT=0 GRIB2 files in NOAA archives"
echo "================================================"
echo "Target date: $TARGET_DATE"
echo "Results directory: $BASE_DIR"
echo ""

# Function to check DRT of a file
check_drt() {
    local url="$1"
    local output_file="$2"
    local filename=$(basename "$url")

    echo "Checking: $filename"

    if wget -q -O "$output_file" "$url" 2>/dev/null; then
        if [ -s "$output_file" ]; then
            drt_values=$(wgrib2 -V "$output_file" 2>&1 | grep -o "grid_template=[0-9]*" | sort -u)
            size=$(du -h "$output_file" | cut -f1)

            echo "  Size: $size"
            echo "  DRT values: $drt_values"

            if echo "$drt_values" | grep -q "grid_template=0"; then
                echo "  ✓ CONTAINS DRT=0"
                return 0
            else
                echo "  ✗ No DRT=0 found"
                rm "$output_file"
                return 1
            fi
        else
            echo "  ✗ Downloaded file is empty"
            rm -f "$output_file"
            return 1
        fi
    else
        echo "  ✗ Failed to download"
        return 1
    fi
}

# Function to test multiple forecast hours from a model
test_model_forecast_hours() {
    local model_name="$1"
    local base_url="$2"
    local file_pattern="$3"
    local hours=("${@:4}") # Remaining args are forecast hours

    echo ""
    echo "=== Testing $model_name ==="

    for hour in "${hours[@]}"; do
        local url=$(echo "$base_url" | sed "s/FFF/$hour/g")
        local output_file="$BASE_DIR/${model_name}_f${hour}.grib2"

        if check_drt "$url" "$output_file"; then
            echo "$url" >> "$BASE_DIR/drt0_candidates.txt"
            echo "  ✓ Found DRT=0 candidate!"
        fi
    done
}

# Clear previous results
> "$BASE_DIR/drt0_candidates.txt"
> "$BASE_DIR/search_log.txt"

echo "=== Search started at $(date) ===" | tee -a "$BASE_DIR/search_log.txt"

# 1. GFS 0.25° from AWS S3
echo "=== 1. GFS 0.25° Global (includes CONUS) ===" | tee -a "$BASE_DIR/search_log.txt"
gfs_base="https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.${TARGET_DATE}/00/atmos/pgrb2.0p25/gfs.t00z.pgrb2.0p25.fFFF"
test_model_forecast_hours "gfs_0p25" "$gfs_base" "pattern" "000" "003" "006"

# 2. GFS 0.50° from AWS S3
echo "=== 2. GFS 0.50° Global (includes CONUS) ===" | tee -a "$BASE_DIR/search_log.txt"
gfs_base="https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.${TARGET_DATE}/00/atmos/pgrb2.0p50/gfs.t00z.pgrb2.0p50.fFFF"
test_model_forecast_hours "gfs_0p50" "$gfs_base" "pattern" "000" "003" "006"

# 3. GFS 1.0° from AWS S3
echo "=== 3. GFS 1.0° Global (includes CONUS) ===" | tee -a "$BASE_DIR/search_log.txt"
gfs_base="https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.${TARGET_DATE}/00/atmos/pgrb2.1p00/gfs.t00z.pgrb2.1p00.fFFF"
test_model_forecast_hours "gfs_1p00" "$gfs_base" "pattern" "000" "003" "006"

# 4. NAM CONUS from NOMADS
echo "=== 4. NAM CONUS 12km ===" | tee -a "$BASE_DIR/search_log.txt"
nam_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.${TARGET_DATE}/nam.t00z.awip12.tm00.grib2"
output_file="$BASE_DIR/nam_conus_awip12.grib2"
if check_drt "$nam_url" "$output_file"; then
    echo "$nam_url" >> "$BASE_DIR/drt0_candidates.txt"
fi

# Also try NAM CONUS nest
nam_nest_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.${TARGET_DATE}/nam.t00z.conusnest.hiresf00.tm00.grib2"
output_file="$BASE_DIR/nam_conusnest.grib2"
if check_drt "$nam_nest_url" "$output_file"; then
    echo "$nam_nest_url" >> "$BASE_DIR/drt0_candidates.txt"
fi

# 5. HRRR from NOMADS
echo "=== 5. HRRR CONUS 3km ===" | tee -a "$BASE_DIR/search_log.txt"
hrrr_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.${TARGET_DATE}/hrrr.t00z.wrfsfcf01.grib2"
output_file="$BASE_DIR/hrrr_conus.grib2"
if check_drt "$hrrr_url" "$output_file"; then
    echo "$hrrr_url" >> "$BASE_DIR/drt0_candidates.txt"
fi

# 6. RAP from NOMADS
echo "=== 6. RAP CONUS 13km ===" | tee -a "$BASE_DIR/search_log.txt"
rap_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/prod/rap.${TARGET_DATE}/rap.t00z.awp130pgrbf00.grib2"
output_file="$BASE_DIR/rap_conus.grib2"
if check_drt "$rap_url" "$output_file"; then
    echo "$rap_url" >> "$BASE_DIR/drt0_candidates.txt"
fi

# 7. GEFS from AWS S3
echo "=== 7. GEFS Ensemble (includes CONUS) ===" | tee -a "$BASE_DIR/search_log.txt"
gefs_base="https://noaa-gefs-pds.s3.amazonaws.com/gefs.${TARGET_DATE}/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.fFFF"
test_model_forecast_hours "gefs_0p50" "$gefs_base" "pattern" "000" "003" "006"

echo ""
echo "=== Search completed at $(date) ===" | tee -a "$BASE_DIR/search_log.txt"
echo ""
echo "Results saved to: $BASE_DIR/drt0_candidates.txt"
echo "Number of candidates found: $(wc -l < "$BASE_DIR/drt0_candidates.txt" || echo "0")"
