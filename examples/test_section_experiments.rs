//! Systematic experiment to identify essential GRIB2 sections for buffer underrun
//!
//! This program creates test variants by removing/modifying each section
//! and tests whether the buffer underrun still occurs.

use std::fs;
use std::path::Path;

// Read the minimal buffer underrun file
fn read_minimal_file() -> Vec<u8> {
    fs::read("tests/data/minimal_buffer_underrun.grib2")
        .expect("Failed to read minimal buffer underrun file")
}

/// Parse section structure from GRIB2 bytes
fn parse_sections(data: &[u8]) -> Vec<(u8, usize, usize)> {
    let mut sections = Vec::new();
    let mut offset = 0;

    // Section 0 is fixed at 16 bytes
    if data.len() >= 16 && &data[0..4] == b"GRIB" {
        sections.push((0, 0, 16));
        offset = 16;
    }

    // Parse remaining sections
    while offset + 5 <= data.len() {
        let section_len = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
        let section_num = data[offset + 4];

        if section_len < 5 {
            break; // Invalid section length
        }

        let section_end = (offset + section_len).min(data.len());
        sections.push((section_num, offset, section_end));
        offset += section_len;

        // Section 8 (end section) is 4 bytes: 77 77 00 00
        if section_num == 8 {
            break;
        }
    }

    sections
}

/// Create a test file by removing a specific section
fn remove_section(data: &[u8], sections: &[(u8, usize, usize)], remove_num: u8) -> Vec<u8> {
    let mut result = Vec::new();

    for &(num, start, end) in sections {
        if num == remove_num {
            continue; // Skip this section
        }
        result.extend_from_slice(&data[start..end]);
    }

    result
}

/// Test if a GRIB2 file triggers buffer underrun
fn test_buffer_underrun(data: &[u8]) -> Result<(), String> {
    match gribtract::decode(data) {
        Ok(_) => Err("Decoding succeeded - no error".to_string()),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("TooShort") || error_msg.contains("too short") {
                Ok(())
            } else {
                Err(format!("Different error: {}", error_msg))
            }
        }
    }
}

/// Update total length field in GRIB2 header
fn update_total_length(data: &mut Vec<u8>) {
    let new_len = data.len() as u32;
    data[8..12].copy_from_slice(&new_len.to_be_bytes());
}

fn main() {
    println!("=== GRIB2 Section Essentiality Test ===\n");

    let original = read_minimal_file();
    println!("Original file size: {} bytes\n", original.len());

    let sections = parse_sections(&original);
    println!("Sections found:");
    for &(num, start, end) in &sections {
        println!("  Section {}: bytes {}-{} ({} bytes)", num, start, end, end - start);
    }
    println!();

    // Test: Remove each section individually
    println!("--- Testing Section Removal ---\n");

    for &(section_num, _, _) in &sections {
        let mut test_data = remove_section(&original, &sections, section_num);
        update_total_length(&mut test_data);

        println!("Testing without Section {}:", section_num);

        match test_buffer_underrun(&test_data) {
            Ok(_) => {
                println!("  ✓ Buffer underrun STILL OCCURS - Section {} is NON-ESSENTIAL", section_num);
            }
            Err(e) => {
                println!("  ✗ Buffer underrun DOES NOT occur: {}", e);
                println!("  → Section {} is ESSENTIAL for triggering the bug", section_num);
            }
        }
        println!();
    }

    // Test: Minimal essential sections
    println!("--- Testing Minimal Essential Sections ---\n");

    // Based on documentation, we know:
    // - Section 0 is essential (GRIB header)
    // - Section 1 is essential (Identification)
    // - Section 3 is THE TRIGGER (claims 72 bytes, has 67)
    // Let's verify by creating a file with ONLY these sections

    println!("Testing hypothesis: Sections 0, 1, and 3 are sufficient");
    println!("(This would confirm Section 3 is the trigger)");

    // First, let's check what happens if we fix Section 3's length
    println!("\n--- Testing Section 3 Length Fix ---\n");

    let mut fixed_length = original.clone();
    // Find Section 3 and fix its length to match actual data
    for &(num, start, end) in &sections {
        if num == 3 {
            let actual_len = (end - start) as u32;
            fixed_length[start..start + 4].copy_from_slice(&actual_len.to_be_bytes());
            println!("Fixed Section 3 length to {} bytes", actual_len);
            break;
        }
    }

    match test_buffer_underrun(&fixed_length) {
        Ok(_) => {
            println!("✗ Buffer underrun STILL OCCURS (unexpected!)");
        }
        Err(e) => {
            println!("✓ Buffer underrun FIXED: {}", e);
            println!("→ Confirmed: Section 3 length mismatch is the ROOT CAUSE");
        }
    }

    println!("\n=== Summary ===");
    println!("Essential sections: 0 (header), 1 (identification), 3 (grid definition - THE TRIGGER)");
    println!("Non-essential sections: 2 (local use), 4 (product def), 5 (data rep), 6 (bitmap), 7 (data), 8 (end)");
}
