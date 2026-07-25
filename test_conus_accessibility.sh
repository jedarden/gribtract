#!/bin/bash
# Test accessibility of verified DRT=0 CONUS files
# Bead: bf-14grj

set -e

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
OUTPUT_FILE="drt0_conus_accessibility_${TIMESTAMP}.json"
TEMP_DIR="/tmp/gribtract_accessibility_test_$$"
mkdir -p "$TEMP_DIR"

echo "Starting accessibility test for verified DRT=0 CONUS files..."
echo "Results will be saved to: $OUTPUT_FILE"

# Start JSON output
cat > "$OUTPUT_FILE" << EOF
{
  "timestamp": "$(date -Iseconds)",
  "summary": {
    "total": 7,
    "tested": 0,
    "accessible": 0,
    "downloaded": 0,
    "valid_grib2": 0
  },
  "results": []
}
EOF

# Array of files to test with their source URLs
declare -a FILES=(
  "/home/coding/gribtract/drt_search_results/gfs_1p00_20260724_f000.grib2|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000"
  "/home/coding/gribtract/drt_search_results/gfs_0p25_20260723_f000.grib2|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000"
  "/home/coding/gribtract/drt_search_results/gefs_0p50_f000.grib2|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"
  "/home/coding/gribtract/drt_search_results/gefs_0p50_f003.grib2|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003"
  "/home/coding/gribtract/drt_search_results/gfs_1p00_20260723_f000.grib2|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000"
  "/home/coding/gribtract/drt_search_results/gfs_0p50_20260724_f000.grib2|https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000"
  "/home/coding/gribtract/drt_search_results/gefs_0p50_f006.grib2|https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006"
)

TOTAL=${#FILES[@]}
TESTED=0
ACCESSIBLE=0
DOWNLOADED=0
VALID_GRIB=0

for entry in "${FILES[@]}"; do
  IFS='|' read -r LOCAL_PATH URL <<< "$entry"
  FILENAME=$(basename "$LOCAL_PATH")

  echo "Testing: $FILENAME"
  echo "URL: $URL"

  TESTED=$((TESTED + 1))

  # Check if local file exists and is valid
  LOCAL_EXISTS=false
  LOCAL_VALID=false
  if [ -f "$LOCAL_PATH" ] && [ -s "$LOCAL_PATH" ]; then
    LOCAL_EXISTS=true
    # Validate local file is GRIB2
    if wgrib2 "$LOCAL_PATH" -match "" >/dev/null 2>&1; then
      LOCAL_VALID=true
    fi
  fi

  # Test HTTP accessibility
  HTTP_STATUS=""
  AUTH_REQUIRED=false
  ACCESSIBLE=false

  HTTP_RESPONSE=$(curl -s -I -L "$URL" 2>&1 || echo "FAILED")

  if echo "$HTTP_RESPONSE" | grep -q "HTTP/"; then
    HTTP_STATUS=$(echo "$HTTP_RESPONSE" | grep "HTTP/" | head -1 | awk '{print $2}')
    if [ "$HTTP_STATUS" = "200" ]; then
      ACCESSIBLE=true
      ACCESSIBLE=$((ACCESSIBLE + 1))
    fi
  elif echo "$HTTP_RESPONSE" | grep -q "403\|401"; then
    AUTH_REQUIRED=true
  fi

  # Try downloading a sample (first 1MB) to verify it's retrievable
  DOWNLOADED_FILE=false
  DOWNLOAD_VALID=false
  DOWNLOAD_SIZE=0
  DOWNLOAD_TIME=0
  SAMPLE_FILE="$TEMP_DIR/${FILENAME}_sample"

  if [ "$ACCESSIBLE" = true ]; then
    START_TIME=$(date +%s.%N)
    curl -s -L -r 0-1048576 "$URL" -o "$SAMPLE_FILE" 2>/dev/null && DOWNLOADED_FILE=true
    END_TIME=$(date +%s.%N)
    DOWNLOAD_TIME=$(echo "$END_TIME - $START_TIME" | bc)

    if [ "$DOWNLOADED_FILE" = true ] && [ -s "$SAMPLE_FILE" ]; then
      DOWNLOAD_SIZE=$(stat -f%z "$SAMPLE_FILE" 2>/dev/null || stat -c%s "$SAMPLE_FILE" 2>/dev/null || echo "0")
      # Validate sample is GRIB2
      if wgrib2 "$SAMPLE_FILE" -match "" >/dev/null 2>&1; then
        DOWNLOAD_VALID=true
        DOWNLOADED=$((DOWNLOADED + 1))
        VALID_GRIB=$((VALID_GRIB + 1))
      fi
    fi
  fi

  # Build JSON entry for this file
  cat >> "$OUTPUT_FILE.tmp" << EOF
    {
      "filename": "$FILENAME",
      "local_path": "$LOCAL_PATH",
      "url": "$URL",
      "local_exists": $LOCAL_EXISTS,
      "local_valid_grib2": $LOCAL_VALID,
      "http_status": "$HTTP_STATUS",
      "accessible": $ACCESSIBLE,
      "auth_required": $AUTH_REQUIRED,
      "download_test": {
        "downloaded": $DOWNLOADED_FILE,
        "download_size": $DOWNLOAD_SIZE,
        "download_time": $DOWNLOAD_TIME,
        "valid_grib2": $DOWNLOAD_VALID
      }
    },
EOF

  echo "  - Local exists: $LOCAL_EXISTS, valid: $LOCAL_VALID"
  echo "  - HTTP status: $HTTP_STATUS"
  echo "  - Accessible: $ACCESSIBLE, Auth required: $AUTH_REQUIRED"
  echo "  - Download test: $DOWNLOADED_FILE, size: $DOWNLOAD_SIZE bytes, valid GRIB2: $DOWNLOAD_VALID"
  echo ""

  rm -f "$SAMPLE_FILE"
done

# Update summary and finalize JSON
python3 - << EOF
import json

with open('$OUTPUT_FILE', 'r') as f:
    data = json.load(f)

with open('$OUTPUT_FILE.tmp', 'r') as f:
    results_text = f.read()
    # Remove trailing comma and add brackets
    results_text = results_text.rstrip(',\n')
    results = json.loads('[' + results_text + ']')

data['summary']['tested'] = $TESTED
data['summary']['accessible'] = $ACCESSIBLE
data['summary']['downloaded'] = $DOWNLOADED
data['summary']['valid_grib2'] = $VALID_GRIB
data['results'] = results

with open('$OUTPUT_FILE', 'w') as f:
    json.dump(data, f, indent=2)

print("Summary:")
print(f"  Total files: {data['summary']['total']}")
print(f"  Tested: {data['summary']['tested']}")
print(f"  Accessible: {data['summary']['accessible']}")
print(f"  Download test successful: {data['summary']['downloaded']}")
print(f"  Valid GRIB2: {data['summary']['valid_grib2']}")
EOF

rm -f "$OUTPUT_FILE.tmp"
rm -rf "$TEMP_DIR"

echo ""
echo "Accessibility test complete!"
echo "Results saved to: $OUTPUT_FILE"
echo ""
echo "Summary:"
echo "  Total files: $TOTAL"
echo "  Tested: $TESTED"
echo "  Accessible: $ACCESSIBLE"
echo "  Download test successful: $DOWNLOADED"
echo "  Valid GRIB2: $VALID_GRIB"
