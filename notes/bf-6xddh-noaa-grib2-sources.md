# NOAA GRIB2 Archive Sources and Access Methods

## Executive Summary

This document catalogs the primary NOAA GRIB2 archive sources available for DRT=0 file searches. All major archives provide public access without authentication requirements, with varying degrees of programmatic interface support.

## Primary Archive Sources

### 1. NCEI (National Centers for Environmental Information)

**Organization:** National Centers for Environmental Information (NOAA)  
**Primary Use:** Long-term archival and data services

#### Access Methods
- **REST API:** https://www.ncei.noaa.gov/access/services/data/v1
- **Web Portal:** https://www.ncei.noaa.gov/access/search/dataset-search/
- **Documentation:** [NCEI Data Service API User Documentation](https://www.ncei.noaa.gov/support/access-data-service-api-user-documentation)

#### Technical Details
- **Base URL:** `https://www.ncei.noaa.gov/access/services/data/v1`
- **Authentication:** None required (public HTTPS)
- **Protocol:** HTTP 1.1 GET requests
- **Data Formats:** CSV, SSV, JSON, PDF, NetCDF
- **HSTS:** Enforced following Federal government policy

#### Temporal Coverage
- Varies by dataset
- Some datasets dating back to 1901
- Quarterly additions of new datasets
- Data transitioned from NOMADS after ~1 month

#### Programmatic Access
- Full REST API with parameter-based queries
- Support for bounding box spatial selection (bbox=N,W,S,E)
- Date range selection (startDate, endDate)
- Station selection for station-based datasets
- Multiple output formats

#### Browse/Search Interface
- Comprehensive web portal at [NCEI Search](https://www.ncei.noaa.gov/access/search/dataset-search/)
- ~110 separate data sources updated daily
- Filter by dataset, station, date range, and data types

#### Key Parameters
```http
?dataset=daily-summaries
&stations=USC00457180,USC00390043
&startDate=2001-11-02T12:45:00Z
&endDate=2024-01-01T00:00:00Z
&dataTypes=MLY-PRCP-NORMAL,MLY-TMIN-NORMAL
&bbox=49.795,-2.073,49.183,-0.992
&format=csv
&includeAttributes=true
```

---

### 2. NOMADS (NOAA Operational Model Archive and Distribution System)

**Organization:** National Centers for Environmental Prediction (NCEP)  
**Primary Use:** Recent operational model data distribution

#### Access Methods
- **Main Portal:** https://nomads.ncep.noaa.gov/
- **Fast GRIB2 Download:** [Partial HTTP Transfers Documentation](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- **GRIB Filter Web App:** For subsetting and repackaging GRIB2 files

#### Technical Details
- **Base URL:** `https://nomads.ncep.noaa.gov/`
- **Authentication:** None required (public HTTP/HTTPS)
- **Protocol:** HTTP random access with index files
- **Data Format:** Primarily GRIB2 (some GRIB legacy)
- **Tools:** wgrib, wgrib2, cURL, perl scripts

#### Temporal Coverage
- **Recent data:** Up to approximately 1 month
- **Older data:** Transitions to NCEI archives
- **Data retention:** Operational models retained for ~30 days
- **Archive transition:** Automatic transfer to NCEI

#### Programmatic Access

**Method 1: Fast Download with Partial HTTP Transfers**

Requires index files (`.idx`) and HTTP program supporting random access:

```bash
# Basic pattern:
get_inv.pl INV_URL | grep FIELDS | get_grib.pl GRIB_URL OUTPUT

# Example: Download 500mb height from GFS forecast
INV_URL="http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.2008120200/gfs.t00z.pgrbf12.grib2.idx"
GRIB_URL="http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.2008120200/gfs.t00z.pgrbf12.grib2"
get_inv.pl $INV_URL | grep ":HGT:500 mb:" | get_grib.pl $GRIB_URL out.grb
```

**Requirements:**
- perl
- grep
- cURL with HTTP range support
- grib files with wgrib inventory index
- `get_inv.pl` and `get_grib.pl` scripts

**Method 2: Direct API Access**
Different models have different API endpoints (GFS, GEM, etc.)

#### Browse/Search Interface
- Main NOMADS portal provides data access method matrix
- Data Set field contains description and availability information
- Column headings describe each access method

#### Model Coverage
- GFS (Global Forecast System)
- GDAS (Global Data Assimilation System)
- GFS Wave Model
- Various regional and specialized models

---

### 3. NOAA Big Data Project / NODD (Open Data Dissemination)

**Organization:** NOAA Open Data Dissemination Program  
**Primary Use:** Cloud-native access to petabyte-scale NOAA datasets

#### Access Methods
- **AWS Registry:** [Registry of Open Data on AWS - NOAA](https://registry.opendata.aws/collab/noaa/)
- **Direct S3 Access:** Public S3 buckets (no AWS credentials required)
- **Documentation:** [NOAA NODD Program Information](https://www.noaa.gov/information-technology/open-data-dissemination)

#### Cloud Platforms
- **Amazon Web Services (AWS)** - Primary platform
- **Microsoft Azure**
- **Google Cloud Platform**

#### Technical Details
- **Authentication:** None required for public data (AWS: anonymous access)
- **Protocol:** S3 API (AWS), equivalent for Azure/GCP
- **Data Format:** Primarily GRIB2, some ZARR/NetCDF
- **Access Speed:** Cloud-native, high-bandwidth

#### Key GRIB2 Datasets on AWS

| Dataset | S3 Bucket | Temporal Coverage | Resolution |
|---------|-----------|-------------------|------------|
| GFS (Global Forecast System) | `noaa-gfs-bdp-pds` | 2019-present | 0.25° global |
| NBM (National Blend of Models) | `noaa-nbm-grib2-pds` | 2021-present | CONUS ~3km |
| HRRR (High-Resolution Rapid Refresh) | `noaa-hrrr-bdp-pds` | 2015-present | 3km CONUS |
| GEFS (Global Ensemble Forecast System) | `noaa-gefs-bdp-pds` | 2019-present | Ensemble |
| RTMA/URMA | Various buckets | 2003-present | ~2.5km CONUS |

#### Direct S3 Access Examples

```bash
# List GFS data in S3 bucket (no AWS credentials needed)
aws s3 ls s3://noaa-gfs-bdp-pds/ --no-sign-request

# Access specific GFS GRIB2 file
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20240101/00/atmos/gfs.t00z.pgrb2.0p25.f000

# NBM GRIB2 file
https://noaa-nbm-grib2-pds.s3.amazonaws.com/nbm.20240101/00/nbm.t00z.conusnest.02.500mb.grib2
```

#### Temporal Coverage by Dataset
- **GFS:** 2019-present (near real-time, 4 cycles/day)
- **HRRR:** 2015-present (hourly updates)
- **NBM:** 2021-present (hourly, CONUS)
- **GEFS:** 2019-present (ensemble members)
- **Climate reanalysis:** Multi-decade datasets available

#### Programmatic Access
- Direct S3 API calls (anonymous access)
- AWS CLI with `--no-sign-request`
- Python boto3 with anonymous configuration
- Community tools: Herbie package, HRRR-B package
- Sample notebooks: [NOAA GFS Quickstart](https://github.com/aws-samples/aws-opendata-samples/blob/main/notebooks/noaa-gfs/noaa_gfs_quickstart.ipynb)

#### Browse/Search Interface
- [AWS S3 Explorer for GFS](https://noaa-gfs-bdp-pds.s3.amazonaws.com/index.html)
- [NODD Dataset Registry](https://www.noaa.gov/nodd/datasets)
- Quarterly updates to dataset registry

---

### 4. NCEP Products (Direct Model Output)

**Organization:** National Centers for Environmental Prediction  
**Primary Use:** Direct access to operational model products

#### Access Methods
- **Products Page:** [NCEP GFS and GDAS Products](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- **FTP Server:** ftp://ftpprd.ncep.noaa.gov/
- **HTTP Server:** http://www.nco.ncep.noaa.gov/pmb/data/

#### Technical Details
- **Base URLs:**
  - FTP: `ftp://ftpprd.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`
  - HTTP: `http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`
- **Authentication:** None (anonymous FTP/public HTTP)
- **Protocol:** FTP, HTTP
- **Data Format:** GRIB2

#### Temporal Coverage
- **Operational:** Real-time, latest model runs
- **Archive:** Varies by model (typically days to weeks)
- **Long-term:** Transitions to NCEI

#### Model Coverage
- GFS (Global Forecast System)
- GDAS (Global Data Assimilation System)
- GFS Wave Model
- NAM (North American Mesoscale)
- RAP (Rapid Refresh)
- HRRR (High-Resolution Rapid Refresh)

#### Programmatic Access
- Standard FTP clients
- wget/curl for HTTP
- Index file-based subsetting (as documented in NOMADS section)
- Direct file path construction

---

### 5. NOAA READY Archives

**Organization:** NOAA Air Resources Laboratory  
**Primary Use:** Archived model output for atmospheric dispersion modeling

#### Access Methods
- **Web Portal:** [READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
- **Documentation:** Integrated with READY system

#### Technical Details
- **Access Method:** Web interface + direct HTTP
- **Authentication:** None required
- **Data Format:** Reprocessed GRIB format

#### Temporal Coverage
- Varies by model
- Focus on meteorological data for dispersion modeling
- Archive extends beyond NOMADS retention period

---

### 6. Additional NOAA GRIB2 Sources

#### NOAA Earthdata (NASA Partnership)
- **Portal:** [NOAA Earthdata - NCEP GFS 0.25 Degree](https://access.earthdata.nasa.gov/collections/C1214110986-SCIOPS)
- **Coverage:** Historical GFS 0.25 degree global forecast grids
- **Schedule:** Model forecast runs at 00, 06, 12, 18 UTC daily
- **Access:** NASA Earthdata system (requires NASA registration)

#### University of Utah HRRR Archive
- **Format:** ZARR (cloud-optimized)
- **Access:** Via AWS or direct download
- **Documentation:** Referenced in AWS HRRR dataset details

#### NOAA ISD (Integrated Surface Database)
- **AWS Registry:** [NOAA ISD Dataset](https://registry.opendata.aws/noaa-isd/)
- **Coverage:** Global hourly observations from 35,000+ stations
- **Temporal:** Some data back to 1901
- **Format:** gzipped fixed width (not GRIB, but related surface obs)

---

## Summary Table of Primary GRIB2 Sources

| Source | Access Method | Auth Required | Temporal Coverage | Programmatic Access | Browse Interface |
|--------|--------------|----------------|-------------------|---------------------|-------------------|
| **NCEI** | REST API | No | Varies (decades+) | Full REST API | Yes, comprehensive web portal |
| **NOMADS** | HTTP + Scripts | No | ~1 month | Yes (scripts, API) | Yes, data access matrix |
| **NODD/AWS** | S3 API | No | Varies (2019+) | Yes (S3, tools) | Yes, S3 Explorer |
| **NCEP Direct** | FTP/HTTP | No | Days-weeks | Yes (standard tools) | Limited (products page) |
| **READY** | HTTP + Web | No | Varies | Yes (direct HTTP) | Yes, archive portal |
| **Earthdata** | HTTPS | Yes (NASA) | Historical | Yes (API) | Yes, Earthdata portal |

---

## Programmatic Access Recommendations by Use Case

### For DRT=0 File Searches
1. **Start with NODD/AWS buckets** - Most comprehensive recent data, cloud-optimized access
2. **Use NCEI REST API** - For structured searches with specific parameters
3. **Leverage NOMADS fast download** - For subsetting specific GRIB2 messages
4. **Cross-reference with NCEP products** - For latest model runs

### For Historical Data
1. **NCEI Archives** - Primary long-term storage
2. **READY Archives** - Reprocessed historical model output
3. **Earthdata** - Historical GFS via NASA partnership

### For Real-Time/Near-Real-Time
1. **NOMADS** - Latest model runs (within ~1 month)
2. **NODD/AWS buckets** - Cloud-native, high-bandwidth access
3. **NCEP FTP/HTTP** - Direct from operational servers

---

## Key Resources and Documentation

- **NCEI API Docs:** [https://www.ncei.noaa.gov/support/access-data-service-api-user-documentation](https://www.ncei.noaa.gov/support/access-data-service-api-user-documentation)
- **NOMADS Fast Download:** [https://nomads.ncep.noaa.gov/info.php?page=fastdownload](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- **NODD Program:** [https://www.noaa.gov/information-technology/open-data-dissemination](https://www.noaa.gov/information-technology/open-data-dissemination)
- **AWS NOAA Registry:** [https://registry.opendata.aws/collab/noaa/](https://registry.opendata.aws/collab/noaa/)
- **NCEP GFS Products:** [https://www.nco.ncep.noaa.gov/pmb/products/gfs/](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- **Community Tools:** Herbie Python package, HRRR-B package

---

## Authentication Summary

**All major NOAA GRIB2 archives provide public access without authentication requirements:**

- ✅ NCEI: No authentication (public HTTPS)
- ✅ NOMADS: No authentication (public HTTP/HTTPS)
- ✅ NODD/AWS: No authentication (public S3 buckets, anonymous access)
- ✅ NCEP FTP/HTTP: No authentication (anonymous FTP/public HTTP)
- ✅ READY: No authentication (public HTTP)

**Exception:** NASA Earthdata portal requires NASA account registration for some historical GFS datasets.

---

## File Naming Conventions

### GFS (NOMADS/NCEP)
```
gfs.YYYYMMDD/HH/gfs.tHHz.pgrb2.fXXX
gfs.t00z.pgrb2.0p25.f000  # Analysis
gfs.t00z.pgrb2.0p25.f003  # 3-hour forecast
```

### GFS (AWS/NODD)
```
gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.0p25.fXXX
```

### NBM
```
nbm.YYYYMMDD/HH/nbm.tHHz.conusnest.02.500mb.grib2
```

### HRRR
```
hrrr.tHHz.wrfsfcfXX.grib2
```

---

## Notes for DRT=0 Search Strategy

1. **DRT=0 files should be present across all these archives** - DRT (Data Representation Type) is part of GRIB2 specification, not archive-specific

2. **Search priority:**
   - NODD/AWS buckets for recent data (fastest, cloud-optimized)
   - NCEI for long-term archives (comprehensive REST API)
   - NOMADS for recent operational data (with subsetting tools)

3. **Index file availability:**
   - NOMADS provides `.idx` files for partial downloads
   - AWS S3 buckets may have index files or require full file downloads
   - NCEI API provides structured metadata for subsetting

4. **For CONUS DRT=0 searches specifically:**
   - NBM (CONUS ~3km) on AWS: `noaa-nbm-grib2-pds`
   - HRRR (3km CONUS) on AWS: `noaa-hrrr-bdp-pds`
   - NCEI CONUS datasets via REST API with bounding box
   - NOMADS GFS CONUS subsetting via GRIB filter

---

*Documentation compiled for bead bf-6xddh: NOAA GRIB2 archive sources and access methods catalog.*
