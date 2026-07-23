# Lambert 3.30 Grid Metadata Verification (bf-3lk5g)

## Summary
Verified that the Lambert Conformal (GDT 3.30) grid metadata is correctly populated after the DRT=3 decode fix. All grid projection parameters are properly extracted from Section 3 of the GRIB2 messages.

## Test Results
Created comprehensive test suite (`crates/gribtract/tests/verify_lambert_grid.rs`) that validates:

1. **Grid template identification**: Correctly identifies GDT 3.30 (Lambert Conformal)
2. **Grid dimensions**: Nx (614), Ny (428), and total points (262,792) are accurate
3. **First point coordinates**: La1 (12.19°N) and Lo1 (226.541°E) are correctly parsed
4. **Lambert projection parameters**:
   - LaD (latitude where Dx/Dy specified): 25°N
   - LoV (central meridian): 265°E
   - Dx (grid spacing in x): 12,191,000 m
   - Dy (grid spacing in y): 12,191,000 m
   - Projection centre: 0 (North Pole)
   - Latin1 (standard parallel 1): 25°N
   - Latin2 (standard parallel 2): 25°N
   - South pole coordinates: -90°, 0°E

## Key Findings
1. **GDT 3.30 parsing is correct**: All Lambert-specific parameters are extracted from Section 3
2. **Consistency across fields**: All 196 fields in the NAM AWIP12 file share identical grid metadata
3. **Lazy decode preservation**: Lazy decode correctly preserves all grid metadata for DRT=3 fields
4. **No DRT=3 interference**: The DRT=3 decode fix did not affect grid metadata parsing

## Verification Method
The test suite uses the actual NAM AWIP12 GRIB2 file (`tests/corpus/large/nam.t00z.awip1200.tm00.grib2`) which contains 196 fields with GDT 3.30 and DRT=3. Each test validates:

- Full decode with `decode_bytes()`
- Lazy decode with `decode_bytes_lazy()`
- Grid geometry consistency across all fields
- Lambert projection parameter correctness

## Detailed Grid Metadata Values

### Common Grid Metadata (from Section 3, GDT 3.30)
- **GDT Template**: 30 (Lambert Conformal Conic)
- **Num data points**: 262,792 (= 614 × 428)
- **Nx** (columns): 614
- **Ny** (rows): 428
- **Lat first** (La1): 12.19° N
- **Lon first** (Lo1): 226.541° E
- **Scanning mode**: 64 (0x40 = +i west→east, +j south→north)
- **Resolution flags**: 56 (0x38)
- **Shape of earth**: 6 (WMO standard sphere, R=6371229 m)

### Lambert Conformal Projection Parameters (GDT 3.30-specific)
- **LaD** (latitude where Dx/Dy specified): 25.0° N
- **LoV** (central meridian/orientation): 265.0° E
- **Dx** (grid spacing x at LaD): 12,191,000 m (~12.19 km)
- **Dy** (grid spacing y at LaD): 12,191,000 m (~12.19 km)
- **Projection centre flag**: 0 (North Pole in plane)
- **Latin1** (1st standard parallel): 25.0° N
- **Latin2** (2nd standard parallel): 25.0° N (tangent cone, Latin1=Latin2)
- **Lat south pole**: -90.0° N
- **Lon south pole**: 0.0° E

## Test Execution Results (2026-07-23)

### Main Test: `verify_nam_lambert_grid_metadata`
✅ **PASSED** - All grid metadata fields populated correctly

Output:
```
=== Field 0 Grid Metadata ===
Template: 30
Num data points: 262792
Nx: 614
Ny: 428
Lat first: 12.19
Lon first: 226.541
Scanning mode: 64
Resolution flags: 56
Shape of earth: 6

=== Lambert Conformal Parameters ===
LaD (latitude where Dx/Dy specified): 25
LoV (central meridian): 265
Dx (m): 12191000
Dy (m): 12191000
Projection centre: 0
Latin1: 25
Latin2: 25
Lat south pole: -90
Lon south pole: 0

=== All Grid Metadata Checks Passed ===
```

### Additional Tests
✅ `verify_lazy_decode_preserves_grid_metadata` - PASSED
✅ `verify_all_nam_fields_have_consistent_grid` - PASSED (verified across 196 fields)
✅ All unit tests in `types.rs` for Lambert projection - PASSED

## Conclusion
The Lambert 3.30 grid metadata population is working correctly after the DRT=3 decode fix. All expected fields are populated with reasonable values that match the NAM grid specification.

The `parse_gdt_30()` function in `crates/gribtract-core/src/decode.rs` correctly implements the byte layout for GDT 3.30 as specified in the WMO GRIB2 standard, and the `LambertConformalParams` struct properly stores all projection parameters for use by the projection algorithms.
