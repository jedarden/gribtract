# Research: Gaussian-grid fixture URLs from NOAA

## ✓ TASK COMPLETED

Successfully identified and verified public NOAA archive URLs for real Gaussian-grid GRIB2 files with GDT 3.40.

### Primary Verified URL

**CORe Archive (NOAA Open Data Dissemination - Google Cloud Storage):**
- **Verified URL:** `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
- **File size:** ~10.5 MB
- **GDT:** 3.40 (Gaussian Latitude/Longitude grid) - VERIFIED with wgrib2
- **Grid:** 512 x 256 Gaussian grid, 131,072 points
- **Accessibility:** Publicly accessible, no authentication required

This URL meets all acceptance criteria:
- ✓ Identified public NOAA URL for Gaussian-grid (GDT 3.40) GRIB2 file
- ✓ URL is accessible and points to a real archived file
- ✓ File characteristics documented (size, grid type, GDT)

---

## Task Summary

Research and identify public NOAA archive URLs for real Gaussian-grid files with GDT 3.40 (Grid Definition Template 3.40 for Gaussian Latitude/Longitude grids).

## Key Findings

### 1. NOAA CORe Archive (NOAA Open Data Dissemination)

**Location:** Google Cloud Storage bucket `noaa-nws-ncep-core`

**Documentation:** https://ftp.cpc.ncep.noaa.gov/CORe/get_core/get_core.txt

**URL Pattern:**
```
https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/YYYY/MM/flx.YYYYMMDDHH.grb
https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/pgb/YYYY/MM/pgb.YYYYMMDDHH.grb
```

**Example URLs:**
- Flux file (3-hourly): `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
- Analysis file (3-hourly): `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/pgb/2024/01/pgb.2024011500.grb`

**File Characteristics:**
- Format: GRIB2
- File size: ~10-11 MB for 3-hourly files
- Index files available (`.idx`) with field inventory
- Verified accessible: Yes

**File Types:**
- `pgb`: Atmospheric analyses created using Unified Post Processor (UPP)
- `flx`: Flux files directly created by FV3GFS model (radiative, heat fluxes, land surface, soil conditions, cloud layers)

**Coverage:**
- Temporal: 1950-present
- Temporal resolution: 3-hourly, daily, monthly means available

**VERIFIED Grid Definition:**
- **GDT: 3.40 (Gaussian Latitude/Longitude grid) - CONFIRMED**
- Grid size: 512 x 256
- Number of latitudes between pole-equator: 128
- Total points: 131,072
- Latitude range: 89.46°N to -89.46°S
- Longitude range: 0° to 359.30°
- Longitude increment: ~0.703°

---

### 2. NCEP GDAS Surface Flux T574 Gaussian Grid

**Source:** NCEP Central Operations (NCO)

**Documentation:** https://www.nco.ncep.noaa.gov/pmb/products/gfs/

**File Naming Convention:**
```
gdas.tCCz.sfluxgrbfFFF.grib2
```
Where:
- CC = cycle time (00, 06, 12, 18)
- FFF = forecast hour (000-009)

**Directory Structure (NOMADS):**
```
/pub/data/nccf/com/gfs/prod/gdas.YYYYMMDD/CC/surface/
```

**Example URL Pattern:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20240115/00/surface/gdas.t00z.sfluxgrbf000.grib2
```

**Grid Characteristics:**
- Grid: T574 Gaussian grid (~0.3125° resolution)
- Format: GRIB2
- **Note:** GDT 3.40 expected but not yet verified in this research

**Access:**
- NOMADS: https://nomads.ncep.noaa.gov/
- UCAR GDEX Dataset: https://gdex.ucar.edu/datasets/d084004/

**Note:** NOMADS maintains only recent data; historical data requires alternative archives.

---

### 3. GDAS Pressure Level Data

**File Naming Convention:**
```
gdas.tCCz.pgrb2.0p25.fFFF
```

**Resolution:** 0.25 degree global latitude-longitude grid

**Note:** These use regular lat-lon grids (GDT 0), not Gaussian grids.

---

### 4. NOAA READY HYSPLIT Archives

**S3 Bucket:** `noaa-oar-arl-hysplit-pds`

**Documentation:** https://www.ready.noaa.gov/archives.php

**Note:** READY archives use a custom 1-byte packing format, not native GRIB2. These files do not use standard GRIB2 GDTs.

---

## GDT 3.40 Documentation

**Official Specification:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml

GDT 3.40 is the GRIB2 Grid Definition Template for "Gaussian Latitude/Longitude" grids.

---

## Sources

- [NCEP GFS/GDAS Products](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [CORe Archive Documentation](https://ftp.cpc.ncep.noaa.gov/CORe/get_core/get_core.txt)
- [GRIB2 Table 3.40 - Gaussian Latitude/Longitude](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)
- [NOMADS at NCEP](https://nomads.ncep.noaa.gov/)
- [UCAR GDEX - NCEP GDAS Surface Flux](https://gdex.ucar.edu/datasets/d084004/)
- [NOAA READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)

---

## Verification Results

### ✓ VERIFIED: CORe Archive uses GDT 3.40

**Command used:**
```bash
curl -s "https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb" | wgrib2 -grid -
```

**Output:**
```
grid_template=40:winds(N/S):
Gaussian grid: (512 x 256)
number of latitudes between pole-equator=128
#points=131072
lat 89.462947 to -89.462947
lon 0.000000 to 359.296875 by 0.703125
```

**Conclusion:** CORe files use **GDT 3.40 (Gaussian Latitude/Longitude grid)** as required.

### Additional Verification Needed

- GDAS T574 files also expected to use GDT 3.40 but not yet verified directly
- GDAS files on NOMADS have limited retention; use alternative archives for historical data
