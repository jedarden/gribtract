# DRT=0 Packing Format Identification - NOAA CONUS Files

**Bead ID:** bf-24ma0  
**Date:** 2026-07-24  
**Purpose:** Identify which specific NOAA CONUS GRIB2 files use DRT=0 (simple packing) versus complex packing

## Executive Summary

✅ **CONFIRMED:** Only analysis products (RTMA/URMA) use pure DRT=0 simple packing  
❌ **FORECAST MODELS:** All operational forecast models use complex packing (DRT=5.3 or DRT=5.40)

**Key Finding:** NOAA's DRT=0 files are **exclusively found in analysis products**, not forecast model output.

---

## Dataset → DRT=0 File Availability Mapping

| Dataset | DRT=0 Available | Pure DRT=0 | DRT Value Mix | Resolution | Archive |
|---------|----------------|------------|---------------|------------|---------|
| **RTMA 2.5 CONUS** | ✅ YES | ✅ 100% | 13/13 DRT=5.0 | 2.5 km | NOMADS |
| **URMA 2.5 CONUS** | ✅ YES | ✅ 100% | 14/14 DRT=5.0 | 2.5 km | NOMADS |
| **HRRR CONUS** | ⚠️ PARTIAL | ❌ 19% | 32/170 DRT=5.0, 138/170 DRT=5.3 | 3 km | AWS S3, NOMADS |
| **GFS (global)** | ❌ NO | ❌ 0.14% | 1/696 DRT=5.0, 695/696 DRT=5.3 | 0.25°-1.00° | AWS S3, NOMADS |
| **GEFS (ensemble)** | ❌ NO | ❌ 0% | 0/71-85 DRT=5.0, rest DRT=5.3 | 0.50° | AWS S3 |
| **NAM CONUS** | ❌ NO | ❌ 0% | 860/860 DRT=5.3 | 5 km nest | NOMADS |
| **RAP CONUS** | ❌ NO | ❌ 0% | 355/355 DRT=5.40 | 13 km | NOMADS |

---

## Pure DRT=0 Files (Analysis Products)

### 1. RTMA 2.5 CONUS (Real-Time Mesoscale Analysis) ⭐ BEST FOR DRT=0

**Specifications:**
- **Packing:** 100% DRT=5.0 (Simple Packing) - VERIFIED
- **Grid:** 2.5 km Lambert conformal CONUS
- **Dimensions:** 2345 × 1597 points (3.7 million grid cells)
- **Records:** 13 analysis fields
- **File Size:** 80-83 MB
- **Update Frequency:** Hourly analyses

**File Pattern:**
```
rtma2p5.tHHz.2dvaranl_ndfd.grb2_wexp
```

**Download URL Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260723/rtma2p5.t12z.2dvaranl_ndfd.grb2_wexp
```

**Verification:**
```bash
wgrib2 rtma2p5.t12z.2dvaranl_ndfd.grb2_wexp -Sec5 | grep "Data Repr. Template"
# Result: All 13 records show "Data Repr. Template=5.0"
```

---

### 2. URMA 2.5 CONUS (Unrestricted Mesoscale Analysis)

**Specifications:**
- **Packing:** 100% DRT=5.0 (Simple Packing) - VERIFIED
- **Grid:** 2.5 km Lambert conformal CONUS (Grid 184)
- **Dimensions:** Similar to RTMA 2.5
- **Records:** 14 analysis fields
- **File Size:** 82-86 MB
- **Update Frequency:** Hourly analyses

**File Pattern:**
```
urma2p5.tHHz.2dvaranl_ndfd.grb2_wexp
```

**Download URL Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/urma/prod/urma2p5.20260724/urma2p5.t00z.2dvaranl_ndfd.grb2_wexp
```

**Verification:**
```bash
wgrib2 urma2p5.t00z.2dvaranl_ndfd.grb2_wexp -Sec5 | grep "Data Repr. Template"
# Result: All 14 records show "Data Repr. Template=5.0"
```

---

## Partial DRT=0 Files (Forecast Models)

### 3. HRRR CONUS (High-Resolution Rapid Refresh) ⚠️ MIXED PACKING

**Specifications:**
- **Packing:** MIXED - 32/170 DRT=5.0 (19%), 138/170 DRT=5.3 (81%)
- **Grid:** 3 km CONUS
- **Records:** 170 total fields per file
- **Update Frequency:** Hourly

**File Pattern:**
```
hrrr.tCCz.wrfsfcfFF.grib2 (surface)
hrrr.tCCz.wrfprsFF.grib2 (pressure)
```

**Download URL Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.20260724/conus/hrrr.t00z.wrfsfcf00.grib2
```

**NOT Suitable** if pure DRT=0 requirement exists - 81% of records use complex packing.

---

## Complex Packing Only (No DRT=0)

### 4. GFS (Global Forecast System) ❌ COMPLEX PACKING

**Specifications:**
- **Packing:** 99.86% DRT=5.3 (Complex + Spatial Differencing)
- **Records:** 696 total, only 1 DRT=5.0 (CLMR:50 mb - climatological moisture)
- **Resolutions:** 0.25°, 0.50°, 1.00° global
- **Coverage:** Global (includes CONUS but not CONUS-specific)

**File Patterns:**
```
gfs.tCCz.pgrb2.0p25.fFFF (0.25°)
gfs.tCCz.pgrb2.0p50.fFFF (0.50°)
gfs.tCCz.pgrb2.1p00.fFFF (1.00°)
```

**NOT Suitable** for DRT=0 requirements - requires complex packing decoder.

---

### 5. GEFS (Global Ensemble Forecast System) ❌ COMPLEX PACKING

**Specifications:**
- **Packing:** 100% DRT=5.3 (Complex + Spatial Differencing)
- **Records:** 71-85 total, 0 DRT=5.0 records
- **Resolution:** 0.50° global
- **Type:** Ensemble mean products

**File Pattern:**
```
geavg.tCCz.pgrb2a.0p50.fFFF (ensemble mean)
```

**NOT Suitable** for DRT=0 requirements.

---

### 6. NAM CONUS (North American Mesoscale) ❌ COMPLEX PACKING

**Specifications:**
- **Packing:** 100% DRT=5.3 (Complex + Spatial Differencing)
- **Records:** 860 total, 0 DRT=5.0 records
- **Grid:** 5 km CONUS nest
- **File Size:** 886 MB

**File Pattern:**
```
nam.tCCz.conusnest.hiresfFF.tm00.grib2
```

**Download URL Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20260723/nam.t12z.conusnest.hiresf00.tm00.grib2
```

**NOT Suitable** for DRT=0 requirements.

---

### 7. RAP CONUS (Rapid Refresh) ❌ COMPLEX PACKING

**Specifications:**
- **Packing:** 100% DRT=5.40 (Complex + JPEG Compression)
- **Records:** 355 total, 0 DRT=5.0 records
- **Grid:** 13 km CONUS
- **File Size:** 18 MB

**File Pattern:**
```
rap.tCCz.awp130pgrbfFF.grib2 (CONUS 13km)
```

**Download URL Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/prod/rap.20260724/rap.t00z.awp130pgrbf00.grib2
```

**NOT Suitable** for DRT=0 requirements - requires JPEG decoder in addition to complex packing.

---

## Technical Details

### Data Representation Template (DRT) Types

| DRT Value | Template Name | Description | Decoder Complexity |
|-----------|---------------|-------------|-------------------|
| **5.0** | Simple Packing | Basic packing without spatial differencing | ⭐ LOW - Easiest |
| 5.2 | Complex Packing | Complex packing only | Medium |
| **5.3** | Complex + Spatial Differencing | Complex packing with spatial differencing | HIGH - Requires decode |
| 5.40 | Complex + JPEG | Complex packing with JPEG compression | VERY HIGH - JPEG + decode |

### wgrib2 Verification Commands

**Check all DRT values:**
```bash
wgrib2 <file.grib2> -Sec5 | grep "Data Repr. Template"
```

**Count DRT types:**
```bash
wgrib2 <file.grib2> -Sec5 | grep -o "Data Repr. Template=5\.[0-9]*" | sort | uniq -c
```

**Full packing analysis:**
```bash
wgrib2 <file.grib2> -packing
```

---

## Acceptance Criteria Met

✅ **Test candidate files from each cataloged archive** - Tested 7 datasets (RTMA, URMA, HRRR, GFS, GEFS, NAM, RAP)  
✅ **Document which datasets/products offer DRT=0 variants** - 2 pure DRT=0, 1 partial DRT=0, 4 complex only  
✅ **Identify specific file names/paths that use DRT=0** - Documented URL patterns and examples for RTMA/URMA  
✅ **Create mapping of dataset → DRT=0 file availability** - Complete mapping table above  

---

## Recommendations

### For Pure DRT=0 Requirements:
1. **Use RTMA 2.5 CONUS** - Best option, 100% DRT=0, hourly updates
2. **Use URMA 2.5 CONUS** - Alternative, 100% DRT=0, unrestricted analysis

### For Complex Packing Tolerance:
1. **Implement DRT=5.3 decoder** - Required for 99.86% of forecast model data
2. **Implement JPEG decoder** - Required for RAP (DRT=5.40)
3. **HRRR as compromise** - 19% DRT=0, but mostly complex packing

---

## Sources and Verification

**Verification Method:** wgrib2 v3.1.3 with `-Sec5` and `-packing` analysis  
**Archives Tested:**
- NCEP NOMADS (https://nomads.ncep.noaa.gov/)
- NOAA AWS S3 (s3://noaa-gfs-bdp-pds, s3://noaa-hrrr-bdp-pds)
- NOAA GEFS AWS (s3://noaa-gefs-pds)

**Related Documentation:**
- Catalog of NOAA CONUS archives (notes/bf-3b63y.md)
- Previous DRT=0 file search (notes/bf-3s515.md)
- DRT verification discrepancy (notes/DRT0_VERIFICATION_RESULTS.md)

---

**Conclusion:** Only NOAA analysis products (RTMA/URMA) provide pure DRT=0 simple packing for CONUS coverage. All operational forecast models use complex packing requiring additional decoding steps.
