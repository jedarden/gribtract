# GRIB2 File Validation - gefs_perturbation_member_pdt41_test.grib2

## Task
Validate that the downloaded GRIB2 file decodes correctly with standard tools.

## Validation Results

### File Information
- **File**: `tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2`
- **Size**: 3.6M
- **Tool**: wgrib2 v3.1.3
- **Status**: ✅ Successfully decodes without errors

### Decode Verification

#### Basic Inventory
```bash
wgrib2 tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2
```

**Result**: 69 records successfully decoded
- Record format: `byte_offset:date=YYYYMMDDHH:parameter:level:forecast_type:ensemble_info`
- Sample output:
  ```
  1:0:d=2017010100:HGT:10 mb:anl:ENS=+1
  2:51175:d=2017010100:TMP:10 mb:anl:ENS=+1
  ...
  69:3645277:d=2017010100:PRMSL:mean sea level:anl:ENS=+1
  ```

#### Product Definition Template (PDT) Verification

```bash
wgrib2 -Sec4 tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2
```

**Result**: All 69 records use **PDT 4.1** (Product Definition Template 4.1)
- Template number: 4.1
- Template size: 37 octets
- Expected size: 37 octets
- Vertical coordinates: 0
- Status: ✅ All PDT 4.1 messages are properly structured and accessible

Sample output:
```
1:0:Sec4 len=37 #vert coordinate=0 Product Defn Template=4.1 size=37 expected size=37
2:51175:Sec4 len=37 #vert coordinate=0 Product Defn Template=4.1 size=37 expected size=37
...
```

#### Ensemble Information

```bash
wgrib2 -ens tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2
```

**Result**: All records contain ensemble information
- Ensemble member: ENS=+1 (perturbation member +1)
- Consistent across all 69 records

#### Diagnostic Validation

```bash
wgrib2 -V tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2
```

**Result**: No errors or corruption detected
- Data values are valid (no undefined values: `undef=0`)
- Statistical information accessible (mean, min, max)
- Grid structure valid: 360x181 lat-lon grid (65,160 points)
- Sample diagnostic output:
  ```
  1:0:vt=2017010100:10 mb:anl:HGT Geopotential Height [gpm]:ENS=+1
      ndata=65160:undef=0:mean=30686.1:min=27944.2:max=31977.3
      grid_template=0:winds(N/S):
      lat-lon grid:(360 x 181) units 1e-06 input WE:NS output WE:SN res 48
      lat 90.000000 to -90.000000 by 1.000000
      lon 0.000000 to 359.000000 by 1.000000 #points=65160
  ```

## Content Summary

### Parameters Included
- **HGT** - Geopotential Height
- **TMP** - Temperature  
- **RH** - Relative Humidity
- **UGRD** - U-component of wind
- **VGRD** - V-component of wind
- **VVEL** - Vertical velocity
- **PRES** - Pressure
- **TSOIL** - Soil temperature
- **SOILW** - Soil water content
- **WEASD** - Water equivalent of accumulated snow depth
- **SNOD** - Snow depth
- **PWAT** - Precipitable water
- **CAPE** - Convective available potential energy
- **CIN** - Convective inhibition
- **PRMSL** - Pressure reduced to mean sea level

### Levels
- Pressure levels: 10, 50, 100, 200, 250, 300, 400, 500, 700, 850, 925, 1000 mb
- Surface levels
- 2m above ground (temperature, humidity)
- 10m above ground (wind)
- 0-0.1m below ground (soil)
- 180-0 mb above ground (CAPE/CIN)

### Temporal Information
- Reference date: 2017-01-01 00:00 UTC
- Forecast type: Analysis (anl)
- Ensemble: GEFS perturbation member +1

## Acceptance Criteria Status

✅ **File decodes successfully with wgrib2** - All 69 records decoded without errors
✅ **No decode errors or corruption detected** - wgrib2 diagnostic shows clean data (undef=0)
✅ **PDT 4.1 messages are confirmed accessible** - All records use PDT 4.1, properly structured
✅ **Decode command and sample output documented** - Commands and results documented above
✅ **File is ready for use as test fixture** - File validated and structurally sound

## Conclusion

The `gefs_perturbation_member_pdt41_test.grib2` file successfully decodes with wgrib2 and contains valid GEFS ensemble perturbation data with PDT 4.1. The file structure is correct, data is accessible, and no corruption or errors were detected. The file is ready for use as a test fixture for PDT 4.1 validation and testing.

## Decode Commands Reference

```bash
# Basic inventory
wgrib2 tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2

# Product Definition Template (PDT) information
wgrib2 -Sec4 tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2

# Ensemble information
wgrib2 -ens tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2

# Diagnostic output with data statistics
wgrib2 -V tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2

# Count records
wgrib2 tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2 | wc -l
```

## Date: 2026-07-23
