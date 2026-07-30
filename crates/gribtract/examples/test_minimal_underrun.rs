//! Minimal GRIB2 Buffer Underrun Test Example
//!
//! This program demonstrates the buffer underrun vulnerability in the GRIB2 parser
//! by testing several minimal GRIB2 files that trigger the issue.
//!
//! ## The Vulnerability
//!
//! The bug occurs when Section 3 (Grid Definition Section) claims to contain
//! more bytes than are actually available in the file. When the parser attempts
//! to read the GDT (Grid Definition Template) data, it triggers a buffer underrun.
//!
//! ## Test Cases
//!
//! 1. **minimal_underrun_2bytes.grib2** (51 bytes) - Minimal case with 2-byte padding
//! 2. **rotated_latlon_gdt1_drt0.grib2** (187 bytes) - Original failing file
//! 3. **minimal_underrun.grib2** (50 bytes) - Ultra-minimal variant
//!
//! ## Expected Results
//!
//! Files with malformed Section 3 should produce `TooShort` errors indicating
//! buffer underrun. Files without Section 3 may produce different errors or
//! succeed depending on their structure.
//!
//! ## Running the Example
//!
//! ```bash
//! cargo run --example test_minimal_underrun
//! ```

use gribtract::decode;
use std::fs;

fn main() {
    println!("=== TESTING MINIMAL BUFFER UNDERRUN CASES ===\n");
    println!("This demonstrates the GRIB2 parser buffer underrun vulnerability.\n");

    // Test the 2-byte padding case
    println!("Test 1: minimal_underrun_2bytes.grib2");
    println!("  Expected: Buffer underrun (Section 3 claims > contains)");
    test_file("tests/corpus/small/minimal_underrun_2bytes.grib2");

    // Test the original case
    println!("Test 2: rotated_latlon_gdt1_drt0.grib2 (original)");
    println!("  Expected: Buffer underrun (original failing file)");
    test_file("tests/corpus/small/rotated_latlon_gdt1_drt0.grib2");

    // Test the clean exit case
    println!("Test 3: minimal_underrun.grib2 (should exit cleanly)");
    println!("  Expected: May succeed or fail with different error");
    test_file("tests/corpus/small/minimal_underrun.grib2");

    println!("=== SUMMARY ===");
    println!("Buffer underrun occurs when Section 3 (Grid Definition Section)");
    println!("claims more bytes than are actually available in the file.");
    println!("This triggers a TooShort error when reading GDT template data.");
}

/// Test a single GRIB2 file and report the results
fn test_file(fixture_path: &str) {
    let bytes = match fs::read(fixture_path) {
        Ok(b) => b,
        Err(e) => {
            println!("  ✗ Failed to read file: {}\n", e);
            return;
        }
    };

    println!("  File size: {} bytes", bytes.len());

    match decode(&bytes) {
        Ok(fields) => {
            println!("  ✓ Decoded {} fields successfully", fields.len());
            println!("  (This file did not trigger the buffer underrun)\n");
        }
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("TooShort") {
                println!("  ✓ Buffer underrun reproduced: {:?}", e);
                println!("  (This is the expected vulnerability trigger)\n");
            } else {
                println!("  ✗ Different error: {:?}", e);
                println!("  (File failed for other reasons)\n");
            }
        }
    }
}
