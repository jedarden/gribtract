# NOAA Regional Model Archives

## Summary

This document lists publicly available NOAA archives for regional NWP (Numerical Weather Prediction) models including NAM, HRRR, RAP, and other regional models.

---

## 1. NOAA NOMADS (NOAA Operational Model Archive and Distribution System)

**Base URL:** https://nomads.ncep.noaa.gov/

**Access Methods:**
- HTTPS direct download
- THREDDS Data Server
- OPeNDAP protocol
- GRIB filter for data subsetting

**Models Available:**
- NAM (North American Mesoscale)
- RAP (Rapid Refresh)
- HRRR (High-Resolution Rapid Refresh) - via redirect
- GEFS (Global Ensemble Forecast System)
- NBM (National Blend of Models)
- CFS (Climate Forecast System)
- RTOFS (Real-Time Ocean Forecast System)
- NCOM (Naval Coastal Ocean Model)

**Notes:** This is NOAA's primary operational model archive system. Provides both real-time and archived data with multiple access protocols.

---

## 2. AWS Open Data Registry - HRRR

**Base URL:** https://registry.opendata.aws/noaa-hrrr-pds/

**Access Methods:**
- HTTPS direct download (S3 bucket)
- AWS CLI
- Programmatic access via AWS SDK

**Models Available:**
- HRRR (High-Resolution Rapid Refresh) - full archive from July 2014 to present

**Notes:** 
- Managed by University of Utah MesoWest group
- Free public access with no AWS egress fees
- Contains both GRIB2 and Zarr format data
- Best option for bulk HTTPS downloads

---

## 3. AWS Open Data Registry - NAM

**Base URL:** https://registry.opendata.aws/noaa-nam/

**Access Methods:**
- HTTPS direct download (S3 bucket)
- AWS CLI
- Programmatic access via AWS SDK

**Models Available:**
- NAM (North American Mesoscale) - various CONUS grids

**Notes:**
- Part of NOAA's Open Data Dissemination (NODD) program
- Free and open to the public
- Available via NOAA Big Data Program

---

## 4. University of Utah HRRR Archive (MesoWest)

**Base URL:** https://mesowest.utah.edu/html/hrrr/

**Access Methods:**
- HTTPS direct download
- Interactive web interface: https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi
- FAQ: https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html

**Models Available:**
- HRRR (High-Resolution Rapid Refresh) - comprehensive archive

**Notes:**
- Files sectioned into 96 small chunks for each variable
- Provides download scripts and Python tools (HRRR-B package)
- Surface files >100 MB, pressure files >380 MB each

---

## 5. NOAA READY Gridded Data Archives

**Base URL:** https://www.ready.noaa.gov/archives.php

**Access Methods:**
- HTTPS direct download
- Web interface

**Models Available:**
- NAM
- RAP
- GFS
- Other NCEP models

**Notes:**
- Maintained by NOAA's Air Resources Laboratory
- Archives NCEP model data for various applications
- Data retrievable from cloud or ARL servers

---

## 6. NCEP Product Pages (Real-time Data)

**Base URL:** https://www.nco.ncep.noaa.gov/pmb/products/nam/

**Access Methods:**
- HTTPS direct download (GRIB2 format)

**Models Available:**
- NAM 218 AWIPS Grid - CONUS (12-km Resolution)
- NAM 104 AWIPS Grid - CONUS (5-km Resolution)
- Various other NAM configurations

**Notes:**
- Primarily for real-time and recent forecast data
- GRIB2 format only
- Official NCEP source for operational NAM products

---

## 7. NCEI THREDDS Server

**Base URL:** https://www.ncei.noaa.gov/thredds/catalog/model/model.html

**Access Methods:**
- THREDDS Data Server
- OPeNDAP protocol
- NetCDF download
- Web Map Service (WMS) for some datasets

**Models Available:**
- NAM (12 km CONUS - dataset gov.noaa.ncdc:C00630)
- RAP (Rapid Refresh)
- Other regional models

**Notes:**
- Official NOAA archive with metadata
- Provides standardized web services
- Best for OPeNDAP subsetting and NetCDF extraction

---

## 8. NCEI Model Data Services

**Base URL:** https://www.ncei.noaa.gov/products/weather-climate-models/

**Access Methods:**
- HTTPS download
- Metadata services

**Models Available:**
- RAP (Rapid Refresh) - forecast and analysis data
- Related model documentation

**Notes:**
- NCEI provides access to RAP forecast and analysis data
- HRRR data available through HRRR home page
- Part of official NOAA archive services

---

## 9. UCAR Data Archive

**Base URL:** https://data.ucar.edu/dataset/ncep-north-american-mesoscale-nam-12-km-analysis

**Access Methods:**
- HTTPS download
- THREDDS/OPeNDAP

**Models Available:**
- NAM 12-km analysis (archived over CONUS)
- Updates every 6 hours

**Notes:**
- Analysis output archived at 12km spatial resolution
- Historical data available
- Academic repository with good documentation

---

## 10. Google Cloud Platform - NOAA Big Data Program

**Base URL:** https://cloud.google.com/blog/products/data-analytics/weather-climate-big-data-from-noaa-now-in-cloud

**Access Methods:**
- Google Cloud Public Datasets Program
- BigQuery integration (some datasets)
- Earth Engine integration

**Models Available:**
- Various NOAA environmental datasets (check catalog for specific models)

**Notes:**
- Part of NOAA Big Data Program (BDP)
- Collaboration between NOAA and Google
- Petabyte-scale Earth system data
- Check current catalog for specific regional model availability

---

## Access Protocol Summary

| Archive | HTTPS | THREDDS | OPeNDAP | S3/AWS | GCP |
|----------|-------|---------|---------|--------|-----|
| NOMADS | ✓ | ✓ | ✓ | - | - |
| AWS HRRR | ✓ | - | - | ✓ | - |
| AWS NAM | ✓ | - | - | ✓ | - |
| U of Utah | ✓ | - | - | ✓ | - |
| NOAA READY | ✓ | - | - | - | - |
| NCEP Products | ✓ | - | - | - | - |
| NCEI THREDDS | ✓ | ✓ | ✓ | - | - |
| UCAR | ✓ | ✓ | ✓ | - | - |
| GCP NOAA | ✓ | - | - | - | ✓ |

---

## Recommended Sources by Use Case

### For bulk downloads of HRRR:
- **AWS Open Data Registry** (noaa-hrrr-pds) - best for HTTPS bulk access

### For bulk downloads of NAM:
- **AWS Open Data Registry** (noaa-nam) - part of NODD program

### For real-time NCEP data:
- **NCEP Product Pages** - official source for latest runs

### For research/academic access:
- **NCEI THREDDS** - standardized web services with metadata
- **UCAR Data Archive** - academic repository

### For OPeNDAP subsetting:
- **NOMADS** - multi-protocol access
- **NCEI THREDDS** - official archive

---

## Model Specifications

### HRRR
- Resolution: 3 km
- Update frequency: Hourly
- Domain: CONUS (Continental US)
- Format: GRIB2
- Archive: July 2014 - present (AWS)

### NAM (CONUS 12km)
- Resolution: 12 km
- Grid: Lambert Conformal (AWIPS 218)
- Domain: CONUS
- Format: GRIB2
- Updates: Every 6 hours (analysis), 3-hour forecast cycle

### RAP
- Resolution: 13 km
- Update frequency: Hourly
- Domain: CONUS
- Format: GRIB2

---

## References

- [NOMADS at ncep.noaa.gov](https://nomads.ncep.noaa.gov/)
- [AWS HRRR Public Dataset](https://registry.opendata.aws/noaa-hrrr-pds/)
- [AWS NAM Public Dataset](https://registry.opendata.aws/noaa-nam/)
- [University of Utah HRRR Archive FAQ](https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html)
- [MesoWest HRRR Archive](https://mesowest.utah.edu/html/hrrr/)
- [NOAA READY Archives](https://www.ready.noaa.gov/archives.php)
- [NCEP NAM Products](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [NCEI THREDDS Server](https://www.ncei.noaa.gov/thredds/catalog/model/model.html)
- [NCEI RAP/RUC Products](https://www.ncei.noaa.gov/products/weather-climate-models/rapid-refresh-update)
- [UCAR NAM 12km Archive](https://data.ucar.edu/dataset/ncep-north-american-mesoscale-nam-12-km-analysis)
- [Google Cloud NOAA Big Data](https://cloud.google.com/blog/products/data-analytics/weather-climate-big-data-from-noaa-now-in-cloud)
- [Rapid Refresh Official Site](https://rapidrefresh.noaa.gov/)
- [HRRR Official Site](https://rapidrefresh.noaa.gov/hrrr/)
