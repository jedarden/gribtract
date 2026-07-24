# CONUS-Covering NOAA Dataset Catalog

## Task Context
- **Bead ID**: bf-697vy
- **Research Date**: 2026-07-24
- **Purpose**: Comprehensive identification of NOAA GRIB2 products covering the continental United States

## CONUS Geographic Definition

**Bounding Box:**
- **Latitude Range**: 24.0°N to 50.0°N  
- **Longitude Range**: 125.0°W to 67.0°W (235.0°E to 293.0°E in 0-360° notation)

**Coverage Validation**: All datasets listed below have been verified or documented to include full CONUS geographic coverage.

---

## Datasets by Priority for DRT=0 (Simple Packing)

### Priority Tier 1: High DRT=0 Likelihood (Global Models)

These datasets use global lat-lon grids which most commonly employ DRT=0 simple packing.

#### 1. GFS (Global Forecast System)

**Characteristics:**
- **Coverage**: Global (includes CONUS)
- **Grid Type**: Global lat-lon regular grid
- **Resolutions Available**: 
  - 0.25° (high): 1,440 × 721 grid points (24,465 CONUS cells)
  - 0.50° (medium): 720 × 361 grid points (6,201 CONUS cells)
  - 1.00° (standard): 360 × 181 grid points (1,593 CONUS cells)
- **Update Frequency**: 4x daily (00z, 06z, 12z, 18z)
- **Forecast Range**: 0-384 hours (16 days)
- **Packing**: DRT=0 verified for analysis fields and many forecast hours

**URL Patterns:**

**NOMADS (Recent 30 days):**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.{resolution}.f{forecast_hour}
```

**AWS NODD Big Data Program:**
```
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.{resolution}.f{forecast_hour}
```

**Example URLs:**
```
# 0.50° analysis, July 24, 2026, 00z cycle
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000

# 0.25° forecast hour 6, July 24, 2026, 12z cycle
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260724/12/atmos/gfs.t12z.pgrb2.0p25.f006

# 1.00° forecast hour 12, July 23, 2026, 00z cycle
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f012
```

**Where:**
- `YYYYMMDD` = Model run date
- `HH` = Cycle hour (00, 06, 12, or 18)
- `{resolution}` = 0p25, 0p50, or 1p00
- `{forecast_hour}` = 000-384 (3-digit)

**DRT=0 Likelihood**: ⭐⭐⭐⭐⭐ **VERIFIED** - Multiple files confirmed DRT=0 in previous verification

---

#### 2. GEFS (Global Ensemble Forecast System)

**Characteristics:**
- **Coverage**: Global (includes CONUS)
- **Grid Type**: Global lat-lon regular grid
- **Resolution**: 0.50° (720 × 361 grid points, 6,201 CONUS cells)
- **Update Frequency**: 4x daily (00z, 06z, 12z, 18z)
- **Forecast Range**: 0-384 hours (16 days)
- **Members**: 31 ensemble members (control + 30 perturbations)
- **Packing**: DRT=0 verified for ensemble mean products

**URL Patterns:**

**AWS S3 (Primary Access):**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/HH/pgrb2a/geavg.tHHz.pgrb2a.0p50.f{forecast_hour}
```

**Individual Members:**
```
# Control member
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/HH/pgrb2a/gec00.tHHz.pgrb2af{forecast_hour}.grib2

# Perturbation members 01-30
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/HH/pgrb2a/gepWW.tHHz.pgrb2af{forecast_hour}.grib2
```

**Example URLs:**
```
# Ensemble mean analysis, July 24, 2026, 00z
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/pgrb2a/geavg.t00z.pgrb2a.0p50.f000

# Ensemble mean 6-hour forecast
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/pgrb2a/geavg.t00z.pgrb2a.0p50.f006

# Perturbation member 15, 12-hour forecast
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/12/pgrb2a/gep15.t12z.pgrb2af012.grib2
```

**Where:**
- `YYYYMMDD` = Model run date
- `HH` = Cycle hour (00, 06, 12, or 18)
- `WW` = Ensemble member number (01-30)
- `f{forecast_hour}` = f000-f384 (3-digit)

**DRT=0 Likelihood**: ⭐⭐⭐⭐ **VERIFIED** - Ensemble mean files confirmed DRT=0

---

### Priority Tier 2: Medium DRT=0 Likelihood (CONUS Regional Models)

These models are CONUS-specific and may use more complex packing schemes, but analysis fields often use DRT=0.

#### 3. NBM (National Blend of Models)

**Characteristics:**
- **Coverage**: CONUS-specific
- **Grid Type**: CONUS Lambert Conformal or regular lat-lon
- **Resolution**: Variable (approximately 2.5km CONUS grid)
- **Update Frequency**: Hourly (00z-23z)
- **Forecast Range**: 0-84 hours (3.5 days)
- **Cycle Times**: 00z, 06z, 12z, 18z (primary blend cycles)
- **Packing**: Mixed - analysis fields likely DRT=0

**URL Patterns:**

**AWS S3:**
```
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.YYYYMMDD/HH/core/blend.tHHz.core.fFFF.co.grib2
```

**Example URLs:**
```
# Analysis, July 24, 2026, 00z
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260724/00/core/blend.t00z.core.f001.co.grib2

# 12-hour forecast
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260724/00/core/blend.t00z.core.f012.co.grib2

# 48-hour forecast, 12z cycle
https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.20260724/12/core/blend.t12z.core.f048.co.grib2
```

**Where:**
- `YYYYMMDD` = Model run date
- `HH` = Cycle hour (00, 06, 12, 18)
- `FFF` = Forecast hour (001-084, 3-digit)
- `co` = CONUS region code

**DRT=0 Likelihood**: ⭐⭐⭐ **LIKELY** - CONUS-focused, analysis fields typically simple packing

---

#### 4. HRRR (High-Resolution Rapid Refresh)

**Characteristics:**
- **Coverage**: CONUS (Alaska sector also available)
- **Grid Type**: Lambert Conformal Conic (GDT 3.30)
- **Resolution**: 3km × 3km
- **Grid Dimensions**: 1,799 × 1,059 points (~1.9 million points)
- **Update Frequency**: Hourly (00z-23z)
- **Forecast Range**: 0-48 hours
- **Archive Start**: September 30, 2014
- **Packing**: Typically DRT=3 (complex packing + spatial differencing)

**URL Patterns:**

**AWS S3:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/hrrr.tCCz.wrfsfcfFF.grib2
```

**NOMADS:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

**Example URLs:**
```
# Analysis field, July 24, 2026, 00z
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260724/hrrr.t00z.wrfsfcf00.grib2

# 1-hour forecast, 06z cycle
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260724/hrrr.t06z.wrfsfcf01.grib2

# 18-hour forecast
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260724/hrrr.t00z.wrfsfcf18.grib2
```

**Where:**
- `YYYYMMDD` = Model run date
- `CC` = Cycle hour (00-23)
- `FF` = Forecast hour (00-48)

**DRT=0 Likelihood**: ⭐ **UNLIKELY** - HRRR typically uses DRT=3 complex packing

---

#### 5. NAM (North American Mesoscale) - CONUS Nest

**Characteristics:**
- **Coverage**: CONUS regional nest (5km resolution)
- **Grid Type**: Lambert Conformal Conic (GDT 3.30) for regional nests
- **Resolution**: 5km for CONUS nest (Grid 227)
- **Update Frequency**: 4x daily (00z, 06z, 12z, 18z)
- **Forecast Range**: 0-60 hours
- **Packing**: Often DRT=3 for regional nests

**URL Patterns:**

**NCEP Products:**
```
https://www.nco.ncep.noaa.gov/pmb/products/nam/YYYYMMDD/nam.tCCz.conusnest.hiresffh.tm00.grib2
```

**Example URLs:**
```
# Analysis, July 24, 2026, 00z
https://www.nco.ncep.noaa.gov/pmb/products/nam/20260724/nam.t00z.conusnest.hiresf00.tm00.grib2

# 12-hour forecast, 12z cycle
https://www.nco.ncep.noaa.gov/pmb/products/nam/20260724/nam.t12z.conusnest.hiresf12.tm00.grib2
```

**Where:**
- `YYYYMMDD` = Model run date
- `CC` = Cycle hour (00, 06, 12, 18)
- `ffh` = Forecast hour (00-60)

**DRT=0 Likelihood**: ⭐⭐ **POSSIBLE** - Some analysis fields may use DRT=0

---

#### 6. RAP (Rapid Refresh)

**Characteristics:**
- **Coverage**: CONUS
- **Grid Type**: Lambert Conformal Conic (GDT 3.30)
- **Resolution**: 13km
- **Update Frequency**: Hourly (00z-23z)
- **Forecast Range**: 0-21 hours (standard), 0-51 hours (extended cycles)
- **Extended Cycles**: 03z, 09z, 15z, 21z
- **Packing**: Typically DRT=3

**URL Patterns:**

**AWS S3:**
```
https://noaa-rap-pds.s3.amazonaws.com/rap.YYYYMMDD/rap.tCCz.awp130pgrbfFF.grib2
```

**NCEP Products:**
```
https://www.nco.ncep.noaa.gov/pmb/products/rap/YYYYMMDD/rap.tCCz.awp130pgrbfFF.grib2
```

**Example URLs:**
```
# Analysis, July 24, 2026, 00z
https://noaa-rap-pds.s3.amazonaws.com/rap.20260724/rap.t00z.awp130pgrbf00.grib2

# 6-hour forecast
https://noaa-rap-pds.s3.amazonaws.com/rap.20260724/rap.t00z.awp130pgrbf06.grib2
```

**Where:**
- `YYYYMMDD` = Model run date
- `CC` = Cycle hour (00-23)
- `FF` = Forecast hour (00-51)

**DRT=0 Likelihood**: ⭐ **UNLIKELY** - RAP typically uses DRT=3

---

### Priority Tier 3: Specialized/Regional Coverage

#### 7. NAM Regional Nests

**Available Nests:**
- **Alaska Nest**: 6km (Grid 198)
- **Hawaii Nest**: 3km (Grid 196)
- **Puerto Rico Nest**: 3km (Grid 194)
- **Fire Weather Nest**: 1.33km CONUS / 1.5km Alaska

**URL Patterns:**
```
# Alaska
https://www.nco.ncep.noaa.gov/pmb/products/nam/YYYYMMDD/nam.tCCz.alaskanest.hiresffh.tm00.grib2

# Hawaii
https://www.nco.ncep.noaa.gov/pmb/products/nam/YYYYMMDD/nam.tCCz.hawaiinest.hiresffh.tm00.grib2

# Puerto Rico
https://www.nco.ncep.noaa.gov/pmb/products/nam/YYYYMMDD/nam.tCCz.priconest.hiresffh.tm00.grib2

# Fire Weather
https://www.nco.ncep.noaa.gov/pmb/products/nam/YYYYMMDD/nam.tCCz.firewxnest.hiresffh.tm00.grib2
```

**DRT=0 Likelihood**: ⭐⭐ **POSSIBLE** - Varies by nest and field

---

#### 8. SREF (Short Range Ensemble Forecast) - Discontinued 2025

**Status**: Proposed for termination July 2025, being replaced by GEFS-based products

**Characteristics:**
- **Coverage**: North America (includes CONUS)
- **Resolution**: Regional (variable, higher than global models)
- **Members**: 21 members
- **Update Frequency**: 4x daily (03z, 09z, 15z, 21z)
- **Forecast Range**: 0-87 hours

**URL Patterns:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/sref.YYYYMMDD/HH/pgrb/sref_[model].tCCz.pgrb212.PP
```

**DRT=0 Likelihood**: ⚠️ **DISCONTINUING** - Use GEFS instead

---

#### 9. HREF (High Resolution Ensemble Forecast) - Discontinued 2025

**Status**: Being discontinued, replaced by REFS

**Characteristics:**
- **Coverage**: CONUS
- **Resolution**: ~3km (convection-allowing)
- **Members**: ~7 members
- **Update Frequency**: 2x daily (00z, 12z)
- **Forecast Range**: 0-30 hours

**URL Patterns:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/href/prod/href.YYYYMMDD/HH/href_[member]_tCCz_fFF_GRIB2
```

**DRT=0 Likelihood**: ⚠️ **DISCONTINUING** - Monitor REFS development

---

## Archive Access Methods Summary

### Primary Access Platforms

| Platform | URL | Authentication | Retention | Rate Limits |
|----------|-----|----------------|-----------|-------------|
| **AWS NODD** | Various S3 buckets | None | Variable (30 days to years) | None observed |
| **NOMADS** | https://nomads.ncep.noaa.gov | None | ~30 days | None observed |
| **NCEI** | https://www.ncei.noaa.gov | None | Long-term archive | API rate limits |
| **NCEP FTP/HTTP** | https://www.nco.ncep.noaa.gov | None | Days to weeks | None observed |

### URL Template Construction

**Standard Pattern Components:**
- `YYYYMMDD` = Model run date (e.g., `20260724`)
- `HH` or `CC` = Cycle hour in UTC (00, 06, 12, 18)
- `FFF` or `FF` = Forecast hour (000-384, varies by model)
- `resolution` = Grid resolution code (0p25, 0p50, 1p00, etc.)

---

## DRT=0 Priority Summary

### Confirmed DRT=0 Candidates ✅

**Global Models (Highest Priority):**
1. **GFS 0.50°** - Best balance of resolution and file size for CONUS
   - Example: `gfs.t00z.pgrb2.0p50.f000` (146 MB)
   - URL Pattern: `.../gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.0p50.f000`

2. **GFS 0.25°** - Highest resolution verified
   - Example: `gfs.t00z.pgrb2.0p25.f000` (487 MB)
   - URL Pattern: `.../gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.0p25.f000`

3. **GEFS Ensemble Mean 0.50°** - Ensemble-based CONUS coverage
   - Example: `geavg.t00z.pgrb2a.0p50.f000` (14 MB)
   - URL Pattern: `.../gefs.YYYYMMDD/HH/pgrb2a/geavg.tHHz.pgrb2a.0p50.f000`

4. **GFS 1.00°** - Smallest file size
   - Example: `gfs.t00z.pgrb2.1p00.f000` (41 MB)
   - URL Pattern: `.../gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.1p00.f000`

### Likely DRT=0 Candidates 🔍

**CONUS Regional Models:**
5. **NBM Core CONUS** - CONUS-specific blend product
   - URL Pattern: `.../blend.YYYYMMDD/HH/core/blend.tHHz.core.fFFF.co.grib2`
   - Note: Analysis fields most likely to use DRT=0

### Unlikely DRT=0 (Use Different DRTs) ⚠️

- **HRRR** - Typically DRT=3 (complex packing + spatial differencing)
- **RAP** - Typically DRT=3
- **NAM Regional Nests** - Often DRT=3
- **SREF/HREF** - Discontinued systems

---

## Download and Verification Commands

### Sample Download Commands

**GFS 0.50° (Recommended Balance):**
```bash
# Latest 00z analysis
wget https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000

# AWS S3 access (anonymous)
aws s3 cp s3://noaa-gfs-bdp-pds/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000 ./gfs_0p50_analysis.grib2 --no-sign-request
```

**GEFS Ensemble Mean:**
```bash
# Latest ensemble mean analysis
wget https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/pgrb2a/geavg.t00z.pgrb2a.0p50.f000

# AWS S3 access
aws s3 cp s3://noaa-gefs-pds/gefs.20260724/00/pgrb2a/geavg.t00z.pgrb2a.0p50.f000 ./gefs_mean.grib2 --no-sign-request
```

### DRT Verification Commands

**Check packing type:**
```bash
# Using wgrib2
wgrib2 -packing <file.grib2>

# Expected DRT=0 output:
# 5.0.0:merc=None:c3=None
# Data Representation Template 5.0 confirmed - simple packing
```

**Check CONUS coverage:**
```bash
# Using wgrid2 to get grid information
wgrib2 -grid <file.grib2>

# Expected global grid output:
# grid_template=0(lat/lon) nx=720 ny=361 lat_start=90.0 lat_end=-90.0 lon_start=0.0 lon_end=359.75
```

---

## Recommendation Summary

### For CONUS DRT=0 Data Access

**Top Recommendation: GFS 0.50° Analysis**
- **Resolution**: 6,201 CONUS cells (53×117 grid points)
- **File Size**: ~146 MB (manageable)
- **Access**: Multiple sources (AWS S3, NOMADS)
- **Update Frequency**: 4x daily
- **URL Pattern**: `.../gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.0p50.f000`
- **DRT=0 Status**: ✅ Verified

**Alternative Options:**
1. **GFS 0.25°** - Higher resolution, larger files (487 MB)
2. **GEFS Ensemble Mean** - Ensemble-based, smaller files (14 MB)
3. **NBM CONUS** - CONUS-specific, likely DRT=0 for analysis fields

**Future Systems to Monitor:**
- **RRFS (Rapid Refresh Forecast System)** - Next-generation regional model
- **REFS** - Replacement for HREF

---

## Acceptance Criteria Verification

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ List NOAA products covering CONUS geographic area | **COMPLETE** | 9 datasets documented (GFS, GEFS, NBM, HRRR, NAM, RAP, SREF, HREF, NAM nests) |
| ✅ Document grid resolution, coverage extent, and update frequency | **COMPLETE** | Comprehensive specifications for each dataset |
| ✅ Identify specific URLs and URL patterns for file access | **COMPLETE** | URL templates with examples for all major datasets |
| ✅ Prioritize candidates likely to use simple packing (DRT=0) | **COMPLETE** | Priority tiers with verified DRT=0 candidates ranked first |

---

## Sources and References

### Primary Documentation
- [GFS Products - NCEP](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [GEFS Products - NCEP](https://www.nco.ncep.noaa.gov/pmb/products/gefs/)
- [NBM Products - NCEP](https://www.nco.ncep.noaa.gov/pmb/products/blend/)
- [HRRR Archive - NOAA](https://rapidrefresh.noaa.gov/hrrr/)
- [AWS Registry - NOAA](https://registry.opendata.aws/collab/noaa/)

### Archive Access
- [NOMADS Documentation](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [NCEI Model Archive](https://www.ncei.noaa.gov/products/weather-climate-models)

### Previous Research Documentation
- `VERIFIED_DRT0_CONUS_FILES.md` - Verified DRT=0 files with CONUS coverage
- `CONUS_COVERAGE_SUMMARY.md` - CONUS geographic validation results
- `OPTIMAL_DRT0_CONUS_FILE.md` - Optimal file selection analysis
- `docs/research/noaa-regional-model-grib2-archives.md` - Regional model archive patterns
- `docs/research/bf-2d57a-noaa-ensemble-url-patterns.md` - Ensemble system URL patterns

---

*Document completed for bead bf-697vy on 2026-07-24*
