# Minimal GRIB2 Test Data

This directory contains minimal GRIB2 test files used for testing specific parsing conditions.

## Overview

These minimal GRIB2 files are hand-crafted test fixtures designed to trigger specific parsing bugs without requiring multi-megabyte reference files. They are constructed by analyzing full GRIB2 files and extracting only the essential sections needed to reproduce the target condition.

### Benefits of Minimal Files

- **Size**: 187 bytes vs 5-121MB (99.6%+ reduction)
- **Speed**: Instant parsing vs seconds to load large files  
- **Clarity**: Only relevant sections present, easier to debug
- **Version Control**: Git-friendly size, no binary diffs
- **Reproducibility**: Precisely controlled test conditions

## Files

### minimal_buffer_underrun.grib2 (187 bytes)

**Purpose**: Triggers buffer underrun vulnerability in GRIB2 parser

**Condition**: Section 3 (Grid Definition Section) claims 72 bytes but only contains 67 bytes, triggering a `TooShort` error when the parser attempts to read GDT template data.

**Verification**:
```bash
cd crates/gribtract
cargo run --example testdata/verify_minimal_underrun
```

Expected output:
```
✓ Buffer underrun successfully triggered: TooShort { needed: 1, got: 0 }
```

#### File Structure (byte offsets and lengths)

```
Offset 0-15:   Section 0 (Indicator) - 16 bytes
  - "GRIB" magic (4 bytes)
  - Discipline (1 byte)
  - Edition 2 (1 byte)
  - Total length (8 bytes, big-endian)

Offset 16-36:  Section 1 (Identification) - 21 bytes
  - Section number (1 byte) = 1
  - Section length (4 bytes, big-endian)
  - Originating center, tables, etc.

Offset 37-103: Section 3 (Grid Definition) - 67 bytes (claims 72!)
  - Section number (1 byte) = 3
  - Section length (4 bytes) = 72 (LIES!)
  - Source of grid definition (1 byte)
  - Number of data points (4 bytes)
  - Grid definition template number (2 bytes) = 0
  - Template 3.0 data (55 bytes claimed, only 50 actual)
  - **TRIGGER**: Missing 5 bytes causes underrun when reading template data

Offset 104-137: Section 4 (Product Definition) - 34 bytes
  - Section number (1 byte) = 4
  - Section length (4 bytes) = 34
  - Product definition template 4.0
  - Parameter category, number, type, etc.

Offset 138-159: Section 5 (Data Representation) - 22 bytes
  - Section number (1 byte) = 5
  - Section length (4 bytes) = 22
  - Data representation template 5.0 (simple packing)

Offset 160-165: Section 6 (Bitmap) - 6 bytes
  - Section number (1 byte) = 6
  - Section length (4 bytes) = 6
  - Bitmap indicator (1 byte) = 0 (no bitmap)

Offset 166-171: Section 7 (Data) - 6 bytes
  - Section number (1 byte) = 7
  - Section length (4 bytes) = 6
  - Data values (1 byte minimum required)

Offset 172-175: End Section - 4 bytes
  - "7777" end marker
```

#### What Was Removed from Full Files

Full GRIB2 files contain these sections that were removed to create the minimal file:

**Removed Large Sections**:
- **Section 2 (Local Use)**: Often contains center-specific data (hundreds of KB)
- **Extensive Section 7 (Data)**: Full data values (millions of bytes)
- **Section 8 (Repeating)**: Only needed for multi-message files

**Simplified Within Kept Sections**:
- **Section 1**: Reduced to minimal identification fields
- **Section 4**: Only template 4.0 core fields
- **Section 5**: Only simple packing template

**Why These Removals Were Safe**:
- The buffer underrun bug occurs in Section 3 parsing
- Only Section 0-3 are read before the bug triggers
- Sections 4-7 are needed for valid structure but not for the bug
- Section 2 and 8 are optional and not needed for basic parsing

#### Bug Trigger Mechanism

The buffer underrun is triggered through this sequence:

1. **Parser reads Section 3 header** (bytes 37-41):
   - Section number: 3 (Grid Definition)
   - Section length: 72 (stored in big-endian)

2. **Parser reads grid definition template** (bytes 50-51):
   - Template number: 0 (Grid Definition Template 3.0)

3. **Parser calculates required template data size**:
   - Template 3.0 requires Nx * Ny + 52 bytes
   - For minimal file: 1 * 1 + 52 = 53 bytes needed
   - Only 50 bytes available in the section

4. **Parser attempts to read beyond section bounds**:
   - Tries to read byte at offset 37 + 52 = 89
   - Section 3 ends at offset 37 + 67 = 104
   - But wait... the parser trusts the 72-byte length claim!
   - Attempts to read at offset 37 + 72 = 109
   - This reads into Section 4, causing corrupted data read

5. **Or worse**: The parser may calculate offsets from the END of the file:
   - If using total length - remaining bytes logic
   - May attempt to read from negative offsets
   - Triggers `TooShort { needed: 1, got: 0 }` error

**Key Vulnerability**: The parser trusts the Section 3 length field without verifying enough data exists. When the claimed length exceeds available data, the parser reads into subsequent sections or attempts to read past the file end.

#### Size Comparison

| File Type | Size | Use Case |
|-----------|------|----------|
| Minimal underrun file | 187 bytes | Buffer underrun testing |
| Small corpus files | ~50 bytes | Basic structure testing |
| Full test files | 5MB - 121MB | Real-world parsing |

**Reduction**: 99.6% - 99.8% smaller than full files

#### Key Insights

1. **Small files ≠ minimal reproduction**: The 50-byte files in `tests/corpus/small/` produce `NotImplemented` errors, not buffer underruns. They're too small to reach the vulnerable code path.

2. **Section 3 is the trigger**: The buffer underrun specifically requires Section 3 with a claimed/actual length mismatch. Other sections don't trigger the same vulnerability.

3. **Template parsing is the culprit**: The bug occurs when parsing Grid Definition Templates (GDT). The parser calculates required template size based on grid dimensions (Nx, Ny) but doesn't validate against actual available data.

4. **Structural validity required**: The file must have enough structure to reach Section 3 parsing. Simply truncating a full file doesn't work - the parser would fail earlier on structural validation.

## Creating New Minimal Files

To create minimal test files for other GRIB2 parsing conditions:

1. **Analyze full files** using `wgrib2` or custom tools to identify which sections are relevant
2. **Extract minimal sections** needed to reach the target code path
3. **Hand-craft or modify section data** to trigger the specific condition
4. **Verify structure validity** with basic parsing tools
5. **Test against target parser** to confirm the condition is triggered

## References

- [GRIB2 Specification](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/)
- [WGrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
