use std::fs;
use gribtract::decode;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());

    match decode(&bytes) {
        Ok(fields) => {
            println!("Decoded {} fields successfully", fields.len());
            for (i, field) in fields.iter().enumerate() {
                println!("Field {}: GDT={}, DRT={}, grid={}, values={}",
                    i, field.gdt_template, field.drt_template, field.grid.template,
                    match &field.values {
                        gribtract::GridValues::Dense(v) => v.len(),
                        gribtract::GridValues::Masked { values, .. } => values.len(),
                    }
                );
            }
        }
        Err(e) => {
            println!("Decode error: {:?}", e);
        }
    }
}
