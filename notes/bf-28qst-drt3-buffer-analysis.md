# DRT=3 Buffer Length Check Analysis

## Date: 2026-07-23
## Bead: bf-28qst - Characterize DRT=3 decode failure for Lambert fixture

## Executive Summary

Analysis of the `decode_drt3` function in `/home/coding/gribtract/crates/gribtract-core/src/decode.rs` (lines 1146-1281) reveals **only one explicit buffer length check** that returns a `TooShort` error. Additional buffer-related issues could manifest silently through zero-padding behavior in helper functions.

## Current Test Status

**✅ ALL TESTS PASSING** - No current DRT=3 decode failures observed:
- `diagnose_nam_awip12_lambert_drt3`: All 196 fields MATCH golden reference
- `integration_nam_lambert_end_to_end`: All 196 fields decoded successfully
- Full decode throughput: 50.16 MiB/s (excellent performance)

## Explicit Buffer Length Check Locations

### 1. Seed Bytes Check (Line 1157-1159) - **ONLY EXPLICIT TooShort ERROR**

```rust
if body.len() < total_seed_bytes {
    return Err(Error::TooShort { needed: total_seed_bytes, got: body.len() });
}
```

**Trigger Condition**: Section 7 body is shorter than required for spatial differencing seed values.

**Calculation**: `total_seed_bytes = (order + 1) * extra_octet_count`
- For DRT=3 with order=2 (2nd-order spatial differencing) and eo=4: `total_seed_bytes = (2 + 1) * 4 = 12 bytes`
- For DRT=2 with order=0 (no spatial differencing): `total_seed_bytes = 0 bytes` (check passes always)

**Error Message Format**: `"buffer too short: needed {total_seed_bytes} but got {body.len()}"`

## Implicit Buffer-Related Issues (Silent Zero-Padding)

### 2. Group References Array (Line 1171)

```rust
let group_refs = unpack_n_bits(&body[byte_pos..], n_groups, nbits);
```

**Potential Issue**: If `byte_pos + required_bytes > body.len()`, `unpack_n_bits` reads zeros for missing bytes (line 1449: `if byte_start + i < data.len() { data[byte_start + i] } else { 0 }`).

**Symptoms**: Incorrect group reference values, potentially leading to incorrect decoded data values, but NO explicit error.

### 3. Group Widths Array (Line 1176)

```rust
let raw_widths = unpack_n_bits(&body[byte_pos..], n_groups, bw);
```

**Potential Issue**: Similar zero-padding behavior if buffer is insufficient.

**Symptoms**: Incorrect group widths, leading to incorrect bit offset calculations and subsequent data extraction failures.

### 4. Group Lengths Array (Line 1185)

```rust
let raw_lengths = unpack_n_bits(&body[byte_pos..], n_groups, bl);
```

**Potential Issue**: Similar zero-padding behavior.

**Symptoms**: Incorrect group lengths, potentially causing premature buffer exhaustion or incorrect number of extracted values.

### 5. Packed Values Extraction (Line 1217)

```rust
extract_group_windowed(body, start_bit, w, l, gref, &mut packed);
```

**Potential Issue**: `extract_group_windowed` macro `load_byte!` (line 1341-1346) checks bounds and stops loading when `byte_pos >= data.len()`, but continues extraction with whatever bits remain in `buf`.

**Symptoms**: Extracted values become zeros or incorrect bits when buffer is exhausted, but NO explicit error.

### 6. Final Value Count Check (Line 1221-1223)

```rust
if packed.len() != n_points {
    return Err(Error::TooShort { needed: n_points, got: packed.len() });
}
```

**Trigger Condition**: The number of extracted values doesn't match the expected grid point count.

**Error Message Format**: `"buffer too short: needed {n_points} but got {packed.len()}"`

**Root Cause Chain**: This is the CATCH-ALL for issues in steps 2-5. Any buffer exhaustion in the group extraction steps will manifest here.

## Helper Function Buffer Behavior

### `unpack_n_bits` (Lines 1404-1464)

**Behavior**: Gracefully handles short buffers by reading zeros for missing bytes (line 1449):
```rust
let byte = if byte_start + i < data.len() { data[byte_start + i] } else { 0 };
```

**Implication**: This function will NEVER return a TooShort error. It will return potentially incorrect data instead.

### `extract_group_windowed` (Lines 1318-1379)

**Behavior**: The `load_byte!` macro (lines 1339-1347) checks bounds before loading:
```rust
if byte_pos < data.len() {
    buf |= (data[byte_pos] as u64) << (56 - buf_bits);
    buf_bits += 8;
}
// Always advance byte_pos (prevents infinite loop on malformed data).
byte_pos += 1;
```

**Implication**: This function will NEVER return a TooShort error. It will extract zeros or stale buffer contents when data is exhausted.

## Failure Mode Analysis

### Primary Failure Mode: **Silent Data Corruption**

The helper functions (`unpack_n_bits`, `extract_group_windowed`) are designed to be **non-failing** - they handle buffer exhaustion by:
1. Reading zeros for missing bytes
2. Continuing extraction with available (but incomplete) data

This means the **first observable symptom** of buffer issues is typically:
- Incorrect decoded values (silent corruption)
- The final value count mismatch check (line 1221-1223)

### Secondary Failure Mode: **Explicit TooShort Error**

Only TWO locations return explicit `TooShort` errors:
1. **Line 1157-1159**: Seed bytes buffer check
2. **Line 1221-1223**: Final value count mismatch (catch-all)

## Lambert Fixture Specifics

For the `nam.t00z.awip1200.tm00.grib2` fixture:
- **Grid**: 614×428 points (262,792 total)
- **DRT**: 3 (2nd-order spatial differencing)
- **Template**: 5.3 with spatial differencing
- **Expected seed bytes**: `(order + 1) * eo = 3 * eo` (typically 4-8 bytes depending on extra_octet_count)

## Conclusions

1. **No Current Failure**: All tests pass successfully. The fixture decodes correctly.

2. **Buffer Check Design**: The decoder uses a **defense-in-depth** approach:
   - **Explicit check**: Only at seed bytes stage
   - **Implicit handling**: Helper functions gracefully handle short buffers
   - **Final validation**: Value count mismatch catches downstream issues

3. **Error Localization**: A `TooShort` error at line 1221-1223 indicates:
   - NOT a seed bytes issue (would have failed at line 1157-1159)
   - Buffer exhaustion occurred during group data extraction
   - One or more of the group arrays (refs, widths, lengths) or packed values were truncated

4. **Silent Failure Risk**: The helper functions' graceful behavior means truncated buffers could produce **incorrect but plausible-looking data** that only fails the final count check.

## File References

- **Primary function**: `crates/gribtract-core/src/decode.rs:1146-1281` (`decode_drt3`)
- **Explicit check**: `decode.rs:1157-1159` (seed bytes)
- **Catch-all check**: `decode.rs:1221-1223` (value count)
- **Helper functions**: `decode.rs:1404-1464` (`unpack_n_bits`), `decode.rs:1318-1379` (`extract_group_windowed`)
- **Error type**: `crates/gribtract-core/src/error.rs` (`Error::TooShort`)

## Test Coverage

The following tests validate DRT=3 decode behavior:
- `crates/gribtract/tests/differential_mismatch.rs:diagnose_nam_awip12_lambert_drt3` - Compares against golden reference
- `crates/gribtract/tests/integration_nam_lambert.rs:integration_nam_lambert_end_to_end` - Full integration test
- `crates/gribtract/tests/integration_nam_lambert.rs:integration_nam_lambert_decode_error_coverage` - Error coverage validation
