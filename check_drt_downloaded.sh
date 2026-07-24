#!/bin/bash
# Check DRT (Data Representation Template) values for downloaded GRIB2 files using wgrib2
# Task: bf-1jvhe

echo "Checking DRT values for GRIB2 files in samples/grib2-noaa-gfs/"
echo "======================================================================"
echo ""

# Output file for results
RESULTS_FILE="/home/coding/gribtract/notes/drt-check-results.txt"
echo "DRT Check Results for Downloaded GRIB2 Files" > "$RESULTS_FILE"
echo "Generated: $(date)" >> "$RESULTS_FILE"
echo "Task: bf-1jvhe" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "============================================================" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Find all GRIB2 files (excluding the verification report)
FILES=$(find samples/grib2-noaa-gfs/ -name "gfs.*" -type f | grep -v verification_report | sort)

DRT_ZERO_COUNT=0
DRT_NONZERO_COUNT=0
EMPTY_FILES=0
ERROR_FILES=0

for file in $FILES; do
    filename=$(basename "$file")

    # Skip empty files
    if [ ! -s "$file" ]; then
        echo "⚠️  EMPTY FILE: $filename"
        echo "⚠️  EMPTY FILE: $filename" >> "$RESULTS_FILE"
        ((EMPTY_FILES++))
        echo "" >> "$RESULTS_FILE"
        echo ""
        continue
    fi

    echo "Checking: $filename"

    # Extract DRT values using wgrib2 -grid option and grep for grid_template
    DRT_VALUES=$(wgrib2 "$file" -grid 2>/dev/null | grep -oP 'grid_template=\K[0-9]+' | sort -u)

    if [ -z "$DRT_VALUES" ]; then
        echo "❌ ERROR: Could not extract DRT from $filename"
        echo "❌ ERROR: Could not extract DRT from $filename" >> "$RESULTS_FILE"
        ((ERROR_FILES++))
    else
        # Convert multiple values to comma-separated list
        DRT_LIST=$(echo "$DRT_VALUES" | tr '\n' ',' | sed 's/,$//')

        # Check if all values are 0
        if echo "$DRT_VALUES" | grep -vq '^0$'; then
            echo "⚠️  NON-ZERO DRT: $filename"
            echo "   DRT values: $DRT_LIST"
            echo "⚠️  NON-ZERO DRT: $filename" >> "$RESULTS_FILE"
            echo "   DRT values: $DRT_LIST" >> "$RESULTS_FILE"
            ((DRT_NONZERO_COUNT++))
        else
            echo "✅ DRT=0: $filename"
            echo "✅ DRT=0: $filename" >> "$RESULTS_FILE"
            ((DRT_ZERO_COUNT++))
        fi
    fi
    echo "" >> "$RESULTS_FILE"
    echo ""
done

echo "============================================================" >> "$RESULTS_FILE"
echo "Summary:" >> "$RESULTS_FILE"
echo "  Files with DRT=0: $DRT_ZERO_COUNT" >> "$RESULTS_FILE"
echo "  Files with non-zero DRT: $DRT_NONZERO_COUNT" >> "$RESULTS_FILE"
echo "  Empty files: $EMPTY_FILES" >> "$RESULTS_FILE"
echo "  Error files: $ERROR_FILES" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "wgrib2 command used: wgrib2 <file> -grid | grep -oP 'grid_template=\\K[0-9]+'" >> "$RESULTS_FILE"

echo "======================================================================"
echo "Summary:"
echo "  Files with DRT=0: $DRT_ZERO_COUNT"
echo "  Files with non-zero DRT: $DRT_NONZERO_COUNT"
echo "  Empty files: $EMPTY_FILES"
echo "  Error files: $ERROR_FILES"
echo ""
echo "Results saved to: $RESULTS_FILE"
echo "wgrib2 command used: wgrib2 <file> -grid | grep -oP 'grid_template=\\K[0-9]+'"
