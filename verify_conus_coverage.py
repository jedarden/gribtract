#!/usr/bin/env python3
"""
CONUS Geographic Coverage Verification for DRT=0 Candidates

Analyzes GRIB2 files to verify they include CONUS (contiguous US) coverage:
- Latitude: 24°N to 50°N
- Longitude: 125°W to 67°W (235°E to 293°E in 0-360° notation)
"""

import json
import subprocess
import re
from pathlib import Path
from typing import Dict, List, Tuple

# CONUS bounding box
CONUS_LAT_MIN = 24.0
CONUS_LAT_MAX = 50.0
CONUS_LON_MIN = 235.0  # 125°W = 235°E
CONUS_LON_MAX = 293.0  # 67°W = 293°E

# DRT=0 candidates from previous verification
CANDIDATES = [
    {
        "candidate_id": "gfs_1p00_20260724_f000",
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_1p00_20260724_f000.grib2"
    },
    {
        "candidate_id": "gfs_0p25_20260723_f000",
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p25_20260723_f000.grib2"
    },
    {
        "candidate_id": "gefs_0p50_20260724_f000",
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f000.grib2"
    },
    {
        "candidate_id": "gefs_0p50_20260724_f003",
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f003.grib2"
    },
    {
        "candidate_id": "gfs_1p00_20260723_f000",
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_1p00_20260723_f000.grib2"
    },
    {
        "candidate_id": "gfs_0p50_20260724_f000",
        "local_path": "/home/coding/gribtract/drt_search_results/gfs_0p50_20260724_f000.grib2"
    },
    {
        "candidate_id": "gefs_0p50_20260724_f006",
        "local_path": "/home/coding/gribtract/drt_search_results/gefs_0p50_f006.grib2"
    }
]

def extract_grid_info(grib_file: str) -> Dict:
    """Extract grid information from GRIB2 file using wgrib2."""
    try:
        result = subprocess.run(
            ['wgrib2', '-grid', grib_file],
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            return {"error": f"wgrib2 failed: {result.stderr}"}

        # Parse grid information
        grid_info = parse_grid_output(result.stdout)
        return grid_info

    except subprocess.TimeoutExpired:
        return {"error": "wgrib2 timeout"}
    except Exception as e:
        return {"error": str(e)}

def parse_grid_output(output: str) -> Dict:
    """Parse wgrib2 -grid output to extract grid parameters."""
    grid_info = {}

    # Extract lat-lon grid information
    lat_pattern = r'lat\s+([\d.-]+)\s+to\s+([\d.-]+)\s+by\s+([\d.-]+)'
    lon_pattern = r'lon\s+([\d.-]+)\s+to\s+([\d.-]+)\s+by\s+([\d.-]+)'
    grid_dim_pattern = r'lat-lon grid:\((\d+)\s+x\s+(\d+)\)'

    lat_match = re.search(lat_pattern, output)
    lon_match = re.search(lon_pattern, output)
    grid_match = re.search(grid_dim_pattern, output)

    if lat_match and lon_match and grid_match:
        lat_start = float(lat_match.group(1))
        lat_end = float(lat_match.group(2))
        lat_step = float(lat_match.group(3))

        lon_start = float(lon_match.group(1))
        lon_end = float(lon_match.group(2))
        lon_step = float(lon_match.group(3))

        nx = int(grid_match.group(1))
        ny = int(grid_match.group(2))

        grid_info = {
            "grid_type": "lat-lon",
            "nx": nx,
            "ny": ny,
            "total_points": nx * ny,
            "lat_start": lat_start,
            "lat_end": lat_end,
            "lat_step": lat_step,
            "lon_start": lon_start,
            "lon_end": lon_end,
            "lon_step": lon_step,
            "lat_range": (lat_start, lat_end),
            "lon_range": (lon_start, lon_end)
        }

    return grid_info

def calculate_conus_coverage(grid_info: Dict) -> Dict:
    """Calculate CONUS coverage percentage and cell counts."""
    if "error" in grid_info or not grid_info:
        return {"error": "Invalid grid info"}

    lat_start = grid_info["lat_start"]
    lat_end = grid_info["lat_end"]
    lat_step = grid_info["lat_step"]
    lon_start = grid_info["lon_start"]
    lon_end = grid_info["lon_end"]
    lon_step = grid_info["lon_step"]

    # Check if grid spans the globe (typical for GFS/GEFS)
    is_global = (lat_start >= 89.9 and lat_end <= -89.9 and
                 lon_start >= 0.0 and lon_end <= 359.9)

    if is_global:
        # Calculate CONUS cell counts for global grids
        # Find latitude indices that cover CONUS
        lat_points_conus = 0
        for lat in [lat_start - i * lat_step for i in range(grid_info["ny"])]:
            if CONUS_LAT_MIN <= lat <= CONUS_LAT_MAX:
                lat_points_conus += 1

        # Find longitude points that cover CONUS
        lon_points_conus = 0
        for lon in [lon_start + i * lon_step for i in range(grid_info["nx"])]:
            # Handle longitude wrapping
            lon_normalized = lon % 360
            if CONUS_LON_MIN <= lon_normalized <= CONUS_LON_MAX:
                lon_points_conus += 1

        conus_cells = lat_points_conus * lon_points_conus
        total_cells = grid_info["total_points"]
        coverage_pct = (conus_cells / total_cells) * 100

        return {
            "is_global": True,
            "conus_cells": conus_cells,
            "total_cells": total_cells,
            "coverage_percentage": round(coverage_pct, 2),
            "lat_points_in_conus": lat_points_conus,
            "lon_points_in_conus": lon_points_conus,
            "conus_covered": True
        }
    else:
        # For regional grids, check if CONUS bounds are within grid
        lat_covers = (lat_start >= CONUS_LAT_MIN and lat_end <= CONUS_LAT_MAX)
        lon_covers = False

        # Check longitude coverage (handle 0-360 vs -180-180)
        if lon_start >= 0:  # 0-360 notation
            lon_covers = (lon_start <= CONUS_LON_MIN and lon_end >= CONUS_LON_MAX)

        return {
            "is_global": False,
            "lat_covers_conus": lat_covers,
            "lon_covers_conus": lon_covers,
            "conus_covered": lat_covers and lon_covers,
            "grid_lat_range": (lat_start, lat_end),
            "grid_lon_range": (lon_start, lon_end)
        }

def verify_file_exists(candidate: Dict) -> bool:
    """Check if candidate file exists and is non-empty."""
    path = Path(candidate["local_path"])
    return path.exists() and path.stat().st_size > 0

def main():
    """Main verification routine."""
    results = {
        "metadata": {
            "verification_date": "2026-07-24",
            "bead": "bf-1evex",
            "conus_bounds": {
                "lat_min": CONUS_LAT_MIN,
                "lat_max": CONUS_LAT_MAX,
                "lon_min": f"{CONUS_LON_MIN}°E (125°W)",
                "lon_max": f"{CONUS_LON_MAX}°E (67°W)"
            },
            "verification_method": "wgrib2 -grid extraction and cell counting"
        },
        "candidates": [],
        "summary": {
            "total_candidates": len(CANDIDATES),
            "verified_conus_coverage": 0,
            "failed_verification": 0,
            "no_conus_coverage": 0
        }
    }

    for candidate in CANDIDATES:
        print(f"Verifying {candidate['candidate_id']}...")

        candidate_result = {
            "candidate_id": candidate["candidate_id"],
            "local_path": candidate["local_path"],
            "verification_status": "pending"
        }

        # Check file exists
        if not verify_file_exists(candidate):
            candidate_result.update({
                "verification_status": "failed",
                "error": "File missing or empty"
            })
            results["summary"]["failed_verification"] += 1
            results["candidates"].append(candidate_result)
            continue

        # Extract grid info
        grid_info = extract_grid_info(candidate["local_path"])

        if "error" in grid_info:
            candidate_result.update({
                "verification_status": "failed",
                "error": grid_info["error"]
            })
            results["summary"]["failed_verification"] += 1
            results["candidates"].append(candidate_result)
            continue

        # Calculate CONUS coverage
        conus_coverage = calculate_conus_coverage(grid_info)

        candidate_result.update({
            "grid_info": grid_info,
            "conus_coverage": conus_coverage,
            "verification_status": "success"
        })

        if conus_coverage.get("conus_covered", False):
            results["summary"]["verified_conus_coverage"] += 1
        else:
            results["summary"]["no_conus_coverage"] += 1

        results["candidates"].append(candidate_result)
        print(f"  ✓ CONUS coverage: {conus_coverage.get('coverage_percentage', 'N/A')}%")

    # Write results
    output_file = Path("/home/coding/gribtract/conus_coverage_verification.json")
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"\nResults written to {output_file}")
    print(f"CONUS coverage verified: {results['summary']['verified_conus_coverage']}/{results['summary']['total_candidates']}")

if __name__ == "__main__":
    main()
