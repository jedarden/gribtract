# NOAA Regional Model Archives

## Summary

This document lists NOAA public archive sources for regional NWP (Numerical Weather Prediction) models, including base URLs, access methods, and available models.

---

## 1. HRRR (High-Resolution Rapid Refresh)

### Primary Archives

| Source | Base URL | Access Protocol | Archive Period |
|--------|----------|-----------------|----------------|
| AWS Open Data (NOAA HRRR Public Data) | `s3://noaa-hrrr-bdp-pds/` | S3 (HTTPS) | 2014-present |
| S3 Explorer | https://noaa-hrrr-bdp-pds.s3.amazonaws.com/index.html | HTTPS (web browser) | 2014-present |
| NOMADS | https://nomads.ncep.noaa.gov/ | HTTP | ~30 days rolling |
| Zarr Format (alternative) | `s3://hrrrzarr/` | S3 (HTTPS) | - |

### Model Specifications
- **Resolution:** 3km
- **Update Frequency:** Hourly
- **Coverage:** CONUS (Continental United States)
- **Format:** GRIB2

### Additional Resources
- Registry: https://registry.opendata.aws/noaa-hrrr-pds/
- Official Portal: https://rapidrefresh.noaa.gov/hrrr/
- University of Utah HRRR Archive FAQ: https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_FAQ.html

---

## 2. NAM (North American Mesoscale)

### Primary Archives

| Source | Base URL | Access Protocol | Archive Period |
|--------|----------|-----------------|----------------|
| READY Archive | https://www.ready.noaa.gov/data/archives/nam12 | HTTPS (direct download) | Historical |
| NCEP Central Operations | https://www.nco.ncep.noaa.gov/pmb/products/nam/ | HTTPS (GRIB2) | Current/recent |
| AWS Open Data | `s3://noaa-nam-pds/` | S3 (HTTPS) | Via registry |
| NCEI | https://www.ncei.noaa.gov/products/weather-climate-models/north-american-mesoscale | HTTPS | Long-term archive |

### Model Specifications
- **Resolutions:** 12km, 3km (various grids)
- **Update Frequency:** Every 6 hours (00, 06, 12, 18 UTC cycles)
- **Coverage:** CONUS and North American domains
- **Format:** GRIB2

### Additional Resources
- AWS Registry: https://registry.opendata.aws/noaa-nam/
- NOMADS: https://nomads.ncep.noaa.gov/
- Main Archive Page: https://www.ready.noaa.gov/archives.php

---

## 3. RAP (Rapid Refresh)

### Primary Archives

| Source | Base URL | Access Protocol | Archive Period |
|--------|----------|-----------------|----------------|
| NOMADS | https://nomads.ncep.noaa.gov/ | HTTP, FTP | ~30 days rolling |
| AWS Open Data | `s3://noaa-rap-pds/` | S3 (HTTPS) | Via registry |
| Microsoft Planetary Computer | https://planetarycomputer.microsoft.com/dataset/storage/noaa-rap | HTTPS | - |

### Model Specifications
- **Resolution:** 20km (RAP), 13km (RAP-X)
- **Update Frequency:** Hourly
- **Coverage:** North America (continental-scale)
- **Format:** GRIB2
- **Forecast Length:** 18 hours

### Additional Resources
- Official Portal: https://rapidrefresh.noaa.gov/
- AWS Registry: https://registry.opendata.aws/noaa-rap/
- NCEI Product Page: https://www.ncei.noaa.gov/products/weather-climate-models/rapid-refresh-update

---

## Access Protocols Summary

| Protocol | Description | Use Case |
|----------|-------------|----------|
| **S3 (HTTPS)** | Direct S3 bucket access via AWS Open Data | Bulk downloads, programmatic access |
| **HTTPS (Direct)** | Direct HTTP download from NOAA servers | Individual file downloads |
| **HTTP/FTP via NOMADS** | NOMADS web interface or FTP access | Recent data, browsing |
| **THREDDS/OPeNDAP** | Not explicitly found in initial search | Subset access, analysis |

---

## Cross-Platform Access

All three regional models (HRRR, NAM, RAP) are available via:
1. **AWS Open Data Registry** - Public S3 buckets (no AWS account required for read)
2. **NOMADS** - For recent data (~30 days rolling)
3. **Microsoft Planetary Computer** - Alternative cloud access

---

## Other Regional Models

- **NAM Nest:** Higher-resolution nested grids within NAM
- **RAP-X:** 13km resolution variant of RAP

---

## References

- [NOAA HRRR on AWS Open Data](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NOAA NAM on AWS Open Data](https://registry.opendata.aws/noaa-nam/)
- [NOAA RAP on AWS Open Data](https://registry.opendata.aws/noaa-rap/)
- [NOMADS - NCEP](https://nomads.ncep.noaa.gov/)
- [READY Gridded Data Archives](https://www.ready.noaa.gov/archives.php)
- [Rapid Refresh - NOAA](https://rapidrefresh.noaa.gov/)
- [NCEP Central Operations - GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/)
