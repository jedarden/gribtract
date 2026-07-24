# DRT Analysis - Selected GRIB2 Candidate Files
# Task: bf-6bcol
# Generated: 2026-07-24

## Selection Summary

Based on the inventory from bf-3qsg9, I have selected 15 candidate GRIB2 files from the GFS archive for comprehensive DRT analysis. These files were chosen to provide diversity across:
- Multiple resolutions (0.25°, 0.50°, 1.00°)
- Different forecast hours (analysis, short-term, medium-term)
- Multiple model run dates
- Different data characteristics and file sizes

## Selected Candidate Files

### High-Resolution Files (0.25°) - 5 candidates

1. **gfs.t00z.pgrb2.0p25.f000** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000
   - Size: 514,251,059 bytes (~490 MB)
   - Type: Analysis (current time)
   - Rationale: Baseline high-resolution analysis file

2. **gfs.t00z.pgrb2.0p25.f003** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f003
   - Size: 544,007,059 bytes (~519 MB)
   - Type: 3-hour forecast
   - Rationale: Short-term forecast, +5.8% size increase from analysis

3. **gfs.t00z.pgrb2.0p25.f006** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f006
   - Size: 546,147,177 bytes (~521 MB)
   - Type: 6-hour forecast
   - Rationale: Medium-term forecast, +6.2% size increase from analysis

4. **gfs.t00z.pgrb2.0p25.f012** (2026-07-23)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f012
   - Size: ~521 MB (estimated based on inventory)
   - Type: 12-hour forecast
   - Rationale: Longer-term forecast from previous day

5. **gfs.t00z.pgrb2.0p25.f000** (2026-07-22)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.0p25.f000
   - Size: 512,102,383 bytes (~488 MB)
   - Type: Historical analysis
   - Rationale: Historical baseline for comparison

### Medium-Resolution Files (0.50°) - 5 candidates

6. **gfs.t00z.pgrb2.0p50.f000** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000
   - Size: 152,106,356 bytes (~145 MB)
   - Type: Analysis (current time)
   - Rationale: Medium-resolution analysis, 3.4x smaller than 0.25°

7. **gfs.t00z.pgrb2.0p50.f003** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f003
   - Size: ~153 MB (estimated)
   - Type: 3-hour forecast
   - Rationale: Medium-resolution short-term forecast

8. **gfs.t00z.pgrb2.0p50.f006** (2026-07-24)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f006
   - Size: ~154 MB (estimated)
   - Type: 6-hour forecast
   - Rationale: Medium-resolution medium-term forecast

9. **gfs.t00z.pgrb2.0p50.f000** (2026-07-23)
   - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000
   - Size: ~151 MB (estimated)
   - Type: Historical analysis
   - Rationale: Historical medium-resolution baseline

10. **gfs.t00z.pgrb2.0p50.f012** (2026-07-21)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260721/00/atmos/gfs.t00z.pgrb2.0p50.f012
    - Size: ~154 MB (estimated)
    - Type: 12-hour forecast (older date)
    - Rationale: Long-term forecast from older model run

### Low-Resolution Files (1.00°) - 5 candidates

11. **gfs.t00z.pgrb2.1p00.f000** (2026-07-24)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000
    - Size: 42,755,881 bytes (~40.8 MB)
    - Type: Analysis (current time)
    - Rationale: Low-resolution analysis, 12x smaller than 0.25°

12. **gfs.t00z.pgrb2.1p00.f003** (2026-07-24)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f003
    - Size: ~43 MB (estimated)
    - Type: 3-hour forecast
    - Rationale: Low-resolution short-term forecast

13. **gfs.t00z.pgrb2.1p00.f006** (2026-07-24)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f006
    - Size: ~43 MB (estimated)
    - Type: 6-hour forecast
    - Rationale: Low-resolution medium-term forecast

14. **gfs.t00z.pgrb2.1p00.f000** (2026-07-23)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000
    - Size: ~42 MB (estimated)
    - Type: Historical analysis
    - Rationale: Historical low-resolution baseline

15. **gfs.t00z.pgrb2.1p00.f024** (2026-07-22)
    - URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.1p00.f024
    - Size: ~44 MB (estimated)
    - Type: 24-hour forecast
    - Rationale: Long-term forecast (24 hours) from older run

## File Size Distribution Summary

- **Total storage required**: ~3.2 GB for all 15 files
- **Largest file**: gfs.t00z.pgrb2.0p25.f003 (519 MB)
- **Smallest file**: gfs.t00z.pgrb2.1p00.f000 (40.8 MB)
- **Average file size**: ~220 MB per file

## Access Verification Strategy

All selected files follow the confirmed accessible pattern:
- Base URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
- Pattern: gfs.YYYYMMDD/CC/atmos/gfs.tCCz.pgrb2.RRRR.fFFF
- All use 00Z cycle (verified accessible)
- All dates within verified accessible range (2026-07-21 to 2026-07-24)

## DRT Analysis Coverage

This selection provides comprehensive coverage for DRT analysis:

1. **Resolution diversity**: Test DRT behavior across different grid resolutions
2. **Forecast hour progression**: Analyze how DRT values change across forecast timelines
3. **Historical comparison**: Enable comparison across different model runs
4. **Size variability**: Test DRT handling with files ranging from 40MB to 520MB
5. **Parameter diversity**: Different forecast hours include different meteorological parameters

## Next Steps

1. Verify download accessibility for each candidate
2. Download files for local DRT analysis
3. Run comprehensive DRT checks using wgrib2 and custom tools
4. Document DRT patterns and findings
5. Cross-reference with existing DRT research from other beads

## Inventory Source

All selections based on comprehensive inventory from:
- Task: bf-3qsg9
- Inventory file: notes/noaa-archive-inventory.txt
- Verification date: 2026-07-24
---
Selection completed: 2026-07-24
Total candidates selected: 15 (exceeds minimum requirement of 10)
Source: GFS archive inventory (bf-3qsg9)
