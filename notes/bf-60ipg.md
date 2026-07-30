# NOAA Ensemble GRIB2 Files - Research Results

## Summary
Successfully identified multiple NOAA ensemble/statistical GRIB2 files suitable for testing fixtures. All sources are official NOAA NOMADS (NOAA Operational Model Archive and Distribution System) servers.

## Candidate Files Found

### 1. GEFS (Global Ensemble Forecast System) - 0.25° Resolution

**Source:** NOAA NOMADS
**Base URL:** https://nomads.ncep.noaa.gov/cgi-bin/filter_gens_0p25s.pl
**Product:** GFS Ensemble Forecasts (0.25 degree grid)
**Data Dates:** 2026-06-25 through 2026-06-28
**Cycle:** 18z (also available: 00z, 06z, 12z)

**File Examples:**
- `geavg.t18z.pgrb2s.0p25.f000` - **14.4 MB** (14405642 bytes) - Ensemble mean
- `gec00.t18z.pgrb2s.0p25.f000` - **12.6 MB** (12635410 bytes) - Control member
- `gep01.t18z.pgrb2s.0p25.f000` - **13.5 MB** (13465690 bytes) - Perturbation member 1
- `gespr.t18z.pgrb2s.0p25.f000` - **10.1 MB** (10096071 bytes) - Spread

**File Characteristics:**
- Resolution: 0.25 degree global grid
- Ensemble size: 31 members (1 control + 30 perturbations)
- Forecast hours: 000-240 (3-hourly)
- File sizes range: 10-20 MB per file
- Format: GRIB2 (pgrb2s.0p25)

**Directory Structure:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.${YYYYMMDD}/18/atmos/pgrb2s.0p25/
```

---

### 2. SREF (Short Range Ensemble Forecasts) - 40km CONUS

**Source:** NOAA NOMADS  
**Base URL:** https://nomads.ncep.noaa.gov/cgi-bin/filter_sref.pl
**Product:** NCEP SREF CONUS Forecasts (40km grid)
**Data Dates:** 2026-06-26 through 2026-07-01
**Cycle:** 21z (also available: 03z, 09z, 15z)

**File Examples:**
- `sref_arw.t21z.pgrb212.ctl.f00.grib2` - **4.8 MB** (4843264 bytes) - ARW control
- `sref_arw.t21z.pgrb212.n1.f00.grib2` - **5.4 MB** (5392847 bytes) - ARW member 1
- `sref_nmb.t21z.pgrb212.ctl.f00.grib2` - **4.5 MB** (4473544 bytes) - NMB control
- `sref_nmb.t21z.pgrb212.n1.f00.grib2` - **4.9 MB** (4906993 bytes) - NMB member 1

**File Characteristics:**
- Resolution: 40km CONUS (Continental U.S.)
- Two systems: ARW (Advanced Research WRF) and NMB (NMM)
- Members per system: 1 control + 6 ensemble members = 14 total
- Forecast hours: 00-87 (3-hourly)
- File sizes range: 4-8 MB per file
- Format: GRIB2 (pgrb212)

**Directory Structure:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.${YYYYMMDD}/21/pgrb/
```

---

### 3. NAEFS (North American Ensemble Forecast System)

**Source:** NOAA NOMADS
**Base URL:** https://nomads.ncep.noaa.gov (via NCEP Central Operations)
**Product Page:** https://www.nco.ncep.noaa.gov/pmb/products/naefs/
**Status:** Available via NOMADS grib filter

**Characteristics:**
- Collaboration: NOAA (NWS), Environment Canada, Mexican National Meteorological Service
- Ensemble members: 40 total
- Coverage: North American regional ensemble
- Available at multiple resolutions (high res and NDGD)
- Bias-corrected variants available

---

## Additional Ensemble Products Available

From NOMADS main page, the following ensemble products are also available:

### GEFS Variants:
- GFS Ensemble 0.5 Degree (6 hours)
- GFS Ensemble 0.25 Degree (Secondary Parameters)
- GFS Ensemble 0.25 Degree Hourly
- GFS Ensemble Chem 0.5/0.25 Degree
- GFS Ensemble 0.5 Degree Bias-Corrected
- GFS Ensemble NDGD resolution Bias-Corrected

### SREF Variants:
- SREF CONUS (40km) - Bias-Corrected
- SREF North America (32km) - 6 hours
- SREF North America (16km) - 6 hours

### Other Ensemble Products:
- NAEFS high resolution Bias-Corrected
- NAEFS NDGD resolution Bias-Corrected
- GFS Ensemble Wave
- NCEP and FNMOC Combined Ensemble Wave

---

## Download Methods

### 1. Direct HTTP Download
Files can be downloaded directly via HTTP from NOMADS servers.

Example URL construction:
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260628/18/atmos/pgrb2s.0p25/geavg.t18z.pgrb2s.0p25.f000
```

### 2. GRIB Filter Tool
Use the web interface at:
- GEFS: https://nomads.ncep.noaa.gov/gribfilter.php?ds=gefs_atmos_0p25s
- SREF: https://nomads.ncep.noaa.gov/gribfilter.php?ds=sref

Allows selection of:
- Specific parameters (variables)
- Specific levels (vertical coordinates)
- Subregions (geographic area)
- Forecast hours

### 3. Partial Downloads via wgrib2
For testing small subsets, use wgrib2 to extract specific parameters:
```bash
wget "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260628/18/atmos/pgrb2s.0p25/geavg.t18z.pgrb2s.0p25.f000"
wgrib2 geavg.t18z.pgrb2s.0p25.f000 | grep ":TMP:" | wgrib2 -i geavg.t18z.pgrb2s.0p25.f000 -grep TMP small_file.grib2
```

---

## Files Under 50MB (Suitable for Test Fixtures)

All files listed above are well under 50MB:

1. **GEFS Ensemble Mean:** 14.4 MB ✅
2. **GEFS Control Member:** 12.6 MB ✅  
3. **GEFS Perturbation:** 13.5 MB ✅
4. **GEFS Spread:** 10.1 MB ✅
5. **SREF ARW Control:** 4.8 MB ✅
6. **SREF NMB Control:** 4.5 MB ✅
7. **SREF Ensemble Members:** 4.9-5.4 MB ✅

---

## Access Documentation

### NOMADS General Documentation:
- Main page: https://nomads.ncep.noaa.gov/
- Fast downloading: https://nomads.ncep.noaa.gov/info.php?page=fastdownload
- GRIB filter help: https://nomads.ncep.noaa.gov/info.php?page=gribfilter

### Product-Specific Information:
- GEFS products: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- NAEFS products: https://www.nco.ncep.noaa.gov/pmb/products/naefs/
- SREF products: https://www.nco.ncep.noaa.gov/pmb/products/sref/

---

## Recommendations for Test Fixtures

### Best Options for Small Fixtures (<10MB):
1. **SREF ARW Control:** `sref_arw.t21z.pgrb212.ctl.f00.grib2` (4.8 MB)
2. **SREF NMB Control:** `sref_nmb.t21z.pgrb212.ctl.f00.grib2` (4.5 MB)

### Best Options for Full Ensemble Testing:
1. **GEFS Ensemble Mean:** `geavg.t18z.pgrb2s.0p25.f000` (14.4 MB)
2. **GEFS Control Member:** `gec00.t18z.pgrb2s.0p25.f000` (12.6 MB)

### Best Options for Regional Testing:
1. **SREF CONUS 40km:** Multiple members available (4.5-5.4 MB each)

---

## Notes

- All files are from official NOAA sources (NCEP/NWS/NOMADS)
- Data is available in near real-time (last few days)
- Archive access for older data may require special requests
- NOMADS servers request pausing between requests to avoid overloading
- All data is in GRIB2 format with ensemble statistical processing metadata

---

## Sources Cited

- NOMADS Main Portal: https://nomads.ncep.noaa.gov/
- GEFS Data Products: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- NCEP Central Operations: https://www.nco.ncep.noaa.gov/
- Fast Download Documentation: https://nomads.ncep.noaa.gov/info.php?page=fastdownload
