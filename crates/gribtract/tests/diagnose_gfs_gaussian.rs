//! Detailed mismatch diagnostic test for GFS Gaussian-grid fixture

use gribtract_testutil::corpus;
use gribtract_testutil::diff::{compare_field, FieldResult};
use gribtract_testutil::golden;

#[test]
fn diagnose_core_gaussian_gdt40() {
    let entry = corpus::fixture_entry("core_gaussian_gdt40").expect("fixture exists");

    let golden_fixture = golden::load_golden(&entry.id)
        .expect("golden exists")
        .expect("golden loaded");

    let bytes = corpus::load(&entry.id).expect("fixture loaded");

    match gribtract::decode(&bytes) {
        Err(e) => {
            panic!("Decode error: {}", e);
        }
        Ok(actual_fields) => {
            println!("=== CORe GFS Gaussian-grid GDT40 Differential Analysis ===");
            println!("Total fields: actual={}, golden={}", actual_fields.len(), golden_fixture.fields.len());
            println!();

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
                        for pm in points.iter().take(5) {
                            println!("  - index {}: expected={}, actual={}, delta={}, tolerance={}",
                                pm.index, pm.expected, pm.actual, pm.delta, pm.tolerance);
                        }
                        if i == 0 && !points.is_empty() {
                            // Show statistics for first field with value mismatches
                            let max_delta = points.iter().map(|p| p.delta).fold(0.0f64, f64::max);
                            let avg_delta: f64 = points.iter().map(|p| p.delta).sum::<f64>() / points.len() as f64;
                            println!("  Statistics for field 0:");
                            println!("    Total mismatches: {}", points.len());
                            println!("    Max delta: {:.6}", max_delta);
                            println!("    Avg delta: {:.6}", avg_delta);
                        }
                    }
                    FieldResult::LengthMismatch { expected, actual } => {
                        println!("Field {}: LENGTH_MISMATCH (expected={}, actual={})", i, expected, actual);
                    }
                    FieldResult::MaskMismatch { index } => {
                        println!("Field {}: MASK_MISMATCH at index {}", i, index);
                    }
                }

                // Show all fields for CORe Gaussian-grid (usually small number of fields)
                if i >= 10 {
                    break;
                }
            }

            println!();
            println!("=== Analysis Complete ===");
        }
    }
}
