//! Station-extraction correctness test for real GFS DRT=3 data.
//!
//! Verifies that `field.grid.nearest_index` + `field.values.get_at` returns
//! the value at the nearest grid point for 7 CONUS stations, and that the
//! returned value matches the golden reference within packing tolerance.
//!
//! This is the persistent regression gate for "station extraction works on
//! real GFS DRT=3 (complex packing with spatial differencing) data."

use gribtract_testutil::{corpus, golden};

/// 7 CONUS stations from the bead spec: (name, lat_deg_N, lon_deg_E).
/// Negative longitudes are western hemisphere; `nearest_index` normalises them.
const STATIONS: &[(&str, f64, f64)] = &[
    ("New York",     40.78, -73.97),
    ("Philadelphia", 39.87, -75.23),
    ("Chicago",      41.79, -87.75),
    ("Miami",        25.79, -80.29),
    ("Austin",       30.32, -97.76),
    ("Denver",       39.85, -104.66),
    ("Los Angeles",  33.94, -118.41),
];

/// Grid parameters for `gfs_tmp2m_1deg_anl`: 360×181, 1° global lat/lon,
/// lat_first=90, lon_first=0, scanning_mode=0 (N→S rows, W→E cols).
///
/// Nearest index for (lat, lon):
///   row  = round((90 − lat) / 1)
///   col  = round(lon_normalised / 1) % 360
///   idx  = row * 360 + col
fn expected_nearest_index(lat: f64, lon: f64) -> usize {
    let nx = 360usize;
    let ny = 181usize;
    let mut lon_n = lon % 360.0;
    if lon_n < 0.0 { lon_n += 360.0; }
    let row = ((90.0 - lat) / 1.0).round() as usize;
    let col = (lon_n / 1.0).round() as usize % nx;
    assert!(row < ny, "row {row} out of bounds for station lat={lat}");
    row * nx + col
}

#[test]
fn station_extraction_drt3_gfs_tmp2m() {
    // ── Load + decode the real GFS fixture (DRT=3, 360×181 global 1°) ──────────
    let bytes = corpus::load("gfs_tmp2m_1deg_anl")
        .expect("fixture gfs_tmp2m_1deg_anl must be present in corpus");

    let fields = gribtract::decode(&bytes)
        .expect("decode must succeed for gfs_tmp2m_1deg_anl");

    assert_eq!(fields.len(), 1, "expected exactly one field in gfs_tmp2m_1deg_anl");
    let field = &fields[0];

    // Sanity-check grid dimensions.
    assert_eq!(field.grid.nx, 360, "expected nx=360");
    assert_eq!(field.grid.ny, 181, "expected ny=181");

    // ── Load golden reference ────────────────────────────────────────────────────
    let golden_fixture = golden::load_golden("gfs_tmp2m_1deg_anl")
        .expect("golden load must not error")
        .expect("golden for gfs_tmp2m_1deg_anl must exist");

    assert_eq!(golden_fixture.fields.len(), 1);
    let golden_field = &golden_fixture.fields[0];

    // Extract the Dense values from the golden (this fixture has no bitmap).
    let golden_values = match &golden_field.values {
        golden::GoldenGridValues::Dense(v) => v.as_slice(),
        golden::GoldenGridValues::Masked { .. } => {
            panic!("gfs_tmp2m_1deg_anl golden must be Dense, not Masked")
        }
    };

    // ── Tolerance from packing parameters ────────────────────────────────────────
    // step = 2^E / 10^D; tolerance = step / 2.
    // For this fixture: E=0, D=1  →  step=0.1, tol=0.05 K.
    let tol = field.packing.tolerance().max(1e-12);
    eprintln!("  packing tolerance = {tol:.6}");

    // ── Per-station assertions ────────────────────────────────────────────────────
    for &(name, lat, lon) in STATIONS {
        // nearest_index must return Some for every CONUS station on a 360×181 grid.
        let idx = field
            .grid
            .nearest_index(lat, lon)
            .unwrap_or_else(|| panic!("nearest_index returned None for station '{name}' ({lat}, {lon})"));

        // Cross-check our manual index calculation matches the library.
        let expected_idx = expected_nearest_index(lat, lon);
        assert_eq!(
            idx, expected_idx,
            "nearest_index mismatch for '{name}': got {idx}, expected {expected_idx}",
        );

        // The decoded value at that index must be present (no bitmap here).
        let decoded_val = field
            .values
            .get_at(idx)
            .unwrap_or_else(|| panic!("get_at returned None for station '{name}' (idx={idx})"));

        // The golden value at the same index.
        let golden_val = golden_values[idx];

        let diff = (decoded_val - golden_val).abs();
        assert!(
            diff <= tol,
            "value mismatch for '{name}' (idx={idx}): decoded={decoded_val:.6}, \
             golden={golden_val:.6}, diff={diff:.9}, tol={tol:.6}",
        );

        eprintln!(
            "  [ok] {name}: lat={lat}, lon={lon} → idx={idx}, \
             val={decoded_val:.4}, golden={golden_val:.4}, diff={diff:.6}",
        );
    }
}
