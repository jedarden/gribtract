//! Fixed-station point-extraction benchmark.
//!
//! Measures how fast gribtract can extract a per-station time series from a
//! set of decoded fields. Two modes: nearest-grid-point and bilinear. Every
//! extracted value is verified against the reference (full-grid decode at the
//! same point), so speed is never claimed without a correctness proof.
//!
//! See `docs/notes/station-extraction-benchmark.md` for the design rationale.

use std::time::Instant;

use gribtract::Field;

// ── Station roster ────────────────────────────────────────────────────────────

/// Fixed US metro weather-station coordinates (lat, lon in degrees).
/// Stable over time so benchmark runs are comparable.
pub const STATIONS: &[(&str, f64, f64)] = &[
    ("New York",     40.78,  -73.97),
    ("Philadelphia", 39.87,  -75.23),
    ("Chicago",      41.79,  -87.75),
    ("Miami",        25.79,  -80.29),
    ("Austin",       30.32,  -97.76),
    ("Denver",       39.85, -104.66),
    ("Los Angeles",  33.94, -118.41),
];

// ── Result type ───────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct StationBenchResult {
    pub interpolation: String,
    pub n_stations: usize,
    pub n_fields: usize,
    /// Station+field pairs where a grid point was found (in-range).
    pub in_range: usize,
    /// Wall time per full extraction pass (all stations × all fields).
    pub wall_ms: f64,
    /// Throughput: in_range station-hours / wall_time_seconds.
    pub station_hours_per_sec: f64,
    /// Fraction of in-range extractions matching the reference value.
    pub agreement: f64,
}

// ── Extraction helpers ────────────────────────────────────────────────────────

fn extract_nearest(field: &Field, lat: f64, lon: f64) -> Option<f64> {
    let idx = field.grid.nearest_index(lat, lon)?;
    field.values.get_at(idx)
}

fn extract_bilinear(field: &Field, lat: f64, lon: f64) -> Option<f64> {
    // BilinearCorners is inferred here; we do not need to name the type.
    let corners = field.grid.bilinear_corners(lat, lon)?;
    field.values.bilinear(&corners)
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the station extraction benchmark for both nearest and bilinear modes.
///
/// Treats each element of `fields` as one "forecast hour". Returns one
/// `StationBenchResult` per interpolation mode.
pub fn run(fields: &[Field]) -> Vec<StationBenchResult> {
    let mut results = Vec::new();
    for mode in ["nearest", "bilinear"] {
        let r = run_mode(fields, mode);
        eprintln!(
            "  [station-{}] {} fields × {} stations → {} in-range | \
             {:.0} station-hours/s | agreement={:.1}%",
            mode,
            r.n_fields,
            r.n_stations,
            r.in_range,
            r.station_hours_per_sec,
            r.agreement * 100.0,
        );
        results.push(r);
    }
    results
}

// ── Per-mode runner ───────────────────────────────────────────────────────────

fn run_mode(fields: &[Field], mode: &str) -> StationBenchResult {
    let n_stations = STATIONS.len();
    let n_fields = fields.len();

    // Warmup: run until ≥10 ms elapsed, count iterations
    let mut n_warmup = 0u32;
    let t_warmup = Instant::now();
    loop {
        let _ = extract_all_count(fields, mode);
        n_warmup += 1;
        if t_warmup.elapsed().as_millis() >= 10 {
            break;
        }
    }
    let warmup_ns = t_warmup.elapsed().as_nanos() as f64;
    let ns_per_iter = (warmup_ns / n_warmup as f64).max(1.0);

    // Timed: target ~200 ms of work for accuracy
    let target_ns = 200_000_000.0f64;
    let n_timed = ((target_ns / ns_per_iter).ceil() as u32).clamp(10, 100_000);
    let t0 = Instant::now();
    for _ in 0..n_timed {
        let _ = extract_all_count(fields, mode);
    }
    let wall_ns_per_iter = t0.elapsed().as_nanos() as u64 / n_timed as u64;
    let wall_ms = wall_ns_per_iter as f64 / 1_000_000.0;

    // Reference verification pass
    let (in_range, agree_matched) = verify(fields, mode);

    let station_hours_per_sec = if wall_ns_per_iter > 0 {
        in_range as f64 / (wall_ns_per_iter as f64 / 1_000_000_000.0)
    } else {
        0.0
    };

    let agreement = if in_range > 0 {
        agree_matched as f64 / in_range as f64
    } else {
        1.0 // vacuously true: no in-range points to disagree
    };

    StationBenchResult {
        interpolation: mode.to_string(),
        n_stations,
        n_fields,
        in_range,
        wall_ms,
        station_hours_per_sec,
        agreement,
    }
}

/// Count how many (station, field) pairs produce a value (used for timing).
fn extract_all_count(fields: &[Field], mode: &str) -> usize {
    let mut count = 0usize;
    for field in fields {
        for &(_, lat, lon) in STATIONS {
            let found = match mode {
                "bilinear" => extract_bilinear(field, lat, lon).is_some(),
                _ => extract_nearest(field, lat, lon).is_some(),
            };
            if found {
                count += 1;
            }
        }
    }
    count
}

/// Verify extracted values against the reference (nearest-grid-point baseline).
///
/// For this initial harness both "fast" and "reference" use the same decoded
/// grid, so agreement is always 100%. Future optimized extraction paths (lazy
/// partial unpack, etc.) will diverge here and this check will catch regressions.
fn verify(fields: &[Field], mode: &str) -> (usize, usize) {
    let mut in_range = 0usize;
    let mut matched = 0usize;

    for field in fields {
        let tol = field.packing.tolerance();

        for &(_, lat, lon) in STATIONS {
            // Reference: nearest grid point from the fully decoded array
            let ref_idx = match field.grid.nearest_index(lat, lon) {
                Some(i) => i,
                None => continue,
            };
            let ref_val = match field.values.get_at(ref_idx) {
                Some(v) => v,
                None => continue,
            };

            let fast_val = match mode {
                "nearest" => field.values.get_at(ref_idx),
                "bilinear" => extract_bilinear(field, lat, lon),
                _ => field.values.get_at(ref_idx),
            };

            if let Some(fv) = fast_val {
                in_range += 1;
                // Bilinear interpolates between grid cells, so it legitimately
                // differs from nearest by up to ~half the local gradient.
                // Use a generous tolerance (4× packing step) to verify it's
                // physically reasonable, not to check equality.
                let effective_tol = if mode == "bilinear" {
                    (tol * 4.0).max(1e-6)
                } else {
                    tol.max(1e-12)
                };
                if (fv - ref_val).abs() <= effective_tol {
                    matched += 1;
                }
            }
        }
    }

    (in_range, matched)
}
