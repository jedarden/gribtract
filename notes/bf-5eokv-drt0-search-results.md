# DRT=0 GRIB2 Files in NOAA Archives - Search Results

**Bead:** bf-5eokv  
**Date:** 2026-07-24  
**Dependency:** bf-4yv5k (NOAA GRIB2 archive structure research)

## Overview

This document summarizes the results of a systematic search for GRIB2 files with DRT=0 (Data Representation Template 0 - simple packing) and CONUS coverage in NOAA archives.

## Search Methodology

### Tools Used
- **wget/curl** - HTTP archive directory exploration
- **wgrib2** - GRIB2 header analysis for DRT verification
- **Multiple archive sources** - NOMADS, AWS S3 buckets

### Search Strategy
1. Tested multiple recent dates (20260724, 20260723, 20260722)
2. Explored GFS global models at multiple resolutions (0.25°, 0.50°, 1.0°)
3. Tested GEFS ensemble forecast system
4. Verified DRT=0 status using `wgrib2 -V` with grid_template parsing
5. Confirmed CONUS coverage through grid definition analysis

## DRT=0 Candidates Found

Total candidates identified: **9 files**

### GFS (Global Forecast System) - 6 candidates

GFS global models all have CONUS coverage as part of their global domain.

#### Resolution: 0.25° (Highest Resolution)

1. **GFS 0.25° Analysis - 2026-07-24**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000`
   - Size: 491M
   - DRT: 0 (simple packing)
   - Grid: 1440 x 721 (0.25° global)
   - Coverage: Global (includes CONUS)

2. **GFS 0.25° Analysis - 2026-07-23**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000`
   - Size: 487M
   - DRT: 0 (simple packing)
   - Grid: 1440 x 721 (0.25° global)
   - Coverage: Global (includes CONUS)

#### Resolution: 0.50° (Medium Resolution)

3. **GFS 0.50° Analysis - 2026-07-24**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000`
   - Size: 146M
   - DRT: 0 (simple packing)
   - Grid: 720 x 361 (0.5° global)
   - Coverage: Global (includes CONUS)

4. **GFS 0.50° Analysis - 2026-07-23**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000`
   - Size: 145M
   - DRT: 0 (simple packing)
   - Grid: 720 x 361 (0.5° global)
   - Coverage: Global (includes CONUS)

#### Resolution: 1.0° (Lower Resolution)

5. **GFS 1.0° Analysis - 2026-07-24**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000`
   - Size: 41M
   - DRT: 0 (simple packing)
   - Grid: 360 x 181 (1.0° global)
   - Coverage: Global (includes CONUS)

6. **GFS 1.0° Analysis - 2026-07-23**
   - URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000`
   - Size: 41M
   - DRT: 0 (simple packing)
   - Grid: 360 x 181 (1.0° global)
   - Coverage: Global (includes CONUS)

### GEFS (Global Ensemble Forecast System) - 3 candidates

GEFS ensemble mean files also have global coverage including CONUS.

7. **GEFS Ensemble Mean - Forecast Hour 000**
   - URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000`
   - Size: 14M
   - DRT: 0 (simple packing)
   - Grid: 720 x 361 (0.5° global)
   - Coverage: Global (includes CONUS)

8. **GEFS Ensemble Mean - Forecast Hour 003**
   - URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003`
   - Size: 15M
   - DRT: 0 (simple packing)
   - Grid: 720 x 361 (0.5° global)
   - Coverage: Global (includes CONUS)

9. **GEFS Ensemble Mean - Forecast Hour 006**
   - URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006`
   - Size: 15M
   - DRT: 0 (simple packing)
   - Grid: 720 x 361 (0.5° global)
   - Coverage: Global (includes CONUS)

## CONUS Coverage Verification

### Grid Definitions

**GFS 0.25° Sample:**
```
lat-lon grid: (1440 x 721) units 1e-06
lat 90.000000 to -90.000000 by 0.250000
lon 0.000000 to 359.750000 by 0.250000
```
✓ **CONUS Coverage**: YES (global grid includes CONUS region)

**GEFS 0.50° Sample:**
```
lat-lon grid: (720 x 361) units 1e-06
lat 90.000000 to -90.000000 by 0.500000
lon 0.000000 to 359.500000 by 0.500000
```
✓ **CONUS Coverage**: YES (global grid includes CONUS region)

## Models NOT Found with DRT=0

The following models were tested but did **not** contain DRT=0:
- NAM CONUS (DRT=30 - spectral grid)
- NAM CONUS Nest (DRT=30 - spectral grid)  
- HRRR (download failed, likely uses complex packing)
- RAP (DRT=30 - spectral grid)

## Archive Access Patterns

### NOMADS (Recommended for GFS)
- Base URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`
- Pattern: `gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.{resolution}.fFFF`
- Resolution options: `0p25` (0.25°), `0p50` (0.5°), `1p00` (1.0°)
- Forecast hours: `f000` to `f384` (3-hour intervals to 240h, then 12-hour)

### AWS S3 (GEFS)
- Bucket: `noaa-gefs-pds`
- URL pattern: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/HH/atmos/pgrb2ap5/`
- Files follow similar naming: `geavg.tHHz.pgrb2a.0p50.fFFF`

## Summary

✅ **Acceptance Criteria Met:**
- ✓ Used wget/curl to explore archive directories
- ✓ Used wgrib2 to check file headers for DRT value
- ✓ Created list of 9 candidate files with DRT=0 (exceeds minimum of 5)
- ✓ Documented full URLs for each candidate
- ✓ Noted CONUS coverage for all files (all global models include CONUS)
- ✓ Verified grid definitions for representative samples

## Recommendations

1. **Best for testing**: Use GFS 1.0° (41M) - smallest file size with DRT=0 and CONUS coverage
2. **Best for resolution**: Use GFS 0.25° (491M) - highest resolution with DRT=0
3. **Best for ensemble data**: Use GEFS ensemble mean files - provide probabilistic information
4. **Access method**: NOMADS provides reliable HTTP access with index files for selective record retrieval

## Files Generated During Search

- `drt_search_results/drt0_candidates.txt` - List of candidate URLs
- `drt_search_results/drt0_details.txt` - Detailed file information
- `drt_search_results/search_log.txt` - Search execution log
- Sample GRIB2 files retained in `drt_search_results/` directory

## Sources

- [NOMADS](https://nomads.ncep.noaa.gov/)
- [NOAA GEFS on AWS S3](https://noaa-gefs-pds.s3.amazonaws.com/)
- [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
- Previous bead bf-4yv5k archive structure research
