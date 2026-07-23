# wgrib2 DRT Flags Execution (bf-1gqu)

## Task Completion Summary

Successfully executed wgrib2 with Data Representation Template (DRT) flags on test GRIB2 files.

## wgrib2 Flag Used

**`-Sec5`** - Shows Section 5 values (Data representation section)

This flag displays:
- Section 5 length in bytes
- Number of defined data points in the grid
- Data Representation Template number (in 5.X format)

## Test Results

### Small Test Files

**drt40_j2k_3x2.grib2** (JPEG 2000 compression):
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.40
```

**drt41_png_3x2.grib2** (PNG compression):
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.41
```

### Large Test Files

**gfswave_arctic_wind_drt40.grib2** (JPEG 2000):
```
1:0:Sec5 len=23 #defined data points=360052 Data Repr. Template=5.40
```

**mrms_carib_refl_drt41.grib2** (PNG):
```
1:0:Sec5 len=21 #defined data points=4500000 Data Repr. Template=5.41
```

## Data Representation Templates Identified

- **5.40** - JPEG 2000 compression template
- **5.41** - PNG compression template

## Command Pattern

```bash
grib2/wgrib2/wgrib2 -Sec5 <grib2_file>
```

## Acceptance Criteria Met

✅ wgrib2 command executed successfully with DRT flags (-Sec5)
✅ Output includes data representation template information
✅ Commands complete without errors across multiple test files
