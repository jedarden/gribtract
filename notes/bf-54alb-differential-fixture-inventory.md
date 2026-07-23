# Differential Fixture Inventory

Generated: 2026-07-23

## Summary

- **Total fixtures**: 12
- **Inline storage**: 9
- **Remote storage**: 3 (large files, fetched via `cargo xtask corpus fetch`)
- **With golden references**: 10
- **Without golden references**: 2
- **Current pass rate**: 100% (default features), 90% (with `jpeg2000` feature)

---

## DRT Type Distribution

| DRT | Template | Description | Count | Status |
|-----|----------|-------------|-------|--------|
| 0 | 5.0 | Simple packing | 3 | ✅ PASS |
| 2 | 5.2 | Complex packing, no spatial differencing | 1 | ✅ PASS |
| 3 | 5.3 | Complex packing with spatial differencing | 3 | ✅ PASS (1 multi-message) |
| 40 | 5.40 | JPEG2000 compression | 2 | ⚠️ 1 PASS, 1 FAIL |
| 41 | 5.41 | PNG compression | 2 | ✅ PASS |

---

## Detailed Fixture List

### ✅ PASSING FIXTURES (Default Features, 100% Agreement)

#### 1. gfs_anl_t2m_5x5
- **DRT**: 0 (simple packing)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: Synthetic (scripts/gen_grib2.py)
- **Grid**: 5x5 lat/lon, 10° spacing, 0-40N/0-40E
- **Values**: 25 points (270-294 K temperature, 2m above ground)
- **Notes**: Minimal GRIB2 fixture, tests basic Section 3.0/4.0/5.0 templates

#### 2. drt2_simple_3x3
- **DRT**: 2 (complex packing, no spatial differencing)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: Synthetic (scripts/gen_grib2_drt2.py)
- **Grid**: 3x3 lat/lon, 10° spacing, 0-20N/0-20E
- **Values**: 9 points (100-108, R=100.0 E=0 D=0 N=8)
- **Notes**: Minimal DRT=2 fixture, 1 group, tests template 5.2

#### 3. gfs_tmp2m_1deg_anl
- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: NOAA NCEP GFS (2026-06-18 cycle)
- **Grid**: 360x181 global lat/lon (65160 points), 1° spacing
- **Values**: 2m temperature analysis
- **Notes**: First real-world DRT=3 fixture to pass

#### 4. drt40_j2k_3x2
- **DRT**: 40 (JPEG2000)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Feature**: `jpeg2000` required
- **Source**: Synthetic (scripts/gen_grib2_drt40.py)
- **Grid**: 3x2 lat/lon, 10° spacing
- **Values**: 6 points (100-105, lossless J2K)
- **Notes**: Minimal DRT=40 fixture, requires `jpeg2000` feature

#### 5. drt41_png_3x2
- **DRT**: 41 (PNG compression)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: Synthetic (scripts/gen_grib2_drt41.py)
- **Grid**: 3x2 lat/lon, 10° spacing
- **Values**: 6 points (100-105, PNG lossless)
- **Notes**: Minimal DRT=41 fixture

#### 6. pdt1_ensemble_3x2
- **DRT**: 0 (simple packing)
- **GDT**: 0 (lat/lon)
- **PDT**: 1 (individual ensemble member)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: Synthetic (scripts/gen_grib2_pdt1.py)
- **Grid**: 3x2 lat/lon, 10° spacing, 20-30N/0-20E
- **Ensemble**: Type=2 (negatively perturbed), perturbation=3, n=20
- **Values**: 6 points (250-255 K, 500 hPa isobaric)
- **Notes**: Tests PDT=1 ensemble member metadata

#### 7. pdt8_accum_3x2
- **DRT**: 0 (simple packing)
- **GDT**: 0 (lat/lon)
- **PDT**: 8 (time-processed statistical product)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: Synthetic (scripts/gen_grib2_pdt8.py)
- **Grid**: 3x2 lat/lon, 10° spacing, 20-30N/0-20E
- **Values**: 6 points (0.0-5.0 kg/m², 6-hour precipitation accumulation)
- **Notes**: Tests PDT=8 temporal aggregation metadata

#### 8. gfswave_arctic_wind_drt40
- **DRT**: 40 (JPEG2000, lossy J=0)
- **GDT**: 20 (polar stereographic)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Feature**: `jpeg2000` required
- **Source**: NOAA GFS Wave (2026-06-20)
- **Grid**: 1,012,036 points (Arctic 9km polar stereographic)
- **Bitmap**: Present (1M bits, masks land/ice)
- **Values**: 360,052 ocean points (10m wind speed m/s)
- **Status**: ⚠️ **MISMATCH** with jpeg2000 feature enabled
- **Notes**: Large DRT=40 fixture with bitmap, currently failing

#### 9. mrms_carib_refl_drt41
- **DRT**: 41 (PNG compression)
- **GDT**: 0 (lat/lon)
- **PDT**: 0 (analysis)
- **Storage**: inline
- **Golden**: ✅ Yes
- **Source**: NOAA MRMS Caribbean (2026-06-20)
- **Grid**: 4,500,000 points (3000x1500 Caribbean)
- **Values**: Radar reflectivity (dBZ), missing=-999.0
- **Notes**: Large DRT=41 fixture, tests PNG decoding with missing values

---

### ⚠️ FIXTURES WITHOUT GOLDEN (Not Yet Testable)

#### 10. nam_awip12_lambert_drt3_20250120
- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 30 (Lambert Conformal Conic)
- **PDT**: 0 (analysis)
- **Storage**: remote (26.3 MiB)
- **Golden**: ❌ No golden generated yet
- **Source**: NOAA NAM awip12 (2025-01-20 00z)
- **Grid**: 614x428 (262,792 points), NCEP Grid 218
- **Messages**: 196 GRIB2 messages, all GDT 3.30 + DRT 3
- **Notes**: Target fixture for DRT=3 + Lambert decoder testing

#### 11. hrrr_conus_drt3_lambert
- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 30 (Lambert Conformal Conic)
- **PDT**: 0/8 (analysis and time-processed)
- **Storage**: remote (135 MiB)
- **Golden**: ❌ No golden generated yet
- **Source**: NOAA HRRR CONUS (2024-06-01 12z)
- **Grid**: 1059x1799 (1.9M points), 3km resolution
- **Messages**: 170 GRIB2 messages
  - 165 messages: GDT 3.30 + PDT 0 + DRT 3
  - 5 messages: GDT 3.30 + PDT 8 + DRT 3
- **Fields**: REFC, UGRD/VGRD (multiple levels), HGT, TMP, DPT, 165+ fields
- **Notes**: Target fixture for DRT=3 + Lambert decoder testing

---

### ✅ MULTI-MESSAGE FIXTURE (Remote Storage, Currently Passing)

#### 12. nam_awip12_lambert_drt3
- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 30 (Lambert Conformal Conic)
- **PDT**: 0/8 (analysis and time-processed)
- **Storage**: remote (25.1 MiB)
- **Golden**: ✅ Yes
- **Source**: NOAA NAM awip12 (2025-01-15 00z)
- **Grid**: 614x428 (262,792 points), NCEP Grid 218
- **Messages**: 196 GRIB2 messages
  - 187 messages: GDT 3.30 + PDT 0 + DRT 3
  - 9 messages: GDT 3.30 + PDT 8 + DRT 3
- **Status**: ✅ **PASS** (all 196 messages match golden)
- **Notes**: Large multi-message fixture, tests DRT=3 + Lambert decoder coverage

---

## Status by Feature Flag

### Default Features (jpeg2000 disabled)
- **Fixtures tested**: 10
- **Fixtures comparable**: 8 (2 DRT=40 fixtures skipped)
- **Agreement**: 8/8 (100%)
- **Skipped**: 2 (drt40_j2k_3x2, gfswave_arctic_wind_drt40 - require `jpeg2000`)

### With `--features jpeg2000`
- **Fixtures tested**: 12
- **Fixtures comparable**: 10 (2 no-golden)
- **Agreement**: 9/10 (90%)
- **Failures**: 1 (gfswave_arctic_wind_drt40 - GDT=20/DRT=40 mismatch)
- **No golden**: 2 (nam_awip12_lambert_drt3_20250120, hrrr_conus_drt3_lambert)

---

## Known Issues and Edge Cases

### gfswave_arctic_wind_drt40 (MISMATCH)
- **Issue**: Fails to match golden reference when jpeg2000 feature is enabled
- **GDT**: 20 (polar stereographic) - large grid with 1M+ points
- **Bitmap**: Present (360k ocean points present, land/ice masked)
- **Expectations**: This is a complex fixture combining:
  - Polar stereographic grid (GDT=20)
  - JPEG2000 compression (DRT=40)
  - Large bitmap (1M bits)
  - Lossy J=0 encoding
- **Current behavior**: Decodes but values don't match golden reference
- **Notes**: May indicate issue with bitmap handling, polar stereographic grid, or lossy JPEG2000 decoding

### Remote fixtures (Large files)
- **Storage**: `tests/corpus/large/` (gitignored)
- **Fetch**: `cargo xtask corpus fetch <id>`
- **Verification**: SHA-256 verified on load
- **Size constraint**: Only fixtures >~20 MiB should use remote storage

### Missing golden references
- **nam_awip12_lambert_drt3_20250120**: Golden not yet generated
- **hrrr_conus_drt3_lambert**: Golden not yet generated
- **Next step**: Generate golden references via internal cluster with eccodes

---

## Template Coverage Summary

### Grid Definition Templates (GDT)
- **GDT=0** (lat/lon): 8 fixtures ✅
- **GDT=20** (polar stereographic): 1 fixture ⚠️ (gfswave_arctic_wind_drt40 - failing)
- **GDT=30** (Lambert Conformal Conic): 3 fixtures ✅ (1 passing multi-message, 2 no-golden)

### Product Definition Templates (PDT)
- **PDT=0** (analysis/forecast): 10 fixtures ✅
- **PDT=1** (ensemble member): 1 fixture ✅
- **PDT=8** (time-processed): 3 fixtures ✅ (1 multi-message with mixed PDT)

### Data Representation Templates (DRT)
- **DRT=0** (simple packing): 3 fixtures ✅
- **DRT=2** (complex packing, no spatial differencing): 1 fixture ✅
- **DRT=3** (complex packing with spatial differencing): 3 fixtures ✅
- **DRT=40** (JPEG2000): 2 fixtures ⚠️ (1 passing, 1 failing)
- **DRT=41** (PNG): 2 fixtures ✅

---

## Recommendations

1. **Fix gfswave_arctic_wind_drt40**: Investigate bitmap handling, polar stereographic grid decoding, or lossy JPEG2000 issues
2. **Generate missing golden references**: Create golden files for nam_awip12_lambert_drt3_20250120 and hrrr_conus_drt3_lambert
3. **Consider lowering AGREEMENT_FLOOR**: Temporarily set to 90% to allow gfswave_arctic_wind_drt40 to run while debugging
4. **Add mixed DRT fixtures**: Consider adding fixtures that mix multiple DRT types in single GRIB2 files
5. **Add bitmap fixtures**: Add more fixtures with bitmaps to test masked value handling

---

## Appendix: Test Execution Commands

```bash
# Run differential test (default features - DRT=40 fixtures skipped)
cargo test --test differential

# Run differential test with JPEG2000 support (includes DRT=40 fixtures)
cargo test --test differential --features jpeg2000

# Fetch remote fixtures
cargo xtask corpus fetch nam_awip12_lambert_drt3_20250120
cargo xtask corpus fetch hrrr_conus_drt3_lambert

# Generate golden reference (requires eccodes)
# (Not yet documented in codebase)
```
