# Technical Specifications - Candidate Files Analysis

**Bead:** bf-44uqx  
**Date:** 2026-07-24

## File-by-File Technical Analysis

### GFS 0.25° (2026-07-24)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p25.f000
Size: 491 MB
Grid: 1440 x 721 (0.25° resolution)
Resolution: 0.25° x 0.25°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GFS 0.25° (2026-07-23)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p25.f000
Size: 487 MB
Grid: 1440 x 721 (0.25° resolution)
Resolution: 0.25° x 0.25°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GFS 0.50° (2026-07-24)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000
Size: 146 MB
Grid: 720 x 361 (0.5° resolution)
Resolution: 0.5° x 0.5°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GFS 0.50° (2026-07-23)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.0p50.f000
Size: 145 MB
Grid: 720 x 361 (0.5° resolution)
Resolution: 0.5° x 0.5°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GFS 1.0° (2026-07-24)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.1p00.f000
Size: 41 MB
Grid: 360 x 181 (1.0° resolution)
Resolution: 1.0° x 1.0°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GFS 1.0° (2026-07-23)
```
URL: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260723/00/atmos/gfs.t00z.pgrb2.1p00.f000
Size: 41 MB
Grid: 360 x 181 (1.0° resolution)
Resolution: 1.0° x 1.0°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~400+ individual parameters/levels
```

### GEFS Ensemble Mean - f000 (2026-07-24)
```
URL: https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
Size: 14 MB
Grid: 720 x 361 (0.5° resolution)
Resolution: 0.5° x 0.5°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~30 ensemble mean parameters
Type: Ensemble mean (probabilistic)
```

### GEFS Ensemble Mean - f003 (2026-07-24)
```
URL: https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f003
Size: 15 MB
Grid: 720 x 361 (0.5° resolution)
Resolution: 0.5° x 0.5°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~30 ensemble mean parameters
Type: Ensemble mean, forecast hour 3
```

### GEFS Ensemble Mean - f006 (2026-07-24)
```
URL: https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260724/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f006
Size: 15 MB
Grid: 720 x 361 (0.5° resolution)
Resolution: 0.5° x 0.5°
Coverage: Global (lat 90°N to 90°S, lon 0° to 360°)
Grid Template: 3.0 (Latitude/Longitude)
DRT: 5.3 (Complex packing - NOT DRT=0)
Records: ~30 ensemble mean parameters
Type: Ensemble mean, forecast hour 6
```

## wgrib2 Commands Used

### DRT Analysis
```bash
wgrib2 -Sec5 <file>.grib2
```
Output: Shows Data Representation Template for each record

### Grid Coverage Analysis
```bash
wgrib2 -grid <file>.grib2
```
Output: Shows grid template, lat/lon bounds, resolution

### Grid Definition Section
```bash
wgrib2 -Sec3 <file>.grib2
```
Output: Shows Grid Definition Section information

## Parameter Examples Found in Files

### Common Parameters (GFS):
- PRMSL: Pressure Reduced to MSL
- TMP: Temperature 
- RH: Relative Humidity
- UGRD/VGRD: Wind Components
- HGT: Geopotential Height
- SNMR/SNOW: Snow Mixing Ratio
- REFD: Reflectivity
- VIS: Visibility

### GEFS Specific:
- Ensemble mean versions of the above parameters
- Probabilistic forecast information

## Data Representation Template Details

### DRT=5.0 (Simple Packing) - NOT FOUND
- Direct storage of values
- Simple scaling
- No compression

### DRT=5.3 (Complex Packing) - FOUND IN ALL FILES
- Spatial differencing
- Bit packing with variable bit widths
- Group compression
- More complex decoding required

## Grid Template Details

### Grid Template 3.0 (Latitude/Longitude)
- Regular lat-lon grid
- Equally spaced points
- Global coverage
- All candidates use this template

## CONUS Coverage Verification

### CONUS Geographic Boundaries
- Latitude: 24°N to 49°N
- Longitude: 125°W to 67°W (235° to 293° in 0-360° notation)

### Grid Coverage Verification
All files have global grids:
- Latitude: 90°N to 90°S (includes 24°N to 49°N)
- Longitude: 0° to 360° (includes 235° to 293°)

**Conclusion: All files fully contain CONUS region**
