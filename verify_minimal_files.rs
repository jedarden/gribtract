use std::fs;
use gribtract::decode;

fn main() {
    println!("=== Verifying Minimal GRIB2 Test Files ===\n");

    let test_files = vec![
        ("tests/corpus/small/minimal_underrun.grib2", 50),
        ("tests/corpus/small/minimal_underrun_2bytes.grib2", 51),
        ("tests/corpus/small/rotated_latlon_gdt1_drt0.grib2", 187),
    ];

    for (path, expected_size) in test_files {
        println!("Testing: {}", path);

        // Check file exists
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                println!("  ✗ Failed to read: {}", e);
                continue;
            }
        };

        // Check file size
        println!("  File size: {} bytes (expected: {})", bytes.len(), expected_size);

        // Check GRIB2 indicator
        if bytes.len() >= 4 {
            let indicator = String::from_utf8_lossy(&bytes[0..4]);
            println!("  Indicator: {}", indicator);
        }

        // Check end marker
        if bytes.len() >= 4 {
            let end = &bytes[bytes.len()-4..];
            println!("  End marker: {:?}", end);
        }

        // Try to decode
        match decode(&bytes) {
            Ok(fields) => {
                println!("  ✓ Decoded {} fields", fields.len());
            }
            Err(e) => {
                println!("  ✗ Decode error: {:?}", e);
                if let gribtract::Error::TooShort { needed, got } = e {
                    println!("  Buffer underrun detected: needed {} bytes, got {}", needed, got);
                }
            }
        }
        println!();
    }

    println!("=== Verification Complete ===");
}
