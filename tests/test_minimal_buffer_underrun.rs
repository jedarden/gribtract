//! Test for minimal GRIB2 buffer underrun vulnerability reproduction
//!
//! This test uses the minimal GRIB2 test file to verify that the parser
//! correctly handles the buffer underrun vulnerability that occurs when
//! Section 3 claims more bytes than are actually available.
//!
//! ## The Vulnerability
//!
//! The buffer underrun occurs when the GRIB2 parser attempts to read Grid Definition
//! Template (GDT) data from Section 3, but the section claims to contain more bytes
//! than are actually available in the file.
//!
//! **Error:** `TooShort { needed: <bytes_needed>, got: 159 }`
//!
//! **Root Cause:** Section 3 claims 72 bytes but only contains 67 bytes - a 5-byte shortage
//! that triggers underrun when the parser tries to read GDT template data.
//!
//! ## Minimization Strategy
//!
//! **Original file:** `rotated_latlon_gdt1_drt0.grib2` (187 bytes)
//! **Minimal file:** `minimal_buffer_underrun.grib2` (159 bytes)
//! **Reduction:** 28 bytes (15% smaller)
//!
//! ### Essential Components (Cannot Be Removed)
//!
//! - **Section 0 (16 bytes):** Fixed GRIB header with magic bytes and edition identifier
//! - **Section 1 (21 bytes):** Identification section with discipline, center, and parameter info
//! - **Section 3 (72 bytes claimed, 67 actual):** **THE TRIGGER** - Must preserve exact claimed/actual mismatch
//!   - The 5-byte shortage (72 claimed vs 67 actual) is what triggers the buffer underrun
//!   - GDT 0.0 template requires 73 octets total, creating the underrun condition
//!
//! ### Non-Essential Components (Minimized)
//!
//! - **Section 4:** Reduced from 34→22 bytes by using simpler PDT 0.0 template
//! - **Section 5:** Kept at 20 bytes using minimal DRT 0 (simple packing) template
//! - **Section 6:** Kept at 6 bytes (minimum possible for bitmap section)
//! - **Section 7:** Reduced from 14→6 bytes by reducing to single 1-byte packed value
//!
//! ### Why Section 3 Cannot Be Removed
//!
//! Files without Section 3 produce `NotImplemented` instead of `TooShort` because
//! the parser takes a different code path. The bug specifically triggers when Section 3
//! exists but contains insufficient data for the declared GDT template.
//!
//! ## Original File Reference
//!
//! This minimal file was created from `rotated_latlon_gdt1_drt0.grib2` (187 bytes)
//! by systematically removing non-essential data while preserving the exact Section 3
//! length mismatch that triggers the vulnerability.
//!
//! ## Test Coverage
//!
//! - `test_minimal_buffer_underrun`: Verifies buffer underrun error is reproduced
//! - `test_buffer_underrun_error_details`: Validates TooShort error parameters
//! - `test_minimal_file_structure`: Validates GRIB2 structure and file integrity

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
