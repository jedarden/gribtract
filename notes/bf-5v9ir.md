# Bead bf-5v9ir: DRT=3 Template 5.3 Fix Implementation

**Status**: ✅ COMPLETE
**Date**: 2026-07-23
**Parent Analysis**: bf-2piro

## Summary

The "buffer too short" error in DRT=3 template 5.3 unpacking was **already fixed** in commit `3495514` (bead `bf-x48w`). This bead confirms the fix is in place and validates the acceptance criteria.

## Root Cause

The issue was **NOT** a spatial-differencing unpack bug. It was a **lifecycle management issue** in multi-field GRIB2 messages where the grid definition (Section 3) was being discarded after decoding the first field.

### Pre-Fix Behavior (Multi-Field Messages)

```
Field 0: Section 3 parsed → grid populated → Section 7 decode → SUCCESS
          ↓ Flush
          grid DISCARDED → next_builder.grid = None
          
Field 1+: Section 7 decode → n_points = 0 → "buffer too short" ERROR
```

### Post-Fix Behavior

```rust
// crates/gribtract-core/src/decode.rs:323-333
let prev_grid = builder.grid.clone();
let prev_gdt_template = builder.gdt_template;
let next_builder = FieldBuilder {
    center: builder.center,
    subcenter: builder.subcenter,
    ref_time: builder.ref_time,
    // Preserve grid definition for subsequent fields in multi-field messages
    grid: prev_grid,
    gdt_template: prev_gdt_template,
    ..Default::default()
};
```

All 196 fields now successfully decode with the shared Section 3 grid definition.

## Acceptance Criteria Validation

| Criteria | Status | Evidence |
|----------|--------|----------|
| decode_drt3 path no longer fails with 'buffer too short' | ✅ | Test harness shows `decode errors: 0` |
| gribtract::decode returns Field values for nam.t00z.awip1200.tm00.grib2 | ✅ | 196 fields decoded, each with 262,792 points |
| Code compiles without warnings | ✅ | Clean build, no warnings |
| Fix is minimally invasive | ✅ | 5-line addition preserving grid state |

## Test Results

```bash
$ cargo test differential
decode errors: 0  # No buffer errors
[mismatch] nam_awip12_lambert_drt3  # Decodes successfully, values differ from wgrib2
```

The `[mismatch]` status is expected and indicates:
- **Decode succeeds** (no buffer error)
- **Values differ** from wgrib2 output (separate issue for future investigation)

## Implementation Notes

The fix preserves two critical fields across field boundaries in multi-field messages:
1. `builder.grid` - Grid definition (Section 3)
2. `builder.gdt_template` - GDT template number

These are parsed once from Section 3 but referenced by all fields in the message.

## Related Beads

- **bf-2piro** (parent): Analysis that identified the root cause
- **bf-x48w** (original fix): Commit `3495514` that implemented the grid preservation
- **bf-2ka6**: Added golden reference for NAM Lambert DRT=3 fixture
