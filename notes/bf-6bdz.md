# bf-6bdz: wgrib2 DRT/Packing Information Display

## Task
Execute wgrib2 with appropriate flags to inspect GRIB2 file and display data representation/packing information.

## Execution Summary

### wgrib2 Binary Location
The wgrib2 binary was found at:
```
/home/coding/gribtract/grib2/wgrib2/wgrib2
```

Version: wgrib2 v3.1.3 10/2023

### Key Flags for Data Representation Information

**-Sec5**: Display Section 5 (Data Representation Section) values including:
- Section length
- Number of defined data points
- Data Representation Template (DRT) number

**-vt**: Show verification time (reference time + forecast time)

**-V**: Show diagnostic output including grid template, statistical values, and metadata

### Results from Various GRIB2 Files

#### NAM DRT3 File (test_data/nam_awip12_drt3.grib2)
All records use **DRT 5.3** (Simple packing):
```
1:0:vt=2025011500:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:vt=2025011500:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
...
```

#### DRT2 Simple Grid (tests/corpus/small/drt2_simple_3x3.grib2)
Uses **DRT 5.2** (Complex packing):
```
1:0:Sec5 len=47 #defined data points=9 Data Repr. Template=5.2
```

#### DRT40 JPEG 2000 (tests/corpus/small/drt40_j2k_3x2.grib2)
Uses **DRT 5.40** (JPEG 2000 packing):
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.40
```

#### DRT41 PNG (tests/corpus/small/drt41_png_3x2.grib2)
Uses **DRT 5.41** (PNG packing):
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.41
```

### Data Representation Templates Observed
- **5.2**: Complex packing (second order packing)
- **5.3**: Simple packing (no spatial differencing)
- **5.40**: JPEG 2000 coding (lossy compression)
- **5.41**: PNG coding (lossless compression)

## Commands Used

```bash
# Basic DRT information
/home/coding/gribtract/grib2/wgrib2/wgrib2 -Sec5 <file.grib2>

# Combined with verification time
/home/coding/gribtract/grib2/wgrib2/wgrib2 -vt -Sec5 <file.grib2>

# Full diagnostic output
/home/coding/gribtract/grib2/wgrib2/wgrib2 -V <file.grib2>
```

## Verification
✅ wgrib2 executed successfully on all test files
✅ Data representation template information displayed correctly
✅ Multiple DRT types confirmed (5.2, 5.3, 5.40, 5.41)
✅ Output includes data point counts, section lengths, and template numbers
