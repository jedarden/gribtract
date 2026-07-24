# DRT Value Analysis - GRIB2 Files

## Task Summary
Checked DRT (Data Representation Template) values for all downloaded GRIB2 files using wgrib2.

## wgrib2 Command Used
```bash
wgrib2 -Sec3 <file> | grep -oP 'Grid Def Template=\K[0-9.]+(?= |$)'
```

The `-Sec3` option shows the Grid Definition Section contents, which includes the DRT value as "Grid Def Template=X.Y"

## Results

### Files Successfully Analyzed: 28 total

#### GFS Files (16 files) - DRT 3.0
All GFS files use **DRT 3.0 (Lambert Conformal Conic grid)**:
- gfs.20260722.t00z.pgrb2.0p25.f003
- gfs.20260723.t00z.pgrb2.0p25.f000
- gfs.20260723.t00z.pgrb2.0p25.f006
- gfs.20260723.t00z.pgrb2.0p50.f000
- gfs.20260723.t00z.pgrb2.1p00.f000
- gfs.20260724.t00z.pgrb2.0p25.f000
- gfs.20260724.t00z.pgrb2.0p25.f012
- gfs.20260724.t00z.pgrb2.0p50.f000
- gfs.20260724.t00z.pgrb2.1p00.f000
- gfs.t00z.pgrb2.0p25.f000
- gfs.t00z.pgrb2.0p25.f003
- gfs.t00z.pgrb2.0p25.f006
- gfs.t00z.pgrb2.0p25.f012
- gfs.t00z.pgrb2.0p50.f000
- gfs.t00z.pgrb2.0p25.f003
- gfs.t00z.pgrb2.1p00.f000

#### HRRR Files (11 files) - DRT 3.30
All HRRR files use **DRT 3.30 (Lambert Conformal Conic grid variant)**:
- hrrr.20260723.t00z.wrfsfcf01.grib2
- hrrr.20260724.t00z.wrfsfcf00.grib2
- hrrr.20260724.t00z.wrfsfcf01.grib2
- hrrr.20260724.t00z.wrfsfcf02.grib2
- hrrr.20260724.t00z.wrfsfcf03.grib2
- hrrr.20260724.t00z.wrfsfcf04.grib2
- hrrr.20260724.t00z.wrfsfcf05.grib2
- hrrr.20260724.t00z.wrfsfcf06.grib2
- hrrr.20260724.t00z.wrfsfcf07.grib2
- hrrr.20260724.t00z.wrfsfcf08.grib2
- hrrr.20260724.t00z.wrfsfcf12.grib2

#### Other Files (1 file) - DRT 3.30
- nam.t00z.awip1200.tm00.grib2 - **DRT 3.30 (Lambert Conformal Conic)**

### Files Skipped (empty/incomplete): 8 files

The following files were too small (< 1KB) and likely represent incomplete downloads:
- gfs.20260721.t00z.pgrb2.0p50.f000 (0 bytes - completely empty)
- hrrr.t00z.wrfsfcf01.grib2 (336 bytes)
- hrrr.20260723.t00z.wrfsfcf03.grib2 (196 bytes)
- hrrr.20260723.t12z.wrfsfcf00.grib2 (196 bytes)
- hrrr.20260724.t06z.wrfsfcf00.grib2 (196 bytes)
- nam.20260724.t00z.conusnest.hiresf00.tm00.grib2 (199 bytes)
- rap.20260724.t00z.awp130pgrbf00.grib2 (196 bytes)
- nam_awip12_20250115_t00z_f00.grib2 (0 bytes - completely empty)

## DRT Value Meanings

- **DRT 0.0**: Latitude/Longitude grid (simple regular lat/lon grid)
- **DRT 3.0**: Lambert Conformal Conic projection grid
- **DRT 3.30**: Lambert Conformal Conic projection grid (variant with specific parameters)
- **DRT 40.0**: Rotated Latitude/Longitude grid
- Other values: Specialized grid types

## Key Findings

1. **No files with DRT=0.0**: None of the downloaded files use simple Latitude/Longitude grids. All use Lambert Conformal Conic projections (DRT 3.0 or 3.30).

2. **Model-specific DRT patterns**:
   - GFS: Uses DRT 3.0 (standard Lambert Conformal Conic)
   - HRRR: Uses DRT 3.30 (Lambert Conformal Conic variant)
   - NAM: Uses DRT 3.30 (Lambert Conformal Conic variant)

3. **Incomplete downloads identified**: 8 files are too small and represent failed or incomplete downloads.

4. **All successful files are valid**: The 28 files that were successfully analyzed are all valid GRIB2 files with proper DRT values.

## Script Created

The analysis was performed using the script: `scripts/check_drt_values.sh`

This script can be re-run to analyze any new GRIB2 files downloaded in the future.
