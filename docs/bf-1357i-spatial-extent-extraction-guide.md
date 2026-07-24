# GRIB2 Spatial Extent Extraction Guide

This guide provides practical methods for extracting and calculating spatial extents from GRIB2 files, with a focus on Lambert Conformal Conic projections where extents are not explicitly stored.

## Quick Reference: Extent Extraction by Grid Type

| Grid Type | First/Last Points Stored? | Extent Method |
|-----------|---------------------------|---------------|
| Regular LL (3.0) | Yes | Direct read |
| Rotated LL (3.1) | Yes | Direct read (rotated coords) |
| Lambert Conformal (3.30) | No | Calculate via projection |
| Gaussian (3.40) | Yes (partial) | Direct read (±ε from poles) |
| Mercator (3.1 variant) | Yes | Direct read |
| Polar Stereographic (3.20) | No | Calculate via projection |

## Method 1: Direct Extraction (Regular Lat-Lon, Gaussian)

For grids where first/last point coordinates are stored in the template:

```bash
# Using grib_ls
grib_ls -p gridType,latitudeOfFirstGridPointInDegrees,longitudeOfFirstGridPointInDegrees,latitudeOfLastGridPointInDegrees,longitudeOfLastGridPointInDegrees file.grib2

# Using wgrib2 (for regular grids)
wgrib2 file.grib2 -grid | grep "lat.*lon.*by"
```

**Example: GEFS Regular Lat-Lon**
```
lat 90.000000 to -90.000000 by 0.250000
lon 0.000000 to 359.750000 by 0.250000
```

**Spatial extent:**
- Min lat: -90.0°
- Max lat: 90.0°
- Min lon: 0.0°
- Max lon: 359.75°

## Method 2: Lambert Conformal Extent Calculation

For Lambert Conformal grids, only the first point (Lat1, Lon1) is stored. The spatial extent must be calculated by converting grid coordinates (i, j) to geographic coordinates (lat, lon).

### Using wgrib2 -grid_out (if available)

```bash
wgrib2 file.grib2 -grid_out
```

This should output corner points, but availability varies by wgrib2 version.

### Using grib_ls with corner point keys

```bash
grib_ls -p latitudeOfFirstGridPointInDegrees,longitudeOfFirstGridPointInDegrees /file.grib2
```

Note: Last corner points return `not_found` for Lambert grids.

### Manual Calculation Method

The spatial extent for a Lambert Conformal grid can be calculated as:

```
Lower-left:  (Lat1, Lon1)           # Stored in template
Lower-right: Calculate at (Ni-1, 0)
Upper-left:  Calculate at (0, Nj-1)
Upper-right: Calculate at (Ni-1, Nj-1)
```

#### Algorithm Outline

1. Extract parameters:
   - Grid dimensions: Ni, Nj
   - First point: Lat1, Lon1
   - Grid spacing: Dx, Dy (in meters)
   - Projection: Latin1, Latin2, LaD, LoV

2. For each corner (i, j):
   - Convert grid coordinates to map coordinates (x, y)
   - Inverse projection from (x, y) to geographic (lat, lon)

3. Determine min/max lat/lon from corners

#### NAM AWIP12 Extent Example

**Stored parameters:**
```
Ni = 614, Nj = 428
Lat1 = 12.19°N, Lon1 = 226.541°E
Latin1 = Latin2 = 25.0°N
LaD = 25.0°N, LoV = 265.0°E (95.0°W)
Dx = Dy = 12191 m
```

**Corners to calculate:**
- Lower-left: (0, 0) → (12.19°N, 133.459°W)
- Lower-right: (613, 0) → Calculate
- Upper-left: (0, 427) → Calculate  
- Upper-right: (613, 427) → Calculate

The actual calculation requires implementing the Lambert Conformal inverse projection equations (see EPSG:9801).

### Using PROJ / pyproj

```python
from pyproj import Transformer, CRS

# Create Lambert Conformal CRS from GRIB parameters
# Note: GRIB stores longitude 0-360°E, convert to -180-180°W
crs = CRS.from_proj4(
    f"+proj=lcc "
    f"+lat_1={Latin1} +lat_2={Latin2} "
    f"+lat_0={LaD} +lon_0={LoV-360} "  # Convert to -180-180
    f"+x_0=0 +y_0=0 "
    f"+a=6371229 +b=6371229 "  # GRS80 sphere used in GRIB2
    f"+units=m +no_defs"
)

# Transform from grid to geographic
transformer = Transformer.from_crs(crs, "EPSG:4326")

# First grid point in meters (relative to origin)
x1 = 0  # i=0
y1 = 0  # j=0
lon1, lat1 = transformer.transform(x1, y1)

# Last grid point in meters
x2 = (Ni - 1) * Dx
y2 = (Nj - 1) * Dy
lon2, lat2 = transformer.transform(x2, y2)
```

## Method 3: Using grib_ls with Native Keys

```bash
# Get all available grid keys
grib_ls -p /gridType file.grib2

# For Lambert, get projection parameters
grib_ls -p /LaD,/LoV,/Latin1,/Latin2,/Nx,/Ny,/DxInMetres,/DyInMetres file.grib2
```

## Spatial Extent Summary by File

### GEFS Ensemble (Regular LL)
- **Min Lat:** -90.0°
- **Max Lat:** 90.0°
- **Min Lon:** 0.0°
- **Max Lon:** 359.75°

### HRRR CONUS (Lambert)
- **First point:** 21.14°N, 122.72°W
- **Approximate extent:** CONUS (exact corners require inverse projection)
- **Dimensions:** 1799 × 1059 points
- **Spacing:** 3 km

### NAM AWIP12 (Lambert)
- **First point:** 12.19°N, 133.46°W
- **Approximate extent:** CONUS + adjacent waters
- **Dimensions:** 614 × 428 points
- **Spacing:** 12 km

### NDFD CONUS (Lambert)
- **First point:** 20.19°N, 121.55°W
- **Approximate extent:** CONUS + adjacent areas
- **Dimensions:** 1073 × 689 points
- **Spacing:** 5 km

### Rotated Lat-Lon (Test)
- **Rotated extent:** 0-40° lat, 0-40° lon (rotated space)
- **South pole:** 30°N, 0°E
- **Geographic extent:** Requires transformation

### Gaussian (ECMWF)
- **Min Lat:** -89.46°
- **Max Lat:** 89.46°
- **Min Lon:** 0.0°
- **Max Lon:** 359.30°

## Tools Summary

| Tool | Strength | Weakness |
|------|----------|----------|
| wgrib2 | Fast, robust inventory | Limited extent calc for projected grids |
| grib_ls | Full parameter access | Requires eccodes install |
| pyproj | Accurate projection | External dependency |
| Manual calc | No dependencies | Complex, error-prone |

## Recommendations

1. **For regular grids:** Use `grib_ls -p ...` with first/last point keys

2. **For Lambert grids:** Use pyproj/PROJ with extracted parameters for accurate extents

3. **For validation:** Compare corner calculations against known domain boundaries

4. **For production:** Cache calculated extents to avoid repeated projection calculations

---

## Notes on GRIB2 Coordinate Conventions

- **Longitude encoding:** GRIB2 typically uses 0-360°E (e.g., 265°E = 95°W)
- **Earth radius:** GRIB2 uses spherical Earth (a = b = 6371229 m) for projections
- **Grid origin:** (i=0, j=0) is typically the lower-left or southwest corner
- **Scanning order:** Most files scan west-to-east (+i), south-to-north (+j)

---

## Document Metadata

**Generated:** 2026-07-23
**Bead ID:** bf-1357i
**Related:** bf-1357i-grid-definition-reference.md
