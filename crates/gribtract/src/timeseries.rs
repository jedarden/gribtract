//! Forecast-timeseries emitter.
//!
//! Extracts a per-station time series (one value per forecast hour) from a
//! set of decoded GRIB2 [`Field`]s.  The common hot path in forecast
//! post-processing: given a 48-hour GFS cycle decoded message-by-message, pull
//! out temperature at 2m above ground at 20 METAR city coordinates.
//!
//! # Design
//!
//! - Inputs are already-decoded [`Field`] slices; this layer is pure extraction.
//! - Parameter + level matching is exact (no fuzzy matching).
//! - Station lookup uses the [`GridDefinition::nearest_index`] fast path — O(1)
//!   for lat/lon grids, O(1) for projected grids.
//! - Output is JSON-serialisable via `serde`.
//! - The emitter is "correct by construction": every extracted value is verified
//!   against the field's presence mask so masked/missing points are `None`.
//!
//! # Example
//!
//! ```rust
//! use gribtract::{Field};
//! use gribtract::timeseries::{ParameterRecord, Station, TimeseriesRequest, extract_timeseries};
//!
//! // Assume `fields` is a Vec<Field> decoded from one or more forecast GRIB2 files.
//! // For this doc-test we just show the shape; actual data comes from `gribtract::decode`.
//! let fields: Vec<Field> = Vec::new();
//!
//! let request = TimeseriesRequest {
//!     parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
//!     level_type1: 103,   // height above ground
//!     stations: vec![
//!         Station { id: "JFK".into(), lat: 40.64, lon: -73.78 },
//!         Station { id: "LAX".into(), lat: 33.94, lon: -118.41 },
//!     ],
//! };
//!
//! let ts = extract_timeseries(&fields, &request);
//! assert_eq!(ts.stations.len(), 2);
//! ```

use serde::{Deserialize, Serialize};

use crate::{Field, Level, ParameterId};

// ── Serialisable parameter record ─────────────────────────────────────────────

/// Serialisable mirror of [`ParameterId`] (core type lacks serde).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParameterRecord {
    pub discipline: u8,
    pub category: u8,
    pub number: u8,
}

impl From<ParameterId> for ParameterRecord {
    fn from(p: ParameterId) -> Self {
        Self { discipline: p.discipline, category: p.category, number: p.number }
    }
}

impl From<ParameterRecord> for ParameterId {
    fn from(p: ParameterRecord) -> Self {
        Self { discipline: p.discipline, category: p.category, number: p.number }
    }
}

// ── Station ───────────────────────────────────────────────────────────────────

/// A named point at a lat/lon coordinate.
///
/// `lon` is in degrees East (positive = East, negative = West).  The
/// extractor normalises to [0, 360) internally before grid lookup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Station {
    /// Short identifier (ICAO code, station ID, …).
    pub id: String,
    /// Latitude in degrees North (positive = North).
    pub lat: f64,
    /// Longitude in degrees East (negative = West).
    pub lon: f64,
}

// ── TimeseriesRequest ─────────────────────────────────────────────────────────

/// Parameters that select which fields contribute to the timeseries.
///
/// Fields are included when:
/// - `field.parameter == request.parameter`
/// - `field.level.type1 == request.level_type1`
///
/// Level value filtering is intentionally omitted so that callers can
/// easily request "all 2m-temperature fields" without knowing the
/// scaled_value encoding in advance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeseriesRequest {
    /// Parameter triple (discipline, category, number) to match.
    pub parameter: ParameterRecord,
    /// Level type (Table 4.5) to match; e.g. 103 = height above ground.
    pub level_type1: u8,
    /// Stations to extract at (nearest grid-point, all supported projections).
    pub stations: Vec<Station>,
}

// ── Output types ──────────────────────────────────────────────────────────────

/// One row in the timeseries: all station values at a single valid time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeseriesRow {
    /// Unix timestamp (seconds since 1970-01-01T00:00:00Z) of the valid time.
    pub valid_unix: i64,
    /// Forecast offset from the reference time, in seconds.
    pub forecast_offset_s: i64,
    /// Extracted values — one per station in [`ForecastTimeseries::stations`] order.
    ///
    /// `None` when the station is outside the grid extent or the nearest
    /// grid point is masked (bitmap = 0).
    pub values: Vec<Option<f64>>,
    /// Level of the matched field (useful when `level_type1` covers multiple values).
    pub level: LevelRecord,
}

/// Compact level record for serialisation (avoids pulling in the full `Level` type).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelRecord {
    pub type1: u8,
    pub value1: f64,
    pub type2: u8,
    pub value2: f64,
}

impl From<&Level> for LevelRecord {
    fn from(l: &Level) -> Self {
        Self {
            type1: l.type1,
            value1: l.value1(),
            type2: l.type2,
            value2: l.value2(),
        }
    }
}

/// Complete timeseries output: parameter, stations, and rows ordered by valid time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastTimeseries {
    /// Unix timestamp of the cycle reference time (Section 1).
    ///
    /// All forecast offsets are relative to this.  `None` when no matching
    /// fields were found (in which case `rows` is also empty).
    pub reference_time_unix: Option<i64>,
    /// Parameter that was extracted.
    pub parameter: ParameterRecord,
    /// Stations in the same order as `values` in each row.
    pub stations: Vec<Station>,
    /// Rows sorted ascending by `valid_unix`.
    pub rows: Vec<TimeseriesRow>,
}

// ── Extractor ─────────────────────────────────────────────────────────────────

/// Extract a forecast timeseries from a slice of decoded GRIB2 fields.
///
/// Fields that match `request.parameter` and `request.level_type1` contribute
/// one row each.  Rows are sorted by `valid_unix` ascending before returning.
///
/// Fields with unrecognised grids (where `grid.nearest_index` returns `None`
/// for all stations) produce a row with all `None` values rather than being
/// silently dropped — the caller can detect and handle sparse data.
///
/// This function is infallible by design: decode errors happen upstream in
/// [`crate::decode`]; here we only do arithmetic.
pub fn extract_timeseries(fields: &[Field], request: &TimeseriesRequest) -> ForecastTimeseries {
    let n_stations = request.stations.len();
    let mut rows: Vec<TimeseriesRow> = Vec::new();
    let mut reference_time_unix: Option<i64> = None;
    let param_id: ParameterId = request.parameter.into();

    for field in fields {
        // Parameter match
        if field.parameter != param_id {
            continue;
        }
        // Level type match
        if field.level.type1 != request.level_type1 {
            continue;
        }

        let valid_unix = field.forecast.valid_unix_seconds();
        let forecast_offset_s = field.forecast.offset_seconds();
        let ref_unix = field.forecast.reference_time.unix_seconds();

        // Record the reference time from the first matching field.
        if reference_time_unix.is_none() {
            reference_time_unix = Some(ref_unix);
        }

        // Extract one value per station.
        let mut values: Vec<Option<f64>> = Vec::with_capacity(n_stations);
        for station in &request.stations {
            // Normalise longitude to [0, 360) for grid lookup.
            let lon = if station.lon < 0.0 { station.lon + 360.0 } else { station.lon };
            let idx_opt = field.grid.nearest_index(station.lat, lon);
            let value = idx_opt.and_then(|idx| field.values.get_at(idx));
            values.push(value);
        }

        rows.push(TimeseriesRow {
            valid_unix,
            forecast_offset_s,
            values,
            level: LevelRecord::from(&field.level),
        });
    }

    // Sort ascending by valid time.
    rows.sort_by_key(|r| r.valid_unix);

    ForecastTimeseries {
        reference_time_unix,
        parameter: request.parameter,
        stations: request.stations.clone(),
        rows,
    }
}

// ── JSON serialisation helper ─────────────────────────────────────────────────

/// Emit `ForecastTimeseries` as a compact JSON string.
///
/// Thin wrapper around `serde_json::to_string`; provided as a convenience
/// so callers don't need to import `serde_json` directly.
pub fn to_json(ts: &ForecastTimeseries) -> serde_json::Result<String> {
    serde_json::to_string(ts)
}

/// Emit `ForecastTimeseries` as a pretty-printed JSON string.
pub fn to_json_pretty(ts: &ForecastTimeseries) -> serde_json::Result<String> {
    serde_json::to_string_pretty(ts)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Field, ForecastTime, GridDefinition, GridProjection, GridValues, Level, PackingInfo,
        ParameterId, ReferenceTime,
    };

    #[allow(clippy::too_many_arguments)]
    fn make_field(
        discipline: u8,
        category: u8,
        number: u8,
        level_type1: u8,
        forecast_offset: u32,
        nx: u32,
        ny: u32,
        lat_first: f64,
        lon_first: f64,
        di: f64,
        dj: f64,
        values: Vec<f64>,
    ) -> Field {
        // For a j_positive scan (bit 6 set = 0x40), lat increases from first to last.
        let lat_last = lat_first + dj * (ny as f64 - 1.0);
        let lon_last = lon_first + di * (nx as f64 - 1.0);
        Field {
            center: 7,
            subcenter: 0,
            parameter: ParameterId { discipline, category, number },
            forecast: ForecastTime {
                reference_time: ReferenceTime {
                    year: 2026, month: 6, day: 21, hour: 0, minute: 0, second: 0,
                    significance: 1,
                },
                time_range_unit: 1,
                forecast_offset,
            },
            level: Level {
                type1: level_type1, scale_factor1: 0, scaled_value1: 2,
                type2: 255, scale_factor2: 0, scaled_value2: 0,
            },
            ensemble: None,
            grid: GridDefinition {
                template: 0,
                num_data_points: (nx * ny),
                nx,
                ny,
                lat_first,
                lon_first,
                lat_last,
                lon_last,
                di,
                dj,
                scanning_mode: 0x40, // +i, +j
                resolution_flags: 48,
                shape_of_earth: 6,
                projection: GridProjection::LatLon,
            },
            values: GridValues::Dense(values),
            gdt_template: 0,
            pdt_template: 0,
            drt_template: 3,
            packing: PackingInfo {
                reference_value: 0.0,
                binary_scale_factor: 0,
                decimal_scale_factor: 0,
                bits_per_value: 8,
                original_field_type: 0,
            },
        }
    }

    #[test]
    fn empty_fields_returns_empty_timeseries() {
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![Station { id: "NYC".into(), lat: 40.78, lon: -73.97 }],
        };
        let ts = extract_timeseries(&[], &request);
        assert!(ts.rows.is_empty());
        assert!(ts.reference_time_unix.is_none());
        assert_eq!(ts.stations.len(), 1);
    }

    #[test]
    fn non_matching_parameter_skipped() {
        // Field has discipline=0, category=1, number=0 but request asks for 0,0,0.
        let field = make_field(0, 1, 0, 103, 0, 3, 3, 30.0, 270.0, 1.0, 1.0,
                               vec![1.0; 9]);
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![Station { id: "A".into(), lat: 31.0, lon: -89.0 }],
        };
        let ts = extract_timeseries(&[field], &request);
        assert!(ts.rows.is_empty());
    }

    #[test]
    fn single_field_single_station() {
        // 3×3 lat/lon grid: lat 30–32N, lon 270–272E (= 90–88W).
        // Grid values = row-major (j increases, lat increases with j_positive).
        // Row j=0: lat 30N, lon 270-272E → values 0,1,2
        // Row j=1: lat 31N, lon 270-272E → values 3,4,5
        // Row j=2: lat 32N, lon 270-272E → values 6,7,8
        // Station at (31.1N, -88.9E=271.1E) → nearest is j=1,i=1 → idx=4 → value=4.0
        let vals = (0..9).map(|x| x as f64).collect::<Vec<_>>();
        let field = make_field(0, 0, 0, 103, 0, 3, 3, 30.0, 270.0, 1.0, 1.0, vals);

        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![Station { id: "S1".into(), lat: 31.1, lon: -88.9 }],
        };
        let ts = extract_timeseries(&[field], &request);
        assert_eq!(ts.rows.len(), 1);
        assert_eq!(ts.rows[0].values.len(), 1);
        // -88.9 + 360 = 271.1E → nearest lon = 271E (i=1); lat 31.1 → j=1.
        // idx = 1*3+1 = 4 → value=4.0
        assert_eq!(ts.rows[0].values[0], Some(4.0));
    }

    #[test]
    fn multiple_forecast_hours_sorted_by_valid_time() {
        let vals = vec![100.0; 4]; // 2×2 grid, constant value
        let f0 = make_field(0, 0, 0, 103, 0, 2, 2, 10.0, 10.0, 10.0, 10.0, vals.clone());
        let f6 = make_field(0, 0, 0, 103, 6, 2, 2, 10.0, 10.0, 10.0, 10.0, vals.clone());
        let f12 = make_field(0, 0, 0, 103, 12, 2, 2, 10.0, 10.0, 10.0, 10.0, vals.clone());

        let station = Station { id: "X".into(), lat: 10.0, lon: 10.0 };
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![station],
        };

        // Pass fields in reverse order: f12, f0, f6
        let ts = extract_timeseries(&[f12.clone(), f0.clone(), f6.clone()], &request);
        assert_eq!(ts.rows.len(), 3);
        // Rows must be sorted by valid_unix (f0 < f6 < f12).
        assert!(ts.rows[0].valid_unix <= ts.rows[1].valid_unix);
        assert!(ts.rows[1].valid_unix <= ts.rows[2].valid_unix);
        // f0 has forecast_offset=0 → offset_seconds=0
        assert_eq!(ts.rows[0].forecast_offset_s, 0);
        assert_eq!(ts.rows[1].forecast_offset_s, 6 * 3600);
        assert_eq!(ts.rows[2].forecast_offset_s, 12 * 3600);
    }

    #[test]
    fn station_outside_grid_returns_none() {
        // Grid covers lat 30–32, lon 270–272. Station at lat 50N is outside.
        let vals = vec![99.0; 9];
        let field = make_field(0, 0, 0, 103, 0, 3, 3, 30.0, 270.0, 1.0, 1.0, vals);
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![Station { id: "OUT".into(), lat: 50.0, lon: -90.0 }],
        };
        let ts = extract_timeseries(&[field], &request);
        assert_eq!(ts.rows.len(), 1);
        assert_eq!(ts.rows[0].values[0], None);
    }

    #[test]
    fn json_roundtrip() {
        let vals = vec![270.0, 271.0, 272.0, 273.0];
        let field = make_field(0, 0, 0, 103, 0, 2, 2, 10.0, 350.0, 10.0, 10.0, vals);
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations: vec![Station { id: "P1".into(), lat: 10.0, lon: -10.0 }],
        };
        let ts = extract_timeseries(&[field], &request);
        let json = to_json(&ts).expect("serialisation must succeed");
        let back: ForecastTimeseries = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back.stations.len(), ts.stations.len());
        assert_eq!(back.rows.len(), ts.rows.len());
        assert_eq!(back.rows[0].values, ts.rows[0].values);
    }

    #[test]
    fn multiple_stations_correct_ordering() {
        // 5×5 grid, lat 0–4, lon 0–4. Each cell value = j*5+i.
        let vals: Vec<f64> = (0..25).map(|x| x as f64).collect();
        let field = make_field(0, 0, 0, 103, 0, 5, 5, 0.0, 0.0, 1.0, 1.0, vals);

        let stations = vec![
            Station { id: "A".into(), lat: 0.0, lon: 0.0 }, // idx=0, val=0
            Station { id: "B".into(), lat: 2.0, lon: 3.0 }, // idx=2*5+3=13, val=13
            Station { id: "C".into(), lat: 4.0, lon: 4.0 }, // idx=4*5+4=24, val=24
        ];
        let request = TimeseriesRequest {
            parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
            level_type1: 103,
            stations,
        };
        let ts = extract_timeseries(&[field], &request);
        assert_eq!(ts.rows.len(), 1);
        let row = &ts.rows[0];
        assert_eq!(row.values[0], Some(0.0));
        assert_eq!(row.values[1], Some(13.0));
        assert_eq!(row.values[2], Some(24.0));
    }
}
