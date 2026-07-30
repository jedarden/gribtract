# DRT (Data Representation Type) Analysis Summary

**Date:** 2026-07-24  
**Analysis ID:** bf-4ey85  
**Workspace:** /home/coding/gribtract

## Overview

This document summarizes the findings from a comprehensive DRT (Data Representation Type) extraction across all GRIB2 files in the gribtract workspace. DRT values are stored in GRIB2 Section 3 and indicate the grid definition template used for the data.

## wgrib2 Command Used

The following command was used to extract DRT values from each GRIB2 file:

```bash
wgrib2 "$FILE" -grid 2>&1 | grep -oP 'grid_template=\K[0-9]+' | sort -u
```

**Components explained:**
- `wgrib2 "$FILE" -grid` - Outputs grid information for each message in the GRIB2 file
- `grep -oP 'grid_template=\K[0-9]+'` - Extracts the grid template number using Perl regex
- `sort -u` - Sorts and removes duplicates (for multi-message files)

**Full extraction script:** `scripts/extract_drt.sh`

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **Total files checked** | 85 | 100% |
| **Successful extractions** | 64 | 75.3% |
| **Errors** | 21 | 24.7% |

## DRT Value Distribution

Among the 64 successfully extracted files:

| DRT Value | Count | Percentage | Typical Products |
|-----------|-------|------------|------------------|
| **0** | 30 | 46.9% | GFS, GEFS models, CONUS data |
| **30** | 31 | 48.4% | HRRR, NAM, RAP models |
| **1** | 1 | 1.6% | Rotated latitude/longitude grids |
| **20** | 1 | 1.6% | Wave model grids |
| **40** | 1 | 1.6% | ECMWF ensemble data |

**Total:** 64 files

### DRT=0 Files (30 files)

The most common DRT value, found in:

**GFS (Global Forecast System):**
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.0p50.f006.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.0p50.f000.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.1p00.f012.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.20260723.t00z.pgrb2.1p00.f006.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.1p00.f006.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.0p50.f003.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.1p00.f003.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.20260722.t00z.pgrb2.1p00.f000.grib2`
- `crates/gribtract/fixtures/noaa-samples/gfs.20260723.t00z.pgrb2.1p00.f000.grib2`
- `downloads/gfs_20260724_00z_1p00_f000.grib2`

**GEFS (Global Ensemble Forecast System):**
- `test_data/ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2`
- `test_data/ensemble/gefs_perturbed_p02_20260723_t00z_f000.grib2`
- `test_data/ensemble/gefs_ensemble_mean_20260723_t00z_f000.grib2`
- `tests/corpus/large/gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2`
- `tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2` (empty file)
- `tests/corpus/large/gefs_ensemble_p01_cape.grib2`
- `tests/corpus/large/gefs.20240101.00.atmos.pgrb2ap5.gep01.t00z.pgrb2a.0p50.f000.grib2`

**CONUS DRT0 Test Files:**
- `tests/corpus/small/conus_drt0.grib2`
- `drt_check_samples/test.grib2`
- `drt_check_samples/gfs_20260724_00z_1p00_f000.grib2`
- `scratch/drt0-test/gfs_sample.grib2`

**Other DRT=0:**
- `tests/corpus/large/ecmwf_ensemble_enso_0h.grib2`
- `tests/corpus/small/gfs_tmp2m_1deg_anl.grib2`
- `tests/corpus/small/pdt8_accum_3x2.grib2`
- `tests/corpus/small/mrms_carib_refl_drt41.grib2`
- `tests/corpus/small/drt2_simple_3x3.grib2`
- `tests/corpus/small/pdt1_ensemble_3x2.grib2`
- `tests/corpus/small/gfs_anl_t2m_5x5.grib2`
- `tests/corpus/small/drt40_j2k_3x2.grib2`
- `tests/corpus/small/drt41_png_3x2.grib2`
- `scratch/drt0-verification/nam.t12z.afwaca00.tm00.grib2`

### DRT=30 Files (31 files)

The second most common DRT value, found primarily in regional models:

**HRRR (High-Resolution Rapid Refresh):**
- `ndfd_temp.grib2`
- `samples/bf-dy62/nam_awip12_20250115_t00z_f00.grib2` (actual DRT=30)
- `samples/hrrr.20260723.t00z.wrfsfcf01.grib2`
- `samples/nam.t00z.awip1200.tm00.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf02.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf08.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf03.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf05.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf07.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf04.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf00.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf01.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf12.grib2`
- `samples/grib2-noaa-hrrr/hrrr.20260724.t00z.wrfsfcf06.grib2`
- `fixtures/conus/hrrr.t12z.wrfsfcf00.grib2`
- `data/conus_drt0_mxuphl.grib2` (actual DRT=30)
- `data/hrrr.t12z.wrfsfcf00.grib2`
- `data/conus_drt0_mxuphl_20260723.grib2` (actual DRT=30)
- `data/noaa-hrrr/hrrr.t12z.wrfsfcf00.grib2`
- `tests/corpus/large/hrrr.t12z.wrfsfcf00.grib2`
- `tests/corpus/large/rap.t12z.awp130pgrbf00.grib2`
- `tests/corpus/large/rap.20240123.t00z.awp130pgrbf00.grib2`
- `tests/corpus/large/nam.t00z.awip1200.tm00.20250120.grib2`
- `tests/corpus/large/hrrr.t12z.wrfsfcf00.20260723.grib2`
- `tests/corpus/large/nam.20240123.t00z.awip1200.tm00.grib2`
- `tests/corpus/large/nam.t00z.awip1200.tm00.grib2`
- `scratch/drt0_test/hrrr_t00z_f01.grib2`
- `scratch/drt0_test/nam_t12z_awphys00.grib2`
- `scratch/drt0_test/hrrr_t12z_f00.grib2`
- `scratch/drt0_test/hrrr_t06z_f00.grib2`
- `scratch/drt0_test/rap_t12z_awp236_f00.grib2`

### DRT≠0 (Non-Zero Values)

| DRT | File |
|-----|------|
| **1** | `tests/corpus/small/rotated_latlon_5x5.grib2` |
| **20** | `tests/corpus/small/gfswave_arctic_wind_drt40.grib2` |
| **40** | `tests/corpus/large/flx.2024011500.grib2` |

## Error Summary

Of the 85 files checked, 21 had errors during DRT extraction:

### Error Categories

1. **Empty files (12 files):**
   - `crates/gribtract/fixtures/noaa-samples/gfs.t00z.pgrb2.1p00.f000.grib2`
   - `test.grib2`
   - `gefs_test.grib2`
   - `samples/nam_awip12_20250115_t00z_f00.grib2`
   - `nam_20250115_awip12.grib2`
   - `test_data/nam_awip12_drt3.grib2`
   - `data/nam.t00z.awip1200.tm00.grib2`
   - `grib2/nam.20250115.t00z.awip1200.tm00.grib2`
   - `grib2/hrrr.t12z.wrfsfcf00.grib2`
   - `tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2`
   - `scratch/drt0-verification/rap.t12z.awip32f00.grib2`

2. **Corrupted or invalid GRIB2 files (9 files):**
   - `samples/hrrr.t00z.wrfsfcf01.grib2`
   - `samples/grib2-noaa-nam/nam.20260724.t00z.conusnest.hiresf00.tm00.grib2`
   - `samples/grib2-noaa-rap/rap.20260724.t00z.awp130pgrbf00.grib2`
   - `samples/grib2-noaa-hrrr/hrrr.20260724.t06z.wrfsfcf00.grib2`
   - `samples/grib2-noaa-hrrr/hrrr.20260723.t00z.wrfsfcf03.grib2`
   - `samples/grib2-noaa-hrrr/hrrr.20260723.t12z.wrfsfcf00.grib2`
   - `tests/corpus/large/gefs_ensemble_mean_sample.grib2`
   - `scratch/drt0-verification/nam.t12z.awp130f00.grib2`
   - `hrrr_test_20260724.grib2`

## Key Findings

1. **DRT Distribution is Highly Bimodal:**
   - DRT=0 and DRT=30 account for 95.3% of all successful extractions (61/64 files)
   - This reflects the dominance of global (DRT=0) and regional (DRT=30) weather models

2. **Model-Product Correlation:**
   - **GFS/GEFS** (global models) consistently use DRT=0
   - **HRRR/NAM/RAP** (regional models) consistently use DRT=30
   - This pattern helps identify model type from DRT value alone

3. **File Quality Issues:**
   - 14.1% of files (12/85) are empty or corrupted
   - These should be cleaned up or redownloaded for accurate testing

4. **Naming Convention Anomalies:**
   - Several files named "conus_drt0" or "drt0" actually have DRT=30
   - Example: `data/conus_drt0_mxuphl.grib2`, `data/conus_drt0_mxuphl_20260723.grib2`
   - This suggests historical naming may not reflect actual DRT values

## Recommendations

1. **Clean up empty/corrupted files** - Remove or redownload the 21 problematic files
2. **Update file naming** - Ensure "drt0" in filenames corresponds to actual DRT=0 values
3. **Use DRT for model identification** - Leverage the strong correlation between DRT and model type for automated classification
4. **Monitor file integrity** - Implement checks to prevent empty files from accumulating

## Raw Data Files

The complete extraction results are available in:
- `drt_extraction_results/drt_extraction_20260724_025219.csv` - Full CSV with all results
- `drt_extraction_results/drt_extraction_20260724_025219.json` - JSON summary
- `drt_extraction_results/drt_extraction_20260724_025219_errors.txt` - Detailed error messages

## Extraction Scripts

- `scripts/extract_drt.sh` - Core DRT extraction using wgrib2
- `run_drt_extraction_all.sh` - Batch extraction across all GRIB2 files

---

**Generated:** 2026-07-24  
**Analysis by:** bf-4ey85  
**Repository:** gribtract
