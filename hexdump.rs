use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());
    println!("\nHex dump (16 bytes per row):");
    println!("Offset  Hex                                              ASCII");
    println!("------ ----------------------------------------------- ----------------");

    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        print!("{:04x}  ", offset);

        // Hex part
        for (j, byte) in chunk.iter().enumerate() {
            print!("{:02x} ", byte);
            if j == 7 {
                print!(" ");
            }
        }

        // Padding for incomplete lines
        for j in chunk.len()..16 {
            print!("   ");
            if j == 7 {
                print!(" ");
            }
        }

        print!(" ");

        // ASCII part
        for &byte in chunk {
            let c = if byte >= 32 && byte <= 126 { byte as char } else { '.' };
            print!("{}", c);
        }

        println!();
    }

    println!("\n=== Section header parsing ===");

    if bytes.len() >= 16 {
        println!("Section 0 (Indicator):");
        println!("  Bytes 0-3: {:.4}", String::from_utf8_lossy(&bytes[0..4]));
        println!("  Reserved: {}", bytes[4]);
        println!("  Discipline: {}", bytes[5]);
        println!("  Edition: {}", bytes[7]);
    }

    if bytes.len() >= 21 {
        println!("\nSection 1 (Identification):");
        println!("  Start: 16");
        let sec1_len = u64::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19], 0, 0, 0, 0]);
        println!("  Length (octets 16-19): {} (0x{:08x})", sec1_len, sec1_len);
        println!("  Section number: {}", bytes[20]);
    }

    if bytes.len() >= 26 {
        println!("\nSection 3 (Grid Definition):");
        let sec1_len = u64::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19], 0, 0, 0, 0]);
        println!("  Start: {}", 16 + sec1_len as usize);
        let sec3_pos = 16 + sec1_len as usize;
        if bytes.len() >= sec3_pos + 5 {
            let sec3_len = u64::from_be_bytes([bytes[sec3_pos], bytes[sec3_pos+1], bytes[sec3_pos+2], bytes[sec3_pos+3], 0, 0, 0, 0]);
            println!("  Length: {} (0x{:08x})", sec3_len, sec3_len);
            println!("  Section number: {}", bytes[sec3_pos + 4]);
        }
    }
}
