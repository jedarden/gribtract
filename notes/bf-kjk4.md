# Task: Identify target GRIB2 test file for DRT inspection

## Target File Selected

**File:** `tests/corpus/small/drt2_simple_3x3.grib2`

## File Details

- **Size:** 217 bytes
- **Last modified:** July 8, 2026 23:29
- **GRIB2 validity:** ✅ Confirmed valid GRIB2 file
- **Content:** Temperature (TMP) at 2 meters above ground, analysis data from 2026-06-21 00:00

## wgrib2 Inventory Output

```
1:0:d=2026062100:TMP:2 m above ground:anl:
```

## Packing Information

```
1:0:packing=Grid point data - complex packing,c1
```

## Why This File?

1. **DRT-named:** Filename contains "drt2" indicating Data Representation Template 2
2. **Small size:** 217 bytes makes it ideal for detailed DRT inspection
3. **Valid GRIB2:** File structure is intact and readable by wgrib2
4. **Simple grid:** 3x3 grid structure makes DRT analysis straightforward
5. **Complex packing:** Uses complex packing, which will demonstrate DRT features

## Available DRT Test Files (Alternative candidates)

- `tests/corpus/small/drt2_simple_3x3.grib2` (217 bytes) - **SELECTED**
- `tests/corpus/small/drt40_j2k_3x2.grib2` (312 bytes)
- `tests/corpus/small/drt41_png_3x2.grib2` (252 bytes)
- `tests/corpus/small/gfswave_arctic_wind_drt40.grib2` (418K)
- `tests/corpus/small/mrms_carib_refl_drt41.grib2` (28K)

## Known Issues

- Full diagnostic `-V` output fails with "internal decode does not support code table 5.4=0"
- Basic inventory and packing options work correctly

## wgrib2 Binary Location

```
./grib2/wgrib2/wgrib2
```

## Recommended wgrib2 Commands for DRT Inspection

```bash
# Basic inventory
./grib2/wgrib2/wgrib2 tests/corpus/small/drt2_simple_3x3.grib2

# Packing information
./grib2/wgrib2/wgrib2 -packing tests/corpus/small/drt2_simple_3x3.grib2

# Section 5 (Data Representation Section) contents
./grib2/wgrib2/wgrib2 -Sec5 tests/corpus/small/drt2_simple_3x3.grib2

# Product Definition Table
./grib2/wgrib2/wgrib2 -pdt tests/corpus/small/drt2_simple_3x3.grib2
```

## Status

✅ **Target file identified and confirmed ready for DRT inspection**
