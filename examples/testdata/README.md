# Minimal GRIB2 Test Data

This directory contains minimal GRIB2 test files used for testing specific parsing conditions.

## Files

### minimal_buffer_underrun.grib2 (187 bytes)

**Purpose**: Triggers buffer underrun vulnerability in GRIB2 parser

**Condition**: Section 3 (Grid Definition Section) claims 72 bytes but only contains 67 bytes, triggering a `TooShort` error when the parser attempts to read GDT template data.

**Verification**:
```bash
cd crates/gribtract
cargo run --example test_underrun
```

Expected output:
```
✓ Buffer underrun successfully triggered: TooShort { needed: 1, got: 0 }
```

**Structure**:
- Section 0 (Indicator): 16 bytes
- Section 1 (Identification): 21 bytes  
- Section 3 (Grid Definition): 72 bytes claimed, 67 actual
- Section 4 (Product Definition): 34 bytes
- Section 5 (Data Representation): 22 bytes
- Section 6 (Bitmap): 6 bytes
- Section 7 (Data): 6 bytes
- End Section: 4 bytes

**Size Comparison**:
- This minimal file: 187 bytes
- Full test files: 5MB - 121MB
- Reduction: 99.6% - 99.8% smaller

**Key Insight**: The 50-byte files in `tests/corpus/small/` do NOT trigger the buffer underrun - they produce `NotImplemented` errors instead. The buffer underrun specifically requires Section 3 with a claimed/actual length mismatch.
