use std::fs;
use gribtract::decode;

fn main() {
    println!("=== TESTING MINIMAL BUFFER UNDERRUN CASES ===\n");

    // Test the 2-byte padding case
    println!("Test 1: minimal_underrun_2bytes.grib2");
    let fixture_path = "tests/corpus/small/minimal_underrun_2bytes.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");
    println!("File size: {} bytes", bytes.len());

    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields (unexpected!)", fields.len());
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
            println!("  This is the expected buffer underrun!");
        }
    }
    println!("");

    // Test the original case
    println!("Test 2: rotated_latlon_gdt1_drt0.grib2 (original)");
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");
    println!("File size: {} bytes", bytes.len());

    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields (unexpected!)", fields.len());
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
            println!("  Original failure mode");
        }
    }
    println!("");

    // Test the clean exit case
    println!("Test 3: minimal_underrun.grib2 (should exit cleanly)");
    let fixture_path = "tests/corpus/small/minimal_underrun.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");
    println!("File size: {} bytes", bytes.len());

    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields", fields.len());
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
            println!("  (This file might fail for other reasons - minimal GRIB2 structure)");
        }
    }
}
