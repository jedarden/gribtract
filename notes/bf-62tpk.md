# NOAA GEFS Ensemble Fixture URLs

## Summary

Research identified public NOAA ensemble/statistical product URLs in GRIB2 format with PDT 4.1 (Individual ensemble forecast) and PDT 4.2 (Derived forecasts based on all ensemble members).

## Primary Data Source

**AWS S3 Bucket:** `noaa-gefs-pds`
- **Browse URL:** https://noaa-gefs-pds.s3.amazonaws.com/index.html
- **AWS CLI:** `aws s3 ls --no-sign-request s3://noaa-gefs-pds/`
- **Region:** us-east-1

## Confirmed Accessible URLs (Verified 2026-07-23)

### Individual Ensemble Forecasts (PDT 4.1)

**Control Member (c00):**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
```
- **File Size:** 13,356,146 bytes (~13.3 MB)
- **PDT:** 4.1 (Individual ensemble forecast at a point in time)
- **Resolution:** 0.5° latitude/longitude grid
- **Forecast Hour:** F000 (analysis/initial time)

**Perturbation Member 01 (p01):**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000
```
- **File Size:** 13,618,045 bytes (~13.6 MB)
- **PDT:** 4.1 (Individual ensemble forecast at a point in time)
- **Resolution:** 0.5° latitude/longitude grid
- **Forecast Hour:** F000 (analysis/initial time)

**Forecast Hour Examples (Control Member):**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f006
```
- **File Size:** 14,959,126 bytes (~15.0 MB)
- **Forecast Hour:** F006 (6-hour forecast)

### Ensemble Mean (PDT 4.2)

**Ensemble Average (avg):**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
```
- **File Size:** 13,664,431 bytes (~13.7 MB)
- **PDT:** 4.2 (Derived forecasts based on all ensemble members at a point in time)
- **Resolution:** 0.5° latitude/longitude grid
- **Forecast Hour:** F000 (analysis/initial time)

## File Naming Convention

```
gefs.YYYYMMDD/CC/atmos/pgrb2ap5/{MEMBER}.tCCz.pgrb2a.0p50.fFFF
```

Where:
- `YYYYMMDD` = Forecast initialization date (e.g., 20240101)
- `CC` = Forecast cycle (00, 06, 12, 18 UTC)
- `MEMBER` = Ensemble member identifier:
  - `gec00` = Control member
  - `gepNN` = Perturbation member (NN = 01-30)
  - `geavg` = Ensemble mean
- `0p50` = 0.5° resolution
- `FFF` = Forecast hour (000, 003, 006, 009, ..., up to 384)

## Ensemble Member Counts

Based on NOAA documentation:
- **1 control member** (c00)
- **30 perturbation members** (p01-p30)
- **1 ensemble mean** (avg)
- **Total:** 32 ensemble products per cycle

## Available Forecast Hours

GRIB2 files are available for forecast hours: F000, F003, F006, F009, F012, F015, F018, F021, F024, F027, ... (continuing in 3-hour intervals up to F384)

## GRIB2 Product Definition Templates (PDT)

Based on NOAA NCEP GRIB2 Code Table 4.0 documentation:

### PDT 4.1 - Individual Ensemble Forecast
Used for: Individual ensemble forecast at a point in time
- **Files:** `gec00.*`, `gepNN.*`
- **Characteristics:** Single ensemble member output

### PDT 4.2 - Derived Ensemble Forecasts
Used for: Derived forecasts based on all ensemble members at a point in time
- **Files:** `geavg.*`
- **Characteristics:** Statistical mean across all ensemble members

### PDT 4.8 - Statistical Processing
Used for: Average, accumulation, extreme or other statistically processed values in a time interval
- Not directly observed in current AWS bucket structure
- Would be used for time-accumulated products (e.g., precipitation accumulation)

## Additional Resources

### Documentation Sources
- [NOAA NCEP GRIB2 Code Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml) - Product Definition Template reference
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/) - Official GEFS product specifications
- [AWS Open Data Registry - NOAA GEFS](https://registry.opendata.aws/noaa-gefs/) - Dataset metadata and access information

### Alternative Access Methods
- **NOMADS:** https://nomads.ncep.noaa.gov/cgi-bin/filter_gens.pl - Web interface for subsetting GRIB2 data
- **Azure:** https://noaagefs.blob.core.windows.net/ - Alternative cloud storage (access varies)

## Verification

All URLs listed above were verified accessible on 2026-07-23 via HTTP HEAD requests returning 200 OK responses with valid Content-Length headers.

## Notes

1. **Index files:** Each GRIB2 file has a corresponding `.idx` file containing byte-range index for efficient subsetting
2. **Resolution:** 0.5° (pgrb2ap5) appears to be the primary resolution available in the public bucket
3. **Secondary parameters:** Additional parameter sets may be available in `pgrb2bp5` directory (less commonly used parameters)
4. **Data retention:** The AWS bucket contains data from 2017 to present (as of 2026-07-23)
