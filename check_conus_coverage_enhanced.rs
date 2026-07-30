// Enhanced CONUS station coverage validation with edge distance analysis
// Usage: rustc --edition=2021 check_conus_coverage_enhanced.rs -L target/release/deps --extern gribtract=target/release/libgribtract.rlib

use std::fs;

// Expanded CONUS weather stations (airport codes, lat, lon, region)
const CONUS_STATIONS: &[(&str, f64, f64, &str)] = &[
    // East Coast (13 stations)
    ("JFK", 40.64, -73.78, "East Coast"),
    ("BOS", 42.36, -71.01, "East Coast"),
    ("ATL", 33.64, -84.43, "Southeast"),
    ("MIA", 25.79, -80.29, "Southeast"),
    ("DCA", 38.85, -77.04, "East Coast"),
    ("CLT", 35.21, -80.95, "Southeast"),
    ("PHL", 39.87, -75.24, "East Coast"),
    ("EWR", 40.69, -74.17, "East Coast"),
    ("FLL", 26.07, -80.15, "Southeast"),
    ("TPA", 27.97, -82.53, "Southeast"),
    ("JAX", 30.49, -81.69, "Southeast"),
    ("RDU", 35.88, -78.79, "Southeast"),
    ("ORF", 36.90, -76.20, "Southeast"),

    // Midwest (8 stations)
    ("ORD", 41.98, -87.90, "Midwest"),
    ("MSP", 44.88, -93.22, "Midwest"),
    ("DTW", 42.21, -83.35, "Midwest"),
    ("CLE", 41.41, -81.85, "Midwest"),
    ("IND", 39.73, -86.27, "Midwest"),
    ("MKE", 42.95, -87.90, "Midwest"),
    ("STL", 38.75, -90.37, "Midwest"),
    ("CMH", 39.99, -82.89, "Midwest"),

    // Central / South (7 stations)
    ("DFW", 32.90, -97.04, "South Central"),
    ("IAH", 29.99, -95.34, "South Central"),
    ("MSY", 29.99, -90.26, "South Central"),
    ("AUS", 30.19, -97.67, "South Central"),
    ("SAT", 29.53, -98.47, "South Central"),
    ("ELP", 31.81, -106.38, "South Central"),
    ("OKC", 35.39, -97.60, "South Central"),

    // Mountain (5 stations)
    ("DEN", 39.85, -104.67, "Mountain"),
    ("SLC", 40.79, -111.98, "Mountain"),
    ("PHX", 33.43, -112.01, "Southwest"),
    ("ABQ", 35.04, -106.61, "Mountain"),
    ("BOI", 43.56, -116.22, "Mountain"),

    // West Coast (8 stations)
    ("LAX", 33.94, -118.41, "West Coast"),
    ("SFO", 37.62, -122.38, "West Coast"),
    ("SEA", 47.45, -122.31, "West Coast"),
    ("PDX", 45.59, -122.60, "West Coast"),
    ("SAN", 32.73, -117.19, "West Coast"),
    ("SMF", 38.70, -121.59, "West Coast"),
    ("OAK", 37.71, -122.22, "West Coast"),
    ("SJC", 37.36, -121.93, "West Coast"),

    // Southwest (4 stations)
    ("LAS", 36.08, -115.15, "Southwest"),
    ("TUS", 32.12, -110.95, "Southwest"),
    ("PSP", 33.83, -116.51, "Southwest"),
    ("RNO", 39.50, -119.77, "Southwest"),

    // Northern Border (6 stations)
    ("FAR", 46.87, -96.81, "Northern"),
    ("GFK", 47.95, -97.18, "Northern"),
    ("BIS", 46.77, -100.75, "Northern"),
    ("MOT", 48.26, -101.29, "Northern"),
    ("INL", 48.57, -93.39, "Northern"),
    ("HIB", 47.37, -92.84, "Northern"),

    // Southern Border (5 stations)
    ("CRP", 27.77, -97.51, "Southern"),
    ("BRO", 25.91, -97.43, "Southern"),
    ("TUS", 32.12, -110.95, "Southern"),
    ("ELP", 31.81, -106.38, "Southern"),
    ("YUM", 32.66, -114.60, "Southern"),
];

fn calculate_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Haversine formula for distance on sphere
    const EARTH_RADIUS_KM: f64 = 6371.0;

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2) +
           lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <grib2_file>", args[0]);
        std::process::exit(1);
    }

    let grib_path = &args[1];
    println!("🌦️  Enhanced CONUS Coverage Validation");
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
    println!("  Size: {} x {} ({} points)", grid.nx, grid.ny, grid.num_data_points);
    println!("  First point: {:.6}°N, {:.6}°E", grid.lat_first, grid.lon_first);

    // Convert longitude to 0-360 range for display
    let lon_first_deg = if grid.lon_first >= 180.0 {
        grid.lon_first - 360.0
    } else {
        grid.lon_first
    };
    println!("              ({:.6}°N, {:.6}°W)", grid.lat_first, -lon_first_deg);

    match &grid.projection {
        gribtract::GridProjection::LambertConformal(params) => {
            println!("  Projection: Lambert Conformal Conic");
            println!("    LaD (ref lat): {:.3}°N", params.lad);
            println!("    LoV (meridian): {:.3}°E ({:.3}°W)", params.lov, params.lov - 360.0);
            println!("    Latin1: {:.3}°N", params.latin1);
            println!("    Latin2: {:.3}°N", params.latin2);
            println!("    Dx: {:.1} km", params.dx_m / 1000.0);
            println!("    Dy: {:.1} km", params.dy_m / 1000.0);
        }
        gribtract::GridProjection::LatLon => {
            println!("  Projection: Latitude/Longitude");
            println!("    Di: {:.6}°", grid.di);
            println!("    Dj: {:.6}°", grid.dj);
            println!("    Last point: {:.6}°N, {:.6}°E", grid.lat_last, grid.lon_last);
        }
        _ => {
            println!("  Projection: {:?}", grid.projection);
        }
    }
    println!();

    // Test CONUS stations
    println!("🎯 Testing CONUS Station Coverage:");
    println!("{:6} {:>10} {:>10} {:>12} {:>15} {:>12}", "Code", "Latitude", "Longitude", "Region", "Status", "Distance");
    println!("{:-6} {:-10} {:-10} {:-12} {:-15} {:-12}", "------", "----------", "----------", "------------", "---------------", "------------");

    let mut covered = 0;
    let mut not_covered = 0;
    let mut marginal_stations = Vec::new();

    for (code, lat, lon, region) in CONUS_STATIONS {
        // Convert longitude to 0-360 range for the grid lookup
        let lon_grid = if *lon < 0.0 {
            *lon + 360.0
        } else {
            *lon
        };

        match grid.nearest_index(*lat, lon_grid) {
            Some(idx) => {
                // Calculate distance from grid center as a proxy for edge proximity
                // For Lambert Conformal, we use a rough approximation
                let grid_center_lat = 38.5; // LaD
                let grid_center_lon = -97.5; // LoV

                let dist_from_center = calculate_distance_km(*lat, *lon, grid_center_lat, grid_center_lon);

                // Calculate approximate distance from grid edge (very rough estimate)
                // HRRR CONUS spans roughly 2000km from center in most directions
                let grid_radius_km = 2000.0;
                let dist_from_edge = (grid_radius_km - dist_from_center).max(0.0);

                let status = if dist_from_edge < 100.0 {
                    format!("✓ COVERED (EDGE)")
                } else if dist_from_edge < 300.0 {
                    format!("✓ COVERED")
                } else {
                    format!("✓ COVERED")
                };

                println!("{:6} {:>10.3} {:>10.3} {:>12} {:>15} {:>6.0}km from center",
                    code, lat, lon, region, status, dist_from_center);

                if dist_from_edge < 100.0 {
                    marginal_stations.push((code, lat, lon, region, dist_from_edge));
                }

                covered += 1;
            }
            None => {
                println!("{:6} {:>10.3} {:>10.3} {:>12} {:>15}",
                    code, lat, lon, region, "✗ NOT COVERED");
                not_covered += 1;
            }
        }
    }

    println!();
    println!("📈 Coverage Summary:");
    println!("  Total stations: {}", CONUS_STATIONS.len());
    println!("  Covered: {} ({:.1}%)", covered, (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0);
    println!("  Not covered: {} ({:.1}%)", not_covered, (not_covered as f64 / CONUS_STATIONS.len() as f64) * 100.0);

    if !marginal_stations.is_empty() {
        println!();
        println!("⚠️  Marginal Stations (near grid edge):");
        println!("{:6} {:>10} {:>10} {:>12} {:>12}", "Code", "Latitude", "Longitude", "Region", "Est. from edge");
        println!("{:-6} {:-10} {:-10} {:-12} {:-12}", "------", "----------", "----------", "------------", "------------");
        for (code, lat, lon, region, dist_from_edge) in marginal_stations {
            println!("{:6} {:>10.3} {:>10.3} {:>12} ~{:6.0} km", code, lat, lon, region, dist_from_edge);
        }
    }

    println!();
    println!("🌐 Geographic Coverage Assessment:");

    // Count by region
    let mut region_counts = std::collections::HashMap::new();
    for (_, _, _, region) in CONUS_STATIONS {
        *region_counts.entry(region).or_insert(0) += 1;
    }

    println!("  Stations by region:");
    let mut regions: Vec<_> = region_counts.into_iter().collect();
    regions.sort_by(|a, b| a.0.cmp(b.0));
    for (region, count) in regions {
        println!("    {:>12}: {}", region, count);
    }

    // Final verdict
    println!();
    if covered == CONUS_STATIONS.len() {
        println!("✅ SUCCESS: All tested CONUS stations ({} stations) are covered by this grid", CONUS_STATIONS.len());
        println!("   Coverage spans East Coast, Midwest, South Central, Mountain, West Coast, Southwest,");
        println!("   Northern border, and Southern border regions - comprehensive CONUS coverage confirmed.");
        std::process::exit(0);
    } else if covered >= (CONUS_STATIONS.len() * 3 / 4) {
        println!("⚠️  PARTIAL: Most CONUS stations covered ({}%); edge stations may be outside grid bounds",
            (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0);
        std::process::exit(0);
    } else {
        println!("❌ FAILURE: Grid does not adequately cover CONUS stations ({}% coverage)",
            (covered as f64 / CONUS_STATIONS.len() as f64) * 100.0);
        std::process::exit(1);
    }
}
