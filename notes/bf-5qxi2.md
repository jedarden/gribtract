# bf-5qxi2: Fixture Download Verification

## Task
Verify both fixtures download with cargo xtask

## Execution Summary

### Initial Fetch (2026-07-23 14:57 UTC)
Ran `cargo xtask corpus fetch` to download and verify both fixtures:

**Downloaded:**
- `gefs_ensemble_mean_pdt48` — 13,344.2 KB (13.7 MB) — GEFS ensemble mean, PDT 4.8
  - Source: https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
  - SHA256: `bb2c90188ec6370aca833b55118b199097c21ef978f0b5e6fe8ab9e9955b3158` ✅

- `core_gaussian_gdt40` — 10,711.4 KB (11.0 MB) — CORe Gaussian-grid, GDT 3.40
  - Source: https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb
  - SHA256: `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` ✅

**Already Present:**
- nam_awip12_lambert_drt3 (26.4 MB)
- nam_awip12_lambert_drt3_20250120 (27.0 MB)
- hrrr_conus_drt3_lambert (141.3 MB)

**Result:** 2 downloaded, 3 already present, 0 failed

### Verification (2026-07-23 15:01 UTC)
Confirmed both fixtures stored in `tests/corpus/large/`:
- `flx.2024011500.grib2` (11M)
- `gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2` (14M)

SHA256 hashes verified via `sha256sum` — both match manifest.json exactly.

### Re-run Verification (2026-07-23 15:02 UTC)
Re-ran `cargo xtask corpus fetch` — both fixtures now reported as:
```
[ok]      gefs_ensemble_mean_pdt48 (already present, sha256 matches)
[ok]      core_gaussian_gdt40 (already present, sha256 matches)
```

## Acceptance Criteria Status
- ✅ cargo xtask corpus fetch completes without errors
- ✅ Both ensemble and Gaussian-grid fixtures download successfully
- ✅ sha256 hash validation passes for both fixtures
- ✅ Files are correctly stored in the corpus directory (tests/corpus/large/)

## Fixture Details

### gefs_ensemble_mean_pdt48
- **ID:** gefs_ensemble_mean_pdt48
- **Path:** large/gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2
- **Size:** 13,664,431 bytes (13.7 MB)
- **GDT:** 0 (lat/lon)
- **PDT:** 4.8 (ensemble mean statistical product)
- **Messages:** 71 GRIB2 messages
- **Grid:** 0.5° global lat/lon
- **Source:** NOAA GEFS (Global Ensemble Forecast System)

### core_gaussian_gdt40
- **ID:** core_gaussian_gdt40
- **Path:** large/flx.2024011500.grib2
- **Size:** 10,968,510 bytes (11.0 MB)
- **GDT:** 3.40 (Gaussian Latitude/Longitude)
- **Grid:** 512 x 256 Gaussian grid (131,072 points)
- **Source:** NOAA CORe Archive (Climate Data Record)

## Conclusion
Both fixtures downloaded successfully, passed SHA256 validation, and are correctly stored in the corpus directory. The `cargo xtask corpus fetch` workflow is functioning correctly for remote fixtures.
