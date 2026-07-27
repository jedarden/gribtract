//! Provider probing and selection

use crate::client::FetchClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Result of probing a single provider for a specific model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    /// The provider that was probed
    pub provider: String,
    /// URL used for probing (typically a recent .idx file)
    pub probe_url: String,
    /// TCP connect time + TTFB for the probe request
    pub connect_ms: u64,
    /// Time to first byte (TTFB) in milliseconds
    pub ttfb_ms: u64,
    /// Throughput measured during the probe (bytes/sec)
    pub throughput_mbs: f64,
    /// Combined score (lower is better)
    pub score: f64,
    /// Whether the probe was successful
    pub success: bool,
    /// Error message if the probe failed
    pub error: Option<String>,
    /// Timestamp of the probe
    pub timestamp: String,
}

/// Provider probe results for all providers and models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProbeResults {
    /// Results per model
    pub models: HashMap<String, Vec<ProbeResult>>,
    /// Timestamp when the probe was run
    pub timestamp: String,
    /// Git commit hash for reproducibility
    pub git_sha: Option<String>,
}

/// Provider probe client
///
/// Probes each provider for each model at startup to determine the fastest provider
/// for the current deployment location.
#[derive(Debug)]
pub struct ProviderProbe {
    client: FetchClient,
    /// Test files to use for probing (small .idx files)
    probe_files: HashMap<String, Vec<(String, String)>>,
    /// Consecutive failure count per provider (provider name -> failure count)
    consecutive_failures: HashMap<String, u32>,
    /// Threshold for consecutive failures before re-probing is needed
    consecutive_failure_threshold: u32,
}

impl ProviderProbe {
    /// Create a new provider probe client
    pub fn new() -> Self {
        Self {
            client: FetchClient::new(),
            probe_files: Self::default_probe_files(),
            consecutive_failures: HashMap::new(),
            consecutive_failure_threshold: 3, // Default threshold
        }
    }

    /// Create a new provider probe client with a custom HTTP client
    pub fn with_client(client: FetchClient) -> Self {
        Self {
            client,
            probe_files: Self::default_probe_files(),
            consecutive_failures: HashMap::new(),
            consecutive_failure_threshold: 3, // Default threshold
        }
    }

    /// Create a new provider probe client with a custom failure threshold
    pub fn with_threshold(mut self, threshold: u32) -> Self {
        self.consecutive_failure_threshold = threshold;
        self
    }

    /// Get default probe files for each model
    ///
    /// Returns a map of model name to list of (provider, url) tuples
    fn default_probe_files() -> HashMap<String, Vec<(String, String)>> {
        let mut files = HashMap::new();

        // Use data from 2 days ago (safe window for NOAA archive availability)
        let date = crate::utils::probe_date_str(2);
        let hour = "00";

        // HRRR model probe files
        files.insert(
            "hrrr".to_string(),
            vec![
                ("s3:hrrr-bdp".to_string(), format!("https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.{date}/conus/hrrr.t{hour}z.wrfsfcf00.grib2.idx")),
                ("gcs:hrrr".to_string(), format!("https://storage.googleapis.com/high-resolution-rapid-refresh/hrrr.{date}/conus/hrrr.t{hour}z.wrfsfcf00.grib2.idx")),
            ],
        );

        // GEFS model probe files
        files.insert(
            "gefs".to_string(),
            vec![
                ("s3:gefs-pds".to_string(), format!("https://noaa-gefs-pds.s3.amazonaws.com/gefs.{date}/{hour}/wave/grb2/gefs.wave.t{hour}z.prtcgrb.idx.0p25")),
                ("gcs:gefs".to_string(), format!("https://storage.googleapis.com/gfs-ensemble-forecast-system/gefs.{date}/{hour}/wave/grb2/gefs.wave.t{hour}z.prtcgrb.idx.0p25")),
            ],
        );

        // NBM model probe files
        files.insert(
            "nbm".to_string(),
            vec![
                ("s3:nbm-grib2".to_string(), format!("https://noaa-nbm-grib2-pds.s3.amazonaws.com/nbm.{date}/nbm.t{hour}z.conusnest.2p5.f000.co.grib2.idx")),
                ("gcs:nbm".to_string(), format!("https://storage.googleapis.com/national-blend-of-models/nbm.{date}/nbm.t{hour}z.conusnest.2p5.f000.co.grib2.idx")),
            ],
        );

        // GFS model probe files
        files.insert(
            "gfs".to_string(),
            vec![
                ("s3:gfs-pds".to_string(), format!("https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.{date}/{hour}/gfs.t{hour}z.pgrb2.0p25.f000.idx")),
                ("nomads:gfs".to_string(), format!("https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.{date}/{hour}/atmos/gfs.t{hour}z.pgrb2.0p25.f000.idx")),
            ],
        );

        files
    }

    /// Probe all providers for all models
    pub async fn probe_all(&self) -> ProviderProbeResults {
        let mut results = HashMap::new();
        let timestamp = chrono::Utc::now().to_rfc3339();

        for (model, probe_files) in &self.probe_files {
            let mut model_results = Vec::new();

            for (provider, url) in probe_files {
                let result = self.probe_url(provider, url).await;
                model_results.push(result);
            }

            // Sort by score (lower is better)
            model_results.sort_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)
            });

            results.insert(model.clone(), model_results);
        }

        ProviderProbeResults {
            models: results,
            timestamp,
            git_sha: Self::get_git_sha(),
        }
    }

    /// Probe a single URL
    async fn probe_url(&self, provider: &str, url: &str) -> ProbeResult {
        let start = std::time::Instant::now();

        match self.probe_url_inner(url).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                ProbeResult {
                    provider: provider.to_string(),
                    probe_url: url.to_string(),
                    connect_ms: result.connect_time.as_millis() as u64,
                    ttfb_ms: result.ttfb.as_millis() as u64,
                    throughput_mbs: result.throughput_mbs,
                    score: Self::calculate_score(&result),
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            }
            Err(e) => {
                let _elapsed = start.elapsed();
                ProbeResult {
                    provider: provider.to_string(),
                    probe_url: url.to_string(),
                    connect_ms: _elapsed.as_millis() as u64,
                    ttfb_ms: _elapsed.as_millis() as u64,
                    throughput_mbs: 0.0,
                    score: f64::MAX,
                    success: false,
                    error: Some(e.to_string()),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            }
        }
    }

    /// Inner probe logic for a single URL
    async fn probe_url_inner(&mut self, url: &str) -> Result<ProbeInnerResult, crate::FetchError> {
        let start = std::time::Instant::now();

        // Use the client's probe method to get basic connection info
        let probe_info = self.client.probe(url).await?;
        let connect_time = probe_info.connect_time;

        // Check if range requests are supported
        let supports_range = probe_info.supports_range;

        // Get content length
        let content_length = probe_info.content_length.unwrap_or(0);

        // Now do a small range request to measure TTFB and throughput
        let range_start = std::time::Instant::now();

        let range_response = if supports_range && content_length > 1024 {
            Some(self.client.fetch_head(url, 1024).await?)
        } else {
            None
        };

        let ttfb = range_start.elapsed();

        let throughput_mbs = if let Some(resp) = range_response {
            let bytes = resp.data.len() as f64;
            let seconds = ttfb.as_secs_f64();
            (bytes / seconds) / (1024.0 * 1024.0)
        } else {
            0.0
        };

        Ok(ProbeInnerResult {
            connect_time,
            ttfb,
            throughput_mbs,
        })
    }

    /// Calculate a combined score for provider selection
    ///
    /// Lower is better. Formula: connect_ms + ttfb_ms + (1 / throughput_mbs)
    fn calculate_score(result: &ProbeInnerResult) -> f64 {
        let connect_penalty = result.connect_time.as_millis() as f64;
        let ttfb_penalty = result.ttfb.as_millis() as f64;
        let throughput_penalty = if result.throughput_mbs > 0.0 {
            1000.0 / result.throughput_mbs
        } else {
            10000.0 // Large penalty for no throughput
        };

        connect_penalty + ttfb_penalty + throughput_penalty
    }

    /// Get the current git SHA for reproducibility
    fn get_git_sha() -> Option<String> {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !sha.is_empty() {
                    Some(sha)
                } else {
                    None
                }
            })
    }

    /// Get the best provider for a model based on probe results
    pub fn get_best_provider<'a>(
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        results.models.get(model).and_then(|model_results| {
            model_results.iter().find(|r| r.success)
        })
    }

    /// Write probe results to a JSON file
    pub fn write_results(results: &ProviderProbeResults, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(path, json)
    }

    /// Load probe results from a JSON file
    pub fn load_results(path: &std::path::Path) -> std::io::Result<ProviderProbeResults> {
        let json = std::fs::read_to_string(path)?;
        let results: ProviderProbeResults = serde_json::from_str(&json)?;
        Ok(results)
    }

    /// Check if probe results are stale (older than 24 hours)
    pub fn is_stale(results: &ProviderProbeResults, max_age: Duration) -> bool {
        match chrono::DateTime::parse_from_rfc3339(&results.timestamp) {
            Ok(timestamp) => {
                let now = chrono::Utc::now();
                let age = now.signed_duration_since(timestamp);
                age.to_std().unwrap_or(Duration::ZERO) > max_age
            }
            Err(_) => true, // Invalid timestamp means stale
        }
    }

    /// Record a failure for the given provider
    ///
    /// Increments the consecutive failure counter for the provider.
    /// Returns the current failure count after incrementing.
    pub fn record_failure(&mut self, provider: &str) -> u32 {
        let count = self.consecutive_failures.entry(provider.to_string()).or_insert(0);
        *count += 1;
        *count
    }

    /// Record a success for the given provider
    ///
    /// Resets the consecutive failure counter for the provider to zero.
    pub fn record_success(&mut self, provider: &str) {
        self.consecutive_failures.insert(provider.to_string(), 0);
    }

    /// Check if a provider should be re-probed due to consecutive failures
    ///
    /// Returns true if the provider has exceeded the consecutive failure threshold.
    /// This should be checked alongside the staleness check when deciding whether
    /// to refresh provider probe results.
    pub fn should_reprobe(&self, provider: &str) -> bool {
        self.consecutive_failures
            .get(provider)
            .map(|&count| count >= self.consecutive_failure_threshold)
            .unwrap_or(false)
    }

    /// Get the current consecutive failure count for a provider
    pub fn failure_count(&self, provider: &str) -> u32 {
        self.consecutive_failures.get(provider).copied().unwrap_or(0)
    }

    /// Get the consecutive failure threshold
    pub fn failure_threshold(&self) -> u32 {
        self.consecutive_failure_threshold
    }

    /// Reset all failure counters
    pub fn reset_failures(&mut self) {
        self.consecutive_failures.clear();
    }
}

impl Default for ProviderProbe {
    fn default() -> Self {
        Self::new()
    }
}

/// Inner result from a single probe
#[derive(Debug)]
struct ProbeInnerResult {
    connect_time: Duration,
    ttfb: Duration,
    throughput_mbs: f64,
}

/// Runtime tracker for consecutive provider failures
///
/// This is a lightweight struct that can be used at runtime to track HTTP request
/// failures per provider and determine when re-probing is needed. It should be used
/// alongside the loaded provider-probe.json results.
///
/// # Example
/// ```no_run
/// use gribtract_fetch::probe::ProviderFailureTracker;
///
/// let mut tracker = ProviderFailureTracker::new(3); // threshold = 3
///
/// // After a failed HTTP request to a provider
/// tracker.record_failure("s3:hrrr-bdp");
///
/// // Before using a provider, check if it should be re-probed
/// if tracker.should_reprobe("s3:hrrr-bdp") {
///     // Trigger re-probing to find a better provider
/// }
///
/// // After a successful request, reset the counter
/// tracker.record_success("s3:hrrr-bdp");
/// ```
#[derive(Debug, Clone)]
pub struct ProviderFailureTracker {
    /// Consecutive failure count per provider
    failures: std::collections::HashMap<String, u32>,
    /// Threshold for consecutive failures before re-probing
    threshold: u32,
}

impl ProviderFailureTracker {
    /// Create a new failure tracker with the specified threshold
    pub fn new(threshold: u32) -> Self {
        Self {
            failures: std::collections::HashMap::new(),
            threshold,
        }
    }

    /// Create a new failure tracker with default threshold (3)
    pub fn default_threshold() -> Self {
        Self::new(3)
    }

    /// Record a failure for the given provider
    ///
    /// Returns the current failure count after incrementing.
    pub fn record_failure(&mut self, provider: &str) -> u32 {
        let count = self.failures.entry(provider.to_string()).or_insert(0);
        *count += 1;
        *count
    }

    /// Record a success for the given provider
    ///
    /// Resets the consecutive failure counter for the provider to zero.
    pub fn record_success(&mut self, provider: &str) {
        self.failures.insert(provider.to_string(), 0);
    }

    /// Check if a provider should be re-probed due to consecutive failures
    ///
    /// Returns true if the provider has exceeded the consecutive failure threshold.
    /// This should be checked alongside the staleness check when deciding whether
    /// to refresh provider probe results.
    pub fn should_reprobe(&self, provider: &str) -> bool {
        self.failures
            .get(provider)
            .map(|&count| count >= self.threshold)
            .unwrap_or(false)
    }

    /// Get the current consecutive failure count for a provider
    pub fn failure_count(&self, provider: &str) -> u32 {
        self.failures.get(provider).copied().unwrap_or(0)
    }

    /// Get the consecutive failure threshold
    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Reset all failure counters (e.g., after re-probing completes)
    pub fn reset_all(&mut self) {
        self.failures.clear();
    }

    /// Reset failure counter for a specific provider
    pub fn reset_provider(&mut self, provider: &str) {
        self.failures.insert(provider.to_string(), 0);
    }
}

#[cfg(test)]
mod failure_tracker_tests {
    use super::*;

    #[test]
    fn test_failure_tracker_basic() {
        let mut tracker = ProviderFailureTracker::new(3);

        // Initially, no failures recorded
        assert_eq!(tracker.failure_count("s3:hrrr"), 0);
        assert!(!tracker.should_reprobe("s3:hrrr"));

        // Record first failure
        assert_eq!(tracker.record_failure("s3:hrrr"), 1);
        assert_eq!(tracker.failure_count("s3:hrrr"), 1);
        assert!(!tracker.should_reprobe("s3:hrrr"));

        // Record second failure
        assert_eq!(tracker.record_failure("s3:hrrr"), 2);
        assert_eq!(tracker.failure_count("s3:hrrr"), 2);
        assert!(!tracker.should_reprobe("s3:hrrr"));

        // Record third failure - reaches threshold
        assert_eq!(tracker.record_failure("s3:hrrr"), 3);
        assert_eq!(tracker.failure_count("s3:hrrr"), 3);
        assert!(tracker.should_reprobe("s3:hrrr"));

        // Record a success - should reset the counter
        tracker.record_success("s3:hrrr");
        assert_eq!(tracker.failure_count("s3:hrrr"), 0);
        assert!(!tracker.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_failure_tracker_multiple_providers() {
        let mut tracker = ProviderFailureTracker::new(2);

        // Record failures for provider A
        assert_eq!(tracker.record_failure("s3:hrrr"), 1);
        assert_eq!(tracker.record_failure("s3:hrrr"), 2);
        assert!(tracker.should_reprobe("s3:hrrr"));

        // Provider B should not be affected
        assert_eq!(tracker.failure_count("gcs:hrrr"), 0);
        assert!(!tracker.should_reprobe("gcs:hrrr"));

        // Record failures for provider B
        assert_eq!(tracker.record_failure("gcs:hrrr"), 1);
        assert!(!tracker.should_reprobe("gcs:hrrr"));

        // Provider A should still be at threshold
        assert!(tracker.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_failure_tracker_default_threshold() {
        let tracker = ProviderFailureTracker::default_threshold();
        assert_eq!(tracker.threshold(), 3);
    }

    #[test]
    fn test_failure_tracker_reset_all() {
        let mut tracker = ProviderFailureTracker::new(3);

        tracker.record_failure("s3:hrrr");
        tracker.record_failure("gcs:hrrr");
        tracker.record_failure("s3:gefs");

        assert_eq!(tracker.failure_count("s3:hrrr"), 1);
        assert_eq!(tracker.failure_count("gcs:hrrr"), 1);
        assert_eq!(tracker.failure_count("s3:gefs"), 1);

        tracker.reset_all();

        assert_eq!(tracker.failure_count("s3:hrrr"), 0);
        assert_eq!(tracker.failure_count("gcs:hrrr"), 0);
        assert_eq!(tracker.failure_count("s3:gefs"), 0);
    }

    #[test]
    fn test_failure_tracker_reset_provider() {
        let mut tracker = ProviderFailureTracker::new(3);

        tracker.record_failure("s3:hrrr");
        tracker.record_failure("gcs:hrrr");

        tracker.reset_provider("s3:hrrr");

        assert_eq!(tracker.failure_count("s3:hrrr"), 0);
        assert_eq!(tracker.failure_count("gcs:hrrr"), 1);
    }

    #[test]
    fn test_failure_tracker_clone() {
        let mut tracker = ProviderFailureTracker::new(2);
        tracker.record_failure("s3:hrrr");
        tracker.record_failure("gcs:hrrr");

        let cloned = tracker.clone();
        assert_eq!(cloned.failure_count("s3:hrrr"), 1);
        assert_eq!(cloned.failure_count("gcs:hrrr"), 1);
        assert_eq!(cloned.threshold(), 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_probe_hrrr() {
        let probe = ProviderProbe::new();
        let results = probe.probe_all().await;

        println!("Probe results: {}", serde_json::to_string_pretty(&results).unwrap());

        // Check that we got results for HRRR
        assert!(results.models.contains_key("hrrr"));

        let hrrr_results = &results.models["hrrr"];
        assert!(!hrrr_results.is_empty());

        // At least one provider should succeed
        let successful = hrrr_results.iter().filter(|r| r.success).count();
        assert!(successful > 0, "At least one HRRR provider should succeed");
    }

    #[test]
    fn test_score_calculation() {
        let result = ProbeInnerResult {
            connect_time: Duration::from_millis(50),
            ttfb: Duration::from_millis(100),
            throughput_mbs: 10.0,
        };

        let score = ProviderProbe::calculate_score(&result);
        // connect_penalty = 50, ttfb_penalty = 100, throughput_penalty = 1000/10 = 100
        // total = 250
        assert!((score - 250.0).abs() < 1.0);
    }

    #[test]
    fn test_consecutive_failure_tracking() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Initially, no failures recorded
        assert_eq!(probe.failure_count("s3:hrrr"), 0);
        assert!(!probe.should_reprobe("s3:hrrr"));

        // Record first failure
        assert_eq!(probe.record_failure("s3:hrrr"), 1);
        assert_eq!(probe.failure_count("s3:hrrr"), 1);
        assert!(!probe.should_reprobe("s3:hrrr"));

        // Record second failure
        assert_eq!(probe.record_failure("s3:hrrr"), 2);
        assert_eq!(probe.failure_count("s3:hrrr"), 2);
        assert!(!probe.should_reprobe("s3:hrrr"));

        // Record third failure - reaches threshold
        assert_eq!(probe.record_failure("s3:hrrr"), 3);
        assert_eq!(probe.failure_count("s3:hrrr"), 3);
        assert!(probe.should_reprobe("s3:hrrr"));

        // Record a success - should reset the counter
        probe.record_success("s3:hrrr");
        assert_eq!(probe.failure_count("s3:hrrr"), 0);
        assert!(!probe.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_multiple_providers_independent() {
        let mut probe = ProviderProbe::new().with_threshold(2);

        // Record failures for provider A
        assert_eq!(probe.record_failure("s3:hrrr"), 1);
        assert_eq!(probe.record_failure("s3:hrrr"), 2);
        assert!(probe.should_reprobe("s3:hrrr"));

        // Provider B should not be affected
        assert_eq!(probe.failure_count("gcs:hrrr"), 0);
        assert!(!probe.should_reprobe("gcs:hrrr"));

        // Record failures for provider B
        assert_eq!(probe.record_failure("gcs:hrrr"), 1);
        assert!(!probe.should_reprobe("gcs:hrrr"));

        // Provider A should still be at threshold
        assert!(probe.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_custom_failure_threshold() {
        let probe = ProviderProbe::new().with_threshold(5);

        // Record 4 failures
        let mut probe = probe;
        for _ in 0..4 {
            probe.record_failure("s3:hrrr");
        }

        assert_eq!(probe.failure_count("s3:hrrr"), 4);
        assert!(!probe.should_reprobe("s3:hrrr"));

        // Record 5th failure - reaches custom threshold
        probe.record_failure("s3:hrrr");
        assert!(probe.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_reset_failures() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Record some failures
        probe.record_failure("s3:hrrr");
        probe.record_failure("gcs:hrrr");
        probe.record_failure("s3:gefs");

        assert_eq!(probe.failure_count("s3:hrrr"), 1);
        assert_eq!(probe.failure_count("gcs:hrrr"), 1);
        assert_eq!(probe.failure_count("s3:gefs"), 1);

        // Reset all failures
        probe.reset_failures();

        assert_eq!(probe.failure_count("s3:hrrr"), 0);
        assert_eq!(probe.failure_count("gcs:hrrr"), 0);
        assert_eq!(probe.failure_count("s3:gefs"), 0);
    }

    #[test]
    fn test_default_probe_files_uses_dynamic_dates() {
        let files = ProviderProbe::default_probe_files();

        // Verify that no hardcoded "20250702" string remains in any URL
        for (model, probe_list) in &files {
            for (provider, url) in probe_list {
                assert!(
                    !url.contains("20250702"),
                    "Probe URL for {model}/{provider} should not contain hardcoded date '20250702': {url}"
                );
            }
        }

        // Verify that URLs contain a dynamic date (current format check)
        // by using our probe_date_str to generate expected date patterns
        let two_days_ago = crate::utils::probe_date_str(2);
        let one_day_ago = crate::utils::probe_date_str(1);
        let today = crate::utils::probe_date_str(0);

        for (model, probe_list) in &files {
            for (provider, url) in probe_list {
                // URLs should contain a date (one of the recent dates)
                let has_recent_date = url.contains(&two_days_ago) ||
                                     url.contains(&one_day_ago) ||
                                     url.contains(&today);

                assert!(
                    has_recent_date,
                    "Probe URL for {model}/{provider} should contain a recent date: {url}"
                );

                // Verify the URL contains a date-like pattern (8 digits)
                // by checking if it contains any of our generated dates
                let has_date_pattern = url.contains(&two_days_ago) ||
                                     url.contains(&one_day_ago) ||
                                     url.contains(&today);
                assert!(has_date_pattern, "URL should contain date pattern: {url}");
            }
        }

        // Verify we have the expected models
        assert!(files.contains_key("hrrr"));
        assert!(files.contains_key("gefs"));
        assert!(files.contains_key("nbm"));
        assert!(files.contains_key("gfs"));

        // Verify each model has at least one probe file
        assert!(!files["hrrr"].is_empty());
        assert!(!files["gefs"].is_empty());
        assert!(!files["nbm"].is_empty());
        assert!(!files["gfs"].is_empty());
    }
}
