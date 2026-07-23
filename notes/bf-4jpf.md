# DRT Inspection Results for bf-4jpf

## Task Completed: 2026-07-23

## Summary
Inspected GRIB2 files using wgrib2 to identify Data Representation Template (DRT) numbers.

## wgrib2 Command Used
```bash
wgrib2 -Sec5 <file>
```

## Key Findings

### Primary File: NAM AWIP12 (Complex Packing)
**File:** `/home/coding/gribtract/samples/nam.t00z.awip1200.tm00.grib2`
- **DRT:** 5.3 (Section 5, Template 3)
- **Packing Type:** Complex Packing (DRT 3)
- **All 186 records** in this file use DRT 3
- **Data points:** 262,792 per record (most records), 129,654 for some
- **Section 5 length:** 49 bytes

### Comparison Files

#### HRRR File
**File:** `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2`
- **DRT:** 5.3 (Complex Packing)
- **Data points:** 1,905,141 per record

#### Simple Packing (DRT 2)
**File:** `/home/coding/gribtract/tests/corpus/small/drt2_simple_3x3.grib2`
- **DRT:** 5.2 (Simple Packing)
- **Data points:** 9
- **Section 5 length:** 47 bytes

#### JPEG2000 Packing (DRT 40)
**File:** `/home/coding/gribtract/tests/corpus/small/drt40_j2k_3x2.grib2`
- **DRT:** 5.40 (JPEG2000 Packing)
- **Data points:** 6
- **Section 5 length:** 21 bytes

#### PNG Packing (DRT 41)
**File:** `/home/coding/gribtract/tests/corpus/small/drt41_png_3x2.grib2`
- **DRT:** 5.41 (PNG Packing)
- **Data points:** 6
- **Section 5 length:** 21 bytes

## DRT Classification Summary

- **DRT 3 (5.3): Complex Packing** - Used by NAM and HRRR operational model files
- **DRT 2 (5.2): Simple Packing** - Used by simple test files
- **DRT 40 (5.40): JPEG2000 Packing** - Used for grid compression
- **DRT 41 (5.41): PNG Packing** - Used for grid compression

## Full Output
Complete wgrib2 -Sec5 output for the primary NAM file is saved in:
`/home/coding/gribtract/notes/bf-4jpf_drt3_output.txt`

## Conclusion
The primary operational GRIB2 files (NAM, HRRR) use **DRT 3 (Complex Packing)** for data representation. This is the most common packing method for numerical weather prediction model output.
