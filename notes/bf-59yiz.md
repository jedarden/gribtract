# Bead bf-59yiz: NOAA CONUS DRT=0 File URL Identification

## Task
Research public NOAA archives to find a GRIB2 file covering CONUS (CONterminous US) with DRT=0 (simple packing, no data representation template complexity).

## Results

### Identified URL

**Primary URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

### URL Pattern and Access

The NOAA HRRR (High-Resolution Rapid Refresh) dataset provides CONUS coverage with DRT=0 messages through the AWS Open Data S3 bucket:

**General Pattern:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

Where:
- `YYYYMMDD`: Date (e.g., `20260723`)
- `CC`: Cycle hour (00, 01, ..., 23)
- `FF`: Forecast hour (00, 01, ..., 48)

**Verified Accessibility:**
```bash
curl -I "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2"
# Returns HTTP 200 OK
```

### Dataset Characteristics

**Source:** NOAA HRRR (High-Resolution Rapid Refresh)
**Coverage:** CONUS (Continental United States)
**Resolution:** 3km (1799 × 1059 grid points = 1.9M points)
**Projection:** Lambert Conformal Conic (GRIB2 GDT 30)
**Format:** GRIB2 with mixed DRT messages (including DRT=0 simple packing)

**DRT=0 Content:**
The HRRR CONUS files contain multiple GRIB2 messages, including messages with DRT=0 (simple packing). For example, message 45 (MXUPHL - Maximum Updraft Helicity) uses DRT=0 simple packing.

### Coverage Details

**Geographic Coverage:**
- Latitude: ~21°N to ~50°N
- Longitude: ~125°W to ~70°W
- Covers the lower 48 United States (CONUS)

### Alternative Access Points

1. **AWS Open Data Registry:**
   - https://registry.opendata.aws/noaa-hrrr-pds/

2. **Brian Blaylock's HRRR Download Tool:**
   - https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi

3. **NCEI Archive:**
   - Historical HRRR data available through NOAA's National Centers for Environmental Information

### Verification Status

✅ **URL identified and documented**
✅ **CONUS geographic coverage verified** (HRRR covers CONUS at 3km resolution)
✅ **DRT=0 presence verified** (confirmed via wgrib2 inspection showing DRT=0 messages)
✅ **Public accessibility confirmed** (HTTP 200 OK response)

## Acceptance Criteria Met

- ✅ A NOAA public URL is identified for a GRIB2 file
- ✅ File covers CONUS geographic region
- ✅ File uses DRT=0 (simple packing) - verified within the multi-message GRIB2 file
- ✅ URL is documented in notes

## References

- Previous verification documented in: `notes/bf-1ftw0.md` (download and SHA256 verification)
- Station coverage verification: `notes/bf-4z73r.md` (CONUS coverage confirmed)
- HRRR dataset documentation: https://www.nco.ncep.noaa.gov/pmb/products/hrrr/

## Date

2026-07-24
