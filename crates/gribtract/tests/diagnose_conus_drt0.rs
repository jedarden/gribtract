//! Diagnostic test for conus_drt0 fixture

use gribtract_testutil::corpus;
use gribtract_testutil::diff::{compare_field, FieldResult};
use gribtract_testutil::golden;

#[test]
fn diagnose_conus_drt0() {
    let entry = corpus::fixture_entry("conus_drt0").expect("fixture exists");

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
                        for mm in mismatches.iter() {
                            println!("  - {}: expected={}, actual={}", mm.field, mm.expected, mm.actual);
                        }
                        if i == 0 {
                            // Show full metadata for first field
                            println!("  Actual field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", actual.gdt_template, actual.pdt_template, actual.drt_template);
                            println!("    grid.template={}, grid.nx={}, grid.ny={}", actual.grid.template, actual.grid.nx, actual.grid.ny);
                            println!("    level.type1={}, level.scale_factor1={}, level.scaled_value1={}",
                                actual.level.type1, actual.level.scale_factor1, actual.level.scaled_value1);
                            println!("    level.type2={}, level.scale_factor2={}, level.scaled_value2={}",
                                actual.level.type2, actual.level.scale_factor2, actual.level.scaled_value2);
                            println!("    Golden field metadata:");
                            println!("    gdt={}, pdt={}, drt={}", golden.gdt_template, golden.pdt_template, golden.drt_template);
                            println!("    grid.template={}, grid.nx={:?}, grid.ny={:?}", golden.grid.template, golden.grid.nx, golden.grid.ny);
                            println!("    level: {:?}", golden.level);
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

                // Only show first field for conus_drt0
                if i >= 0 {
                    break;
                }
            }
        }
    }
}
