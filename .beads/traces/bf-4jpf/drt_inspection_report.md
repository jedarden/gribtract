# Data Representation Template (DRT) Inspection Report

## Task
Use wgrib2 to inspect and document the Data Representation Template number of GRIB2 files.

## Method
The `-Sec5` flag in wgrib2 is used to extract and display Section 5 (Data Representation Section) information from GRIB2 messages. This section contains the Data Representation Template (DRT) number.

## wgrib2 Command
```bash
./grib2/wgrib2/wgrib2 -Sec5 <grib2_file>
```

## DRT Number Format
The DRT appears in the output as:
```
<record>:<byte_offset>:Sec5 len=<length> #defined data points=<count> Data Repr. Template=5.<DRT_NUMBER>
```

The DRT number is the value after "5." in the "Data Repr. Template" field.

## Findings

### Test File: test_data/nam_awip12_drt3.grib2
**DRT: 3** (Complex packing / Spatial differencing)

Sample output:
```
1:0:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
3:481603:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
```

This file contains 186 records, all using DRT 3 (complex packing with spatial differencing).

### Test File: tests/corpus/small/drt2_simple_3x3.grib2
**DRT: 2** (Simple packing)

Output:
```
1:0:Sec5 len=47 #defined data points=9 Data Repr. Template=5.2
```

This is a small 3x3 grid using simple packing (DRT 2).

### Test File: tests/corpus/small/drt40_j2k_3x2.grib2
**DRT: 40** (JPEG 2000 packing)

Output:
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.40
```

This file uses JPEG 2000 compression (DRT 40).

### Test File: tests/corpus/small/drt41_png_3x2.grib2
**DRT: 41** (PNG packing)

Output:
```
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.41
```

This file uses PNG compression (DRT 41).

### Sample File: samples/nam_awip12_20250115_t00z_f00.grib2
**DRT: 3** (Complex packing / Spatial differencing)

Sample output:
```
1:0:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
```

This is a real NAM forecast file using DRT 3 (complex packing).

## Summary

The inspection successfully identified DRT numbers from multiple GRIB2 files:

- **DRT 2**: Simple packing (uncompressed data)
- **DRT 3**: Complex packing with spatial differencing (most common for NCEP models)
- **DRT 40**: JPEG 2000 compression
- **DRT 41**: PNG compression

The wgrib2 `-Sec5` flag provides a reliable method for identifying the Data Representation Template number in GRIB2 files. The DRT value is consistently shown in the format "Data Repr. Template=5.XX" where XX is the DRT number.

## Full Output Files
- `.beads/traces/bf-4jpf/drt2_output.txt` - DRT 2 inspection output
- `.beads/traces/bf-4jpf/drt3_output.txt` - DRT 3 inspection output
- `.beads/traces/bf-4jpf/drt40_output.txt` - DRT 40 inspection output
- `.beads/traces/bf-4jpf/drt41_output.txt` - DRT 41 inspection output
