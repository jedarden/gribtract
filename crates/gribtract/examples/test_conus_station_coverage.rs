//! Test CONUS DRT=0 fixture station coverage
//!
//! This example verifies that the synthetic CONUS DRT=0 fixture covers
//! the US weather stations used in the station extraction benchmark.

use std::fs;

// Copy the station roster from xtask/src/bench_station.rs
const STATIONS: &[(&str, f64, f64)] = &[
    // Eastern Time
    ("New York", 40.7789, -73.9692),      // KNYC Central Park
    ("Miami", 25.7959, -80.2870),         // KMIA
    ("Philadelphia", 39.8721, -75.2411),  // KPHL
    ("Atlanta", 33.6407, -84.4277),       // KATL
    ("Boston", 42.3656, -71.0096),        // KBOS
    ("Washington DC", 38.8512, -77.0402), // KDCA Reagan
    // Central Time
    ("Chicago", 41.7868, -87.7522),       // KMDW Midway
    ("Dallas", 32.8998, -97.0403),        // KDFW
    ("Houston", 29.9902, -95.3368),       // KIAH
    ("Minneapolis", 44.8820, -93.2218),   // KMSP
    ("Austin", 30.1945, -97.6699),        // KAUS
    ("New Orleans", 29.9934, -90.2580),   // KMSY
    ("San Antonio", 29.5337, -98.4698),   // KSAT
    ("Oklahoma City", 35.3931, -97.6007), // KOKC
    // Mountain / Arizona
    ("Denver", 39.8561, -104.6737),  // KDEN
    ("Phoenix", 33.4373, -112.0078), // KPHX Sky Harbor
    // Pacific Time
    ("Los Angeles", 33.9416, -118.4085),   // KLAX
    ("Las Vegas", 36.0840, -115.1537),     // KLAS
    ("Seattle", 47.4502, -122.3088),       // KSEA Sea-Tac
    ("San Francisco", 37.6189, -122.3750), // KSFO
];

fn main() {
    println!("=== CONUS DRT=0 Station Coverage Test ===\n");

    // Test the synthetic CONUS DRT=0 fixture
    let synthetic_path = "tests/corpus/small/conus_drt0.grib2";
    println!("Testing synthetic fixture: {}", synthetic_path);

    let synthetic_bytes =
        fs::read(synthetic_path).expect("Failed to read synthetic CONUS DRT=0 fixture");

    let synthetic_fields =
        gribtract::decode(&synthetic_bytes).expect("Failed to decode synthetic fixture");

    println!(
        "Decoded {} field(s) from synthetic fixture\n",
        synthetic_fields.len()
    );

    for (i, field) in synthetic_fields.iter().enumerate() {
        println!("Field {}:", i);
        println!(
            "  Grid: {} points ({} x {})",
            field.grid.num_data_points, field.grid.nx, field.grid.ny
        );
        println!("  Template: {}", field.grid.template);

        // Count stations in range
        let mut in_range = 0usize;
        let mut out_of_range_stations = Vec::new();

        println!("\n  Testing station coverage:");
        for &(name, lat, lon) in STATIONS {
            match field.grid.nearest_index(lat, lon) {
                Some(idx) => {
                    // Check if this index actually has a value
                    if field.values.get_at(idx).is_some() {
                        in_range += 1;
                        println!("    ✅ {}: ({:.4}, {:.4}) -> index {}", name, lat, lon, idx);
                    } else {
                        out_of_range_stations.push((
                            name,
                            lat,
                            lon,
                            "No value at index".to_string(),
                        ));
                        println!(
                            "    ❌ {}: ({:.4}, {:.4}) -> No value at index {}",
                            name, lat, lon, idx
                        );
                    }
                }
                None => {
                    out_of_range_stations.push((name, lat, lon, "No grid index".to_string()));
                    println!(
                        "    ❌ {}: ({:.4}, {:.4}) -> Not in grid bounds",
                        name, lat, lon
                    );
                }
            }
        }

        println!(
            "\n  Coverage: {}/{} stations in range ({:.1}%)",
            in_range,
            STATIONS.len(),
            (in_range as f64 / STATIONS.len() as f64) * 100.0
        );

        if out_of_range_stations.is_empty() {
            println!(
                "  ✅ All {} US stations covered by synthetic CONUS DRT=0!",
                STATIONS.len()
            );
        } else {
            println!(
                "  ⚠️  {} station(s) NOT covered:",
                out_of_range_stations.len()
            );
            for (name, lat, lon, reason) in &out_of_range_stations {
                println!("     {} ({:.4}, {:.4}): {}", name, lat, lon, reason);
            }
        }
        println!();
    }

    // For comparison, test the old global grid fixture
    let old_global_path = "tests/corpus/small/gfs_anl_t2m_5x5.grib2";
    println!("=== Testing Old Global Grid Fixture (Before CONUS DRT=0) ===");

    let old_bytes = fs::read(old_global_path).expect("Failed to read old global fixture");

    let old_fields = gribtract::decode(&old_bytes).expect("Failed to decode old global fixture");

    println!(
        "Old global fixture grid: {}°N-{}°N, {}°E-{}°E ({}x{} = {} points)",
        old_fields[0].grid.lat_first,
        old_fields[0].grid.lat_last,
        old_fields[0].grid.lon_first,
        old_fields[0].grid.lon_last,
        old_fields[0].grid.nx,
        old_fields[0].grid.ny,
        old_fields[0].grid.num_data_points
    );

    let mut old_in_range = 0usize;
    for &(name, lat, lon) in STATIONS {
        match old_fields[0].grid.nearest_index(lat, lon) {
            Some(_) => {
                old_in_range += 1;
                println!("  ✅ {}: ({:.4}, {:.4}) IN RANGE", name, lat, lon);
            }
            None => println!("  ❌ {}: ({:.4}, {:.4}) OUT OF RANGE", name, lat, lon),
        }
    }
    println!(
        "Old global fixture coverage: {}/{} stations ({}%)\n",
        old_in_range,
        STATIONS.len(),
        (old_in_range as f64 / STATIONS.len() as f64) * 100.0
    );

    // Test the large GFS CONUS DRT=0 fixture if available
    let large_path = "tests/corpus/large/gfs.t00z.pgrb2.0p50.f000";
    if fs::metadata(large_path).is_ok() {
        println!("=== Testing Large GFS CONUS DRT=0 Fixture ===");
        println!("Loading large fixture (this may take a moment)...\n");

        let large_bytes = fs::read(large_path).expect("Failed to read large GFS CONUS fixture");

        println!(
            "File size: {:.1} MB\n",
            large_bytes.len() as f64 / 1_048_576.0
        );

        let large_fields = gribtract::decode(&large_bytes).expect("Failed to decode large fixture");

        println!(
            "Decoded {} field(s) from large fixture\n",
            large_fields.len()
        );

        // Sample first 3 fields to show coverage
        for (i, field) in large_fields.iter().take(3).enumerate() {
            println!("Field {}:", i);
            println!(
                "  Grid: {} points ({} x {})",
                field.grid.num_data_points, field.grid.nx, field.grid.ny
            );
            println!("  Template: {}", field.grid.template);

            // Count stations in range
            let mut in_range = 0usize;
            let mut out_of_range = 0usize;

            for &(name, lat, lon) in STATIONS {
                match field.grid.nearest_index(lat, lon) {
                    Some(idx) => {
                        if field.values.get_at(idx).is_some() {
                            in_range += 1;
                        } else {
                            out_of_range += 1;
                        }
                    }
                    None => {
                        out_of_range += 1;
                    }
                }
            }

            println!(
                "  Coverage: {}/{} stations in range ({:.1}%)",
                in_range,
                STATIONS.len(),
                (in_range as f64 / STATIONS.len() as f64) * 100.0
            );

            if out_of_range == 0 {
                println!(
                    "  ✅ All {} US stations covered by GFS CONUS DRT=0!",
                    STATIONS.len()
                );
            } else {
                println!("  ⚠️  {} station(s) NOT covered", out_of_range);
            }
            println!();
        }

        if large_fields.len() > 3 {
            println!("... ({} more fields not shown)", large_fields.len() - 3);
        }
    } else {
        println!("Large fixture not available: {}", large_path);
    }

    println!("\n=== Summary ===");
    println!(
        "Station roster contains {} US weather stations",
        STATIONS.len()
    );
    println!("Test fixtures: synthetic CONUS DRT=0 (283 bytes), GFS 0.50° global (152 MB)");
}
