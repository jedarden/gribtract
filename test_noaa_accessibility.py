#!/usr/bin/env python3
"""
NOAA Archive File Accessibility Test
Tests documented DRT=0 CONUS GRIB files for HTTP accessibility and file integrity
"""

import urllib.request
import urllib.error
import hashlib
import sys
from pathlib import Path
from typing import Dict, List, Tuple

# Documented URLs from VERIFIED_DRT0_CONUS_FILES.md
TEST_FILES = [
    {
        "name": "gfs_1p00_20260724_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000",
        "expected_size": 42755881,  # ~40.8 MB
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_1p00_20260724_f000.grib2"
    },
    {
        "name": "gfs_0p25_20260723_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000",
        "expected_size": 510275792,  # ~487 MB
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p25_20260723_f000.grib2"
    },
    {
        "name": "gefs_0p50_f000",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000",
        "expected_size": 14272610,  # ~13.6 MB
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f000.grib2"
    },
    {
        "name": "gefs_0p50_f003",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003",
        "expected_size": 0,  # Unknown size
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f003.grib2"
    },
    {
        "name": "gfs_1p00_20260723_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000",
        "expected_size": 0,  # Unknown size
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_1p00_20260723_f000.grib2"
    },
    {
        "name": "gfs_0p50_20260724_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000",
        "expected_size": 152106356,  # ~146 MB
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p50_20260724_f000.grib2"
    },
    {
        "name": "gefs_0p50_f006",
        "url": "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006",
        "expected_size": 0,  # Unknown size
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f006.grib2"
    },
    {
        "name": "gfs_0p25_20260724_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000",
        "expected_size": 0,  # Unknown size
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p25_20260724_f000.grib2"
    },
    {
        "name": "gfs_0p50_20260723_f000",
        "url": "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000",
        "expected_size": 0,  # Unknown size
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p50_20260723_f000.grib2"
    },
]

def check_http_head(url: str) -> Tuple[bool, int, str]:
    """Check HTTP HEAD request for URL accessibility and size"""
    try:
        req = urllib.request.Request(url, method='HEAD')
        with urllib.request.urlopen(req, timeout=10) as response:
            size = int(response.getheader('Content-Length', 0))
            return True, size, "HTTP 200 OK"
    except urllib.error.HTTPError as e:
        return False, 0, f"HTTP Error: {e.code}"
    except urllib.error.URLError as e:
        return False, 0, f"URL Error: {e.reason}"
    except Exception as e:
        return False, 0, f"Error: {str(e)}"

def check_grib2_magic(file_path: str) -> bool:
    """Check if file has valid GRIB2 magic bytes"""
    try:
        with open(file_path, 'rb') as f:
            magic = f.read(16)
            return magic.startswith(b'GRIB') and magic[7:8] == b'\x02'
    except Exception:
        return False

def get_file_size(file_path: str) -> int:
    """Get local file size in bytes"""
    try:
        return Path(file_path).stat().st_size
    except Exception:
        return 0

def format_size(size_bytes: int) -> str:
    """Format bytes to human-readable size"""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.1f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.1f} TB"

def test_all_files() -> List[Dict]:
    """Test all documented files for accessibility and integrity"""
    results = []

    print("Testing NOAA Archive File Accessibility")
    print("=" * 80)
    print()

    for file_info in TEST_FILES:
        result = {
            "name": file_info["name"],
            "url": file_info["url"],
            "accessible": False,
            "remote_size": 0,
            "local_exists": False,
            "local_size": 0,
            "local_valid_grib2": False,
            "error": None
        }

        # Test HTTP accessibility
        accessible, remote_size, status = check_http_head(file_info["url"])
        result["accessible"] = accessible
        result["remote_size"] = remote_size
        result["http_status"] = status

        # Check local file
        local_path = Path(file_info["local_path"])
        if local_path.exists():
            result["local_exists"] = True
            result["local_size"] = get_file_size(file_info["local_path"])
            result["local_valid_grib2"] = check_grib2_magic(file_info["local_path"])

        if not accessible:
            result["error"] = status

        results.append(result)

    return results

def print_results(results: List[Dict]):
    """Print test results in formatted table"""
    print(f"{'File Name':<25} {'HTTP Status':<12} {'Remote Size':>12} {'Local Size':>12} {'GRIB2':<6}")
    print("-" * 80)

    accessible_count = 0
    valid_local_count = 0

    for r in results:
        http_status = "✓ 200 OK" if r["accessible"] else f"✗ {r['http_status'][:20]}"
        remote_size = format_size(r["remote_size"]) if r["remote_size"] > 0 else "Unknown"
        local_size = format_size(r["local_size"]) if r["local_size"] > 0 else "N/A"
        grib2_status = "✓" if r["local_valid_grib2"] else ("✗" if r["local_exists"] else "N/A")

        print(f"{r['name']:<25} {http_status:<12} {remote_size:>12} {local_size:>12} {grib2_status:<6}")

        if r["accessible"]:
            accessible_count += 1
        if r["local_valid_grib2"]:
            valid_local_count += 1

    print("-" * 80)
    print(f"Summary: {accessible_count}/{len(results)} accessible, {valid_local_count}/{len(results)} valid local files")
    print()

def main():
    """Main test execution"""
    results = test_all_files()
    print_results(results)

    # Check for authentication requirements
    print("Authentication Check:")
    print("-" * 40)
    auth_needed = False
    for r in results:
        if "403" in r.get("http_status", "") or "401" in r.get("http_status", ""):
            auth_needed = True
            print(f"  ✗ {r['name']}: Requires authentication")

    if not auth_needed:
        print("  ✓ No authentication required for any files")
    print()

    # Download recommendations
    print("Recommendations:")
    print("-" * 40)
    for r in results:
        if r["accessible"] and not r["local_exists"]:
            print(f"  → {r['name']}: Download available, no local copy")
        elif r["accessible"] and r["local_exists"] and r["local_size"] == 0:
            print(f"  → {r['name']}: Re-download required (corrupt local copy)")
    print()

    return 0

if __name__ == "__main__":
    sys.exit(main())
