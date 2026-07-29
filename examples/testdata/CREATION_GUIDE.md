# Minimal GRIB2 File Creation Guide

This document explains the process of creating minimal GRIB2 test files from full multi-megabyte GRIB2 files, specifically focusing on `minimal_buffer_underrun.grib2`.

## Overview

Full GRIB2 weather data files typically range from 5MB to 121MB and contain extensive meteorological data. For testing specific parser bugs, we can create minimal files that trigger the same conditions while being 99%+ smaller.

### Size Comparison

| File Type | Size | Use Case | Reduction |
|-----------|------|----------|------------|
| Full GRIB2 | 5-121 MB | Real weather data | - |
| Minimal file | 187 bytes | Bug reproduction | 99.9998% smaller |
| Corpus small | ~50 bytes | Basic structure | 99.9999% smaller |

## Full GRIB2 File Structure

A complete GRIB2 file contains these sections in order:

```
Section 0: Indicator Section           (16 bytes)
Section 1: Identification Section      (variable, typically 21+ bytes)
Section 2: Local Use Section          (OPTIONAL, can be 100KB+)
Section 3: Grid Definition Section    (variable, typically 72+ bytes)
Section 4: Product Definition Section (variable, typically 34+ bytes)
Section 5: Data Representation        (variable, typically 22+ bytes)
Section 6: Bitmap Section             (variable, can be MB+ for large grids)
Section 7: Data Section                (variable, typically 50-90% of file size)
Section 8: End Section                 (4 bytes)
```

## What Was Removed

### Section 2: Local Use Section (0 bytes in minimal, up to 100KB+ in full files)

**Purpose**: Center-specific data, local tables, custom metadata

**Why safe to remove**:
- Optional section (many files don't have it)
- Not required for basic parsing
- Not referenced by Section 0-7 parsing logic
- Center-specific data not needed for generic testing

**What was removed**:
- Center-specific table definitions
- Local parameter mappings
- Custom metadata extensions
- Experimental data fields

### Section 6: Bitmap Section (6 bytes in minimal, up to MB in full files)

**Purpose**: Indicates which grid points have valid data

**Why safe to minimize**:
- When bitmap indicator = 0, means "all points have data"
- For 1x1 grid, only 1 bit needed (rounded up to 6 bytes for section overhead)
- Original MB-sized bitmaps for large grids not needed for minimal test

**What was removed**:
- Bitmap for millions of grid points
- Only kept minimum structure (6 bytes) to indicate "no bitmap needed"

### Section 7: Data Section (6 bytes in minimal, 50-90% of file size)

**Purpose**: Actual meteorological data values (temperature, wind, etc.)

**Why safe to minimize**:
- Bug triggers in Section 3, before Section 7 parsing
- Only 1 byte of data needed for structural validity
- Data values don't affect buffer underrun in Section 3

**What was removed**:
- Millions of data values for full grid coverage
- Complex packing algorithms
- Statistical data for each grid point

### Section 8: Repeating Sections (0 bytes in minimal)

**Purpose**: Multi-message GRIB files with multiple products

**Why safe to remove**:
- Only needed for multi-message files
- Single message sufficient for testing
- Parsing logic for sections 0-7 doesn't depend on section 8

## What Was Simplified

### Section 1: Identification Section (21 bytes)

**Original size**: 21-100+ bytes (varies by center)
**Minimal size**: 21 bytes (core fields only)

**Fields kept**:
- Originating center (required)
- Parameter tables (required)
- Reference time (required)

**Fields removed/minimized**:
- Extended metadata fields
- Center-specific identification
- Production process details

### Section 3: Grid Definition Section (67 bytes claimed, 72 bytes needed)

**Original size**: 72-200+ bytes (varies by grid complexity)
**Minimal size**: 67 bytes (but claims 72 to trigger bug)

**Fields kept**:
- Basic grid template structure
- Minimal 1x1 grid dimensions
- Template number identification

**Fields minimized**:
- Grid dimensions reduced to 1x1 (vs millions)
- All lat/lon values zeroed out
- Complex grid projections removed

**The key difference**: Section 3 claims 72 bytes but only provides 67, creating the buffer underrun condition.

### Section 4: Product Definition Section (34 bytes)

**Original size**: 34-100+ bytes
**Minimal size**: 34 bytes (template 4.0 core)

**Fields kept**:
- Product discipline
- Parameter category/number
- Forecast time

**Fields removed**:
- Extended parameter definitions
- Probability/ensemble information
- Statistical processing details

### Section 5: Data Representation (22 bytes)

**Original size**: 22-50+ bytes
**Minimal size**: 22 bytes (simple packing template)

**Fields kept**:
- Simple packing template (template 5.0)
- Basic scale factors (zeroed)

**Fields removed**:
- Complex packing algorithms
- Compression information
- Data scaling details

## Creation Process

### Step 1: Analyze Full File

```bash
# Identify sections and sizes
wgrib2 -s full_file.grib2

# Expected output:
# 1:0:d=2026070100:TEMP:1000 mb:na
# Each field shows sections used
```

### Step 2: Extract Section Structure

```bash
# Dump section information
python3 <<'EOF'
with open('full_file.grib2', 'rb') as f:
    data = f.read()
    
offset = 0
while offset < len(data):
    if data[offset:offset+4] == b'GRIB':
        section_len = int.from_bytes(data[offset+12:offset+16], 'big')
        print(f"Section 0: offset {offset}, length {section_len}")
        offset += 16
    elif offset < len(data):
        section_num = data[offset]
        section_len = int.from_bytes(data[offset+1:offset+5], 'big')
        print(f"Section {section_num}: offset {offset}, length {section_len}")
        offset += section_len
    else:
        break
EOF
```

### Step 3: Identify Critical Sections

For the buffer underrun bug:
- **Critical**: Sections 0-3 (bug triggers here)
- **Needed for structure**: Sections 4-7 (minimal versions)
- **Not needed**: Section 2 (optional), extensive Section 6-7 data

### Step 4: Construct Minimal File

```bash
# Create minimal file by hand-crafting sections
python3 create_minimal_underrun.py
```

Key decisions:
- **Section 0**: Valid GRIB2 indicator
- **Section 1**: Minimal identification (21 bytes)
- **Section 3**: Grid definition with length mismatch (67 vs 72)
- **Sections 4-7**: Minimal valid structure (no extensive data)

### Step 5: Verify Structure

```bash
# Check file is valid GRIB2
wgrib2 -v minimal_buffer_underrun.grib2

# Should show: GRIB edition 2, valid sections
# Should NOT show: structural errors before Section 3
```

### Step 6: Test Bug Trigger

```bash
# Test that underrun is triggered
cd crates/gribtract
cargo run --example testdata/verify_minimal_underrun

# Expected: TooShort error when parsing Section 3
```

## Why This Reduction is Safe

### 1. Section Independence

GRIB2 sections are designed to be largely independent:
- Each section has its own length field
- Sections can be parsed independently (mostly)
- Section order is fixed but content is flexible

### 2. Early Trigger

The buffer underrun occurs early in parsing:
- Section 3 is the 4th section (after 0, 1, and optional 2)
- Bug triggers when reading Section 3 template data
- Later sections (4-7) don't affect the bug

### 3. Template Validation

Grid Definition Templates are well-defined:
- Template 3.0 (Latitude/Longitude) is the simplest
- Requires specific fields but grid size is flexible
- 1x1 grid is valid but minimal

### 4. Parser Behavior

The GRIB2 parser typically:
- Reads sections sequentially
- Validates section structure before content
- Parses templates based on template number
- Only fails when template data is insufficient

## Creating Other Minimal Files

To create minimal files for other bugs:

1. **Analyze bug location**: Which section triggers it?
2. **Identify dependencies**: Which sections must exist first?
3. **Calculate minimal sizes**: What's the smallest valid section?
4. **Add the bug**: How to trigger the condition minimally?
5. **Verify structure**: Is it still valid GRIB2?
6. **Test trigger**: Does it reproduce the bug?

### Example: Section 5 Template Bug

If bug is in Data Representation Template parsing:
- Keep Sections 0-4 minimal
- Craft Section 5 to trigger the bug
- Sections 6-7 can be minimal or omitted

### Example: Section 7 Data Bug

If bug is in data value parsing:
- Keep Sections 0-6 minimal
- Craft Section 7 with specific data pattern
- May need larger Section 7 to trigger bug

## Validation Checklist

Before considering a minimal file complete:

- [ ] File starts with "GRIB" + edition 2
- [ ] All sections up to the bug trigger are valid
- [ ] Total length in Section 0 matches actual file size
- [ ] Section lengths are consistent (except for bug trigger)
- [ ] File reproduces the target bug consistently
- [ ] File is as small as possible while still triggering bug
- [ ] File is documented (hex dump, explanation, creation process)

## Tools for Analysis

### wgrib2

```bash
# Section analysis
wgrib2 -s file.grib2

# Detailed section dump
wgrib2 -v file.grib2

# Hex dump with section markers
wgrib2 -X file.grib2
```

### Python Analysis

```python
import struct

def analyze_grib2(filename):
    with open(filename, 'rb') as f:
        data = f.read()
    
    offset = 0
    sections = []
    
    while offset < len(data):
        if data[offset:offset+4] == b'GRIB':
            edition = data[7]
            total_len = struct.unpack('>Q', data[8:16])[0]
            sections.append(('Indicator', offset, 16, f'GRIB{edition}, total {total_len}'))
            offset += 16
        else:
            sec_num = data[offset]
            sec_len = struct.unpack('>I', data[offset+1:offset+5])[0]
            sections.append((f'Section {sec_num}', offset, sec_len))
            offset += sec_len
    
    return sections
```

### Custom Rust Analysis

```rust
use std::fs;

fn analyze_grib2(path: &str) {
    let data = fs::read(path).expect("Failed to read file");
    let mut offset = 0;
    
    while offset < data.len() {
        if &data[offset..offset+4] == b"GRIB" {
            println!("Section 0 (Indicator): offset={}, size=16", offset);
            offset += 16;
        } else {
            let sec_num = data[offset];
            let sec_len = u32::from_be_bytes([data[offset+1], data[offset+2], 
                                               data[offset+3], data[offset+4]]) as usize;
            println!("Section {}: offset={}, size={}", sec_num, offset, sec_len);
            offset += sec_len as usize;
        }
    }
}
```

## References

- [WMO GRIB2 Specification](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [NCEP GRIB2 Code Tables](https://www.nco.ncep.noaa.gov/pmb/codes/)
- [WGrib2 Documentation](https://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/)
- [GRIB2 Edition 2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
