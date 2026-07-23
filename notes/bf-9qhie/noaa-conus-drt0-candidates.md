# NOAA GRIB2 CONUS DRT=0 Research

**Bead:** bf-9qhie
**Date:** 2026-07-23
**Researcher:** Claude Code Agent

## Executive Summary

This document catalogs NOAA GRIB2 archives covering CONUS (Continental United States) that may use DRT=0 (Data Representation Type 0) simple packing. DRT=0 corresponds to "Grid Point Data - Simple Packing" using GRIB2 Template 5.0.

## Key Finding: Simple Packing (DRT=0) vs Complex Packing

Based on [NCEP GRIB2 documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/):
- **DRT=0**: Grid Point Data - Simple Packing (Template 5.0)
- **DRT=2**: Grid Point Data - Complex Packing (Template 5.2)
- **DRT=3**: Grid Point Data - Complex Packing + Spatial Differencing (Template 5.3)

> **Note:** NDFD specifically uses complex packing (DRT=2/5.3), not simple packing. This is documented in [NDFD GRIB2 Encoding Details](https://graphical.weather.gov/docs/grib_design.html).

## Candidate URLs for DRT=0 Testing

### 1. NAM CONUS (12km) - Primary Candidate

**Product:** North American Mesoscale Model CONUS (12km resolution)
**Source:** [NCEP Central Operations](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
**Archive:** [NOMADS](https://nomads.ncep.noaa.gov/)

#### Access URLs

**HTTPS Direct Download:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/
```

**FTP Alternative:**
```
ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/nam/prod/
```

**GRIB Filter (for subsetting):**
```
https://nomads.ncep.noaa.gov/gribfilter.php?ds=nam
```

#### Provenance Metadata

| Field | Value |
|-------|-------|
| **Agency** | NOAA/NCEP |
| **Model** | NAM (North American Mesoscale) |
| **Domain** | CONUS (12km Lambert Conformal) |
| **Format** | GRIB2 |
| **Update Frequency** | Every 6 hours (00z, 06z, 12z, 18z) |
| **Resolution** | 12 km horizontal |
| **Forecast Hours** | Typically 0-84 hours |
| **File Naming** | `nam.tCCz.awp130fFFYY.grb2` (CC=cycle, FF=forecast hour, YY=extension) |

#### DRT=0 Likelihood: **HIGH**

NAM is a well-established operational model that predates complex packing standards. Older NAM files and basic meteorological fields (temperature, pressure, humidity) may still use simple packing.

#### Sample File Path (Recent Run)

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20260723/nam.t12z.awp130f00.grib2
```

---

### 2. RAP CONUS (13km) - Strong Candidate

**Product:** Rapid Refresh Model CONUS (13km resolution)
**Source:** [NCEP Central Operations - RAP](https://www.nco.ncep.noaa.gov/pmb/products/rap/)
**Archive:** [NOAA Rapid Refresh Portal](https://rapidrefresh.noaa.gov/)

#### Access URLs

**HTTPS Direct Download:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/prod/
```

**FTP Alternative:**
```
ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/rap/prod/
```

**Cloud Archives:**
- [AWS NOAA RAP Registry](https://registry.opendata.aws/noaa-rap/)
- [Microsoft Planetary Computer](https://planetarycomputer.microsoft.com/dataset/storage/noaa-rap)

#### Provenance Metadata

| Field | Value |
|-------|-------|
| **Agency** | NOAA/NCEP |
| **Model** | RAP (Rapid Refresh) |
| **Domain** | CONUS (13km Lambert Conformal) |
| **Format** | GRIB2 |
| **Update Frequency** | Hourly (00z-23z) |
| **Resolution** | 13 km horizontal |
| **Forecast Hours** | 0-21 (standard), 0-51 (extended at 03, 09, 15, 21z) |
| **File Naming** | `rap.tCCz.awp130pgrbfFF.grib2` |

#### DRT=0 Likelihood: **MEDIUM-HIGH**

RAP is a frequently-updated regional model. While newer compression methods exist, the high update frequency (hourly) and operational nature suggest simple packing may still be used for core fields.

#### Sample File Path (Recent Run)

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/prod/rap.20260723/rap.t12z.awp130pgrbf00.grib2
```

---

### 3. HRRR CONUS (3km) - Moderate Candidate

**Product:** High-Resolution Rapid Refresh CONUS (3km resolution)
**Source:** [NCEP Central Operations - HRRR](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
**Archive:** [HRRR Portal](https://rapidrefresh.noaa.gov/hrrr/)

#### Access URLs

**HTTPS Direct Download:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/
```

**FTP Alternative:**
```
ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/
```

**Cloud Archives:**
- [AWS](https://registry.opendata.aws/noaa-hrrr/)
- [Google Cloud](https://cloud.google.com/storage/docs/public-datasets/hrrr)

#### Provenance Metadata

| Field | Value |
|-------|-------|
| **Agency** | NOAA/NCEP / NOAA/GSD |
| **Model** | HRRR (High-Resolution Rapid Refresh) |
| **Domain** | CONUS (3km Lambert Conformal) |
| **Format** | GRIB2 |
| **Update Frequency** | Hourly (00z-23z) |
| **Resolution** | 3 km horizontal |
| **Forecast Hours** | 0-18 (standard), 0-48 (extended at 00, 06, 12, 18z) |
| **File Types** | `wrfprs` (pressure levels), `wrfsfc` (surface), `wrfnat` (native), `wrfsubh` (sub-hourly) |
| **File Naming** | `hrrr.tCCz.wrfprsfFF.grib2` |

#### DRT=0 Likelihood: **MEDIUM**

HRRR is a newer, high-resolution model (3km) designed for convection-allowing forecasts. Higher resolution and newer operational date (2014+) increase likelihood of JPEG2000 or complex packing, but basic fields may still use DRT=0.

#### Sample File Path (Recent Run)

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.20260723/hrrr.t12z.wrfsf00.grib2
```

---

### 4. GFS CONUS Subset - Alternative Candidate

**Product:** Global Forecast System (CONUS subset via latitude/lon bounds)
**Source:** [NCEP Central Operations - GFS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
**Archive:** [NOMADS](https://nomads.ncep.noaa.gov/)

#### Access URLs

**HTTPS Direct Download:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
```

**FTP Alternative:**
```
ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
```

**GRIB Filter (for CONUS subsetting):**
```
https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl
```

#### Provenance Metadata

| Field | Value |
|-------|-------|
| **Agency** | NOAA/NCEP |
| **Model** | GFS (Global Forecast System) |
| **Domain** | Global (CONUS via subsetting) |
| **Format** | GRIB2 |
| **Update Frequency** | Every 6 hours (00z, 06z, 12z, 18z) |
| **Resolutions** | 0.25°, 0.5°, 1.0° |
| **Forecast Hours** | 0-384 (16 days) |
| **File Naming** | `gfs.tCCz.pgrb2.0p25.fFFF` |

#### DRT=0 Likelihood: **VARIABLE**

GFS is a legacy global model with extensive history. Older fields (temperature, geopotential height) may use simple packing, while newer fields and higher resolutions may use complex packing or JPEG2000.

> **Note:** GFS files are global. CONUS data must be extracted via latitude/lon bounding box during post-processing.

---

### 5. NBE (National Blend of Models) - Exploratory Candidate

**Product:** National Blend of Models (formerly National Digital Forecast Database)
**Source:** [NCEP Central Operations](https://www.nco.ncep.noaa.gov/pmb/products/nbm/)

#### Access URLs

```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nbm/prod/
```

#### Provenance Metadata

| Field | Value |
|-------|-------|
| **Agency** | NOAA/NWS |
| **Model** | NBM (National Blend of Models) |
| **Domain** | CONUS, Alaska, Hawaii, Guam, Puerto Rico |
| **Format** | GRIB2 |
| **Update Frequency** | Every 6 hours |
| **Resolution** | ~2.5km CONUS |

#### DRT=0 Likelihood: **LOW**

NBM is a newer product designed to replace legacy NDFD. Given its recent operational history, it likely uses modern packing methods.

---

## Tools for DRT Verification

### wgrib2 (Recommended)

```bash
# Check DRT/packing type of a GRIB2 file
wgrib2 -packing input.grib2

# Output example for DRT=0:
# packing=grid point data - simple packing

# Full inventory with DRT info
wgrib2 -inventory input.grib2
```

**Source:** [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)

### grib_ls (ECCODES)

```bash
grib_ls -l packingType,input.grib2
```

---

## Testing Recommendations

To verify DRT=0 usage in these archives:

1. **Download sample files** from each candidate product
2. **Check with wgrib2**: `wgrib2 -packing sample.grib2`
3. **Document field-specific packing**: Different meteorological fields may use different DRTs within the same file
4. **Test across time periods**: Older archive files are more likely to use simple packing

---

## Additional Resources

### Documentation
- [NCEP GRIB2 Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml) - DRT definitions
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/) - Complete GRIB2 reference
- [wgrib2 Packing Options](https://wgrib2-docs.readthedocs.io/en/latest/options/inventory/packing/) - Packing verification

### Data Portals
- [NOMADS](https://nomads.ncep.noaa.gov/) - Primary model data access
- [READY Gridded Archives](https://www.ready.noaa.gov/archives.php) - Historical data
- [NCEI](https://www.ncei.noaa.gov/) - NOAA archive access

### Cloud Archives
- [AWS NOAA RAP](https://registry.opendata.aws/noaa-rap/)
- [AWS NOAA HRRR](https://registry.opendata.aws/noaa-hrrr/)

---

## Sources

1. [NCEP Data Products - GFS and GDAS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
2. [NCEP Data Products - RAP](https://www.nco.ncep.noaa.gov/pmb/products/rap/)
3. [NCEP Data Products - HRRR](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
4. [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
5. [NCEP GRIB2 Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml)
6. [NOAA Rapid Refresh Portal](https://rapidrefresh.noaa.gov/)
7. [NOAA HRRR Portal](https://rapidrefresh.noaa.gov/hrrr/)
8. [NOMADS Data Server](https://nomads.ncep.noaa.gov/)
9. [NDFD GRIB2 Encoding Details](https://graphical.weather.gov/docs/grib_design.html)
10. [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
11. [READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
12. [NWS NDFD ArcGIS Item](https://www.arcgis.com/home/item.html?id=d0b15c37c20745649598ff0326aa55d0)
13. [NDFD GRIB2 Download](https://tgftp.nws.noaa.gov/SL.us008001/ST.opnl/DF.gr2/DC.ndfd/)
14. [AWS NOAA RAP Registry](https://registry.opendata.aws/noaa-rap/)
15. [Microsoft Planetary Computer - RAP](https://planetarycomputer.microsoft.com/dataset/storage/noaa-rap)
