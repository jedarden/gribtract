//! Test to regenerate golden references for fixtures that need updating
//! This is only meant to be run manually when golden references need to be updated

use gribtract_testutil::corpus::{corpus_root, load};
use gribtract_testutil::golden::*;
use std::fs;

fn convert_field_to_golden(field: &gribtract_core::types::Field) -> GoldenField {
    let values = {
        let mut vals = Vec::new();
        for (val, present) in field.values.iter() {
            if present {
                vals.push(val);
            } else {
                vals.push(f64::NAN);
            }
        }
        GoldenGridValues::Dense(vals)
    };

    let grid = GoldenGridDefinition {
        template: field.grid.template,
        num_data_points: field.grid.num_data_points,
        nx: Some(field.grid.nx),
        ny: Some(field.grid.ny),
        lat_first: field.grid.lat_first,
        lon_first: field.grid.lon_first,
        lat_last: Some(field.grid.lat_last),
        lon_last: Some(field.grid.lon_last),
        di: Some(field.grid.di),
        dj: Some(field.grid.dj),
        scanning_mode: field.grid.scanning_mode,
        resolution_flags: field.grid.resolution_flags,
        shape_of_earth: field.grid.shape_of_earth,
    };

    let reference_time = GoldenReferenceTime {
        year: field.forecast.reference_time.year,
        month: field.forecast.reference_time.month,
        day: field.forecast.reference_time.day,
        hour: field.forecast.reference_time.hour,
        minute: field.forecast.reference_time.minute,
        second: field.forecast.reference_time.second,
        significance: field.forecast.reference_time.significance,
    };

    let forecast = GoldenForecastTime {
        reference_time,
        time_range_unit: field.forecast.time_range_unit,
        forecast_offset: field.forecast.forecast_offset,
    };

    let level = GoldenLevel {
        type1: field.level.type1,
        scale_factor1: Some(field.level.scale_factor1),
        scaled_value1: Some(field.level.scaled_value1),
        type2: field.level.type2,
        scale_factor2: Some(field.level.scale_factor2),
        scaled_value2: Some(field.level.scaled_value2),
    };

    let ensemble = field.ensemble.as_ref().map(|e| GoldenEnsemble {
        member_type: e.member_type,
        number: e.number,
    });

    let parameter = GoldenParameterId {
        discipline: field.parameter.discipline,
        category: field.parameter.category,
        number: field.parameter.number,
    };

    let packing = GoldenPackingInfo {
        reference_value: field.packing.reference_value,
        binary_scale_factor: field.packing.binary_scale_factor,
        decimal_scale_factor: field.packing.decimal_scale_factor,
        bits_per_value: field.packing.bits_per_value,
        original_field_type: field.packing.original_field_type,
    };

    GoldenField {
        center: field.center,
        subcenter: field.subcenter,
        parameter,
        forecast,
        level,
        ensemble,
        grid,
        values,
        gdt_template: field.gdt_template,
        pdt_template: field.pdt_template,
        drt_template: field.drt_template,
        packing,
    }
}

fn generate_golden(fixture_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = load(fixture_id)?;
    let fields = gribtract::decode(&bytes)?;
    let golden_fields: Vec<GoldenField> = fields.iter().map(convert_field_to_golden).collect();

    let golden_fixture = serde_json::json!({
        "fixture_id": fixture_id,
        "fields": golden_fields
    });

    let golden_path = corpus_root()
        .join("golden")
        .join(format!("{}.json", fixture_id));

    let json_string = serde_json::to_string_pretty(&golden_fixture)?;
    fs::write(&golden_path, json_string)?;

    println!("Generated golden reference for {} with {} fields", fixture_id, fields.len());
    println!("Written to: {}", golden_path.display());

    Ok(())
}

#[test]
#[ignore] // Manual use only
fn regenerate_nam_awip12_lambert_drt3() {
    generate_golden("nam_awip12_lambert_drt3").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_mrms_carib_refl_drt41() {
    generate_golden("mrms_carib_refl_drt41").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_gfs_gaussian_gdt40_t1534() {
    generate_golden("gfs_gaussian_gdt40_t1534").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_conus_drt0() {
    generate_golden("conus_drt0").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_pdt1_ensemble_3x2() {
    generate_golden("pdt1_ensemble_3x2").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_gefs_ensemble_mean_pdt48() {
    generate_golden("gefs_ensemble_mean_pdt48").expect("should generate golden");
}

#[test]
#[ignore] // Manual use only
fn regenerate_gefs_member01_pdt41() {
    generate_golden("gefs_member01_pdt41").expect("should generate golden");
}
