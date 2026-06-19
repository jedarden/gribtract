//! Golden reference loader.
//!
//! Golden files live at `tests/corpus/golden/<fixture-id>.json` in the workspace
//! root. They are committed JSON representations of the expected decoded output
//! for each fixture, produced by an authoritative reference decoder (eccodes/wgrib2)
//! and checked in for offline comparison.

use serde::Deserialize;
use crate::corpus::corpus_root;

// ── Mirror types for JSON deserialization ─────────────────────────────────────
// These mirror gribtract_core::types but carry serde derives. The comparator
// maps actual decoded values to these to avoid adding serde to gribtract-core.

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenReferenceTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub significance: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenParameterId {
    pub discipline: u8,
    pub category: u8,
    pub number: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenForecastTime {
    pub reference_time: GoldenReferenceTime,
    pub time_range_unit: u8,
    pub forecast_offset: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenLevel {
    pub type1: u8,
    pub scale_factor1: i8,
    pub scaled_value1: i32,
    pub type2: u8,
    pub scale_factor2: i8,
    pub scaled_value2: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenEnsemble {
    pub member_type: u8,
    pub number: i16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenGridDefinition {
    pub template: u16,
    pub num_data_points: u32,
    pub nx: u32,
    pub ny: u32,
    pub lat_first: f64,
    pub lon_first: f64,
    pub lat_last: f64,
    pub lon_last: f64,
    pub di: f64,
    pub dj: f64,
    pub scanning_mode: u8,
    pub resolution_flags: u8,
    pub shape_of_earth: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GoldenPackingInfo {
    pub reference_value: f32,
    pub binary_scale_factor: i16,
    pub decimal_scale_factor: i16,
    pub bits_per_value: u8,
    pub original_field_type: u8,
}

/// Grid values in golden JSON: `{"Dense": [...]}` or `{"Masked": {...}}`.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum GoldenGridValues {
    Dense(Vec<f64>),
    Masked { values: Vec<f64>, present: Vec<bool> },
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
                .map(|&val| (val, true))
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
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize)]
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
        assert_eq!(f.grid.nx, 5);
        assert_eq!(f.grid.ny, 5);
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
