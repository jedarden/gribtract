use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());

    // Parse Section 0
    if bytes.len() >= 16 {
        println!("Section 0 (Indicator):");
        println!("  Magic: {:02x?}", &bytes[0..4]);
        println!("  Total length (discipline + edition + length): {} bytes",
            u64::from_be_bytes([
                bytes[8], bytes[9], bytes[10], bytes[11],
                bytes[12], bytes[13], bytes[14], bytes[15]
            ]));
        println!("  Discipline: {}", bytes[7]);
        println!("  Edition: {}", bytes[16 - 4]); // This is wrong, let me recalculate
    }

    // Correct parsing
    println!("\nCorrect Section 0 parsing:");
    println!("  Magic: {}", std::str::from_utf8(&bytes[0..4]).unwrap());
    println!("  Reserved: {:02x?}", &bytes[4..6]);
    println!("  Discipline: {}", bytes[6]);
    println!("  Edition: {}", bytes[7]);
    println!("  Total length: {} (big-endian u64)",
        u64::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]]));
    println!("  Message body should end at byte {}", 16 + bytes.len() as u64 - 4);
}
