#![doc = "High-level GRIB2 decoder — message iterator, field selection, public API."]

pub use gribtract_core::types::{
    Ensemble, Field, ForecastTime, GridDefinition, GridValues, Level, Message, PackingInfo,
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
