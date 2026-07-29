// Test that minimal_buffer_underrun.grib2 triggers buffer underrun
use std::fs;
use gribtract::decode;

fn main() {
    let file_path = "examples/testdata/minimal_buffer_underrun.grib2";

    println!("Testing minimal GRIB2 file: {}", file_path);

    let bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            println!("✗ Failed to read file: {}", e);
            std::process::exit(1);
        }
    };

    println!("File size: {} bytes", bytes.len());

    // Verify basic GRIB2 structure
    if &bytes[0..4] == b"GRIB" {
        println!("✓ Valid GRIB indicator");
    }

    if bytes.len() >= 8 && bytes[7] == 2 {
        println!("✓ GRIB Edition 2");
    }

    // Attempt to decode
    match decode(&bytes) {
        Ok(fields) => {
            println!("✗ Decoding succeeded ({} fields) - buffer underrun NOT triggered", fields.len());
            std::process::exit(1);
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("TooShort") {
                println!("✓ Buffer underrun successfully triggered: {:?}", e);
                println!("\n=== SUCCESS ===");
                println!("Minimal GRIB2 file triggers buffer underrun as expected");
            } else {
                println!("✗ Different error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
}
