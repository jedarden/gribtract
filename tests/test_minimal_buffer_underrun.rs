//! Test for minimal GRIB2 buffer underrun vulnerability reproduction
//!
//! This test uses the minimal GRIB2 test file to verify that the parser
//! correctly handles the buffer underrun vulnerability that occurs when
//! Section 3 claims more bytes than are actually available.

use std::path::Path;

/// Test that the minimal GRIB2 file reproduces the buffer underrun error
#[test]
fn test_minimal_buffer_underrun() {
    // Path to the minimal GRIB2 test file
    let test_file_path = Path::new("tests/data/minimal_buffer_underrun.grib2");

    // Read the minimal GRIB2 data
    let minimal_grib2 = std::fs::read(test_file_path)
        .expect("Failed to read minimal buffer underrun test file");

    // Verify the file exists and has the expected size (159 bytes)
    assert_eq!(minimal_grib2.len(), 159, "Test file should be 159 bytes");

    // Attempt to decode the GRIB2 data
    match gribtract::decode(&minimal_grib2) {
        Ok(_) => {
            // If decoding succeeds, this is unexpected - the test should fail
            panic!("Expected buffer underrun error, but decoding succeeded unexpectedly. \
                   The vulnerability may have been fixed or the test file may be corrupted.");
        }
        Err(e) => {
            // Check that the error is a TooShort error (buffer underrun)
            let error_msg = format!("{:?}", e);
            assert!(
                error_msg.contains("TooShort") || error_msg.contains("too short"),
                "Expected 'TooShort' buffer underrun error, got: {:?}", e
            );
        }
    }
}

/// Test that verifies the specific TooShort error parameters
#[test]
fn test_buffer_underrun_error_details() {
    let test_file_path = Path::new("tests/data/minimal_buffer_underrun.grib2");

    let minimal_grib2 = std::fs::read(test_file_path)
        .expect("Failed to read minimal buffer underrun test file");

    match gribtract::decode(&minimal_grib2) {
        Ok(_) => panic!("Expected buffer underrun error"),
        Err(gribtract_core::error::Error::TooShort { needed, got }) => {
            // Verify the error parameters match the expected vulnerability
            // The file is 159 bytes total, but the error occurs when trying to read
            // beyond available data in Section 3
            assert!(got <= 159, "Error should report file size <= 159 bytes");
            assert!(needed > got, "Error should indicate more bytes needed than available");
        }
        Err(e) => {
            panic!("Expected TooShort error but got different error: {:?}", e);
        }
    }
}

/// Test that the minimal file structure is preserved
#[test]
fn test_minimal_file_structure() {
    let test_file_path = Path::new("tests/data/minimal_buffer_underrun.grib2");

    let minimal_grib2 = std::fs::read(test_file_path)
        .expect("Failed to read minimal buffer underrun test file");

    // Verify GRIB magic bytes
    assert_eq!(&minimal_grib2[0..4], b"GRIB", "File should start with GRIB magic bytes");

    // Verify GRIB edition (2)
    assert_eq!(minimal_grib2[7], 2, "File should be GRIB edition 2");

    // Verify total length (little-endian at bytes 8-11)
    let total_length = u32::from_le_bytes([
        minimal_grib2[8], minimal_grib2[9],
        minimal_grib2[10], minimal_grib2[11]
    ]) as usize;
    assert_eq!(total_length, 159, "Declared total length should be 159 bytes");

    // Verify actual file size matches declared length
    assert_eq!(minimal_grib2.len(), total_length,
               "Actual file size should match declared total length");
}
