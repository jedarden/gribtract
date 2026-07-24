#!/bin/bash
# Script to check DRT (grid_template) values from GRIB2 files using wgrib2

echo "DRT (Data Representation Template) Analysis"
echo "==========================================="
echo "Command used: wgrib2 -V <file> | grep grid_template"
echo ""

# Check the main downloaded file
echo "=== Downloaded Files ==="
downloaded_files=(
    "/home/coding/gribtract/downloads/gfs_20260724_00z_1p00_f000.grib2"
)

for file in "${downloaded_files[@]}"; do
    if [ -f "$file" ]; then
        echo "File: $(basename "$file")"
        echo "Path: $file"
        echo "Size: $(du -h "$file" | cut -f1)"

        # Extract unique DRT values from this file
        drt_values=$(wgrib2 -V "$file" 2>&1 | grep -o "grid_template=[0-9]*" | sort -u)
        echo "DRT values found: $drt_values"
        echo ""
    else
        echo "File not found: $file"
        echo ""
    fi
done

echo "=== Sample Files from Different Sources ==="
sample_files=(
    "/home/coding/gribtract/samples/grib2-noaa-gfs/gfs.t00z.pgrb2.1p00.f000.grib2"
    "/home/coding/gribtract/samples/grib2-noaa-nam/nam.20260724.t00z.conusnest.hiresf00.tm00.grib2"
    "/home/coding/gribtract/samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf01.grib2"
    "/home/coding/gribtract/samples/grib2-noaa-rap/rap.20260724.t00z.awp130pgrbf00.grib2"
    "/home/coding/gribtract/tests/corpus/small/conus_drt0.grib2"
    "/home/coding/gribtract/tests/corpus/small/drt2_simple_3x3.grib2"
    "/home/coding/gribtract/tests/corpus/small/drt40_j2k_3x2.grib2"
    "/home/coding/gribtract/tests/corpus/small/drt41_png_3x2.grib2"
    "/home/coding/gribtract/tests/corpus/small/rotated_latlon_5x5.grib2"
)

for file in "${sample_files[@]}"; do
    if [ -f "$file" ] && [ -s "$file" ]; then
        echo "File: $(basename "$file")"
        echo "Path: $file"
        echo "Size: $(du -h "$file" | cut -f1)"

        # Extract unique DRT values from this file
        drt_values=$(wgrib2 -V "$file" 2>&1 | grep -o "grid_template=[0-9]*" | sort -u)
        echo "DRT values found: $drt_values"
        echo ""
    fi
done

echo "=== Summary ==="
echo "Files with DRT=0 (regular lat-lon grid):"
find /home/coding/gribtract -name "*.grib2" -type f -size +100k -exec sh -c '
    file="$1"
    drt=$(wgrib2 -V "$file" 2>&1 | grep -o "grid_template=0" | head -1)
    if [ -n "$drt" ]; then
        echo "$file: DRT=0"
    fi
' sh {} \;

echo ""
echo "Files with non-zero DRT values:"
find /home/coding/gribtract -name "*.grib2" -type f -size +100k -exec sh -c '
    file="$1"
    drt=$(wgrib2 -V "$file" 2>&1 | grep -o "grid_template=[1-9][0-9]*" | head -1)
    if [ -n "$drt" ]; then
        echo "$file: $drt"
    fi
' sh {} \;
