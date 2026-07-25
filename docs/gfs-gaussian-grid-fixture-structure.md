# GFS Gaussian-Grid Fixture Structure

This document describes the structure and components of GFS (Global Forecast System) Gaussian-grid fixtures used in the gribtract test corpus.

## Overview

Gaussian grids are specialized latitude/longitude grids used in numerical weather prediction that provide **uniform longitude spacing** but **non-uniform latitude spacing**. The latitudes are the zeros of the associated Legendre polynomial P_N(sin φ) between the equator and the pole.

### Key Characteristics

- **Grid Definition Template (GDT)**: 3.40 (Gaussian Latitude/Longitude)
- **Product Definition Template (PDT)**: Typically 4.0 (analysis/forecast) or 4.1 (ensemble)
- **Data Representation Template (DRT)**: Variable (0=simple packing, 3=complex packing, 40=JPEG2000, etc.)
- **N parameter**: Number of parallels from pole to equator (determines grid resolution)

## Fixture Data Structure

### Golden JSON Schema

The golden reference files (`tests/corpus/golden/*.json`) follow this structure:

```json
{
  "fixture_id": "string",
  "_provenance": "string",
  "fields": [
    {
      "center": 7,
      "subcenter": 0,
      "parameter": { "discipline": 0, "category": 0, "number": 0 },
      "forecast": {
        "reference_time": {
          "year": 2024, "month": 6, "day": 19,
          "hour": 0, "minute": 0, "second": 0,
          "significance": 0
        },
        "time_range_unit": 1,
        "forecast_offset": 0
      },
      "level": {
        "type1": 103, "scale_factor1": 0, "scaled_value1": 2,
        "type2": 255, "scale_factor2": 0, "scaled_value2": 0
      },
      "ensemble": null | { "member_type": number, "number": number },
      "grid": {
        "template": 40,
        "num_data_points": 131072,
        "nx": 512,
        "ny": 256,
        "lat_first": 89.4629,
        "lon_first": 0,
        "lat_last": -89.4629,
        "lon_last": 359.297,
        "di": 0.703125,
        "dj": null,
        "scanning_mode": 0,
        "resolution_flags": 48,
        "shape_of_earth": 6
      },
      "values": {
        "Dense": [170.819, 170.827, ...]
      },
      "gdt_template": 40,
      "pdt_template": 0,
      "drt_template": 0,
      "packing": {
        "reference_value": 270.0,
        "binary_scale_factor": 0,
        "decimal_scale_factor": 0,
        "bits_per_value": 8,
        "original_field_type": 0
      }
    }
  ]
}
```

## Key Components

### 1. Grid Section (GDT 3.40)

The grid section for Gaussian grids contains:

| Field | Type | Description |
|-------|------|-------------|
| `template` | number | Grid definition template number (40 for Gaussian) |
| `num_data_points` | number | Total grid points (nx × ny) |
| `nx` | number | Number of longitude points |
| `ny` | number | Number of latitude points |
| `lat_first` | number | Latitude of first grid point (degrees N) |
| `lon_first` | number | Longitude of first grid point (degrees E, 0-360) |
| `lat_last` | number | Latitude of last grid point (degrees N) |
| `lon_last` | number | Longitude of last grid point (degrees E) |
| `di` | number | Longitude increment (degrees, uniform) |
| `dj` | number/null | Latitude increment (null for Gaussian - non-uniform) |
| `scanning_mode` | number | Bit flags for scanning direction |
| `resolution_flags` | number | Resolution and component flags |
| `shape_of_earth` | number | Earth shape model (6=WGS84) |

**Special Note for Gaussian Grids:**
- `di` (longitude increment) is uniform
- `dj` (latitude increment) is **null** or **0** because latitude spacing is non-uniform
- True Gaussian latitudes are computed as zeros of Legendre polynomial P_N

### 2. Parameter Section

Describes the meteorological parameter:

| Field | Type | Description |
|-------|------|-------------|
| `discipline` | number | GRIB discipline (0=meteorological) |
| `category` | number | Parameter category (0=temp, 1=moisture, etc.) |
| `number` | number | Parameter number within category |

### 3. Forecast Section

Temporal information:

| Field | Type | Description |
|-------|------|-------------|
| `reference_time` | object | Analysis/reference time |
| `time_range_unit` | number | Unit for forecast time (1=hours) |
| `forecast_offset` | number | Forecast lead time (0=analysis) |

### 4. Level Section

Vertical level information:

| Field | Type | Description |
|-------|------|-------------|
| `type1` | number | First level type (103=height above ground) |
| `scale_factor1` | number | Scale factor for first level |
| `scaled_value1` | number | Scaled value for first level |
| `type2` | number | Second level type (255=not used) |
| `scale_factor2` | number | Scale factor for second level |
| `scaled_value2` | number | Scaled value for second level |

### 5. Ensemble Section

Present only for ensemble forecasts (PDT 4.1):

| Field | Type | Description |
|-------|------|-------------|
| `member_type` | number | Ensemble member type |
| `number` | number | Perturbation number |

### 6. Values Section

Data values:

| Field | Type | Description |
|-------|------|-------------|
| `Dense` | array | Array of data values (length = num_data_points) |

### 7. Packing Section

Data compression information:

| Field | Type | Description |
|-------|------|-------------|
| `reference_value` | number | Reference value (R) for packing |
| `binary_scale_factor` | number | Binary scale factor (E) |
| `decimal_scale_factor` | number | Decimal scale factor (D) |
| `bits_per_value` | number | Number of bits per packed value |
| `original_field_type` | number | Type of original field values |

## Available Gaussian-Grid Fixtures

### core_gaussian_gdt40

- **ID**: `core_gaussian_gdt40`
- **Source**: NOAA CORe Archive (Climate Data Record)
- **Grid**: 512×256 (131,072 points)
- **Resolution**: ~0.7° (N=128)
- **File**: `flx.2024011500.grib2` (10.5 MB, remote storage)
- **Characteristics**: GDT 3.40, 54 GRIB2 messages, surface flux fields
- **Status**: ✅ Fully supported

### gfs_gaussian_gdt40_t1534

- **ID**: `gfs_gaussian_gdt40_t1534`
- **Source**: NOAA GDAS Surface Flux
- **Grid**: 3072×1536 (4,718,592 points)
- **Resolution**: T1534 (N=768, ~0.117° or ~12km)
- **File**: `gdas.t00z.sfluxgrbf000.grib2` (122 MB, remote storage)
- **Characteristics**: High-resolution Gaussian grid, 54 GRIB2 messages
- **Status**: ✅ Fully supported

## Gaussian Grid Parameters

### N Parameter (Number of Parallels)

The **N** parameter defines the Gaussian grid resolution:

- **N = number of parallels from pole to equator**
- Total latitude circles = 2N (or 2N+1 if poles included)
- Higher N = higher resolution

Common N values:
- N=128: T256 Gaussian grid (~0.7° resolution)
- N=768: T1534 Gaussian grid (~0.12° resolution)

### Latitude Distribution

Gaussian latitudes are **not uniformly spaced**. They are computed as the zeros of the Legendre polynomial P_N(sin φ):

```
P_N(sin φ_k) = 0  for k = 1, 2, ..., N
```

This provides optimal quadrature for spectral models.

## Template Identifiers

| Template | Name | Usage |
|----------|------|-------|
| GDT 3.40 | Gaussian Lat/Lon | Grid definition |
| PDT 4.0 | Analysis/Forecast | Standard product definition |
| PDT 4.1 | Ensemble Member | Individual ensemble forecasts |
| PDT 4.8 | Ensemble Mean | Statistical ensemble products |
| DRT 5.0 | Simple Packing | Basic data compression |
| DRT 5.3 | Complex Packing | With spatial differencing |
| DRT 5.40 | JPEG2000 | Lossy wavelet compression |

## Resolution and Component Flags

The `resolution_flags` field (Table 3.3) contains:

| Bits | Meaning |
|------|---------|
| 0-3 | Resolution and component flags |
| 4-5 | Direction increments given |
| 6-7 | Earth model |

Common value: `48` (0b00110000)
- Bits 4-5 = 11: Both di and dj given (or dj=0 for Gaussian)

## Scanning Mode

The `scanning_mode` field (Table 3.4) defines data ordering:

| Bit | Meaning |
|-----|---------|
| 7 (0x80) | i-direction: 0=positive, 1=negative |
| 6 (0x40) | j-direction: 0=negative, 1=positive |
| 5 (0x20) | Adjacent rows scan same direction (0) or alternate (1) |
| 4 (0x10) | All rows same direction (0) or not (1) |

Common value: `0` (0x00)
- i-positive, j-negative (rows scan from north to south)

## Shape of Earth

| Value | Model |
|-------|-------|
| 0 | Spherical (radius 6367470 m) |
| 1 | Spherical (radius 6371229 m) |
| 6 | WGS84 (oblate spheroid) |

Common value: `6` (WGS84)

## Usage Examples

### Loading a Gaussian-Grid Fixture

```rust
use gribtract_testutil::corpus;

let entry = corpus::fixture_entry("core_gaussian_gdt40")
    .expect("fixture exists");

let bytes = corpus::load(&entry.id)
    .expect("fixture loaded");

let fields = gribtract::decode(&bytes)
    .expect("decode successful");
```

### Checking Grid Type

```rust
let grid = &fields[0].grid;

if grid.template == 40 {
    println!("Gaussian grid detected");
    println!("Grid points: {}", grid.num_data_points);
    println!("Resolution: {}×{}", grid.nx, grid.ny);
}
```

### Gaussian-Specific Parameters

For Gaussian grids (GDT 3.40), the grid projection contains:

```rust
use gribtract_core::types::GridProjection;

if let GridProjection::GaussianLatLon(params) = &grid.projection {
    println!("N (parallels pole-to-equator): {}", params.n_parallels);
    println!("Total latitude circles: {}", 2 * params.n_parallels);
}
```

## Testing and Validation

Gaussian-grid fixtures are validated through:

1. **Structure validation**: GRIB2 message parsing and template identification
2. **Golden comparison**: Differential testing against eccodes reference output
3. **Grid correctness**: Nearest-point queries and index calculations
4. **Data accuracy**: Value comparisons within tolerance

See `tests/diagnose_gfs_gaussian.rs` for detailed diagnostic testing.

## References

- WMO GRIB2 Code Tables: https://www.nco.ncep.noaa.gov/pmb/codes/grib2/
- NOAA NCEP Grid Documentation: https://www.nco.ncep.noaa.gov/pmb/docs/gfs/
- Gaussian Quadrature: Numerical analysis textbooks
- gribtract source: `crates/gribtract-core/src/types.rs` (GaussianLatLonParams)

## Notes

- Gaussian grids are primarily used in global spectral models (GFS, ECMWF, etc.)
- The non-uniform latitude spacing provides better numerical stability for spectral transforms
- gribtract approximates Gaussian latitudes as uniform for nearest-point queries (valid at corners)
- True Gaussian quadrature placement requires computing Legendre polynomial zeros (future optimization)
