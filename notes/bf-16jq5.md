# GRIB2 Test File Location

## Target File

**Primary test file:** `tests/corpus/small/gfs_anl_t2m_5x5.grib2`

- **Size:** 204 bytes
- **Type:** Valid GRIB2 file (magic number: `GRIB....`)
- **Grid:** 5x5 temperature analysis
- **Suitability:** Small, simple file ideal for wgrib2 inspection and testing

## Alternative Files

### Small test files (tests/corpus/small/)
- `pdt1_ensemble_3x2.grib2` (188 bytes)
- `pdt8_accum_3x2.grib2` (205 bytes)
- `rotated_latlon_5x5.grib2` (216 bytes)
- `drt40_j2k_3x2.grib2` (312 bytes) - JPEG2000 compression
- `drt41_png_3x2.grib2` (252 bytes) - PNG compression
- `gfs_tmp2m_1deg_anl.grib2` (47K)
- `gfswave_arctic_wind_drt40.grib2` (418K)
- `mrms_carib_refl_drt41.grib2` (28K)

### Large reference file
- `samples/nam_awip12_20250115_t00z_f00.grib2` (26 MB) - Full NAM forecast

## Verification

```bash
# Confirm file exists and is readable
ls -lh tests/corpus/small/gfs_anl_t2m_5x5.grib2

# Verify GRIB2 magic number
xxd -l 8 tests/corpus/small/gfs_anl_t2m_5x5.grib2
# Output: 00000000: 4752 4942 0000 0002  GRIB....
```

All files are valid GRIB2 format (edition 2, indicated by `0002` after `GRIB` magic number).
