# Buffer State Analysis at Failure Point

## Task: Analyze buffer state when 'need 1, got 0' error occurs

## Debug Output Captured

### GRIB2 Message Structure
```
=== Starting GRIB2 message decode ===
Message buffer size: 187 bytes
Section 0 parsed: total_len=187 bytes
Current buf.pos after Section 0: 16
=== Section header at pos 16 ===
Bytes remaining in message: 171
Section 1: length=21, starts at pos 16
Section body will span: 21..37
After section 1, buf.pos = 37
=== Section header at pos 37 ===
Bytes remaining in message: 150
Section 3: length=72, starts at pos 37
Section body will span: 42..109
```

### Buffer Underrun Event
```
=== BUFFER UNDERRUN in Buf::need ===
Requested: 1 bytes
Available: 0 bytes
Current position: 67
Total buffer length: 67
========================================
```

## Analysis

### File Information
- **File**: `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`
- **Total size**: 187 bytes
- **Expected total_len from Section 0**: 187 bytes ✓

### Section 3 (Grid Definition) Analysis

#### Section Structure
- **Start position**: 37 (overall message)
- **Header length**: 5 bytes (4-byte length field + 1-byte section number)
- **Body start**: 42 (37 + 5)
- **Claimed length**: 72 bytes
- **Expected body end**: 109 (42 + 67)

#### Buffer State at Failure
- **Body buffer start**: Position 42 in overall message
- **Body buffer length**: 67 bytes (positions 42..108)
- **Current read position**: 67 (relative to body buffer start)
- **Attempting to read**: 1 byte at position 67
- **Available bytes**: 0
- **Failure**: `TooShort { needed: 1, got: 0 }`

### Root Cause

The buffer underrun occurs because:

1. **Section 3 header claims**: Length of 72 bytes
2. **Actual Section 3 body**: Only 67 bytes available
3. **Shortfall**: 5 bytes missing (72 - 67 = 5)
4. **Position offset**: The failure occurs at byte 68 of the expected 72-byte section (0-indexed: position 67)

### Code Location

The failure occurs in `parse_gdt_0()` function when attempting to read the `scanning_mode` field (octet 72 of GDT template 0.0):

```rust
// crates/gribtract-core/src/decode.rs:482
let scanning_mode = b.read_u8()?; // oct 72 - NEEDS 1 BYTE, HAS 0
```

### Buffer Layout

```
Overall message layout:
[0..15]   Section 0 (Indicator):           16 bytes
[16..36]  Section 1 (Identification):      21 bytes
[37..108] Section 3 (Grid Definition):     72 bytes (CLAIMED)
                                          ↓ actual: 67 bytes
[42..108] Section 3 body:                 67 bytes (ACTUAL)
          ↓ failure at relative pos 67 (absolute pos 109)

Section 3 breakdown:
- Position 37: Section length field (4 bytes) = 72
- Position 41: Section number (1 byte) = 3
- Position 42: Body starts (67 bytes available)
- Position 108: Body ends (42 + 67 - 1)
- Position 109: Next section would start here (BUT SECTION 3 CLAIMS TO GO TO 108)
```

### GDT 0.0 Parsing Requirements

GDT template 0.0 requires 73 octets after the template number field:

```
Octets 0-72 (73 total):
  0:    shape_of_earth
  1-5:  earth radius scale + value
  6-10: major axis scale + value
  11-15: minor axis scale + value
  16-19: Ni (nx)
  20-23: Nj (ny)
  24-27: basic angle
  28-31: subdivisions
  32-35: La1 (lat_first)
  36-39: Lo1 (lon_first)
  40:   resolution_flags
  41-44: La2 (lat_last)
  45-48: Lo2 (lon_last)
  49-52: Di
  53-56: Dj
  57-60: lat_last
  61-64: lon_last
  65-68: Di
  69-72: Dj
  73:   scanning_mode ← FAILS HERE
```

### Available vs Required

```
Available body bytes: 67 (octets 0-66)
Required for GDT 0.0: 73 (octets 0-72)
Missing bytes: 6 (octets 67-72)

Individual read failure:
- Trying to read octet 67 (scanning_mode)
- Only have 67 octets (0-66)
- Need 1 more byte
```

## Conclusion

The buffer underrun at 'need 1, got 0' occurs when parsing GDT template 0.0 in Section 3. The section header claims 72 bytes total (67 bytes of body), but GDT 0.0 requires 73 octets of template data. When the parser attempts to read the final `scanning_mode` field at octet 72 (relative to template start), it finds only 67 octets available, resulting in a 1-byte shortfall.

### Key Findings

1. **Debug output is working correctly** - shows exact buffer position and state
2. **Failure occurs at byte 67** (relative to section body start, 0-indexed)
3. **Section 3 body is 67 bytes** but GDT 0.0 requires 73 octets
4. **The 'need 1, got 0' error** means: need 1 more byte, got 0 bytes remaining
5. **Root cause**: Section length field (72) vs actual available data (67 bytes) mismatch

### Test Output

```bash
$ ./target/release/examples/test_rotated_latlon_gdt1_drt0
=== ROTATED_LATLON_GDT1_DRT0 BUFFER UNDERRUN REPRODUCTION ===
File size: 187 bytes

[hex dump omitted]

=== Starting GRIB2 message decode ===
Message buffer size: 187 bytes
Section 0 parsed: total_len=187 bytes
Current buf.pos after Section 0: 16
=== Section header at pos 16 ===
Bytes remaining in message: 171
Section 1: length=21, starts at pos 16
Section body will span: 21..37
After section 1, buf.pos = 37
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

## Status: ✅ COMPLETE

The debug output successfully captures and displays the buffer state at the exact failure point, showing:
- Requested bytes: 1
- Available bytes: 0
- Current position: 67
- Total buffer length: 67
- Section context and structure

This provides all the information needed to diagnose and fix buffer underrun issues.
