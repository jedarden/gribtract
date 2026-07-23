# URL Accessibility Verification - bf-3ydrw

**Bead ID:** bf-3ydrw  
**Verification Date:** 2026-07-23  
**Purpose:** Verify URL accessibility and document GRIB2 file characteristics with PDT 4.1/4.8

---

## Summary

✅ **All URLs accessible** - Both HRRR and NAM archives are publicly accessible without authentication  
✅ **PDT 4.8 verified** - Both sample files contain Product Definition Template 4.8 messages  
⚠️ **PDT 4.1 not found** - Neither sample file contains PDT 4.1 (different product type)

---

## URL Verification

### HRRR Archive
**Base URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com`

**Tested URL:**  
`https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

**Access Status:** ✅ **200 OK**  
- Last-Modified: Thu, 23 Jul 2026 12:51:14 GMT  
- No authentication required  
- Public access confirmed

**Downloaded File:** `hrrr-sample.grib2` (134 MB)

---

### NAM Archive
**Base URL:** `https://noaa-nam-pds.s3.amazonaws.com`

**Tested URL:**  
`https://noaa-nam-pds.s3.amazonaws.com/nam.20260723/nam.t12z.awip1200.tm00.grib2`

**Access Status:** ✅ **200 OK**  
- Last-Modified: Thu, 23 Jul 2026 13:39:16 GMT  
- No authentication required  
- Public access confirmed

**Downloaded File:** `nam-sample.grib2` (28 MB)

---

## File Characteristics

### HRRR Sample File
**File:** `hrrr.t12z.wrfsfcf00.grib2` (Analysis f00)

| Attribute | Value |
|-----------|-------|
| **File Size** | 134 MB |
| **Total Messages** | 170 |
| **Messages with PDT 4.0** | 143 (84%) |
| **Messages with PDT 4.8** | 27 (16%) |
| **Model Run** | 2026-07-23 12z |
| **Forecast Hour** | f00 (analysis) |
| **Grid Type** | Lambert Conformal Conic (GDT 3.30) |

**PDT 4.8 Message Examples:**
- MAXUVV (Maximum UV wind velocity): 100-1000 mb above ground, 0-0 day max fcst
- MAXDVV (Maximum DV wind velocity): 100-1000 mb above ground, 0-0 day max fcst  
- DZDT (Vertical velocity): 0.5-0.8 sigma layer, 0-0 day average fcst
- MAXREF (Maximum reflectivity): 1000 m above ground, 0-0 day max fcst

---

### NAM Sample File
**File:** `nam.t12z.awip1200.tm00.grib2` (Analysis 00)

| Attribute | Value |
|-----------|-------|
| **File Size** | 28 MB |
| **Total Messages** | 181 |
| **Messages with PDT 4.0** | 173 (96%) |
| **Messages with PDT 4.8** | 8 (4%) |
| **Model Run** | 2026-07-23 12z |
| **Forecast Hour** | 00 (analysis) |
| **Product** | AWIPS Grid 218 (CONUS 12km) |

**PDT 4.8 Message Examples:**
- APCP (Total precipitation): surface, 0-0 day accumulated fcst
- ACPCP (Convective precipitation): surface, 0-0 day accumulated fcst
- WEASD (Water equivalent snow depth): surface, 0-0 day accumulated fcst
- SNOM (Snow melt): surface, 0-0 day accumulated fcst
- SSRUN (Surface runoff): surface, 0-0 day accumulated fcst

---

## Product Definition Template Analysis

### PDT 4.0 (Analysis/Forecast)
**Purpose:** Standard meteorological analysis and forecast products at horizontal levels

**Characteristics:**
- Used for most surface and upper-air parameters
- Temperature, wind, pressure, humidity fields
- Single-time analysis or forecast products

**Usage in Sample Files:**
- HRRR: 143/170 messages (84%)
- NAM: 173/181 messages (96%)

---

### PDT 4.8 (Temporal Statistical Products)
**Purpose:** Temporal aggregates and statistical products

**Characteristics:**
- Maximum/minimum values over time periods
- Accumulated quantities (precipitation, runoff)
- Temporal averages
- Used for derived products that combine multiple time steps

**Statistical Process Types Found:**
- **Maximum (max fcst):** Peak values over forecast period
- **Average (ave fcst):** Temporal mean over period  
- **Accumulation (acc fcst):** Cumulative totals over period

**Usage in Sample Files:**
- HRRR: 27/170 messages (16%)
- NAM: 8/181 messages (4%)

---

## Verification Tools Used

```bash
# URL Accessibility Testing
curl -sI "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2"
curl -sI "https://noaa-nam-pds.s3.amazonaws.com/nam.20260723/nam.t12z.awip1200.tm00.grib2"

# File Download
curl -s "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2" -o hrrr-sample.grib2
curl -s "https://noaa-nam-pds.s3.amazonaws.com/nam.20260723/nam.t12z.awip1200.tm00.grib2" -o nam-sample.grib2

# PDT Analysis
wgrib2 hrrr-sample.grib2 -pdt          # List all PDT values
wgrib2 hrrr-sample.grib2 -pdt -match ':pdt=8:' -s   # Get PDT 4.8 details
wgrib2 nam-sample.grib2 -pdt         # List all PDT values  
wgrib2 nam-sample.grib2 -pdt -match ':pdt=8:' -s    # Get PDT 4.8 details
```

---

## Conclusions

1. **✅ URL Accessibility:** Both HRRR and NAM archives are fully accessible via HTTPS without authentication
2. **✅ PDT 4.8 Presence:** Both sample files contain Product Definition Template 4.8 messages
3. **⚠️ PDT 4.1 Absence:** Neither file contains PDT 4.1 (this template may be for different product types)
4. **✅ File Characteristics:** Both files have reasonable sizes and expected GRIB2 structure
5. **✅ Documentation:** URLs and file characteristics are documented in this report

**Notes:**
- PDT 4.1 appears to be used for different product types not present in these surface/analysis files
- PDT 4.8 is used for temporal statistical products (max/min/accumulation/average)
- Both archives are reliable sources for GRIB2 files with complex packing (DRT=3) and PDT 4.8 products

---

*Verification completed on 2026-07-23 for bead bf-3ydrw*
