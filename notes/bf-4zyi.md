# GRIB2 File Download - bf-4zyi

## Download Details

- **Source URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2`
- **Local Path:** `grib2/hrrr.t12z.wrfsfcf00.grib2`
- **File Size:** 142,393,582 bytes (136 MB)

## Provenance

| Field | Value |
|-------|-------|
| Model | NOAA HRRR (High-Resolution Rapid Refresh) |
| Resolution | 3 km CONUS |
| Date | 2024-06-01 |
| Cycle | 12z |
| Forecast Hour | f00 (analysis) |
| Product | wrfsfc (Surface fields) |
| Projection | Lambert Conformal Conic (GDT 3.30) |
| Packing | Complex packing with spatial differencing (DRT=3) |

## Verification

- File downloaded successfully
- File size is non-zero (142 MB)
- Source is NOAA's public HRRR archive on AWS S3

## Download Command

```bash
curl -L -o grib2/hrrr.t12z.wrfsfcf00.grib2 \
  "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2"
```
