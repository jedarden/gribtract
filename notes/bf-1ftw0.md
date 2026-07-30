# Bead bf-1ftw0: NOAA CONUS DRT=0 Fixture Download & Verification

## Task
Download the identified NOAA DRT=0 file and compute its SHA256 hash. This file will be stored as a storage=remote fixture (not committed to git).

## Results

### File Downloaded

**URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

**Local Path:** `tests/corpus/large/hrrr.t12z.wrfsfcf00.20260723.grib2`

**File Size:** 142,393,582 bytes (~135.8 MiB)

**SHA256:** `22a80611536f4098358ec06eee4db8eb1998ee376da860ba7949c19aaa9adfd0`

### Verification

- ✅ File downloaded from NOAA HRRR BDP S3 (AWS Open Data)
- ✅ SHA256 hash computed and verified
- ✅ File size recorded
- ✅ Added to manifest.json with `storage=remote`
- ✅ Hash and size documented in manifest entry `hrrr_conus_drt0_lambert_20260723`

### Manifest Entry

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
    "description": "HRRR CONUS wrfsfcf analysis, run 2026-07-23 12z, forecast hour F00. GRIB2 messages including DRT 0 (simple packing) messages. Grid: 1799 x 1059 points (1.9M total), 3km resolution, Lambert Conformal projection. Contains MXUPHL (Maximum Updraft Helicity) message 45 with DRT=0 simple packing, extracted as conus_drt0_mxuphl_20260723.grib2 for DRT=0 fixture testing. Coverage: CONUS (La1=21.138N, approximately 20N-55N, 125W-65W). Sourced from noaa-hrrr-bdp-pds S3 (AWS Open Data, public, no auth).",
    "capture_date": "2026-07-23",
    "generated_by": "curl from noaa-hrrr-bdp-pds.s3.amazonaws.com; verified by wgrib2 (gribtract project)"
  }
}
```

## Notes

The file is stored with `storage=remote` which means:
- It lives in `tests/corpus/large/` (gitignored)
- Not committed to git due to size (>1MB)
- Fetched and sha256-verified via `cargo xtask corpus fetch`
- Serves as a real-world fixture for testing DRT=0 (simple packing) decoder with CONUS Lambert Conformal projection data

## Acceptance Criteria Met

- ✅ File downloaded from NOAA URL
- ✅ SHA256 hash computed
- ✅ File size recorded
- ✅ Hash and size documented in manifest.json
