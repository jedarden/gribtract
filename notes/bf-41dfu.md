# Corpus Diff on GFS Fixtures (bf-41dfu)

## Task
Run corpus diff command on GFS fixtures to verify decoder accuracy against golden references.

## Fixtures Tested

### 1. GEFS Ensemble Mean (PDT=2): `gefs_ensemble_mean_pdt48`
- **Status**: ✅ PASS - 100% agreement
- **Fields**: 71/71 matched
- **Grid**: 259,920 points per field
- **Templates**: GDT=0, PDT=2, DRT=3
- **File**: `gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2`

### 2. GEFS Member 01 (PDT=1): `gefs_member01_pdt41`
- **Status**: ✅ PASS - 100% agreement
- **Fields**: 71/71 matched
- **Grid**: 259,920 points per field
- **Templates**: GDT=0, PDT=1, DRT=3
- **File**: `gefs.20240101.00.atmos.pgrb2ap5.gep01.t00z.pgrb2a.0p50.f000.grib2`

## Other GFS Fixtures

### `core_gaussian_gdt40`
- Has golden reference but decoder returns "decode not implemented"
- Requires GDT=40 (Gaussian grid) support

### `gfs_gaussian_gdt40_t1534`
- No golden reference exists yet
- Would need to generate golden reference for comparison

## Diff Output Structure

The `cargo xtask corpus diff <fixture_id>` command provides:

1. **Fixture metadata**:
   - Storage type (inline/remote)
   - File path
   - File size in bytes
   - GRIB edition (always 2 for GRIB2 files)

2. **Decode summary**:
   - Number of fields decoded from the GRIB file

3. **Field-by-field comparison**:
   - GDT/PDT/DRT template numbers
   - Parameter information (discipline, category, number)
   - Grid point count
   - Match status (MATCH, META_MISMATCH, VALUES_MISMATCH, LENGTH_MISMATCH, MASK_MISMATCH)

4. **Summary statistics**:
   - Total fields
   - Matches vs mismatches
   - Agreement percentage

## Commands Used

```bash
cargo xtask corpus list                              # List all fixtures
cargo xtask corpus diff gefs_ensemble_mean_pdt48    # Run diff on ensemble mean
cargo xtask corpus diff gefs_member01_pdt41          # Run diff on member 01
```

## Notes

- Both GEFS fixtures use 0.5-degree global grid (259,920 grid points)
- PDT=2 is used for ensemble mean products
- PDT=1 is used for individual ensemble member products
- All fields matched perfectly, indicating decoder is working correctly for these GFS/GEFS products
