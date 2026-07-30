# NOAA Ensemble/Statistical GRIB2 Files Search Results

## Task Summary

Searched NOAA public archives for ensemble or statistical products using GRIB2 Product Definition Template (PDT) 4.1 or 4.8:
- **PDT 4.1**: Individual ensemble forecast, control and perturbed members
- **PDT 4.8**: Average, Accumulation, Extreme values, or other Statistically-processed values

## Product Types and Sources

### 1. GEFS (Global Ensemble Forecast System)

**Description**: NOAA/NCEP's medium-range atmospheric ensemble forecast system with 21 ensemble members (1 control + 30 perturbed, typically 21 active)

**Run cycles**: 00, 06, 12, 18 UTC daily

**Forecast hours**: Up to 384 hours (16 days)

**Resolutions**:
- 0.5° degree (pgrb2a/pgrb2b)
- 0.25° degree (pgrb2s)
- Wave: 0.25° degree global

**File Naming Conventions**:
- Control: `gec00.tCCz.pgrb2a.0p50.fxxx`
- Perturbed: `gepNN.tCCz.pgrb2a.0p50.fxxx` (NN = 01-30)
- **Ensemble Mean**: `geavg.tCCz.pgrb2a.0p50.fxxx`
- **Ensemble Spread**: `gespr.tCCz.pgrb2a.0p50.fxxx`

Where:
- CC = cycle (00, 06, 12, 18)
- z = zulu time
- xxx = forecast hour (000, 003, 006, ..., 384)

**Candidate URLs**:

1. **NCEP GEFS Products Page**
   URL: https://www.nco.ncep.noaa.gov/pmb/products/gens/
   - Official documentation with file naming and inventory

2. **NOMADS (NOAA Operational Model Archive and Distribution System)**
   Main: https://nomads.ncep.noaa.gov/
   - GEFS 0.5° filter: https://nomads.ncep.noaa.gov/gribfilter.php?ds=gefs_atmos_0p50a
   - Provides GRIB2 subsetting and download

3. **AWS Open Data - NOAA GEFS**
   Registry: https://registry.opendata.aws/noaa-gefs/
   S3 Bucket: https://noaa-gefs-pds.s3.amazonaws.com/index.html
   - Data from 2017 to present
   - Contains ensemble mean (geavg) and spread (gespr) products

4. **GEFS Reforecast (2000-2019)**
   S3 Bucket: https://noaa-gefs-retrospective.s3.amazonaws.com/index.html
   - GEFSv12 reforecasts with 31-member ensemble

5. **GribStream API**
   GEFS Atmos Mean: https://gribstream.com/models/gefsatmosmean
   - 0.25° grid ensemble mean fields

6. **Microsoft Planetary Computer**
   URL: https://planetarycomputer.microsoft.com/dataset/storage/noaa-gefs

### 2. SREF (Short Range Ensemble Forecasts)

**Description**: Short-range ensemble over CONUS with multiple models (NMMB, ARW) at various resolutions

**Run cycles**: 03, 09, 15, 21 UTC daily

**Forecast hours**: 0-87 hours

**Resolutions**:
- 40km CONUS (Grid 212)
- 32km (Grid 221)
- 16km North America (Grid 132)
- 45km Alaska (Grid 216)
- 5km CONUS downscaled (Grid 197)

**File Naming Conventions**:
- Individual members: `sref_(nmb|arw).tCCz.pgrb212.PP.grib2`
- **Ensemble Mean**: `sref.tCCz.pgrb212.mean_3hrly.grib2`
- **Ensemble Spread**: `sref.tCCz.pgrb212.spread_3hrly.grib2`
- **Probability**: `sref.tCCz.pgrb212.prob_3hrly.grib2`

Where:
- CC = cycle (03, 09, 15, 21)
- PP = perturbation (ctl, n1-n3, p1-p3)

**Candidate URLs**:

1. **NCEP SREF Products Page**
   URL: https://www.nco.ncep.noaa.gov/pmb/products/sref/
   - Official documentation

2. **SREF NOMADS**
   Main: https://nomads.ncep.noaa.gov/
   - SREF filter: https://nomads.ncep.noaa.gov/gribfilter.php?ds=sref
   - Data directory: `/pub/data/nccf/com/sref/prod/`
   - Structure: `/sref.YYYYMMDD/HH/pgrb/`

**IMPORTANT NOTE**: SREF is scheduled for retirement by NWS - GEFS is recommended as replacement

### 3. NAEFS (North American Ensemble Forecast System)

**Description**: Multi-model ensemble including GEFS, CMC, other members

**Candidate URL**:
- NCEP NAEFS Products: https://www.nco.ncep.noaa.gov/pmb/products/naefs/
- Available in GRIB2 format via FTP

### 4. NCEI Archive (Historical pre-2020)

**Description**: Archived GEFS data before resolution change in September 2020

**Resolutions available**:
- 2.5 Degree (Dataset ID: gov.noaa.ncdc:C00692)
- 1.0 Degree (Dataset ID: gov.noaa.ncdc:C00691)

**URL**: https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast

## PDT 4.1 and 4.8 Documentation

### GRIB2 Code Tables
- PDT 4.1 (Individual ensemble): https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_table4-0.shtml
- PDT 4.8 (Statistical processing): https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml
- Statistical processing types: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml
  - 0: Unweighted Mean of All Members
  - 1: Weighted Mean of All Members
  - 2: Standard Deviation with respect to Cluster Mean

### Ensemble Processing Formulas
Documentation: https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/ens_processing.html
- Ensemble mean: em = sum(x(i))/n
- Ensemble spread (RMSE): sqrt(sum((x(i)-em)**2)/n)

## Summary of Accessible Data

| Product | Source | URLs | Resolution | Cycles |
|---------|--------|------|------------|--------|
| GEFS Ensemble Mean | AWS S3 | https://noaa-gefs-pds.s3.amazonaws.com/ | 0.5°, 0.25° | 00/06/12/18Z |
| GEFS Ensemble Spread | AWS S3 | https://noaa-gefs-pds.s3.amazonaws.com/ | 0.5°, 0.25° | 00/06/12/18Z |
| GEFS Individual Members | AWS S3 | https://noaa-gefs-pds.s3.amazonaws.com/ | 0.5°, 0.25° | 00/06/12/18Z |
| GEFS Reforecast | S3 | https://noaa-gefs-retrospective.s3.amazonaws.com/ | 0.5° | 00/06/12/18Z |
| SREF Ensemble Mean | NOMADS | https://nomads.ncep.noaa.gov/ | 40km, 16km, 5km | 03/09/15/21Z |
| SREF Ensemble Spread | NOMADS | https://nomads.ncep.noaa.gov/ | 40km, 16km, 5km | 03/09/15/21Z |

## Sources

- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEP SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- [NOMADS](https://nomads.ncep.noaa.gov/)
- [NOAA GEFS on AWS](https://registry.opendata.aws/noaa-gefs/)
- [GEFS Reforecast S3](https://noaa-gefs-retrospective.s3.amazonaws.com/index.html)
- [GribStream GEFS Atmos Mean](https://gribstream.com/models/gefsatmosmean)
- [NCEI GEFS Archive](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [GRIB2 PDT 4.8 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
- [GRIB2 Code Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
- [Ensemble Processing (wgrib2)](https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/ens_processing.html)
