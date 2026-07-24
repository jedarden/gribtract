# Research: GFS Gaussian-grid GDT 3.40 File from NOAA Archives

**Task:** bf-2nx52  
**Date:** 2026-07-24  
**Purpose:** Research and identify a specific GFS Gaussian-grid GRIB2 file from NOAA public archives that uses GDT 3.40

## Executive Summary

✅ **GFS T574 Gaussian grid files documented and URL pattern identified**  
⚠️ **Public accessibility restricted - NOMADS surface flux directory returns 403 Forbidden**  
✅ **CORe alternative provides publicly accessible GDT 3.40 files**

---

## Target File Specification

### GFS T574 Gaussian Grid Surface Flux Files

**File Naming Convention:**
```
gfs.tCCz.sfluxgrbfFFF.grib2
```

**Components:**
- `CC` = Model cycle runtime (00, 06, 12, or 18 UTC)
- `FFF` = Forecast hour (000-384)
- `sfluxgrb` = Surface flux data
- `.grib2` = GRIB2 format

**Technical Specifications:**
- **Grid:** T574 Gaussian grid (~0.3125° effective resolution)
- **Grid Definition Template:** GDT 3.40 (Gaussian Latitude/Longitude)
- **Format:** GRIB2
- **Coverage:** Global
- **Forecast Range:** 0-384 hours (16 days)
- **Data:** Surface flux variables (radiative, heat fluxes, land surface, soil conditions)

### Theoretical Archive URL Pattern

**NOMADS HTTPS:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDD/CC/surface/gfs.tCCz.sfluxgrbfFFF.grib2
```

**Example URLs:**
- `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/surface/gfs.t00z.sfluxgrbf000.grib2`
- `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260723/00/surface/gdas.t00z.sfluxgrbf000.grib2`

**FTP Access:**
```
ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.YYYYMMDD/CC/surface/gdas.tCCz.sfluxgrbfFFF.grib2
```

---

## Accessibility Testing Results

### NOMADS HTTPS Access Tests

**Tested Dates:** 2026-07-20, 2026-07-21, 2026-07-22, 2026-07-23, 2026-07-24  
**Result:** **HTTP 403 Forbidden** for all tested dates

```bash
$ curl -sI "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/surface/gdas.t00z.sfluxgrbf000.grib2"
HTTP/2 403 
server: Apache
x-frame-options: SAMEORIGIN
```

**Conclusion:** GFS/GDAS T574 surface flux files are **not publicly accessible** through the NOMADS HTTPS interface. The `/surface/` directory appears to have access restrictions.

### FTP Access Tests

**Result:** Timeout/no response when attempting FTP access  
**Conclusion:** FTP access may be restricted or very slow for these files

---

## GDAS vs GFS Clarification

**GDAS (Global Data Assimilation System):**
- Provides the analysis/initial conditions for GFS forecasts
- Uses T574 Gaussian grid for surface flux files
- Technically part of the GFS system but operationally distinct

**GFS (Global Forecast System):**
- Operational forecast model
- Uses GDAS analysis as initial conditions
- Produces forecast outputs on various grids

For the purpose of this task, GDAS files are acceptable as they are:
1. Part of the GFS system
2. Use the same T574 Gaussian grid specification
3. Should use GDT 3.40 (though access restrictions prevented verification)

---

## Publicly Accessible Alternative: CORe

While researching restricted GFS files, a publicly accessible alternative using GDT 3.40 was identified:

**CORe Archive URL:**
```
https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb
```

**Verification:**
```bash
$ curl -s "https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb" | wgrib2 -grid -
grid_template=40:winds(N/S):
Gaussian grid: (512 x 256)
number of latitudes between pole-equator=128
#points=131072
lat 89.462947 to -89.462947
lon 0.000000 to 359.296875 by 0.703125
```

**Characteristics:**
- **GDT:** 3.40 (Gaussian Latitude/Longitude) ✓ VERIFIED
- **Grid:** 512 x 256 Gaussian grid
- **Resolution:** ~0.70° effective resolution
- **Format:** GRIB2
- **Accessibility:** Publicly accessible (no authentication required)
- **File Size:** ~10.5 MB

**Note:** CORe is an atmospheric reanalysis product, not operational GFS, but demonstrates that GDT 3.40 Gaussian grid files are publicly available from NOAA.

---

## GDT 3.40 Documentation

**Grid Definition Template 3.40:** Gaussian Latitude/Longitude

**Official Specification:**
- [NCEP GDT 3.40 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)

**Characteristics:**
- Gaussian latitude spacing (reduces pole singularities)
- Regular longitude spacing  
- Used in spectral models like GFS
- More efficient than regular lat-lon for spectral transforms

---

## Sources

### Primary Sources
- [NCEP Data Products GFS and GDAS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/) - Official product inventory
- [GFS Virtual Lab](https://vlab.noaa.gov/web/gfs) - Model resolution specifications
- [NCEI GFS Product Page](https://www.ncei.noaa.gov/products/weather-climate-models/global-forecast) - Official archive information
- [GDT 3.40 Specification](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml) - Grid definition template documentation

### Alternative Access
- [CORe Archive Documentation](https://ftp.cpc.ncep.noaa.gov/CORe/get_core/get_core.txt) - CORe data access
- [GribStream GDAS](https://gribstream.com/blog/gdas-global-analysis-fields-now-available) - GDAS API access (0.25° regular grid, not Gaussian)

### Historical Archives
- [UCAR NCEP GFS Historical Archive](https://data.ucar.edu/dataset/ncep-gfs-0-25-degree-global-forecast-grids-historical-archive) - 0.25° regular grid data

---

## Conclusions

1. **GFS T574 Gaussian grid files exist** and are well-documented in NCEP product specifications
2. **URL pattern identified:** Follows predictable NOMADS directory structure
3. **Access restrictions:** Surface flux files on NOMADS return HTTP 403 Forbidden
4. **GDT 3.40 format expected:** T574 grid should use GDT 3.40, but access restrictions prevented direct verification
5. **Alternative available:** CORe provides publicly accessible GDT 3.40 files, though as reanalysis rather than operational GFS

### Documented File Specification

**Model:** GFS/GDAS T574 Gaussian Grid  
**Product:** Surface Flux (sfluxgrb)  
**URL Pattern:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDD/CC/surface/gfs.tCCz.sfluxgrbfFFF.grib2`  
**Example:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/surface/gfs.t00z.sfluxgrbf000.grib2`  
**Expected GDT:** 3.40 (Gaussian Latitude/Longitude)  
**Access Status:** Restricted (HTTP 403 Forbidden)  
**Public Alternative:** CORe archive provides verified GDT 3.40 files

---

## Task Status

✅ **GFS Gaussian-grid file specification researched and documented**  
✅ **URL pattern identified from official NOAA documentation**  
⚠️ **Public accessibility restricted through NOMADS**  
✅ **Alternative publicly accessible GDT 3.40 files identified**  
✅ **GDT 3.40 format verified with CORe files**

---

**Generated by:** bf-2nx52  
**Date:** 2026-07-24