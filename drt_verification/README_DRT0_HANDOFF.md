# DRT=0 Candidate List for CONUS Verification

**Generated:** 2026-07-24  
**Bead:** bf-64bv0  
**Purpose:** CONUS coverage verification hand-off

## Summary

This directory contains the **final filtered list of DRT=0 candidates** ready for CONUS coverage verification in the next bead.

### Candidate Count

**Total DRT=0 candidates: 7**

All candidates have been verified to use **Simple Packing (DRT=0)** and are suitable for CONUS coverage analysis.

### Distribution by Model

- **GFS:** 4 candidates
- **GEFS:** 3 candidates (ensemble mean)

### Distribution by Resolution

- **0.25°:** 1 candidate (highest resolution)
- **0.50°:** 4 candidates (middle resolution)
- **1.00°:** 2 candidates (standard resolution)

### Dates Covered

- 2026-07-23
- 2026-07-24

## Files

### Primary Hand-off File

- **`drt0_candidates.json`** - Main candidate list with full metadata
  - Contains all 7 verified DRT=0 candidates
  - Includes filename, source URL, local path, DRT confirmation, packing specs
  - Ready for programmatic consumption by next bead

### Supporting Files (Reference)

- **`drt0_filtered_list.json`** - Previous analysis output (bf-4wg4g)
- **`drt_verification_results.json`** - Raw DRT analysis showing non-DRT=0 files
- **`drt0_candidates.txt`** - Simple text listing
- **`drt0_filtered_list.txt`** - Detailed text listing

### Excluded Candidates

Two candidates were excluded due to failed downloads (0-byte files):
- `gfs_0p25_20260724_f000.grib2`
- `gfs_0p50_20260723_f000.grib2`

These require re-download before inclusion in CONUS verification.

## Verification Status

✓ **All DRT=0 confirmed** - True  
✓ **All Simple Packing** - True  
✓ **JSON format parseable** - Validated  
✓ **Local files available** - All 7 candidates present at local_path  
✓ **Source URLs documented** - Ready for re-download if needed  

## Next Steps (CONUS Verification Bead)

The next bead should:

1. Load `drt_verification/drt0_candidates.json`
2. For each candidate, extract grid cells using wgrib2
3. Apply CONUS bounding box filter:
   - Latitude: ~24°N to ~50°N
   - Longitude: ~125°W to ~66°W (-125° to -66°)
4. Calculate:
   - Total CONUS grid cells
   - Percentage of global grid covering CONUS
   - Per-candidate CONUS coverage statistics
5. Generate CONUS coverage report

## Data Structure Reference

Each candidate in `drt0_candidates.json` contains:

```json
{
  "candidate_id": "unique_identifier",
  "filename": "local_filename.grib2",
  "source_url": "https://...",
  "local_path": "/absolute/path/to/file.grib2",
  "model": "GFS|GEFS",
  "resolution": "0.25°|0.50°|1.00°",
  "date": "YYYY-MM-DD",
  "forecast_hour": "f000|f003|f006",
  "size_bytes": integer,
  "drt_confirmation": {
    "value": 0,
    "packing_type": "Simple Packing",
    "method": "wgrib2 -grid analysis",
    "verified_by": "bf-4wg4g"
  },
  "coverage": "Global (includes CONUS)"
}
```

## Dependency Chain

This hand-off builds on:
- **bf-5eokv:** Initial DRT=0 candidate search
- **bf-3ugst:** GRIB2 header download
- **bf-1fmeu:** DRT analysis execution
- **bf-4wg4g:** DRT=0 filtering and parsing

---

**Ready for CONUS verification bead.** All DRT=0 candidates validated and documented.
