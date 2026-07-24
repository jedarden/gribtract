#!/bin/bash
# Verify DRT=0 and CONUS coverage for candidate files

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

mkdir -p verification/drt0_conus_check/samples

for i in "${!CANDIDATE[@]}"; do
    url="${CANDIDATE[$i]}"
    filename=$(basename "$url" | tr '[:upper:]' '[:lower:]')
    outfile="verification/drt0_conus_check/samples/${i}_${filename}"

    echo "=== Candidate $((i+1)): $url ==="

    # Download first 100KB for header analysis
    echo "Downloading headers (first 100KB)..."
    if curl -s -r 0-102399 "$url" -o "$outfile"; then
        echo "Downloaded: $(stat -f%z "$outfile" 2>/dev/null || stat -c%s "$outfile") bytes"

        # Use wgrib2 to check the first message
        echo "Checking DRT value..."
        wgrib2 "$outfile" -V | head -20

        # Get grid info
        echo "Checking grid definition..."
        wgrib2 "$outfile" -grid 2>&1 | head -5

        echo ""
    else
        echo "ERROR: Failed to download headers from $url"
        echo ""
    fi
done
