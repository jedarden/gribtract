# NOAA Ensemble/Statistical GRIB2 Product Archives

## Task Summary

Research and identification of available NOAA ensemble/statistical GRIB2 products in public archives, focusing on products using Product Definition Templates (PDT) 4.1 and 4.8.

## Key Findings

### Product Definition Templates

**PDT 4.1** - Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time
**PDT 4.8** - Average, Accumulation and/or Extreme values or other Statistically-processed values at a horizontal level or in a horizontal layer

Sources:
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [Template 4.8 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
- [Code Table 4.7](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml) - Contains ensemble statistics entries

Code Table 4.7 ensemble-related entries:
- Code 4: Spread of All Members
- Code 200: Equally Weighted Mean
- Codes 8-9: Minimum/Maximum of All Members
- Codes 193-195, 201-204: Percentile values
- Code 10: Variance
- Code 7: Interquartile range

## Candidate Ensemble Products

### 1. GEFS - Global Ensemble Forecast System

**Description**: Weather model generating 21-31 separate forecasts (1 control + 20-30 perturbed members). Current version runs 31 members (1 control + 30 perturbed).

**Archive URLs**:
- **NCEI Historical Archive** (2008-09-23 to 2020-09-23): Dataset [C00691](https://www.ncei.noaa.gov/access/metadata/landing-page/bin/iso?id=gov.noaa.ncdc:C00691)
  - 1.0° resolution (via HAS system)
  - 2.5° resolution (via HAS system)
- **AWS Open Data Registry** (2017-present): [registry.opendata.aws/noaa-gefs](https://registry.opendata.aws/noaa-gefs/)
- **Microsoft Planetary Computer**: [planetarycomputer.microsoft.com/dataset/storage/noaa-gefs](https://planetarycomputer.microsoft.com/dataset/storage/noaa-gefs)
- **NCEP FTP**: `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gens/prod/`
- **NCEP HTTP**: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/`
- **NOMADS**: [nomads.ncep.noaa.gov](https://nomads.ncep.noaa.gov/)
- **GribStream API**: [gribstream.com/models/gefsatmos](https://gribstream.com/models/gefsatmosmean)

**File Naming Patterns**:

Individual members (likely PDT 4.1):
```
gec00.tCCz.pgrb2a.0p50.fXXX     # Control member (c00)
gepNN.tCCz.pgrb2a.0p50.fXXX     # Perturbed member (NN = 01-30)
```

Statistical products (likely PDT 4.8):
```
geavg.tCCz.pgrb2a.0p50.fXXX     # Ensemble mean
gespr.tCCz.pgrb2a.0p50.fXXX     # Spread
gemode.tCCz.pgrb2a.0p50_bcfXXX  # Mode (bias-corrected)
gefs.tCCz.gePPpt.fXXX.alaska_3p0.grib2      # Probability percentiles
gefs.tCCz.geavg.fXXX.conus_ext_2p5.grib2    # Regional mean
gefs.tCCz.gespr.fXXX.conus_ext_2p5.grib2    # Regional spread
```

Where:
- CC = cycle time (00, 06, 12, 18 UTC)
- XXX = forecast hour (000-384, 00Z extends to 840)
- NN = member number (01-30)
- PP = probability percentile (10, 50, 90)
- 0p50 = 0.5° resolution, 0p25 = 0.25° resolution

**PDT Usage**: 
- Individual ensemble members → PDT 4.1
- Statistical products (mean, spread, probability) → PDT 4.8

**Access Methods**: FTP, HTTPS (HTTP), AWS S3

**Additional Documentation**:
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [GEFS Reforecast V2](https://psl.noaa.gov/forecasts/reforecast2/download.html)

### 2. SREF - Short Range Ensemble Forecast

**Description**: Short-range ensemble forecast system with multiple ARW and NMM members producing forecasts every 6 hours.

**Archive URLs**:
- **NCEP Products Page**: [nco.ncep.noaa.gov/pmb/products/sref/](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- **NOMADS SREF Filter**: [nomads.ncep.noaa.gov/gribfilter.php?ds=sref](https://nomads.ncep.noaa.gov/gribfilter.php?ds=sref)
- **NOMADS Inventory**: [nco.ncep.noaa.gov/pmb/products/sref/nomads/](https://www.nco.ncep.noaa.gov/pmb/products/sref/nomads/)

**File Naming Patterns**:

Individual members (likely PDT 4.1):
```
sref_nmb.tCCz.pgrb212.PP.fFF.grib2      # NMMB model, grid 212
sref_nmm.tCCz.pgrb212.PP.fFF.grib2      # NMM model
sref_arw.tCCz.pgrb212.PP.grib2          # ARW model
```

Statistical products (likely PDT 4.8):
```
sref.tCCz.pgrb212.mean_1hrly.grib2      # Mean (FH00-39, 1-hourly)
sref.tCCz.pgrb212.spread_3hrly.grib2    # Spread (FH00-87, 3-hourly)
sref.tCCz.pgrb212.prob_3hrly.grib2      # Probability (FH00-87, 3-hourly)
```

Where:
- CC = cycle time (03, 09, 15, 21 UTC)
- PP = member/perturbation identifier (ctl, n1-n3, p1-p3)
- FF = forecast hour
- pgrb212/132/221/216/243 = different grid resolutions

**Ensemble Member Designations**:
- `ctl` - control member
- `n1, n2, n3` - negative perturbations
- `p1, p2, p3` - positive perturbations

**PDT Usage**:
- Individual ensemble members → PDT 4.1
- Statistical products (mean, spread, probability) → PDT 4.8

**Access Methods**: FTP, HTTPS (HTTP)

**Additional Resources**:
- Both 32km and 16km North American SREF products available
- Confidence/Uncertainty (UC) products providing Max, Min, Mode and percentiles (10, 25, 50, 75, 90)

### 3. NBM - National Blend of Models

**Description**: Nationally consistent forecast guidance based on a blend of NWS and non-NWS numerical models. While not a traditional ensemble, it's a statistical blend of multiple models.

**Archive URLs**:
- **NBM Download Page**: [vlab.noaa.gov/web/mdl/nbm-download](https://vlab.noaa.gov/web/mdl/nbm-download)
- **AWS Open Data Registry**: [registry.opendata.aws/noaa-nbm](https://registry.opendata.aws/noaa-nbm/)
- **GribStream**: [gribstream.com/models/nbm](https://gribstream.com/models/nbm)
- **Herbie Documentation**: [herbie.readthedocs.io/en/2023.12.4/user_guide/_model_notebooks/nbm.html](https://herbie.readthedocs.io/en/2023.12.4/user_guide/_model_notebooks/nbm.html)

**File Naming Patterns**:

```
blend.tCCz.core.fXXX.RR.grib2          # Standard elements
blend.tCCz.qmd.fXXX.RR.grib2          # QMD elements
blend.t00z.master.f001.co.grib2      # Example: master file, cycle 00Z, forecast hour 001, CONUS
```

Where:
- CC = cycle hour
- XXX = forecast hour
- RR = region code (co=CONUS, ak=Alaska, hi=Hawaii, pr=Puerto Rico, gu=Guam, oc=Oceanic)

**Cycle Times**: Not explicitly listed (verify operational cycles)

**PDT Usage**: NBM uses statistical processing (blending) → likely PDT 4.8 for derived products

**Access Methods**: FTP, HTTPS (HTTP), AWS S3

**Download URLs**:
- FTP: `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/blend/prod/blend.YYYYMMDD/CC/grib2/`
- HTTP: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/blend/prod/blend.YYYYMMDD/CC/core/`
- AWS: `https://noaa-nbm-grib2-pds.s3.amazonaws.com/index.html#blend.YYYYMMDD/CC/grib2/`

**Archive Duration**: Operational products have 1-2 day retention on NCEP servers. AWS archive provides longer retention.

### 4. Additional Products (Noted but Less Critical)

#### URMA - Unrestricted Mesoscale Analysis
- **Description**: Hourly analysis for near-surface weather conditions, used for verification and model calibration
- **Type**: Deterministic analysis (not ensemble), but used for statistical processing
- **Archive URLs**:
  - [NCEP RTMA/URMA Products](https://www.nco.ncep.noaa.gov/pmb/products/rtma/)
  - [AWS Open Data Registry](https://registry.opendata.aws/noaa-rtma/)
  - [GribStream API](https://gribstream.com/models/urma)
- **PDT Usage**: Analysis product → likely PDT 4.0 or 4.1

#### NAM - North American Mesoscale Model
- **Products Page**: [nco.ncep.noaa.gov/pmb/products/nam/](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- Deterministic model (not ensemble), but provides GRIB2 output

#### NMME - North American Multi-Model Ensemble
- **Description**: Seasonal forecasting system with multiple coupled models
- **Access**:
  - [CPC NMME Data](https://www.cpc.ncep.noaa.gov/products/NMME/data.html)
  - [AWS Registry](https://registry.opendata.aws/noaa-nmme/)
- **Note**: Multi-model ensemble for seasonal forecasts (different use case than GEFS/SREF)

## Archive Access Methods

| Product | HTTP | HTTPS | FTP | AWS S3 | NOMADS |
|---------|------|-------|-----|--------|--------|
| GEFS    | ✓    | ✓     | ✓   | ✓      | ✓      |
| SREF    | ✓    | ✓     | ✓   | ✗      | ✓      |
| NBM     | ✓    | ✓     | ✓   | ✓      | ✓      |

## Key Resources

- **NCEP Central Operations**: [www.nco.ncep.noaa.gov/pmb/](https://www.nco.ncep.noaa.gov/pmb/)
- **NOMADS Portal**: [nomads.ncep.noaa.gov](https://nomads.ncep.noaa.gov/)
- **NCEI Products**: [www.ncei.noaa.gov/products/](https://www.ncei.noaa.gov/products/)
- **Unified Post Processor (UPP)**: [www.epic.noaa.gov/unified-post-processor/](https://www.epic.noaa.gov/unified-post-processor/) - Software for generating GRIB2 products from model output
- **Herbie Documentation**: Python tool for downloading GRIB2 model data

## Summary

Three primary NOAA ensemble/statistical GRIB2 products were identified:

1. **GEFS** - Global ensemble with 31 members, statistical products (mean/spread) available
2. **SREF** - Short-range regional ensemble, statistical products (mean/spread/prob) available
3. **NBM** - National blend of models, statistical product blending multiple models

All three products are accessible via public archives (NCEP FTP/HTTP, AWS, NOMADS) and provide both individual member data (PDT 4.1) and statistical processed products (PDT 4.8).

## Sources

- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEP SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- [NCEI GEFS Product Page](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [GEFS Reforecast V2](https://psl.noaa.gov/forecasts/reforecast2/download.html)
- [NOMADS](https://nomads.ncep.noaa.gov/)
- [NBM Download Page](https://vlab.noaa.gov/web/mdl/nbm-download)
- [AWS NOAA GEFS Registry](https://registry.opendata.aws/noaa-gefs/)
- [AWS NOAA NBM Registry](https://registry.opendata.aws/noaa-nbm/)
- [Unified Post Processor](https://www.epic.noaa.gov/unified-post-processor/)
