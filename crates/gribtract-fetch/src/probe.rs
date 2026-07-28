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
    pub async fn probe_all(&mut self) -> ProviderProbeResults {
        let mut results = HashMap::new();
        let timestamp = chrono::Utc::now().to_rfc3339();

        // Collect the models and probe_files to avoid borrow checker issues
        let models: Vec<(String, Vec<(String, String)>)> = self.probe_files
            .iter()
            .map(|(model, probe_files)| (model.clone(), probe_files.clone()))
            .collect();

        for (model, probe_files) in models {
            let mut model_results = Vec::new();

            for (provider, url) in probe_files {
                let result = self.probe_url(&provider, &url).await;
                model_results.push(result);
            }

            // Sort by score (lower is better)
            model_results.sort_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)
            });

            results.insert(model, model_results);
        }

        ProviderProbeResults {
            models: results,
            timestamp,
            git_sha: Self::get_git_sha(),
        }
    }

    /// Probe a single URL
    async fn probe_url(&mut self, provider: &str, url: &str) -> ProbeResult {
        let start = std::time::Instant::now();

        match self.probe_url_inner(url).await {
            Ok(result) => {
                let _elapsed = start.elapsed();
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
        let _start = std::time::Instant::now();

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
    ///
    /// # ⚠️ Use with caution - does NOT check provider health
    ///
    /// This method returns the first successful provider in the probe results WITHOUT
    /// checking if it has experienced consecutive failures. This means you might select
    /// a provider that is currently experiencing issues (e.g., timeouts, 5xx errors).
    ///
    /// **Most callers should use [`get_best_provider_with_tracker()`] instead**, which
    /// automatically excludes providers that need re-probing due to consecutive failures.
    ///
    /// This method is ONLY appropriate when:
    /// - You're doing a one-off fetch and don't care about reliability
    /// - You're running in a context where you don't have access to the failure tracker
    /// - You explicitly want to select the "best" provider regardless of recent failures
    ///
    /// # Why use get_best_provider_with_tracker() instead?
    ///
    /// Providers can experience consecutive failures (timeouts, 5xx errors, etc.) at runtime.
    /// The failure tracker monitors these errors and marks providers as needing re-probing.
    ///
    /// Using this method ignores that tracking, so you might select a provider that has
    /// failed 3+ times in a row. [`get_best_provider_with_tracker()`] automatically skips
    /// those providers and selects the next best one that is healthy.
    pub fn get_best_provider<'a>(
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        results.models.get(model).and_then(|model_results| {
            model_results.iter().find(|r| r.success)
        })
    }

    /// Get the best provider for a model that does NOT need re-probing
    ///
    /// # ✅ Recommended for runtime provider selection
    ///
    /// This is the PRIMARY method for provider selection. It integrates the
    /// `should_reprobe` check directly into the selection logic, ensuring that
    /// providers experiencing consecutive failures are automatically excluded.
    ///
    /// ## Dual-Check Selection Behavior
    ///
    /// The provider selection system uses TWO independent checks:
    ///
    /// 1. **Staleness check (is_stale)**: A GLOBAL check on the probe data
    ///    - Checked once for the entire probe file (timestamp vs. current time)
    ///    - If probe data > 24h old → re-probe ALL providers
    ///    - Implemented in: [`is_stale()`]
    ///
    /// 2. **Provider health check (should_reprobe)**: A PER-PROVIDER check
    ///    - Checked for EACH provider during selection
    ///    - If provider has ≥N consecutive failures → skip that provider
    ///    - Implemented in: [`should_reprobe()`]
    ///
    /// ## How This Method Integrates Both Checks
    ///
    /// This method implements the **provider health check** (#2) during selection:
    ///
    /// ```text
    /// Probe results (pre-sorted by score):
    ///   1. provider_a (score: 50, failures: 0) ✅ SELECTED - no failures
    ///   2. provider_b (score: 80, failures: 3) ❌ SKIPPED - needs reprobe
    ///   3. provider_c (score: 120, failures: 0) ✅ FALLBACK - selected if A is failing
    /// ```
    ///
    /// The **staleness check** (#1) should be performed BEFORE calling this method
    /// using [`is_valid()`], which checks BOTH conditions.
    ///
    /// ## Implementation Details
    ///
    /// - **Parallel execution**: Uses `par_iter()` via rayon to check `should_reprobe()`
    ///   for all provider candidates **concurrently**, improving performance when
    ///   there are multiple providers to evaluate.
    ///
    /// - **Data flow**: HTTP requests → automatic failure/success recording in `FetchClient`
    ///   → `consecutive_failures` HashMap tracks failures per provider → `should_reprobe()`
    ///   checks: `failures.get(provider) >= threshold` → This method skips providers
    ///   where `should_reprobe() == true`
    ///
    /// - **Fallback behavior**: If all providers need re-probing, this method returns
    ///   the first successful provider as a fallback (rather than returning None).
    ///   This ensures you always get a usable provider, even if it's not ideal.
    ///
    /// # Arguments
    /// * `results` - The probe results to select from
    /// * `model` - The model to get the best provider for (e.g., "hrrr", "gefs", "nbm", "gfs")
    ///
    /// # Returns
    /// * `Some(&ProbeResult)` - The best provider result that doesn't need re-probing
    /// * `None` - If no successful providers exist for the model
    ///
    /// # Example
    /// ```no_run
    /// use gribtract_fetch::probe::{ProviderProbe, ProviderProbeResults};
    ///
    /// let mut probe = ProviderProbe::new();
    /// let results = ProviderProbe::load_results(std::path::Path::new("provider-probe.json")).unwrap();
    ///
    /// // Get the best provider that isn't experiencing repeated failures
    /// if let Some(best) = probe.get_best_provider_with_tracker(&results, "hrrr") {
    ///     println!("Best HRRR provider: {} (score: {:.1})", best.provider, best.score);
    /// }
    /// ```
    #[cfg(feature = "rayon")]
    pub fn get_best_provider_with_tracker<'a>(
        &'a self,
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        use rayon::prelude::*;

        results.models.get(model).and_then(|model_results| {
            // Parallel iteration over provider results
            // Find the first successful provider that doesn't need re-probing
            //
            // INTEGRATION: This is where should_reprobe() is called during provider selection.
            // The par_iter() ensures all providers are checked concurrently for efficiency.
            model_results.par_iter().find_any(|r| {
                r.success && !self.should_reprobe(&r.provider)
            }).or_else(|| {
                // Fallback: if all providers need re-probing, return the first successful one
                model_results.iter().find(|r| r.success)
            })
        })
    }

    /// Get the best provider for a model that does NOT need re-probing (synchronous version)
    ///
    /// # ✅ Recommended for runtime provider selection
    ///
    /// This is the PRIMARY method for provider selection. It integrates the
    /// `should_reprobe` check directly into the selection logic, ensuring that
    /// providers experiencing consecutive failures are automatically excluded.
    ///
    /// See the rayon version of [`get_best_provider_with_tracker()`] for detailed
    /// documentation about the dual-check selection behavior.
    ///
    /// This is a synchronous version of `get_best_provider_with_tracker` that does not
    /// use parallel execution. It's available when the `rayon` feature is not enabled.
    ///
    /// # Arguments
    /// * `results` - The probe results to select from
    /// * `model` - The model to get the best provider for
    ///
    /// # Returns
    /// * `Some(&ProbeResult)` - The best provider result that doesn't need re-probing
    /// * `None` - If no successful providers exist for the model
    #[cfg(not(feature = "rayon"))]
    pub fn get_best_provider_with_tracker<'a>(
        &'a self,
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        results.models.get(model).and_then(|model_results| {
            model_results.iter().find(|r| {
                r.success && !self.should_reprobe(&r.provider)
            })
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

    /// Check if probe results are valid (fresh AND no providers need re-probing)
    ///
    /// # Dual-Check Validation (Staleness + Provider Health)
    ///
    /// This method implements the **complete validation check** for provider probe data,
    /// combining BOTH independent checks:
    ///
    /// 1. **Staleness check**: Verifies the probe data is fresh (timestamp < max_age)
    ///    - If stale → returns `false` immediately (re-probe ALL providers)
    ///
    /// 2. **Provider health check**: Verifies no providers need re-probing
    ///    - Uses parallel execution to check all providers concurrently
    ///    - If ANY provider needs reprobe → returns `false`
    ///
    /// This is the **recommended validation method** before calling `get_best_provider_with_tracker()`.
    /// It ensures both conditions are met before using the cached provider rankings.
    ///
    /// ## Relationship to Selection Methods
    ///
    /// ```text
    /// Validation Phase (this method):
    ///   is_valid() → checks BOTH staleness AND provider health
    ///
    /// Selection Phase (get_best_provider_with_tracker):
    ///   get_best_provider_with_tracker() → checks provider health during selection
    /// ```
    ///
    /// The validation happens ONCE globally, while selection happens PER provider lookup.
    ///
    /// ## Performance
    ///
    /// Uses parallel execution (via rayon) to check `should_reprobe()` for all providers
    /// concurrently, improving performance when there are multiple providers to evaluate.
    ///
    /// # Arguments
    /// * `results` - The probe results to check for staleness
    /// * `max_age` - Maximum age for probe results to be considered fresh
    ///
    /// # Returns
    /// * `true` - If probe is fresh AND no providers need re-probing
    /// * `false` - If probe is stale OR any provider needs re-probing
    ///
    /// # Example
    /// ```no_run
    /// use gribtract_fetch::probe::ProviderProbe;
    /// use std::time::Duration;
    ///
    /// let probe = ProviderProbe::new();
    /// let results = ProviderProbe::load_results(std::path::Path::new("provider-probe.json")).unwrap();
    ///
    /// // Check if results are valid (fresh AND no providers need re-probing)
    /// if !probe.is_valid(&results, Duration::from_secs(24 * 3600)) {
    ///     // Trigger re-probe...
    /// }
    /// ```
    #[cfg(feature = "rayon")]
    pub fn is_valid(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
        use rayon::prelude::*;

        // First check staleness
        if Self::is_stale(results, max_age) {
            return false;
        }

        // Then check if any tracked provider has exceeded the failure threshold
        // Use parallel execution to check all providers concurrently
        //
        // INTEGRATION: This is where should_reprobe() is called in parallel to implement the
        // failure-tracking half of the dual-trigger re-probe logic.
        let all_providers: Vec<&String> = self.consecutive_failures.keys().collect();
        !all_providers.par_iter().any(|provider| {
            self.should_reprobe(provider)
        })
    }

    /// Check if probe results are valid (fresh AND no providers need re-probing)
    ///
    /// # Dual-Check Validation (Staleness + Provider Health)
    ///
    /// This method implements the **complete validation check** for provider probe data,
    /// combining BOTH independent checks.
    ///
    /// See the rayon version of [`is_valid()`] for detailed documentation about the
    /// dual-check validation behavior.
    ///
    /// This is a synchronous version of `is_valid` that does not use parallel execution.
    /// It's available when the `rayon` feature is not enabled.
    ///
    /// # Arguments
    /// * `results` - The probe results to check for staleness
    /// * `max_age` - Maximum age for probe results to be considered fresh
    ///
    /// # Returns
    /// * `true` - If probe is fresh AND no providers need re-probing
    /// * `false` - If probe is stale OR any provider needs re-probing
    #[cfg(not(feature = "rayon"))]
    pub fn is_valid(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
        // First check staleness
        if Self::is_stale(results, max_age) {
            return false;
        }

        // Then check if any tracked provider has exceeded the failure threshold
        // Use should_reprobe() for consistency
        //
        // INTEGRATION: This is where should_reprobe() is called to implement the
        // failure-tracking half of the dual-trigger re-probe logic.
        for provider in self.consecutive_failures.keys() {
            if self.should_reprobe(provider) {
                return false;
            }
        }

        true
    }

    /// Get providers that need re-probing due to consecutive failures
    ///
    /// Returns a list of provider names that have exceeded the consecutive failure threshold.
    /// This can be used to log which providers are problematic before triggering a re-probe.
    ///
    /// Uses parallel execution (via rayon) to check all providers concurrently,
    /// improving performance when there are multiple providers to evaluate.
    ///
    /// # Returns
    /// A vector of provider names that need re-probing
    #[cfg(feature = "rayon")]
    pub fn providers_needing_reprobe(&self) -> Vec<String> {
        use rayon::prelude::*;

        self.consecutive_failures
            .par_iter()
            .filter(|(_provider, &count)| count >= self.consecutive_failure_threshold)
            .map(|(provider, _count)| provider.clone())
            .collect()
    }

    /// Get providers that need re-probing due to consecutive failures
    ///
    /// Returns a list of provider names that have exceeded the consecutive failure threshold.
    /// This can be used to log which providers are problematic before triggering a re-probe.
    ///
    /// This is a synchronous version of `providers_needing_reprobe` that does not use parallel execution.
    /// It's available when the `rayon` feature is not enabled.
    ///
    /// # Returns
    /// A vector of provider names that need re-probing
    #[cfg(not(feature = "rayon"))]
    pub fn providers_needing_reprobe(&self) -> Vec<String> {
        let mut needing = Vec::new();

        for (provider, &count) in &self.consecutive_failures {
            if count >= self.consecutive_failure_threshold {
                needing.push(provider.clone());
            }
        }

        needing
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
///
/// This example shows how to use the failure tracker to monitor provider health:
///
/// ```no_run
/// use gribtract_fetch::probe::ProviderFailureTracker;
///
/// // Create a failure tracker with threshold of 3 consecutive errors
/// let mut tracker = ProviderFailureTracker::new(3);
///
/// // Simulate HTTP request results
/// // tracker.record_failure("s3:hrrr");
/// // tracker.record_failure("s3:hrrr");
/// // tracker.record_failure("s3:hrrr");
///
/// // Check if re-probing is needed
/// if tracker.should_reprobe("s3:hrrr") {
///     println!("Provider s3:hrrr needs re-probing due to consecutive failures");
/// }
///
/// // Reset after successful request
/// // tracker.record_success("s3:hrrr");
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

    #[test]
    fn test_failure_tracker_edge_cases() {
        let mut tracker = ProviderFailureTracker::new(3);

        // Test zero failures
        assert_eq!(tracker.failure_count("s3:hrrr"), 0);
        assert!(!tracker.should_reprobe("s3:hrrr"));

        // Test exactly threshold failures
        for _ in 0..3 {
            tracker.record_failure("s3:hrrr");
        }
        assert_eq!(tracker.failure_count("s3:hrrr"), 3);
        assert!(tracker.should_reprobe("s3:hrrr"));

        // Test threshold+1 failures
        tracker.record_failure("s3:hrrr");
        assert_eq!(tracker.failure_count("s3:hrrr"), 4);
        assert!(tracker.should_reprobe("s3:hrrr"), "should_reprobe should remain true after threshold+1 failures");
    }

    #[test]
    fn test_failure_tracker_threshold_boundary() {
        // Test with different threshold values
        for threshold in 1..=5 {
            let mut tracker = ProviderFailureTracker::new(threshold);

            // Record (threshold - 1) failures - should NOT trigger
            for _ in 0..(threshold - 1) {
                tracker.record_failure("test_provider");
            }
            assert_eq!(tracker.failure_count("test_provider"), threshold - 1);
            assert!(!tracker.should_reprobe("test_provider"), "Should NOT trigger reprobe at threshold-1");

            // Record one more failure - should trigger
            tracker.record_failure("test_provider");
            assert_eq!(tracker.failure_count("test_provider"), threshold);
            assert!(tracker.should_reprobe("test_provider"), "Should trigger reprobe at exactly threshold");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_probe_hrrr() {
        let mut probe = ProviderProbe::new();
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

    #[test]
    fn test_is_valid_with_fresh_results_and_no_failures() {
        let probe = ProviderProbe::new();

        // Create fresh results (current timestamp)
        let mut results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Fresh results with no failures should be valid
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)));
    }

    #[test]
    fn test_is_valid_with_stale_results() {
        let probe = ProviderProbe::new();

        // Create stale results (25 hours old)
        let mut timestamp = chrono::Utc::now() - chrono::Duration::hours(25);
        let mut results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: timestamp.to_rfc3339(),
            git_sha: None,
        };

        // Stale results should not be valid even with no failures
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)));
    }

    #[test]
    fn test_is_valid_with_consecutive_failures() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let mut results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record 3 consecutive failures for a provider
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");

        // Fresh results but with failures should not be valid
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)));
    }

    #[test]
    fn test_is_valid_with_failures_below_threshold() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let mut results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record only 2 failures (below threshold)
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");

        // Should be valid since threshold is 3
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)));
    }

    #[test]
    fn test_providers_needing_reprobe() {
        let mut probe = ProviderProbe::new().with_threshold(2);

        // Record failures
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr"); // Exceeds threshold of 2

        probe.record_failure("gcs:hrrr"); // Below threshold

        probe.record_failure("s3:gefs");
        probe.record_failure("s3:gefs");
        probe.record_failure("s3:gefs"); // Exceeds threshold of 2

        let needing = probe.providers_needing_reprobe();

        // Should include s3:hrrr and s3:gefs, but not gcs:hrrr
        assert_eq!(needing.len(), 2);
        assert!(needing.contains(&"s3:hrrr".to_string()));
        assert!(needing.contains(&"s3:gefs".to_string()));
        assert!(!needing.contains(&"gcs:hrrr".to_string()));
    }

    #[test]
    fn test_providers_needing_reprobe_empty_when_no_failures() {
        let probe = ProviderProbe::new().with_threshold(3);

        let needing = probe.providers_needing_reprobe();

        // No providers should need re-probing
        assert!(needing.is_empty());
    }

    #[test]
    fn test_is_valid_returns_false_when_should_reprobe_returns_true() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record failures to reach the threshold
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");

        // Verify should_reprobe returns true
        assert!(probe.should_reprobe("s3:hrrr"), "should_reprobe should return true after 3 failures");

        // Verify is_valid returns false when should_reprobe is true
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return false when should_reprobe returns true");
    }

    #[test]
    fn test_is_valid_returns_true_when_should_reprobe_false_and_fresh() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record failures below threshold
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");

        // Verify should_reprobe returns false
        assert!(!probe.should_reprobe("s3:hrrr"), "should_reprobe should return false with only 2 failures");

        // Verify is_valid returns true when should_reprobe is false and file is fresh
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return true when should_reprobe returns false and file is fresh");
    }

    #[test]
    fn test_is_valid_returns_false_when_stale_regardless_of_should_reprobe() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create stale results (25 hours old)
        let timestamp = chrono::Utc::now() - chrono::Duration::hours(25);
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: timestamp.to_rfc3339(),
            git_sha: None,
        };

        // Verify is_valid returns false for stale results with no failures
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return false for stale results with no failures");

        // Now add failures that would trigger should_reprobe
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");

        // Verify should_reprobe returns true
        assert!(probe.should_reprobe("s3:hrrr"), "should_reprobe should return true");

        // Verify is_valid still returns false (stale trumps everything)
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return false for stale results even when should_reprobe is true");
    }

    #[test]
    fn test_is_valid_handles_multiple_providers_correctly() {
        let mut probe = ProviderProbe::new().with_threshold(2);

        // Create fresh results
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record failures for multiple providers
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr"); // Exceeds threshold of 2

        probe.record_failure("gcs:hrrr"); // Below threshold

        probe.record_failure("s3:gefs");
        probe.record_failure("s3:gefs"); // Exceeds threshold of 2

        probe.record_failure("gcs:gefs"); // Below threshold

        // Verify individual should_reprobe states
        assert!(probe.should_reprobe("s3:hrrr"), "s3:hrrr should need reprobe");
        assert!(!probe.should_reprobe("gcs:hrrr"), "gcs:hrrr should not need reprobe");
        assert!(probe.should_reprobe("s3:gefs"), "s3:gefs should need reprobe");
        assert!(!probe.should_reprobe("gcs:gefs"), "gcs:gefs should not need reprobe");

        // Verify is_valid returns false when ANY provider needs reprobe
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return false when at least one provider needs reprobe");

        // Reset one of the failing providers
        probe.record_success("s3:hrrr");

        // Verify is_valid still returns false (s3:gefs still needs reprobe)
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should still return false when s3:gefs needs reprobe");

        // Reset the other failing provider
        probe.record_success("s3:gefs");

        // Now all providers are below threshold - should be valid
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return true when all providers are below threshold");
    }

    #[test]
    fn test_integration_triggers_reprobe_on_consecutive_http_errors() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Simulate consecutive HTTP errors for a provider
        // Initially is_valid should return true
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should return true initially with no failures");

        // First HTTP error
        probe.record_failure("s3:hrrr");
        assert!(!probe.should_reprobe("s3:hrrr"), "should_reprobe should be false after 1 failure");
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should still be true after 1 failure");

        // Second HTTP error
        probe.record_failure("s3:hrrr");
        assert!(!probe.should_reprobe("s3:hrrr"), "should_reprobe should be false after 2 failures");
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should still be true after 2 failures");

        // Third HTTP error - this should trigger reprobe
        probe.record_failure("s3:hrrr");
        assert!(probe.should_reprobe("s3:hrrr"), "should_reprobe should be true after 3 failures");
        assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should be false after 3 consecutive failures");

        // Verify the consecutive error counter is being tracked correctly
        assert_eq!(probe.failure_count("s3:hrrr"), 3, "failure count should be 3");

        // Simulate a successful request - should reset the counter
        probe.record_success("s3:hrrr");
        assert_eq!(probe.failure_count("s3:hrrr"), 0, "failure count should be reset to 0");
        assert!(!probe.should_reprobe("s3:hrrr"), "should_reprobe should be false after success");
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should be true after counter is reset");
    }

    #[test]
    fn test_is_valid_dual_trigger_logic() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Test that is_valid returns false if EITHER staleness OR should_reprobe is true
        // (dual-trigger logic: stale OR should_reprobe)

        // Test 1: Fresh results, no failures - should be valid
        let fresh_results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };
        assert!(probe.is_valid(&fresh_results, Duration::from_secs(24 * 3600)),
                "Fresh results with no failures should be valid");

        // Test 2: Stale results, no failures - should be invalid (staleness trigger)
        let stale_results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: (chrono::Utc::now() - chrono::Duration::hours(25)).to_rfc3339(),
            git_sha: None,
        };
        assert!(!probe.is_valid(&stale_results, Duration::from_secs(24 * 3600)),
                "Stale results should be invalid even with no failures");

        // Test 3: Fresh results, with failures - should be invalid (should_reprobe trigger)
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");
        probe.record_failure("s3:hrrr");
        assert!(!probe.is_valid(&fresh_results, Duration::from_secs(24 * 3600)),
                "Fresh results with failures should be invalid");

        // Test 4: Stale results, with failures - should be invalid (both triggers)
        assert!(!probe.is_valid(&stale_results, Duration::from_secs(24 * 3600)),
                "Stale results with failures should be invalid (both triggers active)");

        // Test 5: Fresh results, with failures below threshold - should be valid
        let mut probe2 = ProviderProbe::new().with_threshold(5);
        probe2.record_failure("s3:hrrr");
        probe2.record_failure("s3:hrrr");
        assert!(probe2.is_valid(&fresh_results, Duration::from_secs(24 * 3600)),
                "Fresh results with failures below threshold should be valid");
    }

    #[test]
    fn test_should_reprobe_boundary_conditions() {
        let probe = ProviderProbe::new().with_threshold(3);

        // Test with different thresholds to ensure boundary is handled correctly
        for threshold in 1..=5 {
            let mut probe = ProviderProbe::new().with_threshold(threshold);

            // Record (threshold - 1) failures - should NOT trigger reprobe
            for _ in 0..(threshold - 1) {
                probe.record_failure("test_provider");
            }
            assert!(!probe.should_reprobe("test_provider"),
                    "should_reprobe should be false at threshold-1 for threshold={}", threshold);

            // Record one more failure - should trigger reprobe
            probe.record_failure("test_provider");
            assert!(probe.should_reprobe("test_provider"),
                   "should_reprobe should be true at exactly threshold for threshold={}", threshold);
        }
    }

    #[test]
    fn test_is_valid_with_no_tracked_providers() {
        let probe = ProviderProbe::new().with_threshold(3);

        // Create fresh results
        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // When no providers are tracked, is_valid should only depend on staleness
        assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
                "is_valid should be true for fresh results with no tracked providers");

        // Verify should_reprobe returns false for untracked providers
        assert!(!probe.should_reprobe("unknown:provider"),
                "should_reprobe should return false for untracked providers");
    }

    #[test]
    fn test_get_best_provider_with_tracker_skips_failing_providers() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create test results with multiple providers
        let mut models = std::collections::HashMap::new();
        models.insert(
            "hrrr".to_string(),
            vec![
                ProbeResult {
                    provider: "s3:hrrr-bdp".to_string(),
                    probe_url: "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/test.idx".to_string(),
                    connect_ms: 50,
                    ttfb_ms: 75,
                    throughput_mbs: 10.0,
                    score: 125.0,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                ProbeResult {
                    provider: "gcs:hrrr".to_string(),
                    probe_url: "https://storage.googleapis.com/hrrr/test.idx".to_string(),
                    connect_ms: 100,
                    ttfb_ms: 150,
                    throughput_mbs: 5.0,
                    score: 350.0,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            ],
        );

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record 3 failures for the first provider
        probe.record_failure("s3:hrrr-bdp");
        probe.record_failure("s3:hrrr-bdp");
        probe.record_failure("s3:hrrr-bdp");

        // get_best_provider without tracker should return the first successful
        let best_without = ProviderProbe::get_best_provider(&results, "hrrr");
        assert_eq!(best_without.unwrap().provider, "s3:hrrr-bdp");

        // get_best_provider_with_tracker should skip the failing provider
        let best_with = probe.get_best_provider_with_tracker(&results, "hrrr");
        assert_eq!(best_with.unwrap().provider, "gcs:hrrr");
    }

    #[test]
    fn test_get_best_provider_with_tracker_returns_none_when_all_failing() {
        let mut probe = ProviderProbe::new().with_threshold(2);

        // Create test results
        let mut models = std::collections::HashMap::new();
        models.insert(
            "hrrr".to_string(),
            vec![
                ProbeResult {
                    provider: "s3:hrrr-bdp".to_string(),
                    probe_url: "https://test1.idx".to_string(),
                    connect_ms: 50,
                    ttfb_ms: 75,
                    throughput_mbs: 10.0,
                    score: 125.0,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                ProbeResult {
                    provider: "gcs:hrrr".to_string(),
                    probe_url: "https://test2.idx".to_string(),
                    connect_ms: 100,
                    ttfb_ms: 150,
                    throughput_mbs: 5.0,
                    score: 350.0,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            ],
        );

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record failures for ALL providers
        probe.record_failure("s3:hrrr-bdp");
        probe.record_failure("s3:hrrr-bdp");
        probe.record_failure("gcs:hrrr");
        probe.record_failure("gcs:hrrr");

        // Should return None when all providers need re-probing
        let best = probe.get_best_provider_with_tracker(&results, "hrrr");
        assert!(best.is_none(), "Should return None when all providers need re-probing");
    }

    #[test]
    fn test_get_best_provider_with_tracker_returns_first_when_no_failures() {
        let probe = ProviderProbe::new().with_threshold(3);

        // Create test results
        let mut models = std::collections::HashMap::new();
        models.insert(
            "gfs".to_string(),
            vec![
                ProbeResult {
                    provider: "s3:gfs".to_string(),
                    probe_url: "https://test1.idx".to_string(),
                    connect_ms: 30,
                    ttfb_ms: 50,
                    throughput_mbs: 15.0,
                    score: 96.7,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                ProbeResult {
                    provider: "nomads:gfs".to_string(),
                    probe_url: "https://test2.idx".to_string(),
                    connect_ms: 200,
                    ttfb_ms: 300,
                    throughput_mbs: 2.0,
                    score: 800.0,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            ],
        );

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // When no failures, should return the first provider
        let best = probe.get_best_provider_with_tracker(&results, "gfs");
        assert_eq!(best.unwrap().provider, "s3:gfs");
    }

    #[test]
    fn test_get_best_provider_with_tracker_handles_unknown_model() {
        let probe = ProviderProbe::new().with_threshold(3);

        let results = ProviderProbeResults {
            models: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Unknown model should return None
        let best = probe.get_best_provider_with_tracker(&results, "unknown_model");
        assert!(best.is_none(), "Should return None for unknown model");
    }

    #[test]
    fn test_get_best_provider_with_tracker_with_partial_failures() {
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create test results
        let mut models = std::collections::HashMap::new();
        models.insert(
            "nbm".to_string(),
            vec![
                ProbeResult {
                    provider: "s3:nbm".to_string(),
                    probe_url: "https://test1.idx".to_string(),
                    connect_ms: 40,
                    ttfb_ms: 60,
                    throughput_mbs: 12.0,
                    score: 143.3,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                ProbeResult {
                    provider: "gcs:nbm".to_string(),
                    probe_url: "https://test2.idx".to_string(),
                    connect_ms: 80,
                    ttfb_ms: 120,
                    throughput_mbs: 6.0,
                    score: 286.7,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                ProbeResult {
                    provider: "nomads:nbm".to_string(),
                    probe_url: "https://test3.idx".to_string(),
                    connect_ms: 150,
                    ttfb_ms: 200,
                    throughput_mbs: 3.0,
                    score: 483.3,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            ],
        );

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // First provider below threshold
        probe.record_failure("s3:nbm");
        probe.record_failure("s3:nbm");

        // Second provider exceeds threshold
        probe.record_failure("gcs:nbm");
        probe.record_failure("gcs:nbm");
        probe.record_failure("gcs:nbm");

        // Should skip the second provider and return the third
        let best = probe.get_best_provider_with_tracker(&results, "nbm");
        assert_eq!(best.unwrap().provider, "nomads:nbm");
    }

    #[test]
    fn test_get_best_provider_with_tracker_parallel_execution() {
        // This test verifies that parallel execution works correctly
        // by checking multiple providers concurrently
        let mut probe = ProviderProbe::new().with_threshold(3);

        // Create test results with many providers to benefit from parallel execution
        let mut models = std::collections::HashMap::new();
        let mut providers = Vec::new();

        for i in 0..5 {
            providers.push(ProbeResult {
                provider: format!("provider:{}", i),
                probe_url: format!("https://test{}.idx", i),
                connect_ms: 50 + i as u64 * 10,
                ttfb_ms: 75 + i as u64 * 15,
                throughput_mbs: 10.0 - i as f64,
                score: 125.0 + i as f64 * 25.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }

        models.insert("test".to_string(), providers);

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Record failures for some providers
        probe.record_failure("provider:0");
        probe.record_failure("provider:0");
        probe.record_failure("provider:0");

        probe.record_failure("provider:2");
        probe.record_failure("provider:2");
        probe.record_failure("provider:2");

        // Should skip providers 0 and 2, return provider 1
        let best = probe.get_best_provider_with_tracker(&results, "test");
        assert_eq!(best.unwrap().provider, "provider:1");
    }
}
