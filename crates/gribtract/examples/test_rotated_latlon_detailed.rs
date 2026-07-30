use gribtract::{decode, Error};
use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("=== DETAILED ROTATED_LATLON_GDT1_DRT0 BUFFER UNDERRUN ANALYSIS ===");
    println!("File size: {} bytes", bytes.len());
    println!();

    // Let's manually trace through the sections to see where the error occurs
    println!("MANUAL SECTION PARSING:");

    // Section 0 (Indicator) - bytes 0-15
    println!("Section 0 (Indicator): bytes 0-15");
    let indicator = &bytes[0..16];
    println!("  GRIB magic: {:02x?}", &indicator[0..4]);
    println!("  Edition: {}", indicator[7]);
    println!(
        "  Total length: {}",
        u64::from_be_bytes([
            indicator[8],
            indicator[9],
            indicator[10],
            indicator[11],
            indicator[12],
            indicator[13],
            indicator[14],
            indicator[15]
        ])
    );
    println!();

    // Section 1 (Identification) - bytes 16-36
    println!("Section 1 (Identification): bytes 16-36");
    if bytes.len() >= 16 {
        let sec1_len = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let sec1_num = bytes[20];
        println!("  Section length: {} bytes", sec1_len);
        println!("  Section number: {}", sec1_num);
        println!("  Section 1 ends at byte: {}", 16 + sec1_len as usize);
        println!();
    }

    // Section 3 (Grid Definition) - should start at byte 37
    println!("Section 3 (Grid Definition): starts at byte 37");
    if bytes.len() >= 37 {
        let sec3_start = 37;
        let sec3_len_bytes = &bytes[sec3_start..sec3_start + 4];
        let sec3_len = u32::from_be_bytes([
            sec3_len_bytes[0],
            sec3_len_bytes[1],
            sec3_len_bytes[2],
            sec3_len_bytes[3],
        ]);
        let sec3_num = bytes[sec3_start + 4];
        println!("  Section length field: {:02x?}", sec3_len_bytes);
        println!("  Section length (interpreted): {} bytes", sec3_len);
        println!("  Section number: {}", sec3_num);

        if sec3_num == 3 {
            println!("  ✓ Section 3 confirmed");

            // GDT version number
            if bytes.len() >= sec3_start + 5 {
                let gdt_version = bytes[sec3_start + 5];
                println!("  GDT version: {}", gdt_version);

                if gdt_version == 1 {
                    println!("  ✓ GDT 3.1 (Rotated LatLon) confirmed");

                    // GDT 3.1 template is 72 bytes
                    let gdt_template_size = 72;
                    let gdt_end = sec3_start + 5 + gdt_template_size;
                    println!("  GDT 3.1 template size: {} bytes", gdt_template_size);
                    println!("  GDT template ends at byte: {}", gdt_end);
                    println!("  File size: {} bytes", bytes.len());

                    if gdt_end > bytes.len() {
                        let missing = gdt_end - bytes.len();
                        println!("  ✗ MISSING BYTES: {} bytes", missing);
                        println!("  This explains the TooShort error!");
                    }
                }
            }
        }
    }
    println!();

    println!("RUNNING FULL DECODE:");
    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields successfully", fields.len());
        }
        Err(e) => {
            println!("✗ Decode error: {:?}", e);
            println!();

            // Try to provide more context about where the error occurs
            match &e {
                Error::TooShort { needed, got } => {
                    println!("BUFFER UNDERRUN DETAILS:");
                    println!("  Bytes needed: {}", needed);
                    println!("  Bytes available: {}", got);
                    println!("  Shortfall: {}", needed - got);
                    println!();
                    println!("ANALYSIS:");
                    println!("  The error occurs when the decoder tries to read a field");
                    println!("  from the GDT 3.1 template but has reached the end of");
                    println!("  the 187-byte file.");
                }
                _ => {
                    println!("Error type (not TooShort): {:?}", e);
                }
            }
        }
    }
}
