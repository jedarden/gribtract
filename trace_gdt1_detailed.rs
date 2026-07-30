use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    // Section 3 starts at position 37 (0x25)
    let sec3_start = 37;
    let sec3_len = u32::from_be_bytes([bytes[sec3_start], bytes[sec3_start+1], bytes[sec3_start+2], bytes[sec3_start+3]]) as usize;
    let sec3_num = bytes[sec3_start + 4];
    let body_start = sec3_start + 5;
    let body_len = sec3_len - 5;

    println!("Section 3 Analysis:");
    println!("  Total length: {} bytes", sec3_len);
    println!("  Section number: {}", sec3_num);
    println!("  Body length: {} bytes", body_len);
    println!("  Expected GDT 3.1 template: 72 bytes");
    println!("  Missing bytes: {}", 72 - body_len);

    let body = &bytes[body_start..body_start + body_len];

    println!("\nGDT 3.1 Field-by-field parsing:");
    println!("  Expected octets vs actual availability:");

    let mut pos = 0;

    // oct 15: shape_of_earth (1 byte)
    if pos < body_len {
        println!("  oct 15  (pos {:2}): shape_of_earth = 0x{:02x} ✓", pos, body[pos]);
        pos += 1;
    } else {
        println!("  oct 15  (pos {:2}): FAIL - need 1 byte, have 0", pos);
        return;
    }

    // oct 16-20: earth radius scale + value (5 bytes)
    println!("  oct 16-20 (pos {:2}): earth radius (5 bytes) {}", pos, if pos + 5 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 5;

    // oct 21-25: major axis (5 bytes)
    println!("  oct 21-25 (pos {:2}): major axis (5 bytes) {}", pos, if pos + 5 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 5;

    // oct 26-30: minor axis (5 bytes)
    println!("  oct 26-30 (pos {:2}): minor axis (5 bytes) {}", pos, if pos + 5 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 5;

    // oct 31-34: Nx (4 bytes)
    if pos + 4 <= body_len {
        let nx = u32::from_be_bytes([body[pos], body[pos+1], body[pos+2], body[pos+3]]);
        println!("  oct 31-34 (pos {:2}): Nx = {} ✓", pos, nx);
        pos += 4;
    } else {
        println!("  oct 31-34 (pos {:2}): FAIL - need 4 bytes, have {}", pos, body_len.saturating_sub(pos));
        return;
    }

    // oct 35-38: Ny (4 bytes)
    if pos + 4 <= body_len {
        let ny = u32::from_be_bytes([body[pos], body[pos+1], body[pos+2], body[pos+3]]);
        println!("  oct 35-38 (pos {:2}): Ny = {} ✓", pos, ny);
        pos += 4;
    } else {
        println!("  oct 35-38 (pos {:2}): FAIL - need 4 bytes, have {}", pos, body_len.saturating_sub(pos));
        return;
    }

    // oct 39-42: La1 (4 bytes)
    println!("  oct 39-42 (pos {:2}): La1 (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 43-46: Lo1 (4 bytes)
    println!("  oct 43-46 (pos {:2}): Lo1 (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 47: resolution_flags (1 byte)
    println!("  oct 47    (pos {:2}): resolution_flags (1 byte) {}", pos, if pos + 1 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 1;

    // oct 48-51: La2 (4 bytes)
    println!("  oct 48-51 (pos {:2}): La2 (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 52-55: Lo2 (4 bytes)
    println!("  oct 52-55 (pos {:2}): Lo2 (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 56-59: Di (4 bytes)
    println!("  oct 56-59 (pos {:2}): Di (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 60-63: Dj (4 bytes)
    println!("  oct 60-63 (pos {:2}): Dj (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 64-67: lat_pole_rot (4 bytes)
    println!("  oct 64-67 (pos {:2}): lat_pole_rot (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 68-71: lon_pole_rot (4 bytes)
    println!("  oct 68-71 (pos {:2}): lon_pole_rot (4 bytes) {}", pos, if pos + 4 <= body_len { "✓" } else { "✗ SHORT" });
    pos += 4;

    // oct 72: angle_rot (1 byte)
    println!("  oct 72    (pos {:2}): angle_rot (1 byte) {}", pos, if pos + 1 <= body_len { "✓" } else { "✗ SHORT - THIS IS THE UNDERRUN" });
    pos += 1;

    // oct 73: scanning_mode (1 byte)
    println!("  oct 73    (pos {:2}): scanning_mode (1 byte) {}", pos, if pos + 1 <= body_len { "✓" } else { "✗ SHORT OR THIS" });

    println!("\nFinal position: {}", pos);
    println!("Body length: {}", body_len);
    println!("Remaining: {}", body_len.saturating_sub(pos));
}
