# Bead bf-51tby: GFS Gaussian-Grid Fixture Analysis

## Task
Analyze test agreement percentage for the GFS Gaussian-grid fixture and identify mismatches.

## Finding
**Cannot calculate agreement percentage - no golden reference exists.**

## Details

### Fixture Information
- **Fixture ID**: `gfs_gaussian_gdt40_t1534`
- **File**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` (122 MB)
- **Decoding**: ✅ Successful (54 fields extracted)
- **Golden Reference**: ❌ Does not exist

### Key Result
The fixture decodes successfully but cannot be tested for agreement because there is no golden reference file at `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`.

Without a golden reference, it is impossible to:
- Calculate an agreement percentage
- Identify specific field mismatches
- Compare metadata or values against a trusted oracle

### Corpus Status
- Total fixtures: 21
- Fixtures with golden references: 13 (testable)
- Fixtures without golden references: 6 (untestable)
- `gfs_gaussian_gdt40_t1534` is one of the 6 untestable fixtures

### Comparison with Related Fixture
There is a related Gaussian-grid fixture `core_gaussian_gdt40`:
- Has golden reference (378 MB)
- Decode error: GDT 3.40 (Gaussian Latitude/Longitude grid) not implemented
- Cannot decode at all

The `gfs_gaussian_gdt40_t1534` fixture has the opposite problem:
- No golden reference
- Decodes successfully (54 fields)
- Cannot measure agreement

### To Enable Testing
A golden reference must be generated using a trusted oracle (wgrib2, ECMWF GRIB API, or NCL) and placed at `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`.

## Output
Analysis saved to `/tmp/gfs_mismatches.txt`
