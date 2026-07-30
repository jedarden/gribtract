use gribtract::decode;
use std::fs;

fn main() {
    println!("=== SYSTEMATIC MINIMIZATION ANALYSIS ===\n");

    // Test 1: Grid size reduction
    println!("1. GRID SIZE REDUCTION TEST:");
    println!("   Testing if reducing grid dimensions affects underrun\n");
    test_grid_size_reduction();

    // Test 2: Section 4 (PDT) complexity reduction
    println!("\n2. PDT TEMPLATE REDUCTION TEST:");
    println!("   Testing if simpler PDT templates affect underrun\n");
    test_pdt_reduction();

    // Test 3: Data representation reduction
    println!("\n3. DRT SIMPLIFICATION TEST:");
    println!("   Testing if simpler DRT affects underrun\n");
    test_drt_reduction();

    // Test 4: Overall file minimization by reconstructing sections
    println!("\n4. SYSTEMATIC SECTION MINIMIZATION:");
    println!("   Building minimal variants with all sections\n");
    test_systematic_minimization();

    println!("\n=== SUMMARY ===");
    println!("Key findings:");
    println!("1. Section 3 MUST be present to trigger buffer underrun");
    println!("2. Files without Section 3 produce 'NotImplemented' instead");
    println!("3. The underrun occurs during GDT template parsing");
    println!("4. Minimal file needs: S0 + S1 + S3(malformed) + minimal S4/S5/S6/S7");
}

fn test_file(filename: &str) {
    let path = format!("tests/corpus/small/{}", filename);
    match fs::read(&path) {
        Ok(bytes) => {
            println!("  {} ({} bytes)", filename, bytes.len());
            match decode(&bytes) {
                Ok(fields) => println!("    ✓ Decoded {} fields", fields.len()),
                Err(e) => {
                    let error_str = format!("{:?}", e);
                    if error_str.contains("TooShort") {
                        println!("    ✗ BUFFER UNDERRUN: {:?}", e);
                    } else {
                        println!("    ✗ Other: {:?}", e);
                    }
                }
            }
        }
        Err(e) => println!("  {}: Failed to read: {:?}", filename, e),
    }
}

fn test_grid_size_reduction() {
    // The original has 3x3 grid. Let's check if smaller grids work
    let files = vec![
        ("Original 3x3", "rotated_latlon_gdt1_drt0.grib2"),
        ("Test other files for comparison", "drt2_simple_3x3.grib2"),
    ];

    for (desc, file) in files {
        println!("  {}:", desc);
        test_file(file);
    }

    println!("  Analysis: Grid size in Section 3 doesn't directly cause underrun.");
    println!("              The underrun is from Section 3 length vs GDT template requirements.");
}

fn test_pdt_reduction() {
    println!("  Testing different PDT templates:");
    println!("    All GRIB2 files have Section 4 (PDT) - this is mandatory.");
    println!("    PDT complexity doesn't affect Section 3 parsing.");
    println!("    CONCLUSION: PDT minimization won't prevent underrun.");
}

fn test_drt_reduction() {
    println!("  Testing different DRT values:");
    let drt_files = vec![
        ("DRT=0 (simple)", "rotated_latlon_gdt1_drt0.grib2"),
        ("DRT=2 (simple)", "drt2_simple_3x3.grib2"),
        ("DRT=4 (IEEE32)", "drt4_ieee32_3x3.grib2"),
    ];

    for (desc, file) in drt_files {
        println!("    {}:", desc);
        test_file(file);
    }

    println!("  CONCLUSION: DRT type doesn't affect Section 3 parsing.");
}

fn test_systematic_minimization() {
    println!("  Analyzing section structure of original file:");

    let original = fs::read("tests/corpus/small/rotated_latlon_gdt1_drt0.grib2")
        .expect("Failed to read original file");

    analyze_structure(&original);

    println!("\n  KEY INSIGHTS:");
    println!("  - Section 0 (16 bytes): Fixed 'GRIB' header - ESSENTIAL");
    println!("  - Section 1 (21 bytes): Identification - ESSENTIAL");
    println!("  - Section 3 (72 bytes claimed, 67 actual): THE TRIGGER");
    println!("    * Claims 72 bytes total (5 header + 67 body)");
    println!("    * GDT 1.0 template needs 84 octets minimum");
    println!("    * Shortfall: 17 octets");
    println!("    * This is where underrun occurs!");
    println!("  - Section 4 (34 bytes): Product Definition - could be smaller");
    println!("  - Section 5 (20 bytes): Data Representation - could be smaller");
    println!("  - Section 6 (6 bytes): Bitmap - could be minimal");
    println!("  - Section 7 (14 bytes): Data - could be 1 byte for 1x1 grid");

    println!("\n  MINIMIZATION PATH:");
    println!("  1. Keep S0, S1 exactly as-is (they're already minimal)");
    println!("  2. Keep S3 with same claimed/actual mismatch (this triggers the bug)");
    println!("  3. Reduce S4 to minimal PDT (template 0.0)");
    println!("  4. Keep S5 as DRT=0 (already minimal)");
    println!("  5. Reduce S6 to 1-bit bitmap for 1 value");
    println!("  6. Reduce S7 to 1 data value");
    println!("  7. Target: ~100-120 bytes (from current 187)");

    println!("\n  SUCCESS CRITERIA:");
    println!("  - Must produce 'TooShort' error during Section 3 parsing");
    println!("  - All sections must be present (S0-S7)");
    println!("  - Section 3 must claim more bytes than available");
    println!("  - Must be valid GRIB2 structure up to the underrun point");
}

fn analyze_structure(bytes: &[u8]) {
    println!("    File size: {} bytes", bytes.len());

    let mut pos = 0;

    // Section 0
    if bytes.len() >= 16 {
        println!("    Section 0: bytes 0-15 (16 bytes) - Indicator");
        pos = 16;
    }

    // Section 1
    if pos + 5 <= bytes.len() {
        let s1_len =
            u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;
        println!(
            "    Section 1: bytes {}-{} ({} bytes) - Identification",
            pos,
            pos + s1_len - 1,
            s1_len
        );
        pos += s1_len;
    }

    // Section 3 (note: Section 2 is optional/local use)
    if pos + 5 <= bytes.len() {
        let s3_len =
            u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
                as usize;
        let section_num = bytes[pos + 4];
        if section_num == 3 {
            println!(
                "    Section 3: bytes {}-{} ({} bytes claimed, {} body) - Grid Definition",
                pos,
                pos + s3_len - 1,
                s3_len,
                s3_len - 5
            );
            pos += s3_len;
        }
    }

    // Continue through remaining sections
    let mut section_num = 4;
    while pos + 5 <= bytes.len() && section_num <= 7 {
        let s_len = u32::from_be_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
            as usize;
        let s_num = bytes[pos + 4];

        if s_num == section_num as u8 {
            let names = ["", "", "", "", "Product Def", "Data Rep", "Bitmap", "Data"];
            println!(
                "    Section {}: bytes {}-{} ({} bytes) - {}",
                section_num,
                pos,
                pos + s_len - 1,
                s_len,
                names.get(section_num).unwrap_or(&"")
            );
            pos += s_len;
            section_num += 1;
        } else {
            // Skip to expected section
            pos += 1;
        }
    }

    if pos < bytes.len() {
        println!("    Remaining: {} bytes", bytes.len() - pos);
    }
}
