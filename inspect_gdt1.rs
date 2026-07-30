use std::fs;
use gribtract::decode;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());
    println!("Full file hex:");
    for (i, b) in bytes.iter().enumerate() {
        print!("{:02x} ", b);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
}
