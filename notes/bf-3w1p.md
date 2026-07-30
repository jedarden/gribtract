# DRT Number Identification - Bead bf-3w1p

## Task Completed
Successfully identified and documented the Data Representation Template (DRT) number from wgrib2 output.

## wgrib2 Command Used
```bash
./grib2/wgrib2/wgrib2 -Sec5 <grib2_file>
```

The `-Sec5` flag displays Section 5 (Data Representation Section) of the GRIB2 message, which contains the DRT information.

## DRT Values Identified

### DRT 3 (Complex Packing) - Confirmed
**File**: `test_data/nam_awip12_drt3.grib2`
**Output**: `Data Repr. Template=5.3`
**Status**: ✅ CONFIRMED - All 186 records in the file use DRT 3 (complex packing)

**Sample output**:
```
1:0:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
...
186:26152913:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
```

### DRT 2 (Simple Packing with Spatial Differences)
**File**: `tests/corpus/small/drt2_simple_3x3.grib2`
**Output**: `Data Repr. Template=5.2`

### DRT 0 (Simple Packing)
**File**: `tests/corpus/small/gfs_anl_t2m_5x5.grib2`
**Output**: `Data Repr. Template=5.0`

## DRT 3 Characteristics
- **Template Number**: 5.3 (Section 5, Template 3)
- **Packing Type**: Complex packing
- **Use Case**: NAM AWIP12 meteorological data
- **File Size**: ~26MB for 186 records with 262,792 data points each

## Key Finding
The NAM AWIP12 file `test_data/nam_awip12_drt3.grib2` **DOES use DRT 3** (complex packing), confirming the expected value based on the file naming convention.

## Date
2026-07-23

## Tool
- wgrib2 v3.1.3
- Located at: `./grib2/wgrib2/wgrib2`
