#!/bin/bash
# Run DRT extraction on all GRIB2 files in the workspace
# Outputs results to CSV and JSON formats

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTRACT_SCRIPT="$SCRIPT_DIR/scripts/extract_drt.sh"
OUTPUT_DIR="$SCRIPT_DIR/drt_extraction_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Output files
CSV_FILE="$OUTPUT_DIR/drt_extraction_${TIMESTAMP}.csv"
JSON_FILE="$OUTPUT_DIR/drt_extraction_${TIMESTAMP}.json"
ERROR_FILE="$OUTPUT_DIR/drt_extraction_${TIMESTAMP}_errors.txt"

# Initialize files
echo "file_path,drt_value,status,error_message" > "$CSV_FILE"
echo "[]" > "$JSON_FILE"
echo "" > "$ERROR_FILE"

# Counter for statistics
TOTAL_FILES=0
SUCCESS_COUNT=0
ERROR_COUNT=0
MULTIPLE_DRT_COUNT=0

echo "Starting DRT extraction on all GRIB2 files..."
echo "Output directory: $OUTPUT_DIR"
echo ""

# Find all GRIB2 files
GRIB2_FILES=$(find /home/coding/gribtract -type f \( -name "*.grib2" -o -name "*.grb2" \) 2>/dev/null)

# Process each file
while IFS= read -r file; do
    TOTAL_FILES=$((TOTAL_FILES + 1))
    FILE_BASENAME=$(basename "$file")
    RELATIVE_PATH=${file#/home/coding/gribtract/}

    echo -n "Processing: $RELATIVE_PATH ... "

    # Run DRT extraction
    if DRT_OUTPUT=$(bash "$EXTRACT_SCRIPT" "$file" 2>&1); then
        # Success
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))

        # Parse DRT value
        if [[ "$DRT_OUTPUT" =~ DRT=([0-9]+) ]]; then
            DRT_VALUE="${BASH_REMATCH[1]}"

            # Check for multiple DRT values
            if [[ "$DRT_OUTPUT" =~ "multiple values" ]]; then
                MULTIPLE_DRT_COUNT=$((MULTIPLE_DRT_COUNT + 1))
                echo "✓ $DRT_VALUE (multiple)"
                echo "$RELATIVE_PATH,$DRT_VALUE,success,\"multiple DRT values\"" >> "$CSV_FILE"
            else
                echo "✓ $DRT_VALUE"
                echo "$RELATIVE_PATH,$DRT_VALUE,success,\"\"" >> "$CSV_FILE"
            fi
        else
            echo "✓ (unknown format)"
            echo "$RELATIVE_PATH,unknown,success,\"$DRT_OUTPUT\"" >> "$CSV_FILE"
        fi
    else
        # Error
        ERROR_COUNT=$((ERROR_COUNT + 1))
        ERROR_MSG=$(echo "$DRT_OUTPUT" | tail -1)
        echo "✗ Error: $ERROR_MSG"
        echo "$RELATIVE_PATH,,error,\"$ERROR_MSG\"" >> "$CSV_FILE"
        echo "File: $RELATIVE_PATH" >> "$ERROR_FILE"
        echo "Error: $ERROR_MSG" >> "$ERROR_FILE"
        echo "" >> "$ERROR_FILE"
    fi
done <<< "$GRIB2_FILES"

# Generate JSON summary
cat > "$JSON_FILE" << EOF
{
  "timestamp": "$TIMESTAMP",
  "summary": {
    "total_files": $TOTAL_FILES,
    "successful": $SUCCESS_COUNT,
    "errors": $ERROR_COUNT,
    "multiple_drt_values": $MULTIPLE_DRT_COUNT
  },
  "details_file": "$(basename "$CSV_FILE")",
  "errors_file": "$(basename "$ERROR_FILE")"
}
EOF

echo ""
echo "================================================================================"
echo "DRT Extraction Complete"
echo "================================================================================"
echo "Total files processed: $TOTAL_FILES"
echo "Successful extractions: $SUCCESS_COUNT"
echo "Errors: $ERROR_COUNT"
echo "Files with multiple DRT values: $MULTIPLE_DRT_COUNT"
echo ""
echo "Results saved to:"
echo "  CSV: $CSV_FILE"
echo "  JSON: $JSON_FILE"
if [ $ERROR_COUNT -gt 0 ]; then
    echo "  Errors: $ERROR_FILE"
fi
echo "================================================================================"

exit 0
