//! HTTP client for byte-range fetching

use crate::error::{FetchError, Result};
use bytes::Bytes;
use std::collections::HashMap;
use std::ops::Range;
use std::time::Duration;

/// HTTP range request specification
#[derive(Debug, Clone)]
pub struct RangeRequest {
    /// Start byte position (inclusive)
    pub start: u64,
    /// End byte position (inclusive), or None for fetch to end
    pub end: Option<u64>,
}

impl RangeRequest {
    /// Create a new range request
    pub fn new(start: u64, end: Option<u64>) -> Self {
        Self { start, end }
    }

    /// Create a range request with a specified length
    pub fn with_length(start: u64, length: u64) -> Self {
        Self {
            start,
            end: Some(start + length - 1),
        }
    }

    /// Create a range request from a Rust Range (end is exclusive)
    pub fn from_range(range: Range<u64>) -> Self {
        Self {
            start: range.start,
            end: if range.end > 0 {
                Some(range.end - 1)
            } else {
                None
            },
        }
    }

    /// Get the Range header value for HTTP requests
    pub fn to_header_value(&self) -> String {
        match self.end {
            Some(end) => format!("bytes={}-{}", self.start, end),
            None => format!("bytes={}-", self.start),
        }
    }

    /// Get the total number of bytes requested
    pub fn length(&self) -> Option<u64> {
        self.end.map(|end| end - self.start + 1)
    }
}

/// Response from a range request
#[derive(Debug)]
pub struct RangeResponse {
    /// The requested bytes
    pub data: Bytes,
    /// The actual content range returned by the server
    pub content_range: ContentRange,
    /// Total size of the resource (if known)
    pub total_size: Option<u64>,
}

/// Content-Range header information
#[derive(Debug, Clone)]
pub struct ContentRange {
    /// Start byte position
    pub start: u64,
    /// End byte position (inclusive)
    pub end: u64,
    /// Total resource size (or None if unknown/asterisk)
    pub total: Option<u64>,
}

impl ContentRange {
    /// Parse from Content-Range header value
    ///
    /// Format: `bytes start-end/total` or `bytes */total` or `bytes start-end/*`
    pub fn parse(header: &str) -> Result<Self> {
        let header = header.trim();
        if !header.starts_with("bytes ") {
            return Err(FetchError::InvalidContentRange(format!(
                "Missing 'bytes' prefix: {}",
                header
            )));
        }

        let parts: Vec<&str> = header[6..].split('/').collect();
        if parts.len() != 2 {
            return Err(FetchError::InvalidContentRange(format!(
                "Invalid format: {}",
                header
            )));
        }

        let total = match parts[1] {
            "*" => None,
            s => Some(
                s.parse::<u64>()
                    .map_err(|_| FetchError::InvalidContentRange(format!("Invalid total: {}", s)))?,
            ),
        };

        if parts[0] == "*" {
            return Ok(ContentRange {
                start: 0,
                end: 0,
                total,
            });
        }

        let range_parts: Vec<&str> = parts[0].split('-').collect();
        if range_parts.len() != 2 {
            return Err(FetchError::InvalidContentRange(format!(
                "Invalid range format: {}",
                header
            )));
        }

        let start = range_parts[0]
            .parse::<u64>()
            .map_err(|_| FetchError::InvalidContentRange(format!("Invalid start: {}", range_parts[0])))?;
        let end = range_parts[1]
            .parse::<u64>()
            .map_err(|_| FetchError::InvalidContentRange(format!("Invalid end: {}", range_parts[1])))?;

        Ok(ContentRange { start, end, total })
    }

    /// Get the length of this range
    pub fn length(&self) -> u64 {
        self.end - self.start + 1
    }
}

/// HTTP client for fetching GRIB2 data via byte-range requests
#[derive(Debug, Clone)]
pub struct FetchClient {
    client: reqwest::Client,
    default_timeout: Duration,
    /// Consecutive failure count per provider (provider identifier -> failure count)
    consecutive_failures: std::collections::HashMap<String, u32>,
    /// Threshold for consecutive failures before re-probe trigger
    consecutive_failure_threshold: u32,
}

impl FetchClient {
    /// Create a new fetch client with default settings
    pub fn new() -> Self {
        Self::with_timeout(Duration::from_secs(30))
    }

    /// Extract provider identifier from a URL
    ///
    /// Returns a string like "s3:hrrr", "gcs:gefs", "nomads:gfs", etc.
    /// Returns None if the provider cannot be determined from the URL.
    fn extract_provider_from_url(url: &str) -> Option<String> {
        let url = url.to_lowercase();

        // NOAA S3 buckets: https://noaa-{bucket}.s3.amazonaws.com/
        if url.contains("s3.amazonaws.com") {
            if url.contains("hrrr") {
                return Some("s3:hrrr".to_string());
            } else if url.contains("gefs") {
                return Some("s3:gefs".to_string());
            } else if url.contains("nbm") {
                return Some("s3:nbm".to_string());
            } else if url.contains("gfs") {
                return Some("s3:gfs".to_string());
            }
        }

        // Google Cloud Storage: https://storage.googleapis.com/{bucket}/
        if url.contains("storage.googleapis.com") {
            if url.contains("high-resolution-rapid-refresh") || url.contains("hrrr") {
                return Some("gcs:hrrr".to_string());
            } else if url.contains("gfs-ensemble-forecast-system") || url.contains("gefs") {
                return Some("gcs:gefs".to_string());
            } else if url.contains("national-blend-of-models") || url.contains("nbm") {
                return Some("gcs:nbm".to_string());
            }
        }

        // NOMADS: https://nomads.ncep.noaa.gov/
        if url.contains("nomads.ncep.noaa.gov") {
            if url.contains("/gfs/") {
                return Some("nomads:gfs".to_string());
            } else if url.contains("/gefs/") {
                return Some("nomads:gefs".to_string());
            } else if url.contains("/nam/") {
                return Some("nomads:nam".to_string());
            }
        }

        None
    }

    /// Create a new fetch client with a specified timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            default_timeout: timeout,
            consecutive_failures: std::collections::HashMap::new(),
            consecutive_failure_threshold: 3, // Default threshold
        }
    }

    /// Create a fetch client from an existing reqwest client
    pub fn from_client(client: reqwest::Client) -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            client,
            consecutive_failures: std::collections::HashMap::new(),
            consecutive_failure_threshold: 3, // Default threshold
        }
    }

    /// Fetch a byte range from a URL
    pub async fn fetch_range(&mut self, url: &str, range: RangeRequest) -> Result<RangeResponse> {
        let provider = Self::extract_provider_from_url(url);
        let header_value = range.to_header_value();

        let response = self
            .client
            .get(url)
            .header("Range", header_value)
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();

                if !status.is_success() {
                    // Record failure for non-success status
                    if let Some(provider) = provider {
                        self.record_failure(&provider);
                    }
                    return Err(FetchError::HttpStatus(status));
                }

                let content_range_header = resp
                    .headers()
                    .get("content-range")
                    .and_then(|v| v.to_str().ok());

                let content_range = match content_range_header {
                    Some(header) => ContentRange::parse(header)?,
                    None => {
                        // Some providers might not return Content-Range for 200 OK responses
                        // (they return the full resource instead of a range)
                        if let Some(provider) = provider {
                            self.record_failure(&provider);
                        }
                        return Err(FetchError::InvalidContentRange(
                            "Missing Content-Range header".into(),
                        ));
                    }
                };

                let total_size = content_range.total;
                let data = resp.bytes().await?;

                // Record success on successful request
                if let Some(provider) = provider {
                    self.record_success(&provider);
                }

                Ok(RangeResponse {
                    data,
                    content_range,
                    total_size,
                })
            }
            Err(e) => {
                // Record failure for request errors (timeout, connection refused, etc.)
                if let Some(provider) = provider {
                    self.record_failure(&provider);
                }
                Err(FetchError::Reqwest(e))
            }
        }
    }

    /// Fetch the first N bytes from a URL
    pub async fn fetch_head(&mut self, url: &str, length: u64) -> Result<RangeResponse> {
        self.fetch_range(url, RangeRequest::with_length(0, length)).await
    }

    /// Fetch the entire resource (no range request)
    pub async fn fetch_all(&mut self, url: &str) -> Result<Bytes> {
        let provider = Self::extract_provider_from_url(url);

        let response = self.client.get(url).send().await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                if !status.is_success() {
                    // Record failure for non-success status
                    if let Some(provider) = provider {
                        self.record_failure(&provider);
                    }
                    return Err(FetchError::HttpStatus(status));
                }

                // Record success on successful request
                if let Some(provider) = provider {
                    self.record_success(&provider);
                }

                Ok(resp.bytes().await?)
            }
            Err(e) => {
                // Record failure for request errors (timeout, connection refused, etc.)
                if let Some(provider) = provider {
                    self.record_failure(&provider);
                }
                Err(FetchError::Reqwest(e))
            }
        }
    }

    /// Get the resource size with a HEAD request (Content-Length header)
    pub async fn resource_size(&self, url: &str) -> Result<Option<u64>> {
        let response = self.client.head(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(FetchError::HttpStatus(status));
        }

        Ok(response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok()))
    }

    /// Check if the URL is accessible and supports range requests
    pub async fn probe(&self, url: &str) -> Result<ProbeInfo> {
        let start = std::time::Instant::now();

        let response = self
            .client
            .head(url)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(FetchError::HttpStatus(status));
        }

        let connect_time = start.elapsed();
        let supports_range = response
            .headers()
            .get("accept-ranges")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_lowercase() == "bytes")
            .unwrap_or(false);

        let content_length = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        Ok(ProbeInfo {
            url: url.to_string(),
            connect_time,
            supports_range,
            content_length,
        })
    }

    // === Failure tracking methods ===

    /// Get the current consecutive failure count for a provider
    pub fn get_failure_count(&self, provider: &str) -> u32 {
        self.consecutive_failures.get(provider).copied().unwrap_or(0)
    }

    /// Set the consecutive failure threshold
    pub fn set_threshold(&mut self, threshold: u32) {
        self.consecutive_failure_threshold = threshold;
    }

    /// Get the current consecutive failure threshold
    pub fn get_threshold(&self) -> u32 {
        self.consecutive_failure_threshold
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
    pub fn should_reprobe(&self, provider: &str) -> bool {
        self.consecutive_failures
            .get(provider)
            .map(|&count| count >= self.consecutive_failure_threshold)
            .unwrap_or(false)
    }

    /// Reset all failure counters
    pub fn reset_failures(&mut self) {
        self.consecutive_failures.clear();
    }
}

impl Default for FetchClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a URL from a probe request
#[derive(Debug, Clone)]
pub struct ProbeInfo {
    /// The URL that was probed
    pub url: String,
    /// Time to establish connection and get HEAD response
    pub connect_time: Duration,
    /// Whether the server supports range requests
    pub supports_range: bool,
    /// Content length if available
    pub content_length: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_request_header() {
        let range = RangeRequest::new(100, Some(199));
        assert_eq!(range.to_header_value(), "bytes=100-199");

        let range = RangeRequest::new(100, None);
        assert_eq!(range.to_header_value(), "bytes=100-");

        let range = RangeRequest::with_length(0, 1024);
        assert_eq!(range.to_header_value(), "bytes=0-1023");

        let range = RangeRequest::from_range(100..200);
        assert_eq!(range.to_header_value(), "bytes=100-199");
    }

    #[test]
    fn test_content_range_parse() {
        let cr = ContentRange::parse("bytes 0-1023/2048").unwrap();
        assert_eq!(cr.start, 0);
        assert_eq!(cr.end, 1023);
        assert_eq!(cr.total, Some(2048));
        assert_eq!(cr.length(), 1024);

        let cr = ContentRange::parse("bytes 0-1023/*").unwrap();
        assert_eq!(cr.start, 0);
        assert_eq!(cr.end, 1023);
        assert_eq!(cr.total, None);

        let cr = ContentRange::parse("bytes */2048").unwrap();
        assert_eq!(cr.start, 0);
        assert_eq!(cr.end, 0);
        assert_eq!(cr.total, Some(2048));
    }

    #[test]
    fn test_provider_urls() {
        use crate::provider::{S3Bucket, GcsBucket, NomadsModel};

        let hrrr = S3Bucket::HrrrBdp;
        assert_eq!(
            hrrr.base_url(),
            "https://noaa-noaa-hrrr-bdp-pds.s3.amazonaws.com/"
        );

        let hrrr_gcs = GcsBucket::HighResolutionRapidRefresh;
        assert_eq!(
            hrrr_gcs.base_url(),
            "https://storage.googleapis.com/high-resolution-rapid-refresh/"
        );

        let gfs = NomadsModel::Gfs;
        assert_eq!(gfs.base_url(), "https://nomads.ncep.noaa.gov/gfs/");
    }

    #[test]
    fn test_failure_tracking_basic() {
        let mut client = FetchClient::new();

        // Initially, no failures recorded
        assert_eq!(client.get_failure_count("s3:hrrr"), 0);
        assert_eq!(client.get_threshold(), 3);
        assert!(!client.should_reprobe("s3:hrrr"));

        // Record first failure
        assert_eq!(client.record_failure("s3:hrrr"), 1);
        assert_eq!(client.get_failure_count("s3:hrrr"), 1);
        assert!(!client.should_reprobe("s3:hrrr"));

        // Record second failure
        assert_eq!(client.record_failure("s3:hrrr"), 2);
        assert_eq!(client.get_failure_count("s3:hrrr"), 2);
        assert!(!client.should_reprobe("s3:hrrr"));

        // Record third failure - reaches threshold
        assert_eq!(client.record_failure("s3:hrrr"), 3);
        assert_eq!(client.get_failure_count("s3:hrrr"), 3);
        assert!(client.should_reprobe("s3:hrrr"));

        // Record a success - should reset the counter
        client.record_success("s3:hrrr");
        assert_eq!(client.get_failure_count("s3:hrrr"), 0);
        assert!(!client.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_failure_tracking_multiple_providers() {
        let mut client = FetchClient::new();

        // Record failures for provider A (need 3 to reach threshold)
        assert_eq!(client.record_failure("s3:hrrr"), 1);
        assert_eq!(client.record_failure("s3:hrrr"), 2);
        assert!(!client.should_reprobe("s3:hrrr")); // Not at threshold yet
        assert_eq!(client.record_failure("s3:hrrr"), 3);
        assert!(client.should_reprobe("s3:hrrr")); // Now at threshold

        // Provider B should not be affected
        assert_eq!(client.get_failure_count("gcs:hrrr"), 0);
        assert!(!client.should_reprobe("gcs:hrrr"));

        // Record failures for provider B
        assert_eq!(client.record_failure("gcs:hrrr"), 1);
        assert!(!client.should_reprobe("gcs:hrrr"));

        // Provider A should still be at threshold
        assert!(client.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_failure_threshold_configurable() {
        let mut client = FetchClient::new();

        // Set custom threshold
        client.set_threshold(5);
        assert_eq!(client.get_threshold(), 5);

        // Record 4 failures
        for _ in 0..4 {
            client.record_failure("s3:hrrr");
        }

        assert_eq!(client.get_failure_count("s3:hrrr"), 4);
        assert!(!client.should_reprobe("s3:hrrr"));

        // Record 5th failure - reaches custom threshold
        client.record_failure("s3:hrrr");
        assert!(client.should_reprobe("s3:hrrr"));
    }

    #[test]
    fn test_reset_failures() {
        let mut client = FetchClient::new();

        // Record some failures
        client.record_failure("s3:hrrr");
        client.record_failure("gcs:hrrr");
        client.record_failure("s3:gefs");

        assert_eq!(client.get_failure_count("s3:hrrr"), 1);
        assert_eq!(client.get_failure_count("gcs:hrrr"), 1);
        assert_eq!(client.get_failure_count("s3:gefs"), 1);

        // Reset all failures
        client.reset_failures();

        assert_eq!(client.get_failure_count("s3:hrrr"), 0);
        assert_eq!(client.get_failure_count("gcs:hrrr"), 0);
        assert_eq!(client.get_failure_count("s3:gefs"), 0);
    }
}
