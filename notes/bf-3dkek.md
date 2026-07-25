# GFS Diff Output Capture (bf-3dkek)

Captured corpus diff output for GFS fixtures to `/tmp/gfs_mismatches.txt`.

## Fixtures Tested

1. **gfs_gaussian_gdt40_t1534** - 54 fields decoded, no golden reference
2. **gfs_conus_drt0_0p50** - 696 fields decoded, no golden reference
3. **gfs_tmp2m_1deg_anl** - 1 field, 100% agreement
4. **gfs_anl_t2m_5x5** - 1 field, 100% agreement

## Output File

- Location: `/tmp/gfs_mismatches.txt`
- Size: 124 lines
- Contains: Full corpus diff analysis with field-by-field comparisons

## Notes

The file is ready for further analysis or debugging. Small fixtures with golden references show 100% agreement. Large fixtures don't have golden references yet for comparison.
