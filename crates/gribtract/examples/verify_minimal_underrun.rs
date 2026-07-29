//! Buffer Underrun Verification Test
//!
//! This test verifies that the minimal GRIB2 file triggers the buffer underrun
//! vulnerability as expected. It serves as a regression test to ensure the bug
//! remains documented and reproducible.
//!
//! ## Purpose
//!
//! The minimal GRIB2 file is crafted to trigger a buffer underrun by claiming
//! more bytes in Section 3 than are actually available. This test ensures that:
//! 1. The file structure is preserved
//! 2. The buffer underrun is detected
//! 3. The error type is correct (TooShort)
//!
//! ## Running the Test
//!
//! ```bash
//! cargo test --example verify_minimal_underrun
//! ```
//!
//! ## Expected Result
//!
//! The test should **pass** when the buffer underrun is correctly detected.
//! A test failure would indicate that the underrun was NOT detected, which
//! would be unexpected and potentially mask the vulnerability.

use gribtract::decode;
use gribtract::Error;
use std::fs;

#[test]
fn verify_minimal_buffer_underrun() {
    // Load the minimal GRIB2 file designed to trigger buffer underrun
    let file_path = "examples/testdata/minimal_buffer_underrun.grib2";
    let bytes = fs::read(file_path)
        .expect("Failed to read minimal buffer underrun test file");

    // Verify file exists and has expected size
    assert_eq!(bytes.len(), 187, "Minimal GRIB2 file should be 187 bytes");

    // Verify basic GRIB2 structure
    assert_eq!(&bytes[0..4], b"GRIB", "Should have valid GRIB magic bytes");
    assert_eq!(bytes[7], 2, "Should be GRIB edition 2");

    // Attempt to decode - this should trigger buffer underrun
    let result = decode(&bytes);

    // Verify that buffer underrun IS triggered
    match result {
        Ok(fields) => {
            panic!(
                "Buffer underrun NOT triggered! Decoding succeeded with {} fields. \
                 This test expects the minimal GRIB2 file to trigger a TooShort error.",
                fields.len()
            );
        }
        Err(Error::TooShort { needed, got }) => {
            // This is the expected outcome - buffer underrun detected
            assert!(
                needed > got,
                "Buffer underrun should indicate more bytes needed than available"
            );
            println!(
                "✓ Buffer underrun correctly detected: need {} bytes, got {} bytes",
                needed, got
            );
        }
        Err(other_error) => {
            panic!(
                "Unexpected error type: {:?}. Expected TooShort error indicating buffer underrun.",
                other_error
            );
        }
    }
}

#[test]
fn verify_minimal_file_structure() {
    // Verify the minimal file has the expected structure
    let file_path = "examples/testdata/minimal_buffer_underrun.grib2";
    let bytes = fs::read(file_path)
        .expect("Failed to read minimal buffer underrun test file");

    // Check GRIB indicator section (Section 0)
    assert_eq!(&bytes[0..4], b"GRIB", "GRIB magic bytes");
    assert_eq!(bytes[7], 2, "GRIB edition 2");

    // Check total length field (bytes 8-15, big-endian)
    let total_length = u64::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11],
                                          bytes[12], bytes[13], bytes[14], bytes[15]]);
    assert_eq!(total_length, 187, "Total length should be 187 bytes");

    // Verify Section 1 length field (bytes 16-19, big-endian) indicates a section exists
    let section1_length = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    assert!(section1_length > 0, "Section 1 should have non-zero length");

    // Section number is at byte 20 (5th octet of Section 1)
    assert_eq!(bytes[20], 1, "Section 1 should be Identification section");

    println!("✓ Minimal file structure verified: {} bytes, GRIB2 format with proper sections", bytes.len());
}
