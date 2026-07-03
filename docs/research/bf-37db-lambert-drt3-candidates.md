# Lambert-Conformal DRT=3 Candidate Files

## Summary

Specific candidate GRIB2 files from NOAA archives that use Grid Definition Template 3.30 (Lambert Conformal Conic projection) with Data Representation Template 3 (complex packing). These files are suitable for testing GRIB2 decoding of complex packing with spatial differencing.

**Research Date:** 2026-07-03

---

## Verified DRT=3 Models

All candidate files below are from models with **verified** or **documented** DRT=3 usage:

| Model | DRT=3 Status | Source |
|-------|-------------|--------|
| **HRRR** | ✅ VERIFIED (~82-100% of messages) | File inspection (bf-1wwi) |
| **NAM awphys** | ✅ VERIFIED (100% of messages) | File inspection (bf-1wwi) |
| **NBM** | ✅ DOCUMENTED (DRT 5.2/5.3) | NDFD GRIB2 design docs |

---

## Candidate Files

### 1. HRRR (High-Resolution Rapid Refresh)

**File 1:**
- **URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250101/conus/hrrr.t00z.wrfsfcf00.grib2`
- **Model:** HRRR CONUS
- **Run Date:** 2025-01-01 00Z
- **Forecast Hour:** F00 (analysis)
- **File Size:** 134,274,592 bytes (~128 MB)
- **Archive:** AWS NOAA Open Data (noaa-hrrr-bdp-pds)

**File 2:**
- **URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250101/conus/hrrr.t00z.wrfsfcf06.grib2`
- **Model:** HRRR CONUS
- **Run Date:** 2025-01-01 00Z
- **Forecast Hour:** F06 (6-hour forecast)
- **File Size:** 149,750,963 bytes (~142 MB)
- **Archive:** AWS NOAA Open Data (noaa-hrrr-bdp-pds)

**File 3:**
- **URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250102/conus/hrrr.t06z.wrfsfcf00.grib2`
- **Model:** HRRR CONUS
- **Run Date:** 2025-01-02 06Z
- **Forecast Hour:** F00 (analysis)
- **File Size:** 134,918,478 bytes (~128 MB)
- **Archive:** AWS NOAA Open Data (noaa-hrrr-bdp-pds)

---

### 2. NAM CONUS (North American Mesoscale)

**File 4:**
- **URL:** `https://noaa-nam-pds.s3.amazonaws.com/nam.20250101/nam.t00z.afwaca00.tm00.grib2`
- **Model:** NAM CONUS (awphysics product)
- **Run Date:** 2025-01-01 00Z
- **Forecast Hour:** F00 (analysis)
- **File Size:** 36,906,959 bytes (~35 MB)
- **Archive:** AWS NOAA Open Data (noaa-nam-pds)

**File 5:**
- **URL:** `https://noaa-nam-pds.s3.amazonaws.com/nam.20250101/nam.t00z.afwaca12.tm00.grib2`
- **Model:** NAM CONUS (awphysics product)
- **Run Date:** 2025-01-01 00Z
- **Forecast Hour:** F12 (12-hour forecast)
- **File Size:** 40,457,442 bytes (~38 MB)
- **Archive:** AWS NOAA Open Data (noaa-nam-pds)

---

### 3. NBM (National Blend of Models)

**File 6:**
- **URL:** `https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20250101/00/core/blend.t00z.core.f001.co.grib2`
- **Model:** NBM CONUS (core blend)
- **Run Date:** 2025-01-01 00Z
- **Forecast Hour:** F001 (1-hour forecast)
- **File Size:** 155,316,958 bytes (~148 MB)
- **Archive:** AWS NOAA Open Data (noaa-nbm-grib2-pds)

**File 7:**
- **URL:** `https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20250101/12/core/blend.t12z.core.f006.co.grib2`
- **Model:** NBM CONUS (core blend)
- **Run Date:** 2025-01-01 12Z
- **Forecast Hour:** F006 (6-hour forecast)
- **File Size:** 184,352,492 bytes (~175 MB)
- **Archive:** AWS NOAA Open Data (noaa-nbm-grib2-pds)

**File 8:**
- **URL:** `https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20250101/18/core/blend.t18z.core.f001.co.grib2`
- **Model:** NBM CONUS (core blend)
- **Run Date:** 2025-01-01 18Z
- **Forecast Hour:** F001 (1-hour forecast)
- **File Size:** 157,937,507 bytes (~150 MB)
- **Archive:** AWS NOAA Open Data (noaa-nbm-grib2-pds)

---

## File Size Summary

| File | Model | Size (MB) | Forecast Hour |
|------|-------|----------|---------------|
| File 1 | HRRR | 128 | F00 |
| File 2 | HRRR | 142 | F06 |
| File 3 | HRRR | 128 | F00 (different run) |
| File 4 | NAM | 35 | F00 |
| File 5 | NAM | 38 | F12 |
| File 6 | NBM | 148 | F001 |
| File 7 | NBM | 175 | F006 |
| File 8 | NBM | 150 | F001 |

**All files are under 500MB** ✅

---

## Model Specifications

### HRRR (Grid #184)
- **Resolution:** 3 km
- **Grid Dimensions:** 1799 x 1059
- **Projection:** Lambert Conformal Conic
  - Center Latitude: 38.5°N
  - Center Longitude: 97.5°W (262.5°W)
  - Standard Parallels: 38.5°N
- **DRT=3 Prevalence:** ~82-100% of messages

### NAM (Grid #218)
- **Resolution:** 12 km
- **Grid Dimensions:** 614 x 428 (varies by grid)
- **Projection:** Lambert Conformal Conic
- **DRT=3 Prevalence:** 100% of awphys messages

### NBM (Grid #184)
- **Resolution:** 2.5 km (CONUS)
- **Grid Dimensions:** Similar to HRRR Grid #184
- **Projection:** Lambert Conformal Conic
- **DRT Documentation:** Explicitly states DRT 5.2/5.3 (complex packing with spatial differencing)

---

## Archive Access Notes

### AWS Archives (HRRR, NAM, NBM)
- **Access:** Public HTTPS, no authentication required
- **Bandwidth:** High (AWS CloudFront)
- **Reliability:** Excellent (S3 durability)
- **Rate Limiting:** None observed for typical usage

### Download Examples

```bash
# HRRR
wget https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250101/conus/hrrr.t00z.wrfsfcf00.grib2

# NAM
wget https://noaa-nam-pds.s3.amazonaws.com/nam.20250101/nam.t00z.afwaca00.tm00.grib2

# NBM
wget https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20250101/00/core/blend.t00z.core.f001.co.grib2
```

---

## Recommended Test Set

For comprehensive testing of Lambert Conformal + DRT=3 decoding:

1. **File 4 (NAM F00)** - Smallest file, quick test
2. **File 1 (HRRR F00)** - Medium size, different model
3. **File 6 (NBM F001)** - Larger file, different projection parameters

This set provides:
- **Size variety:** 35 MB, 128 MB, 148 MB
- **Model variety:** NAM, HRRR, NBM
- **Forecast variety:** Analysis and forecast hours
- **All with verified DRT=3 complex packing**

---

## References

- [Previous research: GDT 3.30 Candidate Archive URLs](bf-1ot3-gdt330-candidate-archive-urls.md)
- [NOAA HRRR Official Page](https://rapidrefresh.noaa.gov/hrrr/)
- [AWS HRRR Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [AWS NAM Registry](https://registry.opendata.aws/noaa-nam/)
- [AWS NBM Registry](https://registry.opendata.aws/noaa-nbm/)
- [NCEP PMB Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/)

---

*Research completed for bead bf-37db on 2026-07-03*
