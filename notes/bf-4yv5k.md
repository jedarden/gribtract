# NOAA GRIB2 Archive Structure Research

**Bead:** bf-4yv5k  
**Date:** 2026-07-24

## Overview

This document summarizes the main NOAA public archives that host GRIB2 files with CONUS (Continental United States) coverage, including directory structures, naming conventions, and access patterns.

---

## Primary NOAA Public Archives

### 1. NOMADS (NOAA Operational Model Archive and Distribution System)

**Base URL:** https://nomads.ncep.noaa.gov/  
**Documentation:** https://nomads.ncep.noaa.gov/info.php?page=fastdownload

NOMADS is the primary portal for real-time and archived NCEP model data. All models below are available through the base directory pattern: `/pub/data/nccf/com/{model}/prod`

#### CONUS-Specific Models on NOMADS

| Model | Resolution | Frequency | GRIB Filter | HTTPS Directory |
|-------|------------|-----------|-------------|-----------------|
| **NAM CONUS (12km)** | 12 km | 6 hours | `nam` | `/pub/data/nccf/com/nam/prod` |
| **NAM NEST CONUS** | Higher res | 6 hours | `nam_conusnest` | `/pub/data/nccf/com/nam/prod` |
| **HIRESW CONUS** | High-res | 00Z, 12Z | `hiresconus` | `/pub/data/nccf/com/hiresw/prod` |
| **HREF CONUS** | 40km | 00Z, 12Z | `hrefconus` | `/pub/data/nccf/com/href/prod` |
| **HRRR** | 3 km | Hourly | `hrrr_2d` | `/pub/data/nccf/com/hrrr/prod` |
| **HRRR Sub-hourly** | 3 km | Hourly | `hrrr_sub` | `/pub/data/nccf/com/hrrr/prod` |
| **RAP** | 13 km | Hourly | `rap` | `/pub/data/nccf/com/rap/prod` |
| **SREF CONUS** | 40km | 6 hours | `sref` | `/pub/data/nccf/com/sref/prod` |
| **National Blend of Models** | Variable | Hourly | `blend` | `/pub/data/nccf/com/blend/prod` |
| **RTMA2.5 CONUS** | 2.5 km | Hourly | `rtma2p5` | `/pub/data/nccf/com/rtma/prod` |

#### Global Models with CONUS Coverage

| Model | Resolution | Frequency | GRIB Filter | HTTPS Directory |
|-------|------------|-----------|-------------|-----------------|
| **GFS 0.25°** | 0.25° global | 6 hours | `gfs_0p25` | `/pub/data/nccf/com/gfs/prod` |
| **GFS 0.50°** | 0.50° global | 6 hours | `gfs_0p50` | `/pub/data/nccf/com/gfs/prod` |
| **GDAS 0.25°** | 0.25° global | 6 hours | `gdas_0p25` | `/pub/data/nccf/com/gfs/prod` |

**Access Pattern:** Data can be accessed through "grib filter" links or direct HTTPS downloads. Index files with wgrib inventory are available for each dataset, enabling random-access HTTP downloads of specific records.

---

### 2. NOAA READY Gridded Data Archives

**Base URL:** https://www.ready.noaa.gov/archives.php  
**FTP:** ftp.arl.noaa.gov (anonymous, limit 2 connections)  
**AWS S3:** s3://noaa-oar-arl-hysplit-pds/

READY provides historical model data archives with CONUS coverage, processed and stored using a 1-byte packing algorithm.

#### Available CONUS Archives

| Dataset | Resolution | Coverage | Period | Access Paths |
|---------|------------|----------|--------|--------------|
| **NAM 12km** | 12 km | CONUS | May 2007 - present | `s3://noaa-oar-arl-hysplit-pds/nam12/` |
| **NAMS Hybrid** | Variable | CONUS, AK, HI | 2010 - present | `s3://noaa-oar-arl-hysplit-pds/nams/` |
| **HRRR 3km** | 3 km | CONUS | June 2019 - present | `s3://noaa-oar-arl-hysplit-pds/hrrr/` |

**Directory Structure:**
- Cloud: `s3://noaa-oar-arl-hysplit-pds/{dataset}/{year}/{month}/`
- FTP: `ftp.arl.noaa.gov:/archives/{dataset}/`
- Web: `https://www.ready.noaa.gov/data/archives/{dataset}/`

**Data Verification:** Each dataset directory contains `listing.md5.txt` with MD5 checksums for file integrity verification.

**Legacy Archives (No Longer Updated):**
- **HRRR v1** (2015-2019): `s3://noaa-oar-arl-hysplit-pds/hrrr.v1/`
- **NAM EDAS 40km** (2004-2018): `s3://noaa-oar-arl-hysplit-pds/edas40/`
- **NAM EDAS 80km** (1997-2004): `s3://noaa-oar-arl-hysplit-pds/edas/`

---

### 3. NOAA Public AWS S3 Buckets

#### GFS (Global Forecast System) Big Data Project

**Bucket:** `noaa-gfs-bdp-pds`  
**Registry:** https://registry.opendata.aws/noaa-gfs-bdp-pds/  
**Size:** ~30 PiB

**Directory Structure:** 
```
gfs.YYYYMMDD/
├── HH/                    # Model cycle (00, 06, 12, 18)
│   ├── atmos/
│   │   ├── pgrb2.0p25/   # 0.25° resolution
│   │   ├── pgrb2.0p50/   # 0.50° resolution
│   │   └── pgrb2.1p00/   # 1.0° resolution
```

**File Naming Convention:** `gfs.tCCz.pgrb2.0p25.fFFF`
- `CC` = model cycle runtime (00, 06, 12, 18)
- `FFF` = forecast hour (000-384)
- `YYYYMMDD` = date

#### GEFS (Global Ensemble Forecast System)

**Bucket:** `noaa-gefs-pds`  
**S3 URL:** https://noaa-gefs-pds.s3.amazonaws.com

**Example Path:**
```
gefs.20230104/12/atmos/pgrb2ap5/geavg.t12z.pgrb2a.0p50.f000
```

**Pattern:** `gefs.YYYYMMDD/HH/atmos/pgrb2{product}/`
- `YYYYMMDD` = date directory
- `HH` = forecast cycle hour
- `atmos/` = atmospheric data
- `pgrb2ap5/` = GRIB2 product type
- `geavg.tHHz.pgrb2a.0p50.fFFF` = file (tHHz = analysis time, fFFF = forecast hour)

#### NBM (National Blend of Models)

**Bucket:** `noaa-nbm-grib2-pds`  
**Explorer:** https://noaa-nbm-grib2-pds.s3.amazonaws.com/index.html

Contains National Blend of Models GRIB2 data with CONUS coverage.

---

### 4. NCEP Central Operations (NCO) Documentation

**Base URL:** https://www.nco.ncep.noaa.gov/  
**GRIB2 Docs:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/

#### GFS Product Inventory

**Documentation:** https://www.nco.ncep.noaa.gov/pmb/products/gfs/

**Directory Base:**
- FTP: `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod`
- HTTPS: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod`

**Resolution Variants:**

| Type | Pattern | Description |
|------|---------|-------------|
| Common parameters | `gfs.tCCz.pgrb2.0p25.fFFF` | 0.25° resolution |
| Common parameters | `gfs.tCCz.pgrb2.0p50.fFFF` | 0.50° resolution |
| Common parameters | `gfs.tCCz.pgrb2.1p00.fFFF` | 1.0° resolution |
| Less common | `gfs.tCCz.pgrb2b.0p25.fFFF` | 0.25° (secondary) |
| Less common | `gfs.tCCz.pgrb2b.0p50.fFFF` | 0.50° (secondary) |
| Combined | `gfs.tCCz.pgrb2full.0p50.fFFF` | Concat of pgrb2.0p50 + pgrb2b.0p50 |
| Specialized | `gfs.tCCz.sfluxgrbfFFF.grib2` | T1534 Semi-Lagrangian |

---

### 5. NCAR Research Data Archive

**URL:** https://data.ucar.edu/dataset/ncep-gfs-0-25-degree-global-forecast-grids-historical-archive

Hosts historical GFS data at 0.25 degree resolution. Comprehensive archive of global forecast grids with CONUS coverage.

---

## Index Files and Access Patterns

### wgrib2 Inventory Format

**Documentation:** 
- https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/
- https://wgrib2-docs.readthedocs.io/
- https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/grb2_mk_inv.html

**Key Points:**
- Each GRIB2 field has one line in the inventory
- Inventory format: one line per gridded field
- Enables random access to specific records without reading entire files
- Index files are typically named `{filename}.idx` or `{filename}.inv`
- Tools like `curl` with HTTP range requests support filtered downloads

**NOMADS Fast Download:**
- Uses index files with wgrib inventory
- Requires HTTP program supporting random access (like cURL)
- Documentation: https://nomads.ncep.noaa.gov/info.php?page=fastdownload

---

## GRIB2 Grid Definition Templates

**Primary Reference:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/

**Key Tables:**
- **Table 3.0** - Source of Grid Definition
- **Table 3.1** - Grid Definition Template Number: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
- **Template 3.1** - Rotated Latitude-Longitude Grid: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-0.shtml

Grid Definition Templates are specified in **Section 3 (Grid Definition Section)** of GRIB2 messages.

---

## Models Most Likely to Contain DRT=0 with CONUS Coverage

Based on the research, the following models are most likely to contain GRIB2 files with DRT=0 (Data Representation Template 0 - simple packing) and CONUS coverage:

1. **GFS 0.25° and 0.50°** - Global model includes CONUS, multiple resolution options
2. **NAM CONUS (12km)** - Explicit CONUS coverage at 12km resolution
3. **HRRR** - 3km CONUS coverage, hourly updates
4. **RAP** - 13km CONUS coverage, hourly updates
5. **NBM (National Blend of Models)** - Blended product with CONUS coverage
6. **HIRESW CONUS** - High-resolution CONUS nest

---

## Access Recommendations

### For Real-Time/Near-Real-Time Data:
1. **NOMADS** - Best for current and recent data with GRIB filter capabilities
2. **NCEP HTTPS servers** - Direct download with index files

### For Historical Data:
1. **NOAA READY Archives** - Processed historical data from 2010 onwards
2. **AWS S3 Buckets** - Mass storage with no authentication required
3. **NCAR Research Data Archive** - Comprehensive historical GFS archive

### For Batch Downloads:
1. Use **AWS S3** for high-throughput, no-rate-limit access
2. Use **FTP** (READY archives) for traditional batch access
3. Use **HTTPS with index files** for selective record retrieval

---

## Sources

- [NOMADS](https://nomads.ncep.noaa.gov/)
- [NOMADS Fast Download Documentation](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [NOAA READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NCEP Data Products GFS and GDAS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [Registry of Open Data on AWS - NOAA](https://registry.opendata.aws/collab/noaa/)
- [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
- [Herbie GEFS Documentation](https://herbie.readthedocs.io/en/latest/gallery/noaa_models/gefs.html)
- [NCAR GFS Historical Archive](https://data.ucar.edu/dataset/ncep-gfs-0-25-degree-global-forecast-grids-historical-archive)
