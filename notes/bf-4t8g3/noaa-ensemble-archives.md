# NOAA Ensemble Product Archives

## Overview

This document catalogs available NOAA ensemble/statistical products and their public archive locations, with specific focus on products likely containing GRIB2 Product Definition Templates (PDT) 4.1 and 4.8.

## GRIB2 PDT 4.1 and 4.8

### Product Definition Template 4.1
- **Purpose**: Individual ensemble forecast (control and perturbed members)
- **Usage**: Raw ensemble member output from models like GEFS, SREF
- **WMO Code**: Individual ensemble forecast, derived forecasts based on ensemble member clusters

### Product Definition Template 4.8
- **Purpose**: Average, accumulation, and/or extreme values or other statistically-processed values
- **Usage**: Ensemble-derived statistics (mean, spread, probability products)
- **WMO Code**: Statistical processing at a horizontal level or in a horizontal layer at a point in time

**Sources:**
- [NCO NCEP GRIB2 Documentation - PDT 4.8](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
- [NCO NCEP GRIB2 Documentation - Code Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
- [OGC MetOcean DWG - Vertical Coordinate Reference Systems](https://external.ogc.org/twiki_public/MetOceanDWG/VerticalCRS)

## Ensemble Products

### 1. GEFS (Global Ensemble Forecast System)

**Description:** Global ensemble system with 21-31 members at 0.25°-0.5° resolution.

**Archive URLs:**
- **NOMADS (HTTPS)**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/
- **NCEP FTP/HTTPS**: ftp://ftp.ncep.noaa.gov/ (also available via HTTPS)
- **AWS Reforecast**: https://noaa-gefs-retrospective.s3.amazonaws.com/index.html (GEFSv12, 2000-2019)

**Products with PDT 4.8 (Probability/Statistics):**
- `geavg.tCCz.pgrb2a.0p50.fxxx` - 0.5° Ensemble Mean
- `gespr.tCCz.pgrb2a.0p50.fxxx` - 0.5° Ensemble Spread
- `gePPpt.tCCz.pgrb2a.0p50_bcfxxx` - Bias-Corrected 10%, 50%, 90% Probability
- `gemode.tCCz.pgrb2a.0p50_bcfxxx` - Bias-Corrected Mode
- `gepqpf.tCCz.pgrb2a.0p50.24hfxxx` - Ensemble PQPF (Probabilistic QPF)
- `geprcp.tCCz.pgrb2a.0p50.bc_24hfxxx` - Bias-Corrected Ensemble QPF
- `geefi.tCCz.prgb2a.0p50_bcfHHH` - Extreme Forecast Index

**Products with PDT 4.1 (Individual Members):**
- `gec00.tCCz.pgrb2a.0p50.fxxx` - Control member
- `gepNN.tCCz.pgrb2a.0p50.fxxx` - 30 Perturbed forecasts (NN = p01-p30)

**File Naming Convention:**
- `CC` = Cycle time (00, 06, 12, 18 UTC)
- `xxx` = Forecast hour (000-384)
- `PP` = Probability percentile (10, 50, 90)

**Documentation:**
- [NCEP GEFS Products Page](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEI GEFS Overview](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [GEFS Reforecast V2 Project](https://psl.noaa.gov/forecasts/reforecast2/download.html)

**Download Mechanisms:**
- HTTPS (via NOMADS)
- FTP (legacy, still available)
- AWS S3 (for reforecast data)
- Microsoft Planetary Computer: https://planetarycomputer.microsoft.com/dataset/storage/noaa-gefs

---

### 2. SREF (Short Range Ensemble Forecast)

**Description:** Regional CONUS ensemble with ARW and NMMB cores, runs 4x daily at 03, 09, 15, 21 UTC.

**Archive URLs:**
- **NOMADS (HTTPS)**: https://nomads.ncep.noaa.gov/
- **NCEP Product Page**: https://www.nco.ncep.noaa.gov/pmb/products/sref/

**Products with PDT 4.8 (Probability/Statistics):**
- `sref.tccz.pgrb212.prob_1hrly.grib2` - Probability Products (1-hourly)
- `sref.tccz.pgrb212.prob_3hrly.grib2` - Probability Products (3-hourly)
- `sref.tccz.pgrb212.mean_1hrly.grib2` - Mean Products (1-hourly)
- `sref.tccz.pgrb212.spread_1hrly.grib2` - Spread Products (1-hourly)
- `sref.tccz.pgrb212.UC_1hrly.grib2` - Confidence/Uncertainty (Max, Min, Mode, 10, 25, 50, 75, 90%)
- `sref.tccz.pgrb212_SPC.prob_1hrly.grib2` - Storm Prediction Center Probability Products

**Products with PDT 4.1 (Individual Members):**
- `sref_nmb.tccz.pgrb212.PP.grib2` - NMMB core members (PP = ctl, n1, n2, n3, p1, p2, p3)
- `sref_arw.tccz.pgrb212.PP.grib2` - ARW core members (PP = ctl, n1, n2, n3, p1, p2, p3)

**File Naming Convention:**
- `cc` = Cycle time (03, 09, 15, 21 UTC)
- `xx` = Forecast hour (00-87)

**Documentation:**
- [NCEP SREF Products Page](https://www.nco.ncep.noaa.gov/pmb/products/sref/)

**Download Mechanisms:**
- HTTPS (via NOMADS)
- FTP (legacy)

---

### 3. NAEFS (North American Ensemble Forecast System)

**Description:** Joint US-Canada-Mexico ensemble system combining NCEP GEFS and Canadian ensemble.

**Archive URLs:**
- **NOMADS (HTTPS)**: https://nomads.ncep.noaa.gov/
- **NCEP Product Page**: https://www.nco.ncep.noaa.gov/pmb/products/naefs/

**Documentation:**
- [Climate Prediction Center NAEFS](https://www.cpc.ncep.noaa.gov/products/predictions/short_range/NAEFS/)
- [Environment Canada NAEFS](https://weather.gc.ca/ensemble/naefs/index_e.html)

**Download Mechanisms:**
- HTTPS (via NOMADS)
- FTP (legacy)

---

## General Download Mechanisms

### HTTPS (Primary)
- **Base URL**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/
- **Directory Structure**: `/<model>/prod/<model>.YYYYMMDD/CC/atmos/`
- **Example**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.20260723/12/atmos/

### FTP (Legacy)
- **Base URL**: ftp://ftp.ncep.noaa.gov/
- **Directory**: /pub/data/nccf/com/
- **Note**: NOAA has been transitioning from FTP to HTTPS in recent years

### NOMADS Web Interface
- **URL**: https://nomads.ncep.noaa.gov/
- **Features**: GRIB filter, web services, Live Access Server
- **Guide**: Use inventory links from product pages for parameter selection

### Archive Access
- **READY Archives**: https://www.ready.noaa.gov/archives.php - Gridded data archives by dataset, year, month
- **UCAR Data**: https://data.ucar.edu/ - Historical GFS data
- **NCEI**: https://www.ncei.noaa.gov/ - Long-term archival (37+ PB)

---

## Summary: Products Likely Containing PDT 4.1/4.8

| Product | PDT 4.1 (Members) | PDT 4.8 (Statistics) | Archive Access |
|---------|-------------------|----------------------|----------------|
| GEFS Atmos | gec00.tCCz.pgrb2a.0p50.fxxx<br>gepNN.tCCz.pgrb2a.0p50.fxxx | geavg.tCCz.pgrb2a.0p50.fxxx<br>gespr.tCCz.pgrb2a.0p50.fxxx<br>gePPpt.tCCz.pgrb2a.0p50_bcfxxx<br>gepqpf.tCCz.pgrb2a.0p50.24hfxxx | NOMADS HTTPS<br>FTP |
| GEFS Wave | gefs.wave.tCCz.c00.global.0p25.fxxx<br>gefs.wave.tCCz.pNN.global.0p25.fxxx.grib2 | gefs.wave.t00z.mean.global.0p25.f000.grib2<br>gefs.wave.t00z.spread.global.0p25.grib2.f000<br>gefs.wave.t00z.prob.global.0p25.f000.grib2 | NOMADS HTTPS<br>FTP |
| SREF | sref_nmb.tccz.pgrb212.PP.grib2<br>sref_arw.tccz.pgrb212.PP.grib2 | sref.tccz.pgrb212.prob_3hrly.grib2<br>sref.tccz.pgrb212.mean_3hrly.grib2<br>sref.tccz.pgrb212.spread_3hrly.grib2<br>sref.tccz.pgrb212.UC_3hrly.grib2 | NOMADS HTTPS<br>FTP |
| NAEFS | Individual members | Bias-corrected mean/spread/probability | NOMADS HTTPS<br>FTP |

---

## References

### Documentation
- [NCEP Central Operations - Products Inventory](https://www.nco.ncep.noaa.gov/pmb/products/)
- [NOMADS Documentation](https://nomads.ncep.noaa.gov/)
- [GRIB2 Code Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
- [PDT 4.8 Specification](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
- [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/)

### External Sources
- [WAFC London GRIB2 Dataset Guide](https://www.icao.int/sites/default/files/METP/Documents/WAFC-London-GRIB2_DatasetGuide_May_2025-V1.7.pdf)
- [Microsoft Planetary Computer - NOAA GEFS](https://planetarycomputer.microsoft.com/dataset/storage/noaa-gefs)

---

## Date: 2026-07-23
