# GEFS Archive URLs and Access Methods

## Task: Identify public archive locations for GEFS (Global Ensemble Forecast System) data

## Primary Archive Locations

### 1. NOMADS (NOAA Operational Model Archive and Distribution System)
**Base URL**: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/`

**Directory Structure**:
```
/prod/
└── gefs.YYYYMMDD/          (e.g., gefs.20260617/)
    └── HH/                 (Forecast cycle: 00, 06, 12, 18)
        └── atmos/          (Atmospheric data)
            └── [GRIB2 files]
```

**Access Method**: HTTPS download
**Data Retention**: ~1 month rolling archive
**Data Size**: Each day's GEFS folder is >100 GB

**Available GEFS Products**:
- GFS Ensemble 0.5 Degree (every 6 hours)
- GFS Ensemble 0.25 Degree (every 6 hours)
- GFS Ensemble Chem 0.5 Degree (every 6 hours)
- GFS Ensemble Chem 0.25 Degree (every 6 hours)
- GFS Ensemble 0.5 Degree Bias-Corrected
- GFS Ensemble NDGD resolution Bias-Corrected

**Dataset Identifiers**:
- `gefs_atmos_0p50a` - 0.5 degree atmospheric
- `gefs_atmos_0p25a` - 0.25 degree atmospheric
- `gefs_chem_0p50` - 0.5 degree chemistry
- `gefs_chem_0p25` - 0.25 degree chemistry

### 2. NOAA Open Data on AWS
**Registry**: https://registry.opendata.aws/noaa-gefs/

**Access Method**: Amazon S3 (cloud access)
**Data Availability**: Recent operational data + some historical
**Cost**: Free (public data)

### 3. NCEI (National Centers for Environmental Information)
**Product Page**: https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast
**Archive**: https://www.ncei.noaa.gov/

**Access Method**: Various (NCEI provides multiple access methods)
**Data Retention**: Long-term archival (NOAA's official archive)
**Archive Size**: >37 petabytes archived, 229 TB added monthly

### 4. GEFS Reforecast V2 Project (Historical Research)
**Download**: https://psl.noaa.gov/forecasts/reforecast2/download.html
**Access Method**: Amazon S3 + NCEI NOMADS
**Data Coverage**: 2000-2019 reforecast data

## Summary Table

| Data Source | Base URL | Time Coverage | Access Method | Retention |
|-------------|----------|---------------|----------------|-----------|
| **NOMADS** | `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/` | ~1 month | HTTPS download | Short-term rolling |
| **AWS Open Data** | https://registry.opendata.aws/noaa-gefs/ | Recent + some historical | Amazon S3 | Varies |
| **NCEI** | https://www.ncei.noaa.gov/ | Long-term historical | Various methods | Permanent archive |
| **Reforecast V2** | https://psl.noaa.gov/forecasts/reforecast2/download.html | 2000-2019 | Amazon S3 | Fixed historical |

## Related Resources
- **NCEP Central Operations**: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- **NOMADS Home**: https://nomads.ncep.noaa.gov/

## Sources
- [NOMADS at NCEP](https://nomads.ncep.noaa.gov/)
- [AWS NOAA GEFS Registry](https://registry.opendata.aws/noaa-gefs/)
- [NCEI Global Ensemble Forecast System](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [GEFS Reforecast V2 Project](https://psl.noaa.gov/forecasts/reforecast2/download.html)
