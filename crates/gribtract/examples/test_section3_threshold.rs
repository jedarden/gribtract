use std::fs;
use gribtract::decode;

fn main() {
    println!("=== SECTION 3 BODY THRESHOLD TESTING ===\n");

    // Read the original test file
    let original = fs::read("tests/corpus/small/rotated_latlon_gdt1_drt0.grib2")
        .expect("Failed to read original file");

    println!("Original file: {} bytes", original.len());
    println!("Original Section 3: 72 bytes claimed, 67 bytes body\n");

    // Test with the original
    println!("1. Original file (67 bytes Section 3 body):");
    test_decode(&original);

    // The key insight is that we need Section 3 to be present but malformed
    // Let me check what happens with different body sizes by manually constructing test cases

    // First, let's understand the exact byte positions
    println!("\n2. Testing manual Section 3 truncation:");
    test_truncated_variants(&original);
}

fn test_decode(bytes: &[u8]) {
    match decode(bytes) {
        Ok(fields) => println!("    ✓ Decoded {} fields", fields.len()),
        Err(e) => println!("    ✗ Error: {:?}", e)
    }
}

fn test_truncated_variants(original: &[u8]) {
    // Section 3 starts at byte 37, total claimed length is 72 bytes
    // Section 3 header is 5 bytes (4 length + 1 section number)
    // So body starts at byte 42, body claimed length is 67 bytes (72 - 5)

    let section3_start = 37;
    let section3_header_end = 42; // 5 bytes header
    let body_claimed = 67;

    // Test different body sizes while keeping claimed length at 72
    let test_sizes = vec![
        ("Full body", body_claimed),           // 67 bytes (original)
        ("Minus 1 byte", body_claimed - 1),     // 66 bytes
        ("Minus 10 bytes", body_claimed - 10),  // 57 bytes
        ("Minus 20 bytes", body_claimed - 20),  // 47 bytes
        ("Minus 30 bytes", body_claimed - 30),  // 37 bytes
        ("Minimal (10 bytes)", 10),             // 10 bytes
    ];

    for (name, new_body_size) in test_sizes {
        if new_body_size <= 0 {
            continue;
        }

        println!("\n  Testing {} ({} bytes body):", name, new_body_size);

        // Create variant by truncating Section 3 body
        let mut variant = Vec::new();

        // Copy everything before Section 3 body
        variant.extend_from_slice(&original[..section3_header_end]);

        // Copy truncated body
        variant.extend_from_slice(&original[section3_header_end..section3_header_end + new_body_size]);

        // Skip the rest of Section 3 and copy everything after
        // Section 3 ends at: section3_start + 72 = 109
        let section3_end = section3_start + 72;
        if section3_end < original.len() {
            variant.extend_from_slice(&original[section3_end..]);
        }

        // Update total length in Section 0
        let new_total = variant.len() as u32;
        variant[8] = (new_total >> 24) as u8;
        variant[9] = (new_total >> 16) as u8;
        variant[10] = (new_total >> 8) as u8;
        variant[11] = (new_total & 0xFF) as u8;

        println!("    New total size: {} bytes", variant.len());
        test_decode(&variant);
    }

    // Test the key question: what's the minimum body size that still triggers TooShort?
    println!("\n3. Binary search for threshold:");
    binary_search_threshold(original, section3_start, section3_header_end, body_claimed);
}

fn binary_search_threshold(original: &[u8], section3_start: usize, section3_header_end: usize, max_body: usize) {
    let section3_claimed = 72; // Section 3 claimed length
    let section3_end = section3_start + section3_claimed;

    let mut min_triggering = None;
    let mut max_non_triggering = None;

    // Test specific body sizes
    for body_size in (1..=max_body).rev() {
        let mut variant = Vec::new();
        variant.extend_from_slice(&original[..section3_header_end]);
        variant.extend_from_slice(&original[section3_header_end..section3_header_end + body_size]);

        if section3_end < original.len() {
            variant.extend_from_slice(&original[section3_end..]);
        }

        let new_total = variant.len() as u32;
        variant[8] = (new_total >> 24) as u8;
        variant[9] = (new_total >> 16) as u8;
        variant[10] = (new_total >> 8) as u8;
        variant[11] = (new_total & 0xFF) as u8;

        match decode(&variant) {
            Ok(_) => {
                println!("  Body size {:3}: OK (no error)", body_size);
                max_non_triggering = Some(body_size);
            }
            Err(e) => {
                if format!("{:?}", e).contains("TooShort") {
                    println!("  Body size {:3}: ✗ TooShort (buffer underrun!)", body_size);
                    min_triggering = Some(body_size);
                } else {
                    println!("  Body size {:3}: Other error: {:?}", body_size, e);
                }
            }
        }

        // Stop once we find the threshold
        if min_triggering.is_some() && body_size < max_body - 10 {
            println!("  Threshold found, stopping search");
            break;
        }
    }

    if let (Some(min), Some(max)) = (min_triggering, max_non_triggering) {
        println!("\n  THRESHOLD: Body size {} is minimum that triggers TooShort", min);
        println!("  Body size {} does NOT trigger TooShort", max);
    }
}