# GRIB2 Grid Definition Reference

This document catalogs the grid definition templates (PDT 3.x) and spatial extents found in GRIB2 files analyzed in the gribtract project.

## Grid Definition Templates Identified

### Template 3.0: Regular Latitude-Longitude Grid

**GRIB2 Code: `grid_template=0` / `regular_ll`**

**Example File:** GEFS Ensemble Mean (`gefs_ensemble_mean_20260723_t00z_f000.grib2`)

#### Grid Parameters
| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 1440 | Number of points along x-axis (longitude) |
| Nj | 721 | Number of points along y-axis (latitude) |
| Latitude of first grid point | 90.0°N | Northernmost point |
| Longitude of first grid point | 0.0°E | Prime Meridian |
| Latitude of last grid point | 90.0°S | Southernmost point |
| Longitude of last grid point | 359.75°E | Just west of Prime Meridian |
| i-direction increment | 0.25° | Longitude spacing |
| j-direction increment | 0.25° | Latitude spacing |
| Scanning mode | 64 (01000000₂) | +i, +j direction, consecutive points along j |

#### Spatial Extent
- **Latitude:** -90.0° to +90.0° (full global coverage)
- **Longitude:** 0.0° to 359.75° (full global coverage)
- **Total points:** 1,038,240 (1440 × 721)
- **Coverage:** Global

#### Grid Characteristics
- **Resolution:** 0.25° (~27.75 km at equator)
- **Type:** Regular 2D latitude-longitude grid
- **Projection:** None (geographic coordinates)
- **Section 3 length:** 72 bytes
- **Source grid definition:** 0 (explicit)

---

### Template 3.1: Rotated Latitude-Longitude Grid

**GRIB2 Code: `grid_template=1` / `rotated_ll`**

**Example File:** Rotated Lat-Lon 5x5 (`rotated_latlon_5x5.grib2`)

#### Grid Parameters
| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 5 | Number of points along rotated x-axis |
| Nj | 5 | Number of points along rotated y-axis |
| Latitude of first grid point | 40.0° | In rotated coordinate system |
| Longitude of first grid point | 0.0° | In rotated coordinate system |
| Latitude of last grid point | 0.0° | In rotated coordinate system |
| Longitude of last grid point | 40.0° | In rotated coordinate system |
| i-direction increment | 10.0° | Rotated longitude spacing |
| j-direction increment | 10.0° | Rotated latitude spacing |
| Latitude of south pole | 30.0°N | Location of rotated pole |
| Longitude of south pole | 0.0°E | Location of rotated pole |
| Angle of rotation | 0.0° | Rotation angle |

#### Spatial Extent (Rotated Coordinates)
- **Rotated latitude:** 0.0° to 40.0°N
- **Rotated longitude:** 0.0° to 40.0°E
- **Total points:** 25 (5 × 5)
- **Coverage:** Regional (in rotated space)

#### Grid Characteristics
- **Resolution:** 10.0° (coarse test fixture)
- **Type:** Rotated latitude-longitude grid
- **Projection:** Rotated pole projection
- **Section 3 length:** 84 bytes
- **South pole location:** 30.0°N, 0.0°E (defining rotation)

#### Notes
- Rotated grids are commonly used in regional climate models to place the "pole" outside the domain of interest, reducing polar singularities
- The actual geographic extent requires coordinate transformation

---

### Template 3.30: Lambert Conformal Conic Projection

**GRIB2 Code: `grid_template=30` / `lambert`**

**Example Files:** NAM AWIP12, HRRR CONUS, NDFD

#### Example 1: NAM AWIP12 (12km CONUS)

**File:** `nam.t00z.awip1200.tm00.grib2`

| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 614 | Number of points along x-axis |
| Nj | 428 | Number of points along y-axis |
| Latitude of first grid point (Lat1) | 12.19°N | Lower-left corner (approx) |
| Longitude of first grid point (Lon1) | 226.541°E | Lower-left corner (133.459°W) |
| Latitude of true latitude 1 (Latin1) | 25.0°N | First standard parallel |
| Latitude of true latitude 2 (Latin2) | 25.0°N | Second standard parallel |
| Latitude of origin (LaD) | 25.0°N | Projection origin latitude |
| Longitude of central meridian (LoV) | 265.0°E | 95.0°W |
| Dx (grid spacing) | 12,191 m | ~12.19 km |
| Dy (grid spacing) | 12,191 m | ~12.19 km |
| Latitude of southern pole (LatSP) | -90.0° | South pole for projection |
| Longitude of southern pole (LonSP) | 0.0° | South pole for projection |
| Scanning mode | 64 (01000000₂) | +i is +E, +j is +N |

**Spatial Extent:**
- **Total points:** 262,792 (614 × 428)
- **Approximate geographic coverage:** CONUS and adjacent waters
- **Grid spacing:** ~12 km

#### Example 2: HRRR CONUS (3km CONUS)

**File:** `hrrr.t12z.wrfsfcf00.grib2`

| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 1799 | Number of points along x-axis |
| Nj | 1059 | Number of points along y-axis |
| Latitude of first grid point (Lat1) | 21.1381°N | Lower-left corner |
| Longitude of first grid point (Lon1) | 237.2805°E | Lower-left corner (122.7195°W) |
| Latitude of true latitude 1 (Latin1) | 38.5°N | First standard parallel |
| Latitude of true latitude 2 (Latin2) | 38.5°N | Second standard parallel |
| Latitude of origin (LaD) | 38.5°N | Projection origin latitude |
| Longitude of central meridian (LoV) | 262.5°E | 97.5°W |
| Dx (grid spacing) | 3000 m | 3 km |
| Dy (grid spacing) | 3000 m | 3 km |
| Latitude of southern pole (LatSP) | 0.0° | Equatorial plane (secant) |
| Longitude of southern pole (LonSP) | 0.0° | Greenwich |

**Spatial Extent:**
- **Total points:** 1,905,141 (1799 × 1059)
- **Approximate geographic coverage:** CONUS
- **Grid spacing:** 3 km (high-resolution)

#### Example 3: NDFD CONUS

**File:** `ndfd_temp.grib2`

| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 1073 | Number of points along x-axis |
| Nj | 689 | Number of points along y-axis |
| Latitude of first grid point (Lat1) | 20.192°N | Lower-left corner |
| Longitude of first grid point (Lon1) | 238.446°E | Lower-left corner (121.554°W) |
| Latitude of true latitude 1 (Latin1) | 25.0°N | First standard parallel |
| Latitude of true latitude 2 (Latin2) | 25.0°N | Second standard parallel |
| Latitude of origin (LaD) | 25.0°N | Projection origin latitude |
| Longitude of central meridian (LoV) | 265.0°E | 95.0°W |
| Dx (grid spacing) | 5079.406 m | ~5.08 km |
| Dy (grid spacing) | 5079.406 m | ~5.08 km |
| Latitude of southern pole (LatSP) | -90.0° | South pole for projection |
| Longitude of southern pole (LonSP) | 0.0° | Greenwich |

**Spatial Extent:**
- **Total points:** 739,297 (1073 × 689)
- **Approximate geographic coverage:** CONUS and adjacent areas
- **Grid spacing:** ~5 km

#### Common Lambert Conformal Characteristics
- **Type:** Lambert Conformal Conic projection
- **Section 3 length:** 81 bytes
- **Standard parallels:** Often same (tangent cone) or two distinct parallels (secant)
- **Projection origin:** Central meridian typically through domain center
- **Pole handling:** South pole at -90°,0° for Northern Hemisphere projections
- **Scanning:** West-to-East (+i), South-to-North (+j)

#### Notes
- Lambert Conformal is conformal (preserves angles), widely used for mid-latitude weather models
- All examples use same standard parallel (tangent cone)
- Grid spacing (Dx, Dy) is in meters at the projection plane
- Geographic corner coordinates require inverse projection calculation

---

### Template 3.40: Gaussian Grid

**GRIB2 Code: `grid_template=40` / `regular_gg`**

**Example File:** ECMWF ENSO (`flx.2024011500.grib2`)

#### Grid Parameters
| Parameter | Value | Description |
|-----------|-------|-------------|
| Ni | 512 | Number of longitude points |
| Nj | 256 | Number of latitude points |
| Latitude of first grid point | 89.4629°N | Near North Pole |
| Longitude of first grid point | 0.0°E | Prime Meridian |
| Latitude of last grid point | 89.4629°S | Near South Pole |
| i-direction increment | 0.703125° | Longitude spacing |
| Gaussian N | 128 | Number of latitudes between pole and equator |
| Total points | 131,072 | (512 × 256) |

#### Spatial Extent
- **Latitude:** -89.4629° to +89.4629° (near-global, excludes poles)
- **Longitude:** 0.0° to 359.296875° (global coverage)
- **Total points:** 131,072 (512 × 256)
- **Coverage:** Near-global

#### Grid Characteristics
- **Type:** Regular Gaussian grid
- **Resolution:** ~0.7° longitude, Gaussian quadrature latitudes
- **Projection:** None (geographic coordinates)
- **Section 3 length:** 72 bytes
- **Gaussian N:** 128 (determines latitude spacing via Gaussian quadrature)

#### Notes
- Gaussian grids are used in spectral models for accurate spectral transforms
- Latitude spacing follows Gaussian quadrature (non-uniform, denser near poles)
- Longitude spacing is uniform
- Grid excludes exact poles to avoid singularities

---

## GRIB2 Section 3 Structure

All grid definitions share common Section 3 (Grid Definition Section) structure:

| Field | Description |
|-------|-------------|
| Section length | Octet 1-4: Length of section |
| Number of section | Octet 5: Always 3 for Grid Definition Section |
| Source of grid definition | Octet 6: 0=explicit, 1=template process |
| Number of data points | Octets 7-10: Optional (0 if specified in template) |
| Grid definition template number | Octets 11-12: PDT 3.X number |
| Template data | Octets 13+: Template-specific data |

### Template Number Mapping
| Template Number | Grid Type | Common Usage |
|----------------|-----------|--------------|
| 0 (3.0) | Regular lat-lon | GFS, GEFS, global models |
| 1 (3.1) | Rotated lat-lon | Regional climate models |
| 30 (3.30) | Lambert Conformal | NAM, HRRR, CONUS models |
| 40 (3.40) | Gaussian grid | ECMWF, spectral models |
| 1 (3.1) | Mercator | Some tropical models |
| 20 (3.20) | Polar Stereographic | Polar regional models |

---

## Scanning Mode Bits

The scanning mode (octet 73 in GDT) flags grid point ordering:

| Bit | Value | Meaning |
|-----|-------|---------|
| Bit 1 (0x80) | 0 | Points scan +i direction (left to right) |
| Bit 2 (0x40) | 1 | Points scan +j direction (bottom to top) |
| Bit 3 (0x20) | 0 | Adjacent points in i-direction are consecutive |
| Bit 4 (0x10) | 0 | All rows have same number of points |

**Scanning mode 64 (01000000₂)** means:
- West-to-East scanning (+i)
- South-to-North scanning (+j)  
- Consecutive points in i-direction
- Regular grid (all rows same length)

---

## Tools Used

### wgrib2
```bash
# Dump grid template information
wgrib2 file.grib2 -grid

# Dump Section 3 (Grid Definition Section)
wgrib2 file.grib2 -Sec3

# Check inventory
wgrib2 file.grib2 -inv
```

### grib_ls (from eccodes)
```bash
# Get specific parameters
grib_ls -p gridType,Ni,Nj,latitudeOfFirstGridPointInDegrees file.grib2

# Get Lambert parameters
grib_ls -p LaD,LoV,Latin1,Latin2,DxInMetres,DyInMetres file.grib2

# JSON output
grib_ls -j file.grib2
```

---

## Summary of Grid Types Found

| Grid Type | Template | Files Found | Resolution | Domain |
|-----------|----------|-------------|------------|--------|
| Regular LL | 3.0 | GEFS, GFS | 0.25° (27km) | Global |
| Rotated LL | 3.1 | rotated_latlon_5x5.grib2 | 10° (test) | Regional (rotated) |
| Lambert Conformal | 3.30 | NAM, HRRR, NDFD | 3-12 km | CONUS |
| Gaussian | 3.40 | ECMWF | 0.7° | Near-global |

---

## Notes on Grid Irregularities

1. **Lambert Conformal grids do not store last point coordinates** - These must be calculated via inverse projection from grid indices (Ni-1, Nj-1).

2. **Rotated grids require coordinate transformation** - The stored coordinates are in the rotated coordinate system; geographic coordinates require applying the rotation matrix.

3. **Gaussian grids exclude exact poles** - The first/last latitude points are at ±(90 - Δlat_gaussian), not at the poles themselves.

4. **Longitude wrapping** - Regular lat-lon grids may use 0-360°E or -180-180° conventions; verify before use.

5. **Scanning mode consistency** - All observed files use scanning mode 64 (+i, +j, consecutive-i), but other modes exist in GRIB2 specification.

---

## Document Metadata

**Generated:** 2026-07-23
**Bead ID:** bf-1357i
**Purpose:** Document GRIB2 grid definitions and spatial extents for gribtract project
**Tools:** wgrib2 v3.1.3, grib_ls (eccodes)
