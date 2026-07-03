# NOAA GRIB2 Archive Infrastructure Research

## Summary

This document catalogs NOAA public archives that host weather model GRIB2 files, including their URL patterns, authentication requirements, and data availability.

## 1. NOMADS (NOAA Operational Model Archive and Distribution System)

**Primary Archive for Operational Weather Model Data**

- **Base URL**: `https://nomads.ncep.noaa.gov/`
- **Direct Data Directory Pattern**: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`
- **Authentication**: **None required** (publicly accessible)
- **Help & Documentation**: https://nomads.ncep.noaa.gov/info.php?page=help
- **Fast Download Guide**: https://nomads.ncep.noaa.gov/info.php?page=fastdownload

### Model Types Available:

**Global Models:**
- GFS (Global Forecast System) - multiple resolutions (0.25°, 0.5°, 1.0°)
- GDAS (Global Data Assimilation System) - 0.25° and standard resolution
- GFS Ensemble (0.25° and 0.5°)
- AIGEFS (Atlantic/Indian Ocean GEFS)
- HGEFS (Hawaii GEFS)
- NAEFS (North American Ensemble Forecast System)

**Regional Models:**
- HRRR (High-Resolution Rapid Refresh) - CONUS, Alaska
- NAM (North American Mesoscale) - CONUS, Alaska, Pacific, Caribbean
- RAP (Rapid Refresh) - CONUS, Alaska, Eastern North Pacific
- RTMA (Real-Time Mesoscale Analysis) - multiple regions
- NBM (National Blend of Models)
- SREF (Short-Range Ensemble Forecast)
- HIRESW (High-Resolution Window) - regional domains
- HWRF, HMON, HAFS (hurricane models)
- Various regional nest configurations

**Climate Models:**
- Climate Forecast System (CFS)
- CORe (Climate Reanalysis - replacing CDAS March 18, 2026)

**Ocean/Lake/River Models:**
- National Water Model
- RTOFS (Real-Time Ocean Forecast System)
- GFS Wave
- STOFS (Surge Tide Operational Forecast System)
- Various regional wave and surge models

**Space Weather Models:**
- WSA-Enlil, SWMF-Geospace, WFS (WAM-IPE)

### Data Availability:
- **Near-real-time**: 7-day rotating archive for most operational models
- **Historical**: Limited; older data available through data requests or cloud partners
- **Forecast cycles**: Typically 00z, 06z, 12z, 18z daily

### URL Patterns:

**GRIB Filter Interface**:
```
https://nomads.ncep.noaa.gov/gribfilter.php?ds=gfs_0p25
```

**Direct GFS 0.25° Downloads**:
```
https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl?file=gfs.tCCz.pgrb2.0p25
```
Where `CC` is the forecast cycle (00, 06, 12, 18)

**Directory Structure**:
```
/gfs.YYYYMMDD/HH/atmos/
```

**AWS Mirror Alternative**:
```
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs...
```

## 2. AWS Open Data Registry (NOAA Big Data Program)

**Cloud Mirror with Better Bulk Download Performance**

- **Registry**: `https://registry.opendata.aws/collab/noaa/`
- **Authentication**: **None required** (public S3 buckets)
- **Access**: Direct S3 HTTP access or via AWS CLI

### Available S3 Buckets:

| Bucket | Model | Content |
|--------|-------|---------|
| `noaa-gfs-bdp-pds` | GFS | Global Forecast System GRIB2 files |
| `noaa-hrrr-pds` | HRRR | High-Resolution Rapid Refresh |
| `noaa-nbm-grib2-pds` | NBM | National Blend of Models GRIB2 |
| `noaa-nam` | NAM | North American Mesoscale models |
| `noaa-ndfd` | NDFD | National Digital Forecast Database |

### Example URL Pattern:
```
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.YYYYMMDD/HH/atmos/
https://noaa-nbm-grib2-pds.s3.amazonaws.com/index.html
```

### Data Availability:
- **Near-real-time**: Most recent model runs
- **Historical**: Extended archives depending on bucket (varies by model)
- **Performance**: Better than NOMADS for bulk downloads

### Additional Resources:
- **NBM Documentation**: https://vlab.noaa.gov/web/mdl/nbm-download
- **AWS Guide**: https://predictandprofit.io/blog/aws-s3-as-a-free-weather-data-pipeline-how-noaa-publishes-fo

## 3. NCEI (National Centers for Environmental Information)

**Historical Climate and Weather Data Archive**

- **Main Portal**: `https://www.ncei.noaa.gov/access/search/index`
- **Climate Data Online (CDO)**: `https://www.ncei.noaa.gov/cdo-web/`
- **Authentication**: **None required** (publicly accessible)

### Data Availability:
- **Historical**: Extensive historical weather and climate data
- **Format**: Station data, gridded data, model output
- **Coverage**: Global historical weather and climate data

### Important Note (2026):
**July 7, 2026** - NCEI will release a redesigned Data Access User Interface. URL patterns and access methods may change after this date.

## 4. RDA/GDEX (Research Data Archive at UCAR/NCAR)

**Deep Historical Archives - Registration Required**

- **Main Portal**: `https://rda.ucar.edu/`
- **GDEX (Relaunching)**: September 9, 2025 as "Geoscience Data Exchange"
- **Authentication**: **Registration required** (free account needed)

### Key Datasets:

**NCEP GFS 0.25 Degree Historical Archive**:
- Dataset: `ds084.1`
- URLs:
  - `https://rda.ucar.edu/datasets/ds084.1/`
  - `https://gdex.ucar.edu/datasets/d084001/`
- Content: Analysis and forecast time steps at 3-hour intervals (0-240 hours) and 12-hour intervals (240-384 hours)
- Forecast cycles: 00z, 06z, 12z, 18z
- Format: GRIB2

**Other Datasets**:
- NARR (NCEP North American Regional Reanalysis) - `ds608.0`
- Various reanalysis and climate model outputs

### Data Availability:
- **Historical**: Deep historical archives
- **Research**: curated for atmospheric and ocean sciences research
- **Access**: Requires user registration and authentication

## 5. NOAA READY (Gridded Data Archives)

**Air Quality Modeling Focus**

- **URL**: `https://www.ready.noaa.gov/archives.php`
- **Authentication**: **None required** (publicly accessible)
- **Managed by**: NOAA Air Resources Laboratory (ARL)

### Data Availability:
- **Models**: NCEP model output in GRIB format
- **Processing**: Reprocessed at ARL with 1-byte packing algorithm
- **Focus**: Air quality transport and dispersion modeling

## 6. Pando Archive (University of Utah)

**HRRR and Other Model Archives**

- **Base URL Pattern**: `https://pando-rgw01.chpc.utah.edu/[model type]`
- **Authentication**: **None required** (publicly accessible)
- **Focus**: HRRR archive

### Models Available:
- HRRR CONUS and Alaska
- Additional regional model data

### Tools:
- **Herbie**: Python package for downloading GRIB2 model data from NOAA Big Data Program partners (AWS, GCP, Pando)
- **hrrrb**: Downloads HRRR, RAP, GFS, NBM data from NOMADS and cloud providers

## Key Developments (2025-2026)

1. **March 18, 2026**: NOAA replacing CDAS with CORe (Climate Reanalysis)
   - CORe will remain in public GRIB2 format
   - AWS mirror expected if NOAA publishes one
   - Source: https://gribstream.com/blog/noaa-replaces-cdas-with-core-march-2026

2. **July 7, 2026**: NCEI Data Access UI Redesign
   - URL patterns may change after this date

3. **September 9, 2025**: RDA relaunches as GDEX (Geoscience Data Exchange)

## Python Tools for Access

- **Herbie**: Downloads GRIB2 model data from NOAA Big Data Program partners
- **hrrrb**: Downloads HRRR, RAP, GFS, NBM data from NOMADS and cloud providers
- **ArchiveGribGrab**: R package for downloading archived NOMADS model data

## Sources

- [NOMADS at NCEP](https://nomads.ncep.noaa.gov/)
- [NCEI Data Access](https://www.ncei.noaa.gov/access/search/index)
- [Registry of Open Data on AWS - NOAA](https://registry.opendata.aws/collab/noaa/)
- [UCAR Research Data Archive](https://rda.ucar.edu/)
- [NOAA READY Archives](https://www.ready.noaa.gov/archives.php)
- [GribStream Blog: NOAA CDAS to CORe Transition](https://gribstream.com/blog/noaa-replaces-cdas-with-core-march-2026)
- [AWS S3 as a Free Weather Data Pipeline](https://predictandprofit.io/blog/aws-s3-as-a-free-weather-data-pipeline-how-noaa-publishes-fo)
