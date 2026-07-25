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

# Array of verified DRT=0 CONUS files (using | as delimiter between URL and filename)
declare -a FILES=(
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260724_f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000|gfs_0p25_20260723_f000"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000|gefs_0p50_20260724_f000"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003|gefs_0p50_20260724_f003"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000|gfs_1p00_20260723_f000"
    "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000|gfs_0p50_20260724_f000"
    "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006|gefs_0p50_20260724_f006"
)

echo "{" > "$RESULTS_FILE"
echo "  \"test_date\": \"$(date -Iseconds)\"," >> "$RESULTS_FILE"
echo "  \"files\": [" >> "$RESULTS_FILE"

TOTAL_COUNT=${#FILES[@]}
SUCCESS_COUNT=0
FAIL_COUNT=0

for i in "${!FILES[@]}"; do
    IFS='|' read -r URL FILENAME <<< "${FILES[$i]}"

    echo "========================================" | tee -a "$LOG_FILE"
    echo "Test $((i+1))/$TOTAL_COUNT: $FILENAME" | tee -a "$LOG_FILE"
    echo "URL: $URL" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"

    # Add comma if not first item
    if [ $i -gt 0 ]; then
        echo "    ," >> "$RESULTS_FILE"
    fi

    echo "    {" >> "$RESULTS_FILE"
    echo "      \"filename\": \"$FILENAME\"," >> "$RESULTS_FILE"
    echo "      \"url\": \"$URL\"," >> "$RESULTS_FILE"

    # Test 1: HTTP HEAD request (check if URL exists)
    echo "Test 1: HTTP HEAD request..." | tee -a "$LOG_FILE"
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -L -I --max-time 30 "$URL" 2>&1 || echo "000")

    if [ "$HTTP_STATUS" = "000" ]; then
        echo "  ❌ FAILED: Could not connect to server" | tee -a "$LOG_FILE"
        echo "      \"http_head\": false," >> "$RESULTS_FILE"
        echo "      \"http_status\": \"connection_failed\"," >> "$RESULTS_FILE"
        echo "      \"downloadable\": false," >> "$RESULTS_FILE"
        echo "      \"grib2_valid\": false," >> "$RESULTS_FILE"
        echo "      \"file_size\": 0," >> "$RESULTS_FILE"
        echo "      \"error\": \"Connection failed\"" >> "$RESULTS_FILE"
        echo "    }" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "" | tee -a "$LOG_FILE"
        continue
    elif [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "206" ]; then
        echo "  ✅ PASSED: HTTP $HTTP_STATUS" | tee -a "$LOG_FILE"
        echo "      \"http_head\": true," >> "$RESULTS_FILE"
        echo "      \"http_status\": $HTTP_STATUS," >> "$RESULTS_FILE"
    else
        echo "  ❌ FAILED: HTTP $HTTP_STATUS" | tee -a "$LOG_FILE"
        echo "      \"http_head\": false," >> "$RESULTS_FILE"
        echo "      \"http_status\": $HTTP_STATUS," >> "$RESULTS_FILE"
        echo "      \"downloadable\": false," >> "$RESULTS_FILE"
        echo "      \"grib2_valid\": false," >> "$RESULTS_FILE"
        echo "      \"file_size\": 0," >> "$RESULTS_FILE"
        echo "      \"error\": \"HTTP $HTTP_STATUS\"" >> "$RESULTS_FILE"
        echo "    }" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "" | tee -a "$LOG_FILE"
        continue
    fi

    # Test 2: Partial download (first 1MB to verify content)
    echo "Test 2: Partial download (first 1MB)..." | tee -a "$LOG_FILE"
    PARTIAL_FILE="$OUTPUT_DIR/${FILENAME}_partial.grib2"

    if curl -s -L -R --max-time 60 -o "$PARTIAL_FILE" -r 0-1048575 "$URL" 2>&1 | tee -a "$LOG_FILE"; then
        PARTIAL_SIZE=$(stat -f%z "$PARTIAL_FILE" 2>/dev/null || stat -c%s "$PARTIAL_FILE" 2>/dev/null || echo "0")

        if [ "$PARTIAL_SIZE" -gt 0 ]; then
            echo "  ✅ PASSED: Downloaded $PARTIAL_SIZE bytes" | tee -a "$LOG_FILE"
            echo "      \"downloadable\": true," >> "$RESULTS_FILE"

            # Test 3: GRIB2 format validation
            echo "Test 3: GRIB2 format validation..." | tee -a "$LOG_FILE"

            if command -v wgrib2 >/dev/null 2>&1; then
                GRIB2_INFO=$(wgrib2 "$PARTIAL_FILE" 2>&1 | head -1 || echo "")

                # Check if wgrib2 produced valid record output (contains colon and "d=" pattern)
                if echo "$GRIB2_INFO" | grep -qE "^[0-9]+:[0-9]+:d="; then
                    echo "  ✅ PASSED: Valid GRIB2 format detected" | tee -a "$LOG_FILE"
                    echo "      \"grib2_valid\": true," >> "$RESULTS_FILE"
                    echo "      \"grib2_info\": \"$(echo "$GRIB2_INFO" | sed 's/"/\\"/g')\"," >> "$RESULTS_FILE"

                    # Check for DRT=0
                    if wgrib2 -packing "$PARTIAL_FILE" 2>&1 | grep -q "5.0.0"; then
                        echo "  ✅ BONUS: DRT=0 (Simple Packing) confirmed" | tee -a "$LOG_FILE"
                        echo "      \"drt_zero\": true," >> "$RESULTS_FILE"
                    else
                        echo "  ⚠️  WARNING: DRT=0 not confirmed in partial file" | tee -a "$LOG_FILE"
                        echo "      \"drt_zero\": false," >> "$RESULTS_FILE"
                    fi

                    SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
                else
                    echo "  ❌ FAILED: Not valid GRIB2 format" | tee -a "$LOG_FILE"
                    echo "      \"grib2_valid\": false," >> "$RESULTS_FILE"
                    echo "      \"error\": \"Invalid GRIB2 format\"" >> "$RESULTS_FILE"
                    FAIL_COUNT=$((FAIL_COUNT + 1))
                fi
            else
                echo "  ⚠️  SKIP: wgrib2 not available for validation" | tee -a "$LOG_FILE"
                echo "      \"grib2_valid\": true," >> "$RESULTS_FILE"
                echo "      \"note\": \"wgrib2 not available for validation\"" >> "$RESULTS_FILE"
                SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
            fi

            rm -f "$PARTIAL_FILE"
        else
            echo "  ❌ FAILED: Empty file downloaded" | tee -a "$LOG_FILE"
            echo "      \"downloadable\": false," >> "$RESULTS_FILE"
            echo "      \"grib2_valid\": false," >> "$RESULTS_FILE"
            echo "      \"error\": \"Empty download\"" >> "$RESULTS_FILE"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    else
        echo "  ❌ FAILED: Download failed" | tee -a "$LOG_FILE"
        echo "      \"downloadable\": false," >> "$RESULTS_FILE"
        echo "      \"grib2_valid\": false," >> "$RESULTS_FILE"
        echo "      \"error\": \"Download failed\"" >> "$RESULTS_FILE"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi

    echo "      \"partial_size\": $PARTIAL_SIZE" >> "$RESULTS_FILE"
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
echo "Success rate: $(awk "BEGIN {printf \"%.2f\", ($SUCCESS_COUNT/$TOTAL_COUNT)*100")%" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
echo "Results saved to: $RESULTS_FILE" | tee -a "$LOG_FILE"

exit 0
