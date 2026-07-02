//! HTTP client for byte-range fetching

use crate::error::{FetchError, Result};
use bytes::Bytes;
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
}

impl FetchClient {
    /// Create a new fetch client with default settings
    pub fn new() -> Self {
        Self::with_timeout(Duration::from_secs(30))
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
        }
    }

    /// Create a fetch client from an existing reqwest client
    pub fn from_client(client: reqwest::Client) -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            client,
        }
    }

    /// Fetch a byte range from a URL
    pub async fn fetch_range(&self, url: &str, range: RangeRequest) -> Result<RangeResponse> {
        let header_value = range.to_header_value();

        let response = self
            .client
            .get(url)
            .header("Range", header_value)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(FetchError::HttpStatus(status));
        }

        let content_range_header = response
            .headers()
            .get("content-range")
            .and_then(|v| v.to_str().ok());

        let content_range = match content_range_header {
            Some(header) => ContentRange::parse(header)?,
            None => {
                // Some providers might not return Content-Range for 200 OK responses
                // (they return the full resource instead of a range)
                return Err(FetchError::InvalidContentRange(
                    "Missing Content-Range header".into(),
                ));
            }
        };

        let total_size = content_range.total;
        let data = response.bytes().await?;

        Ok(RangeResponse {
            data,
            content_range,
            total_size,
        })
    }

    /// Fetch the first N bytes from a URL
    pub async fn fetch_head(&self, url: &str, length: u64) -> Result<RangeResponse> {
        self.fetch_range(url, RangeRequest::with_length(0, length)).await
    }

    /// Fetch the entire resource (no range request)
    pub async fn fetch_all(&self, url: &str) -> Result<Bytes> {
        let response = self.client.get(url).send().await?;

        let status = response.status();
        if !status.is_success() {
            return Err(FetchError::HttpStatus(status));
        }

        Ok(response.bytes().await?)
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
}
