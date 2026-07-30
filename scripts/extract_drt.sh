#!/usr/bin/env bash
# Extract DRT (Data Representation Type) values from GRIB2 files using wgrib2
# Usage: ./extract_drt.sh <grib2_file_path>
#
# This script extracts grid template numbers (DRT values) from GRIB2 files.
# DRT values are stored in GRIB2 Section 3 (Grid Definition Section) and indicate
# the grid definition template used for the data.
#
# Example:
#   ./extract_drt.sh /path/to/file.grib2
#   Output: /path/to/file.grib2: DRT=0
#
# Output format: filename: DRT=<value>
# For multi-message files with different DRTs: filename: DRT=<value1>,<value2>,...

set -euo pipefail

# Check if wgrib2 is available
if ! command -v wgrib2 &> /dev/null; then
    echo "Error: wgrib2 is not installed or not in PATH" >&2
    echo "Install from: https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/" >&2
    exit 1
fi

# Check if file argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <grib2_file_path>" >&2
    echo "Example: $0 /path/to/file.grib2" >&2
    exit 1
fi

FILE="$1"

# Check if file exists
if [ ! -f "$FILE" ]; then
    echo "Error: File not found: $FILE" >&2
    exit 1
fi

# Check if file is readable
if [ ! -r "$FILE" ]; then
    echo "Error: File is not readable: $FILE" >&2
    exit 1
fi

# Check if file is empty
if [ ! -s "$FILE" ]; then
    echo "Error: File is empty: $FILE" >&2
    exit 1
fi

# Extract DRT values using wgrib2 -grid option
# The output format is like: "1:80:grid_template=30:winds(N/S):"
# We extract the number after grid_template= using grep with Perl regex
# Then we sort and deduplicate to get unique DRT values
mapfile -t DRT_VALUES < <(wgrib2 "$FILE" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+' | sort -u)

# Check if we got any results
if [ ${#DRT_VALUES[@]} -eq 0 ]; then
    echo "Error: Could not extract DRT values from $FILE" >&2
    echo "The file may not be a valid GRIB2 file or may be corrupted" >&2
    exit 1
fi

# Convert array to comma-separated list
DRT_LIST=$(IFS=,; echo "${DRT_VALUES[*]}")

# Output the result in specified format: filename: DRT=<value>
if [ ${#DRT_VALUES[@]} -eq 1 ]; then
    # Single DRT value
    echo "${FILE}: DRT=${DRT_VALUES[0]}"
else
    # Multiple DRT values (unusual but possible for multi-message files)
    echo "${FILE}: DRT=${DRT_LIST} (multiple values)"
fi

exit 0
