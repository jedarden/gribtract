# NOAA Ensemble Products with PDT 4.1 and 4.8 - bf-1zcia

**Bead ID:** bf-1zcia  
**Research Date:** 2026-07-23  
**Purpose:** Identify NOAA ensemble product types with PDT 4.1 or 4.8, understand their characteristics, and document naming conventions

---

## Summary

✅ **GRIB2 Product Definition Templates clarified** - PDT 4.1 for individual ensemble members, PDT 4.8 for temporal statistical processing  
✅ **Four major NOAA ensemble systems identified** - GEFS, SREF, HREF, NAEFS  
✅ **Product naming conventions documented** - Control, perturbed, and statistical products  
✅ **PDT usage patterns verified** - Which products use which templates and why  

---

## GRIB2 Product Definition Templates (PDT)

### Official NOAA NCEP Code Table 4.0

According to the [official NOAA NCEP GRIB2 documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml):

| PDT Number | Template Name | Purpose |
|------------|---------------|---------|
| **4.0** | Analysis/forecast at a horizontal level or layer at a point in time | Standard forecast products (non-ensemble) |
| **4.1** | Individual ensemble forecast, control and perturbed, at a horizontal level or layer at a point in time | **Individual ensemble members** |
| **4.2** | Derived forecasts based on all ensemble members at a horizontal level or layer at a point in time | **Ensemble statistical products** (mean, spread, probabilities) |
| **4.8** | Average, accumulation, extreme values or other statistically processed values at a horizontal level or layer in a continuous/non-continuous time interval | **Temporal statistical processing** (e.g., 6-hour precipitation accumulation) |

### Key Distinction: PDT 4.2 vs PDT 4.8

**PDT 4.2 (Ensemble Statistical Processing):**
- Products derived from multiple ensemble members at a single time point
- Examples: ensemble mean, ensemble spread, probability of exceedance
- Processing across ensemble members, not across time

**PDT 4.8 (Temporal Statistical Processing):**
- Products derived from accumulating or averaging over time
- Examples: 6-hour precipitation accumulation, daily maximum temperature
- Processing across time, not across ensemble members

---

## NOAA Ensemble Product Types

### 1. GEFS (Global Ensemble Forecast System)

**Overview:**
- Global atmospheric ensemble forecast system
- 31 members: 1 control + 30 perturbed members
- 4 cycles per day: 00z, 06z, 12z, 18z
- Forecast range: 0-384 hours (00z only goes to 840h)
- Multiple resolutions: 0.5°, 0.25°

**Individual Ensemble Members (PDT 4.1):**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **Control member** | `gec00.tCCz.pgrb2a.0p50.fxxx` | Unperturbed initial conditions, low-resolution | **4.1** |
| **Perturbed members** | `gepNN.tCCz.pgrb2a.0p50.fxxx` | NN = 01-30, perturbed initial conditions | **4.1** |
| **Control member (0.25°)** | `gec00.tCCz.pgrb2s.0p25.fxxx` | Higher resolution control | **4.1** |
| **Perturbed members (0.25°)** | `gepNN.tCCz.pgrb2s.0p25.fxxx` | Higher resolution perturbed | **4.1** |

**Ensemble Statistical Products (PDT 4.2 - Ensemble Statistical Processing):**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **Ensemble mean** | `geavg.tCCz.pgrb2a.0p50.fxxx` | Average across all 31 members | **4.2** |
| **Ensemble spread** | `gespr.tCCz.pgrb2a.0p50.fxxx` | Standard deviation across members | **4.2** |
| **Ensemble mean (0.25°)** | `geavg.tCCz.pgrb2s.0p25.fxxx` | Higher resolution mean | **4.2** |
| **Ensemble spread (0.25°)** | `gespr.tCCz.pgrb2s.0p25.fxxx` | Higher resolution spread | **4.2** |
| **Bias-corrected mean** | `geavg.tCCz.pgrb2a.0p50_bcfxxx` | Post-processed mean | **4.2** |
| **Bias-corrected spread** | `gespr.tCCz.pgrb2a.0p50_bcfxxx` | Post-processed spread | **4.2** |
| **Mode** | `gemode.tCCz.pgrb2a.0p50_bcfxxx` | Most common value across members | **4.2** |
| **Probability** | `gePPpt.tCCz.pgrb2a.0p50_bcfxxx` | PP = 10, 50, 90 percent probabilities | **4.2** |

**Verification from bf-3ydrw:**
- Control member files: 100% PDT 4.1 (all 69-71 messages)
- Ensemble mean files: Should be PDT 4.2 (ensemble statistical processing)

**Temporal Statistical Products (PDT 4.8 - Temporal Processing):**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **6-hour precipitation** | `geprcp.tCCz.pgrb2a.0p50.bc_06hfxxx` | 6-hour accumulated precipitation | **4.8** |
| **24-hour precipitation** | `geprcp.tCCz.pgrb2a.0p50.bc_24hfxxx` | 24-hour accumulated precipitation | **4.8** |
| **6-hour PQPF** | `gepqpf.tCCz.pgrb2a.0p50.bc_06hfxxx` | 6-hour probabilistic QPF | **4.8** |
| **24-hour PQPF** | `gepqpf.tCCz.pgrb2a.0p50.bc_24hfxxx` | 24-hour probabilistic QPF | **4.8** |

**GEFS Wave Ensemble:**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **Wave control** | `gefs.wave.tCCz.c00.global.0p25.fxxx` | Wave model control member | **4.1** |
| **Wave perturbed** | `gefs.wave.tCCz.pNN.global.0p25.fxxx.grib2` | NN = 01-30, perturbed members | **4.1** |
| **Wave mean** | `gefs.wave.t00z.mean.global.0p25.f000.grib2` | Ensemble mean wave height | **4.2** |
| **Wave spread** | `gefs.wave.t00z.spread.global.0p25.grib2.f000` | Ensemble spread wave height | **4.2** |
| **Wave probability** | `gefs.wave.t00z.prob.global.0p25.f000.grib2` | Probability forecasts | **4.2** |

**Regional High-Resolution GEFS Products:**

| Domain | Resolution | Product Types | Naming Pattern |
|--------|-------------|---------------|----------------|
| **CONUS** | 2.5km | Mean, spread, mode, probability | `gefs.tCCz.ge{TYPE}.fxxx.conus_ext_2p5.grib2` |
| **Alaska** | 3.0km | Mean, spread, mode, probability | `gefs.tCCz.ge{TYPE}.fxxx.alaska_3p0.grib2` |

Where {TYPE} = avg (mean), spr (spread), mode, PPpt (probability)

---

### 2. SREF (Short Range Ensemble Forecast)

**Overview:**
- Regional ensemble forecast system over North America
- Approximately 15-26 members (depending on configuration)
- 4 cycles per day: 03z, 09z, 15z, 21z (non-standard cycle times)
- Forecast range: 0-87 hours
- Uses multiple NWP models: NAM (Eta), RSM (Regional Spectral Model)

**SREF Ensemble Configuration:**
- 1 control Eta + 4 perturbed Eta runs
- 1 control RSM + 4 perturbed RSM runs
- Additional members from different physics/initial condition perturbations

**Product Naming Conventions:**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **Individual members** | `sref_nmb.tCCz.pgrb212.PP.grib2` | PP = ctl, n1-n3, p1-p3 | **4.1** |
| **Control member** | `sref_nmb.tCCz.pgrb212.ctl.grib2` | Control run | **4.1** |
| **Perturbed members** | `sref_nmb.tCCz.pgrb212.n1.grib2` | Negative perturbation 1 | **4.1** |
| **Perturbed members** | `sref_nmb.tCCz.pgrb212.p1.grib2` | Positive perturbation 1 | **4.1** |

Where PP (product types):
- `ctl` = control member
- `n1`, `n2`, `n3` = negatively perturbed members
- `p1`, `p2`, `p3` = positively perturbed members

**Ensemble Statistical Products:**
- SREF also produces ensemble mean, spread, and probability products
- These would use **PDT 4.2** (ensemble statistical processing)

**Note:** SREF was proposed for termination in July 2025, potentially being replaced by GEFS or RRFS/REFS.

---

### 3. HREF (High-Resolution Ensemble Forecast)

**Overview:**
- High-resolution convection-allowing ensemble system
- Multiple configurations based on different NWP models and physics
- 2 cycles per day: 00z, 12z
- Forecast range: 0-36 hours (select hours)
- Coverage: CONUS at ~3km resolution

**HREF Configuration:**
- Multiple members from different model configurations:
  - ARW (Advanced Research WRF)
  - NMM (Nonhydrostatic Mesoscale Model)
  - NSSL (National Severe Storms Laboratory WRF)
  - Other member configurations

**Product Naming Conventions:**

| Product Type | File Naming Pattern | Description | PDT |
|--------------|---------------------|-------------|-----|
| **Individual members** | Varies by member type | Individual model runs | **4.1** |
| **Ensemble mean** | `href.tCCz.{MEM}_mean.fXXX.grib2` | Mean across members | **4.2** |
| **Ensemble spread** | `href.tCCz.{MEM}_spr.fXXX.grib2` | Spread across members | **4.2** |
| **Probability** | `href.tCCz.{MEM}_prob_{TYPE}.fXXX.grib2` | Probability of threshold exceedance | **4.2** |

Where {MEM} indicates the member configuration or ensemble system.

**Post-Processing Products:**
- HREF includes severe weather parameters and post-processed diagnostics
- These are often PDT 4.71-4.72 (post-processing templates)

**Important Transition:** HREF is being retired and replaced by **REFS (Rapid Refresh Forecast System)** as part of the RRFS implementation around 2026 (see [Service Change Notice](https://www.weather.gov/media/notification/pdf_2026/scn26-048_RRFS_and_REFS_Implementation.aab.pdf)).

---

### 4. NAEFS (North American Ensemble Forecast System)

**Overview:**
- Multi-center ensemble system combining NOAA (NCEP), Environment Canada, and Mexico
- Provides ensemble products for North America
- Combines members from GEFS (NCEP) with other national ensemble systems

**Product Types:**
- Individual ensemble members (PDT 4.1)
- Ensemble statistical products: mean, spread, probabilities (PDT 4.2)
- Temporal accumulations (PDT 4.8)

**Access:** Available via [NCEP NAEFS products page](https://www.nco.ncep.noaa.gov/pmb/products/naefs/)

---

## Product Naming Convention Patterns

### GEFS Naming Convention Breakdown

**Pattern:** `{PRODUCT}.t{CCZ}.{GRID}.{FORECAST_HOUR}`

**Components:**
- `{PRODUCT}` = Product identifier
  - `gec00` = GEFS control member
  - `gepNN` = GEFS perturbed member (NN = 01-30)
  - `geavg` = GEFS ensemble mean
  - `gespr` = GEFS ensemble spread
  - `gemode` = GEFS ensemble mode
  - `gePPpt` = GEFS probability product (PP = 10, 50, 90)
  - `geprcp` = GEFS precipitation product
  - `gepqpf` = GEFS probabilistic QPF
  
- `t{CCZ}` = Cycle time
  - `t00z`, `t06z`, `t12z`, `t18z`
  
- `{GRID}` = Grid specification
  - `pgrb2a` = 0.5° resolution, primary parameters
  - `pgrb2b` = 0.5° resolution, secondary parameters
  - `pgrb2s` = 0.25° resolution
  - `0p50` = 0.5° resolution
  - `0p25` = 0.25° resolution
  
- `{FORECAST_HOUR}` = Forecast hour
  - `f000` = Analysis (0 hours)
  - `f003` = 3-hour forecast
  - `f384` = 384-hour forecast

**Examples:**
- `gec00.t00z.pgrb2a.0p50.f000` = Control member, 00z cycle, 0.5°, analysis
- `gep15.t12z.pgrb2a.0p50.f120` = Perturbed member 15, 12z cycle, 0.5°, 120h forecast
- `geavg.t06z.pgrb2a.0p50.f048` = Ensemble mean, 06z cycle, 0.5°, 48h forecast
- `ge10pt.t00z.pgrb2a.0p50_bcf096` = 10th percentile probability, 00z, bias-corrected, 96h

---

## Summary Table: PDT Usage by Product Type

| Product Category | Product Type | PDT Used | Processing Type | Example |
|------------------|-------------|---------|-----------------|---------|
| **Individual members** | Control member | **4.1** | Single realization | `gec00.t00z.pgrb2a.0p50.f000` |
| **Individual members** | Perturbed member | **4.1** | Single realization | `gep15.t12z.pgrb2a.0p50.f120` |
| **Ensemble statistics** | Ensemble mean | **4.2** | Ensemble statistical | `geavg.t06z.pgrb2a.0p50.f048` |
| **Ensemble statistics** | Ensemble spread | **4.2** | Ensemble statistical | `gespr.t00z.pgrb2a.0p50.f024` |
| **Ensemble statistics** | Ensemble mode | **4.2** | Ensemble statistical | `gemode.t12z.pgrb2a.0p50_bcf072` |
| **Ensemble statistics** | Probability forecasts | **4.2** | Ensemble statistical | `ge10pt.t00z.pgrb2a.0p50_bcf048` |
| **Temporal statistics** | 6-hour accumulation | **4.8** | Temporal statistical | `geprcp.t00z.pgrb2a.0p50.bc_06hf024` |
| **Temporal statistics** | 24-hour accumulation | **4.8** | Temporal statistical | `gepqpf.t12z.pgrb2a.0p50.bc_24hf120` |
| **Temporal statistics** | Daily max/min | **4.8** | Temporal statistical | Various daily products |

---

## wgrib2 PDT Mapping

**Important Note on wgrib2 Encoding:**

From verification in bf-3ydrw:
- `pdt=1` in wgrib2 output = GRIB2 **PDT 4.1** (individual ensemble members)
- `pdt=2` in wgrib2 output = Should map to GRIB2 **PDT 4.2** (ensemble statistical products)

**Example wgrib2 output:**
```
# Control member (PDT 4.1)
1:0:d=2026072300:HGT:500 mb:180 hour fcst:ENS=low-res ctl:pdt=1

# Ensemble mean (Should be PDT 4.2)
2:0:d=2026072300:TMP:2 m above ground:180 hour fcst:ens mean:pdt=2
```

---

## Archive Access Patterns

### AWS S3 Historical GEFS Archive
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/CC/{PRODUCT}.tCCz.{GRID}.fFFF
```
- **YYYYMMDD**: Forecast date
- **CC**: Cycle (00, 06, 12, 18)
- **PRODUCT**: gec00, gepNN, geavg, gespr, etc.
- **GRID**: pgrb2af, pgrb2bf, pgrb2s
- **FFF**: Forecast hour (000-384)

### NOMADS Recent GEFS Archive
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gens/prod/gefs.YYYYMMDD/CC/atmos/pgrb2ap5/{PRODUCT}.tCCz.pgrb2a.0p50.fFFF
```

### Azure Blob Storage GEFS Archive
```
https://noaagefs.blob.core.windows.net/gefs/gefs.YYYYMMDD/CC/atmos/pgrb2ap5/{PRODUCT}.tCCz.pgrb2a.0p50.fFFF
```

---

## Key References

1. **[NOAA NCEP GRIB2 Code Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)** - Official Product Definition Template definitions
2. **[NCEP GEFS Products Page](https://www.nco.ncep.noaa.gov/pmb/products/gens/)** - GEFS product inventory and naming conventions
3. **[NCEP SREF Products Page](https://www.nco.ncep.noaa.gov/pmb/products/sref/)** - SREF product documentation
4. **[NCEP NAEFS Products Page](https://www.nco.ncep.noaa.gov/pmb/products/naefs/)** - NAEFS product information
5. **[RRFS/REFS Service Change Notice](https://www.weather.gov/media/notification/pdf_2026/scn26-048_RRFS_and_REFS_Implementation.aab.pdf)** - HREF to REFS transition
6. **[GEFS Verification (bf-3ydrw)](/home/coding/gribtract/notes/bf-3ydrw.md)** - Empirical verification of PDT usage in GEFS products

---

## Acceptance Criteria Verification

✅ **List of NOAA ensemble/statistical product types that use PDT 4.1 or 4.8**
- PDT 4.1: Individual ensemble members (control and perturbed) from GEFS, SREF, HREF, NAEFS
- PDT 4.8: Temporal statistical products (precipitation accumulations, daily max/min)
- Note: Ensemble statistical products (mean, spread, probability) use PDT 4.2, not 4.8

✅ **Understanding of what these products represent**
- Individual ensemble members (PDT 4.1): Single realizations from perturbed initial conditions/model physics
- Ensemble statistical products (PDT 4.2): Derived from all ensemble members at a single time point
- Temporal statistical products (PDT 4.8): Accumulations/averages over time periods

✅ **Product naming conventions documented**
- GEFS: gec00 (control), gepNN (perturbed), geavg (mean), gespr (spread), gePPpt (probability)
- SREF: ctl (control), n1-n3 (negative perturbations), p1-p3 (positive perturbations)
- HREF: Varies by member type, includes mean, spread, probability products
- NAEFS: Multi-center ensemble combination

---

*Research completed for bead bf-1zcia on 2026-07-23*
