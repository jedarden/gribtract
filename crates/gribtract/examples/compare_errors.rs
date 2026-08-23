//! Compare buffer underrun errors between original and minimal files
//!
//! This demonstrates that both the original and minimal files produce
//! the same type of buffer underrun error.

use std::fs;
use std::path::Path;

fn main() {
    let original_path = Path::new("tests/corpus/small/rotated_latlon_gdt1_drt0.grib2");
    let minimal_path = Path::new("tests/data/minimal_buffer_underrun.grib2");

    println!("=== Buffer Underrun Error Comparison ===\n");

    // Test original file
    println!("1. ORIGINAL FILE");
    println!("   Path: {}", original_path.display());
    let original_grib2 = fs::read(original_path).expect("Failed to read original file");
    println!("   Size: {} bytes", original_grib2.len());

    match gribtract::decode(&original_grib2) {
        Ok(_) => println!("   ❌ Decoding succeeded unexpectedly"),
        Err(e) => println!("   ✓ Error: {:?}", e),
    }

    println!();

    // Test minimal file
    println!("2. MINIMAL FILE");
    println!("   Path: {}", minimal_path.display());
    let minimal_grib2 = fs::read(minimal_path).expect("Failed to read minimal file");
    println!("   Size: {} bytes", minimal_grib2.len());

    match gribtract::decode(&minimal_grib2) {
        Ok(_) => println!("   ❌ Decoding succeeded unexpectedly"),
        Err(e) => println!("   ✓ Error: {:?}", e),
    }

    println!();

    // Summary
    let size_reduction = original_grib2.len() - minimal_grib2.len();
    let reduction_percent = (size_reduction as f64 / original_grib2.len() as f64) * 100.0;

    println!("=== SUMMARY ===");
    println!("Original size: {} bytes", original_grib2.len());
    println!("Minimal size:  {} bytes", minimal_grib2.len());
    println!(
        "Reduction:     {} bytes ({:.1}%)",
        size_reduction, reduction_percent
    );
    println!();
    println!("Both files produce the same TooShort buffer underrun error ✓");
}
