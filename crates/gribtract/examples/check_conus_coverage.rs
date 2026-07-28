// Validate GRIB2 HRRR file covers CONUS stations
// Usage: rustc --edition=2021 check_conus_coverage.rs -L target/release/deps --extern gribtract=target/release/libgribtract.rlib

use std::fs;

// Key CONUS weather stations (airport codes, lat, lon)
const CONUS_STATIONS: &[(&str, f64, f64)] = &[
    // East Coast
    ("JFK", 40.64, -73.78), // New York
    ("BOS", 42.36, -71.01), // Boston
    ("ATL", 33.64, -84.43), // Atlanta
    ("MIA", 25.79, -80.29), // Miami
    ("DCA", 38.85, -77.04), // Washington DC
    // Midwest
    ("ORD", 41.98, -87.90), // Chicago
    ("MSP", 44.88, -93.22), // Minneapolis
    ("DTW", 42.21, -83.35), // Detroit
    // Central / South
    ("DFW", 32.90, -97.04), // Dallas
    ("IAH", 29.99, -95.34), // Houston
    ("MSY", 29.99, -90.26), // New Orleans
    // Mountain / West
    ("DEN", 39.85, -104.67), // Denver
    ("SLC", 40.79, -111.98), // Salt Lake City
    ("PHX", 33.43, -112.01), // Phoenix
    // West Coast
    ("LAX", 33.94, -118.41), // Los Angeles
    ("SFO", 37.62, -122.38), // San Francisco
    ("SEA", 47.45, -122.31), // Seattle
    ("PDX", 45.59, -122.60), // Portland
    // Southwest
    ("LAS", 36.08, -115.15), // Las Vegas
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <grib2_file>", args[0]);
        std::process::exit(1);
    }

    let grib_path = &args[1];
    println!("🌦️  CONUS Coverage Validation");
    println!("File: {}", grib_path);
    println!();

    // Read and decode the GRIB2 file
    let bytes = match fs::read(grib_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            std::process::exit(1);
        }
    };

    // Decode fields (we only need the first one to get grid definition)
    let fields = match gribtract::decode(&bytes) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Failed to decode GRIB2: {}", e);
            std::process::exit(1);
        }
    };

    if fields.is_empty() {
        eprintln!("❌ No fields found in GRIB2 file");
        std::process::exit(1);
    }

    let grid = &fields[0].grid;

    println!("📊 Grid Definition:");
    println!("  Template: {}", grid.template);
    println!(
        "  Size: {} x {} ({} points)",
        grid.nx, grid.ny, grid.num_data_points
    );
    println!(
        "  First point: {:.6}°N, {:.6}°E",
        grid.lat_first, grid.lon_first
    );

    // Convert longitude to 0-360 range for display
    let lon_first_deg = if grid.lon_first >= 180.0 {
        grid.lon_first - 360.0
    } else {
        grid.lon_first
    };
    println!(
        "              ({:.6}°N, {:.6}°W)",
        grid.lat_first, -lon_first_deg
    );

    match &grid.projection {
        gribtract::GridProjection::LambertConformal(params) => {
            println!("  Projection: Lambert Conformal Conic");
            println!("    LaD (ref lat): {:.3}°N", params.lad);
            println!(
                "    LoV (meridian): {:.3}°E ({:.3}°W)",
                params.lov,
                params.lov - 360.0
            );
            println!("    Latin1: {:.3}°N", params.latin1);
            println!("    Latin2: {:.3}°N", params.latin2);
            println!("    Dx: {:.1} km", params.dx_m / 1000.0);
            println!("    Dy: {:.1} km", params.dy_m / 1000.0);
        }
        gribtract::GridProjection::LatLon => {
            println!("  Projection: Latitude/Longitude");
            println!("    Di: {:.6}°", grid.di);
            println!("    Dj: {:.6}°", grid.dj);
            println!(
                "    Last point: {:.6}°N, {:.6}°E",
                grid.lat_last, grid.lon_last
            );
        }
        _ => {
            println!("  Projection: {:?}", grid.projection);
        }
    }
    println!();

    // Test CONUS stations
    println!("🎯 Testing CONUS Station Coverage:");
    println!(
        "{:6} {:>10} {:>10} {:>12}",
        "Code", "Latitude", "Longitude", "Status"
    );
    println!(
        "{:-6} {:-10} {:-10} {:-12}",
        "------", "----------", "----------", "------------"
    );

    let mut covered = 0;
    let mut not_covered = 0;

    for (code, lat, lon) in CONUS_STATIONS {
        // Convert longitude to 0-360 range for the grid lookup
        let lon_grid = if *lon < 0.0 { *lon + 360.0 } else { *lon };

        match grid.nearest_index(*lat, lon_grid) {
            Some(idx) => {
                println!(
                    "{:6} {:>10.3} {:>10.3} {:>12} (index: {})",
                    code, lat, lon, "✓ COVERED", idx
                );
                covered += 1;
            }
            None => {
                println!(
                    "{:6} {:>10.3} {:>10.3} {:>12}",
                    code, lat, lon, "✗ NOT COVERED"
                );
                not_covered += 1;
            }
        }
    }

    println!();
    println!("📈 Coverage Summary:");
    println!("  Total stations: {}", CONUS_STATIONS.len());
    println!(
        "  Covered: {} ({:.1}%)",
        covered,
        (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0
    );
    println!(
        "  Not covered: {} ({:.1}%)",
        not_covered,
        (not_covered as f64 / CONUS_STATIONS.len() as f64) * 100.0
    );

    // Final verdict
    if covered == CONUS_STATIONS.len() {
        println!();
        println!("✅ SUCCESS: All tested CONUS stations are covered by this grid");
        std::process::exit(0);
    } else if covered >= (CONUS_STATIONS.len() * 3 / 4) {
        println!();
        println!("⚠️  PARTIAL: Most CONUS stations covered ({}%); edge stations may be outside grid bounds",
            (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0);
        std::process::exit(0);
    } else {
        println!();
        println!(
            "❌ FAILURE: Grid does not adequately cover CONUS stations ({}% coverage)",
            (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0
        );
        std::process::exit(1);
    }
}
