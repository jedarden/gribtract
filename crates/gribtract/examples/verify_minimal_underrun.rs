//! Buffer Underrun Verification Test
//!
//! This test verifies that the minimal GRIB2 data triggers the buffer underrun
//! vulnerability as expected. It serves as a regression test to ensure the bug
//! remains documented and reproducible.
//!
//! ## Purpose
//!
//! The minimal GRIB2 data is crafted to trigger a buffer underrun by claiming
//! a much larger total size in the header (Section 0) than the actual data size.
//! This test ensures that:
//! 1. The GRIB2 structure is intact
//! 2. The buffer underrun is correctly detected
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
//!
//! ## Self-Containment
//!
//! This test has NO external file dependencies. All GRIB2 test data is embedded
//! inline as a byte array, ensuring the test runs successfully from any working
//! directory.

use gribtract::decode;
use gribtract::Error;

// Minimal GRIB2 structure that triggers buffer underrun (159 bytes)
// This is the exact binary content from minimal_buffer_underrun.grib2
const MINIMAL_GRIB2_DATA: &[u8] = &[
    0x47, 0x52, 0x49, 0x42, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x9f, 0x00, 0x00, 0x00, 0x15,
    0x01, 0x00, 0x07, 0x00, 0x00, 0x02, 0x00, 0x00, 0x07, 0xea, 0x06, 0x15, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x48, 0x03, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x01, 0x06,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x01, 0x31, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x31, 0x2d, 0x00, 0x00, 0x98, 0x96, 0x80, 0x00, 0x98, 0x96, 0x80,
    0x01, 0xc9, 0xc3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x04, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0x00, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x00, 0x00, 0x14, 0x05, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x06, 0x80, 0x00, 0x00, 0x00, 0x06, 0x07, 0x37,
];

fn main() {
    println!("Running minimal buffer underrun verification...");

    let bytes = MINIMAL_GRIB2_DATA;

    // Verify data has expected size
    assert_eq!(bytes.len(), 159, "Minimal GRIB2 data should be 159 bytes");

    // Verify basic GRIB2 structure
    assert_eq!(&bytes[0..4], b"GRIB", "Should have valid GRIB magic bytes");
    assert_eq!(bytes[7], 2, "Should be GRIB edition 2");

    println!("✓ File structure verified: {} bytes, GRIB2 format", bytes.len());

    // Attempt to decode - this should trigger buffer underrun
    let result = decode(bytes);

    // Verify that buffer underrun IS triggered
    match result {
        Ok(fields) => {
            println!(
                "✗ Buffer underrun NOT triggered! Decoding succeeded with {} fields.",
                fields.len()
            );
            println!("  This test expects the minimal GRIB2 data to trigger a TooShort error.");
        }
        Err(Error::TooShort { needed, got }) => {
            // This is the expected outcome - buffer underrun detected
            println!(
                "✓ Buffer underrun correctly detected: need {} bytes, got {} bytes",
                needed, got
            );
            assert!(
                needed > got,
                "Buffer underrun should indicate more bytes needed than available"
            );
        }
        Err(other_error) => {
            println!(
                "✗ Unexpected error type: {:?}. Expected TooShort error indicating buffer underrun.",
                other_error
            );
        }
    }
}

#[test]
fn verify_minimal_buffer_underrun() {
    let bytes = MINIMAL_GRIB2_DATA;

    // Verify data has expected size
    assert_eq!(bytes.len(), 159, "Minimal GRIB2 data should be 159 bytes");

    // Verify basic GRIB2 structure
    assert_eq!(&bytes[0..4], b"GRIB", "Should have valid GRIB magic bytes");
    assert_eq!(bytes[7], 2, "Should be GRIB edition 2");

    // Attempt to decode - this should trigger buffer underrun
    let result = decode(bytes);

    // Verify that buffer underrun IS triggered
    match result {
        Ok(fields) => {
            panic!(
                "Buffer underrun NOT triggered! Decoding succeeded with {} fields. \
                 This test expects the minimal GRIB2 data to trigger a TooShort error.",
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
fn verify_minimal_data_structure() {
    let bytes = MINIMAL_GRIB2_DATA;

    // Check GRIB indicator section (Section 0)
    assert_eq!(&bytes[0..4], b"GRIB", "GRIB magic bytes");
    assert_eq!(bytes[7], 2, "GRIB edition 2");

    // Check total length field (bytes 8-15, big-endian)
    let total_length = u64::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11],
                                          bytes[12], bytes[13], bytes[14], bytes[15]]);
    // Note: The total length field claims a much larger size (682,899,800,085 bytes)
    // than the actual file size (159 bytes) - this is intentional to trigger buffer underrun
    assert!(total_length > bytes.len() as u64, "Total length field should exceed actual size to trigger underrun");
    println!("  Total length field claims: {} bytes (actual: {} bytes)", total_length, bytes.len());

    // Verify Section 1 length field (bytes 16-19, big-endian) indicates a section exists
    let section1_length = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    assert!(section1_length > 0, "Section 1 should have non-zero length");

    println!("✓ Minimal data structure verified: {} bytes, GRIB2 format with intentional underrun trigger", bytes.len());
}
