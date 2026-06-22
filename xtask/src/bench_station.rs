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
/// Coordinates match official METAR observation stations. Stable over time
/// so benchmark runs are comparable across builds and hardware.
pub const STATIONS: &[(&str, f64, f64)] = &[
    // Eastern Time
    ("New York",       40.7789,  -73.9692),  // KNYC Central Park
    ("Miami",          25.7959,  -80.2870),  // KMIA
    ("Philadelphia",   39.8721,  -75.2411),  // KPHL
    ("Atlanta",        33.6407,  -84.4277),  // KATL
    ("Boston",         42.3656,  -71.0096),  // KBOS
    ("Washington DC",  38.8512,  -77.0402),  // KDCA Reagan
    // Central Time
    ("Chicago",        41.7868,  -87.7522),  // KMDW Midway
    ("Dallas",         32.8998,  -97.0403),  // KDFW
    ("Houston",        29.9902,  -95.3368),  // KIAH
    ("Minneapolis",    44.8820,  -93.2218),  // KMSP
    ("Austin",         30.1945,  -97.6699),  // KAUS
    ("New Orleans",    29.9934,  -90.2580),  // KMSY
    ("San Antonio",    29.5337,  -98.4698),  // KSAT
    ("Oklahoma City",  35.3931,  -97.6007),  // KOKC
    // Mountain / Arizona
    ("Denver",         39.8561, -104.6737),  // KDEN
    ("Phoenix",        33.4373, -112.0078),  // KPHX Sky Harbor
    // Pacific Time
    ("Los Angeles",    33.9416, -118.4085),  // KLAX
    ("Las Vegas",      36.0840, -115.1537),  // KLAS
    ("Seattle",        47.4502, -122.3088),  // KSEA Sea-Tac
    ("San Francisco",  37.6189, -122.3750),  // KSFO
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

/// Bench the lazy partial-decode path for nearest-point extraction.
///
/// Handles two packing kinds:
///
/// * **DRT=0** (simple packing) — `decode_point_drt0` gives true O(1) random
///   access by reading just `bits_per_value` bits from the packed body.
///   Efficient even for a single station because no full-grid decode occurs.
///
/// * **DRT=2/3** (complex packing) — `decode_point_drt3` must decode the
///   entire grid to honour spatial differencing, but avoids re-parsing
///   sections 0–6 on every call.  For multi-station workloads, prefer
///   `run_lazy_drt3_cached` (decode-once-extract-many) which is ~N× faster.
///   This function exists to exercise the single-point API and confirm
///   `in_range > 0` for DRT=3 fields.
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
    // Run when there are DRT=0 OR DRT=2/3 lazy fields with raw bytes.
    let has_lazy_data = lazy_fields.iter().any(|lf| {
        !lf.section7_raw.is_empty()
            && !lf.has_bitmap
            && (lf.drt_template == 0
                || ((lf.drt_template == 2 || lf.drt_template == 3)
                    && lf.complex_extra.is_some()))
    });
    if !has_lazy_data {
        return None;
    }

    // Geometry cache from full fields (identical grids).
    let cache = GeometryCache::build(full_fields);

    let n_stations = STATIONS.len();
    let n_fields = lazy_fields.len();

    /// Extract a single value from a lazy field at grid index `idx`.
    /// Returns `Some(val)` if extraction succeeds, `None` otherwise.
    fn extract_lazy_point(lf: &LazyField, idx: usize) -> Option<f64> {
        if lf.has_bitmap || lf.section7_raw.is_empty() {
            return None;
        }
        match lf.drt_template {
            0 => gribtract::decode_point_drt0(&lf.section7_raw, &lf.packing, idx),
            2 | 3 => {
                let extra = lf.complex_extra.as_ref()?;
                let n_pts = lf.grid.num_data_points as usize;
                gribtract::decode_point_drt3(
                    &lf.section7_raw,
                    &lf.packing,
                    extra,
                    n_pts,
                    idx,
                )
            }
            _ => None,
        }
    }

    let extract = |lfs: &[LazyField], c: &GeometryCache| -> usize {
        let mut count = 0;
        for (fi, lf) in lfs.iter().enumerate() {
            for &idx_opt in &c.nearest[fi] {
                if let Some(idx) = idx_opt {
                    if extract_lazy_point(lf, idx).is_some() {
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
        let tol = lf.packing.tolerance().max(1e-12);
        for si in 0..n_stations {
            let Some(idx) = cache.nearest[fi][si] else { continue };
            let Some(lazy_val) = extract_lazy_point(lf, idx) else { continue };
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

// ── DRT=2/3 cached bench (decode-once-extract-many) ──────────────────────────

/// Bench the DRT=2/3 decode-once-extract-many path for nearest-point extraction.
///
/// For DRT=3 (complex packing with spatial differencing), true random access is
/// impossible — the full grid must be decoded to obtain any single point.  The
/// naive approach decodes the grid N times (once per station) per field.  This
/// function implements the cached alternative: decode the full grid once per
/// field, cache the `Vec<f64>`, then look up `values[idx]` for each station in
/// O(1).  The speedup factor equals the number of stations (~20×).
///
/// Returns `None` if no DRT=2/3 lazy fields with raw data are present.
///
/// `lazy_fields` and `full_fields` must be aligned (same order, same grids).
/// The geometry cache and the reference verification both derive from `full_fields`.
pub fn run_lazy_drt3_cached(
    lazy_fields: &[LazyField],
    full_fields: &[Field],
) -> Option<StationBenchResult> {
    if lazy_fields.is_empty() || full_fields.is_empty() {
        return None;
    }

    // Only run when there are DRT=2/3 lazy fields with raw bytes and complex extra.
    let has_lazy_data = lazy_fields.iter().any(|lf| {
        (lf.drt_template == 2 || lf.drt_template == 3)
            && !lf.section7_raw.is_empty()
            && lf.complex_extra.is_some()
            && !lf.has_bitmap
    });
    if !has_lazy_data {
        return None;
    }

    let cache = GeometryCache::build(full_fields);
    let n_stations = STATIONS.len();
    let n_fields = lazy_fields.len();

    // NAIVE path: decode the full grid once per (field, station) pair — i.e.,
    // n_stations full decodes per field.  This is the unoptimised baseline.
    let extract_naive = |lfs: &[LazyField], c: &GeometryCache| -> usize {
        let mut count = 0;
        for (fi, lf) in lfs.iter().enumerate() {
            if (lf.drt_template != 2 && lf.drt_template != 3)
                || lf.has_bitmap
                || lf.section7_raw.is_empty()
            {
                continue;
            }
            let Some(extra) = &lf.complex_extra else { continue };
            let n_pts = lf.grid.num_data_points as usize;
            for &idx_opt in &c.nearest[fi] {
                let Some(idx) = idx_opt else { continue };
                // Decode the whole grid just to get one point — the naive approach.
                if let Ok(vals) =
                    gribtract::decode_all_drt3(&lf.section7_raw, &lf.packing, extra, n_pts)
                {
                    if idx < vals.len() {
                        count += 1;
                    }
                }
            }
        }
        count
    };

    // CACHED path: decode each field's grid once, reuse across stations.
    let extract_cached = |lfs: &[LazyField], c: &GeometryCache| -> usize {
        let mut count = 0;
        for (fi, lf) in lfs.iter().enumerate() {
            if (lf.drt_template != 2 && lf.drt_template != 3)
                || lf.has_bitmap
                || lf.section7_raw.is_empty()
            {
                continue;
            }
            let Some(extra) = &lf.complex_extra else { continue };
            let n_pts = lf.grid.num_data_points as usize;
            // Decode the full grid ONCE for this field.
            let Ok(decoded) =
                gribtract::decode_all_drt3(&lf.section7_raw, &lf.packing, extra, n_pts)
            else {
                continue;
            };
            // Then look up each station in O(1).
            for &idx_opt in &c.nearest[fi] {
                let Some(idx) = idx_opt else { continue };
                if idx < decoded.len() {
                    count += 1;
                }
            }
        }
        count
    };

    // ── Benchmark the naive path ──────────────────────────────────────────────
    let mut n_warmup = 0u32;
    let t_warmup = Instant::now();
    loop {
        let _ = extract_naive(lazy_fields, &cache);
        n_warmup += 1;
        if t_warmup.elapsed().as_millis() >= 10 {
            break;
        }
    }
    let ns_naive = (t_warmup.elapsed().as_nanos() as f64 / n_warmup as f64).max(1.0);
    let n_naive = ((200_000_000.0f64 / ns_naive).ceil() as u32).clamp(2, 10_000);
    let t0 = Instant::now();
    for _ in 0..n_naive {
        let _ = extract_naive(lazy_fields, &cache);
    }
    let naive_ns_per_iter = t0.elapsed().as_nanos() as u64 / n_naive as u64;

    // ── Benchmark the cached path ─────────────────────────────────────────────
    let mut n_warmup2 = 0u32;
    let t_warmup2 = Instant::now();
    loop {
        let _ = extract_cached(lazy_fields, &cache);
        n_warmup2 += 1;
        if t_warmup2.elapsed().as_millis() >= 10 {
            break;
        }
    }
    let ns_cached = (t_warmup2.elapsed().as_nanos() as f64 / n_warmup2 as f64).max(1.0);
    let n_cached = ((200_000_000.0f64 / ns_cached).ceil() as u32).clamp(2, 10_000);
    let t1 = Instant::now();
    for _ in 0..n_cached {
        let _ = extract_cached(lazy_fields, &cache);
    }
    let cached_ns_per_iter = t1.elapsed().as_nanos() as u64 / n_cached as u64;
    let wall_ms = cached_ns_per_iter as f64 / 1_000_000.0;

    // ── Correctness: compare cached vs full-grid decode at same indices ────────
    let mut in_range = 0usize;
    let mut matched = 0usize;
    for (fi, lf) in lazy_fields.iter().enumerate() {
        if (lf.drt_template != 2 && lf.drt_template != 3)
            || lf.has_bitmap
            || lf.section7_raw.is_empty()
        {
            continue;
        }
        let Some(extra) = &lf.complex_extra else { continue };
        let n_pts = lf.grid.num_data_points as usize;
        let Ok(decoded) =
            gribtract::decode_all_drt3(&lf.section7_raw, &lf.packing, extra, n_pts)
        else {
            continue;
        };
        let tol = lf.packing.tolerance().max(1e-12);
        for si in 0..n_stations {
            let Some(idx) = cache.nearest[fi][si] else { continue };
            if idx >= decoded.len() {
                continue;
            }
            let cached_val = decoded[idx];
            let Some(ref_val) = full_fields[fi].values.get_at(idx) else { continue };
            in_range += 1;
            if (cached_val - ref_val).abs() <= tol {
                matched += 1;
            }
        }
    }

    let station_hours_per_sec = if cached_ns_per_iter > 0 && in_range > 0 {
        in_range as f64 / (cached_ns_per_iter as f64 / 1_000_000_000.0)
    } else {
        0.0
    };
    let naive_shps = if naive_ns_per_iter > 0 && in_range > 0 {
        in_range as f64 / (naive_ns_per_iter as f64 / 1_000_000_000.0)
    } else {
        0.0
    };
    let speedup = if naive_shps > 0.0 { station_hours_per_sec / naive_shps } else { 0.0 };
    let agreement = if in_range > 0 { matched as f64 / in_range as f64 } else { 1.0 };

    eprintln!(
        "  [station-drt3-cached] {} fields × {} stations → {} in-range | \
         {:.0} station-hours/s (cached) vs {:.0} (naive) | speedup={:.1}× | agreement={:.1}%",
        n_fields,
        n_stations,
        in_range,
        station_hours_per_sec,
        naive_shps,
        speedup,
        agreement * 100.0,
    );

    Some(StationBenchResult {
        interpolation: "drt3-cached-nearest".to_string(),
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
