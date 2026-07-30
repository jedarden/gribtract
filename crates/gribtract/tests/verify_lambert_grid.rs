//! Verify Lambert 3.30 grid metadata is populated correctly after DRT=3 decode fix.

use gribtract_core::decode::{decode_bytes, decode_bytes_lazy};
use gribtract_core::types::{GridDefinition, GridProjection, LambertConformalParams};

#[test]
fn verify_nam_lambert_grid_metadata() {
    let corpus_root = {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join("../../tests/corpus")
    };

    // Test the NAM Lambert DRT=3 fixture
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

    // Decode fully and check grid metadata
    let fields = match decode_bytes(&bytes) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to decode fixture: {}", e);
        }
    };

    assert!(
        !fields.is_empty(),
        "Expected at least one field in the fixture"
    );

    // Check first field's grid metadata
    let field = &fields[0];
    let grid = &field.grid;

    println!("=== Field 0 Grid Metadata ===");
    println!("Template: {}", grid.template);
    println!("Num data points: {}", grid.num_data_points);
    println!("Nx: {}", grid.nx);
    println!("Ny: {}", grid.ny);
    println!("Lat first: {}", grid.lat_first);
    println!("Lon first: {}", grid.lon_first);
    println!("Scanning mode: {}", grid.scanning_mode);
    println!("Resolution flags: {}", grid.resolution_flags);
    println!("Shape of earth: {}", grid.shape_of_earth);

    // Verify GDT 3.30
    assert_eq!(grid.template, 30, "Expected GDT 3.30 (Lambert Conformal)");

    // Verify Lambert projection parameters are present
    let lambert_params = match &grid.projection {
        GridProjection::LambertConformal(params) => {
            println!("\n=== Lambert Conformal Parameters ===");
            println!("LaD (latitude where Dx/Dy specified): {}", params.lad);
            println!("LoV (central meridian): {}", params.lov);
            println!("Dx (m): {}", params.dx_m);
            println!("Dy (m): {}", params.dy_m);
            println!("Projection centre: {}", params.proj_centre);
            println!("Latin1: {}", params.latin1);
            println!("Latin2: {}", params.latin2);
            println!("Lat south pole: {}", params.lat_south_pole);
            println!("Lon south pole: {}", params.lon_south_pole);
            params
        }
        _ => panic!("Expected LambertConformal projection variant"),
    };

    // Verify key Lambert parameters are populated and reasonable
    assert!(
        lambert_params.lad != 0.0,
        "LaD should be populated (latitude where Dx/Dy specified)"
    );
    assert!(
        lambert_params.lov > 0.0 && lambert_params.lov <= 360.0,
        "LoV should be between 0 and 360 degrees"
    );
    assert!(
        lambert_params.dx_m > 0.0,
        "Dx should be positive (grid spacing in metres)"
    );
    assert!(
        lambert_params.dy_m > 0.0,
        "Dy should be positive (grid spacing in metres)"
    );

    // Verify Latin1 and Latin2 are populated (standard parallels)
    assert!(
        lambert_params.latin1 != 0.0 || lambert_params.latin2 != 0.0,
        "At least one standard parallel should be defined"
    );

    // Verify grid dimensions are populated
    assert!(grid.nx > 0, "Nx should be positive");
    assert!(grid.ny > 0, "Ny should be positive");
    assert!(
        grid.num_data_points == grid.nx * grid.ny,
        "num_data_points should equal nx * ny"
    );

    // Verify first point coordinates
    assert!(
        grid.lat_first != 0.0,
        "La1 (latitude of first point) should be populated"
    );
    assert!(
        grid.lon_first > 0.0 && grid.lon_first <= 360.0,
        "Lo1 (longitude of first point) should be between 0 and 360"
    );

    println!("\n=== All Grid Metadata Checks Passed ===");
}

#[test]
fn verify_lazy_decode_preserves_grid_metadata() {
    let corpus_root = {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join("../../tests/corpus")
    };

    let fixture_path = corpus_root.join("large/nam.t00z.awip1200.tm00.grib2");
    if !fixture_path.exists() {
        println!("Skipping test: fixture not found");
        return;
    }

    let bytes = match std::fs::read(&fixture_path) {
        Ok(b) => b,
        Err(_) => return,
    };

    // Lazy decode should also preserve grid metadata
    let lazy_fields = match decode_bytes_lazy(&bytes) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed lazy decode: {}", e);
        }
    };

    assert!(!lazy_fields.is_empty());

    let lazy_grid = &lazy_fields[0].grid;
    assert_eq!(
        lazy_grid.template, 30,
        "Lazy decode should preserve GDT template"
    );
    assert!(lazy_grid.nx > 0, "Lazy decode should preserve Nx");
    assert!(lazy_grid.ny > 0, "Lazy decode should preserve Ny");

    match &lazy_grid.projection {
        GridProjection::LambertConformal(params) => {
            assert!(params.lad != 0.0, "Lazy decode should preserve LaD");
            assert!(params.dx_m > 0.0, "Lazy decode should preserve Dx");
            assert!(params.dy_m > 0.0, "Lazy decode should preserve Dy");
        }
        _ => panic!("Lazy decode should preserve projection type"),
    }

    println!("=== Lazy Decode Preserves Grid Metadata ===");
}

#[test]
fn verify_all_nam_fields_have_consistent_grid() {
    let corpus_root = {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join("../../tests/corpus")
    };

    let fixture_path = corpus_root.join("large/nam.t00z.awip1200.tm00.grib2");
    if !fixture_path.exists() {
        println!("Skipping test: fixture not found");
        return;
    }

    let bytes = match std::fs::read(&fixture_path) {
        Ok(b) => b,
        Err(_) => return,
    };

    let fields = match decode_bytes(&bytes) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to decode: {}", e);
        }
    };

    // All fields in a NAM file should share the same grid definition
    let first_grid = &fields[0].grid;

    for (i, field) in fields.iter().enumerate() {
        let grid = &field.grid;

        assert_eq!(
            grid.template, first_grid.template,
            "Field {}: GDT template should match field 0",
            i
        );
        assert_eq!(
            grid.nx, first_grid.nx,
            "Field {}: Nx should match field 0",
            i
        );
        assert_eq!(
            grid.ny, first_grid.ny,
            "Field {}: Ny should match field 0",
            i
        );
        assert_eq!(
            grid.lat_first, first_grid.lat_first,
            "Field {}: La1 should match field 0",
            i
        );
        assert_eq!(
            grid.lon_first, first_grid.lon_first,
            "Field {}: Lo1 should match field 0",
            i
        );

        // Verify Lambert params match
        let first_params = match &first_grid.projection {
            GridProjection::LambertConformal(p) => p,
            _ => panic!("Field 0 should have Lambert projection"),
        };

        let params = match &grid.projection {
            GridProjection::LambertConformal(p) => p,
            _ => panic!("Field {} should have Lambert projection", i),
        };

        assert_eq!(
            params.lad, first_params.lad,
            "Field {}: LaD should match field 0",
            i
        );
        assert_eq!(
            params.lov, first_params.lov,
            "Field {}: LoV should match field 0",
            i
        );
        assert_eq!(
            params.dx_m, first_params.dx_m,
            "Field {}: Dx should match field 0",
            i
        );
        assert_eq!(
            params.dy_m, first_params.dy_m,
            "Field {}: Dy should match field 0",
            i
        );
    }

    println!(
        "=== All {} Fields Have Consistent Grid Metadata ===",
        fields.len()
    );
}
