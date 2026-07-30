# NAM GRIB2 File Inspection Summary

## File Information
- **File**: `nam_20260727_t00z_awip1200_tm00.grib2`
- **Source**: NOAA NAM model output (downloaded via bf-4uqufd)
- **Size**: 28M (28,398,971 bytes)
- **Date**: 2026-07-27 00:00 UTC
- **Records**: 186 total messages

## Grid Information
- **Grid Template**: 30 (Lambert Conformal Conic)
- **Grid Dimensions**: 614 x 428 = 262,792 points per record
- **Projection**: Lambert Conformal
- **Grid Parameters**:
  - Lat1: 12.190000°N
  - Lon1: 226.541000°E
  - LoV: 265.000000°
  - Latin1: 25.000000°N
  - Latin2: 25.000000°N
  - LatD: 25.000000°N
  - Dx: 12,191 m
  - Dy: 12,191 m

## Template Information
- **Product Definition Template (PDT)**: 0 (standard analysis/forecast)
- **Grid Definition Template (GDT)**: 30 (Lambert Conformal)
- **Data Representation**: All records consistent with same grid definition

## Data Coverage
The file contains 186 different meteorological variables including:
- Surface pressure and height fields
- Temperature and humidity at multiple levels
- Wind components (U/V) at various altitudes
- Precipitation variables
- Cloud cover and vertical motion
- Soils parameters
- Radiation fluxes
- Convective parameters (CAPE, CIN, etc.)

## File Output Generated
1. **wgrib2_inspection.txt**: Complete verbose inventory with all variable details
2. **wgrib2_grid_info.txt**: Grid definition information for all records
3. **wgrib2_pdt.txt**: Product Definition Template information
4. **wgrib2_inspection_summary.md**: This summary file

## Key Findings
- File integrity verified - wgrib2 successfully processed all 186 messages
- Consistent grid definition across all variables
- Standard GRIB2 encoding with PDT=0, GDT=30
- Comprehensive meteorological analysis dataset from NAM model
