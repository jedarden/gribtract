# bf-5jgtym: Test Output Capture for rotated_latlon_gdt1_drt0

## Test Execution Summary

**Test Case:** rotated_latlon_gdt1_drt0 buffer underrun reproduction
**File:** `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`
**Test Binary:** `target/release/examples/test_rotated_latlon_gdt1_drt0`

## Commands Used

```bash
# Primary test execution
RUST_BACKTRACE=1 ./target/release/examples/test_rotated_latlon_gdt1_drt0 2>&1 | tee notes/bf-5jgtym-test-output.txt

# Extended debug execution
RUST_BACKTRACE=full RUST_LOG=debug ./target/release/examples/test_rotated_latlon_gdt1_drt0 2>&1 | tee -a notes/bf-5jgtym-test-output.txt
```

## Error Output

```
✗ Decode error: TooShort { needed: 1, got: 0 }

Buffer underrun details:
  Bytes needed: 1
  Bytes available: 0
  Shortfall: 1
```

## Error Analysis

**Error Type:** `TooShort` buffer underrun
**Error Message:** `TooShort { needed: 1, got: 0 }`
**Shortfall:** Exactly 1 byte missing

## Test Context

- **File size:** 187 bytes
- **Error location:** The decoder attempts to read 1 additional byte beyond available buffer
- **GRIB file structure:** The file contains standard GRIB headers (indicated by "GRIB" magic bytes at offset 0x00)

## File Details

The test fixture is located at: `tests/corpus/small/rotated_latlon_gdt1_drt0.grib2`
This appears to be a minimal GRIB2 file designed to reproduce a specific buffer underrun condition related to rotated latitude/longitude grid definitions with GDT template 1 and DRT template 0.

## Next Steps

This error output has been captured for detailed analysis of the buffer underrun issue in the GRIB decoder implementation, particularly in handling rotated_latlon grid definitions with specific template combinations (GDT1, DRT0).
