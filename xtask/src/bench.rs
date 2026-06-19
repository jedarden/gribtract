//! `xtask bench` — measure gribtract decode throughput and write bench-results.json
//! + bench-history.jsonl + dashboard.html.
//! Schema matches docs/plan/plan.md "Benchmark result".

use std::collections::HashMap;
use std::io::Write as _;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use gribtract_testutil::{corpus, diff, golden};
use crate::bench_station;

// ── Schema ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone)]
pub struct BenchRun {
    pub decoder: String,
    /// Workload type: "full-grid" (default) or "station-extract".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload: Option<String>,
    /// DRT template formatted as "5.<n>"; absent for station-extract runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_5x: Option<String>,
    /// Full-grid throughput fields — present for workload=full-grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_sec: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_per_sec: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_points_per_sec: Option<f64>,
    pub wall_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement: Option<f64>,
    /// Station-extract fields — present for workload=station-extract.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_stations: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_fields: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_range: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station_hours_per_sec: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HostInfo {
    pub cpu: String,
    pub cores: u32,
    pub mem_gb: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CorpusInfo {
    pub name: String,
    pub messages: u32,
    pub bytes: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BenchResult {
    pub git_sha: String,
    pub timestamp: String,
    pub host: HostInfo,
    pub corpus: CorpusInfo,
    pub runs: Vec<BenchRun>,
}

// ── Per-template accumulator ──────────────────────────────────────────────────

#[derive(Default)]
struct TemplateAcc {
    messages: u32,
    total_bytes: u64,
    grid_points: u64,
    total_wall_ns: u64,
    agree_attempts: u32,
    agree_matched: u32,
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn run(args: &[String]) {
    let corpus_name = args
        .iter()
        .position(|a| a == "--corpus")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("inline");

    // --workload <full-grid|station-extract>  (default: run all)
    let workload_filter: &str = args
        .iter()
        .position(|a| a == "--workload")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("all");

    eprintln!("xtask bench: corpus={corpus_name} workload={workload_filter}");

    let host = collect_host_info();
    eprintln!("  host: {} ({} cores, {} GB RAM)", host.cpu, host.cores, host.mem_gb);

    let git_sha = get_git_sha();
    eprintln!("  git_sha: {git_sha}");

    let timestamp = get_timestamp();

    let fixtures = corpus::list_fixtures().expect("corpus manifest must load");
    let inline_fixtures: Vec<_> = fixtures.iter().filter(|f| f.storage == "inline").collect();
    eprintln!("  {} inline fixture(s) to bench", inline_fixtures.len());

    let mut by_drt: HashMap<u16, TemplateAcc> = HashMap::new();
    let mut corpus_messages = 0u32;
    let mut corpus_bytes = 0u64;
    // Accumulate all decoded fields for station-extract bench
    let mut all_decoded_fields: Vec<gribtract::Field> = Vec::new();
    let run_full_grid = workload_filter != "station-extract";

    for entry in &inline_fixtures {
        let bytes = match corpus::load(&entry.id) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("  [skip] {}: {}", entry.id, e);
                continue;
            }
        };

        // Decode once to get fields (used by both workloads)
        let fields = match gribtract::decode(&bytes) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("  [decode-err] {}: {}", entry.id, e);
                corpus_messages += 1;
                continue;
            }
        };

        corpus_messages += 1;
        corpus_bytes += bytes.len() as u64;

        // Accumulate for station-extract bench
        all_decoded_fields.extend(fields.iter().cloned());

        if !run_full_grid {
            continue;
        }

        // ── Full-grid timing ──────────────────────────────────────────────────
        // Warmup: run until 10 ms elapse, count iterations
        let mut n_warmup = 0u32;
        let t_warmup = Instant::now();
        loop {
            let _ = gribtract::decode(&bytes);
            n_warmup += 1;
            if t_warmup.elapsed().as_millis() >= 10 {
                break;
            }
        }
        let warmup_ns = t_warmup.elapsed().as_nanos() as f64;
        let ns_per_decode = (warmup_ns / n_warmup as f64).max(1.0);

        // Target ~200 ms of timed work for accuracy
        let target_ns = 200_000_000.0f64;
        let n_timed = ((target_ns / ns_per_decode).ceil() as u32).clamp(10, 200_000);

        let t0 = Instant::now();
        for _ in 0..n_timed {
            let _ = gribtract::decode(&bytes);
        }
        let wall_ns_per_decode = t0.elapsed().as_nanos() as u64 / n_timed as u64;

        // Agreement check against golden (if present)
        let mut agree_attempts = 0u32;
        let mut agree_matched = 0u32;
        if let Ok(Some(golden_fix)) = golden::load_golden(&entry.id) {
            let mut report = diff::CoverageReport::default();
            diff::compare_fixture(&fields, &golden_fix.fields, &mut report);
            for stat in report.by_template.values() {
                agree_attempts += stat.attempts;
                agree_matched += stat.matched;
            }
        }

        // ── Attribute decode time to primary DRT template ─────────────────────
        // Use the most-common DRT in this fixture's fields; break ties low.
        let primary_drt = {
            let mut counts: HashMap<u16, usize> = HashMap::new();
            for f in &fields {
                *counts.entry(f.drt_template).or_default() += 1;
            }
            counts
                .into_iter()
                .max_by_key(|(drt, cnt)| (*cnt, u16::MAX - drt))
                .map(|(drt, _)| drt)
                .unwrap_or(0)
        };

        let total_grid_pts: u64 = fields.iter().map(|f| f.values.len() as u64).sum();

        let acc = by_drt.entry(primary_drt).or_default();
        acc.messages += 1;
        acc.total_bytes += bytes.len() as u64;
        acc.grid_points += total_grid_pts;
        acc.total_wall_ns += wall_ns_per_decode;
        acc.agree_attempts += agree_attempts;
        acc.agree_matched += agree_matched;

        let per_s = 1_000_000_000.0 / wall_ns_per_decode as f64;
        eprintln!(
            "  [ok] {} — {:.1} MB/s ({:.0} µs/msg), {} grid pts, agreement {}/{}",
            entry.id,
            bytes.len() as f64 / 1_000_000.0 * per_s,
            wall_ns_per_decode as f64 / 1_000.0,
            total_grid_pts,
            agree_matched,
            agree_attempts,
        );
    }

    // ── Build runs vec ────────────────────────────────────────────────────────
    let mut runs: Vec<BenchRun> = Vec::new();
    for (drt, acc) in &by_drt {
        if acc.total_wall_ns == 0 {
            continue;
        }
        let wall_s = acc.total_wall_ns as f64 / 1_000_000_000.0;
        let wall_ms = acc.total_wall_ns as f64 / 1_000_000.0;
        let agreement = if acc.agree_attempts > 0 {
            Some(acc.agree_matched as f64 / acc.agree_attempts as f64)
        } else {
            None
        };
        runs.push(BenchRun {
            decoder: "gribtract".to_string(),
            workload: None,
            template_5x: Some(format!("5.{drt}")),
            messages_per_sec: Some(acc.messages as f64 / wall_s),
            mb_per_sec: Some(acc.total_bytes as f64 / 1_000_000.0 / wall_s),
            grid_points_per_sec: Some(acc.grid_points as f64 / wall_s),
            wall_ms,
            agreement,
            interpolation: None,
            n_stations: None,
            n_fields: None,
            in_range: None,
            station_hours_per_sec: None,
        });
    }
    runs.sort_by(|a, b| a.template_5x.cmp(&b.template_5x));

    // ── Station-extract bench ─────────────────────────────────────────────────
    let run_station = !workload_filter.contains(&"full-grid");
    if run_station && !all_decoded_fields.is_empty() {
        eprintln!("xtask bench: station-extract workload");
        let station_results = bench_station::run(&all_decoded_fields);
        for sr in &station_results {
            runs.push(BenchRun {
                decoder: "gribtract".to_string(),
                workload: Some("station-extract".to_string()),
                template_5x: None,
                messages_per_sec: None,
                mb_per_sec: None,
                grid_points_per_sec: None,
                wall_ms: sr.wall_ms,
                agreement: Some(sr.agreement),
                interpolation: Some(sr.interpolation.clone()),
                n_stations: Some(sr.n_stations as u32),
                n_fields: Some(sr.n_fields as u32),
                in_range: Some(sr.in_range as u32),
                station_hours_per_sec: Some(sr.station_hours_per_sec),
            });
        }
    }

    let result = BenchResult {
        git_sha: git_sha.clone(),
        timestamp,
        host,
        corpus: CorpusInfo {
            name: corpus_name.to_string(),
            messages: corpus_messages,
            bytes: corpus_bytes,
        },
        runs,
    };

    // ── Write bench-results.json ──────────────────────────────────────────────
    let json_pretty = serde_json::to_string_pretty(&result).expect("serialize BenchResult");
    std::fs::write("bench-results.json", &json_pretty).expect("write bench-results.json");
    eprintln!("bench-results.json written ({} run entries)", result.runs.len());

    // ── Append to bench-history.jsonl ─────────────────────────────────────────
    let json_line = serde_json::to_string(&result).expect("serialize BenchResult for history");
    let mut history = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("bench-history.jsonl")
        .expect("open bench-history.jsonl");
    writeln!(history, "{json_line}").expect("write bench-history.jsonl");
    eprintln!("bench-history.jsonl appended");

    // ── Write dashboard.html ──────────────────────────────────────────────────
    let history_json = read_history_for_dashboard();
    let dashboard_html = render_dashboard(&json_pretty, &history_json, &git_sha);
    std::fs::write("dashboard.html", &dashboard_html).expect("write dashboard.html");
    eprintln!("dashboard.html written");

    // ── Summary ───────────────────────────────────────────────────────────────
    println!("=== xtask bench summary ===");
    println!("corpus: {} messages, {} bytes", corpus_messages, corpus_bytes);
    for run in &result.runs {
        match run.workload.as_deref() {
            Some("station-extract") => {
                println!(
                    "  station-extract | interp={} | {} stations × {} fields → {:.0} station-hours/s | agreement={}",
                    run.interpolation.as_deref().unwrap_or("?"),
                    run.n_stations.unwrap_or(0),
                    run.n_fields.unwrap_or(0),
                    run.station_hours_per_sec.unwrap_or(0.0),
                    run.agreement.map(|a| format!("{:.1}%", a * 100.0)).unwrap_or_else(|| "n/a".into()),
                );
            }
            _ => {
                println!(
                    "  template {} | decoder={} | {:.1} MB/s | {:.0} msg/s | {:.0} gpts/s | {:.2} ms | agreement={}",
                    run.template_5x.as_deref().unwrap_or("?"),
                    run.decoder,
                    run.mb_per_sec.unwrap_or(0.0),
                    run.messages_per_sec.unwrap_or(0.0),
                    run.grid_points_per_sec.unwrap_or(0.0),
                    run.wall_ms,
                    run.agreement.map(|a| format!("{:.1}%", a * 100.0)).unwrap_or_else(|| "n/a".into()),
                );
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn collect_host_info() -> HostInfo {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let cpu = cpuinfo
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let cores = cpuinfo
        .lines()
        .filter(|l| l.starts_with("processor"))
        .count() as u32;

    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mem_kb: u64 = meminfo
        .lines()
        .find(|l| l.starts_with("MemTotal"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mem_gb = (mem_kb / 1_024 / 1_024) as u32;

    HostInfo { cpu, cores, mem_gb }
}

fn get_git_sha() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn get_timestamp() -> String {
    std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn read_history_for_dashboard() -> String {
    std::fs::read_to_string("bench-history.jsonl")
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(",\n")
}

fn render_dashboard(bench_json: &str, history_csv: &str, git_sha: &str) -> String {
    let short_sha = &git_sha[..git_sha.len().min(8)];
    let template = include_str!("dashboard_template.html");
    template
        .replace("__BENCH_JSON__", bench_json)
        .replace("__HISTORY_CSV__", history_csv)
        .replace("__GIT_SHA__", short_sha)
}
