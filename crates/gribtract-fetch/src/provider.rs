//! Data provider abstractions for NOAA GRIB2 sources

use crate::{error::Result, FetchError};
use std::fmt;

/// Base URL for NOAA S3 buckets (public access, no auth required)
const NOAA_S3_BASE: &str = "https://noaa-";

/// Base URL for Google Cloud Storage buckets (public access)
const GCS_BASE: &str = "https://storage.googleapis.com/";

/// Base URL for NOMADS (NOAA's data server)
const NOMADS_BASE: &str = "https://nomads.ncep.noaa.gov/";

/// Supported GRIB2 data providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataProvider {
    /// NOAA S3 buckets (us-east-1 region)
    S3(S3Bucket),
    /// Google Cloud Storage (CDN-fronted)
    Gcs(GcsBucket),
    /// NOMADS HTTP server
    Nomads(NomadsModel),
}

impl fmt::Display for DataProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProvider::S3(bucket) => write!(f, "S3:{}", bucket),
            DataProvider::Gcs(bucket) => write!(f, "GCS:{}", bucket),
            DataProvider::Nomads(model) => write!(f, "NOMADS:{}", model),
        }
    }
}

/// NOAA S3 buckets available for public access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S3Bucket {
    /// HRRR (High-Resolution Rapid Refresh) model data
    HrrrBdp,
    /// GEFS (Global Ensemble Forecast System) model data
    GefsPds,
    /// NBM (National Blend of Models) GRIB2 data
    NbmGrib2,
    /// GFS (Global Forecast System) model data
    GfsPds,
}

impl fmt::Display for S3Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S3Bucket::HrrrBdp => write!(f, "noaa-hrrr-bdp-pds"),
            S3Bucket::GefsPds => write!(f, "noaa-gefs-pds"),
            S3Bucket::NbmGrib2 => write!(f, "noaa-nbm-grib2-pds"),
            S3Bucket::GfsPds => write!(f, "noaa-gfs-bdp-pds"),
        }
    }
}

impl S3Bucket {
    /// Get the full S3 bucket URL
    pub fn base_url(&self) -> String {
        format!("{}{s}.s3.amazonaws.com/", NOAA_S3_BASE, s = self)
    }

    /// Convert from string (for config parsing)
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "noaa-hrrr-bdp-pds" | "hrrr" => Ok(S3Bucket::HrrrBdp),
            "noaa-gefs-pds" | "gefs" => Ok(S3Bucket::GefsPds),
            "noaa-nbm-grib2-pds" | "nbm" => Ok(S3Bucket::NbmGrib2),
            "noaa-gfs-bdp-pds" | "gfs" => Ok(S3Bucket::GfsPds),
            _ => Err(FetchError::InvalidUrl(format!("Unknown S3 bucket: {}", s))),
        }
    }
}

/// Google Cloud Storage buckets mirroring NOAA data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GcsBucket {
    /// HRRR model data on GCS
    HighResolutionRapidRefresh,
    /// NBM model data on GCS
    NationalBlendOfModels,
    /// GEFS model data on GCS
    GfsEnsembleForecastSystem,
}

impl fmt::Display for GcsBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GcsBucket::HighResolutionRapidRefresh => write!(f, "high-resolution-rapid-refresh"),
            GcsBucket::NationalBlendOfModels => write!(f, "national-blend-of-models"),
            GcsBucket::GfsEnsembleForecastSystem => write!(f, "gfs-ensemble-forecast-system"),
        }
    }
}

impl GcsBucket {
    /// Get the full GCS bucket URL
    pub fn base_url(&self) -> String {
        format!("{}{b}/", GCS_BASE, b = self)
    }

    /// Convert from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "high-resolution-rapid-refresh" | "hrrr" => Ok(GcsBucket::HighResolutionRapidRefresh),
            "national-blend-of-models" | "nbm" => Ok(GcsBucket::NationalBlendOfModels),
            "gfs-ensemble-forecast-system" | "gefs" => Ok(GcsBucket::GfsEnsembleForecastSystem),
            _ => Err(FetchError::InvalidUrl(format!("Unknown GCS bucket: {}", s))),
        }
    }
}

/// NOMADS model endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NomadsModel {
    /// GFS model via NOMADS
    Gfs,
    /// GEFS model via NOMADS
    Gefs,
    /// NAM model via NOMADS
    Nam,
}

impl fmt::Display for NomadsModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NomadsModel::Gfs => write!(f, "gfs"),
            NomadsModel::Gefs => write!(f, "gefs"),
            NomadsModel::Nam => write!(f, "nam"),
        }
    }
}

impl NomadsModel {
    /// Get the base NOMADS URL for this model
    pub fn base_url(&self) -> String {
        format!("{}{m}/", NOMADS_BASE, m = self)
    }

    /// Convert from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "gfs" => Ok(NomadsModel::Gfs),
            "gefs" => Ok(NomadsModel::Gefs),
            "nam" => Ok(NomadsModel::Nam),
            _ => Err(FetchError::InvalidUrl(format!("Unknown NOMADS model: {}", s))),
        }
    }
}

impl DataProvider {
    /// Get the base URL for this provider
    pub fn base_url(&self) -> String {
        match self {
            DataProvider::S3(bucket) => bucket.base_url(),
            DataProvider::Gcs(bucket) => bucket.base_url(),
            DataProvider::Nomads(model) => model.base_url(),
        }
    }

    /// Create a provider from a string identifier
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(FetchError::InvalidUrl(
                "Provider must be in format 'type:name' (e.g., 's3:hrrr')".into(),
            ));
        }

        match parts[0].to_lowercase().as_str() {
            "s3" => Ok(DataProvider::S3(S3Bucket::from_str(parts[1])?)),
            "gcs" => Ok(DataProvider::Gcs(GcsBucket::from_str(parts[1])?)),
            "nomads" => Ok(DataProvider::Nomads(NomadsModel::from_str(parts[1])?)),
            _ => Err(FetchError::InvalidUrl(format!("Unknown provider type: {}", parts[0]))),
        }
    }
}

/// Storage provider trait for common operations
pub trait StorageProvider {
    /// Get the base URL for this provider
    fn base_url(&self) -> String;

    /// Check if this provider supports HTTP range requests
    fn supports_range_requests(&self) -> bool {
        true
    }
}

impl StorageProvider for S3Bucket {
    fn base_url(&self) -> String {
        Self::base_url(self)
    }
}

impl StorageProvider for GcsBucket {
    fn base_url(&self) -> String {
        Self::base_url(self)
    }
}

impl StorageProvider for NomadsModel {
    fn base_url(&self) -> String {
        Self::base_url(self)
    }
}
