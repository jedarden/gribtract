#![doc = "High-level GRIB2 decoder — message iterator, field selection, public API."]

pub mod provider_probe;
pub use provider_probe::{ProviderProbe, ProviderResult};

pub use gribtract_core::types::{
    BilinearCorners, ComplexExtra, Ensemble, Field, ForecastTime, GaussianLatLonParams,
    GridDefinition, GridValues, LazyField, LambertConformalParams, Level, Message, PackingInfo,
    ParameterId, ReferenceTime,
};
pub use gribtract_core::error::{Error, Result};

/// Decode all fields from raw GRIB2 bytes.
///
/// Returns the decoded fields in message order. A single file may contain
/// multiple concatenated messages; fields from all are returned in one flat vec.
pub fn decode(bytes: &[u8]) -> Result<Vec<Field>> {
    gribtract_core::decode::decode_bytes(bytes)
}

/// Decode fields lazily — Section 7 data stored as raw bytes, not decoded.
///
/// DRT=0 (simple packing) fields without a bitmap: `section7_raw` is
/// populated; use [`decode_point_drt0`] for O(1) single-point extraction.
///
/// DRT=2/3 (complex packing) fields without a bitmap: both `section7_raw`
/// and `complex_extra` are populated; use [`decode_all_drt3`] to decode the
/// full grid once and cache it, then index into the `Vec<f64>` for each
/// station (the "decode-once-extract-many" pattern).
pub fn decode_lazy(bytes: &[u8]) -> Result<Vec<LazyField>> {
    gribtract_core::decode::decode_bytes_lazy(bytes)
}

/// Extract a single grid point from a DRT=0 (simple packing) Section 7 body.
///
/// Returns `None` when `idx` is out of range for the given body length and packing.
/// For constant fields (`bits_per_value == 0`) returns a value for any `idx`.
pub fn decode_point_drt0(
    body: &[u8],
    packing: &PackingInfo,
    idx: usize,
) -> Option<f64> {
    gribtract_core::decode::decode_point_drt0(body, packing, idx)
}

/// Decode the full grid from a DRT=2 or DRT=3 (complex packing) Section 7 body.
///
/// Returns all grid values as a flat `Vec<f64>` in grid-point order.  This is
/// the entry point for the "decode-once-extract-many" pattern when working with
/// DRT=2/3 [`LazyField`] values: decode the grid once, cache the `Vec<f64>`,
/// then use `values[idx]` for each station.
///
/// For DRT=3 (spatial differencing), random access without a full decode is
/// impossible because each value depends on all prior values.  The cached path
/// gives the same throughput as a single full decode divided across N stations —
/// ~N× better than calling the decoder once per station.
///
/// `body` is `LazyField::section7_raw`.
/// `packing` is `LazyField::packing`.
/// `extra` is `LazyField::complex_extra.as_ref()` (must be `Some` for DRT=2/3).
/// `n_points` is `LazyField::grid.num_data_points as usize`.
pub fn decode_all_drt3(
    body: &[u8],
    packing: &PackingInfo,
    extra: &ComplexExtra,
    n_points: usize,
) -> Result<Vec<f64>> {
    gribtract_core::decode::decode_all_drt3(body, packing, extra, n_points)
}

/// Extract a single grid point from a DRT=2 or DRT=3 (complex packing) Section 7 body.
///
/// Decodes the full grid and returns `values[idx]`.
///
/// **Performance note:** For DRT=3 (spatial differencing), each call decodes
/// the entire grid.  When extracting multiple stations from the same field,
/// prefer [`decode_all_drt3`] (decode once, cache `Vec<f64>`, index in O(1))
/// to avoid paying the full-decode cost per station.
///
/// Returns `None` if `idx >= n_points` or if decoding fails.
///
/// `body` is `LazyField::section7_raw`.
/// `packing` is `LazyField::packing`.
/// `extra` is `LazyField::complex_extra.as_ref().unwrap()` (must be `Some` for DRT=2/3).
/// `n_points` is `LazyField::grid.num_data_points as usize`.
pub fn decode_point_drt3(
    body: &[u8],
    packing: &PackingInfo,
    extra: &ComplexExtra,
    n_points: usize,
    idx: usize,
) -> Option<f64> {
    gribtract_core::decode::decode_point_drt3(body, packing, extra, n_points, idx)
}
