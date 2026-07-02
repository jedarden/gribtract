// gribtract-fetch: HTTP byte-range fetching for NOAA GRIB2 data
//
// This crate provides HTTP byte-range fetching capabilities for retrieving GRIB2
// data from NOAA's public data sources:
// - NOAA S3 buckets: noaa-hrrr-bdp-pds, noaa-gefs-pds, noaa-nbm-grib2-pds
// - Google Cloud Storage: high-resolution-rapid-refresh, national-blend-of-models, gfs-ensemble-forecast-system
// - NOMADS: nomads.ncep.noaa.gov
//
// # Feature flags
// - `async`: Enable async HTTP client with tokio support
// - `probe`: Enable provider probing and selection

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod client;
pub mod error;
pub mod provider;

pub use client::{FetchClient, RangeRequest};
pub use error::{FetchError, Result};
pub use provider::{DataProvider, GcsBucket, NomadsModel, S3Bucket, StorageProvider};

#[cfg(feature = "probe")]
pub mod probe;

#[cfg(feature = "probe")]
pub use probe::{ProbeResult, ProviderProbe};

/// Re-export reqwest::StatusCode for convenience
pub use reqwest::StatusCode;
