# NOAA Ensemble Products and PDT Documentation Research (bf-3wkqt)

## Summary

Research conducted on NOAA ensemble/statistical GRIB2 products, Product Definition Templates (PDTs), and naming conventions for NAEFS, GEFS, and SREF products.

## Key Resources

### NCEP GRIB2 Documentation
- **Main Documentation**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- **Code Table 4.0 (PDT Numbers)**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml
- **PDT 4.8 (Statistical Processing)**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml
- **Code Table 4.10 (Statistical Processing Types)**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-10.shtml

### Product Pages
- **GEFS**: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- **SREF**: https://www.nco.ncep.noaa.gov/pmb/products/sref/
- **NAEFS**: https://www.nco.ncep.noaa.gov/pmb/products/naefs/
- **NAEFS Situational Awareness**: https://satable.ncep.noaa.gov/naefs/

---

## Product Definition Templates (PDTs) Overview

### PDT 4.1 - Individual Ensemble Forecasts
**Code Table 4.0 Entry**: "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time"

This template is used for:
- Individual ensemble member forecasts
- Both control and perturbed members
- Data at a specific horizontal level/layer and point in time

### PDT 4.8 - Statistical Processing
**Full Name**: "Average, Accumulation and/or Extreme values or other Statistically-processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval"

This template handles:
- **Average** values
- **Accumulation** values  
- **Extreme** values (maximum/minimum)
- **Other statistically-processed** values
- Both discrete sampling (non-zero time increments) and continuous processes (zero increment)
- Multiple nested time ranges (n>1 specifications)

### Other Ensemble-Related PDTs
- **PDT 4.0**: Standard analysis/forecast at a point in time
- **PDT 4.2**: Derived forecasts using all ensemble members at a point in time
- **PDT 4.11**: Individual ensemble forecast over time intervals
- **PDT 4.12**: Derived forecasts from all ensemble members over time intervals
- **PDT 4.117-4.118**: Individual large ensemble forecasts
- **PDT 4.119-4.123**: Probability forecasts from large ensembles
- **PDT 4.137-4.138**: Derived reforecast based on all ensemble members

---

## Statistical Processing Types (Code Table 4.10)

### Basic Statistical Operations
- **0**: Average
- **1**: Accumulation
- **2**: Maximum
- **3**: Minimum
- **4**: Difference (value at end of time range minus value at beginning)
- **5**: Root Mean Square (RMS)
- **6**: Standard Deviation
- **7**: Covariance (temporal variance)
- **8**: Ratio
- **9**: Standardized Anomaly
- **10**: Summation

### Additional Statistical Types
- **11**: Median
- **12**: Mode
- **13**: Variance
- **14**: Coefficient

### Specialized Processing
- **100**: Severity
- **102**: Index processing (e.g., drought indices)
- **191-194**: Return period calculations

### Climatological and Forecast Averages
- **2000-2003**: Climatological Mean Value (various period lengths)
- **2004-2006**: Average of N forecasts (with 6, 12, or 24 hour intervals)
- **2007-2009**: Average of forecast accumulations
- **2010**: Climatological averages of analyses/forecasts one year apart
- **2011**: Climatological Standard Deviation

---

## NOAA Ensemble Product Types

### 1. GEFS (Global Ensemble Forecast System)

**Ensemble Configuration**:
- 31 total members (30 perturbed + 1 control)
- Cycle times: 00, 06, 12, 18 UTC
- Forecast hours: 000-384 (some products extend to 840)

**File Naming Conventions**:

*Individual Members (0.5° resolution)*:
- Control: `gec00.tCCz.pgrb2a.0p50.fxxx`
- Perturbed: `gepNN.tCCz.pgrb2a.0p50.fxxx`
  - `NN` = 01-30 for perturbed members
  - `CC` = cycle time (00, 06, 12, 18)
  - `xxx` = forecast hour (000-384)

*Derived Products (0.5°)*:
- Ensemble mean: `geavg.tCCz.pgrb2a.0p50.fxxx`
- Ensemble spread: `gespr.tCCz.pgrb2a.0p50.fxxx`

*Higher Resolution (0.25°)*:
- Uses `pgrb2s.0p25` or `pgrb2b.0p25` instead of `pgrb2a.0p50`

**PDT Usage**:
- Individual members: **PDT 4.1** (individual ensemble forecast)
- Ensemble mean/spread: **PDT 4.2** (derived from all ensemble members)

### 2. SREF (Short Range Ensemble Forecast)

**Ensemble Configuration**:
- 7 members (1 control + 3 negative perturbations + 3 positive perturbations)
- Cycle times: 03, 09, 15, 21 UTC
- Forecast hours: 00, 03, 06, ..., 87
- Model types: NMMB (nmb) and ARW (arw)

**File Naming Conventions**:

*Individual Members*:
- `sref_[model].t[cycle]z.pgrb[grid].[member].f[xx].grib2`
  - `[model]`: nmb or arw
  - `[cycle]`: 03, 09, 15, 21
  - `[grid]`: pgrb212 (40km), pgrb132 (16km), pgrb221 (32km), etc.
  - `[member]`: ctl, n1, n2, n3, p1, p2, p3
  - `[xx]`: forecast hour

*Member Codes*:
- **ctl**: Control member
- **n1, n2, n3**: Negative perturbations
- **p1, p2, p3**: Positive perturbations

*Statistical Products*:
- `sref.t[cycle]z.pgrb[grid].mean.[interval].grib2` (ensemble mean)
- `sref.t[cycle]z.pgrb[grid].spread.[interval].grib2` (ensemble spread)
- `sref.t[cycle]z.pgrb[grid].prob.[interval].grib2` (probability products)
- `sref.t[cycle]z.pgrb[grid].UC.[interval].grib2` (uncertainty/confidence: Max, Min, Mode, 10, 25, 50, 75, 90%)

**PDT Usage**:
- Individual members: **PDT 4.1** (individual ensemble forecast)
- Mean/spread: **PDT 4.2** (derived from all ensemble members)
- Probability/uncertainty products: **PDT 4.5** (probability forecasts) or **PDT 4.8** (statistical processing)

### 3. NAEFS (North American Ensemble Forecast System)

**Ensemble Configuration**:
- 52 total members
  - 31 GEFS members (30 perturbed + 1 control)
  - 21 GEPS members (Canadian ensemble)
- Cycle times: 00, 06, 12, 18 UTC

**File Naming Conventions**:

*Anomaly Products*:
- `naefs_geavg.tCCz.pgrb2a.0p50_anvfHHH`
  - Anomaly difference between average and climatology

*Climate Percentile*:
- `naefs_geavg.tCCz.pgrb2a.0p50_anfHHH`
  - Climate percentile of NAEFS ensemble mean forecast

*Extreme Forecast Index*:
- `naefs_geefi.tCCz.pgrb2a.0p50_bcfHHH`

*Bias-Corrected Products*:
- Percentile-based: `naefs_ge###.tCCz.pgrb2a.0p50_bcfHHH`
  - `###`: 10pt, 50pt, or 90pt
- Average-based: `naefs_geAAA.tCCz.pgrb2a.0p50_bcfHHH`
  - `AAA`: avg, spr, or mode

*Downscaled Products*:
- CONUS (2.5km): `naefs.tCCz.ge###.fHHH.conus_ext_2p5.grib2`
- Alaska (3km): `naefs.tCCz.ge###.fHHH.ak_ext_3p0.grib2`

**PDT Usage**:
- Individual member contributions: **PDT 4.1** (inherited from GEFS/GEPS)
- Ensemble mean: **PDT 4.2** (derived forecasts)
- Anomaly/EFI products: **PDT 4.8** (statistical processing)
- Percentile products: **PDT 4.6** (percentile forecasts)

---

## PDT Usage Summary

| Product Type | Individual Members | Derived Products | Statistical Products |
|-------------|-------------------|------------------|---------------------|
| **GEFS** | PDT 4.1 | PDT 4.2 (mean, spread) | - |
| **SREF** | PDT 4.1 | PDT 4.2 (mean, spread) | PDT 4.5/4.8 (prob, UC) |
| **NAEFS** | PDT 4.1 (inherited) | PDT 4.2 (mean) | PDT 4.8 (anomaly, EFI) |
| **NAEFS** | - | PDT 4.6 (percentiles) | PDT 4.8 (percentiles) |

### Key Findings:
1. **PDT 4.1** is universally used for individual ensemble member data (control + perturbed)
2. **PDT 4.2** is used for derived products from all ensemble members (mean, spread)
3. **PDT 4.8** is used for statistically-processed products (averages, accumulations, extremes, anomalies)
4. **PDT 4.5** is used for probability forecasts
5. **PDT 4.6** is used for percentile forecasts

---

## Naming Convention Patterns

### GEFS Pattern
```
ge[c00|pNN].tCCz.pgrb[a|b|s].0p[25|50].fXXX
```
- `c00`: control
- `p01-p30`: perturbed members
- `CC`: cycle (00, 06, 12, 18)
- `XXX`: forecast hour

### SREF Pattern
```
sref_[nmb|arw].tCCz.pgrb[grid].[ctl|n1-n3|p1-p3].fXX.grib2
```
- `CC`: cycle (03, 09, 15, 21)
- `XX`: forecast hour
- Member designations: ctl, n1-n3, p1-p3

### NAEFS Pattern
```
naefs_[geavg|ge###].tCCz.pgrb2a.0p50_[type]fHHH
```
- `CC`: cycle (00, 06, 12, 18)
- `HHH`: forecast hour
- `###`: member/percentile type
- Type suffixes: anvf (anomaly), anf (percentile), bcf (bias-corrected)

---

## Additional Notes

1. **Consistency**: All three systems follow similar patterns but have variations specific to their configurations and use cases

2. **PDT Selection**: The choice of PDT depends on:
   - Whether the data represents an individual member or derived/processed product
   - The type of statistical processing applied
   - Temporal characteristics (instantaneous vs. time-interval)

3. **Archive Access**: Products are available via:
   - FTP: `ftp://ftp.ncep.noaa.gov/`
   - HTTPS: `https://www.nco.ncep.noaa.gov/`
   - NOMADS: `https://nomads.ncep.noaa.gov/`

4. **Documentation**: The NCEP PMB (Product Management Branch) maintains the authoritative GRIB2 documentation

---

## Sources

- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NCEP Code Table 4.0 - Product Definition Template Numbers](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
- [NCEP GRIB2 Template 4.8 - Statistical Processing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
- [NCEP Code Table 4.10 - Statistical Processing Types](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-10.shtml)
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEP SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- [NCEP NAEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/naefs/)
- [NAEFS Ensemble Situational Awareness Table](https://satable.ncep.noaa.gov/naefs/)
- [NOAA GEFS Overview (NCEI)](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
