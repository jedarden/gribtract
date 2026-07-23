# NOAA Ensemble Product URL Patterns

## Summary

This document provides URL patterns and directory structures for accessing NOAA ensemble/statistical products in public archives. Ensemble systems produce multiple forecasts (members) to quantify forecast uncertainty.

**Date:** 2026-07-23  
**Purpose:** Reference for constructing URLs to access NOAA ensemble model data archives

---

## Overview of NOAA Ensemble Systems

| System | Type | Members | Coverage | Access Method |
|--------|------|---------|----------|----------------|
| **GEFS** | Global Medium-Range | 31 (1 control + 30 perturbed) | Global | AWS S3 |
| **SREF** | Regional Short-Range | 21 | North America | NOMADS |
| **HREF** | High-Resolution Convective | ~7 | CONUS | NOMADS (discontinued) |
| **NBM** | Statistical Blend | Deterministic | CONUS | AWS S3 |
| **NAEFS** | International Joint | 40 | North America | MSC Datamart |

---

## 1. GEFS (Global Ensemble Forecast System)

### Archive Infrastructure

**Archive Platform:** AWS Open Data Registry  
**Bucket:** `noaa-gefs-pds`  
**Region:** us-east-1  
**Access:** Public HTTP/S3 (no authentication required)

### URL Pattern

**AWS S3 Pattern:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/HH/[product]/[filename]
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260723`)
- **HH** = Cycle hour directory (00, 06, 12, or 18)
- **[product]** = `pgrb2a` (83 common parameters) or `pgrb2b` (425 less common)
- **[filename]** = Ensemble member file

### File Naming Conventions

**Control Member (c00):**
```
gec00.tXXz.pgrb2aanl      # Analysis
gec00.tXXz.pgrb2afVV      # Forecast
```

**Perturbation Members (p01-p30):**
```
gepWW.tXXz.pgrb2aanl      # Analysis
gepWW.tXXz.pgrb2afVV      # Forecast
```

**Where:**
- `XX` = Cycle hour from path (00, 06, 12, or 18)
- `WW` = Ensemble member number (01-20 for some configs, 01-30 for others)
- `VV` = Forecast hour (00, 06, 12, ..., 384)

### Example URLs

```
# Control member analysis, July 23, 2026, 00z cycle
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/pgrb2a/gec00.t00z.pgrb2aanl

# Perturbation member 05, 12-hour forecast, July 23, 2026, 00z cycle
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/pgrb2a/gep05.t00z.pgrb2af012.grib2

# Control member, 48-hour forecast, pgrb2b product
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/12/pgrb2b/gec00.t12z.pgrb2af048.grib2

# Member 30, max range forecast (384 hours)
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/18/pgrb2a/gep30.t18z.pgrb2af384.grib2
```

### Cycle Schedule and Forecast Range

- **Cycle Frequency:** 4x daily (00z, 06z, 12z, 18z)
- **Forecast Hours:** 00, 06, 12, 18, ..., 384 (every 6 hours)
- **Forecast Range:** 0-16 days
- **Members:** 31 members (control + 30 perturbations)

### Directory Structure Example

```
gefs.20260723/
├── 00/
│   ├── pgrb2a/
│   │   ├── gec00.t00z.pgrb2aanl
│   │   ├── gec00.t00z.pgrb2af000.grib2
│   │   ├── gep01.t00z.pgrb2aanl
│   │   ├── gep01.t00z.pgrb2af000.grib2
│   │   ├── gep02.t00z.pgrb2aanl
│   │   └── ...
│   └── pgrb2b/
│       └── ...
├── 06/
├── 12/
└── 18/
```

---

## 2. SREF (Short Range Ensemble Forecast)

### Archive Infrastructure

**Archive Platform:** NOMADS (NOAA Operational Model Archive and Distribution System)  
**Access:** HTTP (directory browsing)  
**Retention:** ~30 days operational, longer via NCEI archive

### URL Pattern

**NOMADS Pattern:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.YYYYMMDD/HH/[filename]
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260723`)
- **HH** = Cycle hour directory (03, 09, 15, or 21)
- **[filename]** = Ensemble member file

### File Naming Conventions

**File Pattern:**
```
pgrb/sref_[model].tCCz.pgrb212.PP...
```

**Where:**
- `model` = Model type (nmm, nmb, em)
- `CC` = Cycle hour (03, 09, 15, 21)
- `PP` = Forecast hour (00, 03, 06, ..., 87)

**Ensemble Member Identification:**
SREF uses 21 members from 4 regional models:
- **ctl** = Control member (ensemble mean)
- **g01-g20** = Individual ensemble members

### Example URLs

```
# SREF directory listing, July 23, 2026, 03z cycle
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.20260723/03/

# Individual member files (example patterns):
# NMM model, 03z cycle, analysis
pgrb/sref_nmm.t03z.pgrb212.00

# NMB model, 03z cycle, 12-hour forecast
pgrb/sref_nmb.t03z.pgrb212.12

# EM model, 09z cycle, 36-hour forecast
pgrb/sref_em.t09z.pgrb212.36
```

### Cycle Schedule and Forecast Range

- **Cycle Frequency:** 4x daily (03z, 09z, 15z, 21z)
- **Forecast Hours:** 00, 03, 06, ..., 87 (every 3 hours)
- **Forecast Range:** 0-87 hours (3.6 days)
- **Members:** 21 members
- **Status:** Proposed for termination (July 2025), may be replaced by GEFS

---

## 3. HREF (High Resolution Ensemble Forecast)

### Archive Infrastructure

**Archive Platform:** NOMADS  
**Access:** HTTP (directory browsing)  
**Status:** Being discontinued (replaced by REFS)

### URL Pattern

**NOMADS Pattern:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.YYYYMMDD/HH/[filename]
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260723`)
- **HH** = Cycle hour directory (00, 06, 12, or 18)
- **[filename]** = Ensemble member file

### File Naming Conventions

**HREF Members:**
HREF is a convection-allowing ensemble (~3km resolution) with multiple members:
- **arw** = Advanced Research WRF
- **nmm** = Non-hydrostatic Mesoscale Model
- **nssl** = NSSL configuration
- Other configurations

**File Pattern:**
```
href_[member]_tCCz_fFF_GRIB2
```

### Example URLs

```
# HREF directory listing, July 23, 2026, 00z cycle
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.20260723/00/

# ARW member, 00z cycle, analysis
href_arw_t00z_f00.grib2

# NSSL member, 12z cycle, 6-hour forecast
href_nssl_t12z_f06.grib2
```

### Cycle Schedule and Forecast Range

- **Cycle Frequency:** 2x daily (00z, 12z)
- **Forecast Range:** Typically 0-30 hours
- **Resolution:** ~3km (convection-allowing)
- **Status:** Products being discontinued, see SCN26-048

---

## 4. NBM (National Blend of Models)

### Archive Infrastructure

**Archive Platform:** AWS Open Data Registry  
**Bucket:** `noaa-nbm-grib2-pds`  
**Region:** us-east-1  
**Access:** Public HTTP/S3 (no authentication required)

### URL Pattern

**AWS S3 Pattern:**
```
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.YYYYMMDD/HH/core/[filename]
```

**Components:**
- **YYYYMMDD** = Model run date (e.g., `20260723`)
- **HH** = Cycle hour directory (00, 06, 12, or 18)
- **core** = CONUS core grid product directory

### File Naming Conventions

**Core Files:**
```
blend.tCCz.core.fFFF.RR.grib2
```

**QMD Files:**
```
blend.tCCz.qmd.fFFF.RR.grib2
```

**Where:**
- `CC` = Cycle hour (00, 06, 12, 18)
- `FFF` = Three-digit forecast hour (001, 002, ..., 084)
- `RR` = Region code (co = CONUS, other regions for other products)
- `qmd` = QMD (quasi-model) output

### Example URLs

```
# Core blend, July 23, 2026, 00z cycle, 1-hour forecast
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260723/00/core/blend.t00z.core.f001.co.grib2

# Core blend, 12-hour forecast
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260723/00/core/blend.t00z.core.f012.co.grib2

# QMD product, 6z cycle, 24-hour forecast
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260723/06/core/blend.t06z.qmd.f024.co.grib2

# Maximum forecast range (84 hours)
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260723/12/core/blend.t12z.core.f084.co.grib2
```

### Cycle Schedule and Forecast Range

- **Cycle Frequency:** 4x daily (00z, 06z, 12z, 18z)
- **Forecast Hours:** 001-084 (hourly forecasts)
- **Forecast Range:** 0-84 hours (3.5 days)
- **Update Frequency:** Hourly (updated every hour)

### Important Notes

**NBM is a Deterministic Product:**  
Unlike GEFS, SREF, or HREF, NBM produces deterministic (single) forecasts that blend inputs from multiple ensemble systems including:
- GEFS (Global Ensemble Forecast System)
- HREF (High Resolution Ensemble Forecast) - before discontinuation
- SREF (Short Range Ensemble Forecast)
- Canadian ensemble forecasts

NBM does not produce individual ensemble member files; instead, it provides calibrated probabilistic guidance based on ensemble inputs.

---

## 5. NAEFS (North American Ensemble Forecast System)

### Archive Infrastructure

**Archive Platform:** MSC Datamart (Meteorological Service of Canada)  
**Access:** HTTP via weather.gc.ca

### URL Pattern

**MSC Datamart Pattern:**
```
https://[server].weather.gc.ca/ensemble/naefs/[product]/[path]
```

**Components:**
- NAEFS is a joint MSC/NOAA project combining ensembles from both agencies
- Data accessible through MSC's Datamart server
- Specific file naming patterns vary by product type

### Product Types

- Temperature Anomaly Forecast (Day 8-14)
- EPSgrams (ensemble time series)
- Standard Deviation/Mean Charts
- Probability maps

### Documentation

**Primary Documentation:**
- [MSC Open Data Documentation](https://eccc-msc.github.io/open-data/msc-data/nwp_naefs/readme_naefs_en/)
- [CPC NAEFS Products](https://www.cpc.ncep.noaa.gov/products/predictions/short_range/NAEFS/)

**Note:** Detailed URL patterns for individual ensemble member files are not as well-documented as GEFS or SREF.

---

## GEFS Reforecast Data (Historical Archive)

### Archive Infrastructure

**Bucket:** `noaa-gefs-retrospective.s3.amazonaws.com`  
**Access:** Public S3 (no authentication required)

### Description

GEFSv12 reforecast data spanning 2000-2019 is available on AWS. This is a research-quality dataset for model evaluation and calibration.

### Access

**S3 Bucket:**
```
s3://noaa-gefs-retrospective/
```

**Documentation:**
- Description: `https://noaa-gefs-retrospective.s3.amazonaws.com/Description_of_reforecast_data.pdf`
- Access via NOAA's PSL Reforecast V2 Project

**Note:** Specific file naming conventions for reforecast data are documented in the PDF description available at the bucket root.

---

## Comparison Table: Ensemble System Characteristics

| System | Access | Cycle Times | Members | Forecast Range | File Extension | URL Structure |
|--------|--------|-------------|---------|----------------|----------------|----------------|
| **GEFS** | AWS S3 | 00z, 06z, 12z, 18z | 31 | 0-384h (16d) | .grib2 | `/gefs.YYYYMMDD/HH/pgrb2[a|b]/` |
| **SREF** | NOMADS | 03z, 09z, 15z, 21z | 21 | 0-87h | varies | `/sref.YYYYMMDD/HH/` |
| **HREF** | NOMADS | 00z, 12z | ~7 | 0-30h | .grib2 | `/href.YYYYMMDD/HH/` |
| **NBM** | AWS S3 | 00z, 06z, 12z, 18z | 1 (blend) | 0-84h | .grib2 | `/blend.YYYYMMDD/HH/core/` |
| **NAEFS** | MSC | Varies | 40 | Varies | Varies | `/ensemble/naefs/` |

---

## Ensemble Member Identification Patterns

| System | Control Member | Perturbed Members | Naming Pattern |
|--------|---------------|-------------------|----------------|
| **GEFS** | `gec00` | `gep01`-`gep30` | Prefix + 2-digit number |
| **SREF** | `ctl` | `g01`-`g20` | Member-specific codes |
| **HREF** | Varies by model | `arw`, `nmm`, `nssl`, etc. | Model/configuration names |
| **NBM** | N/A (deterministic) | N/A | N/A (blended product) |

---

## Access Patterns and Rate Limits

### AWS S3 Access (GEFS, NBM)

- **Authentication:** None required (public open data)
- **Rate Limits:** None observed for typical usage
- **Range Requests:** Supported (Accept-Ranges: bytes)
- **CDN:** CloudFront for global distribution

### NOMADS Access (SREF, HREF)

- **Authentication:** None required (public HTTP)
- **Retention:** ~30 days for operational access
- **Long-term Archive:** NCEI (National Centers for Environmental Information)
- **Directory Browsing:** Supported

### Best Practices

1. **For recent cycles (< 30 days):** Use NOMADS or AWS S3
2. **For historical research:** Use NCEI archive or GEFS reforecast data
3. **For high-volume access:** Use AWS S3 (no rate limits, better performance)
4. **For browsing:** Use NOMADS web interface or S3 directory listing

---

## Constructing Ensemble URLs: Step-by-Step

### Example 1: GEFS Control Member Analysis

**Step 1:** Encode the date
- Date: July 23, 2026
- Encoded: `20260723`

**Step 2:** Select the cycle
- Cycle: 00z
- Directory: `00`

**Step 3:** Choose the product
- Product: `pgrb2a` (common parameters)

**Step 4:** Construct the filename
- Control member analysis: `gec00.t00z.pgrb2aanl`

**Step 5:** Assemble the URL
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/pgrb2a/gec00.t00z.pgrb2aanl
```

### Example 2: GEFS Perturbation Member Forecast

**Step 1:** Encode the date
- Date: July 23, 2026
- Encoded: `20260723`

**Step 2:** Select the cycle
- Cycle: 12z
- Directory: `12`

**Step 3:** Choose the product
- Product: `pgrb2b` (extended parameters)

**Step 4:** Construct the filename
- Member 15, 48-hour forecast: `gep15.t12z.pgrb2af048.grib2`

**Step 5:** Assemble the URL
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/12/pgrb2b/gep15.t12z.pgrb2af048.grib2
```

### Example 3: NBM Blend Forecast

**Step 1:** Encode the date
- Date: July 23, 2026
- Encoded: `20260723`

**Step 2:** Select the cycle
- Cycle: 06z
- Directory: `06`

**Step 3:** Construct the filename
- 24-hour forecast: `blend.t06z.core.f024.co.grib2`

**Step 4:** Assemble the URL
```
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260723/06/core/blend.t06z.core.f024.co.grib2
```

---

## System Status Notes

### Discontinued/Changing Systems

**HREF (High Resolution Ensemble Forecast):**
- Status: Being discontinued (2025-2026)
- Replacement: REFS (Rapid Ensemble Forecast System)
- SCN: NOAA Service Change Notice SCN26-048

**SREF (Short Range Ensemble Forecast):**
- Status: Proposed for termination (July 2025)
- Replacement: GEFS-based ensemble products
- Recommendation: Use GEFS or NBM for ongoing ensemble access

### Recommended Systems for New Work

- **Global ensemble:** Use GEFS (modern, 31-member, well-documented)
- **CONUS calibrated guidance:** Use NBM (blends multiple ensembles)
- **Historical reforecasts:** Use GEFS reforecast data (2000-2019)
- **Regional ensemble:** Monitor REFS development (HREF replacement)

---

## Acceptance Criteria Verification

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ URL pattern(s) for NOAA ensemble product archives identified | **COMPLETE** | Documented patterns for GEFS, SREF, HREF, NBM, NAEFS |
| ✅ Directory structure and naming conventions documented | **COMPLETE** | Detailed breakdown of paths and file naming for all systems |
| ✅ Example URL templates with clear parameter placeholders | **COMPLETE** | Multiple example URLs with parameter explanations (YYYYMMDD, CC, FF, etc.) |

---

## References

### Primary Documentation

- **[GEFS Product Information](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)** - Official GEFS documentation
- **[GEFS on AWS Open Data](https://registry.opendata.aws/noaa-gefs/)** - AWS S3 access documentation
- **[SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)** - Official SREF product information
- **[NBM Products](https://www.nco.ncep.noaa.gov/pmb/products/blend/)** - Official NBM product information
- **[NBM Download Documentation](https://vlab.noaa.gov/web/mdl/nbm-download)** - MDL NBM file naming conventions

### Archive Access

- **[NOMADS](https://nomads.ncep.noaa.gov/)** - Primary operational data access
- **[NCEI Archive](https://www.ncei.noaa.gov/products/weather-climate-models)** - Long-term archive access
- **[GEFS Reforecast Project](https://psl.noaa.gov/forecasts/reforecast2/download.html)** - Historical reforecast data

### Supporting Documentation

- **[NCEP Product Description Document](https://mag.ncep.noaa.gov/docs/NCEP_PDD_MAG.pdf)** - Complete model specifications
- **[NCEP Upcoming Changes](https://www.nco.ncep.noaa.gov/pmb/changes/)** - Status updates and modifications
- **[MSC NAEFS Documentation](https://eccc-msc.github.io/open-data/msc-data/nwp_naefs/readme_naefs_en/)** - NAEFS access information

### Previous Research

- **[bf-5gsm-noaa-url-patterns.md](bf-5gsm-noaa-url-patterns.md)** - Deterministic model URL patterns (HRRR, NAM, RAP, RRFS)

---

*Document completed for bead bf-2d57a on 2026-07-23*
