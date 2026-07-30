# NOAA Ensemble GRIB2 Archive Sources

Research findings for NOAA public archive sources containing ensemble/statistical grib2 files with PDT 4.1 or 4.8 messages.

## Summary

NOAA's ensemble forecast products are available through multiple access points. The primary ensemble systems are:

- **GEFS** (Global Ensemble Forecast System) - Global, 0.5° and 0.25° resolution
- **SREF** (Short Range Ensemble Forecast) - CONUS, being replaced by REFS/RRFS
- **NAEFS** (North American Ensemble Forecast System) - Bias-corrected products

## Key Archive URLs

### 1. NOMADS (Primary Access)
**Base URL:** https://nomads.ncep.noaa.gov/

- **GEFS Ensemble:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/
- **NAEFS (Bias-corrected):** https://nomads.ncep.noaa.gov/pub/data/nccf/com/naefs/prod/
- **SREF:** https://nomads.ncep.noaa.gov/gribfilter.php?ds=sref

### 2. AWS Open Data Registry (High-speed bulk access)
**S3 Bucket:** `noaa-gefs-pds`
**Registry:** https://registry.opendata.aws/noaa-gefs/

AWS provides the fastest access for bulk downloads without authentication.

### 3. GEFS Reforecast V2 (Historical 2000-2019)
**URL:** https://noaa-gefs-retrospective.s3.amazonaws.com/index.html
**Info:** https://psl.noaa.gov/forecasts/reforecast2/download.html

### 4. Direct FTP Access
**URL:** ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gens/prod/

## Directory Navigation Patterns

### GEFS Directory Structure

```
/pub/data/nccf/com/gens/prod/
└── gefs.YYYYMMDD/
    └── HH/
        └── [files]
```

Where:
- `YYYYMMDD` = Date stamp (e.g., 20260723)
- `HH` = Cycle time (00, 06, 12, 18 UTC)

### SREF Directory Structure

```
/sref.YYYYMMDD/
└── HH/
    └── pgrb/
        └── [files]
```

Where:
- `HH` = Cycle time (03, 09, 15, 21 UTC)
- Ensemble runs every 6 hours

## Product Definition Templates (PDT)

### PDT 4.1
Individual ensemble forecast, control and perturbed members at a horizontal level or in a horizontal layer at a point in time.

**Documentation:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml

### PDT 4.8
Used for ensemble products in GRIB2 processing (referenced in wgrib2 tools).

## File Naming Conventions

### GEFS Atmospheric (0.5° resolution)

**Pattern:** `ge[member].t[cycle]z.pgrb2a.0p50.f[forecast_hour]`

Where:
- `member` = c00 (control), p01-p30 (perturbed members)
- `cycle` = 00, 06, 12, 18
- `forecast_hour` = 000-840 (3-hour intervals)

### GEFS Atmospheric (0.25° resolution)

**Pattern:** `ge[member].t[cycle]z.pgrb2s.0p25.f[forecast_hour]`

Forecast hours: 000-240

### GEFS Wave

**Pattern:** `gefs.wave.t[cycle]z.c00.global.0p25.f[forecast_hour].grib2`

### NAEFS Bias-Corrected

**Pattern:** `ge[member].t[cycle]z.pgrb2a.0p50_bcf[forecast_hour]`

### SREF (ARW/NMB models)

**Pattern:** `sref_[model].t[cycle]z.pgrb212.[member].f[forecast_hour].grib2`

Where:
- `model` = arw or nmb
- `cycle` = 03, 09, 15, 21
- `member` = ctl (control), n1-n6 (negative), p1-p6 (positive)
- `forecast_hour` = 00-87 (3-hour intervals)

## Candidate Files with Full URLs

### 1. GEFS 0.5° Control Member (Latest)

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260723/00/gec00.t00z.pgrb2a.0p50.f000
```

- **Product:** GEFS Atmospheric 0.5°
- **Member:** Control (c00)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 000 (analysis)
- **Expected PDT:** 4.1 (individual ensemble member)

### 2. GEFS 0.25° Perturbed Member

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260723/00/gep01.t00z.pgrb2s.0p25.f003
```

- **Product:** GEFS Atmospheric 0.25°
- **Member:** Perturbed #1 (p01)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 003 (3-hour forecast)
- **Expected PDT:** 4.1 (individual ensemble member)

### 3. GEFS Wave Ensemble

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260723/00/gefs.wave.t00z.c00.global.0p25.f000.grib2
```

- **Product:** GEFS Wave 0.25°
- **Member:** Control (c00)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 000 (analysis)
- **Expected PDT:** 4.1 (individual ensemble member)

### 4. SREF ARW Control Member

```
https://nomads.ncep.noaa.gov/sref.20260723/03/pgrb/sref_arw.t03z.pgrb212.ctl.f00.grib2
```

- **Product:** SREF ARW Model
- **Member:** Control (ctl)
- **Cycle:** 03 UTC, 2026-07-23
- **Forecast Hour:** 00 (analysis)
- **Expected PDT:** 4.1 (individual ensemble member)

### 5. SREF NMB Perturbed Member

```
https://nomads.ncep.noaa.gov/sref.20260723/03/pgrb/sref_nmb.t03z.pgrb212.p3.f06.grib2
```

- **Product:** SREF NMB Model
- **Member:** Positive perturbation #3 (p3)
- **Cycle:** 03 UTC, 2026-07-23
- **Forecast Hour:** 06 (6-hour forecast)
- **Expected PDT:** 4.1 (individual ensemble member)

## Important Notes

### SREF Decommissioning
According to NOAA Service Change Notice SCN26-48, SREF is being replaced by:
- **REFS** (Rapid Refresh Forecast System)
- **RRFS** (Rapid Refresh Forecast System)

These newer systems will supersede SREF, NAM, HREF, and HiresW products.

### PDT 4.1 Content
PDT 4.1 messages contain ensemble-specific information including:
- Ensemble type (forecast, control, perturbed)
- Ensemble member number
- Total number of ensemble members
- Product definition template for individual forecasts

### Access Tools
- **wgrib2:** For decoding and analyzing GRIB2 files
- **NOMADS gribfilter:** For selecting specific parameters
- **AWS CLI:** `aws s3 cp s3://noaa-gefs-pds/ ...` for bulk downloads

## References

- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NOMADS Information](https://nomads.ncep.noaa.gov/)
- [AWS NOAA GEFS Registry](https://registry.opendata.aws/noaa-gefs/)
- [GEFS Reforecast V2](https://psl.noaa.gov/forecasts/reforecast2/download.html)
- [SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- [NOAA READY Archives](https://www.ready.noaa.gov/archives.php)
