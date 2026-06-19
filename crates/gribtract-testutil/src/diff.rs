//! Differential comparator: compares decoded `Field`s against golden references.
//!
//! Metadata fields are compared exactly; grid values are compared within the
//! per-field tolerance derived from the packing header (half-ULP of the
//! quantization step — see `docs/notes/oracle-and-tolerance.md`).

use std::collections::HashMap;
use gribtract_core::types::Field;
use crate::golden::GoldenField;

// ── Single-field comparison result ───────────────────────────────────────────

/// A metadata field that did not match.
#[derive(Debug, Clone)]
pub struct MetaMismatch {
    pub field: &'static str,
    pub expected: String,
    pub actual: String,
}

/// A grid point that exceeded tolerance.
#[derive(Debug, Clone)]
pub struct PointMismatch {
    pub index: usize,
    pub expected: f64,
    pub actual: f64,
    pub delta: f64,
    pub tolerance: f64,
}

/// Outcome of comparing one decoded `Field` to one `GoldenField`.
#[derive(Debug)]
pub enum FieldResult {
    Match,
    MetaMismatch(Vec<MetaMismatch>),
    MaskMismatch { index: usize },
    ValuesMismatch(Vec<PointMismatch>),
    LengthMismatch { expected: usize, actual: usize },
}

impl FieldResult {
    pub fn is_match(&self) -> bool {
        matches!(self, FieldResult::Match)
    }
}

// ── Coverage report ───────────────────────────────────────────────────────────

/// Per-(GDT,PDT,DRT) triple statistics.
#[derive(Debug, Default, Clone)]
pub struct TemplateStat {
    pub attempts: u32,
    pub matched: u32,
}

/// Aggregated outcome over all fixtures and fields in the corpus.
#[derive(Debug, Default)]
pub struct CoverageReport {
    pub fixtures_total: usize,
    pub fixtures_matched: usize,
    pub fixtures_decode_error: usize,
    pub fixtures_no_golden: usize,
    /// (gdt_template, pdt_template, drt_template) → stat
    pub by_template: HashMap<(u16, u16, u16), TemplateStat>,
}

impl CoverageReport {
    pub fn agreement_pct(&self) -> f64 {
        let denom = self.fixtures_total.saturating_sub(self.fixtures_no_golden);
        if denom == 0 {
            return 0.0;
        }
        100.0 * self.fixtures_matched as f64 / denom as f64
    }

    pub fn print_report(&self) {
        let comparable = self.fixtures_total.saturating_sub(self.fixtures_no_golden);
        println!("=== Differential Harness Coverage ===");
        println!(
            "Fixtures : {} total  ({} comparable, {} no-golden)",
            self.fixtures_total,
            comparable,
            self.fixtures_no_golden
        );
        println!("  matched      : {}", self.fixtures_matched);
        println!("  decode errors: {}", self.fixtures_decode_error);
        println!(
            "Agreement: {}/{} ({:.1}%)",
            self.fixtures_matched,
            comparable,
            self.agreement_pct()
        );
        if !self.by_template.is_empty() {
            println!("Per-template:");
            let mut templates: Vec<_> = self.by_template.iter().collect();
            templates.sort_by_key(|(k, _)| *k);
            for ((gdt, pdt, drt), stat) in templates {
                println!(
                    "  GDT={gdt} PDT={pdt} DRT={drt}: {}/{}",
                    stat.matched, stat.attempts
                );
            }
        }
        println!("=====================================");
    }
}

// ── Comparison logic ──────────────────────────────────────────────────────────

/// Compare one decoded field against its golden counterpart.
pub fn compare_field(actual: &Field, golden: &GoldenField) -> FieldResult {
    let mut meta_mismatches: Vec<MetaMismatch> = Vec::new();

    macro_rules! check_meta {
        ($field:literal, $actual:expr, $expected:expr) => {
            if $actual != $expected {
                meta_mismatches.push(MetaMismatch {
                    field: $field,
                    expected: format!("{:?}", $expected),
                    actual: format!("{:?}", $actual),
                });
            }
        };
    }

    check_meta!("center", actual.center, golden.center);
    check_meta!("subcenter", actual.subcenter, golden.subcenter);
    check_meta!("parameter.discipline", actual.parameter.discipline, golden.parameter.discipline);
    check_meta!("parameter.category", actual.parameter.category, golden.parameter.category);
    check_meta!("parameter.number", actual.parameter.number, golden.parameter.number);

    // Forecast reference time
    let rt = &actual.forecast.reference_time;
    let grt = &golden.forecast.reference_time;
    check_meta!("forecast.reference_time.year", rt.year, grt.year);
    check_meta!("forecast.reference_time.month", rt.month, grt.month);
    check_meta!("forecast.reference_time.day", rt.day, grt.day);
    check_meta!("forecast.reference_time.hour", rt.hour, grt.hour);
    check_meta!("forecast.reference_time.minute", rt.minute, grt.minute);
    check_meta!("forecast.reference_time.second", rt.second, grt.second);
    check_meta!("forecast.reference_time.significance", rt.significance, grt.significance);
    check_meta!("forecast.time_range_unit", actual.forecast.time_range_unit, golden.forecast.time_range_unit);
    check_meta!("forecast.forecast_offset", actual.forecast.forecast_offset, golden.forecast.forecast_offset);

    // Level
    check_meta!("level.type1", actual.level.type1, golden.level.type1);
    check_meta!("level.scale_factor1", actual.level.scale_factor1, golden.level.scale_factor1);
    check_meta!("level.scaled_value1", actual.level.scaled_value1, golden.level.scaled_value1);
    check_meta!("level.type2", actual.level.type2, golden.level.type2);
    check_meta!("level.scale_factor2", actual.level.scale_factor2, golden.level.scale_factor2);
    check_meta!("level.scaled_value2", actual.level.scaled_value2, golden.level.scaled_value2);

    // Ensemble
    match (&actual.ensemble, &golden.ensemble) {
        (None, None) => {}
        (Some(ae), Some(ge)) => {
            check_meta!("ensemble.member_type", ae.member_type, ge.member_type);
            check_meta!("ensemble.number", ae.number, ge.number);
        }
        (None, Some(_)) => meta_mismatches.push(MetaMismatch {
            field: "ensemble",
            expected: "Some(...)".to_string(),
            actual: "None".to_string(),
        }),
        (Some(_), None) => meta_mismatches.push(MetaMismatch {
            field: "ensemble",
            expected: "None".to_string(),
            actual: "Some(...)".to_string(),
        }),
    }

    // Grid definition
    let ag = &actual.grid;
    let gg = &golden.grid;
    check_meta!("grid.template", ag.template, gg.template);
    check_meta!("grid.num_data_points", ag.num_data_points, gg.num_data_points);
    check_meta!("grid.nx", ag.nx, gg.nx);
    check_meta!("grid.ny", ag.ny, gg.ny);
    check_meta!("grid.lat_first", ag.lat_first.to_bits(), gg.lat_first.to_bits());
    check_meta!("grid.lon_first", ag.lon_first.to_bits(), gg.lon_first.to_bits());
    check_meta!("grid.lat_last", ag.lat_last.to_bits(), gg.lat_last.to_bits());
    check_meta!("grid.lon_last", ag.lon_last.to_bits(), gg.lon_last.to_bits());
    check_meta!("grid.di", ag.di.to_bits(), gg.di.to_bits());
    check_meta!("grid.dj", ag.dj.to_bits(), gg.dj.to_bits());
    check_meta!("grid.scanning_mode", ag.scanning_mode, gg.scanning_mode);
    check_meta!("grid.resolution_flags", ag.resolution_flags, gg.resolution_flags);
    check_meta!("grid.shape_of_earth", ag.shape_of_earth, gg.shape_of_earth);

    // Template identifiers
    check_meta!("gdt_template", actual.gdt_template, golden.gdt_template);
    check_meta!("pdt_template", actual.pdt_template, golden.pdt_template);
    check_meta!("drt_template", actual.drt_template, golden.drt_template);

    // Packing metadata
    check_meta!("packing.reference_value", actual.packing.reference_value.to_bits(), golden.packing.reference_value.to_bits());
    check_meta!("packing.binary_scale_factor", actual.packing.binary_scale_factor, golden.packing.binary_scale_factor);
    check_meta!("packing.decimal_scale_factor", actual.packing.decimal_scale_factor, golden.packing.decimal_scale_factor);
    check_meta!("packing.bits_per_value", actual.packing.bits_per_value, golden.packing.bits_per_value);
    check_meta!("packing.original_field_type", actual.packing.original_field_type, golden.packing.original_field_type);

    if !meta_mismatches.is_empty() {
        return FieldResult::MetaMismatch(meta_mismatches);
    }

    // Grid values
    let tolerance = actual.packing.tolerance();
    let actual_pts: Vec<(f64, bool)> = actual.values.iter().collect();
    let golden_pts: Vec<(f64, bool)> = golden.values.iter().collect();

    if actual_pts.len() != golden_pts.len() {
        return FieldResult::LengthMismatch {
            expected: golden_pts.len(),
            actual: actual_pts.len(),
        };
    }

    // Mask must match exactly; only present values are numerically compared.
    for (i, (a, g)) in actual_pts.iter().zip(golden_pts.iter()).enumerate() {
        if a.1 != g.1 {
            return FieldResult::MaskMismatch { index: i };
        }
    }

    let mut value_mismatches: Vec<PointMismatch> = Vec::new();
    for (i, (a, g)) in actual_pts.iter().zip(golden_pts.iter()).enumerate() {
        if !a.1 {
            continue;
        }
        let delta = (a.0 - g.0).abs();
        if delta > tolerance {
            value_mismatches.push(PointMismatch {
                index: i,
                expected: g.0,
                actual: a.0,
                delta,
                tolerance,
            });
        }
    }

    if !value_mismatches.is_empty() {
        FieldResult::ValuesMismatch(value_mismatches)
    } else {
        FieldResult::Match
    }
}

/// Compare all decoded fields from one fixture against its golden reference.
///
/// Returns `true` if every field matches (count and content).
pub fn compare_fixture(
    actual_fields: &[Field],
    golden_fields: &[GoldenField],
    report: &mut CoverageReport,
) -> bool {
    if actual_fields.len() != golden_fields.len() {
        return false;
    }

    let mut all_match = true;
    for (actual, golden) in actual_fields.iter().zip(golden_fields.iter()) {
        let key = (actual.gdt_template, actual.pdt_template, actual.drt_template);
        let stat = report.by_template.entry(key).or_default();
        stat.attempts += 1;

        let result = compare_field(actual, golden);
        if result.is_match() {
            stat.matched += 1;
        } else {
            all_match = false;
        }
    }
    all_match
}

#[cfg(test)]
mod tests {
    use super::*;
    use gribtract_core::types::{
        Field, ForecastTime, GridDefinition, GridValues, Level, PackingInfo,
        ParameterId, ReferenceTime,
    };
    use crate::golden::{
        GoldenField, GoldenForecastTime, GoldenGridDefinition, GoldenGridValues,
        GoldenLevel, GoldenPackingInfo, GoldenParameterId, GoldenReferenceTime,
    };

    fn sample_field() -> Field {
        Field {
            center: 7,
            subcenter: 0,
            parameter: ParameterId { discipline: 0, category: 0, number: 0 },
            forecast: ForecastTime {
                reference_time: ReferenceTime {
                    year: 2024, month: 6, day: 19,
                    hour: 0, minute: 0, second: 0,
                    significance: 0,
                },
                time_range_unit: 1,
                forecast_offset: 0,
            },
            level: Level {
                type1: 103, scale_factor1: 0, scaled_value1: 2,
                type2: 255, scale_factor2: 0, scaled_value2: 0,
            },
            ensemble: None,
            grid: GridDefinition {
                template: 0, num_data_points: 4, nx: 2, ny: 2,
                lat_first: 10.0, lon_first: 0.0, lat_last: 0.0, lon_last: 10.0,
                di: 10.0, dj: 10.0, scanning_mode: 0, resolution_flags: 48,
                shape_of_earth: 6,
            },
            values: GridValues::Dense(vec![270.0, 271.0, 272.0, 273.0]),
            gdt_template: 0,
            pdt_template: 0,
            drt_template: 0,
            packing: PackingInfo {
                reference_value: 270.0,
                binary_scale_factor: 0,
                decimal_scale_factor: 0,
                bits_per_value: 8,
                original_field_type: 0,
            },
        }
    }

    fn sample_golden(values: Vec<f64>) -> GoldenField {
        GoldenField {
            center: 7,
            subcenter: 0,
            parameter: GoldenParameterId { discipline: 0, category: 0, number: 0 },
            forecast: GoldenForecastTime {
                reference_time: GoldenReferenceTime {
                    year: 2024, month: 6, day: 19,
                    hour: 0, minute: 0, second: 0,
                    significance: 0,
                },
                time_range_unit: 1,
                forecast_offset: 0,
            },
            level: GoldenLevel {
                type1: 103, scale_factor1: 0, scaled_value1: 2,
                type2: 255, scale_factor2: 0, scaled_value2: 0,
            },
            ensemble: None,
            grid: GoldenGridDefinition {
                template: 0, num_data_points: 4, nx: 2, ny: 2,
                lat_first: 10.0, lon_first: 0.0, lat_last: 0.0, lon_last: 10.0,
                di: 10.0, dj: 10.0, scanning_mode: 0, resolution_flags: 48,
                shape_of_earth: 6,
            },
            values: GoldenGridValues::Dense(values),
            gdt_template: 0,
            pdt_template: 0,
            drt_template: 0,
            packing: GoldenPackingInfo {
                reference_value: 270.0,
                binary_scale_factor: 0,
                decimal_scale_factor: 0,
                bits_per_value: 8,
                original_field_type: 0,
            },
        }
    }

    #[test]
    fn exact_match_returns_match() {
        let actual = sample_field();
        let golden = sample_golden(vec![270.0, 271.0, 272.0, 273.0]);
        assert!(compare_field(&actual, &golden).is_match());
    }

    #[test]
    fn within_tolerance_returns_match() {
        // tolerance = 0.5 * 2^0 / 10^0 = 0.5; delta of 0.3 should pass
        let actual = sample_field();
        let golden = sample_golden(vec![270.3, 271.3, 272.3, 273.3]);
        assert!(compare_field(&actual, &golden).is_match());
    }

    #[test]
    fn exceeds_tolerance_returns_mismatch() {
        // delta of 0.6 > tolerance 0.5
        let actual = sample_field();
        let golden = sample_golden(vec![270.0, 271.0, 272.0, 274.0]);
        let result = compare_field(&actual, &golden);
        // grid values match for indices 0-2; index 3: |273-274|=1 > 0.5
        assert!(matches!(result, FieldResult::ValuesMismatch(_)));
    }

    #[test]
    fn meta_mismatch_detected() {
        let mut actual = sample_field();
        actual.center = 98; // wrong center
        let golden = sample_golden(vec![270.0, 271.0, 272.0, 273.0]);
        assert!(matches!(compare_field(&actual, &golden), FieldResult::MetaMismatch(_)));
    }

    #[test]
    fn coverage_report_agreement_pct() {
        let mut report = CoverageReport::default();
        report.fixtures_total = 4;
        report.fixtures_matched = 2;
        report.fixtures_no_golden = 1;
        // comparable = 4 - 1 = 3; matched = 2
        assert!((report.agreement_pct() - 66.666).abs() < 0.01);
    }

    #[test]
    fn zero_agreement_pct_when_no_comparable() {
        let report = CoverageReport::default();
        assert_eq!(report.agreement_pct(), 0.0);
    }
}
