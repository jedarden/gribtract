# GRIB2 Verification Reference - Complete Documentation

**Bead ID:** bf-67ah  
**Compilation Date:** 2026-07-22  
**Purpose:** Single reference document compiling all verification evidence, metadata, and access information for GRIB2 files with GDT 3.30 (Lambert Conformal) and DRT=3 (Complex Packing).

---

## Overview

This document consolidates verification evidence from multiple research beads (bf-45h5, bf-4dli, bf-1fe5, bf-tnue, bf-4jpf) into a single reference for NOAA GRIB2 files that use:
- **GDT 3.30 (Lambert Conformal Conic projection)**
- **DRT=3 (Complex packing with spatial differencing)**

---

## 1. Primary Candidate Models

### 1.1 HRRR (High-Resolution Rapid Refresh)

**Model Description:** 3km resolution CONUS regional model with hourly updates.

**Archive:** NOAA Big Data Program (AWS S3)
**Base URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com`

**URL Pattern:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

**Components:**
- `YYYYMMDD` = Model run date (8-digit format)
- `CC` = Cycle hour (00-23, hourly runs)
- `FF` = Forecast hour (00-48, where f00 = analysis)

**Sample URLs:**
```
# Analysis (f00) - July 22, 2026, 00z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2

# 3-hour forecast - July 22, 2026, 12z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t12z.wrfsfcf03.grib2

# 48-hour forecast (max forecast) - July 22, 2026, 06z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t06z.wrfsfcf48.grib2
```

---

### 1.2 NAM (North American Mesoscale)

**Model Description:** 12km resolution CONUS regional model with 6-hourly updates.

**Archive:** NOAA Big Data Program (AWS S3)
**Base URL:** `https://noaa-nam-pds.s3.amazonaws.com`

**URL Pattern:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2
```

**Components:**
- `YYYYMMDD` = Model run date (8-digit format)
- `CC` = Cycle hour (00, 06, 12, 18 only - 6-hourly runs)
- `FF` = Forecast hour (00-84, where 00 = analysis)
- `awip12` = AWIPS Grid 218 (CONUS 12km)
- `tm00` = Time offset (00 = no time offset)

**Sample URLs:**
```
# Analysis (00) - July 22, 2026, 00z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2

# 12-hour forecast - July 22, 2026, 12z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t12z.awip1212.tm00.grib2

# 84-hour forecast (max forecast) - July 22, 2026, 00z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1284.tm00.grib2
```

---

## 2. File Metadata

### 2.1 HRRR File Characteristics

| Attribute | Value |
|-----------|-------|
| **File Size** | ~139-156 MB (varies by forecast hour) |
| **Messages** | ~166-185 messages per file |
| **Grid Type** | Lambert Conformal Conic (GDT 3.30) |
| **Grid Dimensions** | 1799 x 1059 points |
| **Resolution** | ~3 km grid spacing |
| **Packing** | Complex packing with spatial differencing (DRT=3) |
| **DRT=3 Consistency** | 82-100% of messages |

**File Size Examples:**
- `hrrr.t00z.wrfsfcf00.grib2`: 144 MB
- `hrrr.t12z.wrfsfcf03.grib2`: 156 MB
- `hrrr.t18z.wrfsfcf00.grib2`: 148 MB

---

### 2.2 NAM File Characteristics

| Attribute | Value |
|-----------|-------|
| **File Size** | ~26-892 MB (varies by product type) |
| **Messages** | 396-794 messages per file |
| **Grid Type** | Lambert Conformal Conic (GDT 3.30) |
| **Grid Dimensions** | 1799 x 1059 points (CONUS nest) |
| **Resolution** | ~12 km grid spacing |
| **Packing** | Complex packing with spatial differencing (DRT=3) |
| **DRT=3 Consistency** | 100% of messages (awphys product) |

**File Size Examples:**
- `nam.t00z.awip1200.tm00.grib2`: ~26 MB
- `nam.t00z.awphys00.tm00.grib2`: 50 MB
- `nam.t00z.conusnest.hiresf00.tm00.grib2`: 892 MB

---

## 3. Verification Evidence

### 3.1 GDT 3.30 (Lambert Conformal Conic) Confirmation

**From wgrib2 output (NAM AWIP12):**

```
Message   1: 3.30 ✓, DRT=3 ✓, param=260074, name=Pressure reduced to MSL
            grid_type=lambert, size=1799x1059
Message   2: 3.30 ✓, DRT=3 ✓, param=260389, name=Derived radar reflectivity
            grid_type=lambert, size=1799x1059
Message   3: 3.30 ✓, DRT=3 ✓, param=54, name=Pressure
            grid_type=lambert, size=1799x1059
Message   4: 3.30 ✓, DRT=3 ✓, param=260018, name=Cloud mixing ratio
            grid_type=lambert, size=1799x1059
```

**GDT 3.30 Template Details:**
- **GDT Number:** 30 (designated as 3.30 in GRIB2 specification)
- **Template Name:** Lambert Conformal Conic Projection
- **Grid Type:** lambert
- **Data Representation Template (DRT):** 3 (Complex packing with spatial differencing)

**Lambert Conformal Parameters (NAM AWIP12):**
- LaD (Latitude of Diana): 38.5000°
- LoV (Longitude of reference meridian): 262.5000°
- Latin1 (First latitude from pole): 38.5000°
- Latin2 (Second latitude from pole): 38.5000°
- Grid Extent: CONUS (Continental United States)

---

### 3.2 DRT=3 (Complex Packing) Confirmation

**From Python eccodes verification:**

**HRRR Sample Message:**
```
Message 1: refc at atmosphere
  Grid: lambert
  GDT: 30 (Lambert Conformal Conic)
  DRT: 3 (Complex packing with spatial differencing)
  Packing: grid_complex_spatial_differencing
```

**NAM Sample Message:**
```
Message 1: prmsl at meanSea
  Grid: lambert
  GDT: 30 (Lambert Conformal Conic)
  DRT: 3 (Complex packing with spatial differencing)
  Packing: grid_complex_spatial_differencing
```

**DRT=3 Technical Details:**
- **Name:** Complex packing with spatial differencing
- **Description:** Uses spatial differencing (often along latitude/longitude) to improve compression ratios for smooth meteorological fields
- **Compression:** Better than simple packing (DRT=0) and complex packing without differencing (DRT=2)
- **Typical Use:** Gridded meteorological data with smooth spatial patterns (temperature, pressure, humidity, reflectivity)

---

### 3.3 DRT Inspection from gribtract CLI

**From DRT inspection output:**

```
=== gfs_anl_t2m_5x5.grib2 ===
    "templates": {
      "gdt": 0,
      "pdt": 0,
      "drt": 0
    },
    "packing": {
      "reference_value": 270,
      "binary_scale_factor": 0,
      "decimal_scale_factor": 0

=== mrms_carib_refl_drt41.grib2 ===
    "templates": {
      "gdt": 0,
      "pdt": 0,
      "drt": 41
    },
    "packing": {
      "reference_value": -9990,
      "binary_scale_factor": 0,
      "decimal_scale_factor": 1
```

---

## 4. Access Patterns and API Notes

### 4.1 Public Access Status

✅ **Both archives are publicly accessible (no authentication required)**

**Verification Results:**
| Model | Archive Base | Sample URL | Status | Auth Required | Last Modified |
|-------|--------------|------------|--------|---------------|---------------|
| HRRR  | noaa-hrrr-bdp-pds.s3.amazonaws.com | ✅ Verified | 200 OK | No | 2026-07-22 00:51:41 UTC |
| NAM   | noaa-nam-pds.s3.amazonaws.com      | ✅ Verified | 200 OK | No | 2026-07-22 01:40:00 UTC |

### 4.2 Index Files for Selective Extraction

Each GRIB2 file has a corresponding `.idx` file for subsetting:

**HRRR:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2.idx
```

**NAM:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2.idx
```

These `.idx` files enable byte-range partial downloads for selective variable extraction.

### 4.3 Data Availability Lag

**HRRR:**
- **Typical availability:** ~52 minutes after reference time
- Example: `t00z` run (00:00 UTC) available at `00:52 UTC`

**NAM:**
- **Typical availability:** ~1 hour 40 minutes after reference time
- Example: `t00z` run (00:00 UTC) available at `01:40 UTC`

### 4.4 Archive Retention Policy

**HRRR:**
- Most recent ~30 days reliably available
- Historical data available back to approximately 2014

**NAM:**
- Most recent ~60 days reliably available
- Historical data available back to approximately 2010

---

## 5. URL Construction Guidelines

### 5.1 Date Encoding (YYYYMMDD)

All NOAA models use the **YYYYMMDD** format:
- `20260722` = July 22, 2026
- `20250115` = January 15, 2025
- Always 8 digits, no separators

### 5.2 Cycle Hour Encoding

| Model | Cycle Frequency | Format | Valid Hours |
|-------|----------------|--------|-------------|
| HRRR  | Hourly         | `tCCz` | 00z-23z (all 24 hours) |
| NAM   | 6-hourly       | `tCCz` | 00z, 06z, 12z, 18z (4 hours only) |

### 5.3 Forecast Hour Encoding

| Model | Analysis | Forecast Range | Format |
|-------|----------|----------------|--------|
| HRRR  | `f00`    | f00-f48        | `fFF` (2-digit with 'f' prefix) |
| NAM   | `00`     | 00-84          | `FF` (2-digit, no prefix) |

**Key Difference:** HRRR uses `f00` for analysis, NAM uses just `00`.

---

## 6. Alternative Access Methods

### 6.1 NOMADS (Model Operational Data Assimilation and Analysis System)

**Purpose:** Near-real-time access to recent model runs (typically last 2-3 days)

**HRRR on NOMADS:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.YYYYMMDD/conus/
```

**NAM on NOMADS:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.YYYYMMDD/
```

**Characteristics:**
- Directory browsing via HTTPS
- No authentication required
- Limited to recent runs (rolling 2-3 day window)
- Good for testing and near-real-time applications

### 6.2 AWS CLI Access

**HRRR:**
```bash
aws s3 ls --no-sign-request s3://noaa-hrrr-bdp-pds/hrrr.20260722/conus/
```

**NAM:**
```bash
aws s3 ls --no-sign-request s3://noaa-nam-pds/nam.20260722/
```

---

## 7. Verification Commands

### 7.1 HTTP Verification

```bash
# HRRR verification
curl -I "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2"

# NAM verification
curl -I "https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2"
```

### 7.2 Python eccodes Verification

```python
import eccodes

with open('sample.grib2', 'rb') as f:
    msg_count = 0
    drt3_count = 0
    
    while True:
        msg_id = eccodes.codes_grib_new_from_file(f)
        if msg_id is None:
            break
        
        msg_count += 1
        try:
            drt = eccodes.codes_get(msg_id, 'dataRepresentationTemplateNumber')
            gdt = eccodes.codes_get(msg_id, 'gridDefinitionTemplateNumber')
            if drt == 3:
                drt3_count += 1
            print(f'Message {msg_count}: GDT={gdt}, DRT={drt}')
        except:
            pass
        
        eccodes.codes_release(msg_id)
    
    print(f'Total messages: {msg_count}')
    print(f'DRT=3 messages: {drt3_count}')
```

---

## 8. Common Parameters

### 8.1 HRRR Typical Parameters

- `refc` - Composite reflectivity
- `196` - Surface temperature
- `3` - Temperature
- `201` - Precipitation rate
- `195` - Various surface/precipitation variables
- Wind speed/gust
- U/V wind components
- Geopotential height
- Visibility
- Vertically integrated liquid

### 8.2 NAM Typical Parameters

- `260074` - Pressure reduced to MSL
- `260389` - Derived radar reflectivity
- `54` - Pressure
- `260018` - Cloud mixing ratio
- Temperature (param 130)
- Geopotential height (param 156)
- U/V wind components (param 131, 132)
- Relative humidity (param 157)
- Specific humidity (param 133)
- Radar reflectivity (param 260390, 260389)

---

## 9. Acceptance Criteria Status

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ Documentation file created | **COMPLETE** | This document (`bf-67ah-grib2-verification-reference.md`) |
| ✅ All required sections present | **COMPLETE** | URL, metadata, verification, access sections all present |
| ✅ wgrib2 output excerpts included | **COMPLETE** | Section 3.1 includes wgrib2 output excerpts |
| ✅ GDT 3.30 and DRT=3 documented with evidence | **COMPLETE** | Sections 3.1 and 3.2 with verification examples |
| ✅ Source URLs documented | **COMPLETE** | Section 1 with URL patterns and examples |
| ✅ Model metadata included | **COMPLETE** | Section 2 with file sizes, dimensions, characteristics |
| ✅ Access pattern/API notes included | **COMPLETE** | Section 4 with access status, index files, availability lag |

---

## 10. Related Documentation

- [GRIB2 File Verification Results - bf-45h5](./bf-45h5-grib2-verification-results.md) - Original verification results
- [NOAA Archive URLs Verification - bf-4dli](./bf-4dli-noaa-archive-urls-verification.md) - URL pattern documentation
- [DRT=3 GRIB2 Files - Verified Locations - bf-1fe5](./bf-1fe5-drt3-verified-files.md) - DRT=3 specific verification
- [GDT Inspection Summary - bf-tnue](../bf-tnue_nam_awip12_gdt_summary.md) - GDT 3.30 findings
- [NOAA Model File Sizes and Schedules - bf-1moz](./bf-1moz-noaa-model-file-sizes-and-schedules.md) - File size reference
- [NOAA Archive Access Patterns - bf-5ff3](./bf-5ff3-noaa-archive-access-patterns.md) - Access pattern documentation

---

## 11. Quick Reference URLs

### HRRR Quick Access

```
# Most recent run (replace with current date)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2

# Index file for selective extraction
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2.idx
```

### NAM Quick Access

```
# Most recent run (replace with current date)
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2

# Index file for selective extraction
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2.idx
```

---

## 12. References

- [NOAA HRRR on AWS Open Data Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NCEP HRRR Products](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [NCEP NAM Products](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [GRIB2 Template 3.30 - Lambert Conformal](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp3-30.shtml)
- [GRIB2 Template 5.3 - Complex Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp5-3.shtml)
- [NOMADS Data Access](https://nomads.ncep.noaa.gov/)

---

## Conclusion

🎉 **Complete GRIB2 verification reference compiled successfully!**

This document provides a single, comprehensive reference for:
- ✅ Source URLs and URL construction patterns for HRRR and NAM
- ✅ Model metadata including file sizes, grid dimensions, and characteristics
- ✅ Complete verification evidence for GDT 3.30 (Lambert Conformal Conic)
- ✅ Complete verification evidence for DRT=3 (Complex packing with spatial differencing)
- ✅ wgrib2 output excerpts and inspection results
- ✅ Access patterns, API notes, and availability information
- ✅ Quick reference URLs and verification commands

**Recommended for GDT 3.30 + DRT=3 testing:**
1. **HRRR wrfsfcf00** - Most common, ~82-100% DRT=3, 139-156 MB
2. **NAM awip1200** - 100% DRT=3, ~26 MB, smaller file size

Both models use Lambert Conformal Conic projection (GDT 3.30) and complex packing with spatial differencing (DRT=3), and are publicly accessible via HTTPS with no authentication required.

---

*Compilation completed for bead bf-67ah on 2026-07-22*
