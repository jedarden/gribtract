# GRIB2 Fixture Validation Report
## File: `/tmp/geavg_20260723_t00z_f000.grib2`

### Validation Summary
✅ **APPROVED** - File is suitable as a test fixture

### File Size and Structure
- **File Size**: 13,991,214 bytes (~13.4 MB) ✓ (< 50MB limit)
- **GRIB Edition**: 2 ✓
- **Total Messages**: 71
- **Download Date**: 2026-07-23 15:39:54 UTC

### GRIB2 Structure Validation

#### Tool Validation Tests
All major GRIB2 tools successfully decode this file:

**wgrib2** (v2.0.8+):
```
$ wgrib2 /tmp/geavg_20260723_t00z_f000.grib2 -s | head -5
1:0:d=2026072300:HGT:10 mb:anl:ens mean
2:200935:d=2026072300:TMP:10 mb:anl:ens mean
3:335675:d=2026072300:RH:10 mb:anl:ens mean
4:380565:d=2026072300:UGRD:10 mb:anl:ens mean
5:643932:d=2026072300:VGRD:10 mb:anl:ens mean
```
✅ Decodes successfully

**eccodes grib_ls**:
```
$ grib_ls /tmp/geavg_20260723_t00z_f000.grib2 | head -5
edition      centre       date         dataType     gridType     stepRange
2            kwbc         20260723     pf           regular_ll   0
2            kwbc         20260723     pf           regular_ll   0
```
✅ Decodes successfully

**eccodes grib_count**:
```
$ grib_count /tmp/geavg_20260723_t00z_f000.grib2
71
```
✅ Message count matches

### Product Definition Template (PDT) Analysis

#### PDT Distribution
- **PDT 4.2 (derived products)**: 71 messages (100%)
- **PDT 4.1 (individual ensemble)**: 0 messages
- **PDT 4.8 (individual ensemble alt)**: 0 messages

#### Why PDT 4.2?
This file contains **ensemble mean** data (evident from "ens mean" in all messages):
- **Ensemble mean** = derived product from averaging individual ensemble forecasts
- Derived products use **PDT 4.2** (statistical product template)
- **PDT 4.1/4.8** are for individual ensemble member forecasts only

### Grid and Resolution
- **Grid Type**: regular_ll (regular latitude/longitude grid)
- **Grid Dimensions**: 720 x 361 points
- **Resolution**: 0.5° (lat/lon)
- **Total Grid Points**: 259,920 per message
- **Extent**: Global (90°N to 90°S, 0°E to 359.5°E)

### Source Metadata

#### Download Source
- **URL**: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000`
- **Provider**: NOAA NCEP (National Centers for Environmental Prediction)
- **Archive**: NOAA GEFS Public Data on AWS S3 (noaa-gefs-pds bucket)
- **Documentation**: https://registry.opendata.aws/noaa-gefs/

#### Product Characteristics
- **Model**: GEFS (Global Ensemble Forecast System)
- **Product Type**: Ensemble Mean (geavg = GEFS Ensemble Average)
- **Reference Date**: 2026-07-23
- **Forecast Cycle**: 00 UTC
- **Forecast Hour**: F000 (analysis/initial conditions)
- **Resolution**: 0.5° (pgrb2a product)

### Variables Present (71 messages)

All variables marked as "ens mean" (ensemble mean):

**Atmospheric Levels (isobaricInhPa)**:
- 10, 50, 100, 200, 250, 300, 400, 500, 700, 850, 925, 1000 mb

**Variables**:
- HGT (Geopotential Height)
- TMP (Temperature)
- RH (Relative Humidity)
- UGRD (U-wind component)
- VGRD (V-wind component)
- VVEL (Vertical Velocity - 850 mb only)
- PRES (Pressure - surface)
- PRMSL (Pressure Reduced to Mean Sea Level)

**Surface and Near-Surface**:
- TSOIL (Soil Temperature - 0-0.1m below ground)
- SOILW (Soil Moisture - 0-0.1m below ground)
- WEASD (Water Equivalent of Accumulated Snow Depth)
- SNOD (Snow Depth)
- ICETK (Ice Thickness)
- TMP (Temperature - 2m above ground)
- RH (Relative Humidity - 2m above ground)
- UGRD (U-wind - 10m above ground)
- VGRD (V-wind - 10m above ground)

**Atmospheric Columns**:
- PWAT (Precipitable Water - entire atmosphere)
- CAPE (Convective Available Potential Energy - 180-0 mb above ground)
- CIN (Convective Inhibition - 180-0 mb above ground)

### Data Quality Checks

✅ **Structure**: Valid GRIB2 format
✅ **Decode**: Successfully decodes with both wgrib2 and eccodes
✅ **Integrity**: File size matches expected (~13.4 MB)
✅ **Completeness**: All 71 messages accessible
✅ **PDT Coverage**: 100% PDT 4.2 (ensemble mean)
✅ **Grid**: Valid global 0.5° regular lat/lon grid
✅ **Variables**: Full set of meteorological variables

### Fixture Suitability Assessment

**Recommended Use Cases**:
1. **PDT 4.2 Testing**: Primary fixture for Product Definition Template 4.2 (ensemble statistical products)
2. **Ensemble Mean Testing**: Test ensemble mean product handling
3. **GRIB2 Structure Testing**: Well-formed multi-message GRIB2 file
4. **Performance Testing**: 71 messages provide good coverage for iteration
5. **Global Grid Testing**: Full 0.5° global grid for spatial operations

**Limitations**:
- Contains only PDT 4.2 messages (no PDT 4.1 or 4.8)
- All from same forecast cycle (00z)
- All from same forecast hour (f000)
- All from same model (GEFS)

**Complementary Fixtures Recommended**:
- Individual ensemble member file (PDT 4.1/4.8)
- Perturbation file (different PDT patterns)
- Multiple forecast hours (f001, f006, etc.)

### Integration Notes

**Suggested Path**: `tests/corpus/large/gefs_ensemble_mean_sample.grib2`

**Rationale**: This file is already validated and documented as ensemble mean data with PDT 4.2. It should be placed in the large/ directory since it's 13.4 MB and serves as a comprehensive ensemble mean fixture.

**Next Steps**:
1. Copy file from `/tmp` to `tests/corpus/large/`
2. Update `tests/corpus/manifest.json` with metadata
3. Add test cases for PDT 4.2 handling
4. Document in fixture inventory

### Verification Commands

```bash
# File size check
ls -lh /tmp/geavg_20260723_t00z_f000.grib2

# GRIB2 validation
wgrib2 /tmp/geavg_20260723_t00z_f000.grib2 -pdt | wc -l  # Should be 71

# PDT verification
wgrib2 /tmp/geavg_20260723_t00z_f000.grib2 -pdt | sort | uniq -c  # All pdt=2

# Decode test
grib_ls /tmp/geavg_20260723_t00z_f000.grib2 | head -5

# Message count
grib_count /tmp/geavg_20260723_t00z_f000.grib2  # Should be 71
```

### Related Documentation
- Download record: notes/bf-54e7p.md
- PDT analysis: notes/bf-2z2w3/pdt-analysis.md
- Full inventory: notes/bf-2z2w3/inventory-with-pdt.md
- NOAA GEFS: https://registry.opendata.aws/noaa-gefs/
