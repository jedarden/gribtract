use std::fs;
use gribtract::{decode, Error};

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("=== ROTATED_LATLON_GDT1_DRT0 BUFFER UNDERRUN REPRODUCTION ===");
    println!("File size: {} bytes", bytes.len());
    println!();

    // Show hex dump of first 100 bytes
    println!("First 100 bytes (hex):");
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);
        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02x} ", byte);
            if j == 7 {
                print!(" ");
            }
        }
        println!();
        if i * 16 >= 96 {
            break;
        }
    }
    println!();

    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields successfully", fields.len());
            for (i, field) in fields.iter().enumerate() {
                println!("  Field {}: GDT={}, DRT={}, grid={}, values={}",
                    i, field.gdt_template, field.drt_template, field.grid.template,
                    match &field.values {
                        gribtract::GridValues::Dense(v) => v.len(),
                        gribtract::GridValues::Masked { values, .. } => values.len(),
                    }
                );
            }
        }
        Err(e) => {
            println!("✗ Decode error: {:?}", e);
            println!();

            // Try to provide more context about where the error occurs
            match &e {
                Error::TooShort { needed, got } => {
                    println!("Buffer underrun details:");
                    println!("  Bytes needed: {}", needed);
                    println!("  Bytes available: {}", got);
                    println!("  Shortfall: {}", needed - got);
                }
                _ => {
                    println!("Error type (not TooShort): {:?}", e);
                }
            }
        }
    }
}
