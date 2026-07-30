use gribtract::decode;
use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("=== MINIMAL REPRODUCTION TEST ===");
    println!("File size: {} bytes", bytes.len());
    println!();

    // Try to decode - this should trigger the buffer underrun
    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields", fields.len());
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
        }
    }
}
