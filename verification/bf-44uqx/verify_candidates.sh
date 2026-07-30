#!/bin/bash
# Verification script for DRT=0 and CONUS coverage
# Downloads inventory headers and analyzes GRIB2 files

set -e

VER_DIR="/home/coding/gribtract/verification/bf-44uqx"
mkdir -p "$VER_DIR/downloads"
mkdir -p "$VER_DIR/inventory"
mkdir -p "$VER_DIR/results"

cd "$VER_DIR"

# Candidate files from bf-5eokv
declare -a CANDIDATES=(
    "GFS025_20260724|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000"
    "GFS025_20260723|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000"
    "GFS050_20260724|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000"
    "GFS050_20260723|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000"
    "GFS100_20260724|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000"
    "GFS100_20260723|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000"
    "GEFS_f000|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"
    "GEFS_f003|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003"
    "GEFS_f006|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006"
)

echo "Starting verification at $(date)" | tee "$VER_DIR/verification_log.txt"
echo "========================================" | tee -a "$VER_DIR/verification_log.txt"

for candidate in "${CANDIDATES[@]}"; do
    IFS='|' read -r name url <<< "$candidate"

    echo "" | tee -a "$VER_DIR/verification_log.txt"
    echo "Processing: $name" | tee -a "$VER_DIR/verification_log.txt"
    echo "URL: $url" | tee -a "$VER_DIR/verification_log.txt"

    # Download the file
    echo "  Downloading..." | tee -a "$VER_DIR/verification_log.txt"
    if wget -q -O "$VER_DIR/downloads/${name}.grib2" "$url"; then
        echo "  ✓ Download complete" | tee -a "$VER_DIR/verification_log.txt"

        # Get inventory with wgrib2
        echo "  Analyzing with wgrib2..." | tee -a "$VER_DIR/verification_log.txt"
        wgrib2 -v "$VER_DIR/downloads/${name}.grib2" > "$VER_DIR/inventory/${name}.inv" 2>&1

        # Check for DRT=0 in inventory
        echo "  Checking for DRT=0..." | tee -a "$VER_DIR/verification_log.txt"
        if grep -q "drt=0" "$VER_DIR/inventory/${name}.inv"; then
            echo "  ✓ DRT=0 FOUND" | tee -a "$VER_DIR/verification_log.txt"
        else
            echo "  ✗ DRT=0 NOT FOUND" | tee -a "$VER_DIR/verification_log.txt"
        fi

        # Get grid definition for CONUS coverage check
        echo "  Extracting grid definition..." | tee -a "$VER_DIR/verification_log.txt"
        wgrib2 "$VER_DIR/downloads/${name}.grib2" -grid_template | head -20 | tee -a "$VER_DIR/verification_log.txt"

    else
        echo "  ✗ Download FAILED" | tee -a "$VER_DIR/verification_log.txt"
    fi

    echo "----------------------------------------" | tee -a "$VER_DIR/verification_log.txt"
done

echo "" | tee -a "$VER_DIR/verification_log.txt"
echo "Verification complete at $(date)" | tee -a "$VER_DIR/verification_log.txt"
