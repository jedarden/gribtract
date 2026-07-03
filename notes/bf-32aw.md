# NOAA GRIB2 Source - Complete Provenance Documentation

**Task:** bf-32aw - Document complete NOAA GRIB2 source with provenance

**Completed:** 2026-07-03

---

## Executive Summary

This document provides complete provenance documentation for publicly accessible NOAA GRIB2 files with the following specifications:
- **Grid Definition Template (GDT):** 3.30 (Lambert Conformal Conic projection)
- **Data Representation Template (DRT):** 3 (complex packing with spatial differencing)

These files are suitable for use in gribtract and related GRIB2 processing applications.

---

## Primary Recommendation: NOAA HRRR

### Direct Download URL (Verified Working)

**URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Provenance:**
- **Model Name:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Cycle/Run Date:** June 1, 2024, 12:00 UTC
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 142,393,582 bytes (~135.7 MB)
- **Verification:** HTTP 200 OK as of 2026-07-03

---

## Technical Specifications

### Projection Details (GDT 3.30)

**Grid Definition Template:** 3.30
- **Projection Type:** Lambert Conformal Conic
- **Standard Name:** `lambert`
- **Usage Context:** Standard for regional weather models over mid-latitude domains (CONUS)
- **Resolution:** 3 km horizontal grid spacing

**Reference Documentation:**
- [NCEP GRIB2 Template 3.30](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp3-30.shtml)
- [ECMWF Code Table 3.1](https://codes.ecmwf.int/grib/format/grib2/ctables/3/1/) - Confirms template 30 is "Lambert conformal"

### Compression Details (DRT=3)

**Data Representation Template:** 3
- **Packing Type:** `grid_complex_spatial_differencing`
- **Method:** Complex packing with spatial differencing
- **Purpose:** Improved compression for smooth meteorological fields
- **Advantage:** Better compression than simple packing (DRT=0) or complex packing without differencing (DRT=2)

**Verification Output (via eccodes Python library):**
```
Grid Type: lambert
GDT: 30 (Lambert Conformal Conic)
Packing: grid_complex_spatial_differencing (DRT=3)
```

**Reference Documentation:**
- [NCEP GRIB2 Template 5.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp5-3.shtml)

---

## Archive Access Instructions

### Primary Source: AWS S3 (NOAA Big Data Program)

**Base URLs:**
- HRRR: `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/`
- NAM: `https://noaa-nam-pds.s3.amazonaws.com/`

**Authentication:** None required - public access via HTTPS

**Access Methods:**

#### 1. Direct HTTPS Download (Recommended)
```bash
# HRRR example
curl -O https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2

# NAM example
curl -O https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2
```

#### 2. AWS CLI (Anonymous Access)
```bash
# No AWS account required -- use --no-sign-request
aws s3 ls --no-sign-request s3://noaa-hrrr-bdp-pds/
aws s3 cp --no-sign-request s3://noaa-hrrr-bdp-pds/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2 .
```

#### 3. Python Herbie Package
```python
from herbie import Herbie
H = Herbie('2024-06-01 12:00', model='hrrr', product='wrfsfcf00')
H.download()
```

#### 4. Index Files for Subsetting
Each GRIB2 file has a corresponding `.idx` index file:
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2.idx
```
Use the index to download specific variables without retrieving the entire file.

---

## URL Construction Patterns

### HRRR Pattern
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2
```

**Components:**
- `YYYYMMDD` = Model run date (e.g., 20240601)
- `HH` = Cycle hour 00-23 UTC (e.g., 12 for 12z)
- `FF` = Forecast hour 00-18 (standard), 00-48 (extended)

**Examples:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2
```

### NAM Pattern
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awphysFF.tm00.grib2
```

**Components:**
- `YYYYMMDD` = Model run date
- `HH` = Cycle hour (00, 06, 12, 18)
- `FF` = Forecast hour 00-60
- `awphys` = AWIP physics grid (Grid 218)

**Example:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2
```

---

## Alternative Sources

### 1. NOMADS (NOAA Operational Model Archive and Distribution System)

**Base URL:** `https://nomads.ncep.noaa.gov/`

**Access Methods:**
- HTTP direct download
- GRIB Filter service (recommended for subsets)
- Documentation: [Fast Download Guide](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)

**Note:** Rate limiting implemented April 2021. AWS S3 recommended for bulk downloads.

### 2. University of Utah HRRR Archive

**API Endpoint:** `https://api.mesowest.utah.edu/archive/HRRR/oper/sfc/[YYYYMMDD]/hrrr.t[HH]z.wrfsfcf[FF].grib2`

**Requirements:** Registration required at http://hrrr.chpc.utah.edu/

**Documentation:** [HRRR Archive FAQ](https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html)

### 3. NCEI (National Centers for Environmental Information)

**Base URL:** `https://www.ncei.noaa.gov/access`

**Access Methods:**
- Web search interface
- REST API
- OPeNDAP/THREDDS

**Coverage:** Long-term historical data

---

## Additional Verified Files

### HRRR Alternative Files

| Date | Cycle | Forecast Hour | URL |
|------|-------|---------------|-----|
| 2024-01-01 | 00z | F00 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240101/conus/hrrr.t00z.wrfsfcf00.grib2` |
| 2025-01-15 | 00z | F00 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2` |
| 2025-01-15 | 06z | F03 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2` |

### NAM Alternative File

| Model | Date | Cycle | Forecast Hour | URL |
|-------|------|-------|---------------|-----|
| NAM | 2025-01-15 | 00z | F00 | `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2` |

---

## File Size Comparison

| Model | Product | Size Range | Use Case |
|-------|---------|-----------|----------|
| HRRR | wrfsfc | 124-143 MB | Full 2D surface fields, comprehensive testing |
| NAM | awphys | ~50 MB | Faster downloads, sufficient for GDT/DRT verification |

---

## Tools and Libraries

### Command Line
- **wgrib2:** NOAA CPC utility for GRIB2 manipulation
  - https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/

### Python
- **eccodes:** ECMWF GRIB2 decoder (for verification)
  ```python
  import eccodes
  with open('file.grib2', 'rb') as f:
      msg_id = eccodes.codes_grib_new_from_file(f)
      print(f"Grid Type: {eccodes.codes_get(msg_id, 'gridType')}")
      print(f"GDT: {eccodes.codes_get(msg_id, 'gridDefinitionTemplateNumber')}")
      print(f"Packing: {eccodes.codes_get(msg_id, 'packingType')}")
  ```

- **cfgrib:** GRIB2 reader (requires ECCODES)
- **Herbie:** Smart GRIB2 downloader with multi-source fallback
  - https://herbie.readthedocs.io/

### R
- **rNOMADS:** Functions like `GribGrab()` and `ArchiveGribGrab()`

---

## Source Citations

1. **NOAA HRRR on AWS Open Data Registry**
   https://registry.opendata.aws/noaa-hrrr-pds/

2. **NCEP HRRR Products**
   https://www.nco.ncep.noaa.gov/pmb/products/hrrr/

3. **NCEP GRIB2 Template 3.30 Documentation**
   https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp3-30.shtml

4. **NCEP GRIB2 Template 5.3 Documentation**
   https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp5-3.shtml

5. **NOMADS Fast Download Guide**
   https://nomads.ncep.noaa.gov/info.php?page=fastdownload

6. **AWS Blog: NOAA Big Data Project**
   https://aws.amazon.com/blogs/aws/announcing-the-noaa-big-data-project/

7. **Herbie Documentation**
   https://herbie.readthedocs.io/

8. **University of Utah HRRR Archive FAQ**
   https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html

9. **NCEI Data Access Portal**
   https://www.ncei.noaa.gov/access

10. **ECMWF GRIB2 Code Table 3.1**
    https://codes.ecmwf.int/grib/format/grib2/ctables/3/1/

---

## Acceptance Criteria Checklist

| Criterion | Status | Details |
|-----------|--------|---------|
| Working direct download URL to real GRIB2 file | ✅ Complete | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2` (HTTP 200 OK) |
| File verified to contain GDT 3.30 + DRT=3 | ✅ Complete | Verified via eccodes Python library - Grid: lambert, GDT: 30, Packing: grid_complex_spatial_differencing |
| Model name, cycle/run date, forecast hour documented | ✅ Complete | HRRR, 2024-06-01, 12z, F00 |
| Projection details documented | ✅ Complete | GDT 3.30 (Lambert Conformal Conic), 3 km resolution |
| Compression details documented | ✅ Complete | DRT=3 (complex packing with spatial differencing) |
| Archive access instructions | ✅ Complete | HTTPS, AWS CLI, Herbie, index file subsetting |
| Source citations | ✅ Complete | 10 primary sources with links |
| Document saved in project research notes | ✅ Complete | `/home/coding/gribtract/notes/bf-32aw.md` |

---

## Quick Reference Summary

**Recommended URL for Testing:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Key Specifications:**
- GDT: 3.30 (Lambert Conformal Conic)
- DRT: 3 (complex packing with spatial differencing)
- Model: NOAA HRRR
- Date: 2024-06-01 12z
- Forecast: F00
- Size: ~136 MB

**Download Command:**
```bash
curl -O https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

---

*Document completed for bead bf-32aw on 2026-07-03*
*Research synthesizes findings from beads bf-37kc, bf-13e3, bf-2r5b, and bf-1fe5*
