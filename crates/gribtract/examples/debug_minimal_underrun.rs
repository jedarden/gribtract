//! Debug Tool for GRIB2 Buffer Underrun Analysis
//!
//! This program helps debug and analyze the buffer underrun vulnerability
//! in the GRIB2 parser by examining file structure and error details.
//!
//! ## What It Does
//!
//! 1. Reads the original failing GRIB2 file
//! 2. Displays the last 20 bytes in hex format for analysis
//! 3. Attempts to decode and reports detailed error information
//! 4. Shows buffer underrun specifics (needed vs. available bytes)
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example debug_minimal_underrun
//! ```
//!
//! ## Expected Output
//!
//! The program should show:
//! - File size information
//! - Hex dump of the last 20 bytes
//! - Detailed TooShort error with needed/got byte counts
//! - Explanation of the buffer underrun

use gribtract::decode;
use std::fs;

fn main() {
    println!("=== DEBUGGING BUFFER UNDERRUN ===\n");
    println!("Analyzing the original failing GRIB2 file to understand");
    println!("the buffer underrun vulnerability.\n");

    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");
    println!("File: {}", fixture_path);
    println!("Size: {} bytes\n", bytes.len());

    // Display the last 20 bytes in hex format
    println!("=== Last 20 bytes of file (hex dump) ===");
    print_hex_dump(&bytes, 20);
    println!();

    println!("=== Decoding Attempt ===");
    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded successfully (unexpected!)");
            println!("  Decoded {} fields", fields.len());
        }
        Err(e) => {
            println!("✗ Decoding failed with error:");
            println!("  {:?}\n", e);
            analyze_error(&e);
        }
    }
}

/// Display a hex dump of the last N bytes of the file
fn print_hex_dump(bytes: &[u8], tail_bytes: usize) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = bytes.len() - ((bytes.len() / 16 - i) * 16);
        if offset >= bytes.len() - tail_bytes {
            print!("{:04x}: ", offset);
            for (j, byte) in chunk.iter().enumerate() {
                if offset + j >= bytes.len() {
                    print!("   ");
                } else {
                    print!("{:02x} ", byte);
                }
                if j == 7 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

/// Analyze and explain the error in detail
fn analyze_error(error: &gribtract::Error) {
    println!("=== Error Analysis ===");

    match error {
        gribtract::Error::TooShort { needed, got } => {
            println!("Error Type: Buffer Underrun (TooShort)");
            println!("  Needed: {} bytes", needed);
            println!("  Available: {} bytes", got);
            println!("  Shortage: {} bytes", needed.saturating_sub(*got));
            println!();
            println!("Explanation:");
            println!("  The parser attempted to read more data from the buffer");
            println!("  than was actually available. This typically occurs when:");
            println!("  1. A section claims more bytes than it contains");
            println!("  2. Template parsing assumes minimum data size");
            println!("  3. Missing bounds checking before buffer reads");
        }
        _ => {
            println!("Error Type: {:?}", error);
            println!("  (Not a buffer underrun error)");
        }
    }

    println!();
    println!("Root Cause:");
    println!("  Section 3 (Grid Definition Section) claims 72 bytes but");
    println!("  only contains 67 bytes. When parsing the GDT template,");
    println!("  the parser tries to read beyond available data.");
}
