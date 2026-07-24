# NOAA GRIB Archive Research - bf-2li5w

## Summary

Research conducted on NOAA GRIB archive URLs and access patterns. This document identifies public archives that host GRIB2 files, their URL patterns, and model coverage information.

## Archive URLs

### 1. NCEP NOMADS (NOAA Operational Model Archive and Distribution System)
**URL:** https://nomads.ncep.noaa.gov/

**Primary source** for NCEP operational model data including GFS, NAM, and other models. Provides multiple access methods including GRIB filter service for subsetting data.

- **GRIB Filter:** https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl
- **Documentation:** https://nomads.ncep.noaa.gov/info.php?page=fastdownload
- **OpenDAP Migration Guide:** https://nomads.ncep.noaa.gov/info.php?page=opendap_grib_migration

### 2. NWS tgftp Server
**URL:** ftp://tgftp.nws.noaa.gov/

**Primary FTP server** for NWS operational model GRIB2 data. Uses structured directory paths for organizing models by type, run cycle, and date.

**Base pattern:**
```
ftp://tgftp.nws.noaa.gov/SL.us008001/ST.{TYPE}/DF.gr2/{SUBDIRECTORIES}
```

### 3. NCEP Central Operations (NCO) Product Pages
**URL:** https://www.nco.ncep.noaa.gov/pmb/products/

**Official source** for model product information, file naming conventions, and specifications. Covers GFS/GDAS, HRRR, NAM, and other models.

### 4. AWS Open Data (HRRR)
**Registry:** https://registry.opendata.aws/noaa-hrrr-pds/

**Cloud-hosted archive** for HRRR model output on AWS. Recommended for HRRR data access with free public availability.

### 5. NOAA READY Gridded Data Archives
**URL:** https://www.ready.noaa.gov/archives.php

**Archive system** containing NCEP model output in GRIB format. Includes GDAS data at https://www.ready.noaa.gov/data/archives/gdas0p5.

## URL Patterns

### GFS (Global Forecast System)

#### NOMADS Pattern:
```
http://nomad.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDDHH/gfs.tHHz.pgrbfFF.grib2
```

**Components:**
- `YYYYMMDDHH` = Run date and hour (e.g., `gfs.2008120200`)
- `tHHz` = Cycle time in Zulu (e.g., `t00z` for 00Z run, `t12z` for 12Z)
- `fFF` = Forecast hour (e.g., `f00` for analysis, `f12` for 12-hour forecast)
- `.grib2` = GRIB2 format file
- `.grib2.idx` = Index file for random access

**Example:**
```
http://nomad.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.2008120200/gfs.t00z.pgrbf12.grib2
```

#### tgftp Pattern:
```
ftp://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/MT.gfs_CY.HH/RD.YYYYMMDD/PT.grid_DF.gr2
```

#### GFS 0.25° Resolution:
**NOMADS Filter:** https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl

**File naming:** `gfs.tCCz.pgrb2.0p25.fFFF`
- `CC` = Cycle (00, 06, 12, 18)
- `FFF` = Forecast hour (000, 006, 012, ...)

### HRRR (High-Resolution Rapid Refresh)

**Official page:** https://rapidrefresh.noaa.gov/hrrr/
**Product info:** https://www.nco.ncep.noaa.gov/pmb/products/hrrr/

**Access methods:**
- **AWS:** Most common for programmatic access via registry.opendata.aws
- **Herbie package:** Python tool simplifying downloads from AWS
- **University of Utah interface:** https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi

### NAM (North American Mesoscale)

Available through NOMADS with similar URL structure to GFS, under NAM product directories.

### SREF (Short-Range Ensemble Forecast)

```
ftp://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/MT.sref_CY.CC/RD.YYYYMMDD
```

- `CC` = Cycle (03, 09, 15, 21)
- `YYYYMMDD` = Current date

### GEFS (Global Ensemble Forecast System)

```
ftp://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/MT.ensg_CY.${CYC}/RD...
```

## CONUS-Covering Models

### Models with CONUS Coverage:

1. **HRRR (High-Resolution Rapid Refresh)**
   - 3-km resolution
   - Hourly updated
   - CONUS domain
   - Cloud-resolving, convection-allowing

2. **RAP (Rapid Refresh)**
   - CONUS coverage
   - Similar to HRRR but coarser resolution
   - Official page: https://rapidrefresh.noaa.gov/

3. **NAM (North American Mesoscale)**
   - CONUS and North America coverage
   - Multiple resolution configurations

4. **NDFD (National Digital Forecast Database)**
   - CONUS-specific
   - URL pattern: `ftp://tgftp.nws.noaa.gov/SL.us008001/st.expr/df.gr2/dc.ndfd/ar.conus`

5. **Storm Surge Models**
   - CONUS coastal focus
   - Example naming: `psurge.tDATEz.IDYYYY_e90_cum_agl.hFFF.conus_625m.grib2`

## Authentication and Access Restrictions

### All Archives Are Public
- **No authentication required** for any of the documented archives
- All are freely accessible public data
- No API keys or credentials needed

### Access Notes:
- **HTTP to HTTPS migration:** URLs transitioned from HTTP to HTTPS around 2019
- **NOMADS limitations:** Archive periods vary by model, some sources have download limits
- **Migration recommendation:** NOAA encourages users to move from NOMADS to NCO servers for more reliable access
- **FTP vs HTTPS:** tgftp.nws.noaa.gov uses FTP protocol, while NOMADS uses HTTP/HTTPS

### Rate Limits:
- Some sources may have download limits (check NOMADS documentation for specific model limits)
- AWS Open Data has standard AWS data transfer limits

## Additional Resources

### Python Packages for Programmatic Access:
- **Herbie:** Simplifies downloading from multiple NOAA sources
- **hrrrb:** HRRR-specific download tool
- **stactools-noaa-hrrr:** STAC-compliant HRRR access

### Documentation Links:
- [NOMADS Fast Download](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [CPC NCEP GFS Download Guide](https://www.cpc.ncep.noaa.gov/products/tools/get_gfs.html)
- [GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [Herbie Data Sources](https://herbie.readthedocs.io/en/2025.11.3/user_guide/background/data_sources.html)

## Sources

- [NOMADS at ncep.noaa.gov](https://nomads.ncep.noaa.gov/)
- [NCEP GFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [HRRR Homepage](https://rapidrefresh.noaa.gov/hrrr/)
- [AWS NOAA HRRR Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [READY Archives](https://www.ready.noaa.gov/archives.php)
- [UCAR NCEP GFS 0.25 Degree Archive](https://data.ucar.edu/dataset/ncep-gfs-0-25-degree-global-forecast-grids-historical-archive)
