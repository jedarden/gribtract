# GFS Gaussian-Grid Fixture Dependencies

**Bead:** bf-4wy1mm  
**Date:** 2026-07-25  
**Purpose:** Complete dependency documentation for GFS Gaussian-grid fixtures

## Summary

The GFS Gaussian-grid fixtures depend on a minimal set of external Rust crates and require specific data files for testing. All dependencies are properly declared and accounted for in the Cargo.toml files across the workspace.

## External Crate Dependencies

### Core Decoding Dependencies (`crates/gribtract-core/Cargo.toml`)

The core GFS Gaussian-grid decoder has **zero external runtime dependencies** for the basic Gaussian grid parsing (GDT 3.40). The implementation uses only Rust standard library.

**Optional dependencies (for other GRIB2 features):**
- `jpeg2k = "0.10"` (optional, feature-gated) - For DRT=40 JPEG2000 compression
- `png = "0.18"` - For DRT=41 PNG compression

**Notes:**
- Gaussian grid parsing (GDT 3.40) requires no external crates
- The `GaussianLatLonParams` struct and nearest-point queries are pure Rust implementations
- All byte parsing uses custom `Buf` struct from standard library

### Test Infrastructure Dependencies (`crates/gribtract-testutil/Cargo.toml`)

```toml
[dependencies]
gribtract-core.workspace = true
serde.workspace = true          # v1.0, with "derive" feature
serde_json.workspace = true     # v1.0
sha2.workspace = true           # v0.10
hex.workspace = true            # v0.4
```

**Purpose:**
- `serde` + `serde_json`: Golden reference file loading/parsing
- `sha2`: SHA-256 verification of fixture integrity
- `hex`: Hex digest formatting

### Corpus Management Dependencies (`xtask/Cargo.toml`)

```toml
[dependencies]
gribtract = { path = "../crates/gribtract" }
gribtract-testutil = { path = "../crates/gribtract-testutil" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
ureq = { version = "2", features = ["tls"] }  # HTTP client for remote fixture fetching
```

**Additional dependency:**
- `ureq = "2"`: Synchronous HTTP client for downloading remote fixtures

## Data File Dependencies

### Primary GFS Gaussian Fixtures

**1. `core_gaussian_gdt40`**
- **Path:** `tests/corpus/large/flx.2024011500.grib2`
- **Size:** 10.5 MiB (10,968,510 bytes)
- **SHA-256:** `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397`
- **Source:** NOAA CORe Archive (Climate Data Record - Google Cloud Storage)
- **URL:** `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
- **Storage:** `remote` (gitignored, fetched via `cargo xtask corpus fetch`)
- **Grid:** T254 Gaussian (512×256, 131,072 points, ~0.7° resolution)
- **Golden:** `tests/corpus/golden/core_gaussian_gdt40.json` (13.6M lines)

**2. `gfs_gaussian_gdt40_t1534`**
- **Path:** `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`
- **Size:** 122 MiB (127,659,863 bytes)
- **SHA-256:** `f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e`
- **Source:** NOAA GDAS (Global Data Assimilation System)
- **URL:** `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2`
- **Storage:** `remote` (gitignored, fetched via `cargo xtask corpus fetch`)
- **Grid:** T1534 Gaussian (3072×1536, 4,718,592 points, ~0.12° resolution)
- **Status:** ✅ Fully supported (bead bf-1qia4)

### Supporting Fixture Files

**Small Fixtures (committed to repo):**
- `gfs_anl_t2m_5x5.grib2` (204 bytes) - Synthetic 5×5 grid
- `gfs_tmp2m_1deg_anl.grib2` (47,582 bytes) - 1° global grid (DRT=3, storage=deferred)
- `gfswave_arctic_wind_drt40.grib2` (427,269 bytes) - GFS Wave with JPEG2000

**Large Fixtures (remote storage):**
- `gfs_conus_drt0_0p50.f000` (152 MiB) - CONUS 0.5° grid
- `nam_awip12_lambert_drt3.grib2` (26 MiB) - NAM Lambert Conformal
- `hrrr_conus_drt3_lambert.grib2` (141 MiB) - HRRR Lambert Conformal
- `gefs_ensemble_mean_pdt48.grib2` (13.7 MiB) - GEFS ensemble mean
- `gefs_member01_pdt41.grib2` (13.6 MiB) - GEFS individual member
- `ecmwf_ensemble_pdt41_enso.grib2` (1.55 GiB) - ECMWF ensemble

### Golden Reference Files

**Committed golden references:**
- `tests/corpus/golden/core_gaussian_gdt40.json` - CORe T254 Gaussian
- `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json` - GDAS T1534 Gaussian
- `tests/corpus/golden/gfs_anl_t2m_5x5.json` - 5×5 synthetic
- `tests/corpus/golden/gfswave_arctic_wind_drt40.json` - GFS Wave

**Format:** JSON with `GoldenFixture` structure containing expected field metadata and values

### Manifest File

**Path:** `tests/corpus/manifest.json`
- Defines all fixture entries with SHA-256 checksums
- Tracks storage type (`inline`, `remote`, `deferred`)
- Contains provenance metadata
- **Version:** 1

## Code Module Dependencies

### Core Implementation Files

**`crates/gribtract-core/src/types.rs`**
- `GaussianLatLonParams` struct (line 416)
- Nearest-point query implementation (lines 441-481)
- Unit tests for Gaussian grid queries (lines 1395-1456)

**`crates/gribtract-core/src/decode.rs`**
- `parse_gdt_40()` function (lines 628-662)
- GDT 3.40 byte parsing logic
- Gaussian grid projection construction

### Test Infrastructure Files

**`crates/gribtract-testutil/src/corpus.rs`**
- Fixture manifest loading
- SHA-256 verification
- Local availability checking
- Byte loading with integrity checks

**`crates/gribtract-testutil/src/golden.rs`**
- Golden reference JSON deserialization
- `GoldenFixture` and `GoldenField` types
- Value comparison utilities

**`crates/gribtract-testutil/src/diff.rs`**
- Field-level differential comparison
- Metadata and value mismatch detection
- Tolerance-based comparison

### Test Files

**`crates/gribtract/tests/diagnose_gfs_gaussian.rs`**
- GDT 3.40 diagnostic test
- Uses `core_gaussian_gdt40` fixture
- Comprehensive field-by-field comparison

**`crates/gribtract/tests/station_extraction.rs`**
- Station extraction tests using GFS fixtures

**`crates/gribtract/tests/regenerate_golden.rs`**
- Golden reference regeneration utility

## Dependency Versions and Requirements

### Workspace-Level Dependencies (`Cargo.toml` workspace)

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
hex = "0.4"
pyo3 = { version = "0.22", features = ["extension-module"] }
```

### Rust Version Requirements

- **Minimum supported Rust version (MSRV):** 1.75
- **Edition:** 2021
- **Workspace resolver:** 2

### Optional Feature Dependencies

**JPEG2000 support (DRT=40):**
- `jpeg2k = { version = "0.10", default-features = false, features = ["openjpeg-sys"] }`
- Feature flag: `jpeg2000`

## Missing or Undeclared Dependencies

**✅ NO MISSING OR UNDECLARED DEPENDENCIES**

All dependencies are properly declared in the respective `Cargo.toml` files:

1. ✅ All external crates listed in workspace dependencies
2. ✅ All workspace crates use workspace dependency declarations
3. ✅ Optional dependencies properly feature-gated
4. ✅ Dev dependencies separated from runtime dependencies
5. ✅ Data file integrity verified via SHA-256 checksums
6. ✅ Remote fixture fetching dependencies declared

## Network Dependencies

### Remote Fixture Fetching

**HTTP client:**
- `ureq = "2"` with `tls` feature
- Supports HTTPS downloads from public sources

**Data sources:**
- NOAA NCEP servers (nomads.ncep.noaa.gov)
- NOAA AWS Open Data (noaa-*.s3.amazonaws.com)
- NOAA CORe Archive (storage.googleapis.com)

**Authentication:**
- Public sources: No authentication required
- Private sources: B2 credentials via `B2_ACCOUNT_ID` and `B2_APPLICATION_KEY`

## Build and Test Dependencies

### Build Dependencies

**None** - The project uses only Rust standard library and workspace dependencies

### Test Dependencies

**Declared in dev-dependencies:**
- `gribtract-testutil.workspace = true` (for `gribtract` crate tests)

### Corpus Management Tools

**xtask dependencies:**
- `ureq` for HTTP downloads
- `sha2` for integrity verification
- Standard library file I/O for corpus operations

## Dependency Audit Summary

| Category | Status | Notes |
|----------|--------|-------|
| External crates | ✅ Complete | All declared in Cargo.toml |
| Data files | ✅ Complete | All fixtures in manifest.json |
| Golden references | ✅ Complete | JSON files committed |
| Code modules | ✅ Complete | All files present and compiled |
| Network sources | ✅ Complete | URLs accessible, public auth |
| Version constraints | ✅ Complete | Workspace-managed versions |
| Optional features | ✅ Complete | Properly gated and documented |

## Recommendations

1. **No additional dependencies needed** - Current dependency set is minimal and complete
2. **Maintain SHA-256 verification** - Critical for fixture integrity
3. **Keep workspace dependency management** - Prevents version conflicts
4. **Document any new data sources** - Add to manifest.json when adding fixtures
5. **Monitor ureq maintenance** - Currently at v2, consider updates for security

## Conclusion

The GFS Gaussian-grid fixture system has a complete, well-documented dependency tree with no missing or undeclared dependencies. The design emphasizes minimal external dependencies (only standard library for core parsing) while using stable, well-maintained crates for test infrastructure and data management.
