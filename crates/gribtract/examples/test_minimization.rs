use std::fs;
use gribtract::decode;

fn main() {
    println!("=== MINIMIZATION ANALYSIS ===\n");

    // Test original
    println!("1. ORIGINAL TEST:");
    test_file("rotated_latlon_gdt1_drt0.grib2");

    // Test existing minimal variants
    println!("\n2. EXISTING MINIMAL VARIANTS:");
    test_file("minimal_underrun.grib2");
    test_file("minimal_underrun_2bytes.grib2");

    // Test other small files to see which ones fail
    println!("\n3. OTHER SMALL FILES:");
    let small_files = vec![
        "drt2_simple_3x3.grib2",
        "drt4_ieee32_3x3.grib2",
        "drt4_ieee_float_3x3.grib2",
        "gfs_anl_t2m_5x5.grib2",
        "conus_drt0.grib2",
    ];

    for file in small_files {
        test_file(file);
    }
}

fn test_file(filename: &str) {
    let path = format!("tests/corpus/small/{}", filename);
    match fs::read(&path) {
        Ok(bytes) => {
            println!("  {} ({} bytes)", filename, bytes.len());
            match decode(&bytes) {
                Ok(fields) => println!("    ✓ Decoded {} fields", fields.len()),
                Err(e) => println!("    ✗ Error: {:?}", e)
            }
        }
        Err(e) => println!("  {}: Failed to read: {:?}", filename, e)
    }
}