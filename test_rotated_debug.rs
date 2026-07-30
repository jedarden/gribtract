use std::fs;
use gribtract::{decode, GridValues};

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());
    println!("First 100 bytes (hex): {:02x?}", &bytes[..100.min(bytes.len())]);

    match decode(&bytes) {
        Ok(fields) => {
            println!("Decoded {} fields successfully", fields.len());
            for (i, field) in fields.iter().enumerate() {
                println!("Field {}: DRT={}, grid={}, values={}",
                    i, field.drt_template, field.grid.template,
                    match &field.values {
                        GridValues::Dense(v) => v.len(),
                        GridValues::Masked { values, present: _ } => values.len(),
                    }
                );
            }
        }
        Err(e) => {
            println!("Decode error: {:?}", e);
        }
    }
}
