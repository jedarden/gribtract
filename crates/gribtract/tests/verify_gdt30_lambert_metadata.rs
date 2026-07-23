//! Verify GDT 3.30 (Lambert Conformal) grid metadata is correctly populated.
//!
//! This test validates that the parse_gdt_30 function in decode.rs correctly
//! extracts all Lambert Conformal projection parameters from the GRIB2 grid
//! definition section and populates the GridDefinition struct.
//!
//! Related bead: bf-ufdir

use gribtract_core::decode::decode_bytes;

#[test]
fn verify_lambert_gdt30_metadata_population() {
    let corpus_root = {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir)
            .join("../../tests/corpus")
    };

    let fixture_path = corpus_root.join("large/nam.t00z.awip1200.tm00.grib2");
    if !fixture_path.exists() {
        println!("Skipping test: fixture not found at {:?}", fixture_path);
        return;
    }

    let bytes = match std::fs::read(&fixture_path) {
        Ok(b) => b,
        Err(e) => {
            println!("Skipping test: failed to read fixture: {}", e);
            return;
        }
    };

    // Decode the fixture
    let fields = match decode_bytes(&bytes) {
        Ok(f) => f,
        Err(e) => {
            panic!("Decode failed: {}", e);
        }
    };

    // All fields should have the same grid definition
    let first_grid = &fields[0].grid;

    // Verify grid template is GDT 3.30 (Lambert Conformal)
    assert_eq!(
        first_grid.template, 30,
        "Grid template should be 30 (Lambert Conformal)"
    );

    // Verify grid dimensions from NCEP Grid 218
    assert_eq!(first_grid.nx, 614, "Nx should be 614");
    assert_eq!(first_grid.ny, 428, "Ny should be 428");
    assert_eq!(first_grid.num_data_points, 262792, "Total points should be 262,792");

    // Verify Lambert Conformal projection parameters
    let params = match &first_grid.projection {
        gribtract_core::types::GridProjection::LambertConformal(p) => p,
        _ => panic!("Expected Lambert Conformal projection"),
    };

    // These values are verified against wgrib2 output:
    // wgrib2 -grid tests/corpus/large/nam.t00z.awip1200.tm00.grib2
    //
    // Lambert Conformal: (614 x 428) input WE:SN output WE:SN res 56
    // Lat1 12.190000 Lon1 226.541000 LoV 265.000000
    // LatD 25.000000 Latin1 25.000000 Latin2 25.000000
    // Dx 12191.000000 m Dy 12191.000000 m

    // LaD (latitude where Dx/Dy are specified) - 25° N
    assert_eq!(
        params.lad, 25.0,
        "LaD (latitude of Dx/Dy) should be 25.0° N"
    );

    // LoV (orientation/central meridian) - 265° E
    assert_eq!(
        params.lov, 265.0,
        "LoV (central meridian) should be 265.0° E"
    );

    // Dx (grid spacing in x-direction) - 12191 meters
    assert_eq!(
        params.dx_m, 12191.0,
        "Dx should be 12191.0 meters"
    );

    // Dy (grid spacing in y-direction) - 12191 meters
    assert_eq!(
        params.dy_m, 12191.0,
        "Dy should be 12191.0 meters"
    );

    // Latin1 (first standard parallel) - 25° N (tangent cone)
    assert_eq!(
        params.latin1, 25.0,
        "Latin1 (first standard parallel) should be 25.0° N"
    );

    // Latin2 (second standard parallel) - 25° N (same as Latin1 = tangent cone)
    assert_eq!(
        params.latin2, 25.0,
        "Latin2 (second standard parallel) should be 25.0° N"
    );

    // Projection centre flag - 0 (North Pole in plane)
    // This is the standard for NCEP grids
    assert_eq!(
        params.proj_centre, 0,
        "Projection centre flag should be 0 (North Pole)"
    );

    // South pole of projection - -90°, 0°E (standard non-rotated)
    assert_eq!(
        params.lat_south_pole, -90.0,
        "South pole latitude should be -90.0° (standard)"
    );
    assert_eq!(
        params.lon_south_pole, 0.0,
        "South pole longitude should be 0.0° (standard)"
    );

    // Verify first grid point coordinates
    // From wgrib2: Lat1 12.190000 Lon1 226.541000
    assert!((first_grid.lat_first - 12.19).abs() < 0.001,
        "First latitude should be ~12.19° N, got {}", first_grid.lat_first);
    assert!((first_grid.lon_first - 226.541).abs() < 0.001,
        "First longitude should be ~226.541° E, got {}", first_grid.lon_first);

    // Verify scanning mode - should be 0b01000000 (0x40)
    // Bit 7 (0x80): +i direction (west to east) - 0 = +i
    // Bit 6 (0x40): +j direction (south to north) - 1 = +j
    // wgrib2 shows "input WE:SN" which means +i (west to east), +j (south to north)
    assert_eq!(
        first_grid.scanning_mode, 0x40,
        "Scanning mode should be 0x40 (+i, +j)"
    );

    // Resolution and component flags
    assert_eq!(
        first_grid.resolution_flags, 56,
        "Resolution flags should be 56 (0x38)"
    );

    // Shape of the Earth - 6 = WMO standard sphere (6371229 m)
    assert_eq!(
        first_grid.shape_of_earth, 6,
        "Shape of Earth should be 6 (WMO standard sphere)"
    );

    println!("✅ GDT 3.30 (Lambert Conformal) grid metadata correctly populated:");
    println!("   Grid dimensions: {}×{} ({} points)",
        first_grid.nx, first_grid.ny, first_grid.num_data_points);
    println!("   First point: ({:.3}° N, {:.3}° E)",
        first_grid.lat_first, first_grid.lon_first);
    println!("   LaD (latitude of Dx/Dy): {}°", params.lad);
    println!("   LoV (central meridian): {}°", params.lov);
    println!("   Dx/Dy (grid spacing): {:.1} km / {:.1} km",
        params.dx_m / 1000.0, params.dy_m / 1000.0);
    println!("   Latin1/Latin2 (standard parallels): {}° / {}°",
        params.latin1, params.latin2);
    println!("   Projection centre: {}", params.proj_centre);
    println!("   South pole: ({:.1}°, {:.1}°)",
        params.lat_south_pole, params.lon_south_pole);
    println!("   Scanning mode: 0x{:02x}", first_grid.scanning_mode);
    println!("   Shape of Earth: {}", first_grid.shape_of_earth);

    // Verify all fields have the same grid metadata
    for (i, field) in fields.iter().enumerate().skip(1) {
        assert_eq!(field.grid.template, first_grid.template,
            "Field {}: GDT template mismatch", i);
        assert_eq!(field.grid.nx, first_grid.nx,
            "Field {}: Nx mismatch", i);
        assert_eq!(field.grid.ny, first_grid.ny,
            "Field {}: Ny mismatch", i);
        assert_eq!(field.grid.num_data_points, first_grid.num_data_points,
            "Field {}: point count mismatch", i);

        let field_params = match &field.grid.projection {
            gribtract_core::types::GridProjection::LambertConformal(p) => p,
            _ => panic!("Field {}: Expected Lambert Conformal", i),
        };

        assert!((field_params.lad - params.lad).abs() < 1e-9,
            "Field {}: LaD mismatch", i);
        assert!((field_params.lov - params.lov).abs() < 1e-9,
            "Field {}: LoV mismatch", i);
        assert!((field_params.dx_m - params.dx_m).abs() < 1e-6,
            "Field {}: Dx mismatch", i);
        assert!((field_params.dy_m - params.dy_m).abs() < 1e-6,
            "Field {}: Dy mismatch", i);
    }

    println!("✅ All {} fields have consistent Lambert 3.30 grid metadata", fields.len());
}
