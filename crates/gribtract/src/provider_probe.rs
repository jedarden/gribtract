//! Runtime provider probe — reads `provider-probe.json` and returns the best
//! provider for each NOAA model (GFS, HRRR, NBM, …).
//!
//! The probe file is written by `xtask probe-providers`. At runtime, callers
//! call [`ProviderProbe::load`] to read the cached results.  The file has a
//! 24-hour TTL by default; callers can re-run `xtask probe-providers` to
//! refresh it when it becomes stale.
//!
//! # Example
//! ```no_run
//! use gribtract::ProviderProbe;
//! use std::path::Path;
//!
//! let probe = ProviderProbe::load(Path::new("provider-probe.json")).unwrap();
//! if probe.is_fresh(24 * 3600) {
//!     if let Some(p) = probe.best_provider("gfs") {
//!         println!("Best GFS provider: {p}");
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

// ── Schema ────────────────────────────────────────────────────────────────────

/// Per-(model, provider) probe result as stored in `provider-probe.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResult {
    pub model: String,
    pub provider: String,
    pub head_latency_ms: f64,
    pub throughput_mbs: f64,
    pub score: f64,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ── ProviderProbe ─────────────────────────────────────────────────────────────

/// Loaded provider-probe data.
///
/// Cheaply cloneable; does not hold file handles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProbe {
    /// ISO-8601 timestamp when the probe was run.
    pub timestamp: String,
    /// YYYYMMDD date used to construct probe URLs.
    pub probe_date: String,
    pub results: Vec<ProviderResult>,
    /// Per-model provider ranking (best first, ascending score).
    pub rankings: HashMap<String, Vec<String>>,
}

impl ProviderProbe {
    /// Load and deserialize `provider-probe.json` from `path`.
    ///
    /// Returns `Err` if the file is missing, unreadable, or malformed.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let probe: Self = serde_json::from_str(&contents)?;
        Ok(probe)
    }

    /// Best (lowest-score) provider for `model`, or `None` if the model has no
    /// successful probe entries.
    pub fn best_provider(&self, model: &str) -> Option<&str> {
        self.rankings.get(model)?.first().map(|s| s.as_str())
    }

    /// All providers for `model` in rank order (best first).
    pub fn ranked_providers(&self, model: &str) -> &[String] {
        self.rankings.get(model).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Returns `true` if the probe timestamp is less than `max_age_secs` old.
    ///
    /// The timestamp is parsed as an ISO-8601 UTC datetime. Returns `false` if
    /// parsing fails (conservative: treat unknown age as stale).
    pub fn is_fresh(&self, max_age_secs: u64) -> bool {
        parse_iso8601_secs(&self.timestamp)
            .map(|probe_secs| {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                now.saturating_sub(probe_secs) < max_age_secs
            })
            .unwrap_or(false)
    }

    /// Iterate over all probe results.
    pub fn results(&self) -> &[ProviderResult] {
        &self.results
    }
}

// ── ISO-8601 parse (no external dep) ─────────────────────────────────────────

/// Parse a `YYYY-MM-DDTHH:MM:SSZ` timestamp to Unix seconds (UTC).
///
/// Returns `None` for any parse failure.
fn parse_iso8601_secs(s: &str) -> Option<u64> {
    // Expected: "2026-06-22T05:00:00Z"
    let s = s.trim().trim_end_matches('Z');
    let (date_part, time_part) = s.split_once('T')?;
    let mut date_it = date_part.splitn(3, '-');
    let year: i64 = date_it.next()?.parse().ok()?;
    let month: i64 = date_it.next()?.parse().ok()?;
    let day: i64 = date_it.next()?.parse().ok()?;
    let mut time_it = time_part.splitn(3, ':');
    let hour: i64 = time_it.next()?.parse().ok()?;
    let min: i64 = time_it.next()?.parse().ok()?;
    let sec: i64 = time_it.next()?.trim_end_matches('Z').parse().ok()?;

    // Days since 1970-01-01 via Hinnant's algorithm (inverse of civil_date).
    let days_since_epoch = ymd_to_days(year, month as u32, day as u32)?;
    let total_secs = days_since_epoch * 86400 + hour * 3600 + min * 60 + sec;
    if total_secs < 0 {
        return None;
    }
    Some(total_secs as u64)
}

/// Proleptic Gregorian (year, month, day) → days since 1970-01-01.
/// Returns `None` for obviously invalid inputs.
fn ymd_to_days(y: i64, m: u32, d: u32) -> Option<i64> {
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Hinnant civil_from_days inverse:
    // shift year so March = month 0
    let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let doy = (153 * m as i64 + 2) / 5 + d as i64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    Some(era * 146_097 + doe - 719_468)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ymd_to_days_known() {
        // 1970-01-01 = day 0
        assert_eq!(ymd_to_days(1970, 1, 1), Some(0));
        // 2000-01-01 = 10957
        assert_eq!(ymd_to_days(2000, 1, 1), Some(10957));
        // 2026-06-20 = 20624
        assert_eq!(ymd_to_days(2026, 6, 20), Some(20624));
    }

    #[test]
    fn parse_iso8601_known() {
        // 1970-01-01T00:00:00Z = 0
        assert_eq!(parse_iso8601_secs("1970-01-01T00:00:00Z"), Some(0));
        // 2026-06-22 = day 20626 since epoch (2026-06-20 = 20624, +2 = 20626)
        let days = 20626i64;
        assert_eq!(
            parse_iso8601_secs("2026-06-22T00:00:00Z"),
            Some((days * 86400) as u64)
        );
    }

    #[test]
    fn probe_roundtrip_json() {
        let mut rankings = HashMap::new();
        rankings.insert(
            "gfs".to_string(),
            vec!["noaa-s3".to_string(), "nomads".to_string()],
        );
        let probe = ProviderProbe {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![ProviderResult {
                model: "gfs".into(),
                provider: "noaa-s3".into(),
                head_latency_ms: 45.3,
                throughput_mbs: 12.5,
                score: 45.3 + 1000.0 / 12.5,
                ok: true,
                error: None,
            }],
            rankings,
        };
        let json = serde_json::to_string(&probe).unwrap();
        let decoded: ProviderProbe = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.best_provider("gfs"), Some("noaa-s3"));
        assert_eq!(decoded.best_provider("hrrr"), None);
        assert_eq!(decoded.ranked_providers("gfs"), &["noaa-s3", "nomads"]);
    }

    #[test]
    fn is_fresh_old_timestamp_returns_false() {
        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["noaa-s3".to_string()]);
        let probe = ProviderProbe {
            // Epoch start is definitely stale
            timestamp: "1970-01-01T00:00:00Z".into(),
            probe_date: "19700101".into(),
            results: vec![],
            rankings,
        };
        assert!(!probe.is_fresh(86400), "1970 timestamp should be stale");
    }

    #[test]
    fn parse_iso8601_plausible_seconds() {
        let parsed = parse_iso8601_secs("2026-06-22T05:00:00Z");
        assert!(parsed.is_some(), "must parse successfully");
        let secs = parsed.unwrap();
        // 2026-06-22 is ~56 years after epoch; verify the value is in a sane range.
        let min_expected = 56u64 * 365 * 86400;
        let max_expected = 58u64 * 365 * 86400;
        assert!(
            secs > min_expected && secs < max_expected,
            "parsed seconds {secs} out of expected range"
        );
    }

    #[test]
    fn load_from_temp_file() {
        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["noaa-s3".to_string()]);
        let probe = ProviderProbe {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };
        let json = serde_json::to_string(&probe).unwrap();
        let tmp_file = std::env::temp_dir().join("gribtract-provider-probe-test.json");
        std::fs::write(&tmp_file, &json).unwrap();
        let loaded = ProviderProbe::load(&tmp_file).unwrap();
        assert_eq!(loaded.best_provider("gfs"), Some("noaa-s3"));
        let _ = std::fs::remove_file(&tmp_file);
    }
}
