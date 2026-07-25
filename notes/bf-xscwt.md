# GFS Gaussian-grid Fixture Agreement Percentage (bf-xscwt)

## Task: Determine exact agreement percentage for GFS Gaussian-grid fixture

## Finding: No Agreement Percentage Available

The GFS Gaussian-grid fixtures do not have agreement percentages because neither fixture can currently complete the differential test comparison.

## Fixture Status

### `core_gaussian_gdt40`
- **Status:** `[decode-err]` — decode not implemented
- **Issue:** gribtract does not yet support GDT 3.40 (Gaussian Latitude/Longitude grid)
- **Grid:** 512×256 Gaussian grid, 131,072 points
- **Golden:** ✅ Exists at `tests/corpus/golden/core_gaussian_gdt40.json`
- **Test Result:** Cannot produce agreement percentage due to decode failure

### `gfs_gaussian_gdt40_t1534`
- **Status:** `[no-golden]` — No golden reference exists
- **Issue:** No golden reference file for comparison
- **Grid:** T1534 Gaussian grid (3072×1536, 4.7M points)
- **Golden:** ❌ Does not exist
- **Test Result:** Cannot produce agreement percentage without golden reference

## Overall Test Suite Agreement

While the GFS Gaussian-grid fixtures cannot produce individual agreement percentages, the overall differential test suite shows:

```
Agreement: 11/12 (91.7%)
```

This is from 21 total fixtures (12 comparable, 7 no-golden, 2 skipped-feature).

## Source Documentation

- Test output: `.beads/traces/bf-4pspj/stdout.txt`
- Analysis: `notes/bf-22ucv-gfs-gaussian-test-analysis.md`
- Verification: `notes/bf-4pspj.md`
- Commit: `8141070` (docs(bf-4pspj): document GFS Gaussian-grid fixture verification)

## Conclusion

**Answer:** No agreement percentage exists for GFS Gaussian-grid fixtures because:
1. `core_gaussian_gdt40` fails with decode error (GDT 3.40 not implemented)
2. `gfs_gaussian_gdt40_t1534` has no golden reference to compare against

To obtain agreement percentages for these fixtures, GDT 3.40 decoding must be implemented and golden references must be generated.
