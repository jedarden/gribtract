# GRIB2 Download Record - Bead bf-2rku

**Download Date:** 2026-07-03 09:21 UTC  
**Source URL:** https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260703/conus/hrrr.t00z.wrfsfcf00.grib2

## File Details

| Attribute | Value |
|-----------|-------|
| **Filename** | hrrr.20260703.t00z.wrfsfcf00.grib2 |
| **File Size** | 139 MB (138,879,071 bytes) |
| **File Type** | Gridded binary (GRIB) version 2 |
| **Model** | HRRR (High-Resolution Rapid Refresh) |
| **Date** | 2026-07-03 |
| **Cycle** | 00z (analysis) |
| **Forecast Hour** | f00 (analysis) |
| **Product** | wrfsfc (surface fields) |

## URL Pattern Reference

URL constructed using the HRRR pattern from `docs/research/bf-5gsm-noaa-url-patterns.md`:

```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

Where:
- YYYYMMDD = 20260703
- CC = 00 (cycle hour)
- FF = 00 (forecast hour)

## Verification

✅ File downloaded to samples/ directory  
✅ File size > 0 bytes (139 MB)  
✅ File has .grib2 extension  
✅ Download timestamp recorded (2026-07-03 09:21 UTC)  
✅ File verified as GRIB2 format using `file` command

## Notes

- This is a surface fields analysis file from the HRRR model
- HRRR runs hourly (24 cycles per day)
- The wrfsfc product contains surface-level meteorological fields
- File size is typical for HRRR CONUS domain at 3km resolution
