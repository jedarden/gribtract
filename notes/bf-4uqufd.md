# GRIB2 Download Results - Bead bf-4uqufd

## Task Summary

Download at least one candidate GRIB2 file from NOAA NAM/NEXRAD sources identified in previous research.

**Download Date:** 2026-07-27
**Task Reference:** bf-4uqufd

---

## Files Downloaded

### 1. NAM (North American Mesoscale Model) File

**Source:** NOAA NAM AWS Open Data Registry
**Source URL:** `https://noaa-nam-pds.s3.amazonaws.com/nam.20260727/nam.t00z.awip1200.tm00.grib2`

**Download Metrics:**
- **HTTP Status:** 200 OK
- **File Size:** 29,145,545 bytes (27.8 MB)
- **Transfer Time:** 2.7 seconds
- **Transfer Speed:** ~10.8 MB/s
- **Download Date:** 2026-07-27

**File Specifications:**
- **Model:** NAM (North American Mesoscale)
- **Product:** NAM CONUS analysis on Grid 218 (awip12)
- **Grid:** NCEP Grid 218 (awip12) - Lambert Conformal Conic
- **File Type:** GRIB2 Edition 2
- **Cycle Date:** 2026-07-27
- **Cycle Time:** 00z (00:00 UTC)
- **Forecast Hour:** F00 (analysis)
- **Valid Time:** 2026-07-27 00:00 UTC

**File Location:** `/home/coding/gribtract/downloads/bf-4uqufd/nam_20260727_t00z_awip1200_tm00.grib2`

---

## GRIB2 Format Verification

### Header Check
```bash
xxd nam_20260727_t00z_awip1200_tm00.grib2 | head -1
```

**Result:**
```
00000000: 4752 4942 0000 0002 0000 0000 0003 9a28  GRIB...........(...
```

**Verification:**
- ✅ `GRIB` - Valid GRIB magic number
- ✅ `0000 0002` - Edition 2 (GRIB2 format identifier)
- ✅ File signature confirms GRIB2 Edition 2 format

---

## NEXRAD/Radar Data Context

The NAM model file includes radar-related parameters as part of its meteorological analysis:

**Radar Parameters Found in NAM Files:**
- **Derived radar reflectivity** (param=260389)
- **Maximum/Composite radar reflectivity** (param=260390)
- **Vertically-integrated liquid** (param=260136)

These are model-simulated radar products that are part of the standard NAM GRIB2 output, providing simulated radar reflectivity fields for weather forecasting applications.

---

## Archive Source Details

### NOAA NAM AWS Open Data Registry

**Archive Platform:** AWS Open Data Registry
**Bucket:** `noaa-nam-pds` (NOAA NAM Public Data Store)
**Region:** us-east-1
**Access:** Public HTTP/S3 (no authentication required)
**CDN:** Amazon CloudFront (global distribution)

### URL Structure
**Pattern:** `https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awip1200.tmFF.grib2`

Where:
- `YYYYMMDD`: Cycle date (year, month, day)
- `HH`: Cycle hour (00, 06, 12, or 18)
- `FF`: Forecast hour (00 = analysis, 01-84 = forecasts)

**Access Characteristics:**
- ✅ No authentication required (public AWS Open Data)
- ✅ No rate limiting observed
- ✅ High-speed download (~10.8 MB/s)
- ✅ Supports range requests
- ✅ Global CDN for fast access

---

## Acceptance Criteria Verification

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ Successfully downloaded at least one GRIB2 file | **COMPLETE** | Downloaded NAM file (27.8 MB) |
| ✅ File exists and is non-zero size | **COMPLETE** | 29,145,545 bytes verified |
| ✅ File is identified as GRIB2 format | **COMPLETE** | Valid GRIB2 Edition 2 header confirmed |
| ✅ Source is NOAA NAM/NEXRAD source | **COMPLETE** | NOAA NAM AWS Open Data Registry |

---

## Historical Context

This download builds on previous research beads that identified NOAA NAM sources:
- **bf-6bcol:** Identified NAM as RESTRICTED (403) on NOMADS but available on AWS
- **bf-45h5:** Verified NAM files contain GDT 3.30 + DRT=3 with radar parameters
- **bf-i5ol:** Documented successful NAM download from AWS Open Data (2025-01-15)

**Key Finding:** While NOMADS access shows 403 Forbidden for NAM, the AWS Open Data Registry provides reliable public access to the same NAM products.

---

## Technical Notes

### GRIB2 File Characteristics

Based on previous verification (bf-45h5, bf-i5ol), NAM awip1200 files include:

**Message Composition:**
- **Total Messages:** ~196 messages (for similar NAM awip12 files)
- **Grid Definition:** GDT 3.30 (Lambert Conformal Conic)
- **Data Packing:** DRT=3 (Complex packing with spatial differencing)
- **Grid Size:** 614×428 points (~12 km resolution)

**Content Includes:**
- Surface variables: temperature, dewpoint, wind, pressure
- Precipitation: total liquid, snow, freezing rain
- Upper atmosphere: geopotential height, temperature, wind
- **Radar products:** derived radar reflectivity, composite reflectivity
- Derived fields: visibility, cloud cover, instability indices

---

## Related Files

**Downloaded File:** `downloads/bf-4uqufd/nam_20260727_t00z_awip1200_tm00.grib2` (27.8 MB)
**Reference Documentation:** `samples/bf-i5ol-nam-awip12-provenance.md`
**Previous Verification:** `notes/bf-45h5.md`

---

## References

- **NAM Model Documentation:** https://www.nco.ncep.noaa.gov/pmb/products/nam/
- **NOAA AWS Open Data:** https://registry.opendata.aws/noaa-nam/
- **NEXRAD Information:** https://www.weather.gov/jetserver/nexrad
- **GRIB2 Specification:** WMO FM 92 GRIB Edition 2

---

*Download completed for bead bf-4uqufd on 2026-07-27*