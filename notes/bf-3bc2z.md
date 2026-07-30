# NOAA Ensemble Product Archive Structures

Research findings on how NOAA organizes and stores ensemble forecast products.

## Main NOAA Data Centers Hosting Ensemble Products

### 1. NCEP (National Centers for Environmental Prediction)
- **Primary operational producer** of GEFS ensemble forecasts
- **Access Methods**: NOMADS, AWS S3 (noaa-gefs-pds), HTTPS
- **Documentation**: [NCEP Products Inventory](https://www.nco.ncep.noaa.gov/pmb/products/gens/)

### 2. NCEI (National Centers for Environmental Information)
- **Official archive** for historical model data
- **Access Methods**: THREDDS Data Server, AIRS (Archive Information Request System)
- **Documentation**: [NCEI GEFS Products](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)

### 3. NOMADS (NOAA Operational Model Archive and Distribution System)
- **Web service** for near-real-time data access
- **Access Method**: HTTPS with grib_filter interface
- **URL**: https://nomads.ncep.noaa.gov/

## GRIB Product Definition Templates (PDT) for Ensembles

### PDT 4.1 - Individual Ensemble Forecasts
- **Purpose**: Individual ensemble forecast members (control and perturbed forecasts)
- **Usage**: Raw ensemble member data at horizontal levels/layers
- **Documentation**: [WMO Code Registry](https://codes.wmo.int/grib2/codeflag/4.1/_0-16)

### PDT 4.8 - Statistically Processed Products
- **Purpose**: Statistically processed ensemble products (mean, spread, probabilities)
- **Usage**: Derived forecasts from ensemble processing
- **Documentation**: [COSMO Model Documentation](https://www.cosmo-model.org/content/model/cosmo/grib/pdtemplate_4.11.htm)

## URL Patterns and Directory Structures

### AWS S3 (noaa-gefs-pds) - Primary Public Access
**Bucket**: `noaa-gefs-pds` (us-east-1)

**Directory Structure**:
```
noaa-gefs-pds/gefs.YYYYMMDD/XX/pgrb2a/
noaa-gefs-pds/gefs.YYYYMMDD/XX/pgrb2b/
```

**File Naming Convention**:
- **Control member**: `gec00.tXXz.pgrb2afVV`
- **Perturbed members**: `gepWW.tXXz.pgrb2afVV`
  - `XX` = forecast cycle (00, 06, 12, 18)
  - `WW` = ensemble member (01-30)
  - `VV` = forecast hour (00, 03, 06...384)

**Resolution**:
- `pgrb2a`: ~83 commonly used parameters (0.5° resolution)
- `pgrb2b`: ~425 additional parameters (0.5° resolution)
- `pgrb2s`: Select parameters at 0.25° resolution

### NCEP HTTPS Access (Current)
**Base URL Pattern**: `https://www.nco.ncep.noaa.gov/pmb/products/gens/`

**File Types Available**:
- **GEFS Atmos pgrb2a** (0.5°): Control + 30 perturbed members + mean/spread
- **GEFS Atmos pgrb2b** (0.5°): Secondary parameters
- **GEFS Atmos pgrb2s** (0.25°): Select parameters
- **GEFS Wave** (0.25°): Global wave ensemble
- **GEFS Chem** (0.25°/0.5°): Chemical model output
- **GEFS Bias-Corrected**: Calibrated ensemble products
- **GEFS NDGD**: High-resolution CONUS (2.5km) and Alaska (3km)

**Example Filenames**:
- Control: `gec00.tCCz.pgrb2a.0p50.fxxx`
- Perturbed: `gepNN.tCCz.pgrb2a.0p50.fxxx`
- Mean: `geavg.tCCz.pgrb2a.0p50.fxxx`
- Spread: `gespr.tCCz.pgrb2a.0p50.fxxx`
  - `CC` = cycle (00, 06, 12, 18)
  - `NN` = member number (01-30)
  - `xxx` = forecast hour (000, 003, 006...384)

### NCEI THREDDS Data Server
**Base Catalog**: `https://www.ncei.noaa.gov/thredds/`
**GEFS Catalog**: `https://www.ncei.noaa.gov/thredds/catalog/model-gefs-003/catalog.html`
**Format**: OPeNDAP/HTTP access via THREDDS catalog

### NCEI AIRS (Archive Information Request System)
**URL**: `https://www.ncei.noaa.gov/has/HAS.DsSelect`
**Purpose**: Bulk data requests in native GRIB format

### Historical FTP (Terminated)
**Note**: The old `ftp://ftpprd.ncep.noaa.gov` public FTP service has been **terminated** by NOAA. Use NOMADS or AWS S3 instead.

**Historical Pattern** (for reference):
```
ftp://ftpprd.ncep.noaa.gov/pub/data/nccf/com/gens/prod/prod/gefs.YYYYMMDD/CC/
```

## GEFS Ensemble System Details

### Ensemble Members
- **Total members**: 31 (1 control + 30 perturbed)
- **Control run**: `c00` or `gec00`
- **Perturbed runs**: `p01-p30` or `gep01-gep30`

### Forecast Cycles
- **Frequency**: 4 times per day (every 6 hours)
- **Cycle times**: 00Z, 06Z, 12Z, 18Z

### Forecast Hours
- **Standard**: FH000, FH003, FH006...FH384 (16 days)
- **Extended**: FH003-FH840 (00Z cycle only, 35 days)

### Statistical Products (PDT 4.8)
- **Mean**: Ensemble mean (`geavg`)
- **Spread**: Ensemble spread (`gespr`)
- **Probabilities**: 10%, 50%, 90% percentiles (`ge10pt`, `ge50pt`, `ge90pt`)
- **Mode**: Most likely forecast (`gemode`)

## Access Restrictions and Authentication

### Public Access (No Authentication Required)
- **noaa-gefs-pds S3 bucket**: Fully public via AWS Open Data
- **NOMADS**: No authentication required
- **NCEI THREDDS**: Public HTTP access
- **NCEP HTTPS**: Public HTTPS access

### Rate Limits
- AWS S3: Standard AWS rate limits apply
- NOMADS: May limit concurrent connections

### Data Retention
- **Operational data**: 7-day rotating archive on NCEP servers
- **Archive data**: Long-term storage at NCEI
- **AWS S3**: Multi-decadal reforecast data (2000-2019)

## Additional Resources

### Documentation URLs
- [AWS Registry - NOAA GEFS](https://registry.opendata.aws/noaa-gefs/)
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEI GEFS Archive](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [NOMADS Portal](https://nomads.ncep.noaa.gov/)
- [GRIB2 Code Table 4.0](https://codes.ecmwf.int/grib/format/grib2/ctables/4/0/)
- [WMO GRIB2 Documentation](https://www.wmo.int/pages/prog/www/WMOCodes.html)

### Reforecast Data
- **GEFSv12 Reforecast (2000-2019)**: Available on `noaa-gefs-retrospective` S3 bucket
- **Documentation**: [PDF Description](https://noaa-gefs-retrospective.s3.amazonaws.com/Description_of_reforecast_data.pdf)

## Key Takeaways for gribtract

1. **Primary data source**: `noaa-gefs-pds` S3 bucket is the most reliable public access point
2. **File naming**: Follows consistent patterns with `gec00` (control) and `gepNN` (perturbed) prefixes
3. **PDT identification**: 
   - Raw members use PDT 4.1
   - Statistical products (mean/spread/probabilities) use PDT 4.8
4. **Access method**: S3 HTTP or AWS CLI for direct file access
5. **No authentication**: All public access methods are open

## Sources
- [NCEI GEFS Product Page](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)
- [NCEP Products Inventory](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [AWS Open Data Registry - NOAA GEFS](https://registry.opendata.aws/noaa-gefs/)
- [NOMADS Portal](https://nomads.ncep.noaa.gov/)
- [WMO GRIB2 Code Registry](https://codes.wmo.int/grib2/codeflag/4.1/_0-16)
- [GRIB2 ECMWF Code Tables](https://codes.ecmwf.int/grib/format/grib2/ctables/4/0/)
- [READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
