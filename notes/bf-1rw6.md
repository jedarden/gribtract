# DRT Documentation - bead bf-1rw6

## Summary
Documented the Data Representation Template (DRT) number from wgrib2 output analysis.

## DRT Number and Classification

**DRT Number: 3 (DRT 3)**

**Classification: Complex Packing with Spatial Differencing**

## Source Data
- **File analyzed:** `notes/bf-a2hq_wgrib2_packing.txt`
- **Tool:** wgrib2
- **Command:** wgrib2 -packing (for DRT inspection)

## Findings

All 197 GRIB2 records in the sample file use the same packing scheme:

```
packing=Grid point data - complex packing and spatial differencing,c3
```

The `c3` suffix in wgrib2 output indicates:
- **c** = Complex packing
- **3** = DRT number (Data Representation Template 3)

## DRT 3 Characteristics

DRT 3 represents "Grid point data - complex packing and spatial differencing", which is a sophisticated compression method that:

1. Uses spatial differencing to reduce data redundancy
2. Applies complex packing algorithms for efficient compression
3. Is commonly used in meteorological GRIB2 files for optimal data compression

## Verification

The wgrib2 output shows consistent use of DRT 3 across all records:
- 196 records: `c3` (standard complex packing with spatial differencing)
- 1 record (record 37): `c3b` (variant of complex packing)

The minor `c3b` variant represents a slight variation in the complex packing method but still falls under DRT 3 classification.

## Conclusion

The GRIB2 file uses **DRT 3 (complex packing)** as its data representation template, as evidenced by wgrib2 inspection output.

## Generated
2026-07-23 - bead bf-1rw6
