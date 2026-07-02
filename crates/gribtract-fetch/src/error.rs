//! Error types for gribtract-fetch

/// Result type for fetch operations
pub type Result<T> = std::result::Result<T, FetchError>;

/// Errors that can occur during HTTP fetch operations
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Invalid HTTP status code
    #[error("HTTP error: {0}")]
    HttpStatus(StatusCode),

    /// Invalid range request format
    #[error("Invalid range request: {0}")]
    InvalidRange(String),

    /// Content range header missing or invalid
    #[error("Invalid content-range header: {0}")]
    InvalidContentRange(String),

    /// URL parsing error
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Provider probe error
    #[cfg(feature = "probe")]
    #[error("Provider probe failed: {0}")]
    ProbeError(String),

    /// Timeout
    #[error("Operation timed out after {0:?}")]
    Timeout(std::time::Duration),
}

/// Re-export StatusCode for convenience
pub use reqwest::StatusCode;

impl FetchError {
    /// Check if this error is retryable (e.g., network errors, 5xx status codes)
    pub fn is_retryable(&self) -> bool {
        match self {
            FetchError::HttpError(e) => {
                // Consider network errors as retryable
                e.is_timeout() || e.is_connect() || e.is_body() || e.is_request()
            }
            FetchError::HttpStatus(code) => {
                // Retry on 5xx server errors and 429 (too many requests)
                code.is_server_error() || *code == StatusCode::TOO_MANY_REQUESTS
            }
            _ => false,
        }
    }
}
