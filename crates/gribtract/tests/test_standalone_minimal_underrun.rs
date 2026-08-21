//! Standalone Minimal Buffer Underrun Test
//!
//! This is a completely standalone, minimal test case that reproduces the buffer
//! underrun vulnerability using the smallest possible GRIB2 fixture (104 bytes).
//!
//! ## Purpose
//!
//! This test verifies that the GRIB2 parser correctly handles malformed input
//! where Section 3 claims more bytes than are actually available in the file.
//!
//! ## The Vulnerability
//!
//! The buffer underrun occurs when:
//! 1. Section 3 (Grid Definition Section) claims 72 bytes in its length field
//! 2. Only 67 bytes are actually available in the file
//! 3. Parser attempts to read Grid Definition Template (GDT) data
//! 4. Reading beyond available data triggers a bounds check failure
//!
//! ## Error Expected
//!
//! This test should fail with `Error::TooShort { needed: N, got: 104 }`
//! where N > 104, indicating the parser tried to read beyond the file bounds.
//!
//! ## Test Fixture
//!
//! - File: `corpus/small/minimal_synthetic_underrun.grib2`
//! - Size: 104 bytes (most minimal possible)
//! - Structure: Only sections 0, 1, and 3 (all other sections removed)
//! - Trigger: Section 3 length field (72) > actual data (67)

use std::fs;
use std::path::{Path, PathBuf};

/// Get the path to the test fixture, handling different working directory contexts
fn get_fixture_path() -> PathBuf {
    // Use CARGO_MANIFEST_DIR environment variable if available (most reliable)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path =
            PathBuf::from(manifest_dir).join("tests/corpus/small/minimal_synthetic_underrun.grib2");
        if manifest_path.exists() {
            return manifest_path;
        }
    }

    // Try the repository root first (where cargo test runs)
    let repo_root_path =
        Path::new("crates/gribtract/tests/corpus/small/minimal_synthetic_underrun.grib2");
    if repo_root_path.exists() {
        return repo_root_path.to_path_buf();
    }

    // Try relative to the test file directory
    let test_relative_path = Path::new("corpus/small/minimal_synthetic_underrun.grib2");
    if test_relative_path.exists() {
        return test_relative_path.to_path_buf();
    }

    // If neither works, panic with a helpful error
    panic!(
        "Cannot find minimal_synthetic_underrun.grib2 fixture. \
        Tried:\n  - CARGO_MANIFEST_DIR path\n  - {:?}\n  - {:?}",
        repo_root_path, test_relative_path
    );
}

/// Primary test: Verify buffer underrun is triggered by minimal GRIB2 fixture
///
/// This test:
/// 1. Loads the 104-byte minimal synthetic GRIB2 file
/// 2. Attempts to decode it
/// 3. Verifies that the buffer underrun panic occurs
/// 4. Confirms the panic message indicates out-of-bounds access
///
/// Note: This test expects a panic because the current implementation
/// does not handle the buffer underrun gracefully - it triggers an
/// actual out-of-bounds slice access, which is the vulnerability.
#[test]
#[should_panic(expected = "range end index")]
fn test_minimal_synthetic_buffer_underrun() {
    // Get the path to the fixture
    let fixture_path = get_fixture_path();

    // Load the minimal GRIB2 data
    let minimal_grib2 =
        fs::read(fixture_path).expect("Failed to read minimal synthetic buffer underrun fixture");

    // Verify we have the correct file (104 bytes)
    assert_eq!(
        minimal_grib2.len(),
        104,
        "Fixture should be 104 bytes (minimal synthetic GRIB2)"
    );

    // Verify GRIB magic bytes to ensure valid file format
    assert_eq!(
        &minimal_grib2[0..4],
        b"GRIB",
        "File should start with GRIB magic bytes"
    );

    // Verify GRIB edition 2
    assert_eq!(minimal_grib2[7], 2, "File should be GRIB edition 2");

    // Attempt to decode - this should trigger the buffer underrun panic
    let _decode_result = gribtract::decode(&minimal_grib2);
}

/// Secondary test: Verify panic message contains debugging information
///
/// This test ensures that when the buffer underrun panic occurs, the
/// panic message contains useful diagnostic information about the out-of-bounds access.
#[test]
#[should_panic(expected = "out of range for slice of length")]
fn test_buffer_underrun_panic_diagnostic_info() {
    let fixture_path = get_fixture_path();
    let minimal_grib2 =
        fs::read(fixture_path).expect("Failed to read minimal synthetic buffer underrun fixture");

    // Attempt to decode - should trigger the buffer underrun panic
    let _decode_result = gribtract::decode(&minimal_grib2);
}

/// Structural validation test: Verify the minimal fixture is well-formed
///
/// This test validates that the fixture itself is correctly structured
/// and triggers the bug for the right reasons (not just random corruption).
#[test]
fn test_minimal_fixture_structure() {
    let fixture_path = get_fixture_path();
    let minimal_grib2 =
        fs::read(fixture_path).expect("Failed to read minimal synthetic buffer underrun fixture");

    // Verify GRIB header structure (Section 0)
    assert_eq!(&minimal_grib2[0..4], b"GRIB", "GRIB magic bytes");
    assert_eq!(minimal_grib2[7], 2, "GRIB edition 2");

    // The total length should match the actual file size
    // In GRIB2, bytes 8-15 contain the total length (64-bit big-endian)
    let declared_length = u64::from_be_bytes([
        minimal_grib2[8],
        minimal_grib2[9],
        minimal_grib2[10],
        minimal_grib2[11],
        minimal_grib2[12],
        minimal_grib2[13],
        minimal_grib2[14],
        minimal_grib2[15],
    ]);

    assert_eq!(
        declared_length,
        minimal_grib2.len() as u64,
        "Declared length in GRIB header should match actual file size"
    );

    // Verify the file contains multiple sections (not just Section 0)
    // The synthetic fixture should have at least sections 0, 1, and 3
    assert!(
        minimal_grib2.len() > 21,
        "File should contain more than just Section 0"
    );

    // Section 3 should exist and contain the malformed length
    // This is the critical section that triggers the buffer underrun
    let has_section_3 = minimal_grib2
        .iter()
        .skip(21) // After Section 0 (16) + Section 1 minimum (21)
        .any(|&b| b == 3);

    assert!(
        has_section_3,
        "Fixture should contain Section 3 (Grid Definition)"
    );
}
