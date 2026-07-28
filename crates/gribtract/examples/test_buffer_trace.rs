use std::fs;
use gribtract::{decode, Error};

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("=== PRECISE BUFFER UNDERRUN TRACE ===");
    println!("File size: {} bytes", bytes.len());
    println!();

    // Show the exact bytes from Section 3 onwards
    println!("BYTES FROM SECTION 3 ONWARDS (byte 37 onwards):");
    for (i, chunk) in bytes[37..].chunks(16).enumerate() {
        print!("  {:04x}: ", 37 + i * 16);
        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02x} ", byte);
            if j == 7 {
                print!(" ");
            }
        }
        println!();
    }
    println!();

    println!("SECTION STRUCTURE:");
    println!("  Section 0: bytes 0-15 (16 bytes)");
    println!("  Section 1: bytes 16-36 (21 bytes)");
    println!("  Section 3: starts at byte 37");
    println!();

    // Section 3 structure
    let sec3_start = 37;
    println!("SECTION 3 STRUCTURE:");
    println!("  Section 3 length: bytes 37-40");
    println!("    Bytes: {:02x} {:02x} {:02x} {:02x}", bytes[37], bytes[38], bytes[39], bytes[40]);
    let sec3_len = u32::from_be_bytes([bytes[37], bytes[38], bytes[39], bytes[40]]);
    println!("    Value: {} bytes", sec3_len);
    println!();
    println!("  Section 3 number: byte 41");
    println!("    Value: {}", bytes[41]);
    println!();
    println!("  GDT version: byte 42");
    println!("    Value: {}", bytes[42]);
    println!();

    // Calculate where GDT template ends
    let gdt_start = sec3_start + 5; // Section 3 header is 5 bytes
    println!("  GDT template starts at byte: {}", gdt_start);
    println!();

    println!("BUFFER CALCULATIONS:");
    println!("  File size: {} bytes", bytes.len());
    println!("  Bytes available from GDT start: {}", bytes.len() - gdt_start);
    println!("  If GDT 3.1 (72 bytes), needs to end at: {}", gdt_start + 72);
    println!("  If GDT 0 (has variable size), depends on actual template");
    println!();

    // Try to find what template this actually is
    println!("TEMPLATE ANALYSIS:");
    let gdt_bytes_remaining = bytes.len() - gdt_start;
    println!("  Bytes available for GDT template: {}", gdt_bytes_remaining);

    if gdt_bytes_remaining >= 8 {
        println!("  First 8 bytes of GDT:");
        for i in 0..8 {
            if gdt_start + i < bytes.len() {
                println!("    Byte {} (offset {}): 0x{:02x} = {}", gdt_start + i, i, bytes[gdt_start + i], bytes[gdt_start + i]);
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

            match &e {
                Error::TooShort { needed, got } => {
                    println!("BUFFER UNDERRUN DETAILS:");
                    println!("  Bytes needed: {}", needed);
                    println!("  Bytes available: {}", got);
                    println!("  Shortfall: {}", needed - got);
                    println!();
                    println!("ROOT CAUSE:");
                    println!("  The decoder tries to parse the GDT template but runs");
                    println!("  out of bytes before completing the template. The error");
                    println!("  message shows it needed 1 byte but had 0 bytes available.");
                    println!();
                    println!("  This suggests the decoder is trying to read a field");
                    println!("  (likely scanning_mode or similar) from the template but");
                    println!("  the file is truncated.");
                }
                _ => {
                    println!("Error type: {:?}", e);
                }
            }
        }
    }
}
