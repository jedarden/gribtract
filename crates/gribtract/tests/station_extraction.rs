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

/// Verifies the decode-once-extract-many pattern for DRT=3 data.
///
/// Checks that:
/// 1. `decode_lazy` populates `section7_raw` and `complex_extra` for DRT=3 fields.
/// 2. `decode_all_drt3` reproduces every value of the full `decode` within packing
///    tolerance — i.e. the cached path is bit-identical to the eager decode.
/// 3. Station lookups via the cached path match the golden reference.
#[test]
fn drt3_decode_once_extract_many_matches_full_decode() {
    let bytes = corpus::load("gfs_tmp2m_1deg_anl")
        .expect("fixture gfs_tmp2m_1deg_anl must be present in corpus");

    let full_fields = gribtract::decode(&bytes).expect("full decode");
    let lazy_fields = gribtract::decode_lazy(&bytes).expect("lazy decode");

    assert_eq!(full_fields.len(), lazy_fields.len());

    for (full, lazy) in full_fields.iter().zip(lazy_fields.iter()) {
        // DRT=3 field must have raw bytes.
        assert_eq!(lazy.drt_template, 3, "fixture must be DRT=3");
        assert!(
            !lazy.section7_raw.is_empty(),
            "section7_raw must be populated for DRT=3 lazy field"
        );
        let extra = lazy
            .complex_extra
            .as_ref()
            .expect("complex_extra must be populated for DRT=3 lazy field");

        let n_pts = lazy.grid.num_data_points as usize;
        let decoded_once = gribtract::decode_all_drt3(&lazy.section7_raw, &lazy.packing, extra, n_pts)
            .expect("decode_all_drt3 must succeed");

        assert_eq!(decoded_once.len(), n_pts, "decoded length must match n_points");

        // Compare every grid point against the full eager decode.
        let tol = lazy.packing.tolerance().max(1e-12);
        let gribtract::GridValues::Dense(ref full_vals) = full.values else {
            panic!("expected Dense GridValues for DRT=3 fixture");
        };
        let mut mismatches = 0usize;
        for (idx, (&cached, &full_v)) in decoded_once.iter().zip(full_vals.iter()).enumerate() {
            if (cached - full_v).abs() > tol {
                mismatches += 1;
                if mismatches <= 5 {
                    eprintln!(
                        "  mismatch idx={idx}: cached={cached:.6}, full={full_v:.6}, tol={tol:.6}"
                    );
                }
            }
        }
        assert_eq!(mismatches, 0, "{mismatches} grid points differ between cached and full decode");

        // Also verify the 7 CONUS stations match the golden reference.
        let golden_fixture = golden::load_golden("gfs_tmp2m_1deg_anl")
            .expect("golden load must not error")
            .expect("golden for gfs_tmp2m_1deg_anl must exist");
        let golden_values = match &golden_fixture.fields[0].values {
            golden::GoldenGridValues::Dense(v) => v.clone(),
            golden::GoldenGridValues::Masked { .. } => panic!("expected Dense golden"),
        };
        for &(name, lat, lon) in STATIONS {
            let idx = full.grid.nearest_index(lat, lon)
                .unwrap_or_else(|| panic!("nearest_index None for '{name}'"));
            let cached_val = decoded_once[idx];
            let golden_val = golden_values[idx];
            let diff = (cached_val - golden_val).abs();
            assert!(
                diff <= tol,
                "station '{name}' idx={idx}: cached_drt3={cached_val:.6}, golden={golden_val:.6}, diff={diff:.9}",
            );
        }
        eprintln!("  [ok] decode-once-extract-many: {n_pts} grid points, 0 mismatches vs full decode");
    }
}

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
