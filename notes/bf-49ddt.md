# bf-49ddt: GFS Gaussian-grid Fixture Investigation

## Task

Identify which GFS Gaussian-grid fixture to use and verify accessibility.

## Investigation Results

### Available GFS Gaussian-grid Fixtures

Two GFS Gaussian-grid fixtures exist in `tests/corpus/manifest.json`:

#### 1. core_gaussian_gdt40

- **Fixture ID**: `core_gaussian_gdt40`
- **GRIB2 file**: `tests/corpus/large/flx.2024011500.grib2`
- **Size**: 11 MB
- **Storage**: `remote` (fetched via `cargo xtask corpus fetch`)
- **Golden JSON**: ✅ EXISTS at `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)
- **Source**: NOAA CORe Archive (Google Cloud Storage)
- **Grid specification**:
  - Grid template: GDT 3.40 (Gaussian Latitude/Longitude)
  - Dimensions: 512 x 256
  - Number of latitudes between pole-equator: N=128
  - Total points: 131,072
  - Latitude range: 89.462947°N to -89.462947°S
  - Longitude range: 0°E to 359.296875°E (~0.703° increment)
- **Provenance**: CORe 3-hourly flux file, 2024-01-15 00z
- **Accessibility**: ✅ VERIFIED - accessible via `cargo xtask corpus fetch`

#### 2. gfs_gaussian_gdt40_t1534

- **Fixture ID**: `gfs_gaussian_gdt40_t1534`
- **GRIB2 file**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`
- **Size**: 122 MB
- **Storage**: `remote` (fetched via `cargo xtask corpus fetch`)
- **Golden JSON**: ❌ DOES NOT EXIST (needs generation)
- **Source**: NOAA GDAS (NOMADS)
- **Grid specification**:
  - Grid template: GDT 3.40 (Gaussian Latitude/Longitude)
  - Dimensions: 3072 x 1536 (T1534 resolution)
  - Number of latitudes between pole-equator: N=768
  - Total points: 4,718,592
  - Latitude range: 89.910324°N to -89.910324°S
  - Longitude range: 0°E to 359.882813°E (~0.117° increment)
- **Provenance**: GDAS surface flux analysis, run 2026-07-24 00z, forecast hour F000
- **Support status**: ✅ Fully supported - gribtract's GDT 3.40 decoder successfully handles T1534 Gaussian grids (per manifest note)
- **Accessibility**: ✅ VERIFIED - accessible via `cargo xtask corpus fetch`

### Recommendation

**Use `core_gaussian_gdt40`** for GFS Gaussian-grid integration:

1. ✅ **Golden JSON exists** - `tests/corpus/golden/core_gaussian_gdt40.json` is present (378 MB)
2. ✅ **Accessible** - verified fetchable via `cargo xtask corpus fetch`
3. ✅ **Appropriate scale** - 131K points (manageable for testing)
4. ✅ **GDT 3.40 coverage** - provides Gaussian grid coverage at smaller resolution
5. ✅ **Production source** - from NOAA CORe archive (climate data record)

The `gfs_gaussian_gdt40_t1534` fixture (while fully supported per bf-1qia4) lacks a golden JSON file and would require generation before use. Its 4.7M points per field also make it more suitable for performance testing than basic integration.

## File Paths Summary

For `core_gaussian_gdt40`:
- **Manifest entry**: `tests/corpus/manifest.json` line 232
- **GRIB2 fixture**: `tests/corpus/large/flx.2024011500.grib2`
- **Golden reference**: `tests/corpus/golden/core_gaussian_gdt40.json`
- **Remote URL**: https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb

For `gfs_gaussian_gdt40_t1534`:
- **Manifest entry**: `tests/corpus/manifest.json` line 246
- **GRIB2 fixture**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`
- **Golden reference**: NOT YET GENERATED (would be `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`)
- **Remote URL**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2

## Verification Commands

```bash
# Verify accessibility of both fixtures
cargo xtask corpus fetch

# Check grid details
wgrib2 -match "" -grid tests/corpus/large/flx.2024011500.grib2 | head -5
wgrib2 -match "" -grid tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2 | head -5

# Verify golden JSON exists for core_gaussian_gdt40
ls -lh tests/corpus/golden/core_gaussian_gdt40.json  # 378 MB

# Verify gfs_gaussian_gdt40_t1534 golden JSON is missing
ls tests/corpus/golden/gfs_gaussian_gdt40_t1534.json  # No such file or directory
```

## Next Steps

1. Use `core_gaussian_gdt40` for GDT 3.40 Gaussian grid integration
2. Generate golden JSON for `gfs_gaussian_gdt40_t1534` if high-resolution testing needed
3. Consider adding both fixtures to differential suite for GDT 3.40 regression testing
