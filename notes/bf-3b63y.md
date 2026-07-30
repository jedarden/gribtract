# NOAA CONUS GRIB2 Dataset Catalog

## Overview

This document catalogs NOAA public archives that serve CONUS-covering GRIB2 files, including archive URLs, dataset naming conventions, and catalog structures.

**Date:** 2026-07-24  
**Purpose:** Identify and document all available NOAA public archives for CONUS GRIB2 data

---

## 1. AWS S3 Public Buckets

### 1.1 NOAA GFS (Global Forecast System)

**Archive:** [NOAA GFS-BDP-PDS](https://registry.opendata.aws/noaa-gfs-bdp-pds/)  
**S3 Bucket:** `s3://noaa-gfs-bdp-pds`  
**Region:** us-east-1  
**Format:** GRIB2

#### Directory Structure
```
s3://noaa-gfs-bdp-pds/
├── gfs.YYYYMMDD/
│   ├── 00/
│   │   ├── gfs.t00z.pgrb2.0p25.f000
│   │   ├── gfs.t00z.pgrb2.0p25.f003
│   │   └── ...
│   ├── 06/
│   ├── 12/
│   └── 18/
└── gdas.YYYYMMDD/
    └── hh-6/
        └── RESTART/
```

#### Naming Convention
- **Pattern:** `gfs.tCCz.pgrb2.0p25.fFFF`
- **Components:**
  - `CC` = Model cycle (00, 06, 12, 18 UTC)
  - `0p25` = 0.25° resolution
  - `FFF` = Forecast hour (000-384)

#### CONUS Coverage
GFS is global but includes CONUS in the full domain.

**Explorer:** https://noaa-gfs-bdp-pds.s3.amazonaws.com/index.html  
**Documentation:** https://www.nco.ncep.noaa.gov/pmb/products/gfs/

---

### 1.2 NOAA HRRR (High-Resolution Rapid Refresh)

**Archive:** [NOAA HRRR-PDS](https://registry.opendata.aws/noaa-hrrr-pds/)  
**S3 Buckets:** 
- `s3://noaa-hrrr-bdp-pds` (us-east-1)
- `s3://hrrrzarr` (us-west-1) - Zarr format

#### Directory Structure
```
s3://noaa-hrrr-bdp-pds/
└── hrrr.YYYYMMDD/
    ├── 00/
    │   ├── hrrr.t00z.wrfsfcf00.grib2
    │   ├── hrrr.t00z.wrfprsf00.grib2
    │   └── ...
    ├── 01/
    ├── 02/
    └── ... (hourly cycles 00-23)
```

#### Naming Convention
- **Pattern:** `hrrr.tCCz.wrfsfcfFF.grib2` (surface) or `hrrr.tCCz.wrfprsFF.grib2` (pressure)
- **Components:**
  - `CC` = Model cycle (00-23 UTC, hourly)
  - `FF` = Forecast hour (00-18)
  - `wrfsfc` = Surface fields
  - `wrfprs` = Pressure fields

#### CONUS Coverage
- **Resolution:** 3-km
- **Domain:** CONUS (cloud-resolving, convection-allowing)
- **Update frequency:** Hourly

**Explorer:** https://noaa-hrrr-bdp-pds.s3.amazonaws.com/index.html

---

### 1.3 NOAA READY Gridded Data Archives

**Archive:** [NOAA READY](https://www.ready.noaa.gov/archives.php)  
**S3 Bucket:** `s3://noaa-oar-arl-hysplit-pds`  
**Maintainer:** NOAA Air Resources Laboratory

#### Directory Structure
```
s3://noaa-oar-arl-hysplit-pds/
├── gfs0p25/
│   ├── 2023/
│   ├── 2024/
│   ├── 2025/
│   └── listing.md5.txt
├── nam12km/
│   ├── 2007/
│   ├── 2008/
│   └── ...
└── hrrr/
    ├── 2019/
    └── ...
```

#### Available CONUS Datasets
- **GFS 0.25°** (June 2019-present)
- **NAM 12 km** (May 2007-present)
- **HRRR 3 km** (June 2019-present)
- **NAMS Hybrid sigma-pressure** (2010-present)

**Format:** 1-byte packing algorithm, GRIB format

---

## 2. NCEP FTP/HTTPS Archives

### 2.1 NCEP GFS/GDAS Data Products

**Primary:** [NCEP Data Products - GFS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)

#### FTP/HTTPS Access
- **FTP:** `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`
- **HTTPS:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`

#### Directory Structure
```
/pub/data/nccf/com/gfs/prod/
├── gfs.YYYYMMDD/
│   ├── 00/
│   │   ├── gfs.t00z.pgrb2.0p25.f000
│   │   ├── gfs.t00z.pgrb2.0p25.f003
│   │   └── ...
│   ├── 06/
│   ├── 12/
│   └── 18/
└── gdas.YYYYMMDD/
    └── hh-6/
        └── RESTART/
```

#### Naming Conventions
- **GFS:** `gfs.tCCz.[type].fFFF`
  - `[type]` = `pgrb2.0p25`, `pgrb2.0p50`, `pgrb2.1p00`, `pgrb2full.0p50`, `sfluxgrbfFFF.grib2`
  - `.nc` = NetCDF format (`atmanl.nc`, `sfcfFFF.nc`)

---

### 2.2 NCEP NAM Data Products

**Primary:** [NCEP Data Products - NAM](https://www.nco.ncep.noaa.gov/pmb/products/nam/)

#### FTP/HTTPS Access
- **FTP:** `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/nam/prod/`
- **HTTPS:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/`
- **NWS FTP:** `ftp://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/MT.nam_CY.00`

#### Directory Structure
```
/pub/data/nccf/com/nam/prod/
└── nam.YYYYMMDD/
    ├── nam.tCCz.conusnest.hiresfFF.tm00.grib2
    ├── nam.tCCz.firewxnest.hiresfFF.tm00.grib2
    ├── nam.tCCz.awphysFF.tm00.grib2
    └── ...
```

#### Naming Convention
- **Pattern:** `nam.tCCz.[dataset].tm00.grib2`
- **Components:**
  - `CC` = Model cycle (00, 06, 12, 18 UTC)
  - `YYYYMMDDhh` = Timestamp

#### Available CONUS Datasets

| Dataset | Resolution | Grid | Description | Filename Template |
|---------|------------|------|-------------|-------------------|
| NAM NEST CONUS | 5 km | Grid 227 | High-resolution nest | `nam.tCCz.conusnest.hiresfFF.tm00.grib2` |
| Fire Weather Nest | 1.33 km | Dynamic | On-Call Nest | `nam.tCCz.firewxnest.hiresfFF.tm00.grib2` |
| AWIPS Grid 218 | 12 km | 218 | Pressure + surface | `nam.tCCz.awphysFF.tm00.grib2` |
| AWIPS Grid 218 (surface) | 12 km | 218 | Surface only | `nam.tCCz.awip12FF.tm00.grib2` |
| AWIPS Grid 215 | 20 km | 215 | CONUS regional | `nam.tCCz.awip20FF.tm00.grib2` |
| Smartinit NDFD | 5 km | Grid 197 | CONUS smartinit | `nam.tCCz.smartconusFF.tm00.grib2` |

---

### 2.3 NCEP RAP Data Products

**Primary:** [NCEP Data Products - RAP](https://www.nco.ncep.noaa.gov/pmb/products/rap/)

#### FTP/HTTPS Access
- **FTP:** `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/rap/prod`
- **HTTPS:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/prod`
- **NWS FTP:** `ftp://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/MT.rap_CY.00`

#### Directory Structure
```
/pub/data/nccf/com/rap/prod/
└── rap.YYYYMMDD/
    ├── rap.tCCz.awp130pgrbf00.grib2
    ├── rap.tCCz.awp252pgrbf00.grib2
    └── ...
```

#### Naming Convention
- **Pattern:** `rap.tCCz.awpXXXpgrbfYY.grib2` or `rap.tCCz.awpXXXbgrbfYY.grib2`
- **Components:**
  - `CC` = Model cycle (hourly)
  - `XXX` = Grid identifier (130 = 13km, 252 = 20km, 236 = 40km)
  - `pgrb` = Pressure levels
  - `bgrb` = Native levels
  - `YY` = Forecast hour

#### Available CONUS Datasets

| Region | Resolution | Type | Filename |
|--------|------------|------|-----------|
| CONUS | 13-km | Pressure Levels | `rap.tCCz.awp130pgrbfYY.grib2` |
| CONUS | 20-km | Pressure Levels | `rap.tCCz.awp252pgrbfYY.grib2` |
| CONUS | 40-km | Pressure Levels | `rap.tCCz.awp236pgrbfYY.grib2` |
| CONUS | 13-km | Native Levels | `rap.tCCz.awp130bgrbfYY.grib2` |
| CONUS | 20-km | Native Levels | `rap.tCCz.awp252bgrbfYY.grib2` |

**Forecast extent:** FH21 standard, FH51 extended (03, 09, 15, 21 UTC only)

---

## 3. NOAA NOMADS (NOAA Operational Model Archive and Distribution System)

**Portal:** [https://nomads.ncep.noaa.gov/](https://nomads.ncep.noaa.gov/)

### Access Methods

#### 3.1 GRIB Filter Service
**Documentation:** https://nomads.ncep.noaa.gov/info.php?page=gribfilter

- Web application for subsetting GRIB2 files
- Filter by parameters, levels, forecast times
- Direct download of filtered subsets

#### 3.2 Fast Download with Random Access
**Documentation:** https://nomads.ncep.noaa.gov/info.php?page=fastdownload

- Random-access reading via HTTP
- Requires index file (.idx)
- Efficient for downloading subsets

#### 3.3 Direct HTTP Access
**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/`

**Models available:**
- **GFS** - Global Forecast System
- **NAM** - North American Mesoscale (CONUS 12km)
- **RAP** - Rapid Refresh (CONUS 13km, 20km)
- **HRRR** - High-Resolution Rapid Refresh (CONUS 3km)
- **GDAS** - Global Data Assimilation System

### GRIB Filter Examples
- **HIRESW CONUS:** https://nomads.ncep.noaa.gov/gribfilter.php?ds=hiresconus
- **NAM CONUS:** https://nomads.ncep.noaa.gov/cgi-bin/filter_nam_conusnest.pl

---

## 4. Google Cloud Platform Public Datasets

**Program:** [Google Cloud Public Datasets](https://cloud.google.com/storage/docs/public-datasets)  
**Partnership:** NOAA Open Data Dissemination (NODD)

### 4.1 GFS Ensemble Forecast System
**Bucket:** `gfs-ensemble-forecast-system`  
**Documentation:** https://developers.google.com/earth-engine/datasets/catalog/NOAA_GFS0P25

- Temporal coverage: July 2015 - October 2025
- Resolution: 0.25 degree
- Format: GRIB2

**Additional Resources:**
- [Google Earth Engine Catalog](https://developers.google.com/earth-engine/datasets/catalog/NOAA_GFS0P25)
- [NOAA-Google Cloud Partnership](https://www.noaa.gov/information-technology/stories/noaa-and-google-cloud-data-match-made-in-cloud)

---

## 5. Microsoft Azure (Planetary Computer)

**Portal:** [Microsoft Planetary Computer](https://planetarycomputer.microsoft.com/)

### 5.1 NOAA GFS
**Documentation:** 
- https://planetarycomputer.microsoft.com/dataset/storage/noaa-gfs
- https://microsoft.github.io/AIforEarthDataSets/data/noaa-gfs.html

**Access:**
- Azure Blob Storage
- Azure ML OpenDatasets API: `azureml.opendatasets.NoaaGfsWeather`
- 15-day US hourly weather forecast data

### 5.2 GFS Warm Start Conditions
**Storage:** https://planetarycomputer.microsoft.com/dataset/storage/gfs-warm-start

---

## 6. University Archives

### 6.1 University of Utah HRRR Archive

**Primary:** [HRRR Archive](https://mesowest.utah.edu/html/hrrr/)  
**Download Interface:** https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi

#### Directory Structure
```
/archive/HRRR/
├── oper/
│   ├── sfc/
│   │   └── YYYYMMDD/
│   │       ├── hrrr.tCCz.wrfsfcfFF.grib2
│   │       └── ...
│   └── prs/
│       └── YYYYMMDD/
│           └── hrrr.tCCz.wrfprsFF.grib2
└── Zarr/
    └── YYYYMMDD/
        └── (48 Zarr files per day: analysis + forecast)
```

#### API Endpoint
`https://api.mesowest.utah.edu/archive/HRRR/oper/sfc/YYYYMMDD/`

#### Zarr Documentation
https://mesowest.utah.edu/html/hrrr/zarr_documentation/html/zarr_variables.html

**Features:**
- GRIB2 files chunked by variable (96 chunks per file)
- Zarr format for efficient cloud access
- Daily folders, 48 Zarr files per day
- Analysis and forecast data

---

## 7. Summary Table of CONUS Datasets

| Model | Resolution | Update Frequency | Cycles | Forecast Length | Primary Archives |
|-------|------------|------------------|--------|----------------|------------------|
| **GFS** | 0.25° global | 6-hourly | 00, 06, 12, 18Z | 384 hours | AWS S3, NCEP FTP, READY |
| **HRRR** | 3-km CONUS | Hourly | 00-23Z | 18 hours | AWS S3, U. Utah, READY |
| **NAM** | 12-km CONUS | 6-hourly | 00, 06, 12, 18Z | 84 hours | NCEP FTP, READY, NOMADS |
| **NAM Nest** | 5-km CONUS | 6-hourly | 00, 06, 12, 18Z | 60 hours | NCEP FTP, NOMADS |
| **RAP** | 13-km CONUS | Hourly | Every hour | 21/51 hours | NCEP FTP, NOMADS |

---

## 8. Access Methods Summary

### Command Line Access

#### AWS S3 (no credentials required)
```bash
# List GFS data
aws s3 ls --no-sign-request s3://noaa-gfs-bdp-pds/gfs.20260724/00/

# List HRRR data
aws s3 ls --no-sign-request s3://noaa-hrrr-bdp-pds/hrrr.20260724/00/

# Download file
aws s3 cp --no-sign-request \
  s3://noaa-gfs-bdp-pds/gfs.20260724/00/gfs.t00z.pgrb2.0p25.f000 \
  ./gfs.t00z.pgrb2.0p25.f000.grib2
```

#### NCEP FTP (anonymous)
```bash
# FTP access
ftp ftp.ncep.noaa.gov
cd /pub/data/nccf/com/gfs/prod/gfs.20260724/00/

# Or with wget/curl
wget ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/gfs.t00z.pgrb2.0p25.f000
curl -O https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/gfs.t00z.pgrb2.0p25.f000
```

#### NOMADS HTTP
```bash
# Direct HTTP download
curl -O https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20260724/nam.t12z.conusnest.hiresf00.tm00.grib2
```

### Python Tools

- **Herbie:** Popular package for downloading GRIB data from multiple sources
  - GitHub: https://github.com/blaylockbk/Herbie
- **rNOMADS:** R package for NOMADS access
- **NOMADS API:** Direct HTTP access with filtering

---

## 9. Additional Resources

### Documentation
- **NCEP GRIB2 Documentation:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- **wgrib2 Utility:** https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/
- **NOMADS Guide:** https://twister.caps.ou.edu/METR3334/GFS_data/DownloadingModelDataFromNcepServer.pdf

### Community Tools
- **CPC get_gfs.pl:** https://www.cpc.ncep.noaa.gov/products/tools/get_gfs.html
- **GRIB Data API:** https://github.com/HumphreysCarter/grib-data-api
- **GribStream API:** https://gribstream.com/models/gfs

---

## 10. Archive Preservation

### Retention Policies
- **NCEP FTP:** ~30 days operational archive
- **AWS S3 Buckets:** Full archive (varies by dataset)
  - GFS: 2019-present
  - HRRR: 2014-present
- **READY:** Historical datasets maintained indefinitely
- **University of Utah:** HRRR from September 2014

### Data Formats
- **Primary:** GRIB2 (all modern products since July 2004)
- **Alternate:** NetCDF (selected GFS products), Zarr (HRRR on AWS), 1-byte packed (READY)

---

## Sources

1. NOAA Registry of Open Data on AWS - https://registry.opendata.aws/noaa-gfs-bdp-pds/
2. NOAA Registry of Open Data on AWS - https://registry.opendata.aws/noaa-hrrr-pds/
3. NCEP Data Products - GFS - https://www.nco.ncep.noaa.gov/pmb/products/gfs/
4. NCEP Data Products - NAM - https://www.nco.ncep.noaa.gov/pmb/products/nam/
5. NCEP Data Products - RAP - https://www.nco.ncep.noaa.gov/pmb/products/rap/
6. NOAA READY Archives - https://www.ready.noaa.gov/archives.php
7. NOAA NOMADS - https://nomads.ncep.noaa.gov/
8. Google Cloud Public Datasets - https://cloud.google.com/storage/docs/public-datasets
9. Microsoft Planetary Computer - https://planetarycomputer.microsoft.com/
10. University of Utah HRRR Archive - https://mesowest.utah.edu/html/hrrr/

---

**Document Version:** 1.0  
**Last Updated:** 2026-07-24  
**Bead ID:** bf-3b63y
