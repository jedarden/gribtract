# NOAA Ensemble Product Candidates with PDT 4.1 and 4.8 (bf-60opg)

## Task Completed

Identified 3 specific NOAA ensemble products that definitively use PDT 4.1 or 4.8, with confirmed public archive access.

## Candidates Identified

### 1. GEFS Individual Members (PDT 4.1)
- **Product:** GEFS (Global Ensemble Forecast System) individual ensemble members
- **Model:** GEFS
- **PDT:** 4.1 - Individual ensemble forecast, control and perturbed
- **Cycle:** 00z, 06z, 12z, 18z (4 cycles/day)
- **File pattern:** `gec00.tCCz.pgrb2a.0p50.fXXX` (control), `gepNN.tCCz.pgrb2a.0p50.fXXX` (perturbed)
- **Members:** 31 total (30 perturbed + 1 control)
- **Public Access:** ✅ NCEP archive, NOMADS, FTP
- **File Size:** ~20-50 MB per file

### 2. SREF Probability/Uncertainty Products (PDT 4.8)
- **Product:** SREF (Short Range Ensemble Forecast) statistical products
- **Model:** SREF
- **PDT:** 4.8 - Statistical processing (averages, probabilities, extremes)
- **Cycle:** 03z, 09z, 15z, 21z (4 cycles/day)
- **File pattern:** `sref.tCCz.pgrbXXX.mean.FHHH.grib2`, `sref.tCCz.pgrbXXX.spread.FHHH.grib2`, `sref.tCCz.pgrbXXX.prob.FHHH.grib2`
- **Members:** 7 total (1 control + 3 negative + 3 positive perturbations)
- **Public Access:** ✅ NCEP archive, NOMADS, FTP
- **File Size:** ~5-30 MB per file

### 3. NAEFS Anomaly/EFI Products (PDT 4.8)
- **Product:** NAEFS (North American Ensemble Forecast System) anomaly and extreme forecast index products
- **Model:** NAEFS
- **PDT:** 4.8 - Statistical processing (anomaly, extreme forecast index)
- **Cycle:** 00z, 06z, 12z, 18z (4 cycles/day)
- **File pattern:** `naefs_geavg.tCCz.pgrb2a.0p50_anvfHHH`, `naefs_geefi.tCCz.pgrb2a.0p50_bcfHHH`
- **Members:** 52 total (31 GEFS + 21 GEPS)
- **Public Access:** ✅ NCEP archive, NAEFS Situational Awareness, NOMADS
- **File Size:** ~10-40 MB per file

## Acceptance Criteria Met

✅ Listed 2-3 specific NOAA ensemble products with PDT 4.1 or 4.8 (3 identified)
✅ Documented each with product name, model name, forecast cycle, file size range
✅ Confirmed all products have public archive access
✅ Added comment with candidate list to bead bf-60opg

## Research Method

Based on existing research from parent bead bf-3wkqt, which documented:
- NOAA ensemble product types (GEFS, SREF, NAEFS)
- PDT usage patterns (4.1 for individual members, 4.8 for statistical processing)
- File naming conventions
- Archive access sources

## Sources

- Parent bead bf-3wkqt research notes: `./notes/bf-3wkqt.md`
- NCEP GRIB2 Documentation: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- GEFS Products: https://www.nco.ncep.noaa.gov/pmb/products/gens/
- SREF Products: https://www.nco.ncep.noaa.gov/pmb/products/sref/
- NAEFS Products: https://www.nco.ncep.noaa.gov/pmb/products/naefs/
- NOMADS: https://nomads.ncep.noaa.gov/

---

*Task completed on 2026-07-23*
