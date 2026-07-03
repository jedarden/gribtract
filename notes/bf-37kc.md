# NOAA GRIB2 Public Archives Research

**Task:** bf-37kc - Find publicly accessible NOAA GRIB2 archive URLs

## Summary

This document catalogs publicly accessible NOAA archives hosting GRIB2 files with Lambert-conformal projections, including URL patterns, access methods, and usage constraints.

---

## Primary Archives

### 1. NOMADS (NOAA Operational Model Archive and Distribution System)

**Base URL:** https://nomads.ncep.noaa.gov/

**Access Methods:**
- HTTP direct download
- GRIB Filter service (recommended for subsets)
- HTTPS (transitioned from HTTP in January 2019)

**Authentication:** None required (publicly accessible)

**URL Patterns:**

#### GFS 0.25° Global Model
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDD/CC/atmos/gfs.tCCz.pgrb2.0p25.fFFF
```

Where:
- `YYYYMMDD` = Date (e.g., 20250105)
- `CC` = Cycle hour (00, 06, 12, 18)
- `FFF` = Forecast hour (000-384)
- `0p25` = Resolution (0.25°, also 0p50, 1p00 available)

**Examples:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20250105/00/atmos/gfs.t00z.pgrb2.0p25.f000
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20250105/06/atmos/gfs.t06z.pgrb2.0p25.f001
```

#### GRIB Filter Service (Current Recommended Method)
```
https://nomads.ncep.noaa.gov/gribfilter.php?ds=<model>
```

Models available: `gfs_0p25`, `nam`, `gdas`, and others

**Documentation:**
- [Fast Download Guide](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [GRIB Filter Help](https://nomads.ncep.noaa.gov/info.php?page=gribfilter)
- [OPeNDAP to GRIB Filter Migration](https://nomads.ncep.noaa.gov/info.php?page=opendap_grib_migration)

**Rate Limits:** Not explicitly documented, but fair use policy applies

---

### 2. AWS Open Data Registry - NOAA Big Data Program

**S3 Buckets (publicly accessible without AWS account):**

#### NOAA GFS BDP (Big Data Project)
**Base URL:** https://noaa-gfs-bdp-pds.s3.amazonaws.com/

**AWS CLI access (no authentication):**
```bash
aws s3 ls --no-sign-request s3://noaa-gfs-bdp-pds/
```

#### NOAA NBM GRIB2 (National Blend of Models)
**Base URL:** https://noaa-nbm-grib2-pds.s3.amazonaws.com/index.html

**AWS CLI access:**
```bash
aws s3 ls --no-sign-request s3://noaa-nbm-grib2-pds/
```

#### NOAA HRRR (High-Resolution Rapid Refresh)
**Registry:** https://registry.opendata.aws/noaa-hrrr-pds/
**S3 Bucket:** `s3://noaa-hrrr-pds`

**AWS CLI access:**
```bash
aws s3 ls --no-sign-request s3://noaa-hrrr-pds/
```

**Authentication:** None required - use `--no-sign-request` flag

**Access Methods:**
- Direct HTTPS download via S3 URLs
- AWS CLI without credentials
- Boto3 (Python) with anonymous access

**Rate Limits:** AWS S3 standard request limits apply (generous for public data)

**Documentation:**
- [AWS Registry of Open Data - NOAA](https://registry.opendata.aws/collab/noaa/)
- [NCEI: NOAA Expands Big Data Access](https://www.ncei.noaa.gov/news/noaa-expands-big-data-access)
- [AWS Blog: NOAA Big Data Project](https://aws.amazon.com/blogs/aws/announcing-the-noaa-big-data-project/)

---

### 3. University of Utah HRRR Archive

**Base URL:** https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html

**API Endpoint (GRIB2 files):**
```
https://api.mesowest.utah.edu/archive/HRRR/oper/sfc/[YYYYMMDD]/hrrr.t[HH]z.wrfsfcf[FF].grib2
```

**Index files (metadata):**
```
https://api.mesowest.utah.edu/archive/HRRR/oper/sfc/[YYYYMMDD]/hrrr.t[HH]z.wrfsfcf[FF].grib2.idx
```

**File Naming Pattern:**
```
hrrr.t[00-23]z.wrfsfcf[00-18].grib2
```
- Cycle/run time: `t[00-23]z` (00-23 UTC cycles)
- Forecast hour: `f[00-18]` (0-18 hour forecasts)
- File size: ~120 MB per file

**Example:**
```
https://api.mesowest.utah.edu/archive/HRRR/oper/sfc/20170405/hrrr.t14z.wrfsfcf00.grib2
```

**Authentication:** Registration required at http://hrrr.chpc.utah.edu/

**Web Interface:** https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi

**Python Package:** [hrrrb](https://pypi.org/project/hrrrb/0.0.3/) for programmatic download

**Rate Limits:** Not explicitly documented

---

### 4. NCEI (National Centers for Environmental Information)

**Base URLs:**
- Data Access Search: https://www.ncei.noaa.gov/access/search/index
- Main Access Portal: https://www.ncei.noaa.gov/access
- Archive: https://www.ncei.noaa.gov/archive

**Access Methods:**
- Web search interface
- REST API
- OPeNDAP
- FTP (legacy)

**Authentication:** None required for public datasets

**Coverage:**
- Historical weather data
- Oceanographic data
- Climate data
- Geophysical data

**Note:** NCEI manages over 37 petabytes of environmental data

**Documentation:**
- [NCEI Data Access Portal](https://www.ncei.noaa.gov/access)

---

## Lambert-Conformal Projection Data

### Models with Lambert-Conformal Grids

#### NAM (North American Mesoscale)
- Uses Lambert Conformal Conic projection
- Available via NOMADS: https://nomads.ncep.noaa.gov/gribfilter.php?ds=nam

#### HRRR (High-Resolution Rapid Refresh)
- 3-km resolution CONUS model
- Lambert Conformal projection
- Available via AWS S3 and University of Utah archive

#### NARR (North American Regional Reanalysis)
- 349x277 Lambert Conformal Conic grid (~32km resolution)
- Format documentation: https://psl.noaa.gov/data/narr/format.html

**Processing Tools:**
- [wgrib2](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/) - NOAA CPC utility for GRIB2 manipulation

---

## Access Recommendations

### For Real-Time Data
- **NOMADS** for latest model runs (HTTP direct download)
- **AWS S3** for bulk downloads and high throughput

### For Historical Archives
- **University of Utah HRRR Archive** (registration required)
- **NCEI** for long-term historical data
- **AWS S3 buckets** (retention varies by bucket)

### For Subsetting/Filtering
- **NOMADS GRIB Filter service** - select specific variables/regions
- Use `.idx` index files for partial downloads

---

## Programming Libraries

### R
- **rNOMADS** - functions like `GribGrab()` and `ArchiveGribGrab()`

### Python
- **hrrrb** - for HRRR downloads
- **cfgrib** - GRIB2 reader (requires ECCODES)
- **pygrib** - GRIB2 interface (requires GRIB_API)

### Command Line
- **wgrib2** - GRIB2 utility and library
- **AWS CLI** - for S3 bucket access

---

## References and Documentation

1. [NOMADS Main Page](https://nomads.ncep.noaa.gov/)
2. [NOMADS Fast Download Guide](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
3. [NOMADS GRIB Filter Documentation](https://nomads.ncep.noaa.gov/info.php?page=gribfilter)
4. [OPeNDAP to GRIB Filter Migration](https://nomads.ncep.noaa.gov/info.php?page=opendap_grib_migration)
5. [AWS Registry of Open Data - NOAA](https://registry.opendata.aws/collab/noaa/)
6. [HRRR Archive FAQ - University of Utah](https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html)
7. [NCEI Data Access Portal](https://www.ncei.noaa.gov/access)
8. [NARR Lambert Conformal Grid Documentation](https://psl.noaa.gov/data/narr/format.html)
9. [wgrib2 - NOAA CPC](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
10. [NOAA Expands Big Data Access](https://www.ncei.noaa.gov/news/noaa-expands-big-data-access)

---

**Research Completed:** 2026-07-03
**Task ID:** bf-37kc
