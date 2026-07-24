# wgrib2 DRT=0 Parsing Results - bf-45ad3

**Date:** 2026-07-23
**Task:** Parse wgrib2 output to confirm DRT=0

## Summary

Successfully parsed wgrib2 output from the HRRR CONUS source file to extract and verify the DRT value for message 45.

## Parsed wgrib2 Output

### DRT (Data Representation Template) Value
```bash
$ wgrib2 data/hrrr.t12z.wrfsfcf00.grib2 -d 45 -packing
45:26793288:packing=Grid point data - simple packing,s
```

**Parsed DRT Value:** `0` (simple packing)

### Output Breakdown
- **Message:** 45
- **Byte Offset:** 26,793,288
- **Packing Type:** "Grid point data - simple packing,s"
- **DRT Template:** 0 (Data Representation Template 5.0)

## Verification

The wgrib2 `-packing` option output `"Grid point data - simple packing,s"` corresponds to:
- **DRT=0** (Template 5.0): Simple packing
- Reference value stored as IEEE 32-bit floating-point
- Binary scaling factor applied
- No spatial differencing
- No complex compression

## Additional Context

From the same message (45):
- **Variable:** MXUPHL (Maximum Updraft Helicity 5000-2000m above ground)
- **Grid:** Lambert Conformal (1799 x 1059, 3km resolution)
- **Coverage:** CONUS (La1=21.138°N)
- **Reference Time:** 2024-06-01 12z

## Acceptance Criteria Status

✅ **Output from wgrib2 -pdrt is parsed** - Packed from `-packing` option output
✅ **DRT value is extracted** - DRT=0 confirmed
✅ **Value is confirmed to be 0** - "Grid point data - simple packing,s" = DRT=0

## Note

The `-pdrt` option mentioned in the task description does not exist in wgrib2. The correct option for DRT/packing information is `-packing`, which outputs the packing type including the DRT template.

## Related Work

- **bf-3j9o6**: Original download and validation of CONUS DRT=0 file
- **bf-76gpy**: Earlier CONUS DRT=0 download (2024-06-01)
