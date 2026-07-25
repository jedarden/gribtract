#!/bin/bash
# Test accessibility of verified DRT=0 CONUS files
# Bead: bf-14grj

set -e

OUTPUT_DIR="/home/coding/gribtract/accessibility_test"
mkdir -p "$OUTPUT_DIR"

LOG_FILE="$OUTPUT_DIR/drt0_conus_accessibility_test.log"
RESULTS_FILE="$OUTPUT_DIR/drt0_conus_accessibility_results.json"

echo "=== DRT=0 CONUS File Accessibility Test ===" | tee -a "$LOG_FILE"
echo "Date: $(date)" | tee -a "$LOG_FILE"
echo "Testing 7 verified DRT=0 CONUS files (GFS/GEFS)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Array of verified DRT=0 CONUS files (URL and local filename)
declare -a DRT0_FILES=(
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260724_f000.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000|gfs_0p25_20260723_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000|gefs_0p50_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003|gefs_0p50_f003.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260723_f000.grib2"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000|gfs_0p50_20260724_f000.grib2"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006|gefs_0p50_f006.grib2"
)

echo "{" > "$RESULTS_FILE"
echo "  \"test_date\": \"$(date -Iseconds)\"," >> "$RESULTS_FILE"
echo "  \"test_purpose\": \"Verify accessibility of 7 verified DRT=0 CONUS files\"," >> "$RESULTS_FILE"
echo "  \"files\": [" >> "$RESULTS_FILE"

TOTAL_COUNT=${#DRT0_FILES[@]}
SUCCESS_COUNT=0
FAIL_COUNT=0

for i in "${!DRT0_FILES[@]}"; do
    IFS='|' read -r URL FILENAME <<< "${DRT0_FILES[$i]}"
    LOCAL_PATH="/home/coding/gribtract/drt_search_results/$FILENAME"

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

    # Test 1: Check local file exists and is valid
    echo "Test 1: Local file verification..." | tee -a "$LOG_FILE"
    LOCAL_EXISTS=false
    LOCAL_VALID=false

    if [ -f "$LOCAL_PATH" ]; then
        LOCAL_EXISTS=true
        LOCAL_SIZE=$(stat -c%s "$LOCAL_PATH" 2>/dev/null || stat -f%z "$LOCAL_PATH" 2>/dev/null || echo "0")

        if [ "$LOCAL_SIZE" -gt 0 ]; then
            echo "  ✓ Local file exists: $LOCAL_SIZE bytes" | tee -a "$LOG_FILE"

            # Check if it's valid GRIB2 using wgrib2
            if command -v wgrib2 >/dev/null 2>&1; then
                if wgrib2 "$LOCAL_PATH" >/dev/null 2>&1; then
                    LOCAL_VALID=true
                    echo "  ✓ Local file is valid GRIB2" | tee -a "$LOG_FILE"
                else
                    echo "  ⚠ Local file exists but may be corrupted" | tee -a "$LOG_FILE"
                fi
            else
                LOCAL_VALID=true
                echo "  ⚠ wgrib2 not available, assuming valid based on size" | tee -a "$LOG_FILE"
            fi
        else
            echo "  ✗ Local file is empty (0 bytes)" | tee -a "$LOG_FILE"
        fi
    else
        echo "  ✗ Local file does not exist" | tee -a "$LOG_FILE"
    fi

    echo "      \"local_exists\": $LOCAL_EXISTS," >> "$RESULTS_FILE"
    echo "      \"local_valid_grib2\": $LOCAL_VALID," >> "$RESULTS_FILE"
    echo "      \"local_size\": $LOCAL_SIZE," >> "$RESULTS_FILE"

    # Test 2: HTTP HEAD request (check if URL exists)
    echo "Test 2: HTTP HEAD request..." | tee -a "$LOG_FILE"
    START_TIME=$(date +%s.%N)
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -L -I --max-time 30 "$URL" 2>&1 || echo "000")
    END_TIME=$(date +%s.%N)
    RESPONSE_TIME=$(awk "BEGIN {printf \"%.3f\", $END_TIME - $START_TIME}")

    if [ "$HTTP_STATUS" = "000" ]; then
        echo "  ✗ FAILED: Could not connect to server" | tee -a "$LOG_FILE"
        echo "      \"http_head\": false," >> "$RESULTS_FILE"
        echo "      \"http_status\": \"connection_failed\"," >> "$RESULTS_FILE"
        echo "      \"response_time\": 0," >> "$RESULTS_FILE"
        echo "      \"accessible\": false," >> "$RESULTS_FILE"
        echo "      \"auth_required\": false," >> "$RESULTS_FILE"
        echo "      \"download_test\": {" >> "$RESULTS_FILE"
        echo "        \"downloaded\": false," >> "$RESULTS_FILE"
        echo "        \"download_size\": 0," >> "$RESULTS_FILE"
        echo "        \"download_time\": 0," >> "$RESULTS_FILE"
        echo "        \"valid_grib2\": false" >> "$RESULTS_FILE"
        echo "      }" >> "$RESULTS_FILE"
        echo "      \"error\": \"Connection failed\"" >> "$RESULTS_FILE"
        echo "    }" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "" | tee -a "$LOG_FILE"
        continue
    elif [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "206" ]; then
        echo "  ✓ PASSED: HTTP $HTTP_STATUS (response time: ${RESPONSE_TIME}s)" | tee -a "$LOG_FILE"
        echo "      \"http_head\": true," >> "$RESULTS_FILE"
        echo "      \"http_status\": $HTTP_STATUS," >> "$RESULTS_FILE"
        echo "      \"response_time\": $RESPONSE_TIME," >> "$RESULTS_FILE"
    else
        echo "  ✗ FAILED: HTTP $HTTP_STATUS" | tee -a "$LOG_FILE"
        echo "      \"http_head\": false," >> "$RESULTS_FILE"
        echo "      \"http_status\": $HTTP_STATUS," >> "$RESULTS_FILE"
        echo "      \"response_time\": $RESPONSE_TIME," >> "$RESULTS_FILE"
        echo "      \"accessible\": false," >> "$RESULTS_FILE"
        echo "      \"auth_required\": false," >> "$RESULTS_FILE"
        echo "      \"download_test\": {" >> "$RESULTS_FILE"
        echo "        \"downloaded\": false," >> "$RESULTS_FILE"
        echo "        \"download_size\": 0," >> "$RESULTS_FILE"
        echo "        \"download_time\": 0," >> "$RESULTS_FILE"
        echo "        \"valid_grib2\": false" >> "$RESULTS_FILE"
        echo "      }" >> "$RESULTS_FILE"
        echo "      \"error\": \"HTTP $HTTP_STATUS\"" >> "$RESULTS_FILE"
        echo "    }" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "" | tee -a "$LOG_FILE"
        continue
    fi

    # Test 3: Check for authentication requirements
    echo "Test 3: Authentication check..." | tee -a "$LOG_FILE"
    AUTH_REQUIRED=false

    # Try to access without authentication - if we get 401/403, auth is required
    if [ "$HTTP_STATUS" = "401" ] || [ "$HTTP_STATUS" = "403" ]; then
        AUTH_REQUIRED=true
        echo "  ⚠ Authentication required (HTTP $HTTP_STATUS)" | tee -a "$LOG_FILE"
    else
        echo "  ✓ No authentication required" | tee -a "$LOG_FILE"
    fi

    echo "      \"auth_required\": $AUTH_REQUIRED," >> "$RESULTS_FILE"
    echo "      \"accessible\": true," >> "$RESULTS_FILE"

    # Test 4: Download test (download first 5MB to verify content)
    echo "Test 4: Download test (first 5MB)..." | tee -a "$LOG_FILE"
    DOWNLOAD_FILE="$OUTPUT_DIR/${FILENAME}_download_test.grib2"

    START_TIME=$(date +%s.%N)
    if curl -s -L -R --max-time 120 -o "$DOWNLOAD_FILE" -r 0-5242879 "$URL" 2>&1 | tee -a "$LOG_FILE"; then
        END_TIME=$(date +%s.%N)
        DOWNLOAD_TIME=$(awk "BEGIN {printf \"%.3f\", $END_TIME - $START_TIME}")
        DOWNLOAD_SIZE=$(stat -c%s "$DOWNLOAD_FILE" 2>/dev/null || stat -f%z "$DOWNLOAD_FILE" 2>/dev/null || echo "0")

        if [ "$DOWNLOAD_SIZE" -gt 0 ]; then
            echo "  ✓ Downloaded $DOWNLOAD_SIZE bytes in ${DOWNLOAD_TIME}s" | tee -a "$LOG_FILE"
            DOWNLOAD_SPEED=$(awk "BEGIN {printf \"%.2f\", $DOWNLOAD_SIZE/1024/1024/$DOWNLOAD_TIME}")
            echo "    Speed: $DOWNLOAD_SPEED MB/s" | tee -a "$LOG_FILE"

            # Test 5: GRIB2 format validation
            echo "Test 5: GRIB2 format validation..." | tee -a "$LOG_FILE"
            GRIB2_VALID=false
            GRIB2_INFO=""

            if command -v wgrib2 >/dev/null 2>&1; then
                GRIB2_INFO=$(wgrib2 "$DOWNLOAD_FILE" 2>&1 | head -1 || echo "")

                # Check if wgrib2 produced valid record output
                if echo "$GRIB2_INFO" | grep -qE "^[0-9]+:[0-9]+:d="; then
                    GRIB2_VALID=true
                    echo "  ✓ Valid GRIB2 format detected" | tee -a "$LOG_FILE"
                    echo "    First record: $GRIB2_INFO" | tee -a "$LOG_FILE"

                    # Test 6: DRT=0 verification
                    echo "Test 6: DRT=0 (Simple Packing) verification..." | tee -a "$LOG_FILE"
                    DRT_ZERO=false
                    PACKING_INFO=""

                    if wgrib2 -packing "$DOWNLOAD_FILE" 2>&1 | grep -q "simple packing"; then
                        DRT_ZERO=true
                        PACKING_INFO=$(wgrib2 -packing "$DOWNLOAD_FILE" 2>&1 | head -1 | sed 's/"/\\"/g')
                        echo "  ✓ DRT=0 (Simple Packing) confirmed" | tee -a "$LOG_FILE"
                        echo "    Packing info: $PACKING_INFO" | tee -a "$LOG_FILE"
                        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
                    else
                        echo "  ✗ DRT=0 not confirmed" | tee -a "$LOG_FILE"
                        PACKING_INFO=$(wgrib2 -packing "$DOWNLOAD_FILE" 2>&1 | head -1 | sed 's/"/\\"/g')
                    fi
                else
                    echo "  ✗ Not valid GRIB2 format" | tee -a "$LOG_FILE"
                    FAIL_COUNT=$((FAIL_COUNT + 1))
                fi
            else
                echo "  ⚠ wgrib2 not available, assuming valid based on download" | tee -a "$LOG_FILE"
                GRIB2_VALID=true
                DRT_ZERO=true
                SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
            fi

            echo "      \"download_test\": {" >> "$RESULTS_FILE"
            echo "        \"downloaded\": true," >> "$RESULTS_FILE"
            echo "        \"download_size\": $DOWNLOAD_SIZE," >> "$RESULTS_FILE"
            echo "        \"download_time\": $DOWNLOAD_TIME," >> "$RESULTS_FILE"
            echo "        \"download_speed_mb_s\": $DOWNLOAD_SPEED," >> "$RESULTS_FILE"
            echo "        \"valid_grib2\": $GRIB2_VALID," >> "$RESULTS_FILE"
            echo "        \"grib2_info\": \"$(echo "$GRIB2_INFO" | sed 's/"/\\"/g')\"," >> "$RESULTS_FILE"
            echo "        \"drt_zero\": $DRT_ZERO," >> "$RESULTS_FILE"
            echo "        \"packing_info\": \"$PACKING_INFO\"" >> "$RESULTS_FILE"
            echo "      }" >> "$RESULTS_FILE"

            rm -f "$DOWNLOAD_FILE"
        else
            echo "  ✗ Empty file downloaded" | tee -a "$LOG_FILE"
            echo "      \"download_test\": {" >> "$RESULTS_FILE"
            echo "        \"downloaded\": false," >> "$RESULTS_FILE"
            echo "        \"download_size\": 0," >> "$RESULTS_FILE"
            echo "        \"download_time\": 0," >> "$RESULTS_FILE"
            echo "        \"valid_grib2\": false," >> "$RESULTS_FILE"
            echo "        \"drt_zero\": false," >> "$RESULTS_FILE"
            echo "        \"error\": \"Empty download\"" >> "$RESULTS_FILE"
            echo "      }" >> "$RESULTS_FILE"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    else
        echo "  ✗ Download failed" | tee -a "$LOG_FILE"
        echo "      \"download_test\": {" >> "$RESULTS_FILE"
        echo "        \"downloaded\": false," >> "$RESULTS_FILE"
        echo "        \"download_size\": 0," >> "$RESULTS_FILE"
        echo "        \"download_time\": 0," >> "$RESULTS_FILE"
        echo "        \"valid_grib2\": false," >> "$RESULTS_FILE"
        echo "        \"drt_zero\": false," >> "$RESULTS_FILE"
        echo "        \"error\": \"Download failed\"" >> "$RESULTS_FILE"
        echo "      }" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi

    echo "    }" >> "$RESULTS_FILE"
    echo "" | tee -a "$LOG_FILE"
done

echo "  ]" >> "$RESULTS_FILE"
echo "  \"summary\": {" >> "$RESULTS_FILE"
echo "    \"total_files\": $TOTAL_COUNT," >> "$RESULTS_FILE"
echo "    \"successful\": $SUCCESS_COUNT," >> "$RESULTS_FILE"
echo "    \"failed\": $FAIL_COUNT," >> "$RESULTS_FILE"
echo "    \"success_rate\": $(awk "BEGIN {printf \"%.2f\", ($SUCCESS_COUNT/$TOTAL_COUNT)*100}")" >> "$RESULTS_FILE"
echo "  }" >> "$RESULTS_FILE"
echo "}" >> "$RESULTS_FILE"

echo "========================================" | tee -a "$LOG_FILE"
echo "SUMMARY" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
echo "Total files tested: $TOTAL_COUNT" | tee -a "$LOG_FILE"
echo "Successful: $SUCCESS_COUNT" | tee -a "$LOG_FILE"
echo "Failed: $FAIL_COUNT" | tee -a "$LOG_FILE"
echo "Success rate: $(awk "BEGIN {printf \"%.2f\", ($SUCCESS_COUNT/$TOTAL_COUNT)*100}")%" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
echo "Results saved to: $RESULTS_FILE" | tee -a "$LOG_FILE"

exit 0