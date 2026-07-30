# NOAA DRT=0 GRIB2 Archive URLs and Directory Structures

## Overview

This document researches and documents specific NOAA archive URLs and directory structures that contain GRIB2 files with DRT=0 (Data Representation Type 0, also known as Template 5.0 - Grid Point Data Simple Packing).

**Key Finding:** DRT=0 (Template 5.0) is the standard simple packing method used in GRIB2 files and is the default encoding for most NOAA/NCEP model outputs available through NOMADS archives.

## What is DRT=0?

**DRT=0** refers to **Data Representation Template 5.0** in the GRIB2 specification, which represents "Grid Point Data - Simple Packing." This is:

- The simplest and most common packing method in GRIB2 format
- The GRIB2 equivalent of GRIB1's "grid_simple" packing
- Uses reference values stored as IEEE 32-bit floating-point values
- Template 5.0 includes parameters like reference value, binary scale factor, decimal scale factor, and number of bits per value

**Documentation:**
- [NCEP GRIB2 Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml) - Official DRT definitions
- [GRIB2 Template 5.0 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-0.shtml) - Detailed specifications

## Primary NOAA NOMADS Archive URLs

### 1. GFS (Global Forecast System)
**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/`

**Directory Structure:**
```
/pub/data/nccf/com/gfs/
├── prod/                          # Production data
│   ├── gfs.YYYYMMDD/             # Date-stamped directories
│   │   ├── 00/                    # 00 UTC forecast cycle
│   │   │   ├── atmos/             # Atmospheric data
│   │   │   │   ├── gfs.t00z.pgrb2.0p25.f000
│   │   │   │   ├── gfs.t00z.pgrb2.0p25.f003
│   │   │   │   ├── gfs.t00z.pgrb2.0p50.f000
│   │   │   │   └── gfs.t00z.pgrb2.1p00.f000
│   │   │   └── wave/              # Wave model data
│   │   ├── 06/                    # 06 UTC forecast cycle
│   │   ├── 12/                    # 12 UTC forecast cycle
│   │   └── 18/                    # 18 UTC forecast cycle
│   ├── gdas.YYYYMMDD/             # GDAS (Global Data Assimilation System)
│   └── enkfgdas.YYYYMMDD/         # Ensemble Kalman Filter GDAS
└── v16.3/                         # Version-specific directory
```

**File Naming Convention:**
```
gfs.tCCz.[product].XXXX.fFFF
```
Where:
- `CC` = model cycle runtime (00, 06, 12, 18)
- `XXXX` = resolution code (0p25=0.25°, 0p50=0.50°, 1p00=1.00°)
- `FFF` = forecast hour (000-384)
- `[product]` = pgrb2 (common parameters), pgrb2b (uncommon parameters), pgrb2full (all parameters)

**Example:** `gfs.t00z.pgrb2.0p25.f000` = GFS 00Z cycle, common parameters, 0.25° resolution, analysis (hour 0)

**File Sizes:**
- 0.25° resolution: ~500MB per file
- 0.50° resolution: ~150MB per file  
- 1.00° resolution: ~42MB per file

**Product Documentation:** [NCEP GFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)

### 2. NAM (North American Mesoscale Model)
**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/`

**Directory Structure:**
```
/pub/data/nccf/com/nam/
├── prod/                          # Production data
│   ├── nam.YYYYMMDD/             # Date-stamped directories
│   │   ├── 00/                    # 00 UTC forecast cycle
│   │   ├── 06/                    # 06 UTC forecast cycle
│   │   ├── 12/                    # 12 UTC forecast cycle
│   │   ├── 15/                    # 15 UTC forecast cycle (NAM-specific)
│   │   └── 18/                    # 18 UTC forecast cycle
└── v4.2/                         # Version-specific directory
```

**File Naming Convention:**
```
nam.tCCz.[product][forecast_hour].tm00.grib2
```
Where:
- `CC` = model cycle runtime (00, 06, 12, 15, 18)
- `tm00` = model run time (00)
- `[product]` = product type (afwaca, afwahi, awip3d, awip1200, awip3200, conusnest.hires, alaskanest.hires)
- `[forecast_hour]` = forecast hour (f00-f84)

**Product Types:**
- `afwaca` - Alaska FIRE/Weather (~38-43MB, 3-hourly)
- `afwahi` - Hawaii (~20-23MB, 3-hourly)
- `awip3d` - AWIPS 3D (~11-12MB, hourly through hour 84)
- `awip1200` - AWIPS pressure levels (~28-34MB)
- `awip3200` - AWIPS 3.2km (~48-55MB)
- `conusnest.hires` - CONUS nest high-resolution (~921MB+ for f00)
- `alaskanest.hires` - Alaska nest (~748-805MB)

**Example:** `nam.t00z.awip3df00.tm00.grib2` = NAM 00Z cycle, AWIPS 3D product, analysis

**Product Documentation:** [NCEP NAM Products](https://www.nco.ncep.noaa.gov/pmb/products/nam/)

### 3. RAP (Rapid Refresh) 
**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/rap/`

**Directory Structure:**
```
/pub/data/nccf/com/rap/
├── prod/                          # Production data
│   ├── rap.YYYYMMDD/             # Date-stamped directories
│   │   ├── t00z/                  # 00 UTC forecast cycle
│   │   ├── t01z/                  # 01 UTC forecast cycle
│   │   └── ...                    # (hourly cycles through t23z)
└── v5.1/                         # Version-specific directory
```

**File Naming Convention:**
```
rap.[cycle].[product][forecast_hour].grib2
```
Where:
- `[cycle]` = forecast cycle (t00z, t01z, t02z, ... t23z)
- `[product]` = grid type and resolution
- `[forecast_hour]` = forecast hour (f00-f21)

**Product Types:**
- `awip32` - AWIPS grid 32km (~18-19MB)
- `awp130pgrb` - 130km pressure GRIB (~17-18MB)
- `awp130bgrb` - 130km bogussing/RAOB (~37-39MB)
- `awp200` - 200km (~1.1-1.2MB)
- `awp236pgrb` - 236km pressure (~3.4-3.5MB)
- `awp242` - 242km (~25-27MB)
- `awp243` - 243km (~3.2-3.3MB)
- `awp252pgrb` - 252km pressure (~9.3-9.6MB)
- `wrfnat` - WRF native (~280-301MB, largest files)
- `wrfprs` - WRF pressure (~217-230MB)
- `wrfmsl` - WRF MSL (~48-50MB)

**Technical Details:**
- Format: GRIB2 files
- Coverage: 20km horizontal resolution Lambert Conformal grid covering CONUS
- Data Type: Hourly short-range forecast data
- Forecast length: Up to 21 hours
- Update frequency: Every hour

**Product Documentation:** [NCEP RAP Products](https://www.nco.ncep.noaa.gov/pmb/products/rap/)

### 4. HRRR (High-Resolution Rapid Refresh)
**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/`

**Directory Structure:**
```
/pub/data/nccf/com/hrrr/
├── prod/                          # Production data
│   ├── hrrr.YYYYMMDD/            # Date-stamped directories
│   │   ├── 00/                    # 00 UTC forecast cycle
│   │   ├── 01/                    # 01 UTC forecast cycle
│   │   └── ...                    # (hourly cycles through t23z)
├── v4.1/                         # Version-specific directory
│   └── nwges/hrrrges_sfc/conus/   # Parameter files
└── para/                         # Parameter files
```

**Technical Details:**
- **Format:** GRIB2 files
- **Resolution:** 3km horizontal resolution
- **Coverage:** CONUS (Continental US)
- **Forecast length:** Up to 48 hours
- **Update frequency:** Every hour

**Additional Access:**
- **AWS Open Data:** Available through NOAA's AWS S3 buckets for bulk access
- **University of Utah HRRR Archive:** Historical archive and FAQ

## Historical Context: RUC Model

**Important Note:** The **RUC (Rapid Update Cycle) model was officially decommissioned on May 1, 2012** and replaced by the **RAP (Rapid Refresh)** model.

For historical RUC data, check:
- **NCEI (National Centers for Environmental Information)** archives
- **UCAR/NCAR datasets** that have preserved RUC data
- **NSIDC** for specific RUC-20 datasets

## Identifying DRT=0 in GRIB2 Files

### Using wgrib2 Tools

To check whether a GRIB2 file uses DRT=0 (simple packing):

```bash
# Show data representation template number
wgrib2 -code_table_5.0 gribfile.grb2

# Show packing mode
wgrib2 -packing gribfile.grb2

# Show detailed packing information
wgrib2 -packing -v gribfile.grb2
```

**Expected Output for DRT=0:**
- Data Representation Template Number: `0` (Grid Point Data - Simple Packing)
- Packing mode: `simple`

**Other Packing Types:**
- `complex(1|2|3)` - Complex packing methods
- `jpeg` - JPEG compression
- `aec` - AEC compression  
- `ieee` - IEEE floating point format

### Documentation References

- **NCEP GRIB2 Documentation:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- **wgrib2 Documentation:** https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/long_cmd_list.html
- **GRIB2 Table 5.0:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml

## Data Access Methods

### NOMADS Web Interface
- **Main Server:** https://nomads.ncep.noaa.gov/
- **GFS 0.25° Filter:** https://nomads.ncep.noaa.gov/gribfilter.php?ds=gfs_0p25
- **GFS Data Subset:** https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl
- **RAP Data Subset:** https://nomads.ncep.noaa.gov/cgi-bin/filter_rap.pl

### Direct Download
All archives support direct HTTP(S) access to individual GRIB2 files:
```bash
wget https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000
```

### Index Files (.idx)
Most GRIB2 files have corresponding `.idx` index files that enable selective data subsetting by parameter, level, and region.

### Programmatic Access
- **rNOMADS (R package):** For R users
- **Herbie (Python):** Comprehensive NCEP model data downloader
- **grib-downloader (GitHub):** Daily GRIB data download tool

## Cloud and Bulk Access Options

### AWS Open Data
- **NOAA Big Data Program:** https://registry.opendata.aws/noaa/
- **GFS on AWS:** https://registry.opendata.aws/noaa-gfs-pds/
- **HRRR on AWS:** Available through NOAA's AWS S3 buckets

### Alternative Sources
- **Google Cloud Platform Earth:** Alternative source for NCEP data
- **UCAR Historical Archive:** https://data.ucar.edu/dataset/ncep-gfs-0-25-degree-global-forecast-grids-historical-archive

## Key Findings Summary

1. **DRT=0 is the default:** Most NOAA/NCEP GRIB2 files use DRT=0 (Template 5.0 - simple packing) as their standard encoding method.

2. **Consistent directory structure:** All models follow the `/pub/data/nccf/com/[model]/prod/[model].YYYYMMDD/[cycle]/` pattern.

3. **Comprehensive coverage:** The archives provide data from global models (GFS) to regional models (NAM, RAP, HRRR) covering multiple resolutions and forecast cycles.

4. **Multiple access methods:** Data can be accessed via web interface, direct download, programmatic tools, or cloud platforms.

5. **No special DRT=0 filters:** DRT=0 is the default encoding - no special query parameters or directory naming conventions are needed to identify DRT=0 files.

## Sources and References

- [NOMADS at NCEP](https://nomads.ncep.noaa.gov/) - Main NOMADS server
- [NCEP Data Products GFS](https://www.nco.ncep.noaa.gov/pmb/products/gfs/) - Official GFS documentation
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/) - GRIB2 specifications
- [GRIB2 Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml) - Data Representation Templates
- [wgrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/long_cmd_list.html) - GRIB2 tools
- [NOAA Big Data Program - AWS](https://registry.opendata.aws/noaa/) - Cloud access
- [Herbie Documentation](https://herbie.readthedocs.io/en/2025.11.3/user_guide/background/data_sources.html) - Data sources guide
- [Rapid Refresh/RUC Information](https://www.ncei.noaa.gov/products/weather-climate-models/rapid-refresh-update) - Model history

---
**Document Created:** 2026-07-24  
**Task Reference:** bf-4rf3p  
**Project:** gribtract