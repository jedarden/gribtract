//! Detailed mismatch diagnostic test for specific failing fixtures

use gribtract_testutil::corpus;
use gribtract_testutil::diff::{compare_field, FieldResult, MetaMismatch, PointMismatch};
use gribtract_testutil::golden;

#[test]
fn diagnose_nam_awip12_lambert_drt3() {
    let entry = corpus::fixture_entry("nam_awip12_lambert_drt3").expect("fixture exists");

    let golden_fixture = golden::load_golden(&entry.id)
        .expect("golden exists")
        .expect("golden loaded");

    let bytes = corpus::load(&entry.id).expect("fixture loaded");

    match gribtract::decode(&bytes) {
        Err(e) => {
            panic!("Decode error: {}", e);
        }
        Ok(actual_fields) => {
            println!("Total fields: actual={}, golden={}", actual_fields.len(), golden_fixture.fields.len());

            for (i, (actual, golden)) in actual_fields.iter().zip(golden_fixture.fields.iter()).enumerate() {
                let result = compare_field(actual, golden);
                match result {
                    FieldResult::Match => {
                        if i < 3 {
                            println!("Field {}: MATCH", i);
                        }
                    }
                    FieldResult::MetaMismatch(mismatches) => {
                        println!("Field {}: META_MISMATCH ({} differences)", i, mismatches.len());
                        for mm in mismatches.iter().take(10) {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                        if i == 0 {
                            // Show full metadata for first field
                            println!("  Actual field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", actual.gdt_template, actual.pdt_template, actual.drt_template);
                            println!("    grid.template={}, grid.nx={}, grid.ny={}", actual.grid.template, actual.grid.nx, actual.grid.ny);
                            println!("    lat_first={}, lon_first={}", actual.grid.lat_first, actual.grid.lon_first);
                            println!("    di={}, dj={}", actual.grid.di, actual.grid.dj);
                            println!("    Golden field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", golden.gdt_template, golden.pdt_template, golden.drt_template);
                            println!("    grid.template={}, grid.nx={:?}, grid.ny={:?}", golden.grid.template, golden.grid.nx, golden.grid.ny);
                        }
                    }
                    FieldResult::ValuesMismatch(points) => {
                        println!("Field {}: VALUES_MISMATCH ({} points exceed tolerance)", i, points.len());
                        for pm in points.iter().take(3) {
                            println!("  - index {}: expected={}, actual={}, delta={}, tolerance={}",
                                pm.index, pm.expected, pm.actual, pm.delta, pm.tolerance);
                        }
                    }
                    FieldResult::LengthMismatch { expected, actual } => {
                        println!("Field {}: LENGTH_MISMATCH (expected={}, actual={})", i, expected, actual);
                    }
                    FieldResult::MaskMismatch { index } => {
                        println!("Field {}: MASK_MISMATCH at index {}", i, index);
                    }
                }

                // Stop after 5 fields to keep output manageable
                if i >= 5 {
                    break;
                }
            }
        }
    }
}

#[test]
fn diagnose_mrms_carib_refl_drt41() {
    let entry = corpus::fixture_entry("mrms_carib_refl_drt41").expect("fixture exists");

    let golden_fixture = golden::load_golden(&entry.id)
        .expect("golden exists")
        .expect("golden loaded");

    let bytes = corpus::load(&entry.id).expect("fixture loaded");

    match gribtract::decode(&bytes) {
        Err(e) => {
            panic!("Decode error: {}", e);
        }
        Ok(actual_fields) => {
            println!("Total fields: actual={}, golden={}", actual_fields.len(), golden_fixture.fields.len());

            for (i, (actual, golden)) in actual_fields.iter().zip(golden_fixture.fields.iter()).enumerate() {
                let result = compare_field(actual, golden);
                match result {
                    FieldResult::Match => {
                        println!("Field {}: MATCH", i);
                    }
                    FieldResult::MetaMismatch(mismatches) => {
                        println!("Field {}: META_MISMATCH ({} differences)", i, mismatches.len());
                        for mm in mismatches.iter().take(10) {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                    }
                    FieldResult::ValuesMismatch(points) => {
                        println!("Field {}: VALUES_MISMATCH ({} points exceed tolerance)", i, points.len());
                        for pm in points.iter().take(5) {
                            println!("  - index {}: expected={}, actual={}, delta={}, tolerance={}",
                                pm.index, pm.expected, pm.actual, pm.delta, pm.tolerance);
                        }
                    }
                    FieldResult::LengthMismatch { expected, actual } => {
                        println!("Field {}: LENGTH_MISMATCH (expected={}, actual={})", i, expected, actual);
                    }
                    FieldResult::MaskMismatch { index } => {
                        println!("Field {}: MASK_MISMATCH at index {}", i, index);
                    }
                }

                // Only show first field for MRMS (it's usually single-field)
                if i >= 0 {
                    break;
                }
            }
        }
    }
}

#[test]
fn diagnose_gfswave_arctic_wind_drt40() {
    let entry = corpus::fixture_entry("gfswave_arctic_wind_drt40").expect("fixture exists");

    let golden_fixture = golden::load_golden(&entry.id)
        .expect("golden exists")
        .expect("golden loaded");

    let bytes = corpus::load(&entry.id).expect("fixture loaded");

    match gribtract::decode(&bytes) {
        Err(e) => {
            panic!("Decode error: {}", e);
        }
        Ok(actual_fields) => {
            println!("Total fields: actual={}, golden={}", actual_fields.len(), golden_fixture.fields.len());

            for (i, (actual, golden)) in actual_fields.iter().zip(golden_fixture.fields.iter()).enumerate() {
                let result = compare_field(actual, golden);
                match result {
                    FieldResult::Match => {
                        println!("Field {}: MATCH", i);
                    }
                    FieldResult::MetaMismatch(mismatches) => {
                        println!("Field {}: META_MISMATCH ({} differences)", i, mismatches.len());
                        for mm in mismatches.iter().take(10) {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                        if i == 0 {
                            // Show full metadata for first field
                            println!("  Actual field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", actual.gdt_template, actual.pdt_template, actual.drt_template);
                            println!("    grid.template={}, grid.nx={}, grid.ny={}", actual.grid.template, actual.grid.nx, actual.grid.ny);
                            println!("    lat_first={}, lon_first={}", actual.grid.lat_first, actual.grid.lon_first);
                            println!("    di={}, dj={}", actual.grid.di, actual.grid.dj);
                            println!("    Golden field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", golden.gdt_template, golden.pdt_template, golden.drt_template);
                            println!("    grid.template={}, grid.nx={:?}, grid.ny={:?}", golden.grid.template, golden.grid.nx, golden.grid.ny);
                        }
                    }
                    FieldResult::ValuesMismatch(points) => {
                        println!("Field {}: VALUES_MISMATCH ({} points exceed tolerance)", i, points.len());
                        for pm in points.iter().take(5) {
                            println!("  - index {}: expected={}, actual={}, delta={}, tolerance={}",
                                pm.index, pm.expected, pm.actual, pm.delta, pm.tolerance);
                        }
                    }
                    FieldResult::LengthMismatch { expected, actual } => {
                        println!("Field {}: LENGTH_MISMATCH (expected={}, actual={})", i, expected, actual);
                    }
                    FieldResult::MaskMismatch { index } => {
                        println!("Field {}: MASK_MISMATCH at index {}", i, index);
                    }
                }

                // Only show first field for gfswave (it's usually single-field)
                if i >= 0 {
                    break;
                }
            }
        }
    }
}
