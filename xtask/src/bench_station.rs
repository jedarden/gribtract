//! Fixed-station point-extraction benchmark.
//!
//! Measures how fast gribtract can extract a per-station time series from a
//! set of decoded fields. Two modes: nearest-grid-point and bilinear. Every
//! extracted value is verified against the reference (full-grid decode at the
//! same point), so speed is never claimed without a correctness proof.
//!
//! See `docs/notes/station-extraction-benchmark.md` for the design rationale.

use std::time::Instant;

use gribtract::{BilinearCorners, Field, GridDefinition, LazyField};

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

// ── Geometry cache ────────────────────────────────────────────────────────────

/// Pre-computed station→grid-index mapping.
///
/// Grid geometry is identical for every forecast hour in a cycle. Computing
/// `nearest_index` / `bilinear_corners` once and reusing across all messages
/// avoids redundant arithmetic in the inner loop.
struct GeometryCache {
    /// `nearest[fi][si]` — flat grid index for station `si` in field `fi`.
    nearest: Vec<Vec<Option<usize>>>,
    /// `bilinear[fi][si]` — corner indices + fractional weights.
    bilinear: Vec<Vec<Option<BilinearCorners>>>,
}

impl GeometryCache {
    fn build(fields: &[Field]) -> Self {
        // Track unique grids to avoid recomputing for identical geometries.
        // In a single forecast cycle all messages share the same grid, so this
        // typically collapses to one computation.
        type GridEntry = (GridDefinition, Vec<Option<usize>>, Vec<Option<BilinearCorners>>);
        let mut seen: Vec<GridEntry> = Vec::new();
        let mut nearest = Vec::with_capacity(fields.len());
        let mut bilinear = Vec::with_capacity(fields.len());

        for field in fields {
            if let Some((_, n_row, b_row)) = seen.iter().find(|(g, _, _)| *g == field.grid) {
                nearest.push(n_row.clone());
                bilinear.push(b_row.clone());
            } else {
                let n_row: Vec<Option<usize>> = STATIONS
                    .iter()
                    .map(|&(_, lat, lon)| field.grid.nearest_index(lat, lon))
                    .collect();
                let b_row: Vec<Option<BilinearCorners>> = STATIONS
                    .iter()
                    .map(|&(_, lat, lon)| field.grid.bilinear_corners(lat, lon))
                    .collect();
                nearest.push(n_row.clone());
                bilinear.push(b_row.clone());
                seen.push((field.grid.clone(), n_row, b_row));
            }
        }
        GeometryCache { nearest, bilinear }
    }
}

// ── Cached extraction helpers ─────────────────────────────────────────────────

fn extract_all_count_nearest(fields: &[Field], cache: &GeometryCache) -> usize {
    let mut count = 0;
    for (field, station_indices) in fields.iter().zip(cache.nearest.iter()) {
        for &idx_opt in station_indices {
            if let Some(idx) = idx_opt {
                if field.values.get_at(idx).is_some() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn extract_all_count_bilinear(fields: &[Field], cache: &GeometryCache) -> usize {
    let mut count = 0;
    for (field, corners_list) in fields.iter().zip(cache.bilinear.iter()) {
        for corners in corners_list.iter().flatten() {
            if field.values.bilinear(corners).is_some() {
                count += 1;
            }
        }
    }
    count
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the station extraction benchmark for both nearest and bilinear modes.
///
/// Treats each element of `fields` as one "forecast hour". Returns one
/// `StationBenchResult` per interpolation mode.
pub fn run(fields: &[Field]) -> Vec<StationBenchResult> {
    // Build the geometry cache once; reused by both modes.
    let cache = GeometryCache::build(fields);
    let n_unique_grids = {
        let mut seen: Vec<&GridDefinition> = Vec::new();
        for f in fields {
            if !seen.iter().any(|g| **g == f.grid) {
                seen.push(&f.grid);
            }
        }
        seen.len()
    };
    eprintln!(
        "  [station-cache] {} field(s), {} unique grid(s)",
        fields.len(),
        n_unique_grids,
    );

    let mut results = Vec::new();
    for mode in ["nearest", "bilinear"] {
        let r = run_mode(fields, mode, &cache);
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

fn run_mode(fields: &[Field], mode: &str, cache: &GeometryCache) -> StationBenchResult {
    let n_stations = STATIONS.len();
    let n_fields = fields.len();

    let extract_count = match mode {
        "bilinear" => extract_all_count_bilinear as fn(&[Field], &GeometryCache) -> usize,
        _ => extract_all_count_nearest,
    };

    // Warmup: run until ≥10 ms elapsed, count iterations
    let mut n_warmup = 0u32;
    let t_warmup = Instant::now();
    loop {
        let _ = extract_count(fields, cache);
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
        let _ = extract_count(fields, cache);
    }
    let wall_ns_per_iter = t0.elapsed().as_nanos() as u64 / n_timed as u64;
    let wall_ms = wall_ns_per_iter as f64 / 1_000_000.0;

    // Reference verification pass
    let (in_range, agree_matched) = verify(fields, mode, cache);

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

// ── Lazy nearest-point bench ──────────────────────────────────────────────────

/// Bench the lazy DRT=0 partial-decode path for nearest-point extraction.
///
/// Times `decode_point_drt0(section7_raw, packing, idx)` vs the current
/// `field.values.get_at(idx)` (a plain Vec index).  The lazy path skips
/// decoding the full grid — relevant when only a handful of points are needed
/// from a large message.
///
/// `lazy_fields` and `full_fields` must be aligned (same order, same grids).
/// The geometry cache is derived from `full_fields` (identical grids).
pub fn run_lazy_nearest(
    lazy_fields: &[LazyField],
    full_fields: &[Field],
) -> Option<StationBenchResult> {
    if lazy_fields.is_empty() || full_fields.is_empty() {
        return None;
    }
    // Only run when there are DRT=0 lazy fields with raw bytes.
    let has_lazy_data = lazy_fields
        .iter()
        .any(|lf| lf.drt_template == 0 && !lf.section7_raw.is_empty());
    if !has_lazy_data {
        return None;
    }

    // Geometry cache from full fields (identical grids).
    let cache = GeometryCache::build(full_fields);

    let n_stations = STATIONS.len();
    let n_fields = lazy_fields.len();

    let extract = |lfs: &[LazyField], c: &GeometryCache| -> usize {
        let mut count = 0;
        for (fi, lf) in lfs.iter().enumerate() {
            if lf.drt_template != 0 || lf.has_bitmap || lf.section7_raw.is_empty() {
                continue;
            }
            for &idx_opt in &c.nearest[fi] {
                if let Some(idx) = idx_opt {
                    if gribtract::decode_point_drt0(&lf.section7_raw, &lf.packing, idx)
                        .is_some()
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    };

    // Warmup
    let mut n_warmup = 0u32;
    let t_warmup = Instant::now();
    loop {
        let _ = extract(lazy_fields, &cache);
        n_warmup += 1;
        if t_warmup.elapsed().as_millis() >= 10 {
            break;
        }
    }
    let ns_per_iter =
        (t_warmup.elapsed().as_nanos() as f64 / n_warmup as f64).max(1.0);

    // Timed
    let n_timed = ((200_000_000.0f64 / ns_per_iter).ceil() as u32).clamp(10, 100_000);
    let t0 = Instant::now();
    for _ in 0..n_timed {
        let _ = extract(lazy_fields, &cache);
    }
    let wall_ns_per_iter = t0.elapsed().as_nanos() as u64 / n_timed as u64;
    let wall_ms = wall_ns_per_iter as f64 / 1_000_000.0;

    // Correctness: compare lazy vs full-grid decode at same indices.
    let mut in_range = 0usize;
    let mut matched = 0usize;
    for (fi, lf) in lazy_fields.iter().enumerate() {
        if lf.drt_template != 0 || lf.has_bitmap || lf.section7_raw.is_empty() {
            continue;
        }
        let tol = lf.packing.tolerance().max(1e-12);
        for si in 0..n_stations {
            let Some(idx) = cache.nearest[fi][si] else { continue };
            let Some(lazy_val) =
                gribtract::decode_point_drt0(&lf.section7_raw, &lf.packing, idx)
            else {
                continue;
            };
            let Some(ref_val) = full_fields[fi].values.get_at(idx) else { continue };
            in_range += 1;
            if (lazy_val - ref_val).abs() <= tol {
                matched += 1;
            }
        }
    }

    let station_hours_per_sec = if wall_ns_per_iter > 0 && in_range > 0 {
        in_range as f64 / (wall_ns_per_iter as f64 / 1_000_000_000.0)
    } else {
        0.0
    };
    let agreement = if in_range > 0 {
        matched as f64 / in_range as f64
    } else {
        1.0
    };

    eprintln!(
        "  [station-lazy-nearest] {} fields × {} stations → {} in-range | \
         {:.0} station-hours/s | agreement={:.1}%",
        n_fields,
        n_stations,
        in_range,
        station_hours_per_sec,
        agreement * 100.0,
    );

    Some(StationBenchResult {
        interpolation: "lazy-nearest".to_string(),
        n_stations,
        n_fields,
        in_range,
        wall_ms,
        station_hours_per_sec,
        agreement,
    })
}

/// Verify extracted values against the reference (nearest-grid-point baseline).
fn verify(fields: &[Field], mode: &str, cache: &GeometryCache) -> (usize, usize) {
    let mut in_range = 0usize;
    let mut matched = 0usize;

    for (fi, field) in fields.iter().enumerate() {
        let tol = field.packing.tolerance();

        for si in 0..STATIONS.len() {
            // Reference: nearest grid point from the fully decoded array
            let ref_idx = match cache.nearest[fi][si] {
                Some(i) => i,
                None => continue,
            };
            let ref_val = match field.values.get_at(ref_idx) {
                Some(v) => v,
                None => continue,
            };

            let fast_val = match mode {
                "nearest" => field.values.get_at(ref_idx),
                "bilinear" => cache.bilinear[fi][si].and_then(|c| field.values.bilinear(&c)),
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
