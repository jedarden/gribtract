#!/bin/bash
# Download remaining candidate GRIB2 files - bf-4k5yl
# Simple approach: download files one by one

set -e

CANDIDATE_DIR="downloads/candidates"
LOG_FILE="downloads/candidates/download_log.txt"

# Function to download a single file
download_single() {
    local url="$1"
    local output="$2"
    local description="$3"

    echo "Downloading: $description"
    echo "URL: $url"
    echo "Output: $output"

    if curl -L -o "$output" "$url" --progress-bar; then
        local size=$(stat -f%z "$output" 2>/dev/null || stat -c%s "$output" 2>/dev/null)
        echo "✓ Downloaded: $size bytes"

        # Verify GRIB2 format
        if file "$output" | grep -q "GRIB"; then
            echo "✓ Valid GRIB2 format confirmed"
        else
            echo "✗ WARNING: File may not be valid GRIB2 format"
        fi

        # Log success
        local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        echo "[$timestamp] SUCCESS: $description - $size bytes" >> "$LOG_FILE"
        return 0
    else
        echo "✗ FAILED: $description"
        local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        echo "[$timestamp] FAILED: $description" >> "$LOG_FILE"
        return 1
    fi
}

echo "Continuing downloads of remaining candidate files..."
echo ""

# Remaining high-resolution files
echo "=== High-Resolution Files (0.25°) - Remaining ==="
download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f003" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f003.20260724.grib2" \
    "0p25-f003"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f006" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f006.20260724.grib2" \
    "0p25-f006"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f012" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f012.20260723.grib2" \
    "0p25-f012-20260723"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.0p25.f000" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f000.20260722.grib2" \
    "0p25-f000-20260722"

# Medium-resolution files
echo ""
echo "=== Medium-Resolution Files (0.50°) ==="
download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f000.20260724.grib2" \
    "0p50-f000"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f003" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f003.20260724.grib2" \
    "0p50-f003"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f006" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f006.20260724.grib2" \
    "0p50-f006"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f000.20260723.grib2" \
    "0p50-f000-20260723"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260721/00/atmos/gfs.t00z.pgrb2.0p50.f012" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f012.20260721.grib2" \
    "0p50-f012-20260721"

# Low-resolution files
echo ""
echo "=== Low-Resolution Files (1.00°) ==="
download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f000.20260724.grib2" \
    "1p00-f000"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f003" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f003.20260724.grib2" \
    "1p00-f003"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f006" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f006.20260724.grib2" \
    "1p00-f006"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f000.20260723.grib2" \
    "1p00-f000-20260723"

download_single \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.1p00.f024" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f024.20260722.grib2" \
    "1p00-f024-20260722"

echo ""
echo "Download complete!"
