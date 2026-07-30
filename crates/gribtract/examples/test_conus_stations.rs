use gribtract::decode_lazy;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("hrrr_conus_test.grib2")?;
    let lazy_fields = gribtract::decode_lazy(&bytes)?;

    // Key CONUS weather stations
    let stations = vec![
        ("JFK", 40.64, -73.78),
        ("ORD", 41.979, -87.905),
        ("LAX", 33.943, -118.408),
        ("DFW", 32.898, -97.040),
        ("DEN", 39.856, -104.675),
        ("ATL", 33.641, -84.428),
        ("SEA", 47.449, -122.309),
        ("MIA", 25.795, -80.238),
        ("SFO", 37.619, -122.375),
        ("BOS", 42.364, -71.005),
    ];

    println!("HRRR CONUS Coverage Validation");
    println!("===============================");

    // Use first field to check grid coverage
    if let Some(first_field) = lazy_fields.first() {
        let grid = &first_field.grid;
        println!("Grid: {} x {} points", grid.nx, grid.ny);
        println!("");

        println!("Testing CONUS station coverage:");
        println!("--------------------------------");

        let mut all_covered = true;
        for (name, lat, lon) in &stations {
            match grid.nearest_index(*lat, *lon) {
                Some(idx) => {
                    println!("✅ {}: {}°N, {}°W → Grid index {}", name, lat, lon, idx);
                }
                None => {
                    println!("❌ {}: {}°N, {}°W → NOT COVERED", name, lat, lon);
                    all_covered = false;
                }
            }
        }

        println!("");
        if all_covered {
            println!("✅ SUCCESS: All tested CONUS stations are covered by HRRR CONUS grid");
        } else {
            println!("⚠️  WARNING: Some stations are not covered by the grid");
        }
    }

    Ok(())
}
