# WGrib2 DRT Inspection Results (bf-2ncsh)

## Task Completed: WGrib2 DRT (Data Representation Template) Inspection

### Command Used
```bash
wgrib2 -Sec5 <grib2_file>
```

The `-Sec5` flag displays Section 5 values (Data Representation Section) which contains the DRT information.

### DRTs Identified

#### From Sample Files:
1. **DRT 5.2**: Simple packing
   - File: `tests/corpus/small/drt2_simple_3x3.grib2`
   - Output: `Data Repr. Template=5.2`
   - Data points: 9

2. **DRT 5.40**: JPEG 2000 compression
   - File: `tests/corpus/small/drt40_j2k_3x2.grib2`
   - Output: `Data Repr. Template=5.40`
   - Data points: 6

3. **DRT 5.40**: JPEG 2000 compression (real-world)
   - File: `tests/corpus/small/gfswave_arctic_wind_drt40.grib2`
   - Output: `Data Repr. Template=5.40`
   - Data points: 360,052

4. **DRT 5.41**: PNG compression
   - File: `tests/corpus/small/drt41_png_3x2.grib2`
   - Output: `Data Repr. Template=5.41`
   - Data points: 6

5. **DRT 5.41**: PNG compression (real-world)
   - File: `tests/corpus/small/mrms_carib_refl_drt41.grib2`
   - Output: `Data Repr. Template=5.41`
   - Data points: 4,500,000

#### From Real GRIB2 Files:
1. **DRT 5.3**: Complex packing (HRRR model data)
   - File: `data/hrrr.t12z.wrfsfcf00.grib2`
   - Multiple records all using DRT 5.3
   - Output: `Data Repr. Template=5.3`
   - Data points: 1,905,141 per record

2. **DRT 5.3**: Complex packing (NAM model data)
   - File: `tests/corpus/large/nam.t00z.awip1200.tm00.grib2`
   - Multiple records all using DRT 5.3
   - Output: `Data Repr. Template=5.3`
   - Data points: 262,792 per record

### Key Findings:
- **DRT 5.2**: Simple packing - used in test fixtures
- **DRT 5.3**: Complex packing - used in operational weather models (NAM, HRRR)
- **DRT 5.40**: JPEG 2000 compression - used in wave model data and test fixtures
- **DRT 5.41**: PNG compression - used in MRMS radar data and test fixtures

### WGrib2 Version
```
wgrib2 v3.1.3 10/2023
```

### Notes:
- The `nam_20250115_awip12.grib2` and `nam.20250115.t00z.awip1200.tm00.grib2` files produced no output with `-Sec5` flag
- The `test_data/nam_awip12_drt3.grib2` file is empty (0 bytes)
- All successfully inspected files showed consistent DRT usage within each file
