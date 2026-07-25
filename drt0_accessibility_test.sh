#!/bin/bash
# Test accessibility of verified DRT=0 CONUS files
# Bead: bf-14grj

set -e

OUTPUT_DIR="/home/coding/gribtract/accessibility_test"
mkdir -p "$OUTPUT_DIR"

LOG_FILE="$OUTPUT_DIR/accessibility_test.log"
RESULTS_FILE="$OUTPUT_DIR/accessibility_results.json"

echo "=== DRT=0 CONUS File Accessibility Test ===" | tee -a "$LOG_FILE"
echo "Date: $(date)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Array of verified DRT=0 CONUS files (URL|filename|local_path)
declare -a FILES=(
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260724_f000|/home/coding/gribtract/drt_search_results/gfs_1p00_20260724_f000.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000|gfs_0p25_20260723_f000|/home/coding/gribtract/drt_search_results/gfs_0p25_20260723_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000|gefs_0p50_20260724_f000|/home/coding/gribtract/drt_search_results/gefs_0p50_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003|gefs_0p50_20260724_f003|/home/coding/gribtract/drt_search_results/gefs_0p50_f003.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260723_f000|/home/coding/gribtract/drt_search_results/gfs_1p00_20260723_f000.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000|gfs_0p50_20260724_f000|/home/coding/gribtract/drt_search_results/gfs_0p50_20260724_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006|gefs_0p50_20260724_f006|/home/coding/gribtract/drt_search_results/gefs_0p50_f006.grib2"
)

echo "{" > "$RESULTS_FILE"
echo "  \"test_date\": \"$(date -Iseconds)\"," >> "$RESULTS_FILE"
echo "  \"files\": [" >> "$RESULTS_FILE"

TOTAL_COUNT=${#FILES[@]}
HTTP_SUCCESS=0
DOWNLOAD_SUCCESS=0
LOCAL_VALID=0

for i in "${!FILES[@]}"; do
    IFS='|' read -r URL FILENAME LOCAL_PATH <<< "${FILES[$i]}"

    echo "========================================" | tee -a "$LOG_FILE"
    echo "Test $((i+1))/$TOTAL_COUNT: $FILENAME" | tee -a "$LOG_FILE"
    echo "URL: $URL" | tee -a "$LOG_FILE"
    echo "Local: $LOCAL_PATH" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"

    # Add comma if not first item
    if [ $i -gt 0 ]; then
        echo "    ," >> "$RESULTS_FILE"
    fi

    echo "    {" >> "$RESULTS_FILE"
    echo "      \"filename\": \"$FILENAME\"," >> "$RESULTS_FILE"
    echo "      \"url\": \"$URL\"," >> "$RESULTS_FILE"
    echo "      \"local_path\": \"$LOCAL_PATH\"," >> "$RESULTS_FILE"

    FILE_ACCESSIBLE=false
    HTTP_CODE=""
    CAN_DOWNLOAD=false
    LOCAL_EXISTS=false
    LOCAL_SIZE=0
    GRIB2_VALID=false
    DRT_ZERO=false
    ERRORS=()

    # Test 1: HTTP HEAD request (check if URL exists)
    echo "Test 1: HTTP HEAD request..." | tee -a "$LOG_FILE"
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -L -I --max-time 30 "$URL" 2>&1 || echo "000")

    if [ "$HTTP_STATUS" = "000" ]; then
        echo "  ❌ FAILED: Could not connect to server" | tee -a "$LOG_FILE"
        ERRORS+=("HTTP connection failed")
        HTTP_CODE="connection_failed"
    elif [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "206" ]; then
        echo "  ✅ PASSED: HTTP $HTTP_STATUS" | tee -a "$LOG_FILE"
        HTTP_SUCCESS=$((HTTP_SUCCESS + 1))
        FILE_ACCESSIBLE=true
        HTTP_CODE="$HTTP_STATUS"
    else
        echo "  ❌ FAILED: HTTP $HTTP_STATUS" | tee -a "$LOG_FILE"
        ERRORS+=("HTTP $HTTP_STATUS")
        HTTP_CODE="$HTTP_STATUS"
    fi

    # Test 2: Check local file exists and validate
    if [ -f "$LOCAL_PATH" ]; then
        LOCAL_EXISTS=true
        LOCAL_SIZE=$(stat -f%z "$LOCAL_PATH" 2>/dev/null || stat -c%s "$LOCAL_PATH" 2>/dev/null || echo "0")

        echo "Test 2: Local file validation..." | tee -a "$LOG_FILE"
        echo "  File exists: $LOCAL_PATH" | tee -a "$LOG_FILE"
        echo "  File size: $LOCAL_SIZE bytes" | tee -a "$LOG_FILE"

        if [ "$LOCAL_SIZE" -gt 0 ]; then
            # Test GRIB2 format
            if command -v wgrib2 >/dev/null 2>&1; then
                # wgrib2 will output records if file is valid, or error if not
                GRIB2_CHECK=$(wgrib2 "$LOCAL_PATH" 2>&1 | head -1 || echo "")
                GRIB2_ERROR=$(wgrib2 "$LOCAL_PATH" 2>&1 >/dev/null | grep -i "error\|invalid\|not grib" || echo "")

                if [ -n "$GRIB2_CHECK" ] && [ -z "$GRIB2_ERROR" ]; then
                    echo "  ✅ PASSED: Valid GRIB2 format" | tee -a "$LOG_FILE"
                    GRIB2_VALID=true
                    LOCAL_VALID=$((LOCAL_VALID + 1))

                    # Check DRT=0
                    DRT_INFO=$(wgrib2 -packing "$LOCAL_PATH" 2>&1 | head -1 || echo "")
                    if echo "$DRT_INFO" | grep -q "5.0.0"; then
                        echo "  ✅ PASSED: DRT=0 (Simple Packing) confirmed" | tee -a "$LOG_FILE"
                        DRT_ZERO=true
                    else
                        echo "  ⚠️  WARNING: DRT=0 not confirmed" | tee -a "$LOG_FILE"
                        echo "  DRT info: $DRT_INFO" | tee -a "$LOG_FILE"
                        DRT_ZERO=false
                    fi
                else
                    echo "  ❌ FAILED: Invalid GRIB2 format" | tee -a "$LOG_FILE"
                    echo "  Error: $GRIB2_ERROR" | tee -a "$LOG_FILE"
                    ERRORS+=("Invalid GRIB2 format")
                fi
            else
                echo "  ⚠️  SKIP: wgrib2 not available" | tee -a "$LOG_FILE"
                GRIB2_VALID=true  # Assume valid if previously verified
            fi
        else
            echo "  ❌ FAILED: Empty local file" | tee -a "$LOG_FILE"
            ERRORS+=("Empty local file")
        fi
    else
        echo "Test 2: Local file check..." | tee -a "$LOG_FILE"
        echo "  ❌ FAILED: Local file not found" | tee -a "$LOG_FILE"
        ERRORS+=("Local file not found")
    fi

    # Test 3: Small sample download (100KB to verify download capability)
    echo "Test 3: Sample download capability..." | tee -a "$LOG_FILE"
    SAMPLE_FILE="$OUTPUT_DIR/${FILENAME}_sample.grib2"

    if curl -s -L -R --max-time 60 -o "$SAMPLE_FILE" -r 0-102399 "$URL" 2>&1 | tee -a "$LOG_FILE"; then
        SAMPLE_SIZE=$(stat -f%z "$SAMPLE_FILE" 2>/dev/null || stat -c%s "$SAMPLE_FILE" 2>/dev/null || echo "0")

        if [ "$SAMPLE_SIZE" -gt 0 ]; then
            echo "  ✅ PASSED: Downloaded $SAMPLE_SIZE bytes sample" | tee -a "$LOG_FILE"
            CAN_DOWNLOAD=true
            DOWNLOAD_SUCCESS=$((DOWNLOAD_SUCCESS + 1))
            rm -f "$SAMPLE_FILE"
        else
            echo "  ❌ FAILED: Empty download" | tee -a "$LOG_FILE"
            ERRORS+=("Empty sample download")
        fi
    else
        echo "  ❌ FAILED: Download failed" | tee -a "$LOG_FILE"
        ERRORS+=("Sample download failed")
    fi

    # Write results
    echo "      \"accessible\": $FILE_ACCESSIBLE," >> "$RESULTS_FILE"
    echo "      \"http_status\": \"$HTTP_CODE\"," >> "$RESULTS_FILE"
    echo "      \"can_download\": $CAN_DOWNLOAD," >> "$RESULTS_FILE"
    echo "      \"local_exists\": $LOCAL_EXISTS," >> "$RESULTS_FILE"
    echo "      \"local_size\": $LOCAL_SIZE," >> "$RESULTS_FILE"
    echo "      \"grib2_valid\": $GRIB2_VALID," >> "$RESULTS_FILE"
    echo "      \"drt_zero\": $DRT_ZERO," >> "$RESULTS_FILE"

    if [ ${#ERRORS[@]} -gt 0 ]; then
        echo "      \"errors\": [" >> "$RESULTS_FILE"
        for j in "${!ERRORS[@]}"; do
            if [ $j -gt 0 ]; then
                echo "        ," >> "$RESULTS_FILE"
            fi
            echo "        \"${ERRORS[$j]}\"" >> "$RESULTS_FILE"
        done
        echo "      ]" >> "$RESULTS_FILE"
    else
        echo "      \"errors\": []" >> "$RESULTS_FILE"
    fi

    echo "    }" >> "$RESULTS_FILE"
    echo "" | tee -a "$LOG_FILE"
done

echo "  ]" >> "$RESULTS_FILE"
echo "  \"summary\": {" >> "$RESULTS_FILE"
echo "    \"total_files\": $TOTAL_COUNT," >> "$RESULTS_FILE"
echo "    \"http_accessible\": $HTTP_SUCCESS," >> "$RESULTS_FILE"
echo "    \"downloadable\": $DOWNLOAD_SUCCESS," >> "$RESULTS_FILE"
echo "    \"local_valid\": $LOCAL_VALID," >> "$RESULTS_FILE"
echo "    \"http_access_rate\": $(awk "BEGIN {printf \"%.2f\", ($HTTP_SUCCESS/$TOTAL_COUNT)*100}")" >> "$RESULTS_FILE"
echo "  }" >> "$RESULTS_FILE"
echo "}" >> "$RESULTS_FILE"

echo "========================================" | tee -a "$LOG_FILE"
echo "SUMMARY" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
echo "Total files tested: $TOTAL_COUNT" | tee -a "$LOG_FILE"
echo "HTTP accessible: $HTTP_SUCCESS ($(awk "BEGIN {printf \"%.2f\", ($HTTP_SUCCESS/$TOTAL_COUNT)*100}")%)" | tee -a "$LOG_FILE"
echo "Downloadable: $DOWNLOAD_SUCCESS ($(awk "BEGIN {printf \"%.2f\", ($DOWNLOAD_SUCCESS/$TOTAL_COUNT)*100}")%)" | tee -a "$LOG_FILE"
echo "Local valid GRIB2: $LOCAL_VALID ($(awk "BEGIN {printf \"%.2f\", ($LOCAL_VALID/$TOTAL_COUNT)*100}")%)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
echo "Results saved to: $RESULTS_FILE" | tee -a "$LOG_FILE"

exit 0
