# Task bf-2iupau: Wire DRT 4 into parse_section5 match arm

## Status: Already Complete

This task was already completed in commit `0683f29a2414363abe308607768129dfeefa6f0e` on 2026-07-27.

## Verification

The `parse_section5` function in `crates/gribtract-core/src/decode.rs` (lines 942-946) already includes:

```rust
4 => {
    // Template 5.4: IEEE 754 32-bit floats.
    let packing = parse_drt_4(&mut b)?;
    Ok((4, packing, None, n_values))
}
```

## Acceptance Criteria Met

- [x] Template number 4 case added to match statement in `parse_section5`
- [x] Case calls `parse_drt_4(&mut b)`
- [x] No longer falls through to `Error::NotImplemented` for DRT 4

No new code changes were required for this task.
