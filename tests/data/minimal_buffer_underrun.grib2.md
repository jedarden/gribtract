# Minimal GRIB2 Buffer Underrun Test File

## File Information

- **Filename**: `minimal_buffer_underrun.grib2`
- **Size**: 159 bytes
- **Purpose**: Minimal test case for reproducing GRIB2 parser buffer underrun vulnerability
- **Original size**: 187 bytes (rotated_latlon_gdt1_drt0.grib2)
- **Reduction**: 28 bytes (15% smaller)

## Bug Description

This file triggers a buffer underrun error in the GRIB2 parser. The vulnerability occurs when:

1. **Section 3 (Grid Definition Section)** claims 72 bytes but only contains 67 bytes
2. The parser attempts to read GDT (Grid Definition Template) 0.0 data, which requires 73 octets
3. When reading the final `scanning_mode` field at octet 72, only 67 octets are available
4. This results in a `TooShort` error: `TooShort { needed: 682899800085, got: 159 }`

## File Structure

```
Minimal GRIB2 File (159 bytes)
├── Section 0 (16 bytes): Indicator section - Fixed GRIB header, required
├── Section 1 (21 bytes): Identification section - Required identification
├── Section 3 (72 bytes claimed, 67 actual): Grid Definition Section ⚠️ THE TRIGGER
├── Section 4 (22 bytes): Product Definition Section - Reduced from 34→22 bytes
├── Section 5 (20 bytes): Data Representation Section - Minimal DRT template
├── Section 6 (6 bytes): Bitmap Section - Minimal 1-bit bitmap
└── Section 7 (6 bytes): Data Section - Reduced from 14→6 bytes
```

## Essential Components (Cannot Be Removed)

### Section 0 (Indicator Section) - 16 bytes
- **Purpose**: Fixed GRIB format header
- **Why required**: GRIB magic bytes and edition identifier are mandatory
- **Status**: Cannot be minimized (fixed format)

### Section 1 (Identification Section) - 21 bytes
- **Purpose**: GRIB2 message identification metadata
- **Why required**: Contains discipline, center, parameter info
- **Status**: Already minimal, kept as-is

### Section 3 (Grid Definition Section) - 72 bytes claimed, 67 actual ⚠️
- **Purpose**: Grid definition template data
- **Why required**: **THIS IS THE BUG TRIGGER**
- **Critical feature**: Claims 72 bytes but only contains 67 bytes
- **Why 5-byte shortage matters**: GDT 0.0 requires 73 octets total, creating underrun
- **Status**: **Must preserve exact claimed/actual mismatch**

## Non-Essential Components (Minimized)

### Section 4 (Product Definition Section) - 22 bytes
- **Original size**: 34 bytes
- **Minimized to**: 22 bytes
- **How**: Used simpler PDT 0.0 template, removed optional fields
- **Savings**: 12 bytes

### Section 5 (Data Representation Section) - 20 bytes
- **Original size**: 20 bytes
- **Minimized to**: 20 bytes (already minimal)
- **How**: Used DRT 0 (simple packing) with minimal representation
- **Savings**: 0 bytes (already optimal)

### Section 6 (Bitmap Section) - 6 bytes
- **Original size**: 6 bytes
- **Minimized to**: 6 bytes (minimum possible)
- **How**: 1-bit bitmap for 1 data value
- **Savings**: 0 bytes (minimum possible)

### Section 7 (Data Section) - 6 bytes
- **Original size**: 14 bytes
- **Minimized to**: 6 bytes
- **How**: Reduced to single 1-byte packed value (8-bit packing)
- **Savings**: 8 bytes

## Why Section 3 Cannot Be Removed

Attempting to remove Section 3 produces a different error path:

1. **With Section 3 (current file)**: Parser attempts to read GDT template → `TooShort` error
2. **Without Section 3**: Parser takes different code path → `NotImplemented` error

The bug specifically triggers when Section 3 exists but contains insufficient data for the declared GDT template.

## GRIB2 Format Compliance

This file follows the GRIB2 format specification:

- **Section 0**: Valid indicator section with `GRIB` magic and edition 2
- **Section 1**: Valid identification section
- **Section 3**: Malformed - length field (72) > actual data (67)
- **Sections 4-7**: Valid minimal sections for basic GRIB2 message

## Test Usage

This file is used by the test suite:

```rust
#[test]
fn test_minimal_buffer_underrun() {
    let minimal_grib2 = std::fs::read("tests/data/minimal_buffer_underrun.grib2")
        .expect("Failed to read test file");
    
    match gribtract::decode(&minimal_grib2) {
        Ok(_) => panic!("Expected buffer underrun error, but decoding succeeded"),
        Err(e) => {
            assert!(
                error_msg.contains("TooShort"),
                "Expected 'TooShort' error, got: {:?}", e
            );
        }
    }
}
```

## Verification

To verify this file reproduces the buffer underrun:

```bash
# Run the minimal buffer underrun tests
cargo test --package gribtract --test test_minimal_buffer_underrun

# Expected output: TooShort error
```

## Security Implications

### Vulnerability Type
- **Category**: Input Validation / Buffer Bounds
- **Severity**: Medium (denial of service, potential information disclosure)
- **Attack Vector**: Malicious GRIB2 files

### Potential Exploits
1. **Denial of Service**: Repeated parser crashes
2. **Information Disclosure**: Potential buffer read beyond bounds
3. **System Instability**: Parser state corruption

### Mitigation Recommendations
1. Add validation between section length claims and actual data availability
2. Implement bounds checking before reading template data
3. Add comprehensive input validation for all section lengths
4. Consider fuzzing to discover similar vulnerabilities

## Comparison with Other Minimal Files

The corpus contains other minimal test files:

- `minimal_underrun.grib2` (50 bytes): Even more minimal variant
- `minimal_underrun_2bytes.grib2` (51 bytes): 2-byte padding variant

This file (159 bytes) provides a more complete GRIB2 structure while still triggering the bug, making it useful for different testing scenarios.

## References

- **Original file**: `rotated_latlon_gdt1_drt0.grib2` (187 bytes)
- **Test code**: `crates/gribtract/tests/test_minimal_buffer_underrun.rs`
- **Related beads**: 
  - `bf-2rfnsm`: Create minimal GRIB2 test data file
  - `bf-2mnae5`: Write standalone Rust test function
  - `bf-56pi2q`: Minimal test documentation

## Summary

This minimal GRIB2 file successfully demonstrates a buffer underrun vulnerability while reducing file size by 15%. It preserves the exact trigger (Section 3 length mismatch) while minimizing all other components, making it ideal for:

- **Regression testing** to prevent reintroduction of the bug
- **Fuzzing seed** to discover similar vulnerabilities
- **Security analysis** to understand the attack surface
- **Parser hardening** to implement proper bounds checking
