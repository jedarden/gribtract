//! Verify that minimal_buffer_underrun.grib2 triggers buffer underrun
//!
//! This test demonstrates the buffer underrun vulnerability in the GRIB2 parser
//! when Section 3 (Grid Definition Section) claims a larger size than actually
//! available in the file.
//!
//! ## Bug Trigger Mechanism
//!
//! The buffer underrun occurs through this sequence:
//!
//! 1. Parser reads Section 3 header (bytes 37-41) which claims 72 bytes
//! 2. Parser reads Grid Definition Template number (bytes 50-51) = 0
//! 3. Parser calculates required template data size based on grid dimensions
//! 4. Parser attempts to read template data but section is shorter than claimed
//! 5. Result: `TooShort { needed: 1, got: 0 }` error or undefined behavior
//!
//! ## Why This Matters
//!
//! This vulnerability demonstrates a class of parser bugs where:
//! - The parser trusts section length fields without validation
//! - No bounds checking occurs before reading structured data
//! - Malicious or corrupted files can trigger memory safety issues
//!
//! ## Size Comparison
//!
//! - Minimal test file: 187 bytes
//! - Full GRIB2 files: 5MB - 121MB
//! - Reduction: 99.6%+ smaller while still triggering the bug
//!
//! ## Usage
//!
//! Run as an example:
//! ```bash
//! cargo run --example verify_minimal_underrun
//! ```
//!
//! Run as a test:
//! ```bash
//! cargo test --example verify_minimal_underrun
//! ```

use std::fs;
use gribtract::decode;

fn main() {
    let file_path = "examples/testdata/minimal_buffer_underrun.grib2";

    println!("Testing minimal GRIB2 file: {}", file_path);
    println!("Purpose: Trigger buffer underrun vulnerability\n");

    let bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            println!("✗ Failed to read file: {}", e);
            return;
        }
    };

    println!("File size: {} bytes", bytes.len());

    // Verify basic GRIB2 structure to ensure we're testing the right file
    // Section 0 (Indicator): bytes 0-15
    if &bytes[0..4] == b"GRIB" {
        println!("✓ Valid GRIB indicator (bytes 0-3)");
    } else {
        println!("✗ Invalid GRIB indicator");
        return;
    }

    if bytes.len() >= 8 && bytes[7] == 2 {
        println!("✓ GRIB Edition 2 (byte 7)");
    } else {
        println!("✗ Not GRIB Edition 2");
        return;
    }

    // Section 3 starts at byte 37 and claims 72 bytes (but only has 67)
    // This is where the buffer underrun will trigger
    println!("\nAttempting decode - buffer underrun expected at Section 3...");

    // Attempt to decode - this should trigger the buffer underrun
    match decode(&bytes) {
        Ok(fields) => {
            println!("✗ Decoding succeeded ({} fields) - buffer underrun NOT triggered", fields.len());
            println!("  This means the file is valid or the bug has been fixed");
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("TooShort") {
                println!("✓ Buffer underrun successfully triggered!");
                println!("  Error details: {:?}", e);
                println!("\n  This confirms the vulnerability exists:");
                println!("  - Section 3 claims 72 bytes but only contains 67");
                println!("  - Parser attempts to read beyond available data");
                println!("  - Triggers memory safety check or undefined behavior");
            } else if error_msg.contains("NotImplemented") {
                println!("✗ NotImplemented error - buffer underrun NOT triggered");
                println!("  This happens if the file is too small to reach Section 3");
            } else {
                println!("✗ Different error: {:?}", e);
                println!("  This may indicate the bug has been fixed or file structure changed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_buffer_underrun_triggered() {
        let file_path = "examples/testdata/minimal_buffer_underrun.grib2";
        let bytes = fs::read(file_path).expect("Failed to read test file");

        // Verify file structure
        assert_eq!(&bytes[0..4], b"GRIB", "Should have valid GRIB magic bytes");
        assert_eq!(bytes[7], 2, "Should be GRIB edition 2");
        assert_eq!(bytes.len(), 187, "File should be 187 bytes");

        // Test that buffer underrun is triggered
        let result = decode(&bytes);
        match result {
            Ok(_) => panic!("Buffer underrun NOT triggered - decoding succeeded"),
            Err(e) => {
                let error_msg = format!("{:?}", e);
                assert!(
                    error_msg.contains("TooShort"),
                    "Expected TooShort error, got: {:?}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_file_structure() {
        let file_path = "examples/testdata/minimal_buffer_underrun.grib2";
        let bytes = fs::read(file_path).expect("Failed to read test file");

        // Section 0: Indicator (16 bytes)
        assert_eq!(&bytes[0..4], b"GRIB");
        assert_eq!(bytes[7], 2);

        // Section 1: Identification (starts at byte 16)
        assert_eq!(bytes[16], 1, "Section 1 number");
        let section1_len = u32::from_be_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
        assert_eq!(section1_len, 21, "Section 1 length");

        // Section 3: Grid Definition (starts at byte 37)
        assert_eq!(bytes[37], 3, "Section 3 number");
        let section3_len = u32::from_be_bytes([bytes[38], bytes[39], bytes[40], bytes[41]]);
        assert_eq!(section3_len, 72, "Section 3 claimed length (mismatch!)");

        // End Section marker
        assert_eq!(&bytes[172..176], b"7777", "End section marker");
    }
}
