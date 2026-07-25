#!/bin/bash
# Test HTTP/HTTPS accessibility for verified DRT=0 CONUS files
# Tests download capability, authentication requirements, and file integrity

set -e

# Configuration
OUTPUT_DIR="/home/coding/gribtract/accessibility_test_downloads"
RESULTS_FILE="/home/coding/gribtract/drt0_conus_accessibility_final.json"
mkdir -p "$OUTPUT_DIR"

# Create results file with initial structure
cat > "$RESULTS_FILE" << EOF
{
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%S")",
  "total_tested": 7,
  "results": []
}
