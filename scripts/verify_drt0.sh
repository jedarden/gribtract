#!/bin/bash
# Verify DRT=0 packing for candidate files

CANDIDATE_DIR="/home/coding/gribtract/downloads/candidates"
OUTPUT_DIR="/home/coding/gribtract/drt_analysis"
REPORT_FILE="$OUTPUT_DIR/drt0_verification_report.txt"
DRT0_LIST="$OUTPUT_DIR/drt0_candidates.txt"
NON_DRT0_LIST="$OUTPUT_DIR/non_drt0_candidates.txt"
PACKING_DETAILS="$OUTPUT_DIR/packing_specifications.txt"

mkdir -p "$OUTPUT_DIR"

echo "DRT=0 Verification Report" > "$REPORT_FILE"
echo "Generated: $(date)" >> "$REPORT_FILE"
echo "=" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "# Files with DRT=0 (Simple Packing)" > "$DRT0_LIST"
echo "# These files are candidates for CONUS verification" >> "$DRT0_LIST"
echo "" >> "$DRT0_LIST"

echo "# Files with NON-DRT=0 packing" > "$NON_DRT0_LIST"
echo "# These files use complex packing or spatial differencing" >> "$NON_DRT0_LIST"
echo "" >> "$NON_DRT0_LIST"

echo "# Packing Specifications for All Candidates" > "$PACKING_DETAILS"
echo "# Format: filename | data_repr_template | packing_type | DRT" >> "$PACKING_DETAILS"
echo "" >> "$PACKING_DETAILS"

total_files=0
drt0_count=0
non_drt0_count=0
empty_count=0

# Find all candidate GRIB2 files
for grib_file in $(find "$CANDIDATE_DIR" -type f -name "*.grib2" | sort); do
    filename=$(basename "$grib_file")
    total_files=$((total_files + 1))

    # Check if file is empty
    if [ ! -s "$grib_file" ]; then
        echo "Skipping empty file: $filename" >> "$REPORT_FILE"
        echo "$filename (empty)" >> "$NON_DRT0_LIST"
        empty_count=$((empty_count + 1))
        continue
    fi

    echo "Checking: $filename" >> "$REPORT_FILE"

    # Get packing information using wgrib2
    packing_info=$(wgrib2 "$grib_file" -packing 2>/dev/null | head -1 | sed 's/^[0-9]*:[0-9]*://' || echo "unknown")
    
    # Get Section 5 information to extract Data Representation Template number
    sec5_info=$(wgrib2 "$grib_file" -Sec5 2>/dev/null | head -1 || echo "")
    
    # Extract template number (format: "Data Repr. Template=5.X")
    template_num=$(echo "$sec5_info" | grep -oP 'Data Repr\. Template=\K[0-9.]+' || echo "")
    
    # Determine DRT from template number
    # DRT 0 = Template 5.0 (simple packing)
    # DRT 2 = Template 5.2 (complex packing)  
    # DRT 3 = Template 5.3 (complex packing + spatial differencing)
    drt_info=""
    if [ -n "$template_num" ]; then
        case "$template_num" in
            "5.0")
                drt_info="0"
                ;;
            "5.2")
                drt_info="2"
                ;;
            "5.3")
                drt_info="3"
                ;;
            *)
                drt_info="unknown($template_num)"
                ;;
        esac
    else
        # Fallback: try to infer from packing description
        if echo "$packing_info" | grep -qi "simple packing"; then
            drt_info="0"
        elif echo "$packing_info" | grep -qi "complex.*spatial"; then
            drt_info="3"
        elif echo "$packing_info" | grep -qi "complex packing"; then
            drt_info="2"
        else
            drt_info="unknown"
        fi
    fi

    echo "  Data Repr. Template: $template_num" >> "$REPORT_FILE"
    echo "  Packing: $packing_info" >> "$REPORT_FILE"
    echo "  DRT: $drt_info" >> "$REPORT_FILE"

    # Record packing details
    echo "$filename | $template_num | $packing_info | DRT=$drt_info" >> "$PACKING_DETAILS"

    if [ "$drt_info" = "0" ]; then
        echo "  ✓ DRT=0 (simple packing)" >> "$REPORT_FILE"
        echo "$filename" >> "$DRT0_LIST"
        drt0_count=$((drt0_count + 1))
    else
        echo "  ✗ NOT DRT=0 (DRT=$drt_info)" >> "$REPORT_FILE"
        echo "$filename (Template=$template_num, DRT=$drt_info)" >> "$NON_DRT0_LIST"
        non_drt0_count=$((non_drt0_count + 1))
    fi

    echo "" >> "$REPORT_FILE"
done

echo "" >> "$REPORT_FILE"
echo "=== Summary ===" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "Total files checked: $total_files" >> "$REPORT_FILE"
echo "Empty files: $empty_count" >> "$REPORT_FILE"
echo "Total DRT=0 files: $drt0_count" >> "$REPORT_FILE"
echo "Total NON-DRT=0 files: $non_drt0_count" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "DRT=0 list saved to: $DRT0_LIST" >> "$REPORT_FILE"
echo "NON-DRT0 list saved to: $NON_DRT0_LIST" >> "$REPORT_FILE"
echo "Packing details saved to: $PACKING_DETAILS" >> "$REPORT_FILE"

cat "$REPORT_FILE"
echo ""
echo "Results:"
echo "  DRT=0 candidates: $drt0_count"
echo "  NON-DRT=0: $non_drt0_count"
echo "  Empty files: $empty_count"
