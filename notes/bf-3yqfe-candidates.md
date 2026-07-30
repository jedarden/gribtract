# Candidate GRIB2 Files for DRT Checking
# Generated: 2026-07-24
# Task: bf-3yqfe
# Source: Inventory from bf-3qsg9

## File Selection Strategy
Files selected from the GFS (Global Forecast System) archive to test:
- Different resolutions (0p25, 0p50, 1p00)
- Different dates (2026-07-21, 2026-07-22, 2026-07-23, 2026-07-24)
- Different forecast hours (f000, f003, f006, f012)
- Both analysis (f000) and forecast files

## Candidate File List

### High Resolution (0.25° - 0p25)

1. **2026-07-24 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000
   Size: ~490 MB
   Category: Current day, highest resolution, analysis

2. **2026-07-24 00Z - 12-hour forecast**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f012
   Size: ~521 MB
   Category: Current day, highest resolution, forecast

3. **2026-07-23 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000
   Size: ~487 MB
   Category: Previous day, highest resolution, analysis

4. **2026-07-23 00Z - 6-hour forecast**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f006
   Size: ~521 MB
   Category: Previous day, highest resolution, forecast

5. **2026-07-22 00Z - 3-hour forecast**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260722/00/atmos/gfs.t00z.pgrb2.0p25.f003
   Size: ~519 MB
   Category: 2 days ago, highest resolution, forecast

### Medium Resolution (0.50° - 0p50)

6. **2026-07-24 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000
   Size: ~145 MB
   Category: Current day, medium resolution, analysis

7. **2026-07-23 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000
   Size: ~145 MB
   Category: Previous day, medium resolution, analysis

8. **2026-07-21 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260721/00/atmos/gfs.t00z.pgrb2.0p50.f000
   Size: ~145 MB
   Category: 3 days ago, medium resolution, analysis

### Low Resolution (1.00° - 1p00)

9. **2026-07-24 00Z - Analysis file**
   URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000
   Size: ~40.8 MB
   Category: Current day, lowest resolution, analysis

10. **2026-07-23 00Z - Analysis file**
    URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000
    Size: ~40.8 MB
    Category: Previous day, lowest resolution, analysis

## Summary

**Total candidates**: 10 files

**By resolution**:
- High resolution (0p25): 5 files
- Medium resolution (0p50): 3 files  
- Low resolution (1p00): 2 files

**By date**:
- 2026-07-24 (current): 4 files
- 2026-07-23 (yesterday): 4 files
- 2026-07-22 (2 days ago): 1 file
- 2026-07-21 (3 days ago): 1 file

**By forecast hour**:
- f000 (analysis): 7 files
- f003 (3-hour forecast): 1 file
- f006 (6-hour forecast): 1 file
- f012 (12-hour forecast): 1 file

## Download Command Template

For manual download testing:
```bash
# Example download
wget https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000
```

## Next Steps

1. Download these candidate files to a working directory
2. Use wgrib2 or similar tools to inspect DRT values
3. Record DRT (Decimal Representation Table) values for each file
4. Verify that files use DRT=0 (simple packing) as expected from NOAA archives

---
Task Reference: bf-3yqfe
Source Inventory: bf-3qsg9
Generated: 2026-07-24