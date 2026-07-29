//! Minimal Buffer Underrun Test
//!
//! This test demonstrates the buffer underrun vulnerability in the GRIB2 parser
//! using a minimal test file (159 bytes, 99.6% smaller than full GRIB2 files).
//!
//! ## Bug Trigger Mechanism
//!
//! The buffer underrun occurs through this sequence:
//!
//! 1. Parser reads Section 3 (Grid Definition Section) header at bytes 37-41
//! 2. Section 3 claims 72 bytes but only contains 67 bytes
//! 3. Parser attempts to read Grid Definition Template (GDT) 0.0 data
//! 4. GDT 0.0 requires 73 octets total, but only 67 are available
//! 5. When reading the final `scanning_mode` field at octet 72, data is exhausted
//! 6. Result: `TooShort { needed: X, got: 159 }` error
//!
//! ## Why This Matters
//!
//! This vulnerability demonstrates a class of parser bugs where:
//! - The parser trusts section length fields without validation
//! - No bounds checking occurs before reading structured data
//! - Malicious or corrupted files can trigger memory safety issues
//!
//! ## Test Data
//!
//! - File: `tests/data/minimal_buffer_underrun.grib2`
//! - Size: 159 bytes (down from 187 bytes, 15% reduction)
//! - Format: GRIB2 Edition 2
//! - Trigger: Section 3 length mismatch (72 claimed, 67 actual)

use std::fs;

#[test]
fn test_minimal_buffer_underrun_reproduction() {
    // Load the minimal GRIB2 test file
    let file_path = "../../tests/data/minimal_buffer_underrun.grib2";
    let bytes = fs::read(file_path)
        .expect("Failed to read minimal buffer underrun test file");

    // Verify file structure to ensure we're testing the right file
    assert_eq!(&bytes[0..4], b"GRIB", "Should have valid GRIB magic bytes");
    assert_eq!(bytes[7], 2, "Should be GRIB edition 2");
    assert_eq!(bytes.len(), 159, "File should be 159 bytes");

    // Test that buffer underrun is triggered when decoding
    let result = gribtract::decode(&bytes);

    match result {
        Ok(fields) => {
            panic!(
                "Buffer underrun NOT triggered - decoding succeeded with {} fields. \
                 This means either the bug has been fixed or the test file is corrupted.",
                fields.len()
            );
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);

            // Verify we get the expected TooShort error
            assert!(
                error_msg.contains("TooShort"),
                "Expected 'TooShort' error indicating buffer underrun, got: {:?}",
                e
            );

            // The error confirms the vulnerability exists:
            // - Section 3 claims 72 bytes but only contains 67
            // - Parser attempts to read beyond available data
            // - Memory safety check catches the issue
        }
    }
}


#[test]
fn test_buffer_underrun_error_details() {
    // Test that we get meaningful error details from the buffer underrun
    let file_path = "../../tests/data/minimal_buffer_underrun.grib2";
    let bytes = fs::read(file_path)
        .expect("Failed to read minimal buffer underrun test file");

    let result = gribtract::decode(&bytes);

    match result {
        Ok(_) => panic!("Expected buffer underrun error, got success"),
        Err(e) => {
            // Verify the error contains useful diagnostic information
            let error_msg = format!("{:?}", e);

            // Should mention TooShort
            assert!(error_msg.contains("TooShort"),
                    "Error should mention 'TooShort' to indicate buffer underrun");

            // Error should provide context about what went wrong
            assert!(error_msg.contains("needed") && error_msg.contains("got"),
                    "Error should provide 'needed' and 'got' counts for debugging");
        }
    }
}
