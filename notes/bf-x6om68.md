# GFS Gaussian-Grid Fixture Analysis

## Task Completion Summary

Located and documented the GFS Gaussian-grid fixture (`core_gaussian_gdt40`) in the gribtract codebase.

## Fixture Location

**Golden Reference JSON:**
- Path: `/home/coding/gribtract/tests/corpus/golden/core_gaussian_gdt40.json`
- Size: 378.3 MB
- Structure: Complete golden reference with field metadata and values arrays

**Source GRIB2 Data:**
- Manifest ID: `core_gaussian_gdt40`
- Storage: `remote` (not committed to git, lives in `tests/corpus/large/`)
- Source: NOAA CORe Archive (Climate Data Record)
- URL: `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
- Size: 10.5 MB
- SHA-256: `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397`

## Fixture Structure

### Grid Definition (GDT 3.40 - Gaussian Latitude/Longitude)

```json
"grid": {
    "template": 40,              // GDT 3.40 (Gaussian Lat/Lon)
    "num_data_points": 131072,     // Total grid points (512 × 256)
    "nx": 512,                     // Longitudinal points
    "ny": 256,                     // Latitudinal points
    "lat_first": 89.4629,         // Northernmost latitude
    "lon_first": 0,                // Western longitude
    "lat_last": -89.4629,          // Southernmost latitude
    "lon_last": 359.297,          // Eastern longitude
    "di": 0.703125,               // Longitudinal increment (uniform)
    "dj": null,                    // Latitudinal increment (non-uniform Gaussian)
    "scanning_mode": 0,
    "resolution_flags": 48,
    "shape_of_earth": 6
}
```

### Key Characteristics

1. **Grid Type**: GDT 3.40 (Gaussian Latitude/Longitude)
   - Uses Gaussian latitudes based on Legendre polynomial zeros
   - More efficient for spectral models than regular lat/lon grids
   - Superior for global weather prediction models

2. **Parameter**: 
   - Discipline: 0 (Meteorological)
   - Category: 5 (Radiation)
   - Number: 3 (Downward long-wave radiation flux)

3. **Temporal Coverage**:
   - Reference time: 2024-01-15T00:00:00Z
   - Forecast offset: 0 (analysis)
   - Significance: 0 (analysis time)

4. **Source**: NOAA CORe (Climate Data Record) Archive
   - File type: `flx` (flux files from FV3GFS model)
   - Contains radiative fluxes, heat fluxes, land surface data
   - Coverage: 1950-present

5. **Center**: 7 (US NCEP), Subcenter: 3

## How Gaussian Grids Differ from Regular Lat/Lon

### Regular Latitude/Longitude (GDT 0)

```json
"grid": {
    "template": 0,
    "nx": 360,
    "ny": 181,
    "di": 1.0,     // Uniform longitude spacing
    "dj": 1.0,     // Uniform latitude spacing
    "lat_first": 90.0,
    "lat_last": -90.0
}
```

- **Uniform spacing** in both dimensions (di and dj are numbers)
- Regular grid geometry
- Simpler but less efficient for spectral models

### Gaussian Latitude/Longitude (GDT 3.40)

```json
"grid": {
    "template": 40,
    "nx": 512,
    "ny": 256,
    "di": 0.703125,  // Uniform longitude spacing
    "dj": null,       // Non-uniform latitude spacing (Gaussian)
    "lat_first": 89.4629,  // First Gaussian latitude
    "lat_last": -89.4629   // Last Gaussian latitude
}
```

- **Non-uniform latitude spacing** (dj is null)
- Latitudes are zeros of associated Legendre polynomial P_N
- More points near equator, fewer near poles
- Optimized for spectral transform methods
- Better numerical stability for global models

## Implementation Status

**Current Status**: ❌ **Not Yet Implemented**

- Test `diagnose_core_gaussian_gdt40` exists in `/home/coding/gribtract/crates/gribtract/tests/diagnose_gfs_gaussian.rs`
- Test fails with: `Decode error: decode not implemented`
- Golden reference JSON exists and is properly structured
- Source GRIB2 data is available (can be fetched with `cargo xtask corpus fetch --fixture core_gaussian_gdt40`)

**Required Implementation**:

1. **Grid Definition Template 3.40 decoder** in `gribtract-core`
   - Parse Gaussian grid parameters
   - Handle `dj: null` case
   - Implement Gaussian latitude computation (Legendre polynomial zeros)

2. **Gaussian grid support in nearest-point queries**
   - See `GaussianLatLonParams` struct in `gribtract-core/src/types.rs`
   - Currently approximates latitudes as linearly spaced
   - Future optimization: true Gaussian quadrature placement

## Comparison with Other Fixtures

### Similarities

- **Metadata structure**: Same field layout as all golden fixtures
- **GRIB2 sections**: Uses standard Section 3 (GDT) with template 40
- **Data values**: Dense array format in golden JSON
- **Packing**: Uses simple or complex packing (varies by field)

### Differences

| Aspect | Regular Lat/Lon (GDT 0) | Gaussian (GDT 3.40) |
|--------|------------------------|---------------------|
| Grid template | 0 | 40 |
| Latitude spacing | Uniform (dj = number) | Non-uniform (dj = null) |
| Points distribution | Even spacing | Clustered near equator |
| Model use | General purpose | Spectral models (FV3GFS) |
| Computational efficiency | Lower for spectral methods | Higher for spectral methods |
| Implementation | ✅ Complete | ❌ Pending |

## Integration Readiness

**Fixture Status**: ✅ **Ready for Integration**

The fixture is fully prepared for integration:

1. ✅ **Golden reference exists**: 378MB JSON with complete field data
2. ✅ **Manifest entry exists**: Properly configured in `tests/corpus/manifest.json`
3. ✅ **Source data accessible**: Available from NOAA CORe archive (public URL)
4. ✅ **Test infrastructure exists**: Diagnostic test ready for use
5. ✅ **Type definitions ready**: `GaussianLatLonParams` struct defined
6. ❌ **Decoder pending**: GDT 3.40 decoder not yet implemented

## Next Steps for Integration

To fully integrate this fixture:

1. Implement GDT 3.40 decoder in `gribtract-core`
2. Handle Gaussian latitude computation (Legendre polynomial zeros)
3. Update nearest-point query logic for Gaussian grids
4. Run diagnostic test to verify metadata parsing
5. Add integration test for full decode cycle
6. Update differential testing suite to include Gaussian grid validation

## Technical Notes

### Gaussian Grid Mathematics

Gaussian latitudes are computed as the zeros of the Legendre polynomial P_N, where N is the number of parallels between pole and equator:

- For N=128 (ny=256): 128 latitudes in each hemisphere
- Total parallels: 256 (2N)
- Excludes poles in some implementations
- Optimal for spectral transform numerical methods

### Source File Details

- **Filename**: `flx.2024011500.grb`
- **Contents**: Flux files from FV3GFS model
- **Variables**: Radiation, heat fluxes, land surface, soil conditions, cloud layers
- **Archive**: NOAA CORe (1950-present, 3-hourly)

### Why Gaussian Grids Matter

Gaussian grids are the standard for modern numerical weather prediction (NWP) models like:
- NOAA FV3GFS (Finite-Volume Cubed-Sphere Dynamical Core)
- ECMWF IFS (Integrated Forecasting System)
- Other global spectral models

Implementing GDT 3.40 support is therefore critical for gribtract to handle operational weather prediction data.

---

**Task Completed**: Successfully located and documented the GFS Gaussian-grid fixture. The fixture is ready for integration once GDT 3.40 decoder is implemented.
