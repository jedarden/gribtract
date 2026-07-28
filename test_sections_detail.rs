use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fixture_path = "tests/corpus/small/rotated_latlon_gdt1_drt0.grib2";
    let bytes = fs::read(fixture_path)?;

    println!("=== File Overview ===");
    println!("File size: {} bytes", bytes.len());

    // Check for GRIB edition
    if bytes.len() >= 8 {
        let identifier = std::str::from_utf8(&bytes[0..4]).unwrap_or("???");
        println!("GRIB identifier: {}", identifier);
        println!("Edition: {}", bytes[7]);
    }

    println!("\n=== Section 0 (Indicator) ===");
    if bytes.len() >= 16 {
        let sec0_len = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        println!("Section 0 length: {} bytes", sec0_len);
    }

    println!("\n=== Section 1 (Identification) ===");
    if bytes.len() >= 21 {
        let sec1_len = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        println!("Section 1 starts at: 16");
        println!("Section 1 length: {} bytes", sec1_len);
    }

    println!("\n=== Section 3 (Grid Definition) ===");
    let sec3_start = 21;
    if bytes.len() > sec3_start + 4 {
        let sec3_len = u32::from_be_bytes([bytes[sec3_start], bytes[sec3_start+1], bytes[sec3_start+2], bytes[sec3_start+3]]) as usize;
        println!("Section 3 starts at: {}", sec3_start);
        println!("Section 3 length: {} bytes", sec3_len);
        println!("Section 3 ends at: {}", sec3_start + sec3_len);
    }

    println!("\n=== Section 4 (Product Definition) ===");
    let sec4_start = 21 + 72; // sec3_start + sec3_len
    if bytes.len() > sec4_start + 4 {
        let sec4_len = u32::from_be_bytes([bytes[sec4_start], bytes[sec4_start+1], bytes[sec4_start+2], bytes[sec4_start+3]]) as usize;
        println!("Section 4 starts at: {}", sec4_start);
        println!("Section 4 length: {} bytes", sec4_len);
        println!("Section 4 ends at: {}", sec4_start + sec4_len);
        println!("Remaining bytes after Section 4 header: {}", bytes.len().saturating_sub(sec4_start + 5));
    }

    println!("\n=== Section 5 (Data Representation) ===");
    let sec5_start = sec4_start + 30; // Approximate
    if bytes.len() > sec5_start + 4 {
        let sec5_len = u32::from_be_bytes([bytes[sec5_start], bytes[sec5_start+1], bytes[sec5_start+2], bytes[sec5_start+3]]) as usize;
        println!("Section 5 starts at: {}", sec5_start);
        println!("Section 5 length: {} bytes", sec5_len);
        println!("Section 5 ends at: {}", sec5_start + sec5_len);
        println!("Remaining bytes after Section 5 header: {}", bytes.len().saturating_sub(sec5_start + 5));
    }

    println!("\n=== Section 6 (Bitmap) ===");
    let sec6_start = sec5_start + 22; // Approximate
    if bytes.len() > sec6_start + 4 {
        let sec6_len = u32::from_be_bytes([bytes[sec6_start], bytes[sec6_start+1], bytes[sec6_start+2], bytes[sec6_start+3]]) as usize;
        println!("Section 6 starts at: {}", sec6_start);
        println!("Section 6 length: {} bytes", sec6_len);
        println!("Section 6 number: {}", bytes.get(sec6_start + 4).copied().unwrap_or(0));
        println!("Remaining bytes after Section 6 header: {}", bytes.len().saturating_sub(sec6_start + 5));
    }

    println!("\n=== Section 7 (Data) ===");
    let sec7_start = sec6_start + 6; // Approximate
    if bytes.len() > sec7_start + 4 {
        let sec7_len = u32::from_be_bytes([bytes[sec7_start], bytes[sec7_start+1], bytes[sec7_start+2], bytes[sec7_start+3]]) as usize;
        println!("Section 7 starts at: {}", sec7_start);
        println!("Section 7 length: {} bytes", sec7_len);
        println!("Remaining bytes after Section 7 header: {}", bytes.len().saturating_sub(sec7_start + 5));
    }

    println!("\n=== Actual decode() call ===");
    match gribtract::decode(&bytes) {
        Ok(fields) => {
            println!("✓ Decoded {} fields", fields.len());
        }
        Err(e) => {
            println!("✗ Decode error: {:?}", e);
        }
    }

    Ok(())
}
