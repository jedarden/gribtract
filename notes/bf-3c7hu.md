# NOAA GRIB Archive Structure Research

## Overview

This document summarizes the structure and access patterns for NOAA's public GRIB2 archives, focusing on the NCEP (National Centers for Environmental Prediction) model data distribution systems.

## Main Archive Endpoints

### 1. NOMADS (Primary Archive)
- **URL**: https://nomads.ncep.noaa.gov/
- **Description**: NOAA Operational Model Archive and Distribution System - the primary public access point for operational model data
- **Access**: Public, no authentication required
- **Retention**: ~10 days of operational data
- **Base Path**: `/pub/data/nccf/com/`

### 2. NCEP Central Operations (NCO)
- **URL**: https://www.nco.ncep.noaa.gov/
- **Description**: Documentation and product information for NCEP models
- **Access**: Public documentation
- **Data Access**: Redirects to NOMADS for downloads

### 3. NOAA READY (Historical Archive)
- **URL**: https://www.ready.noaa.gov/archives.php
- **Description**: Gridded meteorological data archives maintained by NOAA Air Resources Laboratory
- **Access**: Public, no authentication required
- **Retention**: Historical data back to 1948 for some datasets
- **Format**: Custom 1-byte packing (not standard GRIB)

### 4. NCEP FTP (Deprecated)
- **URL**: ftp://ftp.ncep.noaa.gov/
- **Status**: Being terminated/deprecated
- **Migration**: Data moved to NOMADS HTTPS access

## Directory Structure

### NCCF COM Structure (NOMADS)
```
/pub/data/nccf/com/[MODEL]/prod/[MODEL].YYYYMMDD/HH/
```

**Components:**
- `/pub/data/nccf/` - Base directory for NCCF (National Centers for Climate Forecasting)
- `/com/` - Communication products (operational model data)
- `[MODEL]` - Model name (gfs, nam, gfswave, etc.)
- `/prod/` - Production directory (operational runs)
- `/para/` - Parallel directory (parallel/test runs)
- `[MODEL].YYYYMMDD/` - Date-based subdirectory
- `HH/` - Forecast cycle hour (00, 06, 12, 18)

### Model Directories Available

Key model directories under `/pub/data/nccf/com/`:

**Atmospheric Models:**
- `gfs/` - Global Forecast System
- `gdas/` - Global Data Assimilation System
- `nam/` - North American Model
- `hrrr/` - High-Resolution Rapid Refresh
- `rap/` - Rapid Refresh
- `sref/` - Short-Range Ensemble Forecast
- `gefs/` - Global Ensemble Forecast System

**Analysis/Blending:**
- `rtma/` - Real-Time Mesoscale Analysis
- `urma/` - Unrestricted Mesoscale Analysis
- `nbm/` - National Blend of Models

**Ocean/Water:**
- `gfswave/` - GFS Wave Model
- `rtofs/` - Real-Time Ocean Forecast System
- `stofs/` - Storm Surge Forecast System
- `nwm/` - National Water Model

**Storm/Hurricane:**
- `hafs/` - Hurricane Analysis and Forecast System
- `hwrf/` - Hurricane Weather Research and Forecasting

## File Naming Conventions

### GFS (Global Forecast System)
**Pattern:**
```
gfs.tCCz.pgrb2.0p25.fFFF.grib2
```

**Components:**
- `gfs.` - Model identifier
- `tCCz` - Forecast cycle time (CC = 00, 06, 12, 18)
- `pgrb2` - Product GRIB2 file indicator
- `0p25` - Resolution (0.25°, 0.50°, 1.00°)
- `fFFF` - Forecast hour (000-384)
- `.grib2` - File extension

**Examples:**
- `gfs.t00z.pgrb2.0p25.f000.grib2` - Analysis, 0.25°
- `gfs.t12z.pgrb2.0p50.f120.grib2` - 120-hour forecast, 0.5°

### GDAS (Global Data Assimilation System)
**Pattern:**
```
gdas.tCCz.pgrb2.0p25.fFFF.grib2
```

**NetCDF Analysis Files:**
```
gdas.YYYYMMDD/CC/atmos/gdas.tCCz.atmanl.nc
gdas.YYYYMMDD/CC/sfc/gdas.tCCz.sfcanl.nc
```

### Directory Naming Pattern
```
gfs.20260724/        - GFS data for July 24, 2026
gdas.20260724/       - GDAS data for July 24, 2026
enkfgdas.20260724/   - Ensemble Kalman Filter GDAS
```

## URL Patterns

### Direct HTTPS Download
**Pattern:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/[MODEL]/prod/[MODEL].YYYYMMDD/[MODEL].tCCz.pgrb2.0p25.fFFF.grib2
```

**Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/gfs.t00z.pgrb2.0p25.f000.grib2
```

### Index Files
Index files (`.idx`) contain byte offsets for random access:
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/gfs.t00z.pgrb2.0p25.f000.grib2.idx
```

### GRIB Filter API
**Pattern:**
```
https://nomads.ncep.noaa.gov/gribfilter.php?ds=[DATASET_CODE]
```

## Access Methods

### 1. HTTPS Direct Download
- **Authentication**: None (public access)
- **Method**: Standard HTTP GET requests
- **Tools**: curl, wget, browsers
- **Rate Limits**: Not explicitly documented

### 2. Random Access with Index Files
- **Requires**:
  - Index file (`.idx`) containing wgrib inventory
  - HTTP client supporting byte-range requests (curl, wget)
- **Procedure**:
  1. Fetch index file
  2. Parse to find desired field byte offsets
  3. Request byte ranges from GRIB2 file

**Example:**
```bash
# Get index
curl https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/gfs.t00z.pgrb2.0p25.f000.grib2.idx -o gfs.idx

# Extract specific field (using byte offsets from index)
curl -r 0-1048576 https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/gfs.t00z.pgrb2.0p25.f000.grib2 -o gfs_subset.grib2
```

### 3. NOAA READY Cloud Archive (AWS S3)
- **Location**: `s3://noaa-oar-arl-hysplit-pds/`
- **Authentication**: None (use `--no-sign-request`)
- **Tools**: AWS CLI, S3 browsers
- **Format**: Custom 1-byte packing (requires CHK_DATA.F utility)

**AWS CLI Commands:**
```bash
aws s3 ls s3://noaa-oar-arl-hysplit-pds/ --no-sign-request
aws s3 cp s3://noaa-oar-arl-hysplit-pds/gfs.0p25/2024/07/ . --recursive --no-sign-request
```

### 4. FTP (READY Archive)
- **Server**: `ftp.arl.noaa.gov`
- **Authentication**: Anonymous (email as password)
- **Restrictions**: Max 2 concurrent connections
- **Mode**: Passive mode required

## Public vs. Restricted Access

### Public Access (No Authentication)

**NOMADS HTTPS:**
- No account required
- No API keys needed
- Open web-based access
- Rate limits not publicly documented

**NOMADS Directory Listings:**
- Full directory browsing available
- https://nomads.ncep.noaa.gov/pub/data/nccf/com/
- https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/

**NOAA READY Archives:**
- Public AWS S3 buckets (no-sign-request)
- Anonymous FTP access
- HTTP web interface

### Historical Archives

**Beyond NOMADS 10-day retention:**

1. **UCAR Research Data Archive (RDA/GDEX)**
   - URL: https://gdex.ucar.edu/
   - Access: Registration required for full API access
   - Public: Dataset discovery and metadata
   - Authentication: Sign-in required for downloads

2. **NOAA CLASS**
   - URL: https://www.class.noaa.gov/
   - Focus: Satellite data (POES, JPSS)
   - Authentication: Likely required (not detailed)

## Data Availability Timeline

| Dataset | Archive | Retention | Access Method |
|---------|---------|-----------|---------------|
| GFS 0.25° | NOMADS | ~10 days | HTTPS public |
| GDAS | NOMADS | ~10 days | HTTPS public |
| NAM | NOMADS | ~10 days | HTTPS public |
| HRRR | NOMADS | ~10 days | HTTPS public |
| GFS 0.25° Historical | READY | 2019-present | AWS S3 public |
| NAM 12km | READY | 2007-present | AWS S3 public |
| GDAS 1° | READY | 2004-present | AWS S3 public |
| NCEP/NCAR Reanalysis | READY | 1948-2026 | AWS S3 public |
| Various Models | UCAR RDA | Decades | Registration required |

## Forecast Cycles

**Standard NCEP Cycle Times:**
- 00 UTC
- 06 UTC
- 12 UTC
- 18 UTC

**Frequency:**
- GFS/GDAS: 4 cycles per day
- NAM: 4 cycles per day (CONUS)
- HRRR: 8 cycles per day (every 3 hours)
- RAP: 8 cycles per day (every 3 hours)

## Additional Resources

### Documentation
- **NCEP GRIB2 Docs**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- **NOMADS Fast Download**: https://nomads.ncep.noaa.gov/info.php?page=fastdownload
- **GRIB Filter Help**: https://nomads.ncep.noaa.gov/info.php?page=gribfilter

### Tools
- **wgrib/wgrib2**: GRIB manipulation tools
- **get_inv.pl**: NOMADS inventory fetching script
- **get_grib.pl**: NOMADS download script
- **CHK_DATA.F**: READY 1-byte unpacking utility

### Alternative Archives
- **UCAR RDA**: https://rda.ucar.edu/
- **NCEP/NCAR Reanalysis**: https://psl.noaa.gov/data/gridded/data.ncep.reanalysis.html
- **AWS NOAA Registry**: https://registry.opendata.aws/noaa/

## Key Takeaways

1. **NOMADS is the primary source** for operational NCEP GRIB2 data (public, no auth)
2. **Directory structure is highly organized** by model, date, and cycle
3. **File naming follows strict conventions** with resolution and forecast hour encoded
4. **HTTPS has replaced FTP** for most NCEP data access
5. **10-day retention on NOMADS** - use READY or UCAR RDA for historical data
6. **No authentication required** for NOMADS and READY public endpoints
7. **Index files enable efficient subsetting** without full file downloads
8. **AWS S3 access** available for READY archive data without authentication

## Sources

- [NOMADS at ncep.noaa.gov](https://nomads.ncep.noaa.gov/)
- [NOMADS Fast Download Documentation](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [NOMADS GRIB Filter Help](https://nomads.ncep.noaa.gov/info.php?page=gribfilter)
- [NCEP GFS/GDAS Products](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [NOAA READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
- [UCAR GDEX](https://gdex.ucar.edu/)
- [NOAA CLASS](https://www.class.noaa.gov/)
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
