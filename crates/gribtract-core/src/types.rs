//! Core GRIB2 data model types shared across crates.

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterId {
    pub discipline: u8,
    pub category: u8,
    pub number: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    pub type1: u8,
    pub value1: f64,
    pub type2: u8,
    pub value2: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForecastTime {
    pub reference_time: i64,
    pub forecast_offset_seconds: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ensemble {
    pub member_type: u8,
    pub number: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridDefinition {
    pub template: u16,
    pub nx: u32,
    pub ny: u32,
    pub lat_first: f64,
    pub lon_first: f64,
    pub lat_last: f64,
    pub lon_last: f64,
    pub scanning_mode: u8,
}

#[derive(Debug, Clone)]
pub enum GridValues {
    Dense(Vec<f64>),
    Masked { values: Vec<f64>, present: Vec<bool> },
}

#[derive(Debug, Clone)]
pub struct Field {
    pub parameter: ParameterId,
    pub level: Level,
    pub forecast: ForecastTime,
    pub ensemble: Option<Ensemble>,
    pub grid: GridDefinition,
    pub values: GridValues,
    pub data_representation_template: u16,
}
