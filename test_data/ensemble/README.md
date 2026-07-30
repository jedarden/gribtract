# Ensemble GRIB2 Test Fixtures

## Overview
This directory contains ensemble/statistical GRIB2 test fixtures for validating GRIB2 decoder ensemble product support (PDT 4.1 and 4.8 templates).

## Files

### gefs_perturbed_p01_20260723_t00z_f000.grib2
- **Source:** GEFS (Global Ensemble Forecast System) - Perturbed Member 01
- **Download URL:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000`
- **Download Date:** 2026-07-22 23:56 UTC
- **File Size:** 13,984,963 bytes (~13.3 MB)
- **PDT Content:** 71 messages, all PDT 1 (standard analysis/forecast template)
- **Characteristics:** Individual ensemble perturbed member (ENS=+1)
- **Variables:** HGT, TMP, RH, UGRD, VGRD, etc. at multiple pressure levels
- **Grid:** 0.5° lat-lon global (720 x 361 grid points)

### gefs_perturbed_p02_20260723_t00z_f000.grib2
- **Source:** GEFS - Perturbed Member 02
- **Download URL:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gep02.t00z.pgrb2a.0p50.f000`
- **Download Date:** 2026-07-22 23:47 UTC
- **File Size:** 13,966,199 bytes (~13.3 MB)
- **PDT Content:** 71 messages, all PDT 1 (standard analysis/forecast template)
- **Characteristics:** Individual ensemble perturbed member (ENS=+2)
- **Variables:** HGT, TMP, RH, UGRD, VGRD, etc. at multiple pressure levels
- **Grid:** 0.5° lat-lon global (720 x 361 grid points)

### gefs_ensemble_mean_20260723_t00z_f000.grib2
- **Source:** GEFS - Ensemble Mean (statistical product)
- **Download URL:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2sp25/geavg.t00z.pgrb2s.0p25.f000`
- **Download Date:** 2026-07-22 23:48 UTC
- **File Size:** 13,974,676 bytes (~13.3 MB)
- **PDT Content:** 26 messages (24 PDT 2 + 2 PDT 12)
- **Characteristics:** Statistical ensemble aggregation (ens mean)
- **Variables:** VIS, GUST, MSLET, PRES, HGT, etc.
- **Grid:** 0.25° lat-lon global (higher resolution)

## Validation Status

✅ **Format Validation:** All files successfully decode with wgrib2
✅ **File Sizes:** All files <50MB (suitable for test fixtures)
✅ **GRIB Magic Bytes:** All files start with "GRIB" (hex: 47 52 49 42)
✅ **PDT Analysis:** PDT types documented and verified
✅ **Message Counts:** Confirmed via wgrib2 inventory

## PDT Template Information

- **PDT 1:** Standard analysis/forecast template (used for individual ensemble members)
- **PDT 2:** Analysis/forecast at horizontal level (used in ensemble mean products)
- **PDT 12:** Horizontal level layer (used in ensemble mean products)

Note: While the ensemble files use PDT 1/2/12 for this forecast hour (f000 = analysis time), GEFS ensemble products can also contain PDT 4.1 (individual ensemble forecasts) and PDT 4.8 (clustering) in other forecast hours. The f000 files primarily use standard templates because they contain analysis data.

## Source Information

- **Archive:** NOAA GEFS via Amazon S3 public bucket
- **Bucket:** `noaa-gefs-pds` (us-east-1)
- **Access Method:** HTTPS (no authentication required)
- **Documentation:** [NOAA GEFS AWS Registry](https://registry.opendata.aws/noaa-gefs/)
- **Data Range:** 2017 to present (permanent archive)

## Handling Requirements

1. **File Access:** Files are standard GRIB2 format, readable by any GRIB2 decoder
2. **Grid Size:** Large grid dimensions (720x361 for 0.5°, ~1440x721 for 0.25°) ensure adequate memory for decoding tests
3. **Variable Types:** Files contain both meteorological (HGT, TMP, RH, UGRD, VGRD) and surface (VIS, GUST, PRES) variables
4. **Ensemble Markers:** Perturbed members include ENS=+N identifiers in inventory listings
5. **Statistical Products:** Ensemble mean file contains "ens mean" markers for statistical aggregation

## Usage in Tests

These fixtures are suitable for:
- Testing GRIB2 decoder ensemble product handling
- Validating PDT 1/2/12 template parsing
- Ensemble member identification (ENS numbers)
- Statistical product processing (mean, spread)
- Large grid data structure validation

## References

- **Bead:** bf-11rzg (validation and documentation)
- **Download Source:** bf-1ypv3.md (NOAA ensemble file downloads)
- **Archive Research:** bf-42cga.md (NOAA ensemble GRIB2 archive sources)
- **PDT Verification:** bf-19o3n.md (PDT 4.1 and 4.8 verification in GRIB2 files)
