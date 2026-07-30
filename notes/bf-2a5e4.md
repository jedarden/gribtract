# Bead bf-2a5e4: Extract Lat/Lon Bounds Using wgrib2

## Summary
Successfully extracted latitude/longitude bounds from HRRR GRIB2 file using wgrib2.

## Work Completed

### 1. File Selection
- Target file: `samples/hrrr.20260723.t00z.wrfsfcf01.grib2`
- HRRR (High-Resolution Rapid Refresh) CONUS domain data
- Full file with complete grid coverage

### 2. wgrib2 Command Execution
```bash
wgrib2 samples/hrrr.20260723.t00z.wrfsfcf01.grib2 -domain
```

The `-domain` flag provides rectangular domain bounds suitable for g2ctl/GrADS plots.

### 3. Lat/Lon Bounds Extracted

**Spatial Bounds:**
- **North (max latitude):** 52.615653°
- **South (min latitude):** 21.140547°  
- **West (min longitude):** -134.095480°
- **East (max longitude):** -60.917193°

**Coverage:** Continental United States (CONUS) domain
- **Latitudinal range:** ~31.5° (21°N to 53°N)
- **Longitudinal range:** ~73.2° (-134° to -61°)

### 4. Raw Output Documentation
Full wgrib2 domain output saved to: `notes/bf-2a5e4/wgrib2_domain_output.txt`

All messages in the file share identical spatial bounds, confirming consistent CONUS coverage across the HRRR dataset.

## Technical Details

**Grid Information (from wgrib2 -grid):**
- Grid Template: 30 (Lambert Conformal Conic)
- Grid dimensions: 1799 x 1059 points
- Projection: Lambert Conformal with standard parallels at 38.5°N
- Grid spacing: 3000m x 3000m

**Command Reference:**
```bash
# Extract spatial bounds (min/max lat/lon)
wgrib2 <file.grib2> -domain

# Extract grid definition details  
wgrib2 <file.grib2> -grid

# Get message inventory
wgrib2 <file.grib2>
```

## Acceptance Criteria Met
- ✅ wgrib2 command executed successfully
- ✅ Lat/lon bounds extracted (min/max latitude and longitude)
- ✅ Raw output saved for reference (wgrib2_domain_output.txt)

## Notes
The HRRR CONUS domain covers the continental United States with a Lambert Conformal projection. The bounds represent the approximate extent of CONUS coverage from southern California/Florida (~21°N) to the northern US border (~53°N), and from west coast (~-134°) to east coast (~-61°).
