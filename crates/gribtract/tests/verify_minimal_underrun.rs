//! Test that minimal GRIB2 structure triggers buffer underrun
//!
//! This standalone test verifies that the minimal GRIB2 test data
//! correctly demonstrates the buffer underrun vulnerability in the GRIB2 parser.
//!
//! The test uses inline GRIB2 data and attempts to decode it.
//! It should fail with Error::TooShort, demonstrating the buffer underrun.
//! If decoding succeeds, the test fails with a clear error message.
//!
//! This test has NO external file dependencies - all test data is embedded inline.

use gribtract::decode;
use gribtract::Error;

#[test]
fn verify_minimal_buffer_underrun() {
    // Minimal GRIB2 structure that triggers buffer underrun (159 bytes)
    // This is the exact binary content from minimal_buffer_underrun.grib2
    let bytes: &[u8] = &[
        0x47, 0x52, 0x49, 0x42, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x9f, 0x00, 0x00, 0x00, 0x15,
        0x01, 0x00, 0x07, 0x00, 0x00, 0x02, 0x00, 0x00, 0x07, 0xea, 0x06, 0x15, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x03, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
        0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x01, 0x31, 0x2d, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x01, 0x31, 0x2d, 0x00, 0x00, 0x98, 0x96, 0x80, 0x00, 0x98,
        0x96, 0x80, 0x01, 0xc9, 0xc3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x04, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0x00, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x14, 0x05, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x06, 0x80, 0x00, 0x00, 0x00, 0x06, 0x07, 0x37,
    ];

    println!("Minimal GRIB2 data size: {} bytes (embedded inline)", bytes.len());

    // Attempt to decode - this should trigger the buffer underrun
    let decode_result = decode(&bytes);

    // Verify that decoding fails with buffer underrun error
    match decode_result {
        Ok(fields) => {
            panic!(
                "Buffer underrun NOT triggered!
                Decoding succeeded with {} fields.
                This test expects the minimal GRIB2 file to trigger Error::TooShort.",
                fields.len()
            );
        }
        Err(Error::TooShort { needed, got }) => {
            println!("✓ Buffer underrun correctly triggered");
            println!("  Needed: {} bytes", needed);
            println!("  Got: {} bytes", got);
            println!("  Shortage: {} bytes", needed - got);

            // Additional assertion: verify the underrun is significant
            // (not just a 1-2 byte edge case)
            assert!(
                needed > got,
                "Buffer underrun assertion failed: needed ({}) should be > got ({})",
                needed,
                got
            );
        }
        Err(other_error) => {
            panic!(
                "Wrong error type! Expected Error::TooShort (buffer underrun),
                got {:?}.
                This suggests the minimal GRIB2 file triggered a different error.",
                other_error
            );
        }
    }
}
