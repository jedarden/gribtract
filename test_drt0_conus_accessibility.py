#!/usr/bin/env python3
"""
Test HTTP/HTTPS accessibility for verified DRT=0 CONUS files.
Tests download capability, authentication requirements, and file integrity.
"""

import requests
import subprocess
import json
import os
import time
from pathlib import Path
from datetime import datetime
import hashlib

# Verified DRT=0 CONUS files from VERIFIED_DRT0_CONUS_FILES.md
DRT0_CONUS_FILES = [
    {
        "candidate_id": "gfs_1p00_20260724_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000",
        "model": "GFS",
        "resolution": "1.00°",
        "expected_size_mb": 40.8
    },
    {
        "candidate_id": "gfs_0p25_20260723_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000",
        "model": "GFS",
        "resolution": "0.25°",
        "expected_size_mb": 487
    },
    {
        "candidate_id": "gefs_0p50_20260724_f000",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000",
        "model": "GEFS",
        "resolution": "0.50°",
        "expected_size_mb": 13.6
    },
    {
        "candidate_id": "gefs_0p50_20260724_f003",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003",
        "model": "GEFS",
        "resolution": "0.50°",
        "expected_size_mb": 14.6
    },
    {
        "candidate_id": "gfs_1p00_20260723_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000",
        "model": "GFS",
        "resolution": "1.00°",
        "expected_size_mb": 40.5
    },
    {
        "candidate_id": "gfs_0p50_20260724_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000",
        "model": "GFS",
        "resolution": "0.50°",
        "expected_size_mb": 145
    },
    {
        "candidate_id": "gefs_0p50_20260724_f006",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006",
        "model": "GEFS",
        "resolution": "0.50°",
        "expected_size_mb": 14.0
    }
]

def test_http_access(url, timeout=30):
    """Test HTTP accessibility and return headers and status."""
    try:
        response = requests.head(url, timeout=timeout, allow_redirects=True)
        headers = dict(response.headers)

        return {
            "http_status": response.status_code,
            "http_headers": headers,
            "accessible": response.status_code == 200,
            "auth_required": response.status_code == 401 or response.status_code == 403,
            "error_message": None if response.status_code == 200 else f"HTTP {response.status_code}"
        }
    except requests.exceptions.Timeout:
        return {
            "http_status": None,
            "http_headers": {},
            "accessible": False,
            "auth_required": False,
            "error_message": "Request timeout"
        }
    except requests.exceptions.RequestException as e:
        return {
            "http_status": None,
            "http_headers": {},
            "accessible": False,
            "auth_required": False,
            "error_message": str(e)
        }

def download_file(url, output_path, timeout=300):
    """Download a file and track performance metrics."""
    try:
        start_time = time.time()
        response = requests.get(url, timeout=timeout, stream=True)

        if response.status_code != 200:
            return {
                "downloaded": False,
                "download_size": 0,
                "download_time": 0.0,
                "download_speed": 0.0,
                "error_message": f"HTTP {response.status_code}"
            }

        total_size = 0
        with open(output_path, 'wb') as f:
            for chunk in response.iter_content(chunk_size=8192):
                if chunk:
                    f.write(chunk)
                    total_size += len(chunk)

        download_time = time.time() - start_time
        download_speed = total_size / download_time if download_time > 0 else 0

        return {
            "downloaded": True,
            "download_size": total_size,
            "download_time": download_time,
            "download_speed": download_speed,
            "error_message": None
        }
    except Exception as e:
        return {
            "downloaded": False,
            "download_size": 0,
            "download_time": 0.0,
            "download_speed": 0.0,
            "error_message": str(e)
        }

def verify_grib2_integrity(file_path):
    """Verify GRIB2 file integrity using wgrib2."""
    try:
        # Check if file exists and has content
        if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
            return {
                "file_valid_grib2": False,
                "wgrib2_output": None,
                "error_message": "File is empty or does not exist"
            }

        # Run wgrib2 to check if it's a valid GRIB2 file
        result = subprocess.run(
            ['wgrib2', '-o', '/dev/null', file_path],
            capture_output=True,
            text=True,
            timeout=60
        )

        # If return code is 0, wgrib2 could read the file
        is_valid = result.returncode == 0

        return {
            "file_valid_grib2": is_valid,
            "wgrib2_output": result.stderr if result.returncode != 0 else "GRIB2 file validated",
            "error_message": result.stderr if result.returncode != 0 else None
        }
    except subprocess.TimeoutExpired:
        return {
            "file_valid_grib2": False,
            "wgrib2_output": None,
            "error_message": "wgrib2 verification timeout"
        }
    except Exception as e:
        return {
            "file_valid_grib2": False,
            "wgrib2_output": None,
            "error_message": str(e)
        }

def calculate_md5(file_path):
    """Calculate MD5 checksum of a file."""
    try:
        md5_hash = hashlib.md5()
        with open(file_path, 'rb') as f:
            for chunk in iter(lambda: f.read(8192), b''):
                md5_hash.update(chunk)
        return md5_hash.hexdigest()
    except Exception:
        return None

def main():
    """Main test function."""
    timestamp = datetime.now().isoformat()
    results = []

    # Create output directory for test downloads
    test_dir = Path("/home/coding/gribtract/accessibility_test_downloads")
    test_dir.mkdir(exist_ok=True)

    print(f"Testing {len(DRT0_CONUS_FILES)} DRT=0 CONUS files for accessibility...")
    print(f"Test timestamp: {timestamp}")
    print()

    for file_info in DRT0_CONUS_FILES:
        print(f"Testing: {file_info['candidate_id']} ({file_info['model']} {file_info['resolution']})")
        print(f"  URL: {file_info['url']}")

        result = {
            "candidate_id": file_info["candidate_id"],
            "url": file_info["url"],
            "model": file_info["model"],
            "resolution": file_info["resolution"],
            "expected_size_mb": file_info["expected_size_mb"]
        }

        # Step 1: Test HTTP access
        print("  → Testing HTTP access...")
        http_result = test_http_access(file_info["url"])
        result.update(http_result)

        if not result["accessible"]:
            print(f"    ✗ FAILED: {result['error_message']}")
            results.append(result)
            print()
            continue

        print(f"    ✓ OK (HTTP {result['http_status']})")

        # Step 2: Download file
        print("  → Downloading file...")
        output_path = test_dir / f"{file_info['candidate_id']}.grib2"
        download_result = download_file(file_info["url"], output_path)
        result.update(download_result)

        if not result["downloaded"]:
            print(f"    ✗ FAILED: {result['error_message']}")
            results.append(result)
            print()
            continue

        actual_size_mb = result["download_size"] / (1024 * 1024)
        print(f"    ✓ Downloaded {actual_size_mb:.1f} MB in {result['download_time']:.1f}s")
        print(f"      Speed: {result['download_speed'] / (1024*1024):.1f} MB/s")

        # Step 3: Verify GRIB2 integrity
        print("  → Verifying GRIB2 integrity...")
        integrity_result = verify_grib2_integrity(output_path)
        result.update(integrity_result)

        if result["file_valid_grib2"]:
            print("    ✓ Valid GRIB2 file")
        else:
            print(f"    ✗ FAILED: {result['error_message']}")

        # Calculate MD5 for verification
        result["md5_checksum"] = calculate_md5(output_path)
        if result["md5_checksum"]:
            print(f"    MD5: {result['md5_checksum']}")

        results.append(result)
        print()

    # Generate summary
    total_files = len(results)
    accessible = sum(1 for r in results if r["accessible"])
    downloaded = sum(1 for r in results if r["downloaded"])
    valid_grib2 = sum(1 for r in results if r["file_valid_grib2"])

    summary = {
        "timestamp": timestamp,
        "total_tested": total_files,
        "accessible": accessible,
        "downloaded": downloaded,
        "valid_grib2": valid_grib2,
        "success_rate": (valid_grib2 / total_files * 100) if total_files > 0 else 0,
        "results": results
    }

    # Save results
    output_file = Path("/home/coding/gribtract/drt0_conus_accessibility_final.json")
    with open(output_file, 'w') as f:
        json.dump(summary, f, indent=2)

    print("=" * 70)
    print("FINAL SUMMARY")
    print("=" * 70)
    print(f"Total files tested: {total_files}")
    print(f"HTTP accessible: {accessible}/{total_files}")
    print(f"Successfully downloaded: {downloaded}/{total_files}")
    print(f"Valid GRIB2 files: {valid_grib2}/{total_files}")
    print(f"Success rate: {summary['success_rate']:.1f}%")
    print()
    print(f"Results saved to: {output_file}")

    return summary

if __name__ == "__main__":
    main()