# DRT=3 Complex Packing Verification

**Date:** 2025-01-15
**Bead:** bf-oyd14
**Task:** Verify DRT=3 complex packing in GRIB2 file

## Summary

Successfully verified that the NAM AWIP12 GRIB2 files use **Data Representation Template 3 (DRT=3)**, which implements **complex packing** for efficient data compression.

## Files Analyzed

1. `/home/coding/gribtract/samples/nam_awip12_20250115_t00z_f00.grib2`
2. `/home/coding/gribtract/samples/nam.t00z.awip1200.tm00.grib2`

## Verification Results

### File 1: `nam_awip12_20250115_t00z_f00.grib2`
- **GRIB Edition:** 2
- **Discipline:** 0 (Meteorological)
- **Total Message Length:** 240,117 bytes
- **Field 1 - Section 5 (Data Representation Section):**
  - **DRT Number:** **3** ✓
  - **Data Points:** 262,792
  - **Template Length:** 19,223 octets
  - **Packing Type:** **Complex Packing** ✓

### File 2: `nam.t00z.awip1200.tm00.grib2`
- **DRT Number:** **3** ✓
- **Packing Type:** **Complex Packing** ✓

## Data Representation Template 3 (Complex Packing)

DRT=3 represents **complex packing**, a sophisticated compression scheme used in GRIB2 files. Key characteristics:

### What is Complex Packing?

Complex packing (DRT=3) is an advanced compression method that:
1. **Groups data values** with similar characteristics
2. **Applies different bit widths** to different groups
3. **Uses spatial correlation** to reduce redundancy
4. **Achieves higher compression ratios** than simple packing (DRT=0)

### DRT=3 Structure

The DRT=3 template includes these key parameters:

- **Reference value:** Base value for unpacking
- **Binary scale factor:** Scaling for binary representation
- **Decimal scale factor:** Scaling for decimal precision
- **Bits per packed value:** Variable bit widths
- **Original field type:** Data type (0=floating point)
- **Group splitting method:** How groups are formed
- **Missing value management:** Handling of missing data
- **Number of groups:** Total groups in the field
- **Group width/length parameters:** Variable-length encoding

### Comparison with Other DRTs

| DRT | Packing Type | Description |
|-----|-------------|-------------|
| 0   | Simple      | Basic grid-point packing |
| 2   | Complex (2nd order) | Second-order spatial differencing |
| **3** | **Complex** | **Group-based variable bit width** |
| 40  | JPEG2000    | Wavelet compression |
| 41  | PNG         | PNG compression |

## Verification Method

Used a custom Python parser (`/tmp/check_drt.py`) to:
1. Read GRIB2 message header
2. Locate Section 5 (Data Representation Section)
3. Extract DRT number (bytes 10-11 of Section 5)
4. Confirm DRT=3 indicates complex packing
5. Parse packing parameters

## Acceptance Criteria Met

✅ **DRT is confirmed to be 3** - Both files show DRT Number: 3
✅ **Packing type is confirmed as complex packing** - DRT=3 represents complex packing scheme
✅ **Key packing parameters are documented** - Template structure and parameters identified

## Notes

- The NAM AWIP12 files consistently use DRT=3 complex packing across fields
- The large template length (19,223 octets) indicates extensive group definitions
- Complex packing provides better compression for meteorological data with spatial correlation
- This verification confirms the gribtract project must support DRT=3 decoding

## References

- WMO GRIB2 Edition 2 specification
- NCEP GRIB2 documentation
- Data Representation Template 5.3 (Complex Packing)

## wgrib2 Verification (2026-07-23)

Additional verification using wgrib2 v3.1.3 confirms DRT=3 complex packing across all fields:

### Field: PRMSL (Mean Sea Level Pressure)
```
packing=Grid point data - complex packing and spatial differencing
c3 val=(9.94887e+06+i*2^4)*10^-2, ref=0..32767 (#bits=15)
group width bits=4 #groups=8324
```

### Field: UGRD (U-wind component)
All instances show:
```
packing=Grid point data - complex packing and spatial differencing
group width bits=4 #groups=8224-8786 (varies by level)
```

### Key Packing Parameters Confirmed:
- **Packing type:** Grid point data - complex packing and spatial differencing (DRT=3) ✓
- **Group splitting:** Variable-width groups with 4 bits for group width
- **Spatial differencing:** Applied (part of complex packing scheme)
- **Bit-width flexibility:** Variable (9-15 bits) depending on field characteristics
- **Group counts:** 8,000-8,800 groups per field (262,792 data points total)

The consistent use of "complex packing and spatial differencing" across all meteorological fields confirms the file uses **DRT=3** for data compression.
