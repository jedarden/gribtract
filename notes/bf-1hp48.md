# GDT 3.30 Lambert-Conformal Projection Verification

## Task
Verify GDT 3.30 (Lambert-conformal projection) in NAM AWIP12 GRIB2 file.

## Files Analyzed
- `data/nam.t00z.awip1200.tm00.grib2` (26M)
- NAM (North American Mesoscale) model output on AWIP12 grid
- Date: 2025-01-15 00Z analysis

## Verification Results

### GDT Number: ✓ CONFIRMED
- **Grid Definition Template: 30** (GDT 3.30)
- This is confirmed by the `grid_template=30` field in the wgrib2 verbose output.

### Projection Type: ✓ CONFIRMED
- **Lambert Conformal Conic** projection
- Explicitly stated as `Lambert Conformal` in the grid description.

### Projection Parameters

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Grid Dimensions** | 614 x 428 | Number of grid points (We:Sn) |
| **Lat1** | 12.190000° | Latitude of first grid point |
| **Lon1** | 226.541000° | Longitude of first grid point |
| **LoV** | 265.000000° | Longitude of central meridian |
| **LatD** | 25.000000° | Latitude of origin (D) |
| **Latin1** | 25.000000° | First standard parallel |
| **Latin2** | 25.000000° | Second standard parallel |
| **LatSP** | -90.000000° | Latitude of southern pole |
| **LonSP** | 0.000000° | Longitude of southern pole |
| **Dx** | 12191.000000 m | Grid spacing in x-direction |
| **Dy** | 12191.000000 m | Grid spacing in y-direction |

### Notes on Projection Parameters

1. **Standard Parallels**: Both Latin1 and Latin2 are 25.0°N, meaning the cone is tangent at a single latitude (25°N). This is a special case of the Lambert Conformal Conic projection where the two standard parallels coincide.

2. **Central Meridian**: 265°W (or equivalently, 95°W) - this runs through the central United States.

3. **Grid Spacing**: Approximately 12.2 km in both x and y directions.

4. **Wind Grid**: The `winds(grid)` notation indicates that the wind components are on the same grid as scalar variables (not staggered).

## Conclusion

The NAM AWIP12 GRIB2 file successfully uses **GDT 3.30 with a Lambert Conformal Conic projection**, as required for this verification task. All projection parameters have been extracted and documented.

## wgrib2 Command Used
```bash
grib2/wgrib2/wgrib2 -V data/nam.t00z.awip1200.tm00.grib2 2>&1 | head -50
```
