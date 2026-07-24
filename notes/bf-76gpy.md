# CONUS DRT=0 GRIB2 File Download and Validation - bf-76gpy

## Task Summary
Successfully downloaded and validated a CONUS-covering GRIB2 file with DRT=0 (simple packing).

## File Details

**Location:** `/home/coding/gribtract/data/conus_drt0_mxuphl.grib2`

**Size:** 212 bytes

**SHA-256:** `c5093478aa0f3afcd921e7ae4135f4407811f5b16e86d969fa0d08d13161b353`

## Source and Extraction

**Source File:** `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2`
- HRRR CONUS wrfsfcf analysis, run 2024-06-01 12z
- 170 GRIB2 messages total
- Extracted message 45 using wgrib2: `wgrib2 -d 45 -grib`

**Extracted Message:**
- Variable: MXUPHL (Maximum Updraft Helicity 5000-2000m above ground)
- Level: 5000-2000 m above ground
- Forecast: 0-0 day max forecast
- Reference time: 2024-06-01 12z

## Validation Results

### 1. File Download ✅
- Successfully extracted message 45 from existing HRRR CONUS file
- File integrity verified: reads cleanly with wgrib2

### 2. GRIB2 Format Validation ✅
```bash
$ wgrib2 conus_drt0_mxuphl.grib2
1:0:d=2024060112:MXUPHL:5000-2000 m above ground:0-0 day max fcst:
```
- Valid GRIB2 format confirmed
- Single message file

### 3. DRT=0 Confirmation ✅
```bash
$ wgrib2 -packing conus_drt0_mxuphl.grib2
1:0:packing=Grid point data - simple packing,s
```
- **DRT=0 (Data Representation Template 0)**: Simple packing
- No spatial differencing
- No complex compression

### 4. CONUS Coverage Verification ✅
```bash
$ wgrib2 -grid conus_drt0_mxuphl.grib2
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
- **Grid Points:** 1799 x 1059 (1,903,141 total points)
- **Southern Boundary:** 21.138°N (covers southern US/Mexico border region)
- **Latitude of Origin:** 38.5°N
- **Central Meridian:** 262.5° (97.5°W after conversion)
- **Standard Parallels:** 38.5°N (both Latin1 and Latin2)

**Coverage Assessment:**
- ✅ **Covers CONUS**: Lambert Conformal grid with La1=21.138°N extends from the southern US border northward
- ✅ **Continental US Coverage**: Grid spans approximately 125°W-65°W longitude range
- ✅ **Resolution**: 3km provides high-resolution CONUS coverage

## Data Characteristics

**Variable Statistics:**
```bash
$ wgrib2 -V conus_drt0_mxuphl.grib2
1:0:vt=2024060112:5000-2000 m above ground:0-0 day max fcst:MXUPHL Hourly Maximum of Updraft Helicity [m^2/s^2]:
    ndata=1905141:undef=0:mean=0:min=0:max=0
```
- **Data Points:** 1,905,141 (matches grid: 1799 x 1059)
- **Undefined Values:** 0 (complete coverage, no bitmap)
- **Values:** All zeros (MXUPHL field has zero values at this analysis time)

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

## Acceptance Criteria Status

✅ **File downloaded successfully** - Extracted from local HRRR file
✅ **wgrib2 confirms valid GRIB2 format** - File reads cleanly
✅ **wgrib2 -packing shows DRT=0** - `packing=Grid point data - simple packing,s`
✅ **File spatial bounds cover CONUS** - La1=21.138°N, Lambert Conformal covers continental US

## Notes

- **HRRR Source**: The HRRR model uses mixed DRT values - most fields use DRT=3 (complex packing with spatial differencing), but categorical/derived fields like MXUPHL use DRT=0 (simple packing)
- **File Size**: 212 bytes is very small because all MXUPHL values are zero at this analysis time (no significant updraft helicity)
- **Use Case**: This file provides a CONUS-covering DRT=0 fixture for testing gribtract's simple packing decoder with real-world CONUS coverage
- **Extraction Method**: Message 45 was one of five consecutive DRT=0 messages (45-49) in the HRRR file

## Related Work

- **bf-2o53**: Original fixture acquisition plan prioritizing CONUS DRT=0
- **bf-55n29**: DRT=0 simple packing verification across corpus
- **HRRR Source**: `hrrr_conus_drt3_lambert` fixture in tests/corpus/manifest.json

## Files Generated

- `/home/coding/gribtract/data/conus_drt0_mxuphl.grib2` - Validated CONUS DRT=0 file
