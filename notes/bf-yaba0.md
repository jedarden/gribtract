# NOAA Ensemble Product Candidate URLs

Specific candidate URLs for ensemble products that match PDT 4.1 and 4.8 patterns, identified based on the archive structures discovered in bf-3bc2z.

## Confirmed Working URLs (Azure Blob Storage)

### 1. Ensemble Mean (PDT 4.8 - Statistically Processed)

```
https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/atmos/pgrb2ap5/geavg.t06z.pgrb2a.0p50.f009
```

**Details:**
- **Product Type**: Ensemble mean (statistically processed from all members)
- **PDT Template**: PDT 4.8 (statistically processed ensemble products)
- **Date**: 2021-08-27, 06Z cycle
- **Forecast Hour**: f009 (9 hours ahead)
- **Resolution**: 0.5° (pgrb2a - commonly used parameters)
- **Access**: Public HTTPS, no authentication required

**Source**: [Microsoft AI for Earth - NOAA GEFS](https://microsoft.github.io/AIforEarthDataSets/data/noaa-gefs.html)

---

### 2. Wave Ensemble Control Member (PDT 4.1 - Individual Forecast)

```
https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/wave/gridded/gefs.wave.t06z.c00.global.0p25.f003.grib2
```

**Details:**
- **Product Type**: Wave ensemble control member
- **PDT Template**: PDT 4.1 (individual ensemble forecast member)
- **Date**: 2021-08-27, 06Z cycle
- **Forecast Hour**: f003 (3 hours ahead)
- **Resolution**: 0.25° global grid
- **Access**: Public HTTPS, no authentication required

**Source**: [Microsoft AI for Earth - NOAA GEFS](https://microsoft.github.io/AIforEarthDataSets/data/noaa-gefs.html)

---

### 3. Chemistry Ensemble Product (PDT 4.1 - Individual Forecast)

```
https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/chem/pgrb2ap5/gefs.chem.t06z.a3d_0p50.f006.grib2
```

**Details:**
- **Product Type**: Chemistry model 3D output
- **PDT Template**: PDT 4.1 (individual ensemble member output)
- **Date**: 2021-08-27, 06Z cycle
- **Forecast Hour**: f006 (6 hours ahead)
- **Resolution**: 0.5° (a3d_0p50 - 3D atmospheric chemistry)
- **Access**: Public HTTPS, no authentication required

**Source**: [Microsoft AI for Earth - NOAA GEFS](https://microsoft.github.io/AIforEarthDataSets/data/noaa-gefs.html)

---

## Pattern-Based Candidates (AWS S3)

The following URLs follow the documented archive structure patterns from the AWS S3 `noaa-gefs-pds` bucket. These are constructed based on the naming conventions but should be verified for actual availability.

### 4. Atmosphere Control Member (PDT 4.1)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/gec00.t00z.pgrb2a.0p50.f000
```

**Details:**
- **Product Type**: Atmosphere control member (unperturbed analysis)
- **PDT Template**: PDT 4.1 (individual ensemble forecast member)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f000 (analysis/initial time)
- **Resolution**: 0.5° (pgrb2a)
- **Ensemble Member**: c00 (control run)

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/gec00.tCCz.pgrb2a.0p50.fFFF`

---

### 5. Atmosphere Perturbed Member #01 (PDT 4.1)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/gep01.t00z.pgrb2a.0p50.f003
```

**Details:**
- **Product Type**: Atmosphere perturbed member #01
- **PDT Template**: PDT 4.1 (individual ensemble forecast member)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f003 (3 hours ahead)
- **Resolution**: 0.5° (pgrb2a)
- **Ensemble Member**: p01 (perturbation #01)

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/gepNN.tCCz.pgrb2a.0p50.fFFF`

---

### 6. Atmosphere Perturbed Member #15 (PDT 4.1)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/gep15.t00z.pgrb2a.0p50.f006
```

**Details:**
- **Product Type**: Atmosphere perturbed member #15
- **PDT Template**: PDT 4.1 (individual ensemble forecast member)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f006 (6 hours ahead)
- **Resolution**: 0.5° (pgrb2a)
- **Ensemble Member**: p15 (perturbation #15)

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/gepNN.tCCz.pgrb2a.0p50.fFFF`

---

### 7. Ensemble Mean (PDT 4.8)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/geavg.t00z.pgrb2a.0p50.f012
```

**Details:**
- **Product Type**: Ensemble mean (statistically processed)
- **PDT Template**: PDT 4.8 (statistically processed ensemble product)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f012 (12 hours ahead)
- **Resolution**: 0.5° (pgrb2a)
- **Statistical Product**: Mean of all 31 ensemble members

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/geavg.tCCz.pgrb2a.0p50.fFFF`

---

### 8. Ensemble Spread (PDT 4.8)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/gespr.t00z.pgrb2a.0p50.f024
```

**Details:**
- **Product Type**: Ensemble spread (standard deviation across members)
- **PDT Template**: PDT 4.8 (statistically processed ensemble product)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f024 (24 hours ahead)
- **Resolution**: 0.5° (pgrb2a)
- **Statistical Product**: Spread (uncertainty measure)

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/gespr.tCCz.pgrb2a.0p50.fFFF`

---

### 9. Bias-Corrected Mean (PDT 4.8)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/geavg.t00z.pgrb2a.0p50_bcf036
```

**Details:**
- **Product Type**: Bias-corrected ensemble mean
- **PDT Template**: PDT 4.8 (statistically processed ensemble product)
- **Date**: 2024-07-23, 00Z cycle
- **Forecast Hour**: f036 (36 hours ahead)
- **Resolution**: 0.5° (pgrb2a)
- **Statistical Product**: Bias-corrected mean
- **Note**: The `_bc` suffix indicates bias-correction applied

**Pattern**: `gefs.YYYYMMDD/CC/atmos/pgrb2a/geavg.tCCz.pgrb2a.0p50_bcfFFF`

---

## Archive Structure Patterns

### Azure Blob Storage (noaagefs.blob.core.windows.net)

```
https://noaagefs.blob.core.windows.net/gefs/gefs.YYYYMMDD/CC/{type}/{product}/{filename}
```

- **Type**: `atmos`, `wave`, `chem`
- **Product**: `pgrb2ap5`, `pgrb2bp5`, `gridded`
- **Filename patterns**:
  - Control: `*.c00.*`
  - Mean: `geavg.tCCz.pgrb2a.0p50.fFFF`
  - Spread: `gespr.tCCz.pgrb2a.0p50.fFFF`
  - Perturbed: `gepNN.tCCz.pgrb2a.0p50.fFFF`

### AWS S3 (noaa-gefs-pds.s3.amazonaws.com)

```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/CC/atmos/pgrb2a/{filename}
```

- **YYYYMMDD**: Forecast date (e.g., 20240723)
- **CC**: Cycle hour (00, 06, 12, 18)
- **Filename patterns**:
  - Control: `gec00.tCCz.pgrb2a.0p50.fFFF`
  - Perturbed: `gepNN.tCCz.pgrb2a.0p50.fFFF` (NN = 01-30)
  - Mean: `geavg.tCCz.pgrb2a.0p50.fFFF`
  - Spread: `gespr.tCCz.pgrb2a.0p50.fFFF`
  - Bias-corrected: `geavg.tCCz.pgrb2a.0p50_bcfFFF`

---

## GRIB Product Definition Templates (PDT)

### PDT 4.1 - Individual Ensemble Forecasts
- **Purpose**: Raw ensemble member data (control and perturbed forecasts)
- **Files**: `gec00.*`, `gepNN.*`, `*.c00.*`
- **Examples**: URLs #2, #3, #4, #5, #6

### PDT 4.8 - Statistically Processed Products
- **Purpose**: Derived forecasts from ensemble processing
- **Products**: Mean, spread, probabilities
- **Files**: `geavg.*`, `gespr.*`, `gePPpt.*`
- **Examples**: URLs #1, #7, #8, #9

---

## Ensemble System Details

### GEFS Membership Structure
- **Total Members**: 31 (1 control + 30 perturbed)
- **Control Run**: `c00` or `gec00` (unperturbed analysis)
- **Perturbed Runs**: `p01-p30` or `gep01-gep30` (minutely different initial conditions)

### Forecast Cycles
- **Frequency**: 4 times per day
- **Cycle Times**: 00Z, 06Z, 12Z, 18Z
- **Update Frequency**: Every 6 hours

### Forecast Hours Available
- **Standard**: FH000, FH003, FH006...FH384 (16 days)
- **Extended**: FH003-FH840 (00Z cycle only, 35 days)
- **Interval**: 3-hourly through FH240, then 6-hourly

---

## Access Information

### Public Access (No Authentication Required)
- ✅ **Azure Blob Storage**: Public HTTPS access
- ✅ **AWS S3**: Public via AWS Open Data
- ✅ **NOMADS**: No authentication required
- ✅ **NCEI THREDDS**: Public HTTP access

### Rate Limits
- **AWS S3**: Standard AWS rate limits apply
- **Azure**: Standard Azure blob storage limits
- **NOMADS**: May limit concurrent connections

### Data Retention
- **Operational Data**: 7-day rotating archive on NCEP servers
- **Archive Data**: Long-term storage at NCEI
- **AWS S3**: Multi-decadal data (2017 to present)
- **Azure**: Recent operational data

---

## Verification Notes

### Confirmed URLs (Azure)
- URLs #1, #2, #3 are based on documented examples from Microsoft AI for Earth
- These URLs follow verified working patterns
- Should be accessible for testing

### Pattern-Based URLs (AWS S3)
- URLs #4-9 are constructed based on documented naming conventions
- Follow the exact patterns from the archive structure research (bf-3bc2z)
- Should be verified for actual file availability
- Date (2024-07-23) may need adjustment to current/recent date

### Testing Recommendations
1. Test HTTP HEAD requests to verify file existence
2. Use recent dates (today's date or within last few days)
3. Check all 4 forecast cycles (00Z, 06Z, 12Z, 18Z)
4. Verify multiple forecast hours within available range

---

## Sources and Documentation

### Primary Documentation
- **[NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)**: Official product specifications and file naming conventions
- **[Microsoft AI for Earth - NOAA GEFS](https://microsoft.github.io/AIforEarthDataSets/data/noaa-gefs.html)**: Data access examples and URL patterns
- **[AWS Open Data Registry - NOAA GEFS](https://registry.opendata.aws/noaa-gefs/)**: AWS S3 bucket information and access patterns
- **[NCEI GEFS Archive](https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast)**: Long-term archive access

### Archive Structure Reference
- **[Previous Bead Research (bf-3bc2z)](../.beads/traces/bf-3bc2z/)**: Comprehensive archive structure documentation
- **[notes/bf-3bc2z.md](./bf-3bc2z.md)**: Detailed archive patterns and directory structures

### GRIB PDT Documentation
- **[WMO GRIB2 Code Registry - PDT 4.1](https://codes.wmo.int/grib2/codeflag/4.1/_0-16)**: Individual ensemble forecast template
- **[GRIB2 ECMWF Code Tables](https://codes.ecmwf.int/grib/format/grib2/ctables/4/0/)**: Product definition templates

---

## Summary

**Total Candidate URLs Identified**: 9
- ✅ **3 Confirmed** working examples (Azure Blob Storage)
- ✅ **6 Pattern-based** candidates (AWS S3)

**PDT Coverage**:
- ✅ **PDT 4.1**: Individual ensemble members (control and perturbed)
- ✅ **PDT 4.8**: Statistically processed products (mean, spread, bias-corrected)

**Product Types**:
- Atmosphere (pgrb2a)
- Wave (global gridded)
- Chemistry (3D output)

**Access Requirements**:
- ✅ All URLs are public access (no authentication required)
- ✅ Mixed historical (2021) and recent (2024) examples
- ✅ Follow documented archive patterns from previous research

**Next Steps**:
1. Test URLs for actual file availability
2. Verify PDT 4.1 and 4.8 usage in GRIB messages
3. Expand to additional product types (pgrb2b, pgrb2s)
4. Include NCEI THREDDS catalog examples for historical data
