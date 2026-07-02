//! Provider probing and selection

use crate::client::FetchClient;
use crate::provider::{DataProvider, S3Bucket, GcsBucket, NomadsModel};
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
}

impl ProviderProbe {
    /// Create a new provider probe client
    pub fn new() -> Self {
        Self {
            client: FetchClient::new(),
            probe_files: Self::default_probe_files(),
        }
    }

    /// Create a new provider probe client with a custom HTTP client
    pub fn with_client(client: FetchClient) -> Self {
        Self {
            client,
            probe_files: Self::default_probe_files(),
        }
    }

    /// Get default probe files for each model
    ///
    /// Returns a map of model name to list of (provider, url) tuples
    fn default_probe_files() -> HashMap<String, Vec<(String, String)>> {
        let mut files = HashMap::new();

        // HRRR model probe files
        files.insert(
            "hrrr".to_string(),
            vec![
                ("s3:hrrr-bdp".to_string(), "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250702/conus/hrrr.t00z.wrfsfcf00.grib2.idx".to_string()),
                ("gcs:hrrr".to_string(), "https://storage.googleapis.com/high-resolution-rapid-refresh/hrrr.20250702/conus/hrrr.t00z.wrfsfcf00.grib2.idx".to_string()),
            ],
        );

        // GEFS model probe files
        files.insert(
            "gefs".to_string(),
            vec![
                ("s3:gefs-pds".to_string(), "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20250702/00/wave/grb2/gefs.wave.t00z.prtcgrb.idx.0p25".to_string()),
                ("gcs:gefs".to_string(), "https://storage.googleapis.com/gfs-ensemble-forecast-system/gefs.20250702/00/wave/grb2/gefs.wave.t00z.prtcgrb.idx.0p25".to_string()),
            ],
        );

        // NBM model probe files
        files.insert(
            "nbm".to_string(),
            vec![
                ("s3:nbm-grib2".to_string(), "https://noaa-nbm-grib2-pds.s3.amazonaws.com/nbm.20250702/nbm.t00z.conusnest.2p5.f000.co.grib2.idx".to_string()),
                ("gcs:nbm".to_string(), "https://storage.googleapis.com/national-blend-of-models/nbm.20250702/nbm.t00z.conusnest.2p5.f000.co.grib2.idx".to_string()),
            ],
        );

        // GFS model probe files
        files.insert(
            "gfs".to_string(),
            vec![
                ("s3:gfs-pds".to_string(), "https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20250702/00/gfs.t00z.pgrb2.0p25.f000.idx".to_string()),
                ("nomads:gfs".to_string(), "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20250702/00/atmos/gfs.t00z.pgrb2.0p25.f000.idx".to_string()),
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
                let elapsed = start.elapsed();
                ProbeResult {
                    provider: provider.to_string(),
                    probe_url: url.to_string(),
                    connect_ms: elapsed.as_millis() as u64,
                    ttfb_ms: elapsed.as_millis() as u64,
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
    async fn probe_url_inner(&self, url: &str) -> Result<ProbeInnerResult, crate::FetchError> {
        let start = std::time::Instant::now();

        // First, do a HEAD request to measure connect time and check range support
        let head_response = self.client.client.head(url).send().await?;
        let connect_time = start.elapsed();

        let status = head_response.status();
        if !status.is_success() {
            return Err(crate::FetchError::HttpStatus(status));
        }

        // Check if range requests are supported
        let supports_range = head_response
            .headers()
            .get("accept-ranges")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_lowercase() == "bytes")
            .unwrap_or(false);

        // Get content length
        let content_length = head_response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

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
    pub fn get_best_provider(
        results: &ProviderProbeResults,
        model: &str,
    ) -> Option<&ProbeResult> {
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
}
