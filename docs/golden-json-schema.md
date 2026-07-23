# Golden JSON Schema Reference

This document describes the JSON schema used for golden test fixtures in `tests/corpus/golden/`.

## Root Structure

```json
{
  "fixture_id": "string",
  "_provenance": "string",
  "fields": [Field, ...]
}
```

### Root Properties

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fixture_id` | string | Yes | Unique identifier for the test fixture (e.g., `"drt2_simple_3x3"`) |
| `_provenance` | string | Yes | Human-readable description of how this fixture was generated |
| `fields` | array[Field] | Yes | Array of GRIB2 field objects (most fixtures contain exactly 1) |

---

## Field Object

Each field represents a complete GRIB2 message with all its metadata sections.

```json
{
  "center": number,
  "subcenter": number,
  "parameter": {
    "discipline": number,
    "category": number,
    "number": number
  },
  "forecast": {
    "reference_time": ReferenceTime,
    "time_range_unit": number,
    "forecast_offset": number
  },
  "level": {
    "type1": number,
    "scale_factor1": number,
    "scaled_value1": number,
    "type2": number,
    "scale_factor2": number,
    "scaled_value2": number
  },
  "ensemble": Ensemble | null,
  "grid": {
    "template": number,
    "num_data_points": number,
    "nx": number,
    "ny": number,
    "lat_first": number,
    "lon_first": number,
    "lat_last": number,
    "lon_last": number,
    "di": number,
    "dj": number,
    "scanning_mode": number,
    "resolution_flags": number,
    "shape_of_earth": number
  },
  "values": {
    "Dense": [number, ...]
  },
  "gdt_template": number,
  "pdt_template": number,
  "drt_template": number,
  "packing": {
    "reference_value": number,
    "binary_scale_factor": number,
    "decimal_scale_factor": number,
    "bits_per_value": number,
    "original_field_type": number
  }
}
```

### Field Properties

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `center` | integer | Yes | Originating center (e.g., 7 = NCEP) |
| `subcenter` | integer | Yes | Subcenter code |
| `parameter` | Parameter | Yes | Parameter discipline/category/number |
| `forecast` | Forecast | Yes | Forecast time metadata |
| `level` | Level | Yes | Vertical level descriptor |
| `ensemble` | Ensemble\|null | Yes | Ensemble member info, or `null` for deterministic |
| `grid` | Grid | Yes | Grid definition template |
| `values` | Values | Yes | Data values array |
| `gdt_template` | integer | Yes | Grid Definition Template number |
| `pdt_template` | integer | Yes | Product Definition Template number |
| `drt_template` | integer | Yes | Data Representation Template number |
| `packing` | Packing | Yes | Data packing parameters |

---

## Nested Objects

### Parameter

```json
{
  "discipline": number,  // GRIB2 discipline (0=meteorological, etc.)
  "category": number,    // Parameter category
  "number": number       // Parameter number
}
```

**Types:** All integers

### ReferenceTime

```json
{
  "year": number,        // 4-digit year
  "month": number,       // 1-12
  "day": number,         // 1-31
  "hour": number,        // 0-23
  "minute": number,      // 0-59
  "second": number,      // 0-59
  "significance": number // 0=analysis, 1=start of forecast
}
```

**Types:** All integers

### Forecast

```json
{
  "reference_time": ReferenceTime,
  "time_range_unit": number,  // Indicator (1=hour, etc.)
  "forecast_offset": number    // Forecast time offset from reference
}
```

**Types:** All integers

### Level

```json
{
  "type1": number,         // First level type (e.g., 100=isobaric, 103=surface)
  "scale_factor1": number,  // Decimal scale factor
  "scaled_value1": number,  // Scaled level value
  "type2": number,          // Second level type (255 for none)
  "scale_factor2": number,  // Decimal scale factor for level 2
  "scaled_value2": number   // Scaled level value 2
}
```

**Types:** All integers

### Ensemble

```json
{
  "member_type": number,  // 0=unspecified, 1=perturbation, 2=control
  "number": number        // Ensemble member number
}
```

**Types:** All integers. When not applicable, `ensemble` is `null`.

### Grid

```json
{
  "template": number,         // GDT number (0=lat/lon regular grid)
  "num_data_points": number,  // Total grid points
  "nx": number,               // Number of points along X (longitude)
  "ny": number,               // Number of points along Y (latitude)
  "lat_first": number,        // Latitude of first grid point (degrees)
  "lon_first": number,        // Longitude of first grid point (degrees)
  "lat_last": number,         // Latitude of last grid point (degrees)
  "lon_last": number,         // Longitude of last grid point (degrees)
  "di": number,               // Longitudinal direction increment (degrees)
  "dj": number,               // Latitudinal direction increment (degrees)
  "scanning_mode": number,    // Bit flags for scanning direction
  "resolution_flags": number, // Resolution and component flags
  "shape_of_earth": number    // Earth shape code (6=sphere)
}
```

**Types:** All numbers (`nx`, `ny`, `num_data_points` are integers; lat/lon/di/dj are floats)

### Values

```json
{
  "Dense": [number, number, ...]  // Flat array of grid values in row-major order
}
```

**Types:** Array of floats. Currently all fixtures use `"Dense"` format.

### Packing

```json
{
  "reference_value": number,      // Reference value (R)
  "binary_scale_factor": number,  // Binary scale factor (E)
  "decimal_scale_factor": number, // Decimal scale factor (D)
  "bits_per_value": number,       // Number of bits per value (N)
  "original_field_type": number   // Original data type
}
```

**Types:** `bits_per_value` and `original_field_type` are integers; others are floats.

---

## Common Values

### Center Codes
- `7` — NCEP (US National Weather Service)

### Discipline
- `0` — Meteorological

### Parameter Categories
- `0` — Temperature
- `1` — Moisture
- `number` varies by category

### Level Types
- `1` — Surface
- `100` — Isobaric (pressure levels)
- `103` — Surface (altitude above ground)
- `255` — Missing/undefined

### Grid Definition Template (GDT)
- `0` — Latitude/longitude regular grid

### Product Definition Template (PDT)
- `0` — Analysis or forecast at a horizontal level
- `1` — Individual ensemble forecast
- `8` — Time-processed statistical product (e.g., accumulation)

### Data Representation Template (DRT)
- `0` — Grid point data - simple packing
- `2` — Grid point data - complex packing
- `40` — Grid point data - JPEG 2000 encoding
- `41` — Grid point data - PNG encoding

---

## Examples

### Simple Analysis Field

```json
{
  "fixture_id": "gfs_anl_t2m_5x5",
  "_provenance": "Synthetic GFS-like 2m temperature, 5x5 lat/lon grid",
  "fields": [{
    "center": 7,
    "subcenter": 0,
    "parameter": { "discipline": 0, "category": 0, "number": 0 },
    "forecast": {
      "reference_time": { "year": 2024, "month": 6, "day": 19, "hour": 0, "minute": 0, "second": 0, "significance": 0 },
      "time_range_unit": 1,
      "forecast_offset": 0
    },
    "level": { "type1": 103, "scale_factor1": 0, "scaled_value1": 2, "type2": 255, "scale_factor2": 0, "scaled_value2": 0 },
    "ensemble": null,
    "grid": { "template": 0, "num_data_points": 25, "nx": 5, "ny": 5, "lat_first": 40.0, "lon_first": 0.0, "lat_last": 0.0, "lon_last": 40.0, "di": 10.0, "dj": 10.0, "scanning_mode": 0, "resolution_flags": 48, "shape_of_earth": 6 },
    "values": { "Dense": [270.0, 271.0, 272.0, 273.0, 274.0, 275.0, 276.0, 277.0, 278.0, 279.0, 280.0, 281.0, 282.0, 283.0, 284.0, 285.0, 286.0, 287.0, 288.0, 289.0, 290.0, 291.0, 292.0, 293.0, 294.0] },
    "gdt_template": 0,
    "pdt_template": 0,
    "drt_template": 0,
    "packing": { "reference_value": 270.0, "binary_scale_factor": 0, "decimal_scale_factor": 0, "bits_per_value": 8, "original_field_type": 0 }
  }]
}
```

### Ensemble Member Field

```json
{
  "fixture_id": "pdt1_ensemble_3x2",
  "_provenance": "PDT=1 (individual ensemble member), 3x2 grid, temperature 500 hPa, 6h forecast",
  "fields": [{
    "center": 7,
    "subcenter": 0,
    "parameter": { "discipline": 0, "category": 0, "number": 0 },
    "forecast": {
      "reference_time": { "year": 2024, "month": 1, "day": 15, "hour": 0, "minute": 0, "second": 0, "significance": 1 },
      "time_range_unit": 1,
      "forecast_offset": 6
    },
    "level": { "type1": 100, "scale_factor1": 0, "scaled_value1": 50000, "type2": 255, "scale_factor2": 0, "scaled_value2": 0 },
    "ensemble": { "member_type": 2, "number": 3 },
    "grid": { "template": 0, "num_data_points": 6, "nx": 3, "ny": 2, "lat_first": 30.0, "lon_first": 0.0, "lat_last": 20.0, "lon_last": 20.0, "di": 10.0, "dj": 10.0, "scanning_mode": 0, "resolution_flags": 48, "shape_of_earth": 6 },
    "values": { "Dense": [250.0, 251.0, 252.0, 253.0, 254.0, 255.0] },
    "gdt_template": 0,
    "pdt_template": 1,
    "drt_template": 0,
    "packing": { "reference_value": 250.0, "binary_scale_factor": 0, "decimal_scale_factor": 0, "bits_per_value": 8, "original_field_type": 0 }
  }]
}
```

---

## Notes

1. All fixtures currently use GDT template 0 (regular lat/lon grids). Other grid types (rotated, stretched, reduced) are not yet represented in the corpus.

2. The `values` field currently always uses `"Dense"` format. A `"Spatial"` format for spatial differencing may be added in the future.

3. Level values use GRIB2's scaled representation: actual value = `scaled_value * 10^(-scale_factor)`. For example, isobaric level 500 hPa is represented as `scale_factor1: 0, scaled_value1: 50000`.

4. The `significance` field in `reference_time` indicates whether the time is an analysis (0) or the start of a forecast (1).
