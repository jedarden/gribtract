# GFS Gaussian-Grid Fixture Reference

This document provides a comprehensive reference for GFS (Global Forecast System) Gaussian-grid fixtures in the gribtract test corpus.

## Overview

GFS Gaussian-grid fixtures are GRIB2 files from NOAA's Global Forecast System that use **Grid Definition Template 3.40 (GDT 40)** - Gaussian Latitude/Longitude grids. These grids have uniform longitude spacing but non-uniform latitude spacing based on Gaussian quadrature, providing optimal numerical properties for spectral weather models.

### Key Characteristics

- **Grid Definition Template**: 3.40 (Gaussian Lat/Lon)
- **Data Representation Templates**: 0 (simple), 3 (complex spatial differencing), 40 (JPEG2000), 41 (PNG)
- **Product Definition Templates**: 0 (analysis/forecast), 1 (ensemble member), 4.8 (statistical products)
- **Center**: 7 (NCEP - US National Centers for Environmental Prediction)
- **Discipline**: 0 (Meteorological)

---

## Fixture Types and Locations

### Small Fixtures (Committed)

Stored in `tests/corpus/small/` (committed to git):

| Fixture ID | File | Size | Description | GDT | DRT |
|------------|------|------|-------------|-----|-----|
| `gfs_anl_t2m_5x5` | `gfs_anl_t2m_5x5.grib2` | 204 bytes | Minimal synthetic fixture, 5×5 lat/lon grid, 2m temperature analysis | 0 | 0 |
| `gfs_tmp2m_1deg_anl` | `gfs_tmp2m_1deg_anl.grib2` | 47 KB | 1° global analysis (360×181=65,160 points), DRT=3 complex packing | 0 | 3 |
| `gfswave_arctic_wind_drt40` | `gfswave_arctic_wind_drt40.grib2` | 418 KB | GFS Wave Arctic 9km polar stereographic, DRT=40 JPEG2000, 1M+ points | 20 | 40 |

### Large Remote Fixtures (Fetched On-Demand)

Stored in `tests/corpus/large/` (gitignored, fetched via `cargo xtask corpus fetch`):

| Fixture ID | File | Size | Description | Grid T-number | Points |
|------------|------|------|-------------|---------------|--------|
| `core_gaussian_gdt40` | `flx.2024011500.grib2` | 10.5 MB | CORe 3-hourly flux, T254 Gaussian grid | T254 (N=127) | 131,072 |
| `gfs_gaussian_gdt40_t1534` | `gdas.t00z.sfluxgrbf000.grib2` | 122 MB | GDAS surface flux analysis, high-res T1534 | T1534 (N=768) | 4,718,592 |
| `nam_awip12_lambert_drt3` | `nam.t00z.awip1200.tm00.grib2` | 26 MB | NAM Lambert Conformal, 196 messages, DRT=3 | N/A | 262,792 |
| `gefs_member01_pdt41` | `gefs.20240101...gep01...f000.grib2` | 13 MB | GEFS ensemble member #01, 71 messages | 0.5° | Variable |

---

## Data Structure Schema

### Root Structure

```json
{
  "fixture_id": "string",
  "_provenance": "string",
  "fields": [Field, ...]
}
```

### Field Object (GFS-Specific)

Each GFS field contains the following structure:

```json
{
  "center": 7,
  "subcenter": 0,
  "parameter": {
    "discipline": 0,
    "category": 0-255,
    "number": 0-255
  },
  "forecast": {
    "reference_time": {
      "year": 2024,
      "month": 1-12,
      "day": 1-31,
      "hour": 0-23,
      "minute": 0-59,
      "second": 0-59,
      "significance": 0
    },
    "time_range_unit": 1,
    "forecast_offset": 0
  },
  "level": {
    "type1": 100/103/1,
    "scale_factor1": 0,
    "scaled_value1": value,
    "type2": 255,
    "scale_factor2": 0,
    "scaled_value2": 0
  },
  "ensemble": null,
  "grid": {
    "template": 0/40,
    "num_data_points": integer,
    "nx": integer,
    "ny": integer,
    "lat_first": degrees,
    "lon_first": degrees,
    "lat_last": degrees,
    "lon_last": degrees,
    "di": degrees,
    "dj": degrees,
    "scanning_mode": 0x00,
    "resolution_flags": 48,
    "shape_of_earth": 6
  },
  "values": {
    "Dense": [number, ...]
  },
  "gdt_template": 0/40,
  "pdt_template": 0/1/8,
  "drt_template": 0/2/3/40/41,
  "packing": {
    "reference_value": float,
    "binary_scale_factor": integer,
    "decimal_scale_factor": integer,
    "bits_per_value": integer,
    "original_field_type": integer
  }
}
```

---

## Key Components and Fields

### 1. Center Identification

- **center**: `7` (NCEP - US National Weather Service)
- **subcenter**: `0` (NCEP central operations)

### 2. Parameter Definition

| Component | Description | Common Values |
|-----------|-------------|---------------|
| discipline | GRIB2 discipline | `0` (Meteorological) |
| category | Parameter category | `0` (Temperature), `1` (Moisture), `2` (Momentum), `3` (Mass) |
| number | Parameter number | Varies by category (e.g., `0` for temperature) |

**Common GFS Parameters:**
- Temperature: `discipline=0, category=0, number=0`
- U-wind: `discipline=0, category=2, number=0`
- V-wind: `discipline=0, category=2, number=1`
- Geopotential height: `discipline=0, category=3, number=0`

### 3. Forecast Time Structure

```json
"forecast": {
  "reference_time": {
    "year": 2026,
    "month": 6,
    "day": 18,
    "hour": 0,
    "minute": 0,
    "second": 0,
    "significance": 0    // 0=analysis, 1=start of forecast
  },
  "time_range_unit": 1,    // 1=hours
  "forecast_offset": 0      // Hours from reference time
}
```

- **significance**: `0` for analysis (F000), `1` for forecast start
- **time_range_unit**: Typically `1` (hours)
- **forecast_offset`: `0` for analysis, `>0` for forecasts

### 4. Vertical Level Specification

| Level Type | type1 | scaled_value1 | Description |
|------------|-------|---------------|-------------|
| Surface | `1` | `0` | Ground/sea surface |
| 2m Height | `103` | `2` | 2 meters above ground |
| Isobaric | `100` | `50000` | 500 hPa pressure level |
| 250 hPa | `100` | `25000` | 250 hPa (jet stream level) |
| 500 hPa | `100` | `50000` | 500 hPa (mid-troposphere) |

**Note**: Actual level value = `scaled_value * 10^(-scale_factor)`

For example, 500 hPa is represented as:
- `type1: 100`
- `scale_factor1: 0`
- `scaled_value1: 50000`

### 5. Grid Definition (GDT 0 vs GDT 40)

#### GDT 0 - Regular Lat/Lon Grid

```json
"grid": {
  "template": 0,
  "num_data_points": 65160,
  "nx": 360,           // Longitudinal points
  "ny": 181,           // Latitudinal points
  "lat_first": 90.0,   // Northernmost latitude
  "lon_first": 0.0,    // Starting longitude
  "lat_last": -90.0,   // Southernmost latitude
  "lon_last": 359.0,    // Ending longitude
  "di": 1.0,           // Longitudinal increment (degrees)
  "dj": 1.0,           // Latitudinal increment (degrees)
  "scanning_mode": 0,  // +i eastward, -j southward
  "resolution_flags": 48,
  "shape_of_earth": 6  // WGS84 spherical Earth
}
```

#### GDT 3.40 - Gaussian Lat/Lon Grid

```json
"grid": {
  "template": 40,
  "num_data_points": 4718592,
  "nx": 3072,          // Longitudinal points (uniform)
  "ny": 1536,          // Latitudinal points (Gaussian spacing)
  "lat_first": 89.910324,
  "lon_first": 0.0,
  "lat_last": -89.910324,
  "lon_last": 359.882813,
  "di": 0.117,         // Uniform longitude increment
  "dj": 0.117,         // Approximate latitude increment
  "scanning_mode": 0,
  "resolution_flags": 48,
  "shape_of_earth": 6
}
```

**Gaussian Grid Characteristics:**
- **Uniform longitude spacing** (`di` is constant)
- **Non-uniform latitude spacing** based on Gaussian quadrature
- **N parameter**: Number of parallels from pole to equator
  - T254 → N=127 → 2N = 254 parallels
  - T1534 → N=768 → 2N = 1536 parallels
- **Total points**: `nx × ny`
  - T254: 512 × 256 = 131,072 points
  - T1534: 3072 × 1536 = 4,718,592 points

### 6. Data Representation Template (DRT)

| DRT | Template | Description | Typical Use |
|-----|----------|-------------|-------------|
| 0 | 5.0 | Simple packing | Small synthetic fixtures |
| 2 | 5.2 | Complex packing (no spatial) | Medium efficiency |
| 3 | 5.3 | Complex packing + spatial differencing | GFS 1° global (high compression) |
| 40 | 5.40 | JPEG2000 compression | GFS Wave imagery |
| 41 | 5.41 | PNG compression | Radar imagery |

**Packing Parameters:**
```json
"packing": {
  "reference_value": 270.0,     // R: Reference value
  "binary_scale_factor": 0,      // E: Binary scale factor
  "decimal_scale_factor": 0,    // D: Decimal scale factor
  "bits_per_value": 8,           // N: Bits per packed value
  "original_field_type": 0       // Floating-point type
}
```

**Unpacking Formula:**
```
Y = R + (X × 2^E) / 10^D
```
Where X is the packed integer value.

### 7. Values Format

```json
"values": {
  "Dense": [270.0, 271.0, 272.0, ...]
}
```

- **Dense**: Flat array of grid values in row-major order
- **Missing values**: Represented as `null` in JSON, interpreted as NaN
- **Units**: Kelvin for temperature, m/s for wind, hPa for pressure, etc.

---

## Special Parameters and Characteristics

### T-Number (Spectral Resolution)

The T-number indicates the spectral truncation of the model:

| T-number | N (pole→equator) | Approx Resolution | Grid Size | Use Case |
|----------|------------------|-------------------|-----------|----------|
| T254 | 127 | ~0.5° (50 km) | 512×256 | CORe historical archive |
| T1534 | 768 | ~0.12° (13 km) | 3072×1536 | GDAS operational analysis |
| T6144 | 3072 | ~0.05° (5 km) | 12288×6144 | Future high-res (not in corpus) |

### Gaussian Latitude Distribution

Gaussian latitudes are not evenly spaced - they cluster toward the poles where spectral methods need more resolution:

- **Equator**: Wider spacing (~0.15° for T1534)
- **Mid-latitudes**: Moderate spacing
- **Poles**: Narrower spacing (~0.05° for T1534)

This distribution is computed from zeros of Legendre polynomials for optimal spectral accuracy.

### Scanning Mode

`scanning_mode` is a bit field:
```
Bit 0 (0x01): iScansNegatively - i direction (longitude)
Bit 1 (0x02): jScansPositively - j direction (latitude)
Bit 2 (0x04): jPointsAreConsecutive - Adjacent points in j
Bit 3 (0x08): alternativeRowScanning - Alternate row direction
```

GFS fixtures typically use `0x00`:
- Scans +i direction (eastward)
- Scans -j direction (southward)
- i consecutive

---

## Golden Reference Examples

### Minimal GFS Analysis (gfs_anl_t2m_5x5)

```json
{
  "fixture_id": "gfs_anl_t2m_5x5",
  "_provenance": "Synthetic GFS-like 2m temperature, 5x5 lat/lon grid",
  "fields": [{
    "center": 7,
    "subcenter": 0,
    "parameter": {"discipline": 0, "category": 0, "number": 0},
    "forecast": {
      "reference_time": {"year": 2024, "month": 6, "day": 19, "hour": 0, "minute": 0, "second": 0, "significance": 0},
      "time_range_unit": 1,
      "forecast_offset": 0
    },
    "level": {"type1": 103, "scale_factor1": 0, "scaled_value1": 2, "type2": 255, "scale_factor2": 0, "scaled_value2": 0},
    "ensemble": null,
    "grid": {
      "template": 0, "num_data_points": 25, "nx": 5, "ny": 5,
      "lat_first": 40.0, "lon_first": 0.0, "lat_last": 0.0, "lon_last": 40.0,
      "di": 10.0, "dj": 10.0, "scanning_mode": 0, "resolution_flags": 48, "shape_of_earth": 6
    },
    "values": {"Dense": [270.0, 271.0, 272.0, 273.0, 274.0, 275.0, 276.0, 277.0, 278.0, 279.0, 280.0, 281.0, 282.0, 283.0, 284.0, 285.0, 286.0, 287.0, 288.0, 289.0, 290.0, 291.0, 292.0, 293.0, 294.0]},
    "gdt_template": 0, "pdt_template": 0, "drt_template": 0,
    "packing": {"reference_value": 270.0, "binary_scale_factor": 0, "decimal_scale_factor": 0, "bits_per_value": 8, "original_field_type": 0}
  }]
}
```

### 1-Degree Global Analysis (gfs_tmp2m_1deg_anl)

**Key Characteristics:**
- Grid: 360×181 (65,160 points)
- Resolution: 1.0° uniform
- DRT: 3 (complex packing with spatial differencing)
- Storage: `deferred` (decoder support pending)

### T1534 Gaussian Grid (gfs_gaussian_gdt40_t1534)

**Key Characteristics:**
- Grid: 3072×1536 (4,718,592 points)
- N = 768 (parallels pole→equator)
- Resolution: ~0.117° (~12 km)
- GDT: 40 (Gaussian Lat/Lon)
- Status: ✅ Fully supported by gribtract

---

## Testing and Validation

### Verification Status

| Fixture | Verification Method | Status | Notes |
|--------|-------------------|--------|-------|
| gfs_anl_t2m_5x5 | Differential test suite | ✅ Pass | Minimal synthetic fixture |
| gfs_tmp2m_1deg_anl | DRT=3 pending | ⏳ Deferred | Awaiting DRT=3 decoder |
| gfs_gaussian_gdt40_t1534 | eccodes comparison | ✅ Pass | 54 fields decoded correctly |
| gfswave_arctic_wind_drt40 | eccodes comparison | ✅ Pass | JPEG2000 decoder working |
| core_gaussian_gdt40 | wgrib2 verified | ✅ Pass | T254 Gaussian grid support |

### Running Tests

```bash
# Run differential tests against golden references
cargo test differential

# Test specific fixture
cargo test test_gfs_anl_t2m_5x5

# Test Gaussian grid decoder
cargo test test_gaussian_latlon_params

# Verify against eccodes
cargo xtask corpus diff gfs_gaussian_gdt40_t1534
```

### Golden Reference Generation

```bash
# Generate golden reference from GRIB2 file using eccodes
python3 scripts/gen_golden.py <grib2_file> <fixture_id> --output-dir tests/corpus/golden/

# Example:
python3 scripts/gen_golden.py tests/corpus/small/gfs_anl_t2m_5x5.grib2 gfs_anl_t2m_5x5
```

---

## Source Data and Provenance

### NOAA NOMADS (Operational Data)

**Base URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/`

**Pattern:** `gfs.YYYYMMDD/HH/atmos/gfs.tHz.pgrb2.1p00.f000`

**Example:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2
```

### NOAA CORe Archive (Historical)

**Base URL:** `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/YYYY/MM/`

**Pattern:** `flx.YYYYMMDDHH.grb`

**Example:**
```
https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb
```

### NOAA AWS NODD (Public Access)

**Base URL:** `https://noaa-gfs-bdp-pds.s3.amazonaws.com/`

**Pattern:** `gfs.YYYYMMDD/HH/atmos/gfs.tHz.pgrb2.0p50.f000`

**Example:**
```
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000
```

---

## References and Related Documentation

- **Golden JSON Schema**: `docs/golden-json-schema.md` - Complete schema reference
- **Gaussian Lat/Lon Implementation**: `crates/gribtract-core/src/types.rs:415-419`
- **GDT 3.40 Decoder**: `crates/gribtract-core/src/decode.rs`
- **Fixture Generation**: `scripts/gen_grib2.py`
- **Golden Generation**: `scripts/gen_golden.py`
- **Corpus Management**: `crates/gribtract-testutil/src/corpus.rs`

---

## Summary Table

| Aspect | Value/Range |
|--------|-------------|
| **Center** | 7 (NCEP) |
| **Discipline** | 0 (Meteorological) |
| **GDT Options** | 0 (Regular), 40 (Gaussian) |
| **PDT Options** | 0 (Analysis), 1 (Ensemble), 4.8 (Statistical) |
| **DRT Options** | 0 (Simple), 2 (Complex), 3 (Spatial), 40 (JPEG2000), 41 (PNG) |
| **T-numbers** | T254, T1534 |
| **Resolutions** | 0.5° (T254), 0.12° (T1534) |
| **Grid Sizes** | 131K (T254), 4.7M (T1534) |
| **Storage** | inline (small), remote (large) |
| **Verification** | eccodes, wgrib2, differential test suite |

---

*Document created: 2026-07-25*
*Bead ID: bf-47jm7a*
*Last updated: See git history*
