//! `xtask probe-providers` — probe NOAA data providers and write provider-probe.json.
//!
//! For each (model, provider) pair this command:
//!  1. Fetches the `.idx` index file (small text) for a recent known cycle.
//!  2. Records head latency (ms) and transfer throughput (MB/s).
//!  3. Ranks providers per model by a combined score:
//!     `score = head_latency_ms + 1000.0 / max(throughput_mbs, 0.001)`.
//!
//! Results are written to `provider-probe.json` in the workspace root.
//! The runtime [`gribtract::ProviderProbe`] reads this file (with a 24-hour TTL)
//! to pick the best provider without re-probing on every startup.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read as _;
use std::time::{Duration, Instant};

// ── JSON schema (shared with gribtract::ProviderProbe) ────────────────────────

/// Per-(model, provider) probe result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResult {
    pub model: String,
    pub provider: String,
    /// Time from start of request to receiving response headers (ms).
    pub head_latency_ms: f64,
    /// Transfer throughput for the response body (MB/s).
    pub throughput_mbs: f64,
    /// Combined score; lower is better.
    /// `score = head_latency_ms + 1000.0 / max(throughput_mbs, 0.001)`
    pub score: f64,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Full probe output written to `provider-probe.json`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeOutput {
    /// ISO-8601 timestamp when the probe was run.
    pub timestamp: String,
    /// YYYYMMDD date used to construct probe URLs.
    pub probe_date: String,
    pub results: Vec<ProviderResult>,
    /// Per-model provider ranking (best first, by ascending `score`).
    pub rankings: HashMap<String, Vec<String>>,
}

// ── Provider catalogue ────────────────────────────────────────────────────────

struct ProviderSpec {
    name: &'static str,
    idx_url: &'static str,
}

struct ModelSpec {
    name: &'static str,
    providers: Vec<ProviderSpec>,
    /// Date pattern marker in the URL.  This will be replaced with a real YYYYMMDD.
    date_placeholder: &'static str,
    /// Hour pattern marker in the URL.  Replaced with "00".
    hour_placeholder: &'static str,
}

fn model_catalogue() -> Vec<ModelSpec> {
    vec![
        ModelSpec {
            name: "gfs",
            date_placeholder: "{date}",
            hour_placeholder: "{hour}",
            providers: vec![
                ProviderSpec {
                    name: "noaa-s3",
                    idx_url: "https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.{date}/{hour}/atmos/gfs.t{hour}z.pgrb2.0p25.f000.idx",
                },
                ProviderSpec {
                    name: "gcs",
                    idx_url: "https://storage.googleapis.com/global-forecast-system/gfs.{date}/{hour}/atmos/gfs.t{hour}z.pgrb2.0p25.f000.idx",
                },
                ProviderSpec {
                    name: "nomads",
                    idx_url: "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.{date}/{hour}/atmos/gfs.t{hour}z.pgrb2.0p25.f000.idx",
                },
            ],
        },
        ModelSpec {
            name: "hrrr",
            date_placeholder: "{date}",
            hour_placeholder: "{hour}",
            providers: vec![
                ProviderSpec {
                    name: "noaa-s3",
                    idx_url: "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.{date}/conus/hrrr.t{hour}z.wrfprsf00.grib2.idx",
                },
                ProviderSpec {
                    name: "gcs",
                    idx_url: "https://storage.googleapis.com/high-resolution-rapid-refresh/hrrr.{date}/conus/hrrr.t{hour}z.wrfprsf00.grib2.idx",
                },
                ProviderSpec {
                    name: "nomads",
                    idx_url: "https://nomads.ncep.noaa.gov/pub/data/nccf/com/hrrr/prod/hrrr.{date}/conus/hrrr.t{hour}z.wrfprsf00.grib2.idx",
                },
            ],
        },
        ModelSpec {
            name: "nbm",
            date_placeholder: "{date}",
            hour_placeholder: "{hour}",
            providers: vec![
                ProviderSpec {
                    name: "noaa-s3",
                    idx_url: "https://noaa-nbm-grib2-pds.s3.amazonaws.com/blend.{date}/{hour}/core/blend.t{hour}z.core.f001.co.grib2.idx",
                },
                ProviderSpec {
                    name: "gcs",
                    idx_url: "https://storage.googleapis.com/national-blend-of-models/blend.{date}/{hour}/core/blend.t{hour}z.core.f001.co.grib2.idx",
                },
                ProviderSpec {
                    name: "nomads",
                    idx_url: "https://nomads.ncep.noaa.gov/pub/data/nccf/com/blend/prod/blend.{date}/{hour}/core/blend.t{hour}z.core.f001.co.grib2.idx",
                },
            ],
        },
    ]
}

// ── Date helpers ──────────────────────────────────────────────────────────────

/// Return `YYYYMMDD` for `days_ago` days before today (UTC).
/// Uses a proleptic Gregorian algorithm (Hinnant) — no `chrono` dependency.
pub fn probe_date_str(days_ago: u64) -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = (secs / 86400).saturating_sub(days_ago) as i64;
    let (y, m, d) = civil_date(days);
    format!("{:04}{:02}{:02}", y, m, d)
}

/// Days-since-1970-01-01 → (year, month, day).
/// Algorithm: <https://howardhinnant.github.io/date_algorithms.html>
pub fn civil_date(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era: i64 = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y: i64 = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

// ── HTTP probe ────────────────────────────────────────────────────────────────

const TIMEOUT_SECS: u64 = 10;
const MAX_BODY_BYTES: usize = 512 * 1024; // 512 KB cap for probe reads

fn probe_url(model: &str, provider: &str, url: &str, verbose: bool) -> ProviderResult {
    let t0 = Instant::now();
    let agent = ureq::builder()
        .timeout_connect(Duration::from_secs(5))
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build();

    let call_result = agent.get(url).call();

    let head_latency_ms = t0.elapsed().as_secs_f64() * 1000.0;

    match call_result {
        Err(e) => {
            if verbose {
                eprintln!("  [err] {model}/{provider}: {e}");
            }
            ProviderResult {
                model: model.to_string(),
                provider: provider.to_string(),
                head_latency_ms,
                throughput_mbs: 0.0,
                score: f64::MAX,
                ok: false,
                error: Some(e.to_string()),
            }
        }
        Ok(resp) => {
            let status = resp.status();
            if !(200..300).contains(&status) {
                let err = format!("HTTP {status}");
                if verbose {
                    eprintln!("  [http-err] {model}/{provider}: {err}");
                }
                return ProviderResult {
                    model: model.to_string(),
                    provider: provider.to_string(),
                    head_latency_ms,
                    throughput_mbs: 0.0,
                    score: f64::MAX,
                    ok: false,
                    error: Some(err),
                };
            }

            // Read body to measure throughput
            let t_body = Instant::now();
            let mut buf = Vec::with_capacity(8192);
            let _ = resp
                .into_reader()
                .take(MAX_BODY_BYTES as u64)
                .read_to_end(&mut buf);
            let body_secs = t_body.elapsed().as_secs_f64().max(1e-6);
            let throughput_mbs = buf.len() as f64 / 1_000_000.0 / body_secs;
            let score = compute_score(head_latency_ms, throughput_mbs);

            if verbose {
                eprintln!(
                    "  [ok] {model}/{provider}: head={head_latency_ms:.1}ms  \
                     throughput={throughput_mbs:.2}MB/s  score={score:.1}  body={}B",
                    buf.len()
                );
            }

            ProviderResult {
                model: model.to_string(),
                provider: provider.to_string(),
                head_latency_ms,
                throughput_mbs,
                score,
                ok: true,
                error: None,
            }
        }
    }
}

/// Combined score; lower = better.
pub fn compute_score(head_latency_ms: f64, throughput_mbs: f64) -> f64 {
    head_latency_ms + 1000.0 / throughput_mbs.max(0.001)
}

/// Rank providers per model (ascending score, failures at the end).
pub fn rank_providers(results: &[ProviderResult]) -> HashMap<String, Vec<String>> {
    let mut by_model: HashMap<String, Vec<&ProviderResult>> = HashMap::new();
    for r in results {
        by_model.entry(r.model.clone()).or_default().push(r);
    }

    let mut rankings = HashMap::new();
    for (model, mut entries) in by_model {
        entries.sort_by(|a, b| {
            // Failures sort last; among successes sort by ascending score.
            let a_ok = a.ok as u8;
            let b_ok = b.ok as u8;
            b_ok.cmp(&a_ok)
                .then(a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
        });
        rankings.insert(model, entries.iter().map(|r| r.provider.clone()).collect());
    }
    rankings
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn run(args: &[String]) {
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let out_path = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("provider-probe.json");

    // Use data from 2 days ago (safe window for NOAA archive availability).
    let date = probe_date_str(2);
    let hour = "00";

    eprintln!("xtask probe-providers: probing with date={date} hour={hour}");

    let catalogue = model_catalogue();
    let mut results: Vec<ProviderResult> = Vec::new();

    for model in &catalogue {
        for provider in &model.providers {
            let url = provider
                .idx_url
                .replace(model.date_placeholder, &date)
                .replace(model.hour_placeholder, hour);

            if verbose {
                eprintln!("  probing {}/{}: {url}", model.name, provider.name);
            } else {
                eprintln!("  probing {}/{} ...", model.name, provider.name);
            }

            let result = probe_url(model.name, provider.name, &url, verbose);
            results.push(result);
        }
    }

    let rankings = rank_providers(&results);

    let timestamp = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let output = ProbeOutput {
        timestamp,
        probe_date: date,
        results,
        rankings: rankings.clone(),
    };

    let json = serde_json::to_string_pretty(&output).expect("serialize ProbeOutput");
    std::fs::write(out_path, &json).expect("write provider-probe.json");
    eprintln!("probe results written to {out_path}");

    println!("=== provider-probe rankings ===");
    let mut models: Vec<&String> = rankings.keys().collect();
    models.sort();
    for model in models {
        let providers = &rankings[model];
        println!("  {model}: {}", providers.join(" > "));
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_date_known_dates() {
        // 1970-01-01 = day 0
        assert_eq!(civil_date(0), (1970, 1, 1));
        // 2000-01-01 = 10957 days since epoch
        // (30 years, accounting for leap years)
        assert_eq!(civil_date(10957), (2000, 1, 1));
        // 2026-06-20 = 20624 days since epoch
        assert_eq!(civil_date(20624), (2026, 6, 20));
    }

    #[test]
    fn probe_date_str_is_8_digits() {
        let s = probe_date_str(2);
        assert_eq!(s.len(), 8, "YYYYMMDD must be 8 chars: {s}");
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn compute_score_lower_is_better() {
        let fast = compute_score(10.0, 50.0);
        let slow = compute_score(200.0, 1.0);
        assert!(fast < slow, "fast should score lower than slow");
    }

    #[test]
    fn rank_providers_orders_by_score() {
        let results = vec![
            ProviderResult {
                model: "gfs".into(),
                provider: "slow".into(),
                head_latency_ms: 300.0,
                throughput_mbs: 1.0,
                score: compute_score(300.0, 1.0),
                ok: true,
                error: None,
            },
            ProviderResult {
                model: "gfs".into(),
                provider: "fast".into(),
                head_latency_ms: 20.0,
                throughput_mbs: 30.0,
                score: compute_score(20.0, 30.0),
                ok: true,
                error: None,
            },
            ProviderResult {
                model: "gfs".into(),
                provider: "broken".into(),
                head_latency_ms: 5000.0,
                throughput_mbs: 0.0,
                score: f64::MAX,
                ok: false,
                error: Some("timeout".into()),
            },
        ];
        let rankings = rank_providers(&results);
        let gfs = rankings.get("gfs").unwrap();
        assert_eq!(gfs[0], "fast");
        assert_eq!(gfs[1], "slow");
        assert_eq!(gfs[2], "broken");
    }

    #[test]
    fn probe_output_rankings() {
        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["noaa-s3".to_string(), "gcs".to_string()]);
        let output = ProbeOutput {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };
        // Access rankings directly (ProbeOutput is the xtask-local write struct;
        // the runtime best_provider API lives in gribtract::ProviderProbe).
        assert_eq!(
            output.rankings.get("gfs").and_then(|v| v.first()).map(|s| s.as_str()),
            Some("noaa-s3")
        );
        assert!(output.rankings.get("hrrr").is_none());
    }

    #[test]
    fn roundtrip_json() {
        let mut rankings = HashMap::new();
        rankings.insert(
            "gfs".to_string(),
            vec!["noaa-s3".to_string(), "nomads".to_string()],
        );
        let original = ProbeOutput {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![ProviderResult {
                model: "gfs".into(),
                provider: "noaa-s3".into(),
                head_latency_ms: 45.3,
                throughput_mbs: 12.5,
                score: compute_score(45.3, 12.5),
                ok: true,
                error: None,
            }],
            rankings,
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: ProbeOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.probe_date, "20260620");
        assert_eq!(decoded.results.len(), 1);
        assert_eq!(decoded.results[0].provider, "noaa-s3");
        assert!((decoded.results[0].head_latency_ms - 45.3).abs() < 1e-9);
    }
}
