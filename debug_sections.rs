use std::fs;
use gribtract_core::decode::decode_bytes;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());

    // Look for section 7 manually
    let mut pos = 0;
    while pos < bytes.len() {
        if pos + 4 > bytes.len() { break; }

        // Check for GRIB magic
        if &bytes[pos..pos+4] == b"GRIB" {
            println!("Found GRIB at offset {}", pos);
            pos += 16; // Skip section 0
            continue;
        }

        // Check for section marker
        if pos + 5 <= bytes.len() {
            let sec_len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
            let sec_num = bytes[pos + 4];

            if sec_len >= 5 && pos + sec_len <= bytes.len() + 4 {
                println!("Section {} at offset {}, length={}, body_len={}",
                    sec_num, pos, sec_len, sec_len - 5);

                if sec_num == 7 {
                    let body_start = pos + 5;
                    let body_len = sec_len - 5;
                    println!("  Section 7 body: {} bytes starting at offset {}", body_len, body_start);
                    println!("  Body data (hex): {:02x?}", &bytes[body_start..body_start+body_len.min(32)]);
                }

                pos += sec_len;
                continue;
            }
        }

        pos += 1;
    }

    println!("\n--- Attempting decode ---");
    match decode_bytes(&bytes) {
        Ok(fields) => println!("Success: {} fields", fields.len()),
        Err(e) => println!("Error: {:?}", e),
    }
}
