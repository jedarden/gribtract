# Bead bf-6cvxy: Add CONUS DRT=0 Entry to Corpus Manifest

## Task
Add a new entry to tests/corpus/manifest.json for the CONUS DRT=0 fixture using the computed SHA256 hash.

## Results

### Entry Already Exists

The CONUS DRT=0 entry was already added to the manifest in a previous commit (6419476: "feat(bf-1laag): add NOAA HRRR CONUS DRT=0 fixture to manifest").

### Existing Entry Details

**Entry ID:** `hrrr_conus_drt0_lambert_20260723`

**Entry JSON:**
```json
{
  "id": "hrrr_conus_drt0_lambert_20260723",
  "path": "large/hrrr.t12z.wrfsfcf00.20260723.grib2",
  "sha256": "22a80611536f4098358ec06eee4db8eb1998ee376da860ba7949c19aaa9adfd0",
  "size_bytes": 142393582,
  "storage": "remote",
  "url": "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2",
  "provenance": {
    "source": "NOAA HRRR (High-Resolution Rapid Refresh, CONUS 3km Lambert Conformal)",
    "description": "HRRR CONUS wrfsfcf analysis, run 2026-07-23 12z, forecast hour F00. GRIB2 messages including DRT 0 (simple packing) messages. Grid: 1799 x 1059 points (1.9M total), 3km resolution, Lambert Conformal projection. Contains MXUPHL (Maximum Updraft Helicity) message 45 with DRT=0 simple packing, extracted as conus_drt0_mxuphl_20260723.grib2 for DRT=0 fixture testing. Coverage: CONUS (La1=21.138N, approximately 20N-55N, 125W-65W). Sourced from noaa-hrrr-bdp-pds S3 (AWS Open Data, public, no auth). Verified via wgrib2: grid_template=30 (Lambert Conformal), packing=grid point data - simple packing (for DRT=0 messages). NOTE: Contains DRT=0 messages suitable for testing simple packing decoder — storage=remote because the 136 MiB file is too large to commit (lives in tests/corpus/large/, gitignored, fetched+sha256-verified by `cargo xtask corpus fetch`).",
    "capture_date": "2026-07-23",
    "generated_by": "curl from noaa-hrrr-bdp-pds.s3.amazonaws.com; verified by wgrib2 (gribtract project)"
  }
}
```

## Acceptance Criteria Verification

All acceptance criteria are met by the existing entry:

- ✅ tests/corpus/manifest.json contains a new entry for the CONUS DRT=0 file - Entry `hrrr_conus_drt0_lambert_20260723` exists
- ✅ storage=remote is set - `"storage": "remote"`
- ✅ sha256 hash is correctly populated - `"sha256": "22a80611536f4098358ec06eee4db8eb1998ee376da860ba7949c19aaa9adfd0"`
- ✅ File size is recorded - `"size_bytes": 142393582`
- ✅ URL is set to the identified NOAA URL - `"url": "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2"`

## Notes

This bead was a split-child of bf-5fuuw (which documented the CONUS DRT=0 fixture download and SHA256 computation). The actual manifest entry was added in bead bf-1laag (commit 6419476).

This bead confirmed that the work was already completed in a prior bead, so no new changes were needed to the manifest.

## Date

2026-07-24
