#!/usr/bin/env bash
#
# Download GRIB2 message headers from candidate files
# For bead bf-3ugst
#

set -euo pipefail

# Configuration
CANDIDATE_JSON="drt_search_results/drt0_candidates_structured.json"
OUTPUT_DIR="headers/bf-3ugst"
MANIFEST_FILE="${OUTPUT_DIR}/manifest.json"
FAILED_LOG="${OUTPUT_DIR}/failed_downloads.log"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Initialize manifest
echo '{"source_bead": "bf-3ugst", "download_date": "'$(date -u +%Y-%m-%d)'", "headers": []}' > "${MANIFEST_FILE}"

# Initialize failed log
echo "Failed header downloads - $(date -u +%Y-%m-%d_%H:%M:%S)" > "${FAILED_LOG}"
echo "========================================" >> "${FAILED_LOG}"

# Read candidates and download headers
candidate_count=$(jq '.candidates | length' "${CANDIDATE_JSON}")
echo "Processing ${candidate_count} candidates..."

# Track success/failure
success_count=0
failed_count=0

for i in $(seq 1 "${candidate_count}"); do
  candidate=$(jq ".candidates[$((i-1))]" "${CANDIDATE_JSON}")

  url=$(echo "${candidate}" | jq -r '.url')
  model=$(echo "${candidate}" | jq -r '.model')
  resolution=$(echo "${candidate}" | jq -r '.resolution')
  date=$(echo "${candidate}" | jq -r '.date')
  forecast_hour=$(echo "${candidate}" | jq -r '.forecast_hour')

  # Create a safe filename from URL
  # Extract filename from URL
  filename=$(basename "${url}")
  header_file="${OUTPUT_DIR}/${filename}.headers.txt"

  echo "Processing [${i}/${candidate_count}]: ${filename}"
  echo "  URL: ${url}"
  echo "  Model: ${model}, Resolution: ${resolution}, Date: ${date}, FH: ${forecast_hour}"

  # Download headers using wgrib2
  # wgrib2 -header dumps all message headers without data
  # Use set -e compatible error handling
  curl_result=0
  curl -sL "${url}" | wgrib2 -header - > "${header_file}" 2>/dev/null || curl_result=$?

  if [ "${curl_result}" -eq 0 ] && [ -s "${header_file}" ]; then
    echo "  ✅ Success: Headers saved to ${header_file}"

    # Count messages in header
    message_count=$(wc -l < "${header_file}" | tr -d ' ')
    echo "  Messages: ${message_count}"

    # Update manifest
    temp_manifest=$(mktemp)
    jq '.headers += [{
      "candidate_id": '"${i}"',
      "url": "'"${url}"'",
      "model": "'"${model}"'",
      "resolution": "'"${resolution}"'",
      "date": "'"${date}"'",
      "forecast_hour": "'"${forecast_hour}"'",
      "header_file": "'"${filename}.headers.txt"'",
      "message_count": '"${message_count}"',
      "status": "success",
      "download_timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
    }]' "${MANIFEST_FILE}" > "${temp_manifest}"
    mv "${temp_manifest}" "${MANIFEST_FILE}"

    ((success_count+=1))
  else
    echo "  ❌ Failed: Download or processing error (exit code: ${curl_result})"
    echo "${url} - Download/processing error (exit code: ${curl_result})" >> "${FAILED_LOG}"
    rm -f "${header_file}"
    ((failed_count+=1))
  fi

  echo ""
done

# Final summary
echo "========================================"
echo "Header Download Summary"
echo "========================================"
echo "Total candidates: ${candidate_count}"
echo "Successful: ${success_count}"
echo "Failed: ${failed_count}"
echo "Output directory: ${OUTPUT_DIR}"
echo "Manifest: ${MANIFEST_FILE}"
echo "Failed log: ${FAILED_LOG}"
echo ""

# Update manifest with summary
temp_manifest=$(mktemp)
jq '{
  source_bead: .source_bead,
  download_date: .download_date,
  summary: {
    total_candidates: '"${candidate_count}"',
    successful: '"${success_count}"',
    failed: '"${failed_count}"',
    success_rate: "'"$(awk "BEGIN {printf \"%.1f\", ${success_count}*100/${candidate_count}}")%"'"
  },
  headers: .headers
}' "${MANIFEST_FILE}" > "${temp_manifest}"
mv "${temp_manifest}" "${MANIFEST_FILE}"

# Exit with error if any failures
if [ "${failed_count}" -gt 0 ]; then
  echo "⚠️  Some candidates failed - check ${FAILED_LOG}"
  exit 1
fi

echo "✅ All header downloads completed successfully!"
exit 0
