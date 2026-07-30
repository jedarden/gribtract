# DRT Inspection - wgrib2 Analysis
**Bead:** bf-4jpf  
**Date:** 2026-07-23

## Task
Inspect and document Data Representation Template (DRT) values in GRIB2 files using wgrib2.

## Files Inspected

### Main Files
1. **samples/nam.t00z.awip1200.tm00.grib2** (26MB)
2. **tests/corpus/large/nam.t00z.awip1200.tm00.grib2**

### Test Corpus (small/)
- drt2_simple_3x3.grib2
- drt40_j2k_3x2.grib2
- drt41_png_3x2.grib2
- gfs_anl_t2m_5x5.grib2
- gfs_tmp2m_1deg_anl.grib2
- gfswave_arctic_wind_drt40.grib2
- mrms_carib_refl_drt41.grib2
- pdt1_ensemble_3x2.grib2
- pdt8_accum_3x2.grib2
- rotated_latlon_5x5.grib2

## wgrib2 Command Used
```bash
wgrib2 <file> -Sec5
```

The `-Sec5` option shows Section 5 (Data Representation Section) which contains the Data Representation Template (DRT) information.

## DRT Values Found

### Main NAM File (196 records)
- **DRT 5.3** — All 196 records use Data Representation Template 5.3
- This is **NOT DRT 3** (complex packing)
- DRT 5.3 is typically used for simple packing with spatial differencing

### Test Corpus Summary
| DRT Value | Count | Template Name | File Example |
|-----------|-------|---------------|--------------|
| 5.0 | 4 | Simple packing | gfs_anl_t2m_5x5.grib2 |
| 5.2 | 1 | Simple packing with decimal scaling | drt2_simple_3x3.grib2 |
| 5.3 | 1 | Simple packing with spatial differencing | gfs_tmp2m_1deg_anl.grib2 |
| 5.40 | 2 | JPEG2000 coding | drt40_j2k_3x2.grib2, gfswave_arctic_wind_drt40.grib2 |
| 5.41 | 2 | PNG coding | drt41_png_3x2.grib2, mrms_carib_refl_drt41.grib2 |

## Key Finding: No DRT 3 Files Found

**DRT 3 (complex packing)** was **NOT FOUND** in any of the inspected files.

- DRT 3.0 is "Complex packing" (a legacy complex packing scheme)
- DRT 5.x is the newer "Data Representation Template 5" series which includes various compression schemes

The workspace contains placeholder files (empty 0-byte files) with names suggesting DRT 3 content:
- test_data/nam_awip12_drt3.grib2 (0 bytes)
- nam_20250115_awip12.grib2 (0 bytes)

However, no actual DRT 3 GRIB2 files were found in the current workspace.

## DRT Classification Summary

From the wgrib2 inspection, the workspace contains:

- **Simple packing schemes**: DRT 5.0, 5.2, 5.3
- **JPEG2000 compression**: DRT 5.40
- **PNG compression**: DRT 5.41
- **No complex packing (DRT 3)**: Not present in inspected files

## Sample Output from wgrib2 -Sec5

```
1:0:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
3:481603:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
```

Each line shows:
- Record number and byte offset
- Section 5 length
- Number of defined data points
- **Data Representation Template number** (the DRT value)

## Conclusion

The GRIB2 files in the gribtract workspace use DRT 5.x series templates (simple packing and compression schemes), **NOT DRT 3 (complex packing)**. 

The actual DRT 3 (complex packing) files are either:
1. Not present in the current workspace
2. Represented only by empty placeholder files
