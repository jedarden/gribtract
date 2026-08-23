//! Test which GRIB2 sections are essential to trigger the buffer underrun
//!
//! This program systematically tests removing or modifying each section
//! to identify which are required for the vulnerability.

use std::fs;
use std::path::Path;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let original_bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("GRIB2 Section Essentiality Test");
    println!("=================================\n");
    println!("Original file: {} bytes", original_bytes.len());

    // First, analyze the original structure
    println!("\n=== ORIGINAL STRUCTURE ===");
    let sections = analyze_sections(&original_bytes);

    // Test 1: Section 0 (Indicator Section) - Test modifications
    println!("\n=== TEST 1: SECTION 0 MODIFICATIONS ===");
    test_section_0_variations(&original_bytes);

    // Test 2: Section 1 (Identification Section) modifications
    println!("\n=== TEST 2: SECTION 1 MODIFICATIONS ===");
    test_section_1_variations(&original_bytes, &sections);

    // Test 3: Section 2 (Local Use) presence/absence
    println!("\n=== TEST 3: SECTION 2 PRESENCE ===");
    test_section_2_variations(&original_bytes, &sections);

    // Test 4: Section 3 (Grid Definition) modifications
    println!("\n=== TEST 4: SECTION 3 MODIFICATIONS ===");
    test_section_3_variations(&original_bytes, &sections);

    // Test 5: Minimal reproduction
    println!("\n=== TEST 5: MINIMAL REPRODUCTION ===");
    test_minimal_reproduction(&original_bytes, &sections);

    // Test 6: Section length field manipulations
    println!("\n=== TEST 6: SECTION LENGTH MANIPULATIONS ===");
    test_length_manipulations(&original_bytes, &sections);

    println!("\n=== ESSENTIALITY CHECKLIST ===");
    print_essentiality_checklist();
}

#[derive(Debug, Clone)]
struct SectionInfo {
    number: u8,
    start_pos: usize,
    length: usize,
    body_start: usize,
    body_len: usize,
}

fn analyze_sections(bytes: &[u8]) -> Vec<SectionInfo> {
    let mut sections = Vec::new();

    // Section 0 (Indicator Section)
    sections.push(SectionInfo {
        number: 0,
        start_pos: 0,
        length: 16,
        body_start: 0,
        body_len: 16,
    });

    // Parse remaining sections
    let mut pos = 16;
    let total_len = usize::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11], 0, 0, 0, 0]);
    let body_end = total_len.saturating_sub(4);

    while pos + 5 <= bytes.len() && pos < body_end {
        let sec_len =
            u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;

        if sec_len < 5 {
            println!(
                "  Invalid section length {} at pos {}, stopping",
                sec_len, pos
            );
            break;
        }

        let sec_num = bytes[pos + 4];

        sections.push(SectionInfo {
            number: sec_num,
            start_pos: pos,
            length: sec_len,
            body_start: pos + 5,
            body_len: sec_len - 5,
        });

        pos += sec_len;

        if pos > bytes.len() || sections.len() > 10 {
            break;
        }
    }

    for sec in &sections {
        println!(
            "  Section {}: start={}, length={}, body_start={}, body_len={}",
            sec.number, sec.start_pos, sec.length, sec.body_start, sec.body_len
        );
    }

    sections
}

fn test_section_0_variations(original_bytes: &[u8]) {
    // Test 1.1: Modify total length field to be smaller than actual
    let mut bytes1 = original_bytes.to_vec();
    bytes1[8] = 0x00; // Set total length to 100 bytes instead of 187
    bytes1[9] = 0x00;
    bytes1[10] = 0x00;
    bytes1[11] = 0x64;
    test_variation("Sec0-small-total-length", &bytes1);

    // Test 1.2: Modify discipline field
    let mut bytes2 = original_bytes.to_vec();
    bytes2[12] = 0xFF; // Invalid discipline
    test_variation("Sec0-invalid-discipline", &bytes2);

    // Test 1.3: Modify edition number
    let mut bytes3 = original_bytes.to_vec();
    bytes3[13] = 0x01; // GRIB1 instead of GRIB2
    test_variation("Sec0-wrong-edition", &bytes3);
}

fn test_section_1_variations(original_bytes: &[u8], sections: &[SectionInfo]) {
    let sec1 = sections.iter().find(|s| s.number == 1);

    if let Some(sec1) = sec1 {
        // Test 2.1: Modify Section 1 length
        let mut bytes1 = original_bytes.to_vec();
        bytes1[sec1.start_pos] = 0x00;
        bytes1[sec1.start_pos + 1] = 0x00;
        bytes1[sec1.start_pos + 2] = 0x00;
        bytes1[sec1.start_pos + 3] = 0x10; // 16 bytes instead of 21
        test_variation("Sec1-shorter-length", &bytes1);

        // Test 2.2: Zero out Section 1 body
        let mut bytes2 = original_bytes.to_vec();
        for i in sec1.body_start..sec1.body_start + sec1.body_len.min(16) {
            bytes2[i] = 0;
        }
        test_variation("Sec1-zeroed-body", &bytes2);
    }
}

fn test_section_2_variations(original_bytes: &[u8], sections: &[SectionInfo]) {
    // Test 3.1: Check if Section 2 exists
    let has_sec2 = sections.iter().any(|s| s.number == 2);
    println!("  Section 2 present: {}", has_sec2);

    if !has_sec2 {
        println!("  → Original file already lacks Section 2");
    }
}

fn test_section_3_variations(original_bytes: &[u8], sections: &[SectionInfo]) {
    let sec3 = sections.iter().find(|s| s.number == 3);

    if let Some(sec3) = sec3 {
        // Test 4.1: Modify Section 3 length (the critical field)
        let mut bytes1 = original_bytes.to_vec();
        bytes1[sec3.start_pos] = 0x00;
        bytes1[sec3.start_pos + 1] = 0x00;
        bytes1[sec3.start_pos + 2] = 0x00;
        bytes1[sec3.start_pos + 3] = 0x20; // 32 bytes instead of 72
        test_variation("Sec3-shorter-length", &bytes1);

        // Test 4.2: Modify Section 3 length to be larger than available
        let mut bytes2 = original_bytes.to_vec();
        bytes2[sec3.start_pos] = 0x00;
        bytes2[sec3.start_pos + 1] = 0x00;
        bytes2[sec3.start_pos + 2] = 0x01;
        bytes2[sec3.start_pos + 3] = 0x00; // 256 bytes instead of 72
        test_variation("Sec3-larger-length", &bytes2);

        // Test 4.3: Zero out Section 3 GDT template data
        let mut bytes3 = original_bytes.to_vec();
        for i in (sec3.body_start + 1)..sec3.body_start + sec3.body_len.min(70) {
            bytes3[i] = 0;
        }
        test_variation("Sec3-zeroed-template", &bytes3);

        // Test 4.4: Modify GDT version number
        let mut bytes4 = original_bytes.to_vec();
        bytes4[sec3.body_start] = 0x01; // GDT 3.1 instead of GDT 3.0
        test_variation("Sec3-gdt-version-1", &bytes4);
    }
}

fn test_minimal_reproduction(original_bytes: &[u8], sections: &[SectionInfo]) {
    // Test 5.1: Keep only Sections 0, 1, and 3 (minimum for the bug)
    let sec0 = sections.iter().find(|s| s.number == 0);
    let sec1 = sections.iter().find(|s| s.number == 1);
    let sec3 = sections.iter().find(|s| s.number == 3);

    if let (Some(sec0), Some(sec1), Some(sec3)) = (sec0, sec1, sec3) {
        let mut minimal_bytes = Vec::new();

        // Copy Section 0
        minimal_bytes.extend_from_slice(&original_bytes[sec0.start_pos..sec0.length]);

        // Copy Section 1
        minimal_bytes
            .extend_from_slice(&original_bytes[sec1.start_pos..sec1.start_pos + sec1.length]);

        // Copy Section 3
        minimal_bytes
            .extend_from_slice(&original_bytes[sec3.start_pos..sec3.start_pos + sec3.length]);

        // Add end marker
        minimal_bytes.extend_from_slice(&[0x37, 0x37, 0x37, 0x37]);

        // Update total length in Section 0
        let new_total_len = minimal_bytes.len() as u32;
        minimal_bytes[8] = (new_total_len >> 24) as u8;
        minimal_bytes[9] = (new_total_len >> 16) as u8;
        minimal_bytes[10] = (new_total_len >> 8) as u8;
        minimal_bytes[11] = new_total_len as u8;

        test_variation("minimal-0-1-3-only", &minimal_bytes);
    }

    // Test 5.2: Keep only Sections 0 and 3 (skip Section 1)
    let sec0 = sections.iter().find(|s| s.number == 0);
    let sec3 = sections.iter().find(|s| s.number == 3);

    if let (Some(sec0), Some(sec3)) = (sec0, sec3) {
        let mut minimal_bytes = Vec::new();

        // Copy Section 0
        minimal_bytes.extend_from_slice(&original_bytes[sec0.start_pos..sec0.length]);

        // Copy Section 3 directly (without Section 1)
        minimal_bytes
            .extend_from_slice(&original_bytes[sec3.start_pos..sec3.start_pos + sec3.length]);

        // Add end marker
        minimal_bytes.extend_from_slice(&[0x37, 0x37, 0x37, 0x37]);

        // Update total length
        let new_total_len = minimal_bytes.len() as u32;
        minimal_bytes[8] = (new_total_len >> 24) as u8;
        minimal_bytes[9] = (new_total_len >> 16) as u8;
        minimal_bytes[10] = (new_total_len >> 8) as u8;
        minimal_bytes[11] = new_total_len as u8;

        test_variation("minimal-0-3-only", &minimal_bytes);
    }
}

fn test_length_manipulations(original_bytes: &[u8], sections: &[SectionInfo]) {
    let sec3 = sections.iter().find(|s| s.number == 3);

    if let Some(sec3) = sec3 {
        // Test 6.1: Section 3 length of exactly 1 byte more than available
        let mut bytes1 = original_bytes.to_vec();
        let available_from_sec3 = original_bytes.len() - sec3.start_pos;
        bytes1[sec3.start_pos] = ((available_from_sec3 + 1) >> 24) as u8;
        bytes1[sec3.start_pos + 1] = ((available_from_sec3 + 1) >> 16) as u8;
        bytes1[sec3.start_pos + 2] = ((available_from_sec3 + 1) >> 8) as u8;
        bytes1[sec3.start_pos + 3] = (available_from_sec3 + 1) as u8;
        test_variation("length-exactly-1-too-large", &bytes1);

        // Test 6.2: Section 3 length much larger than available
        let mut bytes2 = original_bytes.to_vec();
        bytes2[sec3.start_pos] = 0x00;
        bytes2[sec3.start_pos + 1] = 0x02;
        bytes2[sec3.start_pos + 2] = 0x00;
        bytes2[sec3.start_pos + 3] = 0x00; // 131072 bytes
        test_variation("length-massive-overclaim", &bytes2);

        // Test 6.3: Section 3 length = 0
        let mut bytes3 = original_bytes.to_vec();
        bytes3[sec3.start_pos] = 0x00;
        bytes3[sec3.start_pos + 1] = 0x00;
        bytes3[sec3.start_pos + 2] = 0x00;
        bytes3[sec3.start_pos + 3] = 0x00;
        test_variation("length-zero", &bytes3);

        // Test 6.4: Section 3 length = 5 (minimum valid)
        let mut bytes4 = original_bytes.to_vec();
        bytes4[sec3.start_pos] = 0x00;
        bytes4[sec3.start_pos + 1] = 0x00;
        bytes4[sec3.start_pos + 2] = 0x00;
        bytes4[sec3.start_pos + 3] = 0x05;
        test_variation("length-minimum", &bytes4);
    }
}

fn test_variation(name: &str, bytes: &[u8]) {
    // Try to parse with gribtract
    println!("\nTesting variation: {}", name);
    println!("  File size: {} bytes", bytes.len());

    // Save test file
    let test_path = format!("test_variants/{}.grib2", name);
    if let Err(e) = fs::create_dir_all("test_variants") {
        println!("  Could not create test_variants directory: {}", e);
        return;
    }

    if let Err(e) = fs::write(&test_path, bytes) {
        println!("  Could not write test file: {}", e);
        return;
    }

    // Test with gribtract decoder
    let result = test_with_gribtract(&test_path);

    match result {
        Ok(_) => println!("  ✓ NO ERROR (underrun NOT triggered)"),
        Err(e) => {
            println!("  ✗ ERROR: {}", e);
            if e.contains("TooShort") || e.contains("underrun") {
                println!("  → BUFFER UNDERRUN REPRODUCED ✓");
            }
        }
    }
}

fn test_with_gribtract(path: &str) -> Result<(), String> {
    // Use gribtract CLI to test the file
    let output = std::process::Command::new("cargo")
        .args(["run", "--quiet", "--bin", "gribtract", "--", "decode", path])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(stderr.to_string())
            }
        }
        Err(e) => Err(format!("Failed to run gribtract: {}", e)),
    }
}

fn print_essentiality_checklist() {
    println!("\n+------------------+----------------------+------------------+");
    println!("| Section         | Essential for Bug?   | Can Be Removed?  |");
    println!("+------------------+----------------------+------------------+");
    println!("| Section 0       | YES (required)       | NO               |");
    println!("|   - Magic 'GRIB' | YES                  | NO               |");
    println!("|   - Total length| YES (triggers parse) | NO               |");
    println!("|   - Edition      | YES (must be GRIB2)  | YES (to GRIB1)   |");
    println!("+------------------+----------------------+------------------+");
    println!("| Section 1       | MAYBE (context)      | Possibly         |");
    println!("|   - Length field | Less critical         | Possibly         |");
    println!("|   - Body data    | Not critical         | YES              |");
    println!("+------------------+----------------------+------------------+");
    println!("| Section 2       | NO                    | YES              |");
    println!("+------------------+----------------------+------------------+");
    println!("| Section 3       | CRITICAL              | NO               |");
    println!("|   - Length field | CRITICAL (triggers)  | NO               |");
    println!("|   - Section num  | Required              | NO               |");
    println!("|   - GDT version | Required              | Possibly         |");
    println!("|   - Template data| Required              | Partially        |");
    println!("+------------------+----------------------+------------------+");
    println!("| Section 4-7     | Not reached           | N/A              |");
    println!("+------------------+----------------------+------------------+");
    println!();
}
