# Buffer Underrun Reproduction: rotated_latlon_gdt1_drt0

## Summary
Successfully reproduced the buffer underrun error for the `rotated_latlon_gdt1_drt0.grib2` test file.

## Error Details

**Exact Error Message:**
```
✗ Error: TooShort { needed: 1, got: 0 }
```

**Error Location:**
- Byte position: 67 (0-based indexing in the overall file)
- File section: Section 3 (Grid Definition Section)
- Octet within Section 3: Octet 25 (within the section body)
- Field being read: "minor axis scale + value" (octets 26-30)

**Buffer State at Error:**
- Total file size: 187 bytes
- Current buffer position: 67 bytes
- Bytes requested: 1 byte
- Bytes available: 0 bytes
- Shortfall: 1 byte

## GRIB Message Structure

The file structure at the point of error:

```
Section 0 (Indicator): bytes 0-15 (16 bytes)
  - GRIB identifier: 'GRIB'
  - Edition: 2
  - Total length claimed: 187 bytes

Section 1 (Identification): bytes 16-37 (21 bytes)
  - Length: 21 bytes
  - Spans: bytes 16-37

Section 3 (Grid Definition): starts at byte 37
  - Section length claimed in header: 72 bytes
  - Should span: bytes 37-109
  - Template: GDT 1 (Rotated Latitude/Longitude)
  - Header: 5 bytes (bytes 37-42)
  - Body should start: byte 42
  - Body should end: byte 109
```

## Detailed Section 3 Layout (GDT 1)

The Grid Definition Template 1 (Rotated Latitude/Longitude) layout:

| Octets | File Bytes | Field |
|--------|------------|-------|
| 15 | 42 | shape_of_earth |
| 16-20 | 43-47 | earth radius scale + value |
| 21-25 | 48-52 | major axis scale + value |
| 26-30 | 53-57 | **minor axis scale + value** (ERROR OCCURS HERE) |
| 31-34 | 58-61 | Nx (Ni) - grid points in x direction |
| 35-38 | 62-65 | Ny (Nj) - grid points in y direction |
| 39-42 | 66-69 | La1 - latitude of first grid point |
| 43-46 | 70-73 | Lo1 - longitude of first grid point |
| 47 | 74 | resolution flags |
| 48-51 | 75-78 | La2 - latitude of last grid point |
| 52-55 | 79-82 | Lo2 - longitude of last grid point |
| 56-59 | 83-86 | Di - longitudinal direction increment |
| 60-63 | 87-90 | Dj - latitudinal direction increment |
| 64-67 | 91-94 | latitude of southern pole |
| 68-71 | 95-98 | longitude of southern pole |
| 72 | 99 | angle of rotation |
| 73 | 100 | scanning mode |

**Required bytes for Section 3:**
- Header: 5 bytes
- Body (up to octet 73): 59 more bytes
- Total: 64 bytes minimum

## Root Cause

The Section 3 header claims a length of **72 bytes**, but the actual file only contains:

- Section 3 starts at byte 37
- After 5-byte header, body starts at byte 42
- Error occurs at byte 67 (only 25 bytes into the body)
- File ends at byte 186

**The discrepancy:** The section length field (72 bytes) suggests the section should span bytes 37-109, but the actual data is truncated around byte 67.

## Minimal Reproduction Case

The minimal file that triggers this issue:

1. **File**: `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`
2. **Size**: 187 bytes
3. **Template**: GDT 1 (Rotated Latitude/Longitude)
4. **Error condition**: Section 3 length field (72 bytes) > actual available data (~30 bytes of section body)

## Test Execution

**Command:**
```bash
cargo run --example test_rotated_latlon_gdt1_drt0
```

**Output:**
```
=== ROTATED_LATLON_GDT1_DRT0 BUFFER UNDERRUN REPRODUCTION ===
File size: 187 bytes

=== Starting GRIB2 message decode ===
Message buffer size: 187 bytes
Section 0 parsed: total_len=187 bytes
Current buf.pos after Section 0: 16

=== Section header at pos 37 ===
Bytes remaining in message: 150
Section 3: length=72, starts at pos 37
Section body will span: 42..109

=== BUFFER UNDERRUN in Buf::need ===
Requested: 1 bytes
Available: 0 bytes
Current position: 67
Total buffer length: 67
========================================
✗ Decode error: TooShort { needed: 1, got: 0 }

Buffer underrun details:
  Bytes needed: 1
  Bytes available: 0
  Shortfall: 1
```

## Analysis

This is a **truncated GRIB file** where:

1. The Section 3 length field correctly reports 72 bytes
2. However, the actual file data is truncated at byte 67
3. The parser attempts to read the "minor axis scale + value" field at octet 26 (bytes 53-57)
4. But only has access up to byte 67
5. Result: `TooShort { needed: 1, got: 0 }` error

The error message indicates that at byte position 67, there are **0 bytes available** when trying to read **1 byte**, which is the correct behavior for a truncated file.

## Related Files

- Test file: `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`
- Example code: `crates/gribtract/examples/test_rotated_latlon_gdt1_drt0.rs`
- Parser code: `crates/gribtract-core/src/decode.rs` (function `parse_gdt_1`)
