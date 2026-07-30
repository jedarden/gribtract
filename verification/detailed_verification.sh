#!/bin/bash
# Detailed verification of DRT=0 and CONUS coverage

CANDIDATE=(
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006"
)

mkdir -p verification/drt0_conus_check/results

echo "# DRT=0 and CONUS Coverage Verification Results" > verification/drt0_conus_check/results/verification_report.md
echo "" >> verification/drt0_conus_check/results/verification_report.md
echo "**Date:** $(date)" >> verification/drt0_conus_check/results/verification_report.md
echo "" >> verification/drt0_conus_check/results/verification_report.md

for i in "${!CANDIDATE[@]}"; do
    url="${CANDIDATE[$i]}"
    filename=$(basename "$url")
    result_file="verification/drt0_conus_check/results/${i}_${filename}.txt"

    echo "=== Candidate $((i+1)): $filename ===" | tee -a verification/drt0_conus_check/results/verification_report.md
    echo "URL: $url" | tee -a verification/drt0_conus_check/results/verification_report.md

    # Download first 1MB for analysis
    echo "Downloading sample data (first 1MB)..."
    if curl -s -r 0-1048575 "$url" -o "$result_file.sample"; then
        size=$(stat -c%s "$result_file.sample" 2>/dev/null || stat -f%z "$result_file.sample")
        echo "Downloaded: $(numfmt --to=iec $size)B" | tee -a verification/drt0_conus_check/results/verification_report.md

        # Get inventory with DRT info
        echo "Checking GRIB inventory and DRT values..." | tee -a verification/drt0_conus_check/results/verification_report.md
        wgrib2 "$result_file.sample" -inventory 2>&1 | head -50 | tee "$result_file.inventory"

        # Check for grid_template (DRT indicator)
        echo "" | tee -a verification/drt0_conus_check/results/verification_report.md
        echo "Grid template analysis:" | tee -a verification/drt0_conus_check/results/verification_report.md
        grep -i "grid_template" "$result_file.inventory" | head -10 | tee -a verification/drt0_conus_check/results/verification_report.md

        # Check for lat/lon bounds to verify CONUS coverage
        echo "" | tee -a verification/drt0_conus_check/results/verification_report.md
        echo "Geographic coverage:" | tee -a verification/drt0_conus_check/results/verification_report.md
        wgrib2 "$result_file.sample" -grid 2>&1 | head -20 | tee -a verification/drt0_conus_check/results/verification_report.md

        echo "" | tee -a verification/drt0_conus_check/results/verification_report.md
        echo "---" | tee -a verification/drt0_conus_check/results/verification_report.md
        echo "" | tee -a verification/drt0_conus_check/results/verification_report.md
    else
        echo "ERROR: Failed to download sample" | tee -a verification/drt0_conus_check/results/verification_report.md
        echo "" | tee -a verification/drt0_conus_check/results/verification_report.md
    fi
done

echo "Verification complete. Results saved to verification/drt0_conus_check/results/"
