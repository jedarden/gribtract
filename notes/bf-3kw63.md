# GFS Gaussian-grid Corpus Diff Analysis (bf-3kw63)

## Task: Run corpus diff analysis on GFS Gaussian-grid fixture

## Finding: Diff Analysis Cannot Be Performed

The `cargo xtask corpus diff` command referenced in the task **does not exist**. The available xtask corpus subcommands are:
- `cargo xtask corpus list` - List all fixtures
- `cargo xtask corpus fetch` - Download remote fixtures

## GFS Gaussian-grid Fixture Status

### `gfs_gaussian_gdt40_t1534` (T1534 Gaussian grid)
- **Status**: Cannot perform diff analysis
- **Issue**: No golden reference exists (`tests/corpus/golden/gfs_gaussian_gdt40_t1534.json` missing)
- **Grid**: T1534 Gaussian grid (3072×1536, 4.7M points)
- **Diagnostic test exists**: `crates/gribtract/tests/diagnose_gfs_gaussian.rs`
- **Test result**: Fails due to missing golden reference

### `core_gaussian_gdt40` (512×256 Gaussian grid)
- **Status**: Cannot perform diff analysis  
- **Issue**: Decode error (GDT 3.40 - Gaussian Latitude/Longitude grid not implemented)
- **Grid**: 512×256 Gaussian grid, 131,072 points
- **Golden**: ✅ Exists at `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)
- **Test result**: Fails during decode before comparison can occur

## Context from Previous Investigation

From bead **bf-xscwt** (completed 2026-07-25):
- **No agreement percentage exists** for GFS Gaussian-grid fixtures
- `core_gaussian_gdt40`: decode error (GDT 3.40 not implemented)
- `gfs_gaussian_gdt40_t1534`: no golden reference
- Overall suite shows 91.7% agreement, but not specific to Gaussian fixtures

## Alternative Diff Methods

### Diagnostic Test Attempt
Attempted to run the diagnostic test:
```bash
cargo test diagnose_gfs_gaussian_gdt40_t1534 -- --nocapture
```

**Result**: FAILED - Test panicked at line 13 because golden reference doesn't exist.

### Regenerate Golden Reference
A regeneration function exists: `regenerate_gfs_gaussian_gdt40_t1534()` in `regenerate_golden.rs`, but:
1. This would generate a golden reference from current decode output
2. Since GDT 3.40 is not implemented, this would likely produce incorrect results
3. Golden references should only be generated from a trusted oracle (NCL, wgrib2, etc.)

## Conclusion

**Answer**: The corpus diff analysis cannot be performed as requested because:
1. The `cargo xtask corpus diff` command does not exist
2. The GFS Gaussian-grid fixtures cannot be compared:
   - `gfs_gaussian_gdt40_t1534`: No golden reference exists
   - `core_gaussian_gdt40`: Decode fails (GDT 3.40 not implemented)

To perform diff analysis on these fixtures:
1. Implement GDT 3.40 (Gaussian Latitude/Longitude grid) decoding
2. Generate golden references from trusted oracles (NCL/wgrib2)
3. Then run diagnostic tests or differential comparison

## Dependencies Met

✓ Depends on completion of bf-xscwt (confirmed agreement < 100% cannot be determined)
✓ Only run if agreement < 100% (condition met - agreement cannot be determined)

## Related Documentation

- `notes/bf-xscwt.md` - GFS Gaussian-grid fixture agreement percentage finding
- `crates/gribtract/tests/diagnose_gfs_gaussian.rs` - Diagnostic test (blocked by missing golden)
- `crates/gribtract/tests/regenerate_golden.rs` - Golden regeneration (blocked by GDT 3.40)
