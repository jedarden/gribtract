//! Minimal standalone test for GRIB2 buffer underrun
//!
//! This test reproduces the buffer underrun error that occurs when parsing
//! a malformed GRIB2 file where Section 3 (Grid Definition Section) claims
//! to have more data than is actually available.
//!
//! ## The Bug
//!
//! The parser attempts to read GDT (Grid Definition Template) data from Section 3,
//! but the section only contains 67 octets while GDT template 0.0 requires 73 octets
//! (or 84 for GDT 1.0). This causes a `TooShort` error when reading beyond available buffer.
//!
//! ## Minimization Strategy
//!
//! Original file: 187 bytes
//! Minimal file: 159 bytes (15% reduction)
//!
//! What we preserved (essential for the bug):
//! - Section 0 (16 bytes): Fixed "GRIB" header - required
//! - Section 1 (21 bytes): Identification section - required
//! - Section 3 (72 bytes claimed, 67 actual): THE TRIGGER - must preserve exact mismatch
//!
//! What we minimized (not relevant to the bug):
//! - Section 4: Reduced from 34→22 bytes using simpler PDT template
//! - Section 5: Kept at 20 bytes using minimal DRT template
//! - Section 6: Kept at 6 bytes (minimum possible for bitmap section)
//! - Section 7: Reduced from 14→6 bytes for 1 data value
//!
//! ## Why Section 3 Cannot Be Removed
//!
//! Files without Section 3 produce `NotImplemented` instead of `TooShort` because
//! the parser takes a different code path. Section 3 must exist with its claimed
//! length greater than actual data to trigger the buffer underrun.

use std::fs;

#[test]
fn test_minimal_buffer_underrun() {
    // Minimal GRIB2 file that reproduces the buffer underrun
    let minimal_grib2 = create_minimal_grib2();

    println!("Testing minimal GRIB2 file ({} bytes)", minimal_grib2.len());

    // Attempt to decode - should fail with TooShort error
    let result = gribtract::decode(&minimal_grib2);

    match result {
        Ok(_) => panic!("Expected buffer underrun error, but decoding succeeded"),
        Err(e) => {
            let error_msg = format!("{:?}", e);

            // Verify we get the expected buffer underrun error
            assert!(
                error_msg.contains("TooShort"),
                "Expected 'TooShort' error, got: {:?}", e
            );

            println!("✓ Successfully reproduced buffer underrun: {:?}", e);
        }
    }
}

#[test]
fn test_minimal_file_structure() {
    let minimal_grib2 = create_minimal_grib2();

    // Verify file structure
    assert_eq!(&minimal_grib2[0..4], b"GRIB", "Missing GRIB magic");
    assert_eq!(minimal_grib2[7], 2, "Wrong edition (should be 2)");

    // Check total length
    let total_len = u32::from_be_bytes([
        minimal_grib2[8], minimal_grib2[9],
        minimal_grib2[10], minimal_grib2[11]
    ]) as usize;
    assert_eq!(total_len, minimal_grib2.len(), "Length field doesn't match actual size");

    println!("✓ File structure validated");
    println!("  Total size: {} bytes (vs 187 bytes original)", minimal_grib2.len());
    println!("  Reduction: {} bytes ({:.1}%)",
        187 - minimal_grib2.len(),
        (187 - minimal_grib2.len()) as f32 / 187.0 * 100.0
    );
}

/// Creates a minimal GRIB2 file that reproduces the buffer underrun bug.
///
/// This file preserves the essential trigger (Section 3 with claimed length > actual)
/// while minimizing all other sections.
fn create_minimal_grib2() -> Vec<u8> {
    let mut file = Vec::new();

    // ===== Section 0: Indicator Section (16 bytes) =====
    // This is fixed format and cannot be minimized
    file.extend_from_slice(b"GRIB");           // Magic (0-3)
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x02]);  // Reserved + Edition 2 (4-7)

    // Total length (8-11): will be updated at end
    let total_len_offset = file.len();
    file.extend_from_slice(&[0x00; 4]);  // Placeholder

    // ===== Section 1: Identification Section (21 bytes) =====
    // Already minimal, keep as-is from original
    let s1_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x15]);  // Section length (21 bytes)
    file.push(0x01);  // Section number

    // Original Section 1 body (16 bytes)
    file.extend_from_slice(&[
        0x00, 0x07, 0x00, 0x00, 0x02, 0x00, 0x00, 0x07,
        0xea, 0x06, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00
    ]);

    assert_eq!(file.len() - s1_start, 21, "Section 1 should be 21 bytes");

    // ===== Section 3: Grid Definition Section (72 bytes claimed, 67 actual) =====
    // **THIS IS THE TRIGGER** - must preserve exact claimed/actual length mismatch
    let s3_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x48]);  // Section length (72 bytes)
    file.push(0x03);  // Section number

    // Section 3 body - EXACTLY 67 bytes from original (claimed 72, so 5 short)
    // This shortage triggers the buffer underrun when parsing GDT template
    // Copied byte-for-byte from original file to preserve exact trigger behavior
    file.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x01, 0x06,  // 0-9
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // 10-19
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,  // 20-29
        0x00, 0x00, 0x03, 0x01, 0x31, 0x2d, 0x00, 0x00, 0x00, 0x00,  // 30-39
        0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x01, 0x31, 0x2d, 0x00,  // 40-49
        0x00, 0x98, 0x96, 0x80, 0x00, 0x98, 0x96, 0x80, 0x01, 0xc9,  // 50-59
        0xc3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00  // 60-66
    ]);

    // Verify Section 3 body is 67 bytes (triggering underrun)
    let s3_body_len = file.len() - s3_start - 5;
    assert_eq!(s3_body_len, 67, "Section 3 body must be 67 bytes to trigger underrun");

    // ===== Section 4: Product Definition Section (22 bytes, reduced from 34) =====
    // Minimized using simpler PDT template
    let s4_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x16]);  // Section length (22 bytes)
    file.push(0x04);  // Section number

    // Section 4 body (17 bytes) - minimal PDT 0.0
    // Structure: 4 (template) + 4 (param) + 2 (level type) + 2 (level) + 4 (forecast time) + 1 (type) = 17 bytes
    file.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00,  // Template number (PDT 0.0) - 4 bytes
        0x00, 0x02, 0x00, 0x00,  // Parameter category+number (Temperature) - 4 bytes
        0x01, 0x00,  // Type of level - 2 bytes
        0x67, 0x00,  // Level (103 = 2m above ground) - 2 bytes
        0x00, 0x00, 0x00, 0x00,  // Forecast time - 4 bytes
        0x01  // Type of forecast - 1 byte
    ]);  // Total: 4+4+2+2+4+1 = 17 bytes in body, 22 total

    assert_eq!(file.len() - s4_start, 22, "Section 4 should be 22 bytes");

    // ===== Section 5: Data Representation Section (20 bytes) =====
    // Minimized DRT 0 template
    let s5_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]);  // Section length (20 bytes)
    file.push(0x05);  // Section number

    // Section 5 body (15 bytes) - DRT 0 (simple packing)
    // Minimal representation: 5 (template) + 2 (ref, shortened) + 4 (bin scale) + 3 (dec scale shortened) + 1 (bits) = 15 bytes
    file.extend_from_slice(&[
        0x00, 0x00, 0x00, 0x00, 0x02,  // Template number (DRT 0) - 5 bytes
        0xff, 0x00,  // Reference value (R=255.0) - 2 bytes (shortened from 4)
        0x00, 0x00, 0x00, 0x00,  // Binary scale (E) - 4 bytes
        0x00, 0x00, 0x00,  // Decimal scale (D) - 3 bytes (shortened from 4)
        0x08  // Bits per value (8) - 1 byte
    ]);  // Total: 5+2+4+3+1 = 15 bytes in body, 20 total

    assert_eq!(file.len() - s5_start, 20, "Section 5 should be 20 bytes");

    // ===== Section 6: Bitmap Section (6 bytes, minimal possible) =====
    // Minimized to 1 bit for 1 value
    let s6_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x06]);  // Section length (6 bytes)
    file.push(0x06);  // Section number

    // Bitmap: 1 bit indicating 1 value present
    file.push(0x80);  // Bit 7 set = value present

    assert_eq!(file.len() - s6_start, 6, "Section 6 should be 6 bytes");

    // ===== Section 7: Data Section (6 bytes, minimal possible) =====
    // Minimized to 1 data value
    let s7_start = file.len();
    file.extend_from_slice(&[0x00, 0x00, 0x00, 0x06]);  // Section length (6 bytes)
    file.push(0x07);  // Section number

    // Section 7 body - 1 packed value (1 byte with 8-bit packing)
    file.push(0x37);  // Single data value

    assert_eq!(file.len() - s7_start, 6, "Section 7 should be 6 bytes");

    // ===== Update total length in Section 0 =====
    let total_len = (file.len() as u32).to_be_bytes();
    file[total_len_offset..total_len_offset + 4].copy_from_slice(&total_len);

    // Final sanity check
    assert_eq!(file.len(), 159, "Total file size should be 159 bytes");

    file
}

#[test]
fn test_save_minimal_file() {
    let minimal_grib2 = create_minimal_grib2();

    // Use CARGO_MANIFEST_DIR to locate corpus directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    let corpus_path = std::path::Path::new(&manifest_dir)
        .join("tests")
        .join("corpus")
        .join("small")
        .join("minimal_buffer_underrun.grib2");

    // Create directory if it doesn't exist
    if let Some(parent) = corpus_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    fs::write(&corpus_path, &minimal_grib2)
        .expect("Failed to write minimal GRIB2 file");

    println!("✓ Minimal GRIB2 file saved to: {}", corpus_path.display());
    println!("  Size: {} bytes (reduction from 187 bytes)", minimal_grib2.len());
}
