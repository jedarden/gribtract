# Minimal Buffer Underrun GRIB2 File - Annotated Hex Dump

This document provides an annotated hex dump of `minimal_buffer_underrun.grib2` (187 bytes) to help understand the file structure and identify which bytes trigger the buffer underrun vulnerability.

## Quick Reference

- **File Size**: 187 bytes
- **Vulnerability**: Section 3 claims 72 bytes but only contains 67 bytes
- **Trigger**: Parser attempts to read Grid Definition Template data beyond section bounds
- **Error**: `TooShort { needed: 1, got: 0 }` or undefined behavior

## Complete Annotated Hex Dump

```
OFFSET  HEX                                                       ASCII       SECTION/DESCRIPTION
------  ----                                                       -----       --------------------
000000  47 52 49 42 00 00 00 00 00 00 00 BB                       GRIB.......  Section 0: Indicator (16 bytes)
         ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^
         |   |   |   |   |   |   |   |   |   |   |   |   Total length: 0x000000BB = 187 bytes
         |   |   |   |   |   |   |   |   |   |   |   Edition: 2
         |   |   |   |   |   Discipline: 0 (meteorological)
         |   |   |   Reserved
         Magic "GRIB"

000010  00 01 00 15 00 00 00 00 00 01 00 00 00 00 00 01          ............  Section 1: Identification (21 bytes)
         ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^
         |   |   |   |   |   |   |   |   |   |   |   |   |
         |   |   |   |   |   |   |   |   |   |   |   |   Parameter table version: 1
         |   |   |   |   |   |   |   |   |   |   Local table version: 0
         |   |   |   |   |   |   |   |   Significance of reference time: 0
         |   |   |   |   |   |   |   Reference time: 1970-01-01 00:00
         |   |   |   |   |   Production process: 0
         |   |   |   |   Backward processing center: 0
         |   |   |   Originating center: 0 (NCEP)
         |   Section length: 0x15 = 21 bytes
         Section number: 1

000020  02 00 00 00 48 00 00 00 00 00 00 00 00 00 01 00          ...H........  Section 1 (continued)
         ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^
         |   |   |   |   |   |   |   |   |   |   |   |   Year: 2026
         |   |   |   |   |   |   |   |   |   |   Day: 1
         |   |   |   |   |   |   |   |   Month: 1
         |   Type of processed data: 2 (analysis)
         |   |   Analysis or forecast: 0
         |   Observation: 0

000030  00 03 00 00 00 48 00 00 00 00 00 00 00 00 00 00          ...H........  Section 1 end + Section 3 start
         |   |   |   |   |   |   |   |   |   |   |   |   Hour: 0
         |   Minute: 0
         Second: 0
         Section 3 Grid Definition Section starts here (offset 37)
         ^^ Section number: 3
         ^^^^ Section length: 0x00000048 = 72 bytes (LIES! Only 67 bytes exist)

000040  FF FF 00 00 00 00 00 01 00 00 00 00 00 00 00 01          ............  Section 3: Grid Definition (continued)
         ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^
         |   |   |   |   |   |   |   |   |   |   |   |   Number of points along Y-axis: 1
         |   |   |   |   |   |   |   |   |   |   |   Number of points along X-axis: 1
         |   |   |   |   |   |   |   |   |   |
         |   |   |   |   |   |   |   |   |   Shape of Earth: 0 (spherical)
         |   |   |   |   |   |   Grid units: 0 (angular units)
         |   |   |   |   Source of grid definition: 255 (predefined)
         |   Grid definition template number: 0 (Latitude/Longitude)

000050  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00          ............  Section 3: Grid Definition Template 3.0 data
         |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   Longitude of last grid point: 0
         |   |   |   |   |   |   |   |   |   |   |   |   |   |   Longitude of first grid point: 0
         |   |   |   |   |   |   |   |   |   |   |   |   |   Latitude of last grid point: 0
         |   |   |   |   |   |   Latitude of first grid point: 0
         |   |   |   |   Direction increments given: 0
         |   |   |   Ni: number of points along latitude: 1
         |   |   |   Nj: number of points along longitude: 1
         |   |   |   Grid shape: 0 (rectangular)
         |   Template number: 0

000060  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00          ............  Section 3: More template data
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
         Diagonal directions and scanning mode (zeros)

000070  00 00 00 04 00 00 00 16 00 00 00 00 00 00 00 05          ............  Section 3 end + Section 4 start
         ^^ Section 4 number: 4
         ^^^^ Section 4 length: 0x16 = 22 bytes
         ^^^^ Product definition template number: 0
         ^^ Product discipline: 0
         Parameter category: 0
         Parameter number: 0

000080  00 00 00 00 00 01 01 00 00 00 00 00 00 00 00 00          ............  Section 4: Product Definition Template 4.0
         ^^ Type of generating process: 0
         ^ Background process: 1
         Forecast time: 0
         First surface type: 1 (surface)
         First surface value: 0

000090  00 00 00 00 00 00 00 05 00 00 00 22 00 00 00 00          ............  Section 4 end + Section 5 start
         ^^ Section 5 number: 5
         ^^^^ Section 5 length: 0x16 = 22 bytes
         ^^^^ Data representation template number: 0
         Binary scale factor: 0
         Decimal scale factor: 0

0000A0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00          ............  Section 5: Data Representation Template 5.0
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
         Reference value, number of bits, original field type (zeros)

0000B0  00 00 00 00 00 06 00 00 00 06 00 00 00 06 00 00          ............  Section 5 end + Sections 6/7 start
         ^^ Section 6 number: 6
         ^^^^ Section 6 length: 0x06 = 6 bytes
         Bitmap indicator: 0 (no bitmap)
         ^^ Section 7 number: 7
         ^^^^ Section 7 length: 0x06 = 6 bytes

0000C0  00 01 00 00 00 00 00 00 37 37 37 37                       ....7777     Section 7 end + End Section
         ^^ Data point: 1
         ^^^^^^^^ Padding
         ^^^^^^^^ End marker "7777"
```

## Section Breakdown by Byte Offsets

| Section | Offset Range | Length | Claimed Length | Notes |
|---------|--------------|--------|----------------|-------|
| Section 0 | 0-15 | 16 bytes | 16 bytes | Indicator section (valid) |
| Section 1 | 16-36 | 21 bytes | 21 bytes | Identification (valid) |
| **Section 3** | 37-103 | **67 bytes** | **72 bytes** | **MISMATCH - triggers underrun** |
| Section 4 | 104-137 | 34 bytes | 34 bytes | Product definition (valid) |
| Section 5 | 138-159 | 22 bytes | 22 bytes | Data representation (valid) |
| Section 6 | 160-165 | 6 bytes | 6 bytes | Bitmap (valid) |
| Section 7 | 166-171 | 6 bytes | 6 bytes | Data section (valid) |
| End Section | 172-175 | 4 bytes | 4 bytes | "7777" marker |

## The Vulnerability Explained

### Where the Claimed Length Comes From

```
Offset 40-41: Section 3 length field = 0x00000048 = 72 bytes (big-endian)
```

### Where the Actual Length Ends

```
Section 3 starts at offset 37
Section 3 contains 67 bytes of actual data
Section 3 ends at offset 37 + 67 = 104
But claims to end at offset 37 + 72 = 109
```

### Why This Triggers Underrun

1. **Parser reads Section 3 header**: Trusts the 72-byte claim
2. **Parser reads Grid Definition Template 3.0**: Requires 52 bytes + Nx*Ny
3. **Parser calculates expected section end**: offset 37 + 72 = 109
4. **Parser attempts to read template data**: Expects to read up to offset 109
5. **Reality check**: Section 3 actually ends at offset 104
6. **Result**: Parser reads into Section 4 or triggers bounds check failure

### The Critical Byte Range

```
Offset 103-104: Last byte of actual Section 3 data
Offset 104-105: First byte of Section 4 (which parser shouldn't see)
```

When the parser attempts to read Grid Definition Template data starting from
offset 50, it expects 55 bytes (52 + Nx*Ny where Nx=Ny=1), taking it to:
- Expected end: offset 50 + 55 = offset 105
- Actual section end: offset 37 + 67 = offset 104
- **Underrun**: Parser attempts to read 1 byte beyond section bounds

## Modifying This File

To create similar minimal test files for other conditions:

1. **Modify Section 3 length claim** (offsets 40-41):
   - Set to larger than actual data size to trigger underrun
   - Set to smaller to trigger overread of next section

2. **Modify grid dimensions** (offsets 52-55, 56-59):
   - Increase Nx/Ny to require larger template data size
   - This makes the underrun more severe

3. **Modify template number** (offsets 50-51):
   - Use different Grid Definition Templates
   - Each template has different size requirements

4. **Add/remove sections** to test different code paths:
   - Section 2 (Local Use) for local data parsing
   - Different Section 5 templates for data representation variations

## Verification

To verify the hex dump matches the actual file:

```bash
xxd examples/testdata/minimal_buffer_underrun.grib2 | head -12
```

To test the underrun:

```bash
cd crates/gribtract
cargo run --example testdata/verify_minimal_underrun
```

Expected output:
```
✓ Buffer underrun successfully triggered: TooShort { needed: 1, got: 0 }
```
