# GDT 3.30 + DRT=3 Candidate GRIB2 Files

## Overview
This document documents specific, publicly accessible GRIB2 files from NOAA archives that use Grid Definition Template 3.30 (Lambert Conformal Conic) and Data Representation Template 3 (complex packing with spatial differencing).

All URLs are verified accessible via HTTPS as of 2026-07-03.

---

## HRRR (High-Resolution Rapid Refresh) Files

### Candidate 1: HRRR Surface Fields - Recent Analysis

**File:** `hrrr.t12z.wrfsfcf00.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Provenance:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** June 1, 2024
- **Cycle:** 12z (12:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 142,393,582 bytes (~135.7 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Contains surface meteorological fields (temperature, wind, precipitation, etc.)

---

### Candidate 2: HRRR Surface Fields - Very Recent Analysis

**File:** `hrrr.t00z.wrfsfcf00.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2
```

**Provenance:**
- **Model:** NOAA HRRR
- **Date:** January 15, 2025
- **Cycle:** 00z (00:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 130,489,114 bytes (~124.4 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Most recent file in this candidate set

---

### Candidate 3: HRRR Surface Fields - 3-Hour Forecast

**File:** `hrrr.t06z.wrfsfcf03.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2
```

**Provenance:**
- **Model:** NOAA HRRR
- **Date:** January 15, 2025
- **Cycle:** 06z (06:00 UTC model run)
- **Forecast Hour:** F03 (3-hour forecast)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 143,390,041 bytes (~136.7 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Demonstrates forecast hour (not analysis) encoding

---

## NAM (North American Mesoscale) File

### Candidate 4: NAM AWIP Physics Grid

**File:** `nam.t00z.awphys00.tm00.grib2`

**Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2
```

**Provenance:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Date:** January 15, 2025
- **Cycle:** 00z (00:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** awphys (AWIP physics grid, Grid 218)
- **File Size:** 52,938,055 bytes (~50.5 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- 12km resolution CONUS domain
- Smaller file size than HRRR, good for faster testing

---

## URL Construction Patterns

### HRRR Pattern
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2
```

Where:
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00-23)
- `FF` = forecast hour (00-18 for standard cycles, 00-48 for extended cycles)

### NAM Pattern
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awphysFF.tm00.grib2
```

Where:
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00, 06, 12, 18)
- `FF` = forecast hour (00-60)

---

## File Size Considerations

All candidate files are practical for download and testing:

| Model | Size Range | Typical Use Case |
|-------|-----------|------------------|
| HRRR wrfsfc | 124-143 MB | Full 2D surface fields, comprehensive testing |
| NAM awphys | ~50 MB | Faster downloads, sufficient for GDT/DRT verification |

---

## Verification

All URLs were tested on 2026-07-03 using:
```bash
curl -sI '<url>' | grep -E '(HTTP|Content-Length)'
```

All returned HTTP 200 OK, confirming public accessibility.

---

## Index Files

Each GRIB2 file has a corresponding `.idx` index file that allows subsetting individual variables without downloading the entire file:

**HRRR Example:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2.idx
```

**NAM Example:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2.idx
```

---

## References

- [NOAA HRRR on AWS Open Data Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NCEP HRRR Products](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [GRIB2 Template 3.30 - Lambert Conformal](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp3-30.shtml)
- [GRIB2 Template 5.3 - Complex Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp5-3.shtml)

---

*Document created for bead bf-2r5b on 2026-07-03*
