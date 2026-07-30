# bf-5tu45: GFS Gaussian-grid Fixture ID Identification

## Task

Identify the exact fixture_id for the GFS Gaussian-grid fixture in the test corpus.

## Results

### Available GFS Gaussian-grid Fixtures

The corpus contains **two** GFS Gaussian-grid fixtures:

#### 1. `core_gaussian_gdt40` (Recommended)

- **Fixture ID**: `core_gaussian_gdt40`
- **GRIB2 file**: `tests/corpus/large/flx.2024011500.grib2`
- **Size**: 11 MB
- **Storage**: `remote`
- **Golden JSON**: ✅ EXISTS at `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)
- **Grid specification**:
  - Grid template: GDT 3.40 (Gaussian Latitude/Longitude)
  - Dimensions: 512 x 256
  - Number of latitudes between pole-equator: N=128
  - Total points: 131,072
  - Latitude range: 89.462947°N to -89.462947°S
  - Longitude range: 0°E to 359.296875°E (~0.703° increment)
- **Provenance**: CORe 3-hourly flux file, 2024-01-15 00z
- **Source**: NOAA CORe Archive (Google Cloud Storage)

#### 2. `gfs_gaussian_gdt40_t1534`

- **Fixture ID**: `gfs_gaussian_gdt40_t1534`
- **GRIB2 file**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`
- **Size**: 122 MB
- **Storage**: `remote`
- **Golden JSON**: ❌ DOES NOT EXIST (needs generation)
- **Grid specification**:
  - Grid template: GDT 3.40 (Gaussian Latitude/Longitude)
  - Dimensions: 3072 x 1536 (T1534 resolution)
  - Number of latitudes between pole-equator: N=768
  - Total points: 4,718,592
  - Latitude range: 89.910324°N to -89.910324°S
  - Longitude range: 0°E to 359.882813°E (~0.117° increment)
- **Provenance**: GDAS surface flux analysis, run 2026-07-24 00z, forecast hour F000
- **Source**: NOAA GDAS (NOMADS)

### Primary Fixture ID

**`core_gaussian_gdt40`** is the primary GFS Gaussian-grid fixture used for integration testing.

## Verification

```bash
# List all fixtures
cargo xtask corpus list

# Verify fixture details
cargo xtask corpus show core_gaussian_gdt40

# Check golden JSON exists
ls -lh tests/corpus/golden/core_gaussian_gdt40.json  # 378 MB
```

## References

- Investigation notes: `notes/bf-49ddt.md`
- Manifest entry: `tests/corpus/manifest.json` (line 232)
- Bead history: bf-49ddt (investigation), bf-dag1f (manifest addition), bf-1qia4 (verification)
