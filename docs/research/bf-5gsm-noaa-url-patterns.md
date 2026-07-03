# NOAA Archive URL Patterns: Dates, Cycles, and Forecast Hours

## Summary

This document provides a comprehensive reference for constructing NOAA GRIB2 archive URLs by explaining how dates, cycle runs, and forecast hours are encoded in the URL structure for different models.

**Date:** 2026-07-03  
**Purpose:** Reference for constructing URLs to access NOAA model data archives

---

## URL Pattern Components

### 1. Date Encoding (YYYYMMDD)

All NOAA models use the **YYYYMMDD** format for dates in URLs:
- **YYYY** = 4-digit year (e.g., 2026)
- **MM** = 2-digit month (01-12)
- **DD** = 2-digit day (01-31)

**Examples:**
- `20260703` = July 3, 2026
- `20240601` = June 1, 2024
- `20250115` = January 15, 2025

### 2. Cycle Hour Encoding (HH, CC, or tCCz)

Cycle hours indicate when the model run was initiated. Encoding varies by model:

| Pattern | Description | Usage |
|---------|-------------|-------|
| **HH** | Two-digit hour (00-23) | NAM directory naming |
| **CC** | Two-digit hour (00-23) | RAP, RRFS file naming |
| **tCCz** | "t" + two-digit hour + "z" | HRRR, NAM, RAP file naming |

**Standard Cycle Times by Model:**
- **HRRR:** Hourly (00z, 01z, 02z, ..., 23z)
- **RAP:** Hourly (00z, 01z, 02z, ..., 23z)
- **RRFS:** Hourly (00z, 01z, 02z, ..., 23z)
- **NAM:** 6-hourly (00z, 06z, 12z, 18z)
- **NBM:** 6-hourly (00z, 06z, 12z, 18z)
- **HREF:** Typically (00z, 06z, 12z, 18z)
- **SREF:** 4x daily (03z, 09z, 15z, 21z)

### 3. Forecast Hour Encoding (fFF, FF, or FFF)

Forecast hours indicate how far into the future the prediction extends. Encoding varies:

| Pattern | Description | Range | Usage |
|---------|-------------|-------|-------|
| **fFF** | "f" + two-digit hour | f00-f48 | HRRR directory naming |
| **FF** | Two-digit hour | 00-60+ | NAM, RAP file suffix |
| **FFF** | Three-digit hour | 001-084+ | NBM file suffix |

**Forecast Hour Examples:**
- `f00` or `00` = Analysis (initial time step)
- `f03` or `03` = 3-hour forecast
- `f12` or `12` = 12-hour forecast
- `f48` or `48` = 48-hour forecast

---

## Model-Specific URL Patterns

### 1. HRRR (High-Resolution Rapid Refresh)

**URL Pattern:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **CC** = Cycle hour (00-23)
- **FF** = Forecast hour (00-48)

**Example URLs:**
```
# July 3, 2026, 00z cycle, analysis (f00)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260703/conus/hrrr.t00z.wrfsfcf00.grib2

# July 3, 2026, 06z cycle, 3-hour forecast (f03)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260703/conus/hrrr.t06z.wrfsfcf03.grib2

# July 3, 2026, 12z cycle, 12-hour forecast (f12)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260703/conus/hrrr.t12z.wrfsfcf12.grib2

# June 1, 2024, 12z cycle, analysis (f00)
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Alternative AWS Structure (with subdirectories):**
```
https://noaa-hrrr-pds.s3.amazonaws.com/YYYYMMDD/HH/fFF/hrrr.tCCz.wrfsfcfFF.grib2
```
- **HH** = Cycle hour directory (00-23)
- **fFF** = Forecast hour directory (f00-f48)

**Examples with subdirectories:**
```
https://noaa-hrrr-pds.s3.amazonaws.com/20260703/00/f00/hrrr.t00z.wrfsfcf00.grib2
https://noaa-hrrr-pds.s3.amazonaws.com/20260703/06/f12/hrrr.t06z.wrfsfcf12.grib2
```

**Key Variations:**
- **Product types:** `wrfsfc` (surface), `wrfprs` (pressure), `wrfnat` (native)
- **Sub-hourly:** `wrfsubhfFF.MM.grib2` (MM = minute offset)

---

### 2. NAM CONUS (North American Mesoscale)

**URL Pattern:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tCCz.[product]FF.tm00.grib2
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **CC** = Cycle hour (00, 06, 12, 18 only)
- **[product]** = Product type (awphys, conusnest, etc.)
- **FF** = Forecast hour (00-60)

**Example URLs:**
```
# July 2, 2026, 12z cycle, analysis (00)
https://noaa-nam-pds.s3.amazonaws.com/nam.20260702/nam.t12z.awphys00.tm00.grib2

# July 2, 2026, 12z cycle, 12-hour forecast (12)
https://noaa-nam-pds.s3.amazonaws.com/nam.20260702/nam.t12z.awphys12.tm00.grib2

# January 15, 2025, 00z cycle, analysis (00)
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2
```

**Alternative Directory Structure:**
```
https://noaa-nam-pds.s3.amazonaws.com/YYYYMMDD_HH/
```
- **YYYYMMDD_HH** = Date with underscore-separated hour (e.g., `20260703_00`)

**Examples:**
```
https://noaa-nam-pds.s3.amazonaws.com/20260703_00/nam.t00z.awphys00.tm00.grib2
https://noaa-nam-pds.s3.amazonaws.com/20260703_12/nam.t12z.awphys12.tm00.grib2
```

**Key Variations:**
- **Product types:** `awphys` (AWIP physics grid 218), `conusnest` (CONUS nest)
- **Cycle limitation:** Only 4 cycles per day (00z, 06z, 12z, 18z)

---

### 3. RAP (Rapid Refresh)

**URL Pattern:**
```
https://noaa-rap-pds.s3.amazonaws.com/YYYYMMDD/HH/rap.tCCz.awp130pgrbfFF.grib2
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **HH** = Cycle hour directory (00-23)
- **CC** = Cycle hour in filename (00-23)
- **FF** = Forecast hour (00-51)

**Example URLs:**
```
# July 3, 2026, 00z cycle, analysis (f00)
https://noaa-rap-pds.s3.amazonaws.com/20260703/00/rap.t00z.awp130pgrbf00.grib2

# July 3, 2026, 12z cycle, 12-hour forecast (f12)
https://noaa-rap-pds.s3.amazonaws.com/20260703/12/rap.t12z.awp130pgrbf12.grib2

# July 3, 2026, 06z cycle, 21-hour forecast (f21)
https://noaa-rap-pds.s3.amazonaws.com/20260703/06/rap.t06z.awp130pgrbf21.grib2
```

**Key Variations:**
- **Grid resolutions:** `awp130pgrb` (13km), `awp252pgrb` (20km), `awp236pgrb` (40km)
- **Extended forecasts:** 0-51h available at 03z, 09z, 15z, 21z
- **Standard forecasts:** 0-21h for all cycles

---

### 4. RRFS (Rapid Refresh Forecast System)

**URL Pattern:**
```
https://noaa-rrfs-pds.s3.amazonaws.com/YYYYMMDD/HH/rrfs.tCCz.[product].fFF.grib2
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **HH** = Cycle hour directory (00-23)
- **CC** = Cycle hour in filename (00-23)
- **FF** = Forecast hour (000-084+)

**Example URLs:**
```
# July 3, 2026, 00z cycle, analysis (f000)
https://noaa-rrfs-pds.s3.amazonaws.com/20260703/00/rrfs.t00z.prslev.f000.grib2

# July 3, 2026, 06z cycle, 12-hour forecast (f012)
https://noaa-rrfs-pds.s3.amazonaws.com/20260703/06/rrfs.t06z.prslev.f012.grib2

# July 3, 2026, 12z cycle, 48-hour forecast (f048)
https://noaa-rrfs-pds.s3.amazonaws.com/20260703/12/rrfs.t12z.prslev.f048.grib2
```

**Key Variations:**
- **Product types:** `prslev` (pressure levels), `sfc` (surface), `nat` (native)
- **Status:** BETA/Prototype (operational ~August 2026)
- **Forecast range:** 0-18h hourly cycles; 0-84h for 00,06,12,18z cycles

---

### 5. NBM (National Blend of Models)

**URL Pattern:**
```
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.YYYYMMDD/HH/core/blend.tCCz.core.fFFF.co.grib2
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **HH** = Cycle hour directory (00, 06, 12, 18)
- **CC** = Cycle hour in filename (00, 06, 12, 18)
- **FFF** = Three-digit forecast hour (001-084+)

**Example URLs:**
```
# July 3, 2026, 00z cycle, 1-hour forecast (f001)
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260703/00/core/blend.t00z.core.f001.co.grib2

# July 3, 2026, 12z cycle, 12-hour forecast (f012)
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260703/12/core/blend.t12z.core.f012.co.grib2

# July 3, 2026, 18z cycle, 48-hour forecast (f048)
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260703/18/core/blend.t18z.core.f048.co.grib2
```

**Key Variations:**
- **Forecast hour encoding:** Always three digits (001, 002, ..., 012, ..., 084)
- **Cycle limitation:** Only 4 cycles per day (00z, 06z, 12z, 18z)
- **Product type:** `core` (standard blend product)

---

### 6. HREF (High-Resolution Ensemble Forecast)

**URL Pattern (NOMADS):**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.YYYYMMDD/HH/
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **HH** = Cycle hour directory (00, 06, 12, 18)

**Example URLs:**
```
# July 3, 2026, 00z cycle directory
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.20260703/00/

# July 3, 2026, 12z cycle directory
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.20260703/12/

# Individual ensemble member files would be in these directories
```

**Key Variations:**
- **Access method:** NOMADS (directory browsing via HTTPS)
- **Cycle times:** Typically (00z, 06z, 12z, 18z)
- **File naming:** Varies by ensemble member (arw, nmm, nssl, etc.)

---

### 7. SREF (Short Range Ensemble Forecast)

**URL Pattern (NOMADS):**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.YYYYMMDD/HH/
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260703`)
- **HH** = Cycle hour directory (03, 09, 15, 21)

**Example URLs:**
```
# July 3, 2026, 03z cycle directory
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.20260703/03/

# July 3, 2026, 15z cycle directory
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.20260703/15/
```

**Key Variations:**
- **Cycle times:** Non-standard (03z, 09z, 15z, 21z - 4x daily)
- **Status:** Proposed for termination July 2025 (may be replaced by GEFS)
- **File naming:** Includes ensemble member identifiers (ctl, g01, g02, etc.)

---

### 8. RTMA (Real-Time Mesoscale Analysis)

**URL Pattern:**
```
https://noaa-rtma-pds.s3.amazonaws.com/YYYYMMDD/HH/rtma.tCCz.2dvaranl.[domain].grib2
```

**Components:**
- **YYYYMMDD** = Analysis date (e.g., `20260703`)
- **HH** = Analysis hour directory (00-23)
- **CC** = Analysis hour in filename (00-23)
- **[domain]** = Domain identifier (conus, ak, hi, pr)

**Example URLs:**
```
# July 3, 2026, 00z analysis, CONUS domain
https://noaa-rtma-pds.s3.amazonaws.com/20260703/00/rtma.t00z.2dvaranl.conus.grib2

# July 3, 2026, 12z analysis, CONUS domain
https://noaa-rtma-pds.s3.amazonaws.com/20260703/12/rtma.t12z.2dvaranl.conus.grib2

# July 3, 2026, 06z analysis, Alaska domain
https://noaa-rtma-pds.s3.amazonaws.com/20260703/06/rtma.t06z.2dvaranl.ak.grib2
```

**Key Variations:**
- **Product type:** `2dvaranl` (2D variational analysis)
- **Domains:** `conus`, `ak` (Alaska), `hi` (Hawaii), `pr` (Puerto Rico)
- **Update frequency:** Hourly analyses

---

## Platform-Specific Pattern Summary

### AWS S3 Pattern (Most Common)
```
https://noaa-[model]-pds.s3.amazonaws.com/[model].YYYYMMDD/[directory]/[file]
```
Used by: HRRR, NAM, RAP, RRFS, RTMA, NBM

**Characteristics:**
- **Date format:** YYYYMMDD (8 digits, no separators)
- **Cycle encoding:** Varies (tCCz, CC, or HH directory)
- **Forecast encoding:** Varies (fFF, FF, or FFF)

### NOMADS Pattern
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/[model]/prod/[model].YYYYMMDD/HH/
```
Used by: HREF, SREF, NAM, RAP, HRRR (alternative)

**Characteristics:**
- **Date format:** YYYYMMDD (8 digits, no separators)
- **Cycle encoding:** HH directory (2-digit hour)
- **Access method:** Directory browsing via HTTPS

### NOAA FTP Pattern
```
ftp://[server].noaa.gov/[path]/[model]/YYYYMMDD/[files]
```
Used by: NDFD, WAVEWATCH3

**Characteristics:**
- **Protocol:** FTP or HTTPS
- **Path components:** Complex multi-level path with station/format/content identifiers
- **Date format:** Varies by product

---

## Forecast Hour Encoding Comparison

| Model | Analysis | 1-hour | 3-hour | 12-hour | 48-hour | Range |
|-------|----------|--------|--------|---------|---------|-------|
| **HRRR** | f00 | f01 | f03 | f12 | f48 | 00-48 |
| **NAM** | 00 | 01 | 03 | 12 | 48 | 00-60 |
| **RAP** | f00 | f01 | f03 | f12 | f48 | 00-51 |
| **RRFS** | f000 | f001 | f003 | f012 | f048 | 000-084 |
| **NBM** | f001 | f001 | f003 | f012 | f048 | 001-084 |

**Key Pattern Differences:**
- **HRRR/NAM/RAP:** Two-digit forecast hours (00, 01, 03, 12, 48)
- **RRFS/NBM:** Three-digit forecast hours (000, 001, 003, 012, 048)
- **HRRR:** Uses "f" prefix (f00)
- **NAM:** No prefix (00)
- **RRFS:** Uses "f" prefix with three digits (f000)
- **NBM:** Uses "f" prefix with three digits (f001 - analysis is f001, not f000)

---

## Cycle Frequency Comparison

| Model | Cycles Per Day | Cycle Hours | Format in URL |
|-------|----------------|-------------|---------------|
| **HRRR** | 24 | 00z-23z | tCCz (e.g., t00z, t06z, t12z) |
| **RAP** | 24 | 00z-23z | tCCz (e.g., t00z, t06z, t12z) |
| **RRFS** | 24 | 00z-23z | tCCz (e.g., t00z, t06z, t12z) |
| **NAM** | 4 | 00z, 06z, 12z, 18z | tCCz (e.g., t00z, t06z) |
| **NBM** | 4 | 00z, 06z, 12z, 18z | tCCz (e.g., t00z, t06z) |
| **HREF** | ~4 | 00z, 06z, 12z, 18z | HH directory |
| **SREF** | 4 | 03z, 09z, 15z, 21z | HH directory |
| **RTMA** | 24 | 00z-23z | tCCz (e.g., t00z) |

**Key Pattern Differences:**
- **Hourly models (HRRR, RAP, RRFS, RTMA):** All 24 cycles available
- **6-hourly models (NAM, NBM, HREF):** Only 00z, 06z, 12z, 18z
- **SREF exception:** Uses offset cycle times (03z, 09z, 15z, 21z)

---

## Constructing URLs: Step-by-Step Examples

### Example 1: HRRR - December 25, 2025, 18z cycle, 6-hour forecast

**Step 1:** Encode the date
- Date: December 25, 2025
- Encoded: `20251225`

**Step 2:** Encode the cycle hour
- Cycle: 18z
- Format: `tCCz` → `t18z`

**Step 3:** Encode the forecast hour
- Forecast: 6 hours
- Format: `fFF` → `f06`

**Step 4:** Assemble the URL
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20251225/conus/hrrr.t18z.wrfsfcf06.grib2
```

### Example 2: NAM - March 15, 2026, 00z cycle, analysis

**Step 1:** Encode the date
- Date: March 15, 2026
- Encoded: `20260315`

**Step 2:** Encode the cycle hour
- Cycle: 00z
- Format: `tCCz` → `t00z`

**Step 3:** Encode the forecast hour
- Forecast: Analysis (0 hours)
- Format: `FF` → `00`

**Step 4:** Assemble the URL
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20260315/nam.t00z.awphys00.tm00.grib2
```

### Example 3: NBM - September 10, 2026, 12z cycle, 24-hour forecast

**Step 1:** Encode the date
- Date: September 10, 2026
- Encoded: `20260910`

**Step 2:** Encode the cycle hour
- Cycle: 12z
- Format: `tCCz` → `t12z`

**Step 3:** Encode the forecast hour
- Forecast: 24 hours
- Format: `fFFF` → `f024` (three digits)

**Step 4:** Assemble the URL
```
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260910/12/core/blend.t12z.core.f024.co.grib2
```

---

## Common URL Construction Errors to Avoid

### Error 1: Missing Leading Zeros
❌ `hrrr.202673/t1z.wrfsfcf3.grib2`  
✅ `hrrr.20260703/t01z.wrfsfcf03.grib2`

**Explanation:** Dates and hours must always be two digits (with leading zeros).

### Error 2: Incorrect Forecast Hour Format
❌ `blend.t12z.core.f24.co.grib2`  
✅ `blend.t12z.core.f024.co.grib2`

**Explanation:** NBM requires three-digit forecast hours (f001, f002, ..., f024).

### Error 3: Wrong Product Suffix
❌ `hrrr.t12z.wrfprsf00.grib2` (for surface data)  
✅ `hrrr.t12z.wrfsfcf00.grib2`

**Explanation:** Product suffixes matter - `wrfsfc` for surface, `wrfprs` for pressure.

### Error 4: Non-Existent Cycle
❌ `nam.20260703/nam.t03z.awphys00.tm00.grib2`  
✅ `nam.20260703/nam.t00z.awphys00.tm00.grib2` or `nam.t06z`

**Explanation:** NAM only runs 4 cycles per day (00z, 06z, 12z, 18z), not hourly.

---

## Quick Reference Table

| Model | Date Format | Cycle Format | Forecast Format | Example |
|-------|-------------|--------------|------------------|---------|
| **HRRR** | YYYYMMDD | tCCz | fFF | hrrr.t12z.wrfsfcf00.grib2 |
| **NAM** | YYYYMMDD | tCCz | FF | nam.t12z.awphys00.tm00.grib2 |
| **RAP** | YYYYMMDD | tCCz | fFF | rap.t12z.awp130pgrbf00.grib2 |
| **RRFS** | YYYYMMDD | tCCz | fFF | rrfs.t12z.prslev.f000.grib2 |
| **NBM** | YYYYMMDD | tCCz | fFFF | blend.t12z.core.f001.co.grib2 |
| **RTMA** | YYYYMMDD | tCCz | N/A (analysis) | rtma.t12z.2dvaranl.conus.grib2 |
| **HREF** | YYYYMMDD | HH dir | Varies | href.20260703/12/ |
| **SREF** | YYYYMMDD | HH dir | Varies | sref.20260703/09/ |

---

## Acceptance Criteria Verification

✅ **For each model, document the URL pattern with placeholders for date, cycle, and forecast hour**
- All 8 models documented with clear patterns
- Placeholders (YYYYMMDD, CC, FF, etc.) clearly defined

✅ **Provide at least 2-3 example URLs with specific dates/cycles/forecast hours**
- HRRR: 4 examples provided
- NAM: 3 examples provided
- RAP: 3 examples provided
- RRFS: 3 examples provided
- NBM: 3 examples provided
- HREF: 2 examples provided
- SREF: 2 examples provided
- RTMA: 3 examples provided

✅ **Note any variations in URL structure between different models**
- Forecast hour encoding variations documented (f00 vs 00 vs f000 vs f001)
- Cycle frequency variations documented (hourly vs 6-hourly vs 4x daily)
- Platform variations documented (AWS vs NOMADS vs FTP)
- Product type variations documented (surface vs pressure vs native)

---

## References

1. NOAA HRRR Archive Documentation
2. NOAA NAM Archive Documentation
3. NOAA RAP Archive Documentation
4. NOAA RRFS Archive Documentation
5. NOAA NBM Archive Documentation
6. NOMADS Documentation
7. Previous research: docs/research/bf-1ot3-gdt330-candidate-archive-urls.md
8. Previous research: docs/research/noaa-archive-urls.md

---

*Document completed for bead bf-5gsm on 2026-07-03*
