# GDT 3.40 Public Accessibility Verification

## Task
Verify the GDT-3.40 file is publicly accessible and document all required metadata.

## Download Test Result
✅ **SUCCESS** - File downloaded via HTTPS without authentication

## File Metadata

### Exact URL
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2
```

### File Size
- **126,155,368 bytes** (120.3 MB)
- Download time: ~16 seconds at 7.64 MB/s average

### Model Run Date/Time
- **Date:** 2026-07-27
- **Cycle:** 00Z (00:00 UTC)
- **Analysis time:** 2026-07-27 00:00 UTC

### Forecast Hour
- **f000** (0 hours - initial analysis, not a forecast)

### Grid Definition Template (GDT)
- **GDT:** 40 (Grid Definition Template 3.40)
- **Grid Type:** regular_gg (Gaussian latitude/longitude grid)

### Grid Resolution
- **Nx:** 3072 grid points (longitude)
- **Ny:** 1536 grid points (latitude)
- **Effective resolution:** ~0.117° (720/3072 ≈ 0.234° between Gaussian latitudes)
- **Grid spacing:** 0.117188° longitude spacing at equator

### Verification Tool
Used `grib_ls` from ECCodes:
```bash
grib_ls -p gridType,gridDefinitionTemplateNumber,Nx,Ny,dataDate,forecastTime /tmp/gfs_sflux_gdt40.grib2
```

Output confirmed:
```
gridType: regular_gg
gridDefinitionTemplateNumber: 40
Nx: 3072
Ny: 1536
dataDate: 20260727
forecastTime: 0
```

## Important Finding: GDT vs File Type

**Key Discovery:** Different GFS file product types use different grid definitions:

### GDT 3.40 (Gaussian Grid) - Surface Flux Files
- File pattern: `gfs.tCCz.sfluxgrbfFFF.grib2`
- Grid: `regular_gg` (Gaussian latitude/longitude)
- GDT: 40 (Template 3.40)
- Resolution: 3072 x 1536
- ✅ **Publicly accessible**

### GDT 0 (Regular Lat/Lon Grid) - Pressure Level Files  
- File pattern: `gfs.tCCz.pgrb2.0p50.fFFF.grib2`
- Grid: `regular_ll` (regular latitude/longitude)
- GDT: 0 (Template 3.0)
- Resolution: 720 x 361 (0.5° resolution)
- ✅ **Publicly accessible** (but not GDT 3.40)

## Accessibility Confirmation

### No Authentication Required
- Downloaded via public HTTPS URL
- No API token, authentication header, or credentials needed
- Standard HTTP GET request sufficient

### Archive Stability
- **Archive:** NOAA NOMADS (Operational data server)
- **Base URL:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
- **URL Pattern:** `gfs.YYYYMMDD/CC/atmos/gfs.tCCz.sfluxgrbfFFF.grib2`
- **Availability:** Real-time operational data (current and recent runs)

## Related Public Files (Same GDT 3.40)

Other forecast hours in same cycle also use GDT 3.40:
- `gfs.t00z.sfluxgrbf001.grib2` (f001 - 3 hour forecast)
- `gfs.t00z.sfluxgrbf002.grib2` (f002 - 6 hour forecast)  
- `gfs.t00z.sfluxgrbf003.grib2` (f003 - 9 hour forecast)
- ... up to f384 (384 hour forecast)

All surface flux files follow the same Gaussian grid pattern.

## Acceptance Criteria Status

✅ File successfully downloaded via HTTP(S) without authentication  
✅ All metadata documented (URL, size, model run, forecast hour, GDT value)  
✅ Clear confirmation that GDT = 3.40 (template number 40)  

## Sources
- NOAA NOMADS GFS Archive: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
- File downloaded: 2026-07-27T16:12:28Z
- Verified with ECCodes grib_ls tool
