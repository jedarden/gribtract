# Bead bf-5fuuw: CONUS DRT=0 Fixture Download and SHA256 Computation

## Task
Download the identified NOAA CONUS DRT=0 GRIB2 file and compute its SHA256 hash.

## Results

### Downloaded File

**Source URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

**Local Path:** `fixtures/conus/hrrr.t12z.wrfsfcf00.grib2`

**File Size:** 134M (139,571,312 bytes)

### SHA256 Hash

```
bd819c1bb741683b951f5a79094188e0836261bbdd86f5b17ea3a445a13ba2c0
```

### Verification Commands Used

```bash
# Download
curl -L -o fixtures/conus/hrrr.t12z.wrfsfcf00.grib2 \
  "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2"

# SHA256 computation
sha256sum fixtures/conus/hrrr.t12z.wrfsfcf00.grib2

# File size
ls -lh fixtures/conus/hrrr.t12z.wrfsfcf00.grib2
```

## Acceptance Criteria Met

- ✅ File is downloaded from the identified NOAA URL
- ✅ SHA256 hash is computed using sha256sum
- ✅ File size is recorded (134M)
- ✅ SHA256 and file size are documented (ready for manifest entry)

## Manifest Entry Data

For future reference in `fixtures/conus/manifest.jsonl`:

```json
{
  "name": "hrrr.t12z.wrfsfcf00.grib2",
  "description": "NOAA HRRR CONUS surface analysis - contains DRT=0 messages",
  "source_url": "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2",
  "sha256": "bd819c1bb741683b951f5a79094188e0836261bbdd86f5b17ea3a445a13ba2c0",
  "size_bytes": 139571312,
  "coverage": "CONUS",
  "projection": "Lambert Conformal Conic",
  "resolution": "3km",
  "grid_points": "1799x1059",
  "drt_messages": "mixed (includes DRT=0)"
}
```

## References

- URL identification documented in: `notes/bf-59yiz.md`
- Previous SHA256 verification: `notes/bf-1ftw0.md`

## Date

2026-07-24
