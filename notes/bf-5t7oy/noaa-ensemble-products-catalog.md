# NOAA Ensemble and Statistical Post-Processing Products Catalog

*Research compiled: 2026-07-23*

## Overview

This document catalogs the major NOAA ensemble forecast systems and statistical post-processing products. Ensemble forecasting provides probabilistic guidance by running multiple model simulations to account for uncertainty in initial conditions and model physics.

---

## Global/Continental Scale Systems

### GEFS - Global Ensemble Forecast System

| Attribute | Specification |
|-----------|---------------|
| **Full Name** | Global Ensemble Forecast System |
| **Operator** | NOAA/NCEP (National Centers for Environmental Prediction) |
| **Coverage** | Global |
| **Spatial Resolution** | 0.25° × 0.25° (v12), 0.5° × 0.5° options |
| **Forecast Range** | 16 days (v12), 35 days planned (v13) |
| **Temporal Output** | 3-hourly (first 8-10 days), 6-hourly (remainder) |
| **Update Frequency** | 4 times daily (00, 06, 12, 18 UTC) |
| **Ensemble Members** | 21 total (1 control + 20 perturbed in v12; 1 control + 30 perturbed in v13) |
| **Current Version** | GEFS v12 (operational), v13 in development |

**Description:** NOAA's primary global ensemble prediction system. Provides probabilistic forecasts for medium-range to subseasonal timescales. GEFSv13 will be a 6-way coupled system including atmosphere, ocean, and other components.

**Sources:**
- [NCEI Global Ensemble Forecast](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [GEFSv12 OSTI Modeling](https://vlab.noaa.gov/web/osti-modeling/gefsv12)
- [NCEP Central Operations GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)

---

### NAEFS - North American Ensemble Forecast System

| Attribute | Specification |
|-----------|---------------|
| **Full Name** | North American Ensemble Forecast System |
| **Operators** | NOAA/NWS (US), ECCC (Canada), Mexico (collaborative) |
| **Coverage** | North America |
| **Forecast Range** | 1-14 days |
| **Update Frequency** | 4 times daily (aligned with GEFS cycles) |
| **Ensemble Members** | 52 total (31 from NOAA GEFS + 21 from Canada GEPS) |
| **Type** | Multi-national grand ensemble |

**Description:** A collaborative ensemble system combining NOAA's GEFS (31 members: 30 perturbed + 1 control) with Environment and Climate Change Canada's GEPS (21 members). Provides probabilistic weather forecast guidance across North America.

**Products Available:**
- Experimental 8-14 Day Outlook (temperature/precipitation)
- Ensemble Situational Awareness Table
- Probabilistic Wind Speed Guidance (marine)

**Sources:**
- [Environment Canada NAEFS Portal](https://weather.gc.ca/ensemble/naefs/index_e.html)
- [NOAA CPC NAEFS Outlook](https://www.cpc.ncep.noaa.gov/products/predictions/short_range/NAEFS/NAEFS_Temp_About.html)
- [Ocean Weather Probabilistic Guidance](https://ocean.weather.gov/prob_guidance.php?model=naefs)

---

## Regional/Storm-Scale Systems

### HREF - High Resolution Ensemble Forecast (v3)

| Attribute | Specification |
|-----------|---------------|
| **Full Name** | High Resolution Ensemble Forecast version 3 |
| **Operator** | NOAA/SPC (Storm Prediction Center) |
| **Coverage** | CONUS (Continental United States) |
| **Spatial Resolution** | ~3 km (convection-allowing) |
| **Forecast Range** | 48 hours |
| **Update Frequency** | Twice daily |
| **Ensemble Members** | 10 total (5 deterministic CAM + 5 time-lagged) |
| **Dynamical Cores** | WRF-ARW, FV3, HRRR |
| **Type** | Convection-allowing model (CAM) ensemble |

**Description:** An operational convection-allowing ensemble system designed for storm-scale prediction of severe weather, heavy precipitation, and aviation hazards. HREFv3 is the operational version of the SPC Storm Scale Ensemble of Opportunity (SSEO).

**Member Configuration:**
- Multiple dynamical cores (ARW, FV3, HRRR)
- Varying physics options (PBL schemes, microphysics)
- Time-lagged members for ensemble spread

**Note:** The NMMB dynamical core from HREFv2 was replaced with FV3 in HREFv3.

**Sources:**
- [SPC HREF Ensemble Viewer](https://www.spc.noaa.gov/exper/href/)
- [Weather.gov HREF Upgrade Announcement](https://www.weather.gov/news/211205-href-model-upgrade)
- [HREF Technical Documentation (Authorea)](https://www.authorea.com/doi/pdf/10.1002/essoar.10501462.1/v1)

---

### SREF - Short-Range Ensemble Forecast (Legacy)

| Attribute | Specification |
|-----------|---------------|
| **Full Name** | Short-Range Ensemble Forecast System |
| **Operator** | NOAA/NCEP |
| **Coverage** | North America (US, Canada, Mexico, Eastern Pacific, Western Atlantic) |
| **Forecast Range** | 0-3 days |
| **Temporal Resolution** | Up to 10-minute intervals |
| **Ensemble Members** | 21 (4 regional models, multi-IC and multi-physics) |
| **Status** | **⚠️ Proposed for termination July 25, 2025** |

**Description:** A regional ensemble prediction system designed for short-range forecasting using multicore and multiphysics approaches. Provided high-temporal-resolution forecasts for severe weather and convective outlooks.

**Important Note:** On July 25, 2025, NOAA/NWS issued a proposal to terminate SREF, with GEFS suggested as the replacement system. Users should transition to GEFS or other ensemble products.

**Sources:**
- [SPC SREF Archive](https://www.spc.noaa.gov/exper/sref/)

---

## Statistical Post-Processing Systems

### NBM - National Blend of Models

| Attribute | Specification |
|-----------|---------------|
| **Full Name** | National Blend of Models |
| **Operator** | NOAA/NWS |
| **Coverage** | CONUS (Continental United States) |
| **Type** | Statistical post-processing / multi-model blend |
| **Input Models** | ~30 different NWP models (NWS and non-NWS) |
| **Output** | Calibrated, statistically post-processed guidance |
| **Resolution** | High-resolution gridded output |

**Description:** NOAA's flagship post-processing system that blends approximately 30 numerical weather prediction models from both NWS and non-NWS sources. Provides nationwide consistent and skillful forecast guidance through sophisticated statistical blending algorithms.

**Key Features:**
- Multi-model ensemble blending
- Statistical post-processing for calibration
- Consistent nationwide coverage (CONUS core grid)
- Supports probabilistic precipitation forecasting
- Recent versions (3.1, 3.2) include improved blending techniques

**Resources:**
- [NBM Dashboard](https://blend.mdl.nws.noaa.gov/nbm-dashboard)
- Documentation on field-selected algorithms
- Peer-reviewed research (2017 Meteorological Applications, 90+ citations)

**Sources:**
- [NBM CONUS Core Grid](https://gribstream.com/models/nbm)

---

## Comparison Summary

| Product | Scale | Members | Resolution | Range | Status |
|---------|-------|---------|-------------|-------|--------|
| **GEFS** | Global | 21 (v12) / 31 (v13) | 0.25° | 16 days | Operational |
| **NAEFS** | North America | 52 | Varies | 14 days | Operational |
| **HREFv3** | CONUS | 10 | 3 km | 48 hours | Operational |
| **SREF** | North America | 21 | Regional | 3 days | ⚠️ Legacy (proposed termination) |
| **NBM** | CONUS | ~30 models | High-res | Varies | Operational |

---

## Notes

1. **MEPS (Mesoscale Ensemble Prediction System)** is a Japan Meteorological Agency (JMA) product, not a NOAA system.

2. **Future Developments:**
   - GEFSv13 will extend forecast range to 35 days with 31 ensemble members
   - RRFS (Rapid Refresh Forecast System) Ensemble is in development as a successor to some legacy systems
   - HREF is expected to evolve toward RRFS Ensemble Forecast System (REFS)

3. **Access:**
   - Most products available via NOAA NCEP data servers
   - Visualization tools available through SPC and other NOAA portals
   - GRIB2 data downloads available for research and operational use

---

*Sources:*
- NOAA NCEI, NCEP, SPC, and CPC official documentation
- peer-reviewed AMS journals and conference proceedings
- NOAA Spring Forecasting Experiment materials
- Weather.gov and Ocean.weather.gov product pages
