//! Core GRIB2 data model types.
//!
//! These types represent the decoded output of a GRIB2 message, matching what
//! eccodes reports field-by-field. They are the inputs to the differential harness.

// ── Reference time (Section 1) ──────────────────────────────────────────────

/// GRIB2 reference time from Section 1, expressed as calendar components.
///
/// The significance (Table 1.2) records what the reference time *means*:
/// analysis, start of forecast, verifying time, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    /// Table 1.2: 0=analysis, 1=start-of-forecast, 2=verifying, 3=observation, 4=current.
    pub significance: u8,
}

impl ReferenceTime {
    /// Encode as seconds since 1970-01-01T00:00:00Z (UTC, no leap seconds).
    /// Used for quick arithmetic; the components are the canonical form.
    pub fn unix_seconds(&self) -> i64 {
        // Simplified proleptic Gregorian; good enough for GRIB2 forecast cycles.
        let y = self.year as i64;
        let m = self.month as i64;
        let d = self.day as i64;
        // Days since epoch (Gregorian), from Fliegel-Van Flandern.
        let jdn = (1461 * (y + 4800 + (m - 14) / 12)) / 4
            + (367 * (m - 2 - 12 * ((m - 14) / 12))) / 12
            - (3 * ((y + 4900 + (m - 14) / 12) / 100)) / 4
            + d
            - 32075;
        let unix_days = jdn - 2440588; // JDN of 1970-01-01
        unix_days * 86400
            + self.hour as i64 * 3600
            + self.minute as i64 * 60
            + self.second as i64
    }
}

// ── Parameter identity ───────────────────────────────────────────────────────

/// Unique parameter identifier: the triple (discipline, category, number)
/// from Sections 0 and 4. Indexes WMO GRIB2 parameter tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterId {
    /// Section 0, octets 7: discipline (Table 0.0).
    pub discipline: u8,
    /// Section 4, parameter category (Table 4.1.x).
    pub category: u8,
    /// Section 4, parameter number (Table 4.2.x.y).
    pub number: u8,
}

// ── Level / fixed surface (Section 4, PDT 0+) ───────────────────────────────

/// GRIB2 level description — two fixed surfaces (type + scaled value).
///
/// The physical value of surface 1 is `scaled_value1 * 10^(-scale_factor1)`.
/// Surface 2 is only meaningful when type2 != 255 (missing).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Level {
    /// Table 4.5: type of first fixed surface (e.g. 100 = isobaric level).
    pub type1: u8,
    pub scale_factor1: i8,
    pub scaled_value1: i32,
    /// Table 4.5: type of second fixed surface.
    pub type2: u8,
    pub scale_factor2: i8,
    pub scaled_value2: i32,
}

impl Level {
    /// Physical value of first fixed surface.
    pub fn value1(&self) -> f64 {
        self.scaled_value1 as f64 * 10f64.powi(-(self.scale_factor1 as i32))
    }

    /// Physical value of second fixed surface (255 / missing → not applicable).
    pub fn value2(&self) -> f64 {
        self.scaled_value2 as f64 * 10f64.powi(-(self.scale_factor2 as i32))
    }
}

// ── Forecast time (Section 4, PDT 0+) ───────────────────────────────────────

/// Forecast time: reference + offset expressed in GRIB2 units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForecastTime {
    pub reference_time: ReferenceTime,
    /// Table 4.4: time range unit of `forecast_offset` (0=minute, 1=hour, 2=day, …).
    pub time_range_unit: u8,
    /// Forecast offset in units of `time_range_unit`.
    pub forecast_offset: u32,
}

impl ForecastTime {
    /// Forecast offset converted to seconds (best-effort; months/years are approximated).
    pub fn offset_seconds(&self) -> i64 {
        let secs_per_unit: i64 = match self.time_range_unit {
            0 => 60,
            1 => 3_600,
            2 => 86_400,
            3 => 30 * 86_400,
            4 => 365 * 86_400,
            10 => 3 * 3_600,
            11 => 6 * 3_600,
            12 => 12 * 3_600,
            13 => 15 * 60,
            14 => 30 * 60,
            _ => 0,
        };
        self.forecast_offset as i64 * secs_per_unit
    }

    /// Unix timestamp of the valid (verifying) time.
    pub fn valid_unix_seconds(&self) -> i64 {
        self.reference_time.unix_seconds() + self.offset_seconds()
    }
}

// ── Ensemble (Section 4, PDT 1/11/…) ────────────────────────────────────────

/// Ensemble product descriptor (Table 4.6 + member number).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ensemble {
    /// Table 4.6: 0=unperturbed, 1=positively perturbed, 2=negatively perturbed.
    pub member_type: u8,
    pub number: i16,
}

// ── Packing info (Section 5) — drives tolerance derivation ──────────────────

/// Packing metadata extracted from the Data Representation Section header.
///
/// Used by the differential harness to derive the tolerance for grid-value
/// comparison: `tolerance = 0.5 * quantization_step()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PackingInfo {
    /// R in GRIB2 terminology — the reference value (f32, stored as per spec).
    pub reference_value: f32,
    /// E: binary scale factor (packed integers are `(X * 2^E)`).
    pub binary_scale_factor: i16,
    /// D: decimal scale factor (unpacked value = `(R + X*2^E) / 10^D`).
    pub decimal_scale_factor: i16,
    /// Bits per packed value (n).
    pub bits_per_value: u8,
    /// Table 5.1: 0 = floating point field, 1 = integer field.
    pub original_field_type: u8,
}

impl PackingInfo {
    /// Smallest representable step for this packing: `2^E / 10^D`.
    ///
    /// Returns 0.0 for IEEE-float templates where bits_per_value is 32
    /// and both scale factors are 0 — those fields are compared exactly.
    pub fn quantization_step(&self) -> f64 {
        if self.bits_per_value == 0 {
            return 0.0;
        }
        let e = self.binary_scale_factor as i32;
        let d = self.decimal_scale_factor as i32;
        2f64.powi(e) / 10f64.powi(d)
    }

    /// Half-ULP tolerance for differential comparison.
    pub fn tolerance(&self) -> f64 {
        0.5 * self.quantization_step()
    }
}

// ── Grid definition (Section 3) ─────────────────────────────────────────────

/// Grid geometry from the Grid Definition Section.
///
/// Common fields span all templates; template-specific fields (e.g. Lambert
/// projection parameters) are added when template dispatch is implemented.
#[derive(Debug, Clone, PartialEq)]
pub struct GridDefinition {
    /// Grid Definition Template number (Table 3.1): 0=lat/lon, 20=polar stereo, 30=Lambert…
    pub template: u16,
    /// Total number of data points in the grid.
    pub num_data_points: u32,
    /// i-direction (column) count; 0 if not defined by this template.
    pub nx: u32,
    /// j-direction (row) count.
    pub ny: u32,
    /// Latitude of first grid point, in degrees (positive N).
    pub lat_first: f64,
    /// Longitude of first grid point, in degrees (positive E, 0–360).
    pub lon_first: f64,
    /// Latitude of last grid point.
    pub lat_last: f64,
    /// Longitude of last grid point.
    pub lon_last: f64,
    /// i-direction increment in degrees (0 if not uniform).
    pub di: f64,
    /// j-direction increment in degrees (0 if not uniform).
    pub dj: f64,
    /// Scanning mode flags (Table 3.4): bit 7 = +i direction, bit 6 = +j direction, …
    pub scanning_mode: u8,
    /// Resolution and component flags (Table 3.3).
    pub resolution_flags: u8,
    /// Shape of the Earth (Table 3.2): 0=spherical 6367.47km, 6=WGS84, …
    pub shape_of_earth: u8,
}

impl GridDefinition {
    /// True if points are stored with i (longitude) varying fastest (row-major).
    pub fn i_positive(&self) -> bool {
        self.scanning_mode & 0x80 == 0
    }

    /// True if j (latitude) increases from first to last point.
    pub fn j_positive(&self) -> bool {
        self.scanning_mode & 0x40 != 0
    }

    /// True if adjacent rows alternate scan direction (boustrophedon).
    pub fn alternating_rows(&self) -> bool {
        self.scanning_mode & 0x20 != 0
    }
}

// ── Grid values (Section 6 + 7) ─────────────────────────────────────────────

/// Decoded grid: either all present (Dense) or with a missing-value mask.
#[derive(Debug, Clone)]
pub enum GridValues {
    /// All points present; length == grid.num_data_points.
    Dense(Vec<f64>),
    /// `values[i]` is meaningful only when `present[i]` is true.
    Masked { values: Vec<f64>, present: Vec<bool> },
}

impl GridValues {
    pub fn len(&self) -> usize {
        match self {
            GridValues::Dense(v) => v.len(),
            GridValues::Masked { values, .. } => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over (value, is_present) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (f64, bool)> + '_ {
        match self {
            GridValues::Dense(v) => {
                let v2: Vec<bool> = vec![true; v.len()];
                v.iter()
                    .zip(v2.into_iter())
                    .map(|(&val, p)| (val, p))
                    .collect::<Vec<_>>()
                    .into_iter()
            }
            GridValues::Masked { values, present } => values
                .iter()
                .zip(present.iter())
                .map(|(&val, &p)| (val, p))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

// ── Field ────────────────────────────────────────────────────────────────────

/// A single decoded GRIB2 field — the unit of comparison in the differential harness.
#[derive(Debug, Clone)]
pub struct Field {
    // ── Identification (Section 1) ───────────────────────────────────────
    /// Originating center (Table 0): 7=US-NCEP, 98=ECMWF, …
    pub center: u16,
    pub subcenter: u16,

    // ── Parameter ───────────────────────────────────────────────────────
    pub parameter: ParameterId,

    // ── Forecast ────────────────────────────────────────────────────────
    pub forecast: ForecastTime,

    // ── Level ───────────────────────────────────────────────────────────
    pub level: Level,

    // ── Ensemble (absent for deterministic products) ─────────────────────
    pub ensemble: Option<Ensemble>,

    // ── Grid geometry ────────────────────────────────────────────────────
    pub grid: GridDefinition,

    // ── Decoded values ───────────────────────────────────────────────────
    pub values: GridValues,

    // ── Template numbers (for coverage tracking) ─────────────────────────
    /// Grid Definition Template (3.x).
    pub gdt_template: u16,
    /// Product Definition Template (4.x).
    pub pdt_template: u16,
    /// Data Representation Template (5.x).
    pub drt_template: u16,

    // ── Packing metadata (tolerance derivation) ───────────────────────────
    pub packing: PackingInfo,
}

// ── Message ──────────────────────────────────────────────────────────────────

/// A GRIB2 message: zero or more fields sharing a common identification section.
///
/// A single `.grib2` file may contain multiple concatenated messages.
/// Within a message, sections 2–7 may repeat to encode multiple fields.
#[derive(Debug, Clone)]
pub struct Message {
    pub fields: Vec<Field>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_time_unix() {
        let t = ReferenceTime {
            year: 1970, month: 1, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        assert_eq!(t.unix_seconds(), 0);

        let t2 = ReferenceTime {
            year: 1970, month: 1, day: 1,
            hour: 0, minute: 0, second: 60,
            significance: 0,
        };
        assert_eq!(t2.unix_seconds(), 60);

        // 2024-01-01T00:00:00Z
        let t3 = ReferenceTime {
            year: 2024, month: 1, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        // Verified with: date -d "2024-01-01" +%s → 1704067200
        assert_eq!(t3.unix_seconds(), 1_704_067_200);
    }

    #[test]
    fn level_scale_factor() {
        // 500 hPa isobaric level: type=100, scale_factor=0, scaled_value=50000
        // value = 50000 * 10^0 = 50000 Pa
        let level = Level {
            type1: 100, scale_factor1: 0, scaled_value1: 50000,
            type2: 255, scale_factor2: 0, scaled_value2: 0,
        };
        assert!((level.value1() - 50000.0).abs() < 1e-9);

        // 1.5m above ground: scale_factor=1, scaled_value=15 → 15 * 10^-1 = 1.5
        let level2 = Level {
            type1: 103, scale_factor1: 1, scaled_value1: 15,
            type2: 255, scale_factor2: 0, scaled_value2: 0,
        };
        assert!((level2.value1() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn forecast_time_offset_seconds() {
        let ref_time = ReferenceTime {
            year: 2024, month: 6, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        let ft = ForecastTime { reference_time: ref_time, time_range_unit: 1, forecast_offset: 6 };
        assert_eq!(ft.offset_seconds(), 6 * 3600);
        assert_eq!(ft.valid_unix_seconds(), ref_time.unix_seconds() + 6 * 3600);
    }

    #[test]
    fn packing_tolerance() {
        // binary_scale=-1, decimal_scale=0 → step = 2^-1 / 10^0 = 0.5
        // tolerance = 0.25
        let p = PackingInfo {
            reference_value: 0.0,
            binary_scale_factor: -1,
            decimal_scale_factor: 0,
            bits_per_value: 16,
            original_field_type: 0,
        };
        assert!((p.quantization_step() - 0.5).abs() < 1e-12);
        assert!((p.tolerance() - 0.25).abs() < 1e-12);

        // binary_scale=0, decimal_scale=2 → step = 1 / 100 = 0.01
        let p2 = PackingInfo {
            reference_value: 0.0,
            binary_scale_factor: 0,
            decimal_scale_factor: 2,
            bits_per_value: 12,
            original_field_type: 0,
        };
        assert!((p2.quantization_step() - 0.01).abs() < 1e-12);
        assert!((p2.tolerance() - 0.005).abs() < 1e-12);
    }

    #[test]
    fn packing_zero_bits() {
        // Constant field: bits_per_value=0, step=0, tolerance=0
        let p = PackingInfo {
            reference_value: 273.15,
            binary_scale_factor: 0,
            decimal_scale_factor: 0,
            bits_per_value: 0,
            original_field_type: 0,
        };
        assert_eq!(p.quantization_step(), 0.0);
        assert_eq!(p.tolerance(), 0.0);
    }

    #[test]
    fn grid_scanning_flags() {
        // scanning_mode=0b00000000: +i (west→east), −j (north→south), no alternating
        let g = GridDefinition {
            template: 0, num_data_points: 100, nx: 10, ny: 10,
            lat_first: 90.0, lon_first: 0.0, lat_last: -90.0, lon_last: 350.0,
            di: 1.0, dj: 1.0, scanning_mode: 0x00, resolution_flags: 0x30,
            shape_of_earth: 6,
        };
        assert!(g.i_positive());
        assert!(!g.j_positive());
        assert!(!g.alternating_rows());

        // scanning_mode=0b01100000: +i, +j, alternating
        let g2 = GridDefinition { scanning_mode: 0x60, ..g };
        assert!(g2.i_positive());
        assert!(g2.j_positive());
        assert!(g2.alternating_rows());
    }

    #[test]
    fn grid_values_iter() {
        let gv = GridValues::Dense(vec![1.0, 2.0, 3.0]);
        let pairs: Vec<_> = gv.iter().collect();
        assert_eq!(pairs, vec![(1.0, true), (2.0, true), (3.0, true)]);

        let gv2 = GridValues::Masked {
            values: vec![1.0, 0.0, 3.0],
            present: vec![true, false, true],
        };
        let pairs2: Vec<_> = gv2.iter().collect();
        assert_eq!(pairs2, vec![(1.0, true), (0.0, false), (3.0, true)]);
    }
}
