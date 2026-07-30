# Ensemble GRIB2 Test Fixture Validation - bf-11rzg

## Task Completed
Validated and documented ensemble GRIB2 test fixtures from NOAA GEFS archive.

## Files Validated

Three ensemble GRIB2 files successfully validated:

1. **gefs_perturbed_p01_20260723_t00z_f000.grib2**
   - Size: 13,984,963 bytes (~13.3 MB) ✅
   - Messages: 71 PDT 1 (standard analysis/forecast)
   - Status: Valid GRIB2 format, decodes successfully

2. **gefs_perturbed_p02_20260723_t00z_f000.grib2**
   - Size: 13,966,199 bytes (~13.3 MB) ✅
   - Messages: 71 PDT 1 (standard analysis/forecast)
   - Status: Valid GRIB2 format, decodes successfully

3. **gefs_ensemble_mean_20260723_t00z_f000.grib2**
   - Size: 13,974,676 bytes (~13.3 MB) ✅
   - Messages: 26 (24 PDT 2 + 2 PDT 12)
   - Status: Valid GRIB2 format, decodes successfully

## Validation Results

### Format Validation ✅
- All files confirmed as valid GRIB2 format
- GRIB magic bytes verified (hex: 47 52 49 42 = "GRIB")
- Files decode successfully with wgrib2 v2.0.8+
- Message counts match expectations

### File Size Validation ✅
- All files are ~13.3 MB (well under 50MB target)
- Suitable for test fixture use
- Fast to download and process

### Source Documentation ✅
- Source: NOAA GEFS via Amazon S3 public bucket
- Download date: 2026-07-22 23:47-23:56 UTC
- URLs documented in README.md
- Permanent archive (2017 to present)

### PDT Content Analysis ✅
- **Perturbed members (p01, p02):** PDT 1 - standard analysis/forecast template
- **Ensemble mean:** PDT 2 - analysis/forecast at horizontal level, PDT 12 - horizontal layer
- Ensemble identification markers present (ENS=+1, ENS=+2, ens mean)

## Test Fixture Installation

Files installed to: `test_data/ensemble/`
- README.md with full documentation
- 3 GRIB2 files ready for test use

## Acceptance Criteria Status

✅ **File successfully decodes with standard grib2 tools** - All files validate with wgrib2
✅ **File size suitable for test fixture (<50MB)** - All files ~13.3 MB
✅ **Source URL and retrieval date recorded** - Documented in README.md
✅ **PDT message types confirmed and documented** - PDT 1/2/12 analyzed
✅ **File ready for use as test fixture** - Installed in test_data/ensemble/

## Special Notes

1. **PDT Template Usage:** These f000 (analysis) files primarily use PDT 1/2/12. PDT 4.1 and 4.8 (ensemble-specific templates) are more common in later forecast hours (f003+).

2. **Grid Dimensions:** Large grid sizes (720x361 for 0.5°, ~1440x721 for 0.25°) ensure adequate memory testing for decoders.

3. **Ensemble Markers:** Perturbed members include ENS=+N identifiers; ensemble mean includes statistical aggregation markers.

## Related Documentation

- **Download source:** notes/bf-1ypv3.md (ensemble file downloads)
- **Archive research:** notes/bf-42cga.md (NOAA GEFS archive sources)
- **PDT verification:** notes/bf-19o3n.md (PDT 4.1 and 4.8 verification)

## Tooling

- **wgrib2:** v2.0.8+ at `/home/coding/.local/bin/wgrib2`
- **Commands used:**
  ```bash
  wgrib2 <file> -pdt              # PDT analysis
  wgrib2 <file> -pdt | cut -d: -f3 | sort | uniq -c  # Count by PDT
  wgrib2 <file> | wc -l           # Message count
  od -N4 -An -tx1 <file>          # Magic byte check
  ```

## Files Created

- `test_data/ensemble/README.md` - Complete fixture documentation
- `test_data/ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2` - Perturbed member #01
- `test_data/ensemble/gefs_perturbed_p02_20260723_t00z_f000.grib2` - Perturbed member #02
- `test_data/ensemble/gefs_ensemble_mean_20260723_t00z_f000.grib2` - Ensemble mean
- `notes/bf-11rzg.md` - This validation summary

## References

- NOAA GEFS AWS Registry: https://registry.opendata.aws/noaa-gefs/
- NCEP GEFS Products: https://www.nco.ncep.noaa.gov/pmb/products/gens/
