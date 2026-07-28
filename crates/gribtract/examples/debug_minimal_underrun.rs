use std::fs;
use gribtract::decode;

fn main() {
    println!("=== DEBUGGING BUFFER UNDERRUN ===\n");

    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");
    println!("File size: {} bytes\n", bytes.len());

    // Check the end of the file
    println!("=== Last 20 bytes of file ===");
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = bytes.len() - ((bytes.len() / 16 - i) * 16);
        if offset >= bytes.len() - 20 {
            print!("{:04x}: ", offset);
            for (j, byte) in chunk.iter().enumerate() {
                if offset + j >= bytes.len() {
                    print!("   ");
                } else {
                    print!("{:02x} ", byte);
                }
                if j == 7 {
                    print!(" ");
                }
            }
            println!();
        }
    }
    println!();

    match decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields", fields.len());
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
            println!();
            println!("This error indicates a buffer underrun.");
            println!("The decoder tried to read {} byte(s) but only {} were available.",
                match &e {
                    gribtract::Error::TooShort { needed, .. } => needed,
                    _ => &0
                },
                match &e {
                    gribtract::Error::TooShort { got, .. } => got,
                    _ => &0
                }
            );
        }
    }
}
