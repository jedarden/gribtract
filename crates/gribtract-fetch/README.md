# gribtract-fetch

HTTP byte-range fetching for NOAA GRIB2 data from S3, GCS, and NOMADS.

## Overview

This crate provides HTTP byte-range request support for retrieving GRIB2 files from NOAA's public data sources:

- **NOAA S3 buckets**: `noaa-hrrr-bdp-pds`, `noaa-gefs-pds`, `noaa-nbm-grib2-pds`, `noaa-gfs-bdp-pds`
- **Google Cloud Storage**: `high-resolution-rapid-refresh`, `national-blend-of-models`, `gfs-ensemble-forecast-system`
- **NOMADS**: `nomads.ncep.noaa.gov`

Byte-range requests allow efficient retrieval of specific GRIB2 messages within a file without downloading the entire file.

## Features

- `async`: Enable async HTTP client with tokio support
- `probe`: Enable provider probing and selection

## Example

```rust
use gribtract_fetch::{FetchClient, RangeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FetchClient::new();

    // Fetch a 1KB range from a GRIB2 file
    let range = RangeRequest::with_length(0, 1024);
    let url = "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250702/conus/hrrr.t00z.wrfsfcf00.grib2";

    let response = client.fetch_range(url, range).await?;
    println!("Fetched {} bytes", response.data.len());

    Ok(())
}
```

## Provider Probing

The `probe` feature enables automatic provider selection based on latency and throughput:

```rust
use gribtract_fetch::probe::ProviderProbe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let probe = ProviderProbe::new();
    let results = probe.probe_all().await;

    // Get the best provider for HRRR
    if let Some(best) = ProviderProbe::get_best_provider(&results, "hrrr") {
        println!("Best HRRR provider: {} (score: {:.2})", best.provider, best.score);
    }

    Ok(())
}
```

## License

MIT OR Apache-2.0
