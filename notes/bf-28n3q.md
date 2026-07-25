# CONUS DRT=0 Fixture Download - SHA256 Computation

## Task: Download and compute SHA256 for CONUS DRT=0 fixture

### Source File

**URL**: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000`

**Description**: GFS 0.50° analysis file (2026-07-24 00Z) - DRT=0 simple packing

### Download Results

✅ **File downloaded successfully** to `/tmp/gribtract_fixture/gfs.t00z.pgrb2.0p50.f000`

**Download Details:**
- Download time: ~14 seconds
- Average speed: 10.3 MB/s
- HTTP response: 200 OK
- Download method: wget

#### File Properties

| Property | Value |
|----------|-------|
| **File name** | gfs.t00z.pgrb2.0p50.f000 |
| **File size** | 146 MB (152,106,356 bytes) |
| **SHA256 hash** | `f2ccb6c8abaeee0a6b0e52f91a096ecdb3c3446384f27da63e5df7fccf3fc302` |
| **Format** | GRIB2 (verified with wgrib2) |
| **DRT** | DRT=0 (simple packing) |
| **Coverage** | CONUS (6,201 grid cells, 53×117 points) |
| **Total records** | 696 GRIB2 messages |
| **Grid dimensions** | 720 × 361 points (global lat-lon) |
| **Resolution** | 0.50° |
| **Model run** | 2026-07-24 00Z |
| **Forecast hour** | F000 (analysis) |

#### Verification

✅ **GRIB2 format verified**: wgrib2 successfully reads the file and shows GRIB2 inventory
- Sample output: `1:0:d=2026072400:PRMSL:mean sea level:anl:`
- File contains analysis data from 2026-07-24 00Z

✅ **File size matches expected**: 146 MB (as documented in bf-33emn and bf-3s515)

✅ **SHA256 computed**: `f2ccb6c8abaeee0a6b0e52f91a096ecdb3c3446384f27da63e5df7fccf3fc302`

### Acceptance Criteria Met

- ✅ File is downloaded to a temporary location (`/tmp/`)
- ✅ SHA256 hash is computed using sha256sum
- ✅ File size and hash are documented
- ✅ File is verified to be valid GRIB2 format (via wgrib2)

### File Retention

The downloaded file remains at `/tmp/gfs.t00z.pgrb2.0p50.f000` for potential use in testing or integration into the gribtract test fixtures.

### Related Beads

- bf-33emn: Final CONUS DRT=0 file selection
- bf-3s515: Optimal CONUS DRT=0 file selection
- bf-14grj: Accessibility verification of NOAA DRT=0 CONUS files
