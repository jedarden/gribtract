//! Golden reference loader.
//!
//! Golden files live at `tests/corpus/golden/<fixture-id>.json` in the workspace
//! root. They are committed JSON representations of the expected decoded output
//! for each fixture, produced by an authoritative reference decoder (eccodes/wgrib2)
//! and checked in for offline comparison.

use serde::{Deserialize, Serialize};
use crate::corpus::corpus_root;

// ── Mirror types for JSON deserialization ─────────────────────────────────────
// These mirror gribtract_core::types but carry serde derives. The comparator
// maps actual decoded values to these to avoid adding serde to gribtract-core.

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenReferenceTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub significance: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenParameterId {
    pub discipline: u8,
    pub category: u8,
    pub number: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenForecastTime {
    pub reference_time: GoldenReferenceTime,
    pub time_range_unit: u8,
    pub forecast_offset: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenLevel {
    pub type1: u8,
    pub scale_factor1: i8,
    pub scaled_value1: i32,
    pub type2: u8,
    pub scale_factor2: i8,
    pub scaled_value2: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenEnsemble {
    pub member_type: u8,
    pub number: i16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenGridDefinition {
    pub template: u16,
    pub num_data_points: u32,
    #[serde(default = "default_nx")]
    pub nx: Option<u32>,
    #[serde(default = "default_ny")]
    pub ny: Option<u32>,
    pub lat_first: f64,
    pub lon_first: f64,
    #[serde(default = "default_zero_f64")]
    pub lat_last: Option<f64>,
    #[serde(default = "default_zero_f64")]
    pub lon_last: Option<f64>,
    #[serde(default = "default_zero_f64")]
    pub di: Option<f64>,
    #[serde(default = "default_zero_f64")]
    pub dj: Option<f64>,
    pub scanning_mode: u8,
    pub resolution_flags: u8,
    pub shape_of_earth: u8,
}

fn default_nx() -> Option<u32> { None }
fn default_ny() -> Option<u32> { None }
fn default_zero_f64() -> Option<f64> { None }

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GoldenPackingInfo {
    pub reference_value: f32,
    pub binary_scale_factor: i16,
    pub decimal_scale_factor: i16,
    pub bits_per_value: u8,
    pub original_field_type: u8,
}

/// Grid values in golden JSON: `{"Dense": [...]}` or `{"Masked": {...}}`.
///
/// The `Dense` variant allows `null` values in the JSON array, which are
/// interpreted as missing/undefined data points and represented as NaN.
#[derive(Debug, Clone, PartialEq)]
pub enum GoldenGridValues {
    Dense(Vec<f64>),
    Masked { values: Vec<f64>, present: Vec<bool> },
}

// Custom deserializer to handle null values in Dense arrays
impl<'de> Deserialize<'de> for GoldenGridValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor, MapAccess};
        use std::fmt;

        struct GoldenGridValuesVisitor;

        impl<'de> Visitor<'de> for GoldenGridValuesVisitor {
            type Value = GoldenGridValues;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with keys 'Dense' or 'Masked'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut dense_value = None;
                let mut masked_values = None;
                let mut masked_present = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "Dense" => {
                            // Deserialize as Vec<Option<f64>> to handle nulls
                            let raw: Vec<Option<f64>> = map.next_value()?;
                            // Convert nulls to NaN
                            dense_value = Some(raw.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect());
                        }
                        "values" => {
                            masked_values = Some(map.next_value()?);
                        }
                        "present" => {
                            masked_present = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                if let Some(dense) = dense_value {
                    Ok(GoldenGridValues::Dense(dense))
                } else if let (Some(values), Some(present)) = (masked_values, masked_present) {
                    Ok(GoldenGridValues::Masked { values, present })
                } else {
                    Err(de::Error::custom("expected 'Dense' key or both 'values' and 'present' keys"))
                }
            }
        }

        deserializer.deserialize_any(GoldenGridValuesVisitor)
    }
}

// Custom serializer to convert NaN values to null in JSON output
impl Serialize for GoldenGridValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        match self {
            GoldenGridValues::Dense(values) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_key("Dense")?;
                // Convert Vec<f64> to JSON array with nulls for NaN
                let json_values: Vec<Option<f64>> = values
                    .iter()
                    .map(|&v| if v.is_nan() { None } else { Some(v) })
                    .collect();
                map.serialize_value(&json_values)?;
                map.end()
            }
            GoldenGridValues::Masked { values, present } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_key("values")?;
                map.serialize_value(values)?;
                map.serialize_key("present")?;
                map.serialize_value(present)?;
                map.end()
            }
        }
    }
}

impl GoldenGridValues {
    pub fn len(&self) -> usize {
        match self {
            GoldenGridValues::Dense(v) => v.len(),
            GoldenGridValues::Masked { values, .. } => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (f64, bool)> + '_ {
        match self {
            GoldenGridValues::Dense(v) => v
                .iter()
                .map(|&val| {
                    // NaN values (from null in JSON) are treated as missing
                    let present = !val.is_nan();
                    (val, present)
                })
                .collect::<Vec<_>>()
                .into_iter(),
            GoldenGridValues::Masked { values, present } => values
                .iter()
                .zip(present.iter())
                .map(|(&val, &p)| (val, p))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

/// A single expected decoded field.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoldenField {
    pub center: u16,
    pub subcenter: u16,
    pub parameter: GoldenParameterId,
    pub forecast: GoldenForecastTime,
    pub level: GoldenLevel,
    pub ensemble: Option<GoldenEnsemble>,
    pub grid: GoldenGridDefinition,
    pub values: GoldenGridValues,
    pub gdt_template: u16,
    pub pdt_template: u16,
    pub drt_template: u16,
    pub packing: GoldenPackingInfo,
}

/// The full golden reference for one corpus fixture.
#[derive(Debug, Deserialize, Serialize)]
pub struct GoldenFixture {
    pub fixture_id: String,
    pub fields: Vec<GoldenField>,
}

/// Load the golden reference for a fixture.
///
/// Returns `Ok(None)` if no golden file exists yet — this is not an error;
/// the fixture will be counted as `no_golden` in the coverage report.
pub fn load_golden(fixture_id: &str) -> Result<Option<GoldenFixture>, String> {
    let path = corpus_root()
        .join("golden")
        .join(format!("{}.json", fixture_id));
    if !path.exists() {
        return Ok(None);
    }
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read golden {}: {}", path.display(), e))?;
    let fixture: GoldenFixture = serde_json::from_str(&json)
        .map_err(|e| format!("cannot parse golden '{}': {}", fixture_id, e))?;
    Ok(Some(fixture))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_gfs_anl_t2m_5x5_loads() {
        let golden = load_golden("gfs_anl_t2m_5x5")
            .expect("load should not error")
            .expect("golden file should exist");
        assert_eq!(golden.fixture_id, "gfs_anl_t2m_5x5");
        assert_eq!(golden.fields.len(), 1);
        let f = &golden.fields[0];
        assert_eq!(f.center, 7);
        assert_eq!(f.parameter.discipline, 0);
        assert_eq!(f.parameter.category, 0);
        assert_eq!(f.parameter.number, 0);
        assert_eq!(f.grid.nx, Some(5));
        assert_eq!(f.grid.ny, Some(5));
        assert_eq!(f.gdt_template, 0);
        assert_eq!(f.pdt_template, 0);
        assert_eq!(f.drt_template, 0);
        assert_eq!(f.values.len(), 25);
        // First value should be R + 0 = 270.0
        assert_eq!(f.values.iter().next().unwrap(), (270.0, true));
    }

    #[test]
    fn golden_missing_returns_none() {
        let result = load_golden("does_not_exist").expect("should not error");
        assert!(result.is_none());
    }
}
