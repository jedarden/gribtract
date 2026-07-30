use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("File size: {} bytes", bytes.len());

    // Parse Section 0
    println!("\n=== Section 0 (Indicator) ===");
    println!("Magic: {} (bytes 0-3)", std::str::from_utf8(&bytes[0..4]).unwrap());
    let total_len = u64::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]]);
    println!("Total length: {} bytes", total_len);

    let mut pos = 16usize;

    // Parse Section 1
    println!("\n=== Section 1 ===");
    let sec1_len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    let sec1_num = bytes[pos + 4];
    println!("Section 1: length={}, number={}", sec1_len, sec1_num);
    pos += sec1_len;
    println!("After Section 1, pos = {}", pos);

    // Parse Section 3
    println!("\n=== Section 3 (Grid Definition) ===");
    let sec3_len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    let sec3_num = bytes[pos + 4];
    println!("Section 3: length={}, number={}", sec3_len, sec3_num);
    println!("Section 3 data starts at byte {}, ends at byte {}", pos + 5, pos + sec3_len);

    let sec3_data_start = pos + 5;
    let sec3_data_len = sec3_len - 5;

    println!("\nSection 3 data ({} bytes):", sec3_data_len);
    for i in 0..sec3_data_len {
        print!("{:02x} ", bytes[sec3_data_start + i]);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();

    // Parse GDT 3.1 fields
    println!("\n=== GDT 3.1 Field Parsing ===");
    let mut b = sec3_data_start + 5; // Skip octets 6-14 (source, num_points, optional list fields, template number)

    println!("oct 15 (shape_of_earth): {:02x} = {}", bytes[b], bytes[b]);
    b += 1;

    println!("oct 16-20 (earth radius): {:02x} {:02x} {:02x} {:02x} {:02x} (skipped)",
        bytes[b], bytes[b+1], bytes[b+2], bytes[b+3], bytes[b+4]);
    b += 5;

    println!("oct 21-25 (major axis): {:02x} {:02x} {:02x} {:02x} {:02x} (skipped)",
        bytes[b], bytes[b+1], bytes[b+2], bytes[b+3], bytes[b+4]);
    b += 5;

    println!("oct 26-30 (minor axis): {:02x} {:02x} {:02x} {:02x} {:02x} (skipped)",
        bytes[b], bytes[b+1], bytes[b+2], bytes[b+3], bytes[b+4]);
    b += 5;

    let nx = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 31-34 (Nx): {:02x} {:02x} {:02x} {:02x} = {}", bytes[b], bytes[b+1], bytes[b+2], bytes[b+3], nx);
    b += 4;

    let ny = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 35-38 (Ny): {:02x} {:02x} {:02x} {:02x} = {}", bytes[b], bytes[b+1], bytes[b+2], bytes[b+3], ny);
    b += 4;

    // Latitude La1 (signed)
    let la1_raw = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    let la1_mag = (la1_raw & 0x7FFF_FFFF) as i64;
    let la1 = if la1_raw & 0x8000_0000 != 0 { -la1_mag } else { la1_mag };
    println!("oct 39-42 (La1): raw={:08x}, signed={} microdeg = {}°", la1_raw, la1, la1 as f64 / 1_000_000.0);
    b += 4;

    // Longitude Lo1 (unsigned)
    let lo1 = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 43-46 (Lo1): raw={:08x} = {} microdeg = {}°", lo1, lo1, lo1 as f64 / 1_000_000.0);
    b += 4;

    println!("oct 47 (resolution_flags): {:02x}", bytes[b]);
    b += 1;

    // Latitude La2 (signed)
    let la2_raw = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    let la2_mag = (la2_raw & 0x7FFF_FFFF) as i64;
    let la2 = if la2_raw & 0x8000_0000 != 0 { -la2_mag } else { la2_mag };
    println!("oct 48-51 (La2): raw={:08x}, signed={} microdeg = {}°", la2_raw, la2, la2 as f64 / 1_000_000.0);
    b += 4;

    // Longitude Lo2 (unsigned)
    let lo2 = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 52-55 (Lo2): raw={:08x} = {} microdeg = {}°", lo2, lo2, lo2 as f64 / 1_000_000.0);
    b += 4;

    let di = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 56-59 (Di): raw={:08x} = {} microdeg = {}°", di, di, di as f64 / 1_000_000.0);
    b += 4;

    let dj = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 60-63 (Dj): raw={:08x} = {} microdeg = {}°", dj, dj, dj as f64 / 1_000_000.0);
    b += 4;

    // Latitude of southern pole (signed)
    let lat_pole_raw = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    let lat_pole_mag = (lat_pole_raw & 0x7FFF_FFFF) as i64;
    let lat_pole = if lat_pole_raw & 0x8000_0000 != 0 { -lat_pole_mag } else { lat_pole_mag };
    println!("oct 64-67 (lat_pole_rot): raw={:08x}, signed={} microdeg = {}°", lat_pole_raw, lat_pole, lat_pole as f64 / 1_000_000.0);
    b += 4;

    // Longitude of southern pole (unsigned)
    let lon_pole = u32::from_be_bytes([bytes[b], bytes[b+1], bytes[b+2], bytes[b+3]]);
    println!("oct 68-71 (lon_pole_rot): raw={:08x} = {} microdeg = {}°", lon_pole, lon_pole, lon_pole as f64 / 1_000_000.0);
    b += 4;

    println!("\n=== Checking remaining bytes ===");
    println!("Current position b = {} (relative to start of Section 3 data)", b - sec3_data_start);
    println!("Section 3 data length = {}", sec3_data_len);
    println!("Bytes remaining in Section 3 data: {}", sec3_data_len - (b - sec3_data_start));

    if b < sec3_data_start + sec3_data_len {
        println!("oct 72 (angle_rot): {:02x} = {} (raw)", bytes[b], bytes[b]);
    } else {
        println!("oct 72 (angle_rot): NOT AVAILABLE - buffer underrun!");
    }

    println!("\n=== Expected vs Actual ===");
    println!("Expected oct 72 to be at byte: {}", sec3_data_start + 72 - 1);
    println!("Section 3 data ends at byte: {}", sec3_data_start + sec3_data_len - 1);
    println!("Section 3 length: {} (should be {} for GDT 3.1)", sec3_len, sec3_len + (72 - (b - sec3_data_start)));

    println!("\n=== Complete File Layout ===");
    println!("Section 0: bytes 0-15 (16 bytes)");
    println!("Section 1: bytes 16-36 (21 bytes)");
    println!("Section 3: bytes 37-87 (51 bytes)");
    println!("Section 4: bytes 88-");
    pos = 88;
    let sec4_len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
    let sec4_num = bytes[pos + 4];
    println!("Section 4: bytes {}-{} ({} bytes)", pos, pos + sec4_len - 1, sec4_len);
    println!("Section 4 number: {}", sec4_num);
}
