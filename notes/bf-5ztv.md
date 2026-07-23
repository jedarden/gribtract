# GDT 3.30 Lambert-Conformal Projection Verification

## Task Summary
Verification that the downloaded HRRR GRIB2 file uses Grid Definition Template (GDT) version 3.30 with Lambert-conformal projection.

## File Verified
- **File**: `grib2/hrrr.t12z.wrfsfcf00.grib2`
- **Source**: HRRR (High-Resolution Rapid Refresh) model
- **Date**: 2024-06-01 12Z
- **File**: HRRR surface forecast (f00)

## Verification Results

### ✅ GDT Version Confirmed
- **Grid Template**: 30 (GDT 3.30)
- **Template Number**: 3.30
- **Shape of Earth**: Earth assumed spherical with radius 6371.229 km (NCEP standard)

### ✅ Projection Type Confirmed
- **Projection**: Lambert Conformal Conic
- **Grid Dimensions**: 1799 x 1059 points

### Lambert Conformal Projection Parameters

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Lat1** | 21.138123° | Latitude of first grid point (SW corner) |
| **Lon1** | 237.280472° | Longitude of first grid point (-122.72° or 122.72°W) |
| **LoV** | 262.500000° | Longitude of central meridian (-97.5° or 97.5°W) |
| **LatD** | 38.500000° | Latitude of origin/projection center |
| **Latin1** | 38.500000° | First standard parallel (38.5°N) |
| **Latin2** | 38.500000° | Second standard parallel (38.5°N) - *tangent case* |
| **LatSP** | 0.000000° | Latitude of southern pole (regular, not oblique) |
| **LonSP** | 0.000000° | Longitude of southern pole (regular, not oblique) |
| **Dx** | 3000.000000 m | Grid spacing in x-direction (3 km) |
| **Dy** | 3000.000000 m | Grid spacing in y-direction (3 km) |

### Grid Characteristics
- **Scan Mode**: WE:SN (West to East, South to North)
- **Resolution**: 8 (bits per packed value)
- **Grid Spacing**: 3 km x 3 km
- **Projection**: Tangent Lambert Conformal (single standard parallel at 38.5°N)

### Tool Used
```bash
wgrib2 -grid grib2/hrrr.t12z.wrfsfcf00.grib2
wgrib2 -V grib2/hrrr.t12z.wrfsfcf00.grib2
```

### Key Observations

1. **GDT 3.30 Confirmed**: The grid_template=30 confirms this uses Grid Definition Template 3.30 as specified in NCEP GRIB2 documentation.

2. **Lambert Conformal**: The projection is explicitly identified as "Lambert Conformal" by wgrib2.

3. **Tangent Case**: Latin1 = Latin2 = 38.5°N indicates a tangent Lambert Conformal projection (cone touches the Earth at a single parallel), rather than a secant case (two different parallels).

4. **HRRR Standard**: This matches the standard HRRR grid configuration covering the CONUS (Contiguous United States) at 3 km resolution.

5. **Regular Orientation**: LatSP = 0°, LonSP = 0° confirms this is a regular Lambert Conformal projection (not oblique), with the pole at the North Pole.

## References
- NCEP GRIB2 Grid Definition Template 3.30 specification
- WMO GRIB2 Code Table 3.4
- HRRR model documentation

## Conclusion
✅ **All acceptance criteria met:**
- GDT confirmed as version 3.30 (grid_template=30)
- Projection type confirmed as Lambert-conformal
- All projection parameters documented above
