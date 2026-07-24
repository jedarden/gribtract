#!/bin/bash
# Download candidate GRIB2 files for DRT analysis (bf-4k5yl)
# Generated: 2026-07-24

set -e

CANDIDATE_DIR="downloads/candidates"
LOG_FILE="downloads/candidates/download_log.txt"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Create log file
echo "GRIB2 Candidate Download Log - bf-4k5yl" > "$LOG_FILE"
echo "Started: $TIMESTAMP" >> "$LOG_FILE"
echo "===========================================" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Function to download and verify a file
download_file() {
    local url="$1"
    local output="$2"
    local expected_size="$3"
    local category="$4"

    local filename=$(basename "$output")
    local file_timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    echo "Downloading: $filename"
    echo "URL: $url"
    echo "Expected size: $expected_size bytes"

    # Download with curl, showing progress
    if curl -L -o "$output" --progress-bar "$url"; then
        local actual_size=$(stat -f%z "$output" 2>/dev/null || stat -c%s "$output" 2>/dev/null || echo "0")

        # Verify file size (allow 5% tolerance for size variations)
        if [ "$expected_size" != "N/A" ]; then
            local size_diff=$(( (actual_size - expected_size) * 100 / expected_size ))
            local size_diff_abs=${size_diff#-}

            if [ "$size_diff_abs" -gt 5 ]; then
                echo "  WARNING: Size mismatch! Expected: $expected_size, Got: $actual_size ($size_diff% diff)"
            else
                echo "  ✓ Size verified: $actual_size bytes ($size_diff% from expected)"
            fi
        else
            echo "  ✓ Downloaded: $actual_size bytes (no size verification)"
        fi

        # Verify it's a valid GRIB2 file using file command
        if file "$output" | grep -q "GRIB"; then
            echo "  ✓ Valid GRIB2 format confirmed"
        else
            echo "  ✗ WARNING: File may not be valid GRIB2 format"
        fi

        # Log the download
        echo "[$file_timestamp] SUCCESS: $filename" >> "$LOG_FILE"
        echo "  Category: $category" >> "$LOG_FILE"
        echo "  URL: $url" >> "$LOG_FILE"
        echo "  Size: $actual_size bytes" >> "$LOG_FILE"
        echo "  Expected: $expected_size bytes" >> "$LOG_FILE"
        echo "" >> "$LOG_FILE"

        return 0
    else
        echo "  ✗ FAILED to download $filename"
        echo "[$file_timestamp] FAILED: $filename - Download error" >> "$LOG_FILE"
        echo "  URL: $url" >> "$LOG_FILE"
        echo "" >> "$LOG_FILE"
        return 1
    fi
}

# Create directories
mkdir -p "$CANDIDATE_DIR"/{0p25,0p50,1p00}

download_count=0
failed_count=0

# High-Resolution Files (0.25°)
echo "==========================================="  >> "$LOG_FILE"
echo "High-Resolution Files (0.25°)" >> "$LOG_FILE"
echo "==========================================="  >> "$LOG_FILE"

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f000.20260724.grib2" \
    "514251059" \
    "0p25-analysis" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f003" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f003.20260724.grib2" \
    "544007059" \
    "0p25-short-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f006" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f006.20260724.grib2" \
    "546147177" \
    "0p25-medium-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f012" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f012.20260723.grib2" \
    "N/A" \
    "0p25-long-term-hist" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.0p25.f000" \
    "$CANDIDATE_DIR/0p25/gfs.t00z.pgrb2.0p25.f000.20260722.grib2" \
    "512102383" \
    "0p25-historical" && ((download_count++)) || ((failed_count++))

# Medium-Resolution Files (0.50°)
echo "==========================================="  >> "$LOG_FILE"
echo "Medium-Resolution Files (0.50°)" >> "$LOG_FILE"
echo "==========================================="  >> "$LOG_FILE"

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f000.20260724.grib2" \
    "152106356" \
    "0p50-analysis" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f003" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f003.20260724.grib2" \
    "N/A" \
    "0p50-short-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f006" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f006.20260724.grib2" \
    "N/A" \
    "0p50-medium-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f000.20260723.grib2" \
    "N/A" \
    "0p50-historical" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260721/00/atmos/gfs.t00z.pgrb2.0p50.f012" \
    "$CANDIDATE_DIR/0p50/gfs.t00z.pgrb2.0p50.f012.20260721.grib2" \
    "N/A" \
    "0p50-long-term-old" && ((download_count++)) || ((failed_count++))

# Low-Resolution Files (1.00°)
echo "==========================================="  >> "$LOG_FILE"
echo "Low-Resolution Files (1.00°)" >> "$LOG_FILE"
echo "==========================================="  >> "$LOG_FILE"

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f000.20260724.grib2" \
    "42755881" \
    "1p00-analysis" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f003" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f003.20260724.grib2" \
    "N/A" \
    "1p00-short-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f006" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f006.20260724.grib2" \
    "N/A" \
    "1p00-medium-term" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f000.20260723.grib2" \
    "N/A" \
    "1p00-historical" && ((download_count++)) || ((failed_count++))

download_file \
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.1p00.f024" \
    "$CANDIDATE_DIR/1p00/gfs.t00z.pgrb2.1p00.f024.20260722.grib2" \
    "N/A" \
    "1p00-24hour-forecast" && ((download_count++)) || ((failed_count++))

# Summary
end_timestamp=$(date '+%Y-%m-%d %H:%M:%S')
echo "" >> "$LOG_FILE"
echo "===========================================" >> "$LOG_FILE"
echo "DOWNLOAD SUMMARY" >> "$LOG_FILE"
echo "===========================================" >> "$LOG_FILE"
echo "Completed: $end_timestamp" >> "$LOG_FILE"
echo "Total candidates: 15" >> "$LOG_FILE"
echo "Successfully downloaded: $download_count" >> "$LOG_FILE"
echo "Failed downloads: $failed_count" >> "$LOG_FILE"

echo ""
echo "==========================================="
echo "DOWNLOAD COMPLETE"
echo "==========================================="
echo "Successfully downloaded: $download_count/15 files"
echo "Failed downloads: $failed_count"
echo "Details logged to: $LOG_FILE"
