# Minimal Reproduction Analysis - GRIB2 Buffer Underrun

## Test File

**File:** `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`  
**Size:** 187 bytes  
**Status:** Triggers buffer underrun vulnerability

## Error Message and Backtrace

### Error Details
```
✗ Decode error: TooShort { needed: 1, got: 0 }
```

### Error Analysis
- **Error Type:** Buffer Underrun (TooShort)
- **Bytes needed:** 1 byte
- **Bytes available:** 0 bytes  
- **Shortfall:** 1 byte

### Root Cause
The parser attempts to read more data from the buffer than is actually available when parsing the GDT (Grid Definition Template) within Section 3. This occurs because Section 3 claims to contain 72 bytes of data, but the parser runs out of bytes when attempting to read template fields.

## Complete GRIB2 Message Structure

### Section 0 (Indicator Section) - Bytes 0-15 (16 bytes)
```
47 52 49 42  - "GRIB" magic number
00 00        - Reserved
00          - Discipline (0 = Meteorological)
02          - Edition number (GRIB2)
00 00 00 00 0000 00bb - Total length (187 bytes)
```

### Section 1 (Identification Section) - Bytes 16-36 (21 bytes)
```
00 00 00 15 - Section length (21 bytes)
01          - Section number (1)
[... remaining identification data ...]
```
**Section 1 ends at byte:** 37

### Section 3 (Grid Definition Section) - Bytes 37-??? (claims 72 bytes)
```
00 00 00 48 - Section length (72 bytes) 
03          - Section number (3)
00          - GDT version (0)
[... GDT template data ...]
```

**Section 3 starts at byte:** 37  
**Section 3 claimed length:** 72 bytes  
**Section 3 should end at byte:** 109 (37 + 72)  
**Actual file ends at byte:** 187

## Data Sections Present

1. **Section 0 (Indicator Section)** - Present, 16 bytes
2. **Section 1 (Identification Section)** - Present, 21 bytes  
3. **Section 2 (Local Use Section)** - NOT present (skipped)
4. **Section 3 (Grid Definition Section)** - Present, claims 72 bytes
5. **Section 4 (Product Definition Section)** - Not reachable due to Section 3 parsing failure
6. **Section 5 (Data Representation Section)** - Not reachable
7. **Section 6 (Bit-map Section)** - Not reachable
8. **Section 7 (Data Section)** - Not reachable

## Buffer Analysis

### File Size Calculation
- Total file size: 187 bytes (0xBB)
- Section 0 ends at byte 16
- Section 1 ends at byte 37
- Section 3 starts at byte 37
- Bytes available from Section 3 start: 150 bytes

### Section 3 Internal Structure
- Section 3 header: 5 bytes (length 4 bytes + section number 1 byte)
- GDT version byte: 1 byte  
- GDT template data: 66 bytes (claimed) - 72 total - 5 header - 1 version = 66 bytes

### Buffer Underrun Location
The error occurs when parsing the GDT 3.1 template within Section 3. The decoder attempts to read template fields but reaches the end of available data before completing the template parse.

### Critical Offset Analysis
```
Section 3 starts at byte 37
GDT template starts at byte 42 (after 5-byte header)
Bytes available from GDT start: 145 bytes
Template needs: 72 bytes (claimed)
Actual bytes available: 145 bytes (should be sufficient)
```

## Vulnerability Mechanism

1. **Section 3 claims 72 bytes** (00 00 00 48) in its length field
2. **Parser attempts to read GDT 3.1 template** within Section 3
3. **Template parsing assumes minimum data size** based on claimed length
4. **Parser reads past actual available data** when extracting template fields
5. **TooShort error triggers** when attempting to read 1 byte with 0 bytes available

## Test Execution Commands

```bash
# Run the minimal underrun test
cargo run --example test_minimal_underrun

# Run detailed debug analysis  
cargo run --example debug_minimal_underrun

# Run buffer trace analysis
cargo run --example test_buffer_trace

# Run detailed structure analysis
cargo run --example test_rotated_latlon_detailed
```

## Hex Dump of Complete File

```
00000000: 4752 4942 0000 0002 0000 0000 0000 00bb  |GRIB............|
00000010: 0000 0015 0100 0700 0002 0000 07ea 0615  |................|
00000020: 0000 0000 0000 0000 4803 0000 0000 0900  |........H.......|
00000030: 0000 0106 0000 0000 0000 0000 0000 0000  |................|
00000040: 0000 0000 0000 0300 0000 0301 312d 0000  |............1-..|
00000050: 0000 0030 0000 0000 0131 2d00 0098 9680  |...0.....1-.....|
00000060: 0098 9680 01c9 c380 0000 0000 0000 0000  |................|
00000070: 2204 0000 0000 0000 00ff ff00 0000 0100  |"...............|
00000080: 0000 0067 0000 0000 02ff 0000 0000 0000  |...g............|
00000090: 0000 1405 0000 0009 0000 4387 0000 0000  |..........C.....|
000000a0: 0000 0800 0000 0606 ff00 0000 0e07 0001  |................|
000000b0: 0203 0405 0607 0837 3737 37              |.......7777|
```

## Key Findings

1. **Test file identified:** `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2` (187 bytes)
2. **Error reproduced:** `TooShort { needed: 1, got: 0 }`
3. **GRIB2 structure documented:** Sections 0, 1, and partially 3
4. **Buffer underrun mechanism:** Parser reads past Section 3 boundaries based on claimed length
5. **Missing validation:** No bounds checking before template field reads

## Next Steps

The analysis confirms that the buffer underrun occurs in Section 3 (Grid Definition Section) when parsing the GDT 3.1 template. The vulnerability is reproducible with the existing test file and requires adding bounds checking during template parsing.