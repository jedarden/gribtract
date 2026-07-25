# GFS Fixtures — Complete Reference

## Overview

GFS (Global Forecast System) fixtures in the gribtract test corpus represent GRIB2 messages from NOAA's Global Forecast System and related NCEP products. These fixtures cover multiple grid types, data representation templates, and use cases from synthetic test fixtures to production operational data.

## Fixture Categories

### 1. Gaussian-Grid Fixtures (GDT 40)

Grid Definition Template 3.40 — Gaussian Latitude/Longitude grids with non-uniform latitude spacing based on Gaussian quadrature.

| Fixture ID | Grid | Resolution | Points | Source | Status |
|------------|------|------------|--------|--------|--------|
| `core_gaussian_gdt40` | T254 | ~0.7° (~70 km) | 131,072 (512×256) | NOAA CORe Archive | ✅ Supported |
| `gfs_gaussian_gdt40_t1534` | T1534 | ~0.12° (~12 km) | 4,718,592 (3072×1536) | NOAA GDAS | ✅ Supported |

**Detailed Documentation**: See [gfs-gaussian-grid-structure.md](gfs-gaussian-grid-structure.md)

### 2. Regular Lat/Lon Fixtures (GDT 0)

Grid Definition Template 3.0 — Regular Latitude/Longitude grids with uniform spacing in both dimensions.

#### Synthetic Fixtures

| Fixture ID | Grid | Resolution | Points | Purpose |
|------------|------|------------|--------|---------|
| `gfs_anl_t2m_5x5` | 5×5 | 10° | 25 | Minimal synthetic test fixture |
| `conus_drt0` | 13×8 | 5° | 104 | CONUS regional test fixture |

#### Production Fixtures

| Fixture ID | Grid | Resolution | Points | DRT | Status |
|------------|------|------------|--------|-----|--------|
| `gfs_tmp2m_1deg_anl` | 360×181 | 1.0° | 65,160 | 3 | ⚠️ DRT 3 pending |
| `gfs_conus_drt0_0p50` | 720×361 | 0.5° | 259,920 | 0 | ✅ Supported |

**Grid Structure (GDT 0)**:

```json
{
  "grid": {
    "template": 0,
    "num_data_points": 65160,
    "nx": 360,
    "ny": 181,
    "lat_first": 90.0,
    "lon_first": 0.0,
    "lat_last": -90.0,
    "lon_last": 359.0,
    "di": 1.0,
    "dj": 1.0,
    "scanning_mode": 0,
    "resolution_flags": 48,
    "shape_of_earth": 6
  }
}
```

### 3. Rotated Lat/Lon Fixtures (GDT 1)

Grid Definition Template 3.1 — Rotated Latitude/Longitude grids.

| Fixture ID | Grid | Resolution | Points | Purpose |
|------------|------|------------|--------|---------|
| `rotated_latlon_5x5` | 5×5 | 10° | 25 | Rotated grid test fixture |

## Data Representation Templates (DRT)

### DRT 0 — Simple Packing

Grid point data - simple packing (Section 5 template 5.0).

**Structure**:
```json
{
  "packing": {
    "reference_value": 270.0,
    "binary_scale_factor": 0,
    "decimal_scale_factor": 0,
    "bits_per_value": 8,
    "original_field_type": 0
  }
}
```

**Formula**: `unpacked_value = (R + (packed_value × 2^E)) / 10^D`

Where:
- R = reference_value
- E = binary_scale_factor  
- D = decimal_scale_factor

**Fixtures using DRT 0**:
- `gfs_anl_t2m_5x5` (synthetic)
- `conus_drt0` (synthetic)
- `gfs_conus_drt0_0p50` (production)

### DRT 3 — Complex Packing + Spatial Differencing

Complex packing with spatial differencing (Section 5 template 5.3).

**Characteristics**:
- Group-based encoding for better compression
- Spatial differencing to reduce data variance
- Higher compression ratios than simple packing
- More complex decoding (requires group decoding and differencing)

**Fixtures using DRT 3**:
- `gfs_tmp2m_1deg_anl` (production GFS)
- `nam_awip12_lambert_drt3` (NAM Lambert Conformal)
- `hrrr_conus_drt3_lambert` (HRRR CONUS)

**Status**: ⚠️ DRT 3 decoder support is pending implementation

## Common GFS Parameter Categories

### Discipline 0 (Meteorological) Parameters

| Category | Number | Parameter | Units | Common Levels |
|---------|--------|-----------|-------|---------------|
| 0 | 0 | Temperature (TMP) | K | surface, 2m, isobaric |
| 0 | 1 | Specific humidity (SPFH) | kg/kg | isobaric |
| 0 | 2 | Relative humidity (RH) | % | isobaric, surface |
| 2 | 0 | U-component of wind (UGRD) | m/s | 10m, isobaric |
| 2 | 1 | V-component of wind (VGRD) | m/s | 10m, isobaric |
| 2 | 2 | Wind speed (wind) | m/s | 10m |
| 3 | 0 | Geopotential height (HGT) | gpm | isobaric |
| 1 | 8 | Total precipitation (APCP) | kg/m² | surface |
| 1 | 22 | Snow depth (WEASD) | kg/m² | surface |
| 6 | 1 | Pressure (PRES) | Pa | surface |

### Level Types

| Type Code | Level Type | Description | Example Values |
|-----------|------------|-------------|----------------|
| 1 | Surface | Ground/sea surface | 0 |
| 100 | Isobaric | Pressure levels | 100000, 85000, 50000 Pa |
| 103 | Height above ground | Fixed altitude above surface | 2, 10 m |
| 104 | Depth below land | Below surface | 0-200 cm |
| 105 | Depth below water | Below sea surface | 0-200 m |
| 255 | Not used | Missing/undefined | — |

## Center and Subcenter Codes

| Center | Name | Description |
|--------|------|-------------|
| 7 | US National Weather Service (NCEP) | Standard GFS products |
| 98 | ECMWF | European forecast model |

| Subcenter (Center 7) | Description |
|---------------------|-------------|
| 0 | Generic NCEP |
| 3 | Specific NCEP center designation |

## Common Grid Resolutions

### Global Grids

| Name | Resolution | Grid Size | Coverage | Use Case |
|------|------------|-----------|----------|----------|
| 1-degree | 1.0° | 360×181 | Global | Coarse global analysis |
| 0.5-degree | 0.5° | 720×361 | Global | Standard operational GFS |
| 0.25-degree | 0.25° | 1440×721 | Global | High-resolution GFS |

### Gaussian Grids

| T-Number | N | Resolution | Grid Size | Coverage |
|----------|---|------------|-----------|----------|
| T254 | 128 | ~0.7° | 512×256 | Global |
| T1534 | 768 | ~0.12° | 3072×1536 | Global |

### Regional Grids

| Name | Resolution | Grid Size | Coverage |
|------|------------|-----------|----------|
| CONUS 5° | 5° | 13×8 | 20-55°N, 125-65°W |
| CONUS 0.5° | 0.5° | ~900×~600 | 20-55°N, 125-65°W |

## File Structure Patterns

### Single-Message Files

Fixtures containing a single GRIB2 message (most test fixtures):
- `gfs_anl_t2m_5x5.grib2` (204 bytes)
- `core_gaussian_gdt40` messages (extracted from multi-message files)

### Multi-Message Files

Production files containing multiple GRIB2 messages:
- `gfs.t00z.pgrb2.0p50.f000` (696 messages)
- `gdas.t00z.sfluxgrbf000.grib2` (54 messages)
- `nam.t00z.awip1200.tm00.grib2` (196 messages)

## Provenance and Sources

### NOAA Operational Sources

1. **NOMADS** (NOAA Operational Model Archive and Distribution System)
   - Real-time operational GFS data
   - Public HTTP access
   - Example: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/`

2. **AWS NODD** (NOAA Big Data Program on AWS)
   - Historical and operational data
   - Public S3 buckets
   - Example: `noaa-gfs-bdp-pds.s3.amazonaws.com`

3. **CORe Archive** (Climate Data Record)
   - Google Cloud Storage public bucket
   - 1950-present historical data
   - Example: `storage.googleapis.com/noaa-nws-ncep-core`

4. **NCEP Grids** (Lambert Conformal)
   - AWS NODD S3 buckets
   - Regional high-resolution models
   - Example: `noaa-nam-pds.s3.amazonaws.com`

### Synthetic Fixtures

Generated by project scripts in `scripts/`:
- `gen_grib2.py` — Basic GRIB2 message generation
- `gen_grib2_drt2.py` — DRT 2 (complex packing)
- `gen_grib2_drt40.py` — DRT 40 (JPEG 2000)
- `gen_grib2_drt41.py` — DRT 41 (PNG)
- `gen_grib2_pdt1.py` — PDT 4.1 (ensemble members)
- `gen_grib2_pdt8.py` — PDT 4.8 (accumulations)

## Testing and Validation

### Golden Reference Generation

```bash
# Using eccodes CLI (preferred)
grib_dump -j -d fixture.grib2 > golden.json

# Using eccodes Python bindings
python scripts/gen_golden.py fixture.grib2
```

### Integration Tests

```bash
# Run all GFS-related tests
cargo test --package gribtract gfs

# Test specific fixture
cargo test --package gribtract core_gaussian_gdt40
```

### Validation Status

| Fixture | Grid Template | DRT | Status |
|---------|--------------|-----|--------|
| `gfs_anl_t2m_5x5` | GDT 0 | 0 | ✅ Full support |
| `conus_drt0` | GDT 0 | 0 | ✅ Full support |
| `gfs_conus_drt0_0p50` | GDT 0 | 0 | ✅ Full support |
| `core_gaussian_gdt40` | GDT 40 | varies | ✅ Full support |
| `gfs_gaussian_gdt40_t1534` | GDT 40 | varies | ✅ Full support |
| `gfs_tmp2m_1deg_anl` | GDT 0 | 3 | ⚠️ DRT 3 pending |
| `rotated_latlon_5x5` | GDT 1 | 0 | ⚠️ GDT 1 pending |

## Code Implementation Reference

### Rust Types (crates/gribtract-core/src/types.rs)

```rust
/// Regular Latitude/Longitude grid parameters (GDT 3.0)
#[derive(Debug, Clone, PartialEq)]
pub struct RegularLatLonParams {
    pub nx: u32,
    pub ny: u32,
    pub lat_first: f64,
    pub lon_first: f64,
    pub lat_last: f64,
    pub lon_last: f64,
    pub di: f64,
    pub dj: f64,
}

/// Gaussian Latitude/Longitude grid parameters (GDT 3.40)
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianLatLonParams {
    pub n_parallels: u32,
}
```

## Related Documentation

- [GFS Gaussian-Grid Structure](gfs-gaussian-grid-structure.md) — Detailed Gaussian grid documentation
- [NAM Lambert Conformal Fixtures](bf-4p7j0-nam-lambert-final-state.md) — DRT 3 implementation notes
- [GRIB2 Schema Reference](../schema/README.md) — Complete GRIB2 data structure reference

## References

- WMO GRIB2 Edition 2, Tables 3.0, 3.40 (Grid Definition Templates)
- WMO GRIB2 Edition 2, Tables 5.0, 5.3 (Data Representation Templates)
- NOAA NCEP Documentation: GFS Model Grids and Parameters
- NOAA NWS NCEP CORe Archive Documentation

---

**Last Updated**: 2026-07-25  
**Fixture Categories**: Gaussian-grid (GDT 40), Regular lat/lon (GDT 0), Rotated lat/lon (GDT 1)  
**DRT Coverage**: 0 (simple packing), 3 (complex + spatial differencing)  
**Code Reference**: `crates/gribtract-core/src/types.rs`, `tests/corpus/`
