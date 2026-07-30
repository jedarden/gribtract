use std::fs;

fn main() {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path).expect("Failed to read fixture");

    println!("Full GRIB2 Section Trace:");
    println!("File size: {} bytes (0x{:02x})\n", bytes.len(), bytes.len());

    // Parse Section 0
    println!("Section 0 (Indicator):");
    println!("  Magic: {:02x} {:02x} {:02x} {:02x}", bytes[0], bytes[1], bytes[2], bytes[3]);
    let total_len = u64::from_be_bytes([
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11]
    ]) as usize;
    let discipline = bytes[12];
    let edition = bytes[13];
    println!("  Total length: {} bytes", total_len);
    println!("  Discipline: {}", discipline);
    println!("  Edition: {}", edition);

    // Section iteration starts at position 16
    let mut pos = 16;
    let body_end = total_len - 4; // Before "7777" marker

    println!("\nSection iteration (body ends at position {}):", body_end);
    println!("{:>6} | {:>8} | {:>6} | {:>10} | {:>8}", "Sec#", "Position", "Length", "Body Start", "Body Len");
    println!("{:-^6}-|-{:->8}-|-{:->6}-|-{:->10}-|-{:->8}", "------", "--------", "------", "----------", "--------");

    let mut section_count = 0;

    while pos < body_end {
        let sec_start = pos;
        let sec_len = u32::from_be_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]) as usize;
        let sec_num = bytes[pos + 4];

        let body_start = pos + 5;
        let body_len = sec_len - 5;

        println!("{:>6} | {:>8} | {:>6} | {:>10} | {:>8}",
                 sec_num, sec_start, sec_len, body_start, body_len);

        // Check if reading this section would go beyond the message
        if sec_start + sec_len > body_end {
            println!("  ⚠️  WARNING: Section extends beyond body_end!");
            println!("     sec_start + sec_len = {} > body_end = {}", sec_start + sec_len, body_end);
        }

        // Check if we have enough bytes to read the next section header
        let next_pos = sec_start + sec_len;
        if next_pos + 5 > bytes.len() && next_pos < body_end {
            println!("  ⚠️  WARNING: Next section header would read beyond file!");
            println!("     next_pos + 5 = {} > file_len = {}", next_pos + 5, bytes.len());
        }

        pos = next_pos;
        section_count += 1;

        if section_count > 10 {
            println!("  ... too many sections, stopping");
            break;
        }
    }

    println!("\nEnd marker check:");
    println!("  Expected '7777' at position {}:", body_end);
    if body_end + 4 <= bytes.len() {
        println!("    Bytes: {:02x} {:02x} {:02x} {:02x}",
                 bytes[body_end], bytes[body_end+1],
                 bytes[body_end+2], bytes[body_end+3]);
    } else {
        println!("    ERROR: Not enough bytes for end marker!");
        println!("    body_end + 4 = {} > file_len = {}", body_end + 4, bytes.len());
    }

    println!("\nBuffer analysis:");
    println!("  File length: {}", bytes.len());
    println!("  Total message length: {}", total_len);
    println!("  Position after last section: {}", pos);
    println!("  Bytes remaining: {}", bytes.len().saturating_sub(pos));
}
