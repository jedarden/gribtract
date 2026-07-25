# GFS Corpus Diff Mismatch Analysis (bf-2x5y1)

## Task Objective
Document specific mismatched fields and parameters from the GFS Gaussian-grid fixture corpus diff output.

## Finding: No Golden References Available

After analyzing the corpus diff output at `/tmp/gfs_mismatches.txt`, **no field-level mismatches could be documented** because golden references do not exist for the large GFS fixtures.

## Fixtures Tested

| Fixture ID | Fields Decoded | Golden Reference Status | Comparison Result |
|------------|----------------|------------------------|-------------------|
| `gfs_gaussian_gdt40_t1534` | 54 | ❌ Not found | No comparison possible |
| `gfs_conus_drt0_0p50` | 696 | ❌ Not found | No comparison possible |
| `gfs_tmp2m_1deg_anl` | 1 | ✅ Exists | 100% agreement |
| `gfs_anl_t2m_5x5` | 1 | ✅ Exists | 100% agreement |

## Diff Output Analysis

The corpus diff tool (`cargo xtask corpus diff <fixture_id>`) successfully decoded the large fixtures:
- 54 fields for `gfs_gaussian_gdt40_t1534` (127 MB GRIB2 file)
- 696 fields for `gfs_conus_drt0_0p50` 

However, when loading the golden reference, the tool reported:
```
No golden reference found for 'gfs_gaussian_gdt40_t1534'
Golden references are stored in tests/corpus/golden/<fixture_id>.json
```

## Missing Golden References

The following golden reference files need to be generated before field-level mismatch analysis can proceed:

1. `/home/coding/gribtract/tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`
2. `/home/coding/gribtract/tests/corpus/golden/gfs_conus_drt0_0p50.json`

## Small Fixture Baseline

The smaller GFS fixtures with existing golden references show **100% agreement**, indicating that the GRIB decoding logic for simpler GFS products is working correctly:
- `gfs_tmp2m_1deg_anl`: 1 field, perfect match
- `gfs_anl_t2m_5x5`: 1 field, perfect match

## Conclusion

**No specific field mismatches to document** - the analysis is blocked by missing golden references for large GFS Gaussian-grid fixtures. To proceed with mismatch documentation, the golden references must first be generated using the corpus generation tool.

## Next Steps (for future work)

1. Generate golden reference for `gfs_gaussian_gdt40_t1534` (54 fields)
2. Generate golden reference for `gfs_conus_drt0_0p50` (696 fields)  
3. Re-run corpus diff to identify actual field-level mismatches
4. Categorize any found mismatches by type (coordinate grids, data values, metadata)
