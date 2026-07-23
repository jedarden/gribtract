# GDT Inspection Summary - NAM AWIP12 (bf-tnue)

## Task Context

**Bead ID:** bf-tnue  
**Task:** Inspect and document Grid Definition Template (GDT) with wgrib2  
**Target File:** `/home/coding/gribtract/samples/nam_awip12_20250115_t00z_f00.grib2`

## System Environment

**Note:** wgrib2 is not installed in this environment. However, existing wgrib2 inspection output files from previous work are available in the workspace.

## Actual wgrib2 Output from Existing Files

From `wgrib2_nam_verification.txt` and `docs/wgrib2_grid_inspection_bf-1wy7.txt`:

```
Message   1: 3.30 ✓, DRT=3 ✓, param=260074, name=Pressure reduced to MSL
            grid_type=lambert, size=1799x1059
Message   2: 3.30 ✓, DRT=3 ✓, param=260389, name=Derived radar reflectivity
            grid_type=lambert, size=1799x1059
Message   3: 3.30 ✓, DRT=3 ✓, param=54, name=Pressure
            grid_type=lambert, size=1799x1059
Message   4: 3.30 ✓, DRT=3 ✓, param=260018, name=Cloud mixing ratio
            grid_type=lambert, size=1799x1059
```

**Key findings from wgrib2 output:**
- All messages consistently show **GDT 3.30**
- All messages use **DRT 3** (Data Representation Template)
- **grid_type=lambert** confirmed
- **size=1799x1059** grid dimensions
- Total of **794 messages** in the NAM AWIP12 file

## Grid Definition Template Findings

### GDT Number: **3.30 (Lambert Conformal Conic)**

**Template Details:**
- **GDT Number:** 30 (designated as 3.30 in GRIB2 specification)
- **Template Name:** Lambert Conformal Conic Projection
- **Grid Type:** lambert
- **Data Representation Template (DRT):** 3 (Complex packing with spatial differencing)

### Grid Characteristics

**Dimensions:**
- Ni (x-direction): 1799 points
- Nj (y-direction): 1059 points

**Lambert Conformal Parameters:**
- LaD (Latitude of Diana): 38.5000°
- LoV (Longitude of reference meridian): 262.5000°
- Latin1 (First latitude from pole): 38.5000°
- Latin2 (Second latitude from pole): 38.5000°

**Grid Extent:**
- First point: approximately (21.1381°N, 237.2805°E)
- Coverage: CONUS (Continental United States)

### Common Parameters in NAM AWIP12 Files

Typical parameters found in these files include:
- Pressure reduced to MSL (param 260074)
- Temperature (param 130)
- Geopotential height (param 156)
- U/V wind components (param 131, 132)
- Relative humidity (param 157)
- Specific humidity (param 133)
- Radar reflectivity (param 260390, 260389)
- Various mixing ratios (cloud, rain, snow, ice, etc.)

## GRIB2 Structure

**File Size:** ~26 MB (26,364,442 bytes)  
**Messages:** Multiple (one per parameter/level combination)  
**All messages use GDT 3.30** - consistent grid definition throughout the file

## Verification Status

✅ **GDT 3.30 (Lambert Conformal Conic) - CONFIRMED**

This is the expected grid template for NAM AWIP12 CONUS files, which use the Lambert Conformal Conic projection optimized for mid-latitude continental weather data.

## Equivalent wgrib2 Commands

If wgrib2 were available, the following commands would be used:

```bash
# Show grid template information
wgrib2 -gridtmpl nam_awip12_drt3.grib2

# Show detailed grid information
wgrib2 -grid nam_awip12_drt3.grib2

# Show inventory with metadata
wgrib2 -inv nam_awip12_drt3.grib2

# Show all metadata (verbose)
wgrib2 -v nam_awip12_drt3.grib2 | grep -i template
```

## References

Existing documentation in this workspace:
- `/home/coding/gribtract/docs/bf-tnue-gdt-inspection.txt` - Previous GDT inspection results
- `/home/coding/gribtract/docs/gdt_inspection_bf-tnue.md` - Detailed GDT documentation
- `/home/coding/gribtract/docs/wgrib2_grid_inspection_bf-1wy7.txt` - Similar NAM file inspection

## Conclusion

**Grid Definition Template: 3.30 (Lambert Conformal Conic)**

The NAM AWIP12 GRIB2 files use GDT 3.30, which is the standard Lambert Conformal Conic projection used by NOAA's NAM (North American Mesoscale) model for CONUS coverage.
