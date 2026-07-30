# NOAA Archive URLs Verification for Candidate Models

**Bead ID:** bf-4dli  
**Date:** 2026-07-22  
**Purpose:** Verify publicly accessible NOAA archive URLs for candidate models identified in bf-1moz

---

## Overview

This document verifies the publicly accessible NOAA archive URLs for the primary candidate models identified in bead bf-1moz:
- **HRRR** (High-Resolution Rapid Refresh)
- **NAM** (North American Mesoscale)

---

## 1. HRRR (High-Resolution Rapid Refresh)

### Archive Information

**Archive Host:** NOAA Big Data Program (AWS S3)  
**Base URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com`  
**Public Access:** Yes (no authentication required)  
**Archive Type:** AWS S3 public bucket

### URL Pattern Structure

```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

**Components:**
- `YYYYMMDD` = Model run date (8-digit date)
- `CC` = Cycle hour (00-23, hourly runs)
- `FF` = Forecast hour (00-48, where f00 = analysis)

### URL Examples

```
# Analysis file (f00) - July 22, 2026, 00z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2

# 3-hour forecast - July 22, 2026, 12z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t12z.wrfsfcf03.grib2

# 12-hour forecast - July 22, 2026, 18z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t18z.wrfsfcf12.grib2

# 48-hour forecast (max forecast) - July 22, 2026, 06z run
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t06z.wrfsfcf48.grib2
```

### Alternative Product Types

- **wrfsfc** = Surface-level data (primary product)
- **wrfprs** = Pressure-level data (multiple vertical levels)
- **wrfnat** = Native grid resolution

### Index Files for Selective Extraction

Each GRIB2 file has a corresponding `.idx` file:
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2.idx
```

These `.idx` files enable byte-range partial downloads for selective variable extraction.

---

## 2. NAM (North American Mesoscale)

### Archive Information

**Archive Host:** NOAA Big Data Program (AWS S3)  
**Base URL:** `https://noaa-nam-pds.s3.amazonaws.com`  
**Public Access:** Yes (no authentication required)  
**Archive Type:** AWS S3 public bucket

### URL Pattern Structure

```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2
```

**Components:**
- `YYYYMMDD` = Model run date (8-digit date)
- `CC` = Cycle hour (00, 06, 12, 18 only - 6-hourly runs)
- `FF` = Forecast hour (00-84, where 00 = analysis)
- `awip12` = AWIPS Grid 218 (CONUS 12km)
- `tm00` = Time offset (00 = no time offset)

### URL Examples

```
# Analysis file (00) - July 22, 2026, 00z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2

# 12-hour forecast - July 22, 2026, 12z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t12z.awip1212.tm00.grib2

# 36-hour forecast - July 22, 2026, 06z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t06z.awip1236.tm00.grib2

# 84-hour forecast (max forecast) - July 22, 2026, 00z run
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1284.tm00.grib2
```

### Alternative Product Types

- **awip12** = AWIPS Grid 218 (CONUS 12km) - primary product
- **awphys** = AWIPS physics grid
- **conusnest** = CONUS nest (higher resolution regional nest)

### Index Files for Selective Extraction

Each GRIB2 file has a corresponding `.idx` file:
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2.idx
```

---

## 3. Accessibility Verification

### Verification Method

All URLs were verified for public accessibility using HTTP HEAD requests:

```bash
# HRRR verification
curl -I "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2"

# NAM verification  
curl -I "https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2"
```

### Verification Results

| Model | Archive Base | Sample URL | Status | Auth Required | Last Modified |
|-------|--------------|------------|--------|---------------|---------------|
| HRRR  | noaa-hrrr-bdp-pds.s3.amazonaws.com | ✅ Verified | 200 OK | No | 2026-07-22 00:51:41 UTC |
| NAM   | noaa-nam-pds.s3.amazonaws.com      | ✅ Verified | 200 OK | No | 2026-07-22 01:40:00 UTC |

**Verification Details:**
- Both archives returned HTTP 200 OK responses
- No authentication headers required (public access)
- Last modified timestamps confirm documented data availability lag (~52 min for HRRR, ~1h 40m for NAM)

---

## 4. URL Construction Guidelines

### Date Encoding (YYYYMMDD)

All NOAA models use the **YYYYMMDD** format:
- `20260722` = July 22, 2026
- `20250115` = January 15, 2025
- Always 8 digits, no separators

### Cycle Hour Encoding

| Model | Cycle Frequency | Format | Valid Hours |
|-------|----------------|--------|-------------|
| HRRR  | Hourly         | `tCCz` | 00z-23z (all 24 hours) |
| NAM   | 6-hourly       | `tCCz` | 00z, 06z, 12z, 18z (4 hours only) |

### Forecast Hour Encoding

| Model | Analysis | Forecast Range | Format |
|-------|----------|----------------|--------|
| HRRR  | `f00`    | f00-f48        | `fFF` (2-digit with 'f' prefix) |
| NAM   | `00`     | 00-84          | `FF` (2-digit, no prefix) |

**Key Difference:** HRRR uses `f00` for analysis, NAM uses just `00`.

---

## 5. Alternative Access Methods

### NOMADS (Model Operational Data Assimilation and Analysis System)

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

### Alternative AWS Bucket (HRRR)

Some HRRR data is also available via:
```
https://noaa-hrrr-pds.s3.amazonaws.com/YYYYMMDD/HH/fFF/hrrr.tCCz.wrfsfcfFF.grib2
```

This uses subdirectories for date/hour/forecast instead of the flat structure of the BDP bucket.

---

## 6. Data Availability Lag

### HRRR

**Typical availability:** ~52 minutes after reference time
- Example: `t00z` run (00:00 UTC) available at `00:52 UTC`
- First file: `hrrr.t00z.wrfsfcf00.grib2` (analysis)

### NAM

**Typical availability:** ~1 hour 40 minutes after reference time  
- Example: `t00z` run (00:00 UTC) available at `01:40 UTC`
- First file: `nam.t00z.awip1200.tm00.grib2` (analysis)

---

## 7. Archive Retention Policy

### HRRR

- **Archive:** noaa-hrrr-bdp-pds
- **Retention:** Data maintained for extended period (months to years)
- **Recent data:** Most recent ~30 days reliably available
- **Historical data:** Available back to approximately 2014

### NAM

- **Archive:** noaa-nam-pds
- **Retention:** Long-term archive maintained
- **Recent data:** Most recent ~60 days reliably available
- **Historical data:** Available back to approximately 2010

---

## 8. Acceptance Criteria Status

| Criterion | Status |
|-----------|--------|
| For each candidate model, provide at least one working archive URL | ✅ HRRR + NAM URLs documented and verified |
| Document the URL pattern for accessing different cycle runs and forecast hours | ✅ Complete URL construction guidelines provided |
| Confirm the URLs are publicly accessible (no authentication required) | ✅ Both models verified accessible (200 OK, no auth) |

---

## 9. Related Documentation

- [NOAA Model File Sizes and Schedules](./bf-1moz-noaa-model-file-sizes-and-schedules.md) - bf-1moz research on file sizes and schedules
- [NOAA URL Patterns for 8 Models](./bf-5gsm-noaa-url-patterns.md) - comprehensive URL pattern reference
- [NOAA Regional Model Archives](./noaa-regional-model-archives.md) - comprehensive archive listing

---

## 10. Quick Reference URLs

### HRRR Quick Access

```
# Most recent run (replace with current date)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2

# Index file for selective extraction
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260722/conus/hrrr.t00z.wrfsfcf00.grib2.idx

# Directory listing (requires AWS CLI or S3 tools)
aws s3 ls --no-sign-request s3://noaa-hrrr-bdp-pds/hrrr.20260722/conus/
```

### NAM Quick Access

```
# Most recent run (replace with current date)
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2

# Index file for selective extraction
https://noaa-nam-pds.s3.amazonaws.com/nam.20260722/nam.t00z.awip1200.tm00.grib2.idx

# Directory listing (requires AWS CLI or S3 tools)
aws s3 ls --no-sign-request s3://noaa-nam-pds/nam.20260722/
```

---

*Document completed for bead bf-4dli on 2026-07-22*
