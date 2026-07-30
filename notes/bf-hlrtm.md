# Documentation of Verified Ensemble Product URLs and Patterns

## Task Completed

Successfully consolidated and documented all verified ensemble product URLs and patterns for PDT 4.1 and 4.8 products in the parent bead bf-v1lrs.

## Work Performed

### 1. Reviewed Previous Research
- Examined notes/bf-yaba0.md which contained 9 candidate URLs for ensemble products
- Identified 3 confirmed working URLs from Azure Blob Storage
- Identified 6 pattern-based candidates from AWS S3

### 2. Consolidated Documentation
Added comprehensive comment (ID: 17) to parent bead bf-v1lrs containing:

#### Verified Working URLs (3 total)
1. **Ensemble Mean (PDT 4.8)** - Azure Blob Storage
   - URL: https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/atmos/pgrb2ap5/geavg.t06z.pgrb2a.0p50.f009
   - Real GRIB2 file, public access

2. **Wave Ensemble Control (PDT 4.1)** - Azure Blob Storage
   - URL: https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/wave/gridded/gefs.wave.t06z.c00.global.0p25.f003.grib2
   - Real GRIB2 file, public access

3. **Chemistry Ensemble (PDT 4.1)** - Azure Blob Storage
   - URL: https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/chem/pgrb2ap5/gefs.chem.t06z.a3d_0p50.f006.grib2
   - Real GRIB2 file, public access

#### URL Pattern Templates
Documented complete URL construction patterns for:
- **Azure Blob Storage**: `https://noaagefs.blob.core.windows.net/gefs/gefs.YYYYMMDD/CC/{type}/{product}/{filename}`
- **AWS S3**: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/CC/atmos/pgrb2a/{filename}`

With detailed filename patterns for:
- Control members: `gec00.tCCz.pgrb2a.0p50.fFFF`
- Perturbed members: `gepNN.tCCz.pgrb2a.0p50.fFFF`
- Ensemble mean: `geavg.tCCz.pgrb2a.0p50.fFFF`
- Ensemble spread: `gespr.tCCz.pgrb2a.0p50.fFFF`

#### Data Center Information
- **Azure Blob Storage**: Microsoft Azure via AI for Earth partnership
  - Public HTTPS access, no authentication
  - Standard Azure rate limits
  - Recent operational data

- **AWS S3**: AWS Open Data Registry
  - Public HTTPS access, no authentication
  - Standard AWS rate limits
  - Multi-decadal data (2017 to present)

#### Ensemble System Details
- 31 total members (1 control + 30 perturbed)
- 4 forecast cycles daily (00Z, 06Z, 12Z, 18Z)
- Forecast hours: FH000-FH384 (standard), extended to FH840 (00Z only)
- PDT 4.1: Individual ensemble forecasts
- PDT 4.8: Statistically processed products

### 3. Verification Criteria Met

All acceptance criteria satisfied:
- ✅ Added comment to parent bead bf-v1lrs with verified working URLs
- ✅ URLs point to real GRIB2 files in NOAA public archives (Azure Blob Storage)
- ✅ Documented URL patterns/templates for constructing similar download URLs
- ✅ Notes on which NOAA data centers host the products
- ✅ Access notes (rate limits, authentication requirements)
- ✅ Documentation is clear enough for others to construct similar URLs

## Acceptance Criteria Status

**All criteria met:**
- [x] Added comment to parent bead bf-v1lrs with verified URLs and patterns
- [x] At least 1 verified working URL for an ensemble product (3 provided)
- [x] URL pattern/template for constructing similar download URLs
- [x] Notes on which NOAA data center hosts the products
- [x] Relevant access notes (rate limits, authentication, etc.)
- [x] URLs point to real GRIB2 files in NOAA public archives
- [x] Documentation is clear enough for others to construct similar URLs

## Related Artifacts

- **Parent bead**: bf-v1lrs - Comment ID 17 added
- **Source documentation**: notes/bf-yaba0.md - 9 candidate URLs documented
- **Archive structure research**: notes/bf-3bc2z.md - Original archive patterns
- **Previous verification**: notes/bf-2ql7w.md - GRIB2 file accessibility verification

## Source References

- Microsoft AI for Earth: https://microsoft.github.io/AIforEarthDataSets/data/noaa-gefs.html
- NCEP Products: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- AWS Registry: https://registry.opendata.aws/noaa-gefs/

## Summary

This final documentation step consolidates all verified ensemble product URL information into the parent bead, providing a complete reference for:
- Working downloadable GRIB2 files for PDT 4.1 and 4.8 ensemble products
- URL construction patterns for both Azure Blob Storage and AWS S3 archives
- Access information and data center details
- Ensemble system configuration and product details

The documentation enables other team members to construct similar URLs and access NOAA ensemble forecast data independently.
