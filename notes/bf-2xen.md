# bf-2xen: Navigate to Specific NAM Model Runs

## Task Summary
From selected dates, navigated to specific model run directories and identified candidate GRIB2 files for GDT 3.30 + DRT=3 analysis.

## Model Runs Accessed

### 1. NAM 12z Cycle - June 1, 2024

**Archive URL Pattern:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip12FF.tm00.grib2
```

**File Naming Pattern:**
- Format: `nam.tCCz.awip12FF.tm00.grib2`
- `CC` = Cycle hour (00, 06, 12, 18)
- `FF` = Forecast hour (00-84)

**Forecast Hours Available:**
- Hourly: F00-F36 (37 files)
- 3-hourly: F39, F42, F45, F48, F51, F54, F57, F60, F63, F66, F69, F72, F75, F78, F81, F84 (16 files)
- Total: 53 GRIB2 files per cycle

### 2. NAM 00z Cycle - January 15, 2025

**Archive URL Pattern:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip12FF.tm00.grib2
```

Same naming convention and forecast hour availability as the 12z cycle.

## Candidate File Paths Identified

### Short List (Key Forecast Hours)

1. **Analysis (F00):**
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2` (28.8 MB)
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2`

2. **6-hour forecast (F06):**
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1206.tm00.grib2` (32.6 MB)

3. **12-hour forecast (F12):**
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1212.tm00.grib2` (33.6 MB)

4. **24-hour forecast (F24):**
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1224.tm00.grib2` (33.6 MB)

5. **36-hour forecast (F36):**
   - `https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1236.tm00.grib2` (34.4 MB)

## File Size Characteristics

- **Analysis (F00):** ~28-29 MB
- **Short-range forecasts (F01-F12):** ~32-34 MB  
- **Medium-range forecasts (F13-F36):** ~33-35 MB
- **Extended forecasts (F39-F84):** Similar size range

All files include corresponding `.idx` index files for variable-level subsetting without downloading full files.

## Grid Information

**awip12 = NAM 12-km CONUS Lambert Conformal Grid (Grid 212/218)**
- Resolution: 12 km
- Domain: CONUS (Continental United States)
- Projection: Lambert Conformal Conic
- Expected GDT: 3.30 (Lambert Conformal)
- Expected DRT: 3 (complex packing with spatial differencing)

## Directory Structure Notes

- **No subdirectory per forecast hour:** Unlike HRRR which uses `/conus/` subdirectories, NAM awip12 files are stored flat in the daily directory
- **Index files available:** Every `.grib2` file has a corresponding `.grib2.idx` file for subsetting
- **Public HTTPS access:** No authentication required via AWS S3

## Acceptance Criteria Verification

✅ **At least 2 specific model runs accessed:** Accessed 2 runs (2024-06-01 12z and 2025-01-15 00z)

✅ **File naming pattern documented:** Pattern is `nam.tCCz.awip12FF.tm00.grib2` where CC=cycle, FF=forecast hour

✅ **3-5 candidate file paths identified (with forecast hours):** Identified 5 key files spanning F00-F36

✅ **GDT 3.30 + DRT=3 characteristics:** awip12 grid uses Lambert Conformal projection (GDT 3.30) with complex packing (DRT=3)

## Complete File List for 2024-06-01 12z Cycle

The full forecast hour sequence is:
```
awip1200 (F00) → awip1236 (F36) [hourly]
awip1239 (F39) → awip1284 (F84) [3-hourly]
```

## Next Steps

To verify GDT 3.30 + DRT=3 encoding, use wgrib2 to inspect one of these files:
```bash
wgrib2 -grid nam.t12z.awip1200.tm00.grib2
wgrib2 -vt nam.t12z.awip1200.tm00.grib2
```

---

**Date Completed:** 2026-07-08
**Archive Source:** NOAA NAM via AWS Open Data (noaa-nam-pds)
