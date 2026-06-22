#![doc = "High-level GRIB2 decoder — message iterator, field selection, public API."]

pub use gribtract_core::types::{
    BilinearCorners, Ensemble, Field, ForecastTime, GaussianLatLonParams, GridDefinition,
    GridValues, LazyField, Level, Message, PackingInfo, ParameterId, ReferenceTime,
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
/// Only DRT=0 (simple packing) fields without a bitmap populate
/// `LazyField::section7_raw`. Use [`decode_point_drt0`] to extract individual
/// grid points on demand without decoding the full grid.
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
