//! Runtime provider probe — reads `provider-probe.json` and returns the best
//! provider for each NOAA model (GFS, HRRR, NBM, …).
//!
//! The probe file is written by `xtask probe-providers`. At runtime, callers
//! call [`ProviderProbe::load`] to read the cached results.  The file has a
//! 24-hour TTL by default; callers can re-run `xtask probe-probes` to
//! refresh it when it becomes stale.
//!
//! # Re-probe triggers
//!
//! Providers should be re-probed in three cases:
//! 1. **File absent**: `ProviderProbe::load` returns `Err`
//! 2. **File stale**: `is_fresh()` returns `false` (timestamp > 24h old)
//! 3. **Consecutive HTTP errors**: `ProviderFailureTracker::should_reprobe()` returns `true`
//!
//! The `ProviderFailureTracker` from `gribtract_fetch` tracks consecutive
//! HTTP failures per provider at runtime. Use it alongside the staleness check:
//!
//! # Example
//!
//! This example shows the complete runtime provider selection workflow with
//! re-probe triggers for staleness and consecutive HTTP errors:
//!
//! ```no_run
//! use gribtract::ProviderProbe;
//! use gribtract_fetch::probe::ProviderFailureTracker;
//! use std::path::Path;
//!
//! // Load provider probe results
//! let probe = match ProviderProbe::load(Path::new("provider-probe.json")) {
//!     Ok(p) => p,
//!     Err(_) => {
//!         // Case 1: File absent — trigger re-probe
//!         eprintln!("provider-probe.json not found, running initial probe...");
//!         run_probe_and_write_json(); // Your implementation
//!         return;
//!     }
//! };
//!
//! // Initialize failure tracker (threshold = 3 consecutive errors)
//! let tracker = ProviderFailureTracker::default_threshold();
//!
//! // Check if we should re-probe (Case 2 OR Case 3)
//! // is_valid() checks both: file staleness AND consecutive failures
//! if !probe.is_valid(24 * 3600, &tracker) {
//!     // Get specific providers that need re-probing for logging
//!     let needing = probe.providers_needing_reprobe(&tracker);
//!
//!     if !probe.is_fresh(24 * 3600) {
//!         eprintln!("provider-probe.json is stale (>24h old), triggering re-probe...");
//!     }
//!
//!     if !needing.is_empty() {
//!         eprintln!("Re-probe triggered by consecutive failures: {}",
//!             needing.join(", "));
//!     }
//!
//!     run_probe_and_write_json(); // Your implementation
//!     return;
//! }
//!
//! // Safe to use cached provider rankings
//! if let Some(provider) = probe.best_provider("gfs") {
//!     println!("Best GFS provider: {provider}");
//!
//!     // After HTTP requests, record success/failure
//!     match fetch_data_from_provider(provider).await {
//!         Ok(_) => {
//!             tracker.record_success(provider);
//!             println!("Successfully fetched from {provider}");
//!         }
//!         Err(_) => {
//!             let count = tracker.record_failure(provider);
//!             eprintln!("HTTP error from {provider} (failure {count}/3)");
//!         }
//!     }
//! }
//! # fn fetch_data_from_provider(p: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
//! # fn run_probe_and_write_json() { }
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
    ///
    /// # ⚠️ Use with caution - does NOT check provider health
    ///
    /// This method returns the first provider in the rankings WITHOUT checking if it
    /// has experienced consecutive failures. This means you might select a provider
    /// that is currently experiencing issues (e.g., timeouts, 5xx errors).
    ///
    /// **Most callers should use [`best_provider_with_tracker()`] instead**, which
    /// automatically excludes providers that need re-probing due to consecutive failures.
    ///
    /// This method is ONLY appropriate when:
    /// - You're doing a one-off fetch and don't care about reliability
    /// - You're running in a context where you don't have access to the failure tracker
    /// - You explicitly want to select the "best" provider regardless of recent failures
    ///
    /// # Example
    /// ```no_run
    /// use gribtract::ProviderProbe;
    ///
    /// let probe = ProviderProbe::load(Path::new("provider-probe.json")).unwrap();
    ///
    /// // ⚠️ This might select a failing provider
    /// if let Some(provider) = probe.best_provider("gfs") {
    ///     println!("Best GFS provider (may be failing): {provider}");
    /// }
    /// ```
    ///
    /// # Why use best_provider_with_tracker() instead?
    ///
    /// Providers can experience consecutive failures (timeouts, 5xx errors, etc.) at runtime.
    /// The failure tracker monitors these errors and marks providers as needing re-probing.
    ///
    /// Using this method ignores that tracking, so you might select a provider that has
    /// failed 3+ times in a row. [`best_provider_with_tracker()`] automatically skips
    /// those providers and selects the next best one that is healthy.
    pub fn best_provider(&self, model: &str) -> Option<&str> {
        self.rankings.get(model)?.first().map(|s| s.as_str())
    }

    /// Best (lowest-score) provider for `model` that does NOT need re-probing.
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
    /// 1. **Staleness check (is_fresh)**: A GLOBAL check on the probe data
    ///    - Checked once for the entire probe file (timestamp vs. current time)
    ///    - If probe data > 24h old → re-probe ALL providers
    ///    - Implemented in: [`is_fresh()`](Self::is_fresh)
    ///
    /// 2. **Provider health check (should_reprobe)**: A PER-PROVIDER check
    ///    - Checked for EACH provider during selection
    ///    - If provider has ≥N consecutive failures → skip that provider
    ///    - Implemented in: [`gribtract_fetch::probe::ProviderFailureTracker::should_reprobe()`]
    ///
    /// ## How This Method Integrates Both Checks
    ///
    /// This method implements the **provider health check** (#2) during selection:
    ///
    /// ```text
    /// Rankings (pre-sorted by score):
    ///   1. provider_a (score: 50, failures: 0) ✅ SELECTED - no failures
    ///   2. provider_b (score: 80, failures: 3) ❌ SKIPPED - needs reprobe
    ///   3. provider_c (score: 120, failures: 0) ✅ FALLBACK - selected if A is failing
    /// ```
    ///
    /// The **staleness check** (#1) should be performed BEFORE calling this method:
    ///
    /// ```text
    /// if probe.is_fresh(24 * 3600) {
    ///     // Safe to select providers with tracker
    ///     if let Some(provider) = probe.best_provider_with_tracker("gfs", &tracker) {
    ///         // Use provider
    ///     }
    /// } else {
    ///     // Probe is stale → re-run xtask probe-providers
    /// }
    /// ```
    ///
    /// Or use the convenience method [`is_valid()`](Self::is_valid) which checks BOTH.
    ///
    /// ## Performance
    ///
    /// Uses parallel execution (via rayon) to check `should_reprobe()` for all providers
    /// concurrently, improving performance when there are multiple providers to evaluate.
    ///
    /// # Arguments
    /// * `model` - The model to get the best provider for (e.g., "gfs", "hrrr")
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch` that tracks runtime failures
    ///
    /// # Returns
    /// * `Some(provider)` - The best provider that doesn't need re-probing
    /// * `None` - If all providers need re-probing or the model doesn't exist
    ///
    /// # Example
    /// ```no_run
    /// use gribtract::ProviderProbe;
    /// use gribtract_fetch::probe::ProviderFailureTracker;
    ///
    /// let probe = ProviderProbe::load(Path::new("provider-probe.json")).unwrap();
    /// let tracker = ProviderFailureTracker::default_threshold();
    ///
    /// // Get the best provider that isn't experiencing repeated failures
    /// if let Some(provider) = probe.best_provider_with_tracker("gfs", &tracker) {
    ///     println!("Best GFS provider: {provider}");
    /// }
    /// ```
    #[cfg(all(feature = "provider-probe", feature = "rayon"))]
    pub fn best_provider_with_tracker(
        &self,
        model: &str,
        tracker: &gribtract_fetch::probe::ProviderFailureTracker,
    ) -> Option<&str> {
        use rayon::prelude::*;

        self.rankings.get(model)?.par_iter().find_any(|provider| {
            !tracker.should_reprobe(provider)
        }).map(|s| s.as_str())
    }

    /// Best (lowest-score) provider for `model` that does NOT need re-probing.
    ///
    /// # ✅ Recommended for runtime provider selection
    ///
    /// This is the PRIMARY method for provider selection. It integrates the
    /// `should_reprobe` check directly into the selection logic, ensuring that
    /// providers experiencing consecutive failures are automatically excluded.
    ///
    /// See the rayon version of [`best_provider_with_tracker()`] for detailed
    /// documentation about the dual-check selection behavior.
    ///
    /// This is a synchronous version of `best_provider_with_tracker` that does not
    /// use parallel execution. It's available when the `rayon` feature is not enabled.
    ///
    /// # Arguments
    /// * `model` - The model to get the best provider for (e.g., "gfs", "hrrr")
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch` that tracks runtime failures
    ///
    /// # Returns
    /// * `Some(provider)` - The best provider that doesn't need re-probing
    /// * `None` - If all providers need re-probing or the model doesn't exist
    #[cfg(all(feature = "provider-probe", not(feature = "rayon")))]
    pub fn best_provider_with_tracker(
        &self,
        model: &str,
        tracker: &gribtract_fetch::probe::ProviderFailureTracker,
    ) -> Option<&str> {
        self.rankings.get(model)?.iter().find(|provider| {
            !tracker.should_reprobe(provider)
        }).map(|s| s.as_str())
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

    /// Check if probe results are fresh AND no provider needs re-probing due to consecutive failures.
    ///
    /// # Dual-Check Validation (Staleness + Provider Health)
    ///
    /// This method implements the **complete validation check** for provider probe data,
    /// combining BOTH independent checks:
    ///
    /// 1. **Staleness check**: Verifies the probe data is fresh (timestamp < max_age_secs)
    ///    - If stale → returns `false` immediately (re-probe ALL providers)
    ///
    /// 2. **Provider health check**: Verifies no providers need re-probing
    ///    - Uses parallel execution to check all providers concurrently
    ///    - If ANY provider needs reprobe → returns `false`
    ///
    /// This is the **recommended validation method** before calling `best_provider_with_tracker()`.
    /// It ensures both conditions are met before using the cached provider rankings.
    ///
    /// ## Relationship to Selection Methods
    ///
    /// ```text
    /// Validation Phase (this method):
    ///   is_valid() → checks BOTH staleness AND provider health
    ///
    /// Selection Phase (best_provider_with_tracker):
    ///   best_provider_with_tracker() → checks provider health during selection
    /// ```
    ///
    /// The validation happens ONCE globally, while selection happens PER provider lookup.
    ///
    /// ## When to Use This Method
    ///
    /// Use this method when you want to:
    /// - Validate the probe data before using it
    /// - Check if it's safe to use the cached rankings
    /// - Determine if re-probing is needed
    ///
    /// ## Performance
    ///
    /// Uses parallel execution (via rayon) to check `should_reprobe()` for all providers
    /// concurrently, improving performance when there are multiple providers to evaluate.
    ///
    /// # Arguments
    /// * `max_age_secs` - Maximum age in seconds for the probe to be considered fresh
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch` that tracks runtime failures
    ///
    /// # Returns
    /// * `true` - If probe is fresh AND no providers need re-probing
    /// * `false` - If probe is stale OR any provider needs re-probing
    ///
    /// # Example
    /// ```no_run
    /// use gribtract::ProviderProbe;
    /// use gribtract_fetch::probe::ProviderFailureTracker;
    ///
    /// let probe = ProviderProbe::load(Path::new("provider-probe.json")).unwrap();
    /// let tracker = ProviderFailureTracker::default_threshold();
    ///
    /// // Check if probe is fresh AND no provider needs re-probing
    /// if probe.is_valid(24 * 3600, &tracker) {
    ///     // Safe to use cached provider rankings
    ///     if let Some(provider) = probe.best_provider_with_tracker("gfs", &tracker) {
    ///         println!("Best GFS provider: {provider}");
    ///     }
    /// } else {
    ///     // Re-probe needed (either stale or providers are failing)
    /// }
    /// ```
    #[cfg(all(feature = "provider-probe", feature = "rayon"))]
    pub fn is_valid(&self, max_age_secs: u64, tracker: &gribtract_fetch::probe::ProviderFailureTracker) -> bool {
        // First check staleness
        if !self.is_fresh(max_age_secs) {
            return false;
        }

        // Then check if any tracked provider has exceeded the failure threshold
        // Use parallel execution to check all providers concurrently
        use rayon::prelude::*;
        use std::collections::HashSet;

        // Collect all unique provider names
        let all_providers: HashSet<&str> = self.rankings.values()
            .flat_map(|providers| providers.iter().map(|s| s.as_str()))
            .collect();

        // Check all providers in parallel - return false if any needs re-probing
        !all_providers.par_iter().any(|provider| tracker.should_reprobe(provider))
    }

    /// Check if probe results are fresh AND no provider needs re-probing due to consecutive failures.
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
    /// * `max_age_secs` - Maximum age in seconds for the probe to be considered fresh
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch` that tracks runtime failures
    ///
    /// # Returns
    /// * `true` - If probe is fresh AND no providers need re-probing
    /// * `false` - If probe is stale OR any provider needs re-probing
    #[cfg(all(feature = "provider-probe", not(feature = "rayon")))]
    pub fn is_valid(&self, max_age_secs: u64, tracker: &gribtract_fetch::probe::ProviderFailureTracker) -> bool {
        // First check staleness
        if !self.is_fresh(max_age_secs) {
            return false;
        }

        // Then check if any tracked provider has exceeded the failure threshold
        for (_model, providers) in &self.rankings {
            for provider in providers {
                if tracker.should_reprobe(provider) {
                    return false;
                }
            }
        }

        true
    }

    /// Get providers that need re-probing due to consecutive failures.
    ///
    /// Returns a list of provider names that have exceeded the consecutive failure threshold.
    /// This can be used to log which providers are problematic before triggering a re-probe.
    ///
    /// Uses parallel execution (via rayon) to check `should_reprobe()` for all providers
    /// concurrently, improving performance when there are multiple providers to evaluate.
    ///
    /// # Arguments
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch`
    ///
    /// # Returns
    /// A vector of provider names that need re-probing
    #[cfg(all(feature = "provider-probe", feature = "rayon"))]
    pub fn providers_needing_reprobe(
        &self,
        tracker: &gribtract_fetch::probe::ProviderFailureTracker,
    ) -> Vec<String> {
        use rayon::prelude::*;
        use std::collections::HashSet;

        // Collect all unique provider names
        let all_providers: HashSet<&str> = self.rankings.values()
            .flat_map(|providers| providers.iter().map(|s| s.as_str()))
            .collect();

        // Filter providers in parallel to find those needing re-probing
        all_providers
            .into_par_iter()
            .filter(|provider| tracker.should_reprobe(provider))
            .map(|s| s.to_string())
            .collect()
    }

    /// Get providers that need re-probing due to consecutive failures.
    ///
    /// Returns a list of provider names that have exceeded the consecutive failure threshold.
    /// This can be used to log which providers are problematic before triggering a re-probe.
    ///
    /// This is a synchronous version of `providers_needing_reprobe` that does not use parallel execution.
    /// It's available when the `rayon` feature is not enabled.
    ///
    /// # Arguments
    /// * `tracker` - The `ProviderFailureTracker` from `gribtract_fetch`
    ///
    /// # Returns
    /// A vector of provider names that need re-probing
    #[cfg(all(feature = "provider-probe", not(feature = "rayon")))]
    pub fn providers_needing_reprobe(
        &self,
        tracker: &gribtract_fetch::probe::ProviderFailureTracker,
    ) -> Vec<String> {
        let mut needing = Vec::new();

        for (_model, providers) in &self.rankings {
            for provider in providers {
                if tracker.should_reprobe(provider) {
                    if !needing.contains(provider) {
                        needing.push(provider.clone());
                    }
                }
            }
        }

        needing
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

    /// Generate a timestamp that is guaranteed to be "fresh" (within the last hour)
    ///
    /// This helper creates an ISO-8601 timestamp representing a time that is
    /// 1 hour ago from the current system time. This ensures tests that check
    /// staleness with a 24-hour threshold will always see this as fresh.
    fn fresh_timestamp() -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Go back 1 hour to ensure we're well within the 24-hour threshold
        let one_hour_ago = now.saturating_sub(3600);

        // Convert Unix seconds back to ISO-8601
        // This is the inverse of parse_iso8601_secs
        let days_since_epoch = (one_hour_ago / 86400) as i64;
        let secs_within_day = (one_hour_ago % 86400) as u32;

        // Convert days since epoch to YYYY-MM-DD (inverse of ymd_to_days)
        let (year, month, day) = days_to_ymd(days_since_epoch);

        let hour = secs_within_day / 3600;
        let min = (secs_within_day % 3600) / 60;
        let sec = secs_within_day % 60;

        format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, min, sec)
    }

    /// Convert days since epoch to (year, month, day)
    ///
    /// This is the inverse of ymd_to_days. For simplicity, we use a
    /// straightforward algorithm that works for dates after 1970.
    fn days_to_ymd(mut days: i64) -> (i32, u32, u32) {
        // Approximate year from days (1970 + days / 365)
        let mut year = 1970 + (days / 365) as i32;

        // Adjust for leap years
        loop {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            if days < days_in_year as i64 {
                break;
            }
            days -= days_in_year as i64;
            year += 1;
        }

        // Now 'days' is the day-of-year (0-indexed)
        let day_of_year = days as u32;

        // Convert to month and day
        let (month, day) = day_of_year_to_md(day_of_year, year);

        (year, month, day)
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn day_of_year_to_md(day_of_year: u32, year: i32) -> (u32, u32) {
        let days_in_month = if is_leap_year(year) {
            [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        let mut day = day_of_year + 1; // 1-indexed
        let mut month = 1;

        while month <= 12 && day > days_in_month[month as usize] {
            day -= days_in_month[month as usize];
            month += 1;
        }

        (month, day)
    }

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

    #[cfg(feature = "provider-probe")]
    #[test]
    fn is_valid_with_fresh_probe_and_no_failures() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["noaa-s3".to_string(), "nomads".to_string()]);
        rankings.insert("hrrr".to_string(), vec!["s3:hrrr-bdp".to_string(), "gcs:hrrr".to_string()]);

        // Use a timestamp that is less than 24 hours old
        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let tracker = ProviderFailureTracker::new(3);

        // Fresh probe with no failures should be valid
        assert!(probe.is_valid(86400, &tracker));
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn is_valid_with_stale_probe_returns_false() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["noaa-s3".to_string()]);

        let probe = ProviderProbe {
            timestamp: "1970-01-01T00:00:00Z".into(),
            probe_date: "19700101".into(),
            results: vec![],
            rankings,
        };

        let tracker = ProviderFailureTracker::new(3);

        // Stale probe should not be valid even with no failures
        assert!(!probe.is_valid(86400, &tracker));
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn is_valid_with_consecutive_failures_returns_false() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("hrrr".to_string(), vec!["s3:hrrr-bdp".to_string(), "gcs:hrrr".to_string()]);

        // Use a timestamp that is less than 24 hours old
        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(3);

        // Record 3 consecutive failures for s3:hrrr-bdp
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");

        // Fresh probe but with failures should not be valid
        assert!(!probe.is_valid(86400, &tracker));
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn is_valid_with_failures_below_threshold() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("hrrr".to_string(), vec!["s3:hrrr-bdp".to_string()]);

        // Use a timestamp that is less than 24 hours old
        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(3);

        // Record only 2 failures (below threshold)
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");

        // Should be valid since threshold is 3
        assert!(probe.is_valid(86400, &tracker));
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn providers_needing_reprobe_with_failures() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("hrrr".to_string(), vec!["s3:hrrr-bdp".to_string(), "gcs:hrrr".to_string()]);
        rankings.insert("gfs".to_string(), vec!["s3:gfs".to_string()]);

        let probe = ProviderProbe {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(2);

        // Record failures for s3:hrrr-bdp (exceeds threshold of 2)
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");

        // Record failure for gcs:hrrr (below threshold)
        tracker.record_failure("gcs:hrrr");

        let needing = probe.providers_needing_reprobe(&tracker);

        // Only s3:hrrr-bdp should be in the list
        assert_eq!(needing.len(), 1);
        assert!(needing.contains(&"s3:hrrr-bdp".to_string()));
        assert!(!needing.contains(&"gcs:hrrr".to_string()));
        assert!(!needing.contains(&"s3:gfs".to_string()));
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn providers_needing_reprobe_empty_when_no_failures() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert("gfs".to_string(), vec!["s3:gfs".to_string()]);

        let probe = ProviderProbe {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let tracker = ProviderFailureTracker::new(3);

        let needing = probe.providers_needing_reprobe(&tracker);

        // No providers should need re-probing
        assert!(needing.is_empty());
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn providers_needing_reprobe_deduplicates() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        // s3:hrrr-bdp appears in both model rankings
        rankings.insert("hrrr".to_string(), vec!["s3:hrrr-bdp".to_string()]);
        rankings.insert("hrrr-conus".to_string(), vec!["s3:hrrr-bdp".to_string()]);

        let probe = ProviderProbe {
            timestamp: "2026-06-22T00:00:00Z".into(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(3);
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");

        let needing = probe.providers_needing_reprobe(&tracker);

        // Should only list s3:hrrr-bdp once even though it appears in multiple models
        assert_eq!(needing.len(), 1);
        assert_eq!(needing[0], "s3:hrrr-bdp");
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn best_provider_with_tracker_skips_failing_providers() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert(
            "hrrr".to_string(),
            vec![
                "s3:hrrr-bdp".to_string(),
                "gcs:hrrr".to_string(),
                "nomads:hrrr".to_string(),
            ],
        );

        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(3);

        // Record 3 failures for the first provider (s3:hrrr-bdp)
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");

        // best_provider_without tracker should still return the first
        assert_eq!(probe.best_provider("hrrr"), Some("s3:hrrr-bdp"));

        // best_provider_with_tracker should skip the failing provider
        assert_eq!(
            probe.best_provider_with_tracker("hrrr", &tracker),
            Some("gcs:hrrr")
        );
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn best_provider_with_tracker_returns_none_when_all_failing() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert(
            "hrrr".to_string(),
            vec!["s3:hrrr-bdp".to_string(), "gcs:hrrr".to_string()],
        );

        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(2);

        // Record failures for ALL providers
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("s3:hrrr-bdp");
        tracker.record_failure("gcs:hrrr");
        tracker.record_failure("gcs:hrrr");

        // Should return None when all providers need re-probing
        assert_eq!(
            probe.best_provider_with_tracker("hrrr", &tracker),
            None
        );
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn best_provider_with_tracker_returns_first_when_no_failures() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert(
            "gfs".to_string(),
            vec!["s3:gfs".to_string(), "nomads:gfs".to_string()],
        );

        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let tracker = ProviderFailureTracker::new(3);

        // When no failures, should return the first provider
        assert_eq!(
            probe.best_provider_with_tracker("gfs", &tracker),
            Some("s3:gfs")
        );
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn best_provider_with_tracker_handles_unknown_model() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let rankings = HashMap::new();
        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let tracker = ProviderFailureTracker::new(3);

        // Unknown model should return None
        assert_eq!(
            probe.best_provider_with_tracker("unknown_model", &tracker),
            None
        );
    }

    #[cfg(feature = "provider-probe")]
    #[test]
    fn best_provider_with_tracker_with_partial_failures() {
        use gribtract_fetch::probe::ProviderFailureTracker;

        let mut rankings = HashMap::new();
        rankings.insert(
            "nbm".to_string(),
            vec![
                "s3:nbm".to_string(),
                "gcs:nbm".to_string(),
                "nomads:nbm".to_string(),
            ],
        );

        let probe = ProviderProbe {
            timestamp: fresh_timestamp(),
            probe_date: "20260620".into(),
            results: vec![],
            rankings,
        };

        let mut tracker = ProviderFailureTracker::new(3);

        // First provider exceeds threshold
        tracker.record_failure("s3:nbm");
        tracker.record_failure("s3:nbm");
        tracker.record_failure("s3:nbm");

        // Second provider below threshold
        tracker.record_failure("gcs:nbm");
        tracker.record_failure("gcs:nbm");

        // Should skip the first provider and return the second
        assert_eq!(
            probe.best_provider_with_tracker("nbm", &tracker),
            Some("gcs:nbm")
        );
    }
}
