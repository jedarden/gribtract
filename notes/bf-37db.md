# bf-37db: Lambert-conformal DRT=3 Candidate Files

## Task Summary
Identified specific GRIB2 files from NOAA archives that use Lambert-conformal projection (GDT 3.30) with complex packing (DRT=3).

## Candidate Files Found

### 1. HRRR Surface Fields - Recent Analysis (2026)

**Source URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260702/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Model Information:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** July 2, 2026
- **Cycle:** 12z (12:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 140,656,288 bytes (~134.2 MB)

**Characteristics:**
- Grid Definition Template (GDT): 3.30 (Lambert Conformal Conic)
- Data Representation Template (DRT): 3 (complex packing with spatial differencing)
- Resolution: 3km CONUS domain
- Public access via HTTPS (no authentication required)

---

### 2. HRRR Surface Fields - 2024 Analysis

**Source URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Model Information:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** June 1, 2024
- **Cycle:** 12z (12:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 142,393,582 bytes (~135.7 MB)

**Characteristics:**
- Grid Definition Template (GDT): 3.30 (Lambert Conformal Conic)
- Data Representation Template (DRT): 3 (complex packing with spatial differencing)
- Resolution: 3km CONUS domain
- Public access via HTTPS (no authentication required)

---

### 3. NAM AWIP Physics Grid - Recent Analysis (2026)

**Source URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20260702/nam.t12z.awphys00.tm00.grib2
```

**Model Information:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Date:** July 2, 2026
- **Cycle:** 12z (12:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** awphys (AWIP physics grid, Grid 218)
- **File Size:** 56,822,223 bytes (~54.2 MB)

**Characteristics:**
- Grid Definition Template (GDT): 3.30 (Lambert Conformal Conic)
- Data Representation Template (DRT): 3 (complex packing with spatial differencing)
- Resolution: 12km CONUS domain
- Smaller file size suitable for faster testing
- Public access via HTTPS (no authentication required)

---

### 4. HRRR Surface Fields - January 2025

**Source URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2
```

**Model Information:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** January 15, 2025
- **Cycle:** 00z (00:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 130,489,114 bytes (~124.4 MB)

**Characteristics:**
- Grid Definition Template (GDT): 3.30 (Lambert Conformal Conic)
- Data Representation Template (DRT): 3 (complex packing with spatial differencing)
- Resolution: 3km CONUS domain
- Public access via HTTPS (no authentication required)

---

### 5. HRRR Surface Fields - 3-Hour Forecast (January 2025)

**Source URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2
```

**Model Information:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** January 15, 2025
- **Cycle:** 06z (06:00 UTC model run)
- **Forecast Hour:** F03 (3-hour forecast)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 143,390,041 bytes (~136.7 MB)

**Characteristics:**
- Grid Definition Template (GDT): 3.30 (Lambert Conformal Conic)
- Data Representation Template (DRT): 3 (complex packing with spatial differencing)
- Resolution: 3km CONUS domain
- Demonstrates forecast hour (not analysis) encoding
- Public access via HTTPS (no authentication required)

---

## URL Construction Patterns

### HRRR Pattern
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2
```
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00-23)
- `FF` = forecast hour (00-18 for standard cycles, 00-48 for extended cycles)

### NAM Pattern
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awphysFF.tm00.grib2
```
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00, 06, 12, 18)
- `FF` = forecast hour (00-60)

## Acceptance Criteria Verification

- ✅ **Identified at least 2 candidate file URLs:** Found 5 candidate files
- ✅ **Documented source URL, model/run info, and file size for each:** All documented above
- ✅ **Files are from public NOAA archives:** All URLs are publicly accessible via HTTPS
- ✅ **Files are reasonably sized (< 500MB):** All files range from 54MB to 143MB

## Notes

- All files use GDT 3.30 (Lambert Conformal Conic projection)
- All files use DRT 3 (complex packing with spatial differencing)
- Files were verified accessible on 2026-07-03
- Index files (.idx) are available for subsetting individual variables without downloading the entire file

## References

- Previous research: docs/research/bf-13e3-noaa-gdt330-drt3-urls.md
- Previous research: docs/research/bf-2r5b-gdt330-drt3-candidate-files.md
- docs/research/noaa-regional-model-grib2-archives.md
