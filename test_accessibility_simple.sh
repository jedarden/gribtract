#!/bin/bash
# Test HTTP/HTTPS accessibility for verified DRT=0 CONUS files

OUTPUT_DIR="/home/coding/gribtract/accessibility_test_downloads"
mkdir -p "$OUTPUT_DIR"

echo "=========================================="
echo "DRT=0 CONUS Files Accessibility Test"
echo "=========================================="
echo "Test started: $(date -u)"
echo ""

# Test each file
test_file() {
  local candidate_id="$1"
  local url="$2"
  local model="$3"
  local resolution="$4"
  
  echo "Testing: $candidate_id ($model $resolution)"
  echo "  URL: $url"
  
  # Test HTTP access
  echo "  → Testing HTTP access..."
  local http_status=$(curl -s -o /dev/null -w "%{http_code}" --max-time 30 "$url" 2>&1 || echo "000")
  
  if [ "$http_status" != "200" ]; then
    echo "    ✗ FAILED: HTTP $http_status"
    echo ""
    return 1
  fi
  
  echo "    ✓ OK (HTTP $http_status)"
  
  # Download file
  echo "  → Downloading file..."
  local output_file="$OUTPUT_DIR/${candidate_id}.grib2"
  
  local download_start=$(date +%s)
  curl -s -o "$output_file" --max-time 300 "$url"
  local download_end=$(date +%s)
  local download_time=$((download_end - download_start))
  
  if [ ! -s "$output_file" ]; then
    echo "    ✗ FAILED: Download failed or file is empty"
    echo ""
    return 1
  fi
  
  local download_size=$(stat -c%s "$output_file" 2>/dev/null)
  local download_size_mb=$(echo "scale=1; $download_size / 1048576" | bc)
  
  echo "    ✓ Downloaded ${download_size_mb} MB in ${download_time}s"
  
  if [ "$download_time" -gt 0 ]; then
    local download_speed=$(echo "scale=1; $download_size / $download_time / 1048576" | bc)
    echo "      Speed: ${download_speed} MB/s"
  fi
  
  # Verify GRIB2 integrity
  echo "  → Verifying GRIB2 integrity..."
  if wgrib2 "$output_file" >/dev/null 2>&1; then
    echo "    ✓ Valid GRIB2 file"
  else
    echo "    ✗ FAILED: Invalid GRIB2 file"
  fi
  
  echo ""
  return 0
}

# Test all 7 files
test_file "gfs_1p00_20260724_f000" "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000" "GFS" "1.00°"

test_file "gfs_0p25_20260723_f000" "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000" "GFS" "0.25°"

test_file "gefs_0p50_20260724_f000" "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000" "GEFS" "0.50°"

test_file "gefs_0p50_20260724_f003" "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003" "GEFS" "0.50°"

test_file "gfs_1p00_20260723_f000" "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000" "GFS" "1.00°"

test_file "gfs_0p50_20260724_f000" "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000" "GFS" "0.50°"

test_file "gefs_0p50_20260724_f006" "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006" "GEFS" "0.50°"

echo "=========================================="
echo "Test completed: $(date -u)"
echo "Downloads saved to: $OUTPUT_DIR"
