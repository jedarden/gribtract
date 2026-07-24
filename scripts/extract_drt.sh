#!/bin/bash
# Extract DRT (Data Representation Type) values from a GRIB2 file using wgrib2
# Usage: ./extract_drt.sh <grib2_file_path>
#
# This script extracts the grid template numbers (DRT values) from a GRIB2 file.
# DRT values are stored in GRIB2 Section 3 and indicate the grid definition template used.
#
# Example:
#   ./extract_drt.sh /path/to/file.grib2
#   Output: DRT=30

set -e

# Check if wgrib2 is available
if ! command -v wgrib2 &> /dev/null; then
    echo "Error: wgrib2 is not installed or not in PATH" >&2
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
DRT_VALUES=$(wgrib2 "$FILE" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+' | sort -u)

# Check if we got any results
if [ -z "$DRT_VALUES" ]; then
    echo "Error: Could not extract DRT values from $FILE" >&2
    echo "The file may not be a valid GRIB2 file or may be corrupted" >&2
    exit 1
fi

# Convert multiple values to space-separated list
DRT_LIST=$(echo "$DRT_VALUES" | tr '\n' ' ' | sed 's/ $//')

# Output the result
if [ $(echo "$DRT_VALUES" | wc -l) -eq 1 ]; then
    # Single DRT value
    echo "DRT=$DRT_VALUES"
else
    # Multiple DRT values (unusual but possible for multi-message files)
    echo "DRT=$DRT_LIST (multiple values)"
fi

exit 0
