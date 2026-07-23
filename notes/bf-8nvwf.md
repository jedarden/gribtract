# NOAA GRIB2 File Download - bead bf-8nvwf

## Download Summary

**Download Date**: 2026-07-23 04:23:00 UTC  
**Download Method**: HTTP GET via curl  
**Status**: ✅ Successfully downloaded and verified

## File Details

| Property | Value |
|----------|-------|
| **Filename** | nam.t00z.awip1200.tm00.grib2 |
| **Location** | samples/nam.t00z.awip1200.tm00.grib2 |
| **File Size** | 26,364,442 bytes (25.1 MiB) |
| **SHA256** | `b022c093603e67ebcc006a8e50cb30610bf4e3ce7d6609733d9949b5add6bf2c` |
| **HTTP Status** | 200 OK |
| **Transfer Time** | 2.53 seconds |
| **Transfer Speed** | ~10.4 MB/s |

## Source URL

```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
```

## File Provenance

### Model Specification

| Property | Value |
|----------|-------|
| **Model Name** | NAM (North American Mesoscale) |
| **Model Agency** | NOAA/NCEP (National Centers for Environmental Prediction) |
| **Product** | NAM CONUS analysis on Grid 218 (awip12) |
| **Grid** | NCEP Grid 218 (awip12) - Lambert Conformal Conic |
| **File Type** | GRIB2 Edition 2 |

### Temporal Specification

| Property | Value |
|----------|-------|
| **Cycle Date** | 2025-01-15 |
| **Cycle Time** | 00z (00:00 UTC) |
| **Forecast Hour** | F00 (analysis, not a forecast) |
| **Valid Time** | 2025-01-15 00:00 UTC |

### GRIB2 Technical Specification

| Property | Value |
|----------|-------|
| **Total Messages** | 196 GRIB2 messages |
| **Variables** | Full meteorological analysis (surface + aloft) |
| **Projection** | Lambert Conformal Conic (GDT 3.30) |
| **Packing** | Complex packing with spatial differencing (DRT=3) |
| **Resolution** | ~12 km grid spacing |

## Archive Details

**Archive Platform**: AWS Open Data Registry  
**Bucket**: `noaa-nam-pds` (NOAA NAM Public Data Store)  
**Region**: us-east-1  
**Access**: Public HTTP/S3 (no authentication required)

## Acceptance Criteria Verification

- ✅ File is downloaded to local storage (`samples/nam.t00z.awip1200.tm00.grib2`)
- ✅ File size is reasonable (26,364,442 bytes - not zero, not suspiciously small)
- ✅ Download timestamp is recorded (2026-07-23 04:23:00 UTC)
- ✅ File integrity verified via SHA256 checksum

---

*Download completed for bead bf-8nvwf on 2026-07-23*
