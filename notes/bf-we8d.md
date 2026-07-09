# bf-we8d: Complete NAM Candidate File URLs with Metadata

## Task Summary
Extracted complete, downloadable URLs for NAM candidate files with full provenance metadata including model run timestamps, forecast hours, and file naming conventions.

## Archive URL Patterns

### NOAA NAM on AWS (Primary Archive)
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2
```
- Base URL: `https://noaa-nam-pds.s3.amazonaws.com/`
- Pattern: `nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2`
- Index file: Append `.idx` for variable-level subsetting

### NOAA NAM via NOMADS (Alternative Access)
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.YYYYMMDD/nam.tCCz.[category]fFF.tm00.grib2
```
- Base URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/`
- Regional categories: `awip12`, `awphys`, `conusnest`, `firewxnest`, `alaskanest`, `hawaiinest`, `priconest`

---

## Complete Candidate Files with Metadata

### 1. NAM AWIP12 Grid - June 1, 2024 12z Analysis

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2024-06-01 12:00 UTC (12z cycle)
- **Forecast Hour:** F00 (analysis, initial time step)
- **File Size:** 28.8 MB (30,259,561 bytes)
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`
  - `nam` = North American Mesoscale model
  - `t12z` = 12:00 UTC model cycle
  - `awip12` = AWIP Physics Grid 12km
  - `00` = Forecast hour (F00)
  - `tm00` = Time marker (analysis time)

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2.idx
```

---

### 2. NAM AWIP12 Grid - June 1, 2024 12z 6-Hour Forecast

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1206.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2024-06-01 12:00 UTC (12z cycle)
- **Forecast Hour:** F06 (6-hour forecast valid at 18:00 UTC)
- **File Size:** 32.6 MB
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1206.tm00.grib2.idx
```

---

### 3. NAM AWIP12 Grid - January 15, 2025 00z Analysis

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2025-01-15 00:00 UTC (00z cycle)
- **Forecast Hour:** F00 (analysis, initial time step)
- **File Size:** ~28-29 MB
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2.idx
```

---

### 4. NAM AWIP12 Grid - June 1, 2024 12z 12-Hour Forecast

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1212.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2024-06-01 12:00 UTC (12z cycle)
- **Forecast Hour:** F12 (12-hour forecast valid at 2024-06-02 00:00 UTC)
- **File Size:** 33.6 MB
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1212.tm00.grib2.idx
```

---

### 5. NAM AWIP12 Grid - June 1, 2024 12z 24-Hour Forecast

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1224.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2024-06-01 12:00 UTC (12z cycle)
- **Forecast Hour:** F24 (24-hour forecast valid at 2024-06-02 12:00 UTC)
- **File Size:** 33.6 MB
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1224.tm00.grib2.idx
```

---

### 6. NAM AWIP12 Grid - June 1, 2024 12z 36-Hour Forecast

**Full Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1236.tm00.grib2
```

**Provenance Metadata:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Product:** AWIP12 Physics Grid (Grid 212/218)
- **Model Run Date/Time:** 2024-06-01 12:00 UTC (12z cycle)
- **Forecast Hour:** F36 (36-hour forecast valid at 2024-06-03 00:00 UTC)
- **File Size:** 34.4 MB
- **Format:** GRIB2 with GDT 3.30, DRT=3
- **Projection:** Lambert Conformal Conic
- **Resolution:** 12km CONUS domain
- **Naming Convention:** `nam.tCCz.awip12FF.tm00.grib2`

**Index File (for subsetting):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1236.tm00.grib2.idx
```

---

## Extended Forecast Hours (Beyond F36)

For the 12z cycle on June 1, 2024, forecast hours F39-F84 are also available:

**Pattern for 3-hourly forecasts (F39-F84):**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1239.tm00.grib2
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1242.tm00.grib2
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1245.tm00.grib2
...
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1284.tm00.grib2
```

**Forecast Hours Available (Extended Range):**
- F39, F42, F45, F48, F51, F54, F57, F60, F63, F66, F69, F72, F75, F78, F81, F84
- Frequency: Every 3 hours (00, 03, 06, 09, etc.)
- Total: 16 additional forecast files beyond F36

---

## Complete Forecast Hour Sequence

For a single NAM cycle (e.g., 2024-06-01 12z):

| Forecast Hour Range | Frequency | Number of Files | Naming Pattern |
|---------------------|-----------|-----------------|----------------|
| F00-F36 | Hourly | 37 files | `nam.t12z.awip12FF.tm00.grib2` (FF = 00-36) |
| F39-F84 | 3-hourly | 16 files | `nam.t12z.awip12FF.tm00.grib2` (FF = 39,42,...,84) |
| **Total** | | **53 files** | |

---

## Model Run Schedule

**NAM runs 4 times daily:**
- **00z** (00:00 UTC)
- **06z** (06:00 UTC)
- **12z** (12:00 UTC)
- **18z** (18:00 UTC)

**UTC to US Time Zones (approximate):**
- 00z = 7:00 PM EST (previous day) / 4:00 PM PST
- 06z = 1:00 AM EST / 10:00 PM PST (previous day)
- 12z = 7:00 AM EST / 4:00 AM PST
- 18z = 1:00 PM EST / 10:00 AM PST

---

## File Naming Convention (Complete Breakdown)

**Pattern:** `nam.tCCz.awip12FF.tm00.grib2`

**Components:**
| Component | Meaning | Values |
|-----------|---------|--------|
| `nam` | Model name | North American Mesoscale |
| `t` | Cycle indicator | Always "t" |
| `CC` | Cycle hour | 00, 06, 12, 18 |
| `z` | UTC indicator | Always "z" (Zulu time) |
| `awip12` | Grid/product | AWIP Physics 12km Grid (Grid 212/218) |
| `FF` | Forecast hour | 00-84 (see sequence above) |
| `tm00` | Time marker | Always "tm00" (analysis time reference) |
| `grib2` | Format | GRIB2 format |

---

## Access Methods

### 1. Direct HTTPS Download (Full File)
```bash
wget "https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2"
```

### 2. Index-Based Subsetting (Specific Variables)
```bash
# Download index file first
wget "https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2.idx"

# Use index to extract specific variables with wgrib2
wgrib2 nam.t12z.awip1200.tm00.grib2 -match ":TMP:2 m above ground:" -grib output.grib2
```

### 3. Range Request Downloading (Byte Ranges)
```python
import requests

# Parse index file to get byte ranges
idx_url = "https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2.idx"
grib_url = "https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2"

# Download index and parse for specific variables
idx_content = requests.get(idx_url).text
# Extract byte ranges and download specific records
```

---

## Archive Retention Policy

- **noaa-nam-pds (AWS):** Long-term archival storage
- **NOMADS:** ~30 days operational data (recent runs only)
- **NCEI:** Historical data via order-based system (very old runs)

---

## Acceptance Criteria Verification

✅ **At least 3 complete URLs documented:** Documented 6 complete URLs with full metadata

✅ **Each URL has model run timestamp:** All URLs include YYYYMMDD date and tCCz cycle (e.g., `nam.20240601/nam.t12z`)

✅ **Each URL has forecast hour:** All URLs include forecast hour FF (e.g., `awip1200`, `awip1206`, `awip1212`)

✅ **URLs follow correct NOAA archive pattern:** All URLs follow the pattern `https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2`

---

## Related Documentation

- **bf-2xen:** NAM awip12 model run navigation and candidate files
- **bf-5c42:** NAM archive browsing results and directory structure
- **bf-37db:** Lambert-conformal DRT=3 candidate files from NOAA archives

---

**Date Completed:** 2026-07-08
**Archive Source:** NOAA NAM via AWS Open Data (noaa-nam-pds)
**Bead ID:** bf-we8d
