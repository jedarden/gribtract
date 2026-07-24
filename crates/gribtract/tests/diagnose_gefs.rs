//! Diagnostic test for GEFS ensemble fixtures

use gribtract_testutil::corpus;
use gribtract_testutil::diff::{compare_field, FieldResult};
use gribtract_testutil::golden;

#[test]
fn diagnose_gefs_member01_pdt41() {
    let entry = corpus::fixture_entry("gefs_member01_pdt41").expect("fixture exists");

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
                        for mm in mismatches.iter().take(5) {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                        if i == 0 {
                            println!("  Actual: gdt={}, pdt={}, drt={}", actual.gdt_template, actual.pdt_template, actual.drt_template);
                            println!("  Golden: gdt={}, pdt={}, drt={}", golden.gdt_template, golden.pdt_template, golden.drt_template);
                        }
                    }
                    FieldResult::ValuesMismatch(points) => {
                        println!("Field {}: VALUES_MISMATCH ({} points exceed tolerance)", i, points.len());
                        for pm in points.iter().take(2) {
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

                // Stop after 5 fields
                if i >= 5 {
                    break;
                }
            }
        }
    }
}

#[test]
fn diagnose_gefs_ensemble_mean_pdt48() {
    let entry = corpus::fixture_entry("gefs_ensemble_mean_pdt48").expect("fixture exists");

    let golden_fixture = golden::load_golden(&entry.id)
        .expect("golden exists")
        .expect("golden loaded");

    let bytes = corpus::load(&entry.id).expect("fixture loaded");

    println!("Testing GEFS ensemble mean (PDT 4.8) fixture...");
    println!("Total golden fields: {}", golden_fixture.fields.len());

    match gribtract::decode(&bytes) {
        Err(e) => {
            println!("Decode error: {}", e);
            println!("This is expected - PDT 4.8 decode is not yet implemented");
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
                        for mm in mismatches.iter().take(3) {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                        if i == 0 {
                            println!("  Actual: gdt={}, pdt={}, drt={}", actual.gdt_template, actual.pdt_template, actual.drt_template);
                            println!("  Golden: gdt={}, pdt={}, drt={}", golden.gdt_template, golden.pdt_template, golden.drt_template);
                        }
                    }
                    FieldResult::ValuesMismatch(points) => {
                        println!("Field {}: VALUES_MISMATCH ({} points exceed tolerance)", i, points.len());
                    }
                    FieldResult::LengthMismatch { expected, actual } => {
                        println!("Field {}: LENGTH_MISMATCH (expected={}, actual={})", i, expected, actual);
                    }
                    FieldResult::MaskMismatch { index } => {
                        println!("Field {}: MASK_MISMATCH at index {}", i, index);
                    }
                }

                if i >= 3 {
                    break;
                }
            }
        }
    }
}
