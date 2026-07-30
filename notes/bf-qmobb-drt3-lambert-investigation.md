# bf-qmobb: DRT=3 Lambert Fixture Investigation - RESOLVED

## Investigation Summary

This bead investigated the historical "buffer too short" decode failure for the Lambert-conformal fixture (nam.t00z.awip1200.tm00.grib2). **The issue has been resolved** and is documented here for reference.

## Current Status: ✅ RESOLVED

- **Fixture**: `nam.t00z.awip1200.tm00.grib2` (NAM 218 grid, Lambert Conformal + DRT=3)
- **Decode Status**: ✅ SUCCESS (196 fields, 0 decode errors)
- **Differential Agreement**: ✅ 100% (8/8 fixtures, 196/196 fields passing)
- **Fix Commit**: `941b631` (2026-07-23)
- **Verification Commit**: `61d879e` (2026-07-23)

## Historical Failure Analysis

### Original Failure Mode

**Error**: "buffer too short" / "attempt to subtract with overflow"  
**Location**: `crates/gribtract-core/src/decode.rs:1226` (pre-fix)  
**Trigger**: NAM awip12 Lambert fixture with DRT=3 template 5.3

### Root Cause

The failure occurred in the DRT=3 complex packing decoder when calculating buffer requirements for packed values:

```rust
// PROBLEMATIC CODE (pre-fix)
let start_bit = byte_pos * 8;
let last_bit = start_bit + total_bits_needed - 1;  // UNDERFLOWS when total_bits_needed = 0
let last_byte = last_bit / 8;
let bytes_needed = last_byte - byte_pos + 1;
```

**The Issue**: When all groups in DRT=3 have zero width (total_bits_needed=0):
- `last_bit = start_bit + 0 - 1 = start_bit - 1` (underflow)
- In debug builds, this causes a panic: "attempt to subtract with overflow"
- In release builds, this wraps to `usize::MAX` and causes "buffer too short" errors

**Why Zero-Width Groups Occur**: The Lambert fixture uses DRT=3 with spatial differencing that produces very small residuals. When all values in a group are identical after differencing, the group width becomes 0 bits.

### The Fix

**Commit**: `941b631 fix(drt3): handle zero-width groups in buffer length calculation`

**Solution**: Added special-case handling for zero-width groups:

```rust
// FIXED CODE
let bytes_needed_packed = if total_bits_needed == 0 {
    0  // Special case: no bits needed → no bytes needed
} else {
    let start_bit = byte_pos * 8;
    let last_bit = start_bit + total_bits_needed - 1;  // Safe: total_bits_needed > 0
    let last_byte = last_bit / 8;
    last_byte - byte_pos + 1
};
```

## Template 5.3 Spatial-Differencing Implementation

### Current Architecture

The DRT=3 decoder (`decode_drt3` in `decode.rs:1125-1285`) implements three-layer defense:

1. **Layer 1**: Explicit seed bytes validation (lines 1157-1159)
2. **Layer 2**: Per-section buffer checks with `check_bytes` helper (lines 1171-1223)
3. **Layer 3**: Final value count validation (lines 1276-1285)

### Unpack Pipeline

```rust
1. Extract seed values (ival1, ival2, minsd) from template 5.3
2. Decode group references (n_groups × bits_per_value)
3. Decode group widths (n_groups × bits_group_widths)  
4. Decode group lengths (n_groups × bits_scaled_group_lengths)
5. Extract packed values using variable-width groups (THE FIX LOCATION)
6. Apply spatial differencing reconstruction (2nd order)
7. Scale values to original range (decimal_scale_factor, binary_scale_factor)
```

### Key Components

- **`unpack_n_bits`**: Extracts n-bit values from byte buffer
- **`extract_group_windowed`**: Handles variable-width groups with bit-level precision
- **`spatial_diff_2nd_order`**: Reconstructs values from 2nd-order spatial differences
- **`apply_scaling`**: Converts decoded integers to float values

## Verification Results

### End-to-End Decode

```bash
$ cargo run --bin gribtract -- decode samples/nam.t00z.awip1200.tm00.grib2
# Result: 196 fields decoded successfully
# Grid: 614×428 (262,792 points per field)
# Projection: Lambert Conformal (LaD=25°N, LoV=265°E)
```

### Test Suite

- ✅ `diagnose_nam_awip12_lambert_drt3`: PASS (196/196 fields)
- ✅ `verify_lambert_gdt30_metadata_population`: PASS  
- ✅ `differential_coverage_report`: PASS (100% agreement)
- ✅ All inline differential fixtures: PASS

### Golden Reference Agreement

Compared against eccodes-generated golden reference:
- **GDT=30 PDT=0 DRT=3**: 187/187 fields passing (100%)
- **GDT=30 PDT=8 DRT=3**: 9/9 fields passing (100%)
- **Total Decode Errors**: 0

## Code References

### Key Files

- **`crates/gribtract-core/src/decode.rs:1125-1285`**: `decode_drt3` function
- **`crates/gribtract-core/src/types.rs:650-780`**: Lambert 3.30 grid parser
- **`crates/gribtract/tests/differential_mismatch.rs:8-71`**: Diagnostic test

### Critical Lines (Post-Fix)

- **Line 1171-1177**: `check_bytes` helper function
- **Line 1180-1183**: Group references buffer check
- **Line 1189-1192**: Group widths buffer check  
- **Line 1200-1203**: Group lengths buffer check
- **Line 1218-1228**: Packed values buffer check **(THE FIX)**
- **Line 1230-1240**: Group extraction loop
- **Line 1242-1270**: Spatial differencing reconstruction

## Related Documentation

- **`notes/bf-x48w-completion.md`**: End-to-end verification documentation
- **`notes/bf-28qst-drt3-buffer-analysis.md`**: Detailed buffer architecture analysis
- **`notes/bf-28qst-root-cause-analysis.md`**: Root cause hypotheses (pre-fix)

## Conclusion

The DRT=3 decode failure for Lambert fixtures was caused by an integer underflow when calculating buffer requirements for zero-width groups. The fix (commit `941b631`) adds special-case handling for this edge case. The decoder is now fully functional and verified against golden reference data with 100% agreement.

**No outstanding issues identified. The fixture decodes correctly.**

---

*Investigation completed: 2026-07-23*  
*Fix commit: 941b631*  
*Verified by: bf-x48w, bf-28qst*