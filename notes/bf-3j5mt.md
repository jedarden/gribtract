# bf-3j5mt: GRIB2 File Inventory

## Summary

Located **20 GRIB2 files** in the gribtract repository. Files are categorized below by size and purpose.

## File Inventory

### Empty/Placeholder Files (0 bytes)
These files appear to be placeholders or broken symlinks:
- `/home/coding/gribtract/grib2/hrrr.t12z.wrfsfcf00.grib2`
- `/home/coding/gribtract/grib2/nam.20250115.t00z.awip1200.tm00.grib2`
- `/home/coding/gribtract/nam_20250115_awip12.grib2`
- `/home/coding/gribtract/samples/nam_awip12_20250115_t00z_f00.grib2`
- `/home/coding/gribtract/test_data/nam_awip12_drt3.grib2`

### Large Production Files (26M - 136M)
Primary files for real-world DRT inspection:
- `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2` (136M) - HRRR surface forecast
- `/home/coding/gribtract/data/nam.t00z.awip1200.tm00.grib2` (26M) - NAM AWIP12
- `/home/coding/gribtract/samples/bf-dy62/nam_awip12_20250115_t00z_f00.grib2` (26M)
- `/home/coding/gribtract/samples/nam.t00z.awip1200.tm00.grib2` (26M)
- `/home/coding/gribtract/tests/corpus/large/nam.t00z.awip1200.tm00.grib2` (26M)

### Small Test Corpus (188 bytes - 418K)
Test files for specific DRT variants (in `tests/corpus/small/`):
- `pdt1_ensemble_3x2.grib2` (188 bytes)
- `gfs_anl_t2m_5x5.grib2` (204 bytes)
- `pdt8_accum_3x2.grib2` (205 bytes)
- `rotated_latlon_5x5.grib2` (216 bytes)
- `drt2_simple_3x3.grib2` (217 bytes)
- `drt41_png_3x2.grib2` (252 bytes)
- `drt40_j2k_3x2.grib2` (312 bytes)
- `mrms_carib_refl_drt41.grib2` (28K)
- `gfs_tmp2m_1deg_anl.grib2` (47K)
- `gfswave_arctic_wind_drt40.grib2` (418K)

## Primary Files for DRT Inspection

### Priority 1: Large production files
- `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2` - 136M (largest, most comprehensive)
- `/home/coding/gribtract/data/nam.t00z.awip1200.tm00.grib2` - 26M

### Priority 2: Named DRT variant test files
- `/home/coding/gribtract/tests/corpus/small/drt2_simple_3x3.grib2` - DRT 2 (simple packing)
- `/home/coding/gribtract/tests/corpus/small/drt40_j2k_3x2.grib2` - DRT 40 (JPEG 2000)
- `/home/coding/gribtract/tests/corpus/small/drt41_png_3x2.grib2` - DRT 41 (PNG)
- `/home/coding/gribtract/tests/corpus/small/mrms_carib_refl_drt41.grib2` - DRT 41 (larger example)
- `/home/coding/gribtract/tests/corpus/small/gfswave_arctic_wind_drt40.grib2` - DRT 40 (larger example)

## Existing DRT Documentation

The `notes/` directory contains extensive DRT analysis from previous beads (bf-13e3, bf-16jq5, bf-1dd2, etc.), which should be referenced for context before new wgrib2 inspections.

## Next Steps

Ready for wgrib2 DRT inspection on:
1. Large production files to understand real-world packing configurations
2. Small DRT-named test files for targeted variant verification
