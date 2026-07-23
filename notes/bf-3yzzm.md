# Ensemble File Candidate Selection and PDT Verification - bf-3yzzm

## Task Summary
Selected a specific NOAA GEFS ensemble GRIB2 file candidate and verified it contains the required Product Definition Template (PDT) 4.1 for ensemble forecast data.

## Selected File

### File Information
**Local Path:** `/tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2`

**Source URL:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000
```

**File Details:**
- **Product:** GEFS (Global Ensemble Forecast System)
- **Member:** Perturbed member 01 (p01) - positively perturbed forecast
- **Resolution:** 0.5° (pgrb2ap5 field set)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 000 (analysis time)
- **File Size:** 13,984,963 bytes (~13.3 MB) ✅ Within <50MB target
- **Messages:** 71 GRIB2 messages in the file

## PDT Verification

### Product Definition Template Confirmation
Using `grib_dump` to inspect the GRIB2 message structure, **PDT 1 (GRIB2 template 4.1)** is confirmed:

```
productDefinitionTemplateNumber = 1 [Individual ensemble forecast, control and perturbed, 
at a horizontal level or in a horizontal layer at a point in time (grib2/tables/2/4.0.table) ]
```

This corresponds to:
- **PDT 4.1**: Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time

### Additional Ensemble Metadata from GRIB2

From `grib_dump` output for Message 1:

**Section 4 (Product Definition):**
- `typeOfProcessedData = 4` [Perturbed forecast products]
- `typeOfGeneratingProcess = 4` [Ensemble forecast]
- `typeOfEnsembleForecast = 3` [Positively perturbed forecast]
- `perturbationNumber = 1` (Member +1)
- `numberOfForecastsInEnsemble = 30`

**Parameter:**
- `parameterCategory = 3` [Mass]
- `parameterNumber = 5` [Geopotential height (gpm)]
- First surface: 10 mb isobaric level

### Ensemble Message Verification

All 71 messages in the file contain ensemble data (verified with `wgrib2`):
- Each message shows `ENS=+1` indicating perturbed member 01
- Consistent ensemble structure across all atmospheric variables (HGT, TMP, RH, UGRD, VGRD, etc.)
- Multiple pressure levels from surface (10 mb) through upper atmosphere

## File Structure Summary

**Grid Definition:**
- Template: Latitude/longitude (equidistant cylindrical)
- Resolution: 720 x 361 points (0.5° spacing)
- Coverage: Global (90°N to 90°S, 0° to 359.5°E)

**Data Representation:**
- Template 3: Complex packing and spatial differencing
- Compression: 11 bits per value
- Second-order spatial differencing

## Acceptance Criteria Verification

✅ **Specific ensemble file selected with URL** - GEFS perturbed member 01 from AWS S3 archive
✅ **Verification that file contains PDT 4.1** - Confirmed via `grib_dump` showing PDT 1 (GRIB2 4.1)
✅ **File size documented** - 13,984,963 bytes (~13.3 MB), well under 50MB target
✅ **Source URL and date recorded** - Full URL and cycle date (2026-07-23 00Z) documented

## Technical Context

### Why This File Was Selected

1. **PDT 4.1 Compliance**: File contains individual ensemble forecast data using PDT 4.1 template
2. **Representative**: Perturbed member 01 is representative of the 30-member GEFS ensemble
3. **Manageable Size**: ~13.3 MB is well under the 50MB target for test fixtures
4. **Authentic Source**: Downloaded from official NOAA GEFS archive on AWS S3
5. **Well-Documented**: GEFS is the standard global ensemble forecast system from NCEP

### PDT 4.1 Significance

PDT 4.1 is the standard template for individual ensemble forecasts in GRIB2:
- Used for both control and perturbed ensemble members
- Contains ensemble member identification (perturbation number, total members)
- Indicates ensemble type (positive/negative perturbation or control)
- Essential for ensemble processing applications

## References

- **NOAA GEFS AWS Registry**: https://registry.opendata.aws/noaa-gefs/
- **NCEP GEFS Products**: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- **GRIB2 PDT 4.1 Documentation**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1.shtml
- **Previous Research**: notes/bf-1dj5r.md (NOAA ensemble product archives)
- **Download Source**: notes/bf-1ypv3.md (GEFS file downloads)

## Verification Commands

```bash
# Verify ensemble members
wgrib2 /tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2

# Verify PDT structure
grib_dump -O /tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2

# Check file size
ls -lh /tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2

# Count messages
wgrib2 /tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2 | wc -l
```

## Conclusion

The selected GEFS perturbed member 01 file from 2026-07-23 00Z cycle is verified to contain PDT 4.1 (Product Definition Template 4.1 - Individual ensemble forecast) with complete ensemble metadata. The file is suitable for use as a test fixture for GRIB2 decoder validation with ensemble data, meeting all acceptance criteria for size, authenticity, and template compliance.
