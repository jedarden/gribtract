# NOAA Regional Model GRIB2 Archives Research

## Overview

This document summarizes research into NOAA's public archives for regional model outputs in GRIB2 format, with focus on models using Grid Definition Template (GDT) 3.30 (Lambert-conformal conic) and Data Representation Template (DRT) 5.3 (complex packing).

## Models Using Lambert Conformal Conic Projection (GDT 3.30)

### 1. HRRR (High-Resolution Rapid Refresh)

**Characteristics:**
- **Resolution:** 3km
- **Domain:** CONUS and Alaska sectors
- **Projection:** Lambert Conformal Conic (GDT 3.30)
- **Update frequency:** Hourly (every hour, 00z-23z)
- **Archive start:** September 30, 2014 (University of Utah archive)

**File Naming Convention:**
```
hrrr.tCCz.wrfsfcfFF.grib2
hrrr.tCCz.wrfprsfFF.grib2
hrrr.tCCz.wrfnatfFF.grib2
```
Where:
- `CC` = cycle runtime (00-23)
- `FF` = forecast hour (00-48)

**Run Schedule:**
- **Standard cycles:** All hours (00z-23z), forecast out to FH18
- **Extended cycles:** 00z, 06z, 12z, 18z run out to FH48
- **Sub-hourly data:** Available for FH01-FH18 (15, 30, 45 min before the hour)

**Typical File Size:** ~120 MB per GRIB2 file

**Archive URLs:**
- NOAA NCEP: https://www.nco.ncep.noaa.gov/pmb/products/hrrr/
- University of Utah CHPC Archive: https://mesowest.utah.edu/html/hrrr/
- AWS Open Data Registry: https://registry.opendata.aws/noaa-hrrr-pds/
- NOMADS: https://nomads.ncep.noaa.gov/gribfilter.php?ds=hrrr_2d

**Access Methods:**
- FTP/HTTPS from NCEP servers
- AWS S3 (noaa-hrrr-pds bucket)
- NOMADS with subsetting capabilities
- University of Utah download tool with metadata (.idx) files

### 2. NAM (North American Mesoscale) - Regional Nests

**Characteristics:**
- **Resolution:** Varies by domain (1.33km - 6km for regional nests)
- **Domains:** 
  - CONUS Nest (5km, Grid 227)
  - Alaska Nest (6km, Grid 198)
  - Hawaii Nest (3km, Grid 196)
  - Puerto Rico Nest (3km, Grid 194)
  - Fire Weather Nest (1.33km CONUS / 1.5km Alaska)
- **Projection:** Lambert Conformal Conic for regional nests
- **Update frequency:** 4x daily (00z, 06z, 12z, 18z)

**File Naming Convention:**
```
nam.tCCz.conusnest.hiresffh.tm00.grib2
nam.tCCz.alaskanest.hiresffh.tm00.grib2
nam.tCCz.hawaiinest.hiresffh.tm00.grib2
nam.tCCz.priconest.hiresffh.tm00.grib2
nam.tCCz.firewxnest.hiresffh.tm00.grib2
```
Where:
- `CC` = cycle runtime (00, 06, 12, 18)
- `ffh` = forecast hour (00-60)

**Run Schedule:**
- **Cycle times:** 00z, 06z, 12z, 18z
- **Forecast hours:** FH00-FH60 for most nests (FH00-FH36 for Fire Weather)

**Archive URLs:**
- NOAA NCEP: https://www.nco.ncep.noaa.gov/pmb/products/nam/
- NOMADS: https://nomads.ncep.noaa.gov/
- UCAR GDEX: https://gdex.ucar.edu/datasets/d609000/
- READY Archives: https://www.ready.noaa.gov/archives.php

**Access Methods:**
- FTP/HTTPS from NCEP servers
- NOMADS with GRIB2 subsetting
- AWS S3 commands via READY

### 3. RAP (Rapid Refresh)

**Characteristics:**
- **Resolution:** 13km (CONUS Lambert Conformal)
- **Domain:** CONUS on Lambert Conformal projection
- **Projection:** Lambert Conformal Conic (GDT 3.30)
- **Update frequency:** Hourly (every hour, 00z-23z)

**File Naming Convention:**
```
rap.tCCz.awp130pgrbfxx.grib2  (13km pressure levels)
rap.tCCz.awp252pgrbfxx.grib2  (20km pressure levels)
rap.tCCz.awp236pgrbfxx.grib2  (40km pressure levels)
```
Where:
- `CC` = cycle runtime (00-23)
- `xx` = forecast hour

**Run Schedule:**
- **Standard cycles:** All hours, forecast out to FH21
- **Extended cycles:** 03z, 09z, 15z, 21z run out to FH51

**Archive URLs:**
- NOAA NCEP: https://www.nco.ncep.noaa.gov/pmb/products/rap/
- AWS Open Data Registry: https://registry.opendata.aws/collab/noaa/

**Access Methods:**
- FTP/HTTPS from NCEP servers
- AWS S3 (noaa-rap-pds bucket)

## GRIB2 Encoding Specifications

### Grid Definition Template 3.30: Lambert Conformal Conic

**Documentation:**
- NOAA NCO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml
- NCEP Table 3.1: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml

**Key Parameters:**
- Grid lengths in units of 10^-3 m at specified latitude (LaD)
- If Latin1 = Latin2, projection is on a tangent cone
- LoV is the longitude of the meridian parallel to the y-axis (orientation)

### Data Representation Template 5.3: Complex Packing with Spatial Differencing

**Documentation:**
- NOAA NCO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml
- ECMWF Code Table 5.0: https://codes.ecmwf.int/grib/format/grib2/ctables/5/0/

**Description:**
- "Grid point data - complex packing and spatial differencing"
- Divides data fields into groups with similar value ranges
- Uses spatial differencing to improve compression efficiency
- Commonly used in meteorological data encoding

## Archive Access Patterns

### HTTPS Access Pattern (NCEP)
```
https://<server>.ncep.noaa.gov/data/<model>/<YYYYMMDD>/<filename>
```

### NOMADS Subsetting
- Time subsetting (specific forecast hours)
- Variable subsetting (specific parameters)
- Level subsetting (vertical levels)
- Regional subsetting (lat/lon bounding boxes)

### AWS S3 Access Pattern
```
s3://noaa-hrrr-pds/<YYYYMMDD>/hrrr.tCCz.wrfsfcfFF.grib2
```

## File Size Characteristics

| Model | Typical File Size | Resolution | Forecast Hours |
|-------|------------------|------------|----------------|
| HRRR 2D Surface | ~120 MB | 3km | FH00-48 |
| HRRR 3D Pressure | ~200+ MB | 3km | FH00-48 |
| NAM CONUS Nest | ~50-100 MB | 5km | FH00-60 |
| RAP 13km | ~30-80 MB | 13km | FH00-51 |

## Model Run Availability Patterns

### HRRR
- **Current operational:** V4 (since May 2019)
- **Archive depth:** University of Utah archive from Sep 2014
- **Update latency:** ~1 hour after run time
- **Retention:** AWS 30-day rolling window

### NAM
- **Current operational:** Multiple nests
- **Archive depth:** Varies by nest and source
- **Update latency:** ~1-2 hours after run time
- **Retention:** Variable (NOMADS has historical archive)

### RAP
- **Current operational:** RAPv4 (since July 2018)
- **Archive depth:** Varies by source
- **Update latency:** ~1 hour after run time
- **Retention:** AWS 30-day rolling window

## Future Developments

**RRFS (Rapid Refresh Forecast System):**
- NOAA's next-generation regional NWP model
- Will eventually replace HRRR, RAP, and NAM
- Currently in development

## Sources

- [High-Resolution Rapid Refresh (HRRR) - NOAA](https://rapidrefresh.noaa.gov/hrrr/)
- [NCEP Data Products HRRR](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [NCEP Data Products NAM](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [NCEP Data Products RAP](https://www.nco.ncep.noaa.gov/pmb/products/rap/)
- [GRIB2 Table 3.1 - Grid Definition Template Number](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
- [GRIB2 Table 5.0 - Data Representation Template Number](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml)
- [GRIB2 Template 3.30 - Lambert Conformal](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)
- [GRIB2 Template 5.3 - Complex Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml)
- [Registry of Open Data on AWS - NOAA](https://registry.opendata.aws/collab/noaa/)
- [NOAA HRRR on AWS](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NOMADS Data Server](https://nomads.ncep.noaa.gov/)
- [READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
- [University of Utah HRRR Archive](https://mesowest.utah.edu/html/hrrr/)
- [Brian Blaylock's HRRR FAQ](https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html)
- [RAP Model Metadata - NOAA NCEI](https://www.ncei.noaa.gov/access/metadata/landing-page/bin/iso?id=gov.noaa.ncdc:C00690)
