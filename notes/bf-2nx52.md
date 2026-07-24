# Research: GFS Gaussian-Grid GRIB2 File with GDT 3.40

**Task:** bf-2nx52  
**Date:** 2026-07-24  
**Status:** Complete

## Target File Identified

### File URL
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2
```

### File Details
- **Product:** GDAS (Global Data Assimilation System) Surface Flux
- **Model Run:** 2026-07-24 00Z
- **Forecast Hour:** F000 (analysis)
- **Format:** GRIB2
- **File Size:** 122 MB
- **Grid Definition Template:** GDT 3.40 (confirmed)

### Grid Specifications (T1534 Gaussian Grid)
- **Grid Type:** Gaussian Latitude/Longitude (GDT 3.40)
- **Dimensions:** 3072 x 1536 grid points
- **N Parameter:** 768 (number of parallels between pole and equator)
- **Approximate Resolution:** ~0.117° longitude (~12 km)
- **Latitude Range:** 89.910324° to -89.910324°
- **Longitude Range:** 0.000000° to 359.882813°
- **Total Grid Points:** 4,718,592
- **Scanning Mode:** West-to-East, North-to-South

## Archive Structure

### Directory Pattern
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
├── gdas.YYYYMMDD/
│   └── HH/                      # Cycle: 00, 06, 12, 18
│       └── atmos/
│           ├── gdas.tHHz.sfluxgrbf000.grib2
│           ├── gdas.tHHz.sfluxgrbf003.grib2
│           ├── gdas.tHHz.sfluxgrbf006.grib2
│           └── gdas.tHHz.sfluxgrbf009.grib2
```

### File Naming Convention
- **Pattern:** `gdas.tCCz.sfluxgrbfFFF.grib2`
- **CC:** Cycle (00, 06, 12, 18)
- **FFF:** Forecast hour (000, 003, 006, 009)
- **Product:** Surface flux on T1534 Gaussian grid

## Additional Available Files

### GDAS Gaussian Grid Files (Same Run)
- `gdas.t00z.sfluxgrbf003.grib2` (F003)
- `gdas.t00z.sfluxgrbf006.grib2` (F006)
- `gdas.t00z.sfluxgrbf009.grib2` (F009)

### Related Products
- **GFS version:** `gfs.tCCz.sfluxgrbfFFF.grib2` (T1534 Semi-Lagrangian, FH000-FH384)
- **Analysis files:** `gdas.tHHz.atmanl.grib2` (various resolutions)

## Verification

### GDT 3.40 Confirmation
```
$ wgrib2 gdas_t00z_sfluxgrbf000.grib2 -grid
1:0:grid_template=40:winds(N/S):
    Gaussian grid: (3072 x 1536) units 1e-06 input WE:NS output WE:SN
    number of latitudes between pole-equator=768 #points=4718592
```

**grid_template=40** confirms GDT 3.40 (Gaussian Latitude/Longitude).

### Public Accessibility
- **HTTP Status:** 200 OK
- **Server:** Apache
- **Access Method:** HTTPS (no authentication required)
- **Source:** NOMADS (NOAA Operational Model Archive and Distribution System)

## Related GFS/GDAS Products

### Regular Lat-Lon Grids (Not Gaussian)
- `gfs.tHHz.pgrb2.0p25.fFFF.grib2` (0.25° global)
- `gfs.tHHz.pgrb2.0p50.fFFF.grib2` (0.5° global)
- `gfs.tHHz.pgrb2.1p00.fFFF.grib2` (1.0° global)

### Gaussian Grid Products
- **GDAS Surface Flux:** `gdas.tCCz.sfluxgrbfFFF.grib2` (T574/T1534)
- **GFS Surface Flux:** `gfs.tCCz.sfluxgrbfFFF.grib2` (T1534)

## References

1. **NCEP GFS Products:** https://www.nco.ncep.noaa.gov/pmb/products/gfs/
2. **GRIB2 Table 3.1:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
3. **GRIB2 Template 3.40:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml
4. **NOMADS:** https://nomads.ncep.noaa.gov/
5. **NOAAPORT:** https://www.weather.gov/noaaport/

## Key Findings Summary

✅ **GFS Gaussian-grid file identified:** GDAS Surface Flux product
✅ **GDT 3.40 confirmed:** grid_template=40 in GRIB2 metadata
✅ **Public URL verified:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2
✅ **Source documented:** NCEP NOMADS, GDAS 2026-07-24 00Z run
✅ **Grid specs documented:** T1534 Gaussian (3072x1536, N=768)
