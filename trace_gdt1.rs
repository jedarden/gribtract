use std::fs;
use gribtract::decode;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());
    println!("Hex dump of entire file:");
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);
        for (j, &b) in chunk.iter().enumerate() {
            print!("{:02x} ", b);
            if j == 7 { print!(" "); }
        }
        println!();
    }

    println!("\nAttempting decode...");
    match decode(&bytes) {
        Ok(fields) => {
            println!("Decoded {} fields successfully", fields.len());
        }
        Err(e) => {
            println!("Decode error: {:?}", e);
            println!("Error message: {}", e);
        }
    }
}
