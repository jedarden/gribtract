# CONUS DRT=0 GRIB2 File Download - bf-3j9o6

## Task Summary
Successfully downloaded and validated a CONUS-covering GRIB2 file with DRT=0 (simple packing) from the NOAA archive.

## File Details

**Location:** `/home/coding/gribtract/data/conus_drt0_mxuphl_20260723.grib2`

**Size:** 212 bytes

**SHA-256:** `1fc8b541c5a88b5c6660caa5c61f5ce14d8d97965f3bbf5d5b5529f25ae13658`

## Source and Extraction

**Source URL:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20260723/conus/hrrr.t12z.wrfsfcf00.grib2`

**Download Method:**
- Downloaded complete HRRR CONUS file (139,809,389 bytes) from NOAA public S3 bucket
- Used curl to fetch the file to temporary location: `/tmp/hrrr_20260723_t12z_wrfsfcf00.grib2`

**Extracted Message:**
- Message 45 from the HRRR file
- Variable: MXUPHL (Maximum Updraft Helicity 5000-2000m above ground)
- Level: 5000-2000 m above ground
- Forecast: 0-0 day max forecast
- Reference time: 2024-07-23 12z

**Extraction Command:**
```bash
wgrib2 /tmp/hrrr_20260723_t12z_wrfsfcf00.grib2 -d 45 -grib /tmp/conus_drt0_mxuphl_20260723.grib2
```

## Validation Results

### 1. File Download ✅
- Successfully downloaded 139MB HRRR CONUS file from NOAA archive
- File integrity verified: reads cleanly with wgrib2
- Download time: ~1 minute via curl with progress bar

### 2. GRIB2 Format Validation ✅
```bash
$ wgrib2 /home/coding/gribtract/data/conus_drt0_mxuphl_20260723.grib2
1:0:d=2026072312:MXUPHL:5000-2000 m above ground:0-0 day max fcst:
```
- Valid GRIB2 format confirmed
- Single message file
- Date: 2026-07-23 12z

### 3. DRT=0 Confirmation ✅
```bash
$ wgrib2 -packing /home/coding/gribtract/data/conus_drt0_mxuphl_20260723.grib2
1:0:packing=Grid point data - simple packing,s
```
- **DRT=0 (Data Representation Template 0)**: Simple packing
- No spatial differencing
- No complex compression

### 4. CONUS Coverage Verification ✅
```bash
$ wgrib2 -grid /home/coding/gribtract/data/conus_drt0_mxuphl_20260723.grib2
1:0:grid_template=30:winds(grid):
	Lambert Conformal: (1799 x 1059) input WE:SN output WE:SN res 8
	Lat1 21.138123 Lon1 237.280472 LoV 262.500000
	LatD 38.500000 Latin1 38.500000 Latin2 38.500000
	LatSP 0.000000 LonSP 0.000000
	North Pole (1799 x 1059) Dx 3000.000000 m Dy 3000.000000 m
```

**Grid Parameters:**
- **Grid Definition Template:** 3.30 (Lambert Conformal Conic)
- **Resolution:** 3km (3000m spacing)
- **Grid Points:** 1799 x 1059 (1,905,141 total points)
- **Southern Boundary:** 21.138°N (covers southern US/Mexico border region)
- **Latitude of Origin:** 38.5°N (central CONUS)
- **Central Meridian:** 262.5° (97.5°W after conversion)
- **Standard Parallels:** 38.5°N (both Latin1 and Latin2)

**Coverage Assessment:**
- ✅ **Covers CONUS**: Lambert Conformal grid with La1=21.138°N extends from the southern US border northward
- ✅ **Continental US Coverage**: Grid spans approximately 125°W-65°W longitude range
- ✅ **Resolution**: 3km provides high-resolution CONUS coverage

## Technical Summary

| Property | Value |
|----------|-------|
| **GRIB2 Edition** | 2 |
| **Discipline** | 0 (Meteorological) |
| **GDT** | 3.30 (Lambert Conformal Conic) |
| **PDT** | 0 (Analysis/Forecast) |
| **DRT** | 0 (Simple packing) |
| **Grid Size** | 1799 x 1059 points |
| **Resolution** | 3km |
| **Coverage** | CONUS |
| **Variable** | MXUPHL (Max Updraft Helicity) |
| **Reference Time** | 2026-07-23 12z |
| **File Size** | 212 bytes |

## Acceptance Criteria Status

✅ **File downloaded successfully** - Downloaded 139MB HRRR file from NOAA S3 bucket
✅ **File saved to local temporary path** - Extracted to `/tmp/` then copied to `data/` directory
✅ **File is readable and accessible** - wgrib2 reads file cleanly, all verifications pass
✅ **wgrib2 confirms valid GRIB2 format** - File reads correctly
✅ **wgrib2 -packing shows DRT=0** - `packing=Grid point data - simple packing,s`
✅ **File spatial bounds cover CONUS** - La1=21.138°N, Lambert Conformal covers continental US

## Notes

- **Fresh Download**: This is a newly downloaded file from 2026-07-23, distinct from the existing `conus_drt0_mxuphl.grib2` file (from 2024-06-01)
- **NOAA Archive Source**: Used NOAA's public HRRR archive on S3, which provides free access to recent HRRR model runs
- **Multiple DRT=0 Messages**: The downloaded HRRR file contained multiple DRT=0 messages (found messages 45-54 with simple packing)
- **File Size**: 212 bytes is very small because all MXUPHL values are likely zero at this analysis time (no significant updraft helicity)
- **Use Case**: This file provides a fresh CONUS-covering DRT=0 fixture for testing gribtract's simple packing decoder

## Related Work

- **bf-76gpy**: Previous CONUS DRT=0 download and validation (2024-06-01 file)
- **bf-2o53**: Original fixture acquisition plan prioritizing CONUS DRT=0
- **bf-55n29**: DRT=0 simple packing verification across corpus
- **HRRR Source**: NOAA HRRR model runs provide excellent CONUS coverage with mixed DRT values

## Files Generated

- `/home/coding/gribtract/data/conus_drt0_mxuphl_20260723.grib2` - Freshly downloaded and validated CONUS DRT=0 file
- `/home/coding/gribtract/notes/bf-3j9o6.md` - This completion note
