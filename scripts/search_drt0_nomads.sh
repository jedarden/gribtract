#!/bin/bash
# Expanded search for DRT=0 GRIB2 files using multiple NOAA sources
# Bead bf-5eokv

# Don't exit on errors - we want to continue searching

BASE_DIR="/home/coding/gribtract/drt_search_results"
mkdir -p "$BASE_DIR"

# Try multiple recent dates
DATES=("20260724" "20260723" "20260722")

echo "Expanded DRT=0 GRIB2 Search"
echo "==========================="
echo "Testing multiple dates: ${DATES[*]}"
echo ""

# Function to check DRT of a file
check_drt() {
    local url="$1"
    local output_file="$2"
    local filename=$(basename "$url")

    echo "  Checking: $filename"

    if wget -q -O "$output_file" "$url" 2>/dev/null; then
        if [ -s "$output_file" ]; then
            drt_values=$(wgrib2 -V "$output_file" 2>&1 | grep -o "grid_template=[0-9]*" | sort -u)
            size=$(du -h "$output_file" | cut -f1)

            echo "    Size: $size"
            echo "    DRT: $drt_values"

            if echo "$drt_values" | grep -q "grid_template=0"; then
                echo "    ✓ DRT=0 FOUND"
                echo "$url" >> "$BASE_DIR/drt0_candidates.txt"
                echo "$filename - $size - DRT=0" >> "$BASE_DIR/drt0_details.txt"
                return 0
            else
                echo "    ✗ No DRT=0"
                rm "$output_file"
                return 1
            fi
        else
            echo "    ✗ Empty file"
            rm -f "$output_file"
            return 1
        fi
    else
        echo "    ✗ Download failed"
        return 1
    fi
}

# Clear previous results
> "$BASE_DIR/drt0_candidates.txt"
> "$BASE_DIR/drt0_details.txt"
> "$BASE_DIR/search_log.txt"

echo "=== Expanded Search started at $(date) ===" | tee -a "$BASE_DIR/search_log.txt"
count=0

for TARGET_DATE in "${DATES[@]}"; do
    echo ""
    echo "=== Date: $TARGET_DATE ===" | tee -a "$BASE_DIR/search_log.txt"

    # 1. GFS from NOMADS (different resolutions)
    echo "  GFS 0.25° from NOMADS:"
    gfs_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.${TARGET_DATE}/00/atmos/gfs.t00z.pgrb2.0p25.f000"
    output="$BASE_DIR/gfs_0p25_${TARGET_DATE}_f000.grib2"
    if check_drt "$gfs_url" "$output"; then
        ((count++))
    fi

    echo "  GFS 0.50° from NOMADS:"
    gfs_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.${TARGET_DATE}/00/atmos/gfs.t00z.pgrb2.0p50.f000"
    output="$BASE_DIR/gfs_0p50_${TARGET_DATE}_f000.grib2"
    if check_drt "$gfs_url" "$output"; then
        ((count++))
    fi

    echo "  GFS 1.0° from NOMADS:"
    gfs_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.${TARGET_DATE}/00/atmos/gfs.t00z.pgrb2.1p00.f000"
    output="$BASE_DIR/gfs_1p00_${TARGET_DATE}_f000.grib2"
    if check_drt "$gfs_url" "$output"; then
        ((count++))
    fi

    # 2. NBM (National Blend of Models)
    echo "  NBM CONUS:"
    nbm_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/blend/prod/blend.${TARGET_DATE}/12/blend.t12z.core.f000.co.grib2"
    output="$BASE_DIR/nbm_core_${TARGET_DATE}.grib2"
    if check_drt "$nbm_url" "$output"; then
        ((count++))
    fi

    # 3. RTMA CONUS
    echo "  RTMA CONUS 2.5km:"
    rtma_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma.${TARGET_DATE}/rtma.t12z.2p5.anl.tm00.grib2"
    output="$BASE_DIR/rtma_2p5_${TARGET_DATE}.grib2"
    if check_drt "$rtma_url" "$output"; then
        ((count++))
    fi

    # 4. SREF CONUS
    echo "  SREF CONUS:"
    sref_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.${TARGET_DATE}/00/sref.t00z.mean.f00.grib2"
    output="$BASE_DIR/sref_mean_${TARGET_DATE}.grib2"
    if check_drt "$sref_url" "$output"; then
        ((count++))
    fi

    echo "  Current count: $count DRT=0 files found"

    # If we have enough candidates, we can stop
    if [ $count -ge 5 ]; then
        echo "  ✓ Found 5+ candidates, stopping search"
        break
    fi
done

# Also try GDAS (analysis data, often uses simpler packing)
if [ $count -lt 5 ]; then
    echo ""
    echo "=== Trying GDAS (analysis files) ===" | tee -a "$BASE_DIR/search_log.txt"

    for TARGET_DATE in "${DATES[@]}"; do
        echo "  GDAS 0.50°:"
        gdas_url="https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.${TARGET_DATE}/00/atmos/gdas.t00z.pgrb2.0p50.f000"
        output="$BASE_DIR/gdas_0p50_${TARGET_DATE}_f000.grib2"
        if check_drt "$gdas_url" "$output"; then
            ((count++))
        fi

        if [ $count -ge 5 ]; then
            break
        fi
    done
fi

echo ""
echo "=== Search completed at $(date) ===" | tee -a "$BASE_DIR/search_log.txt"
echo ""
echo "Total DRT=0 candidates found: $count"
echo "Results saved to: $BASE_DIR/drt0_candidates.txt"
echo ""
echo "=== Candidate List ==="
cat "$BASE_DIR/drt0_candidates.txt" | while read url; do
    echo "  - $url"
done
