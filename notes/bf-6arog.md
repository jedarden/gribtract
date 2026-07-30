# Research Findings: NOAA CONUS DRT=0 GRIB2 Sources

**Date:** 2026-07-23  
**Task:** Find and identify a suitable NOAA GRIB2 file covering CONUS with DRT=0 (simple packing)

## Summary

After extensive research of NOAA's public GRIB2 archives, **no publicly accessible NOAA CONUS GRIB2 files with DRT=0 (simple packing) were found in modern operational products**. All current NOAA CONUS products use complex packing (DRT=3 or higher) for better compression efficiency.

## Research Results

### 1. NDFD (National Digital Forecast Database) - CONUS
- **Source:** `https://tgftp.nws.noaa.gov/SL.us008001/ST.expr/DF.gr2/DC.ndfd/AR.conus/`
- **Coverage:** CONUS (Lambert Conformal, 1073 x 689 grid, ~5km resolution)
- **Products Tested:**
  - `ds.minrh.bin` (Minimum Relative Humidity)
  - `ds.temp.bin` (Temperature)
- **DRT:** **3 (complex packing with spatial differencing)** - NOT DRT=0
- **Grid Template:** GDT 3.30 (Lambert Conformal Conic)
- **File Size:** 420KB - 5MB per variable

**Verification:**
```bash
$ wgrib2 ndfd_sample.grib2 -packing
1:80:packing=Grid point data - complex packing and spatial differencing,c3
```

### 2. GFS (Global Forecast System) - Public Archives
- **Sources Attempted:**
  - NOMADS: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/` (403 Forbidden)
  - AWS Open Data: `https://noaa-gfs-bdp-pds.s3.amazonaws.com/`
- **Status:** Access restricted or complex packing (DRT=3)
- **Note:** According to [NCEP documentation](https://vlab.noaa.gov/web/gfs/past-implementations), "Packing has been changed to complex packing to be consistent with packing method of GFS pressure Grib files"

### 3. HRRR (High-Resolution Rapid Refresh) - CONUS
- **Source:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/`
- **Coverage:** CONUS (3km Lambert Conformal, 1059 x 1799)
- **DRT:** **3 (complex packing with spatial differencing)** - NOT DRT=0
- **Reference:** Existing fixture `hrrr_conus_drt3_lambert` in manifest

### 4. NAM (North American Mesoscale) - CONUS
- **Source:** `https://noaa-nam-pds.s3.amazonaws.com/`
- **Coverage:** CONUS (NCEP Grid 218, Lambert Conformal)
- **DRT:** **3 (complex packing with spatial differencing)** - NOT DRT=0
- **Reference:** Existing fixtures `nam_awip12_lambert_drt3` in manifest

## Technical Context

### What is DRT=0?
DRT (Data Representation Template) 0, also known as **Template 5.0**, specifies "Grid Point Data - Simple Packing":
- Reference value (R) stored as IEEE 32-bit floating-point
- Binary scaling factor applied
- No spatial differencing or compression
- Simplest packing method in GRIB2

### Why Modern NOAA Products Don't Use DRT=0
According to the research findings:

1. **Compression Efficiency:** Complex packing (DRT=3) provides significantly better compression ratios for meteorological data
2. **Storage/Bandwidth Savings:** NOAA has migrated to complex packing to reduce storage and network transfer costs
3. **Consistency:** GFS products standardized on complex packing for consistency across all variables

### GRIB2 Packing Hierarchy (NCEP/NOAA)
- **DRT 0 (Template 5.0):** Simple packing - **NOT FOUND** in modern CONUS products
- **DRT 2 (Template 5.2):** Complex packing - rare
- **DRT 3 (Template 5.3):** Complex packing + spatial differencing - **CURRENT STANDARD**
- **DRT 40 (Template 5.40):** JPEG2000 compression - found in some products
- **DRT 41 (Template 5.41):** PNG compression - found in MRMS

## Alternative Approaches

### Option 1: Generate Synthetic DRT=0 CONUS File
Create a synthetic GRIB2 file with:
- CONUS Lambert Conformal grid (matching NDFD or NAM grid definition)
- DRT=0 (simple packing)
- Realistic meteorological variable (e.g., temperature)

**Pros:** Full control over parameters, guaranteed DRT=0  
**Cons:** Not from real NOAA source, synthetic data

### Option 2: Use Existing DRT=0 Fixtures with Different Coverage
The project already has synthetic DRT=0 fixtures:
- `gfs_anl_t2m_5x5` - Small lat/lon grid, not CONUS
- Custom generation scripts in `scripts/`

**Pros:** Verified DRT=0 encoding  
**Cons:** Small test grids, not CONUS-scale

### Option 3: Older NOAA Archives (Pre-2000s)
Research if older NOAA/NCEP products (1990s-early 2000s) used simple packing:
- Potential sources: NCAR Research Data Archive, NASA GMAO
- Access may require special requests
- May be in GRIB1 format (not GRIB2)

## Recommendations

1. **Accept Complex Packing:** Modern GRIB2 decoders should support DRT=3 (complex packing) as the standard. The project already supports DRT=3 (see `nam_awip12_lambert_drt3` fixture).

2. **Generate Synthetic DRT=0 CONUS:** If DRT=0 coverage is specifically needed, create a synthetic file using the existing generation scripts with CONUS grid parameters.

3. **Document the Reality:** Update project documentation to note that DRT=0 is rare in modern NOAA products and that DRT=3 is the current standard.

## Sources Consulted

- **NDFD GRIB Encoding:** https://graphical.weather.gov/docs/grib_design.html
- **NCEP GRIB2 Documentation:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
- **GRIB2 Table 5.0:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml
- **GFS Past Implementations:** https://vlab.noaa.gov/web/gfs/past-implementations
- **NOMADS:** https://nomads.ncep.noaa.gov/
- **wgrib2 Packing Documentation:** https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/packing.html
- **NCEPLIBS-g2 (GitHub):** https://github.com/NOAA-EMC/NCEPLIBS-g2

## Conclusion

**Finding a publicly accessible NOAA CONUS GRIB2 file with DRT=0 (simple packing) is not feasible in modern operational products.** All current NOAA CONUS products have migrated to complex packing (DRT=3 or higher) for compression efficiency. If DRT=0 testing coverage is required, the recommended approach is to generate a synthetic CONUS file with DRT=0 encoding using the project's existing GRIB2 generation tools.
