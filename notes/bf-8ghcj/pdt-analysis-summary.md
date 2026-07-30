# PDT 4.1 and 4.8 Message Analysis

## Task Summary
Verified GRIB2 files contain Product Definition Template (PDT) 4.1 and 4.8 messages using wgrib2 and grib_ls tools.

## Tools Used
- **wgrib2**: `/home/coding/.local/bin/wgrib2`
- **grib_ls**: `/home/coding/.nix-profile/bin/grib_ls`

## PDT 4.1 Messages Found (productDefinitionTemplateNumber = 1)

### 1. GEFS Perturbation Member Test File
**File**: `tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2`
- **Size**: 3.6M
- **Message count**: 69 messages
- **All messages**: PDT 4.1 (ensemble perturbation member)
- **Date**: 2017-01-01 00:00
- **Ensemble member**: ENS=+1 (perturbation member)

**Message types identified**:
- HGT (Geopotential Height) - multiple levels (10, 50, 100, 200, 250, 500, 700, 850, 925, 1000 mb, surface)
- TMP (Temperature) - multiple levels
- RH (Relative Humidity) - multiple levels  
- UGRD (U-component of wind) - multiple levels
- VGRD (V-component of wind) - multiple levels
- PRES (Pressure) - surface
- TSOIL (Soil temperature) - 0-0.1m below ground
- SOILW (Soil moisture/water content) - 0-0.1m below ground
- WEASD (Water equivalent of accumulated snow depth) - surface
- SNOD (Snow depth) - surface
- PWAT (Precipitable water) - entire atmosphere
- CAPE (Convective available potential energy) - 180-0 mb above ground
- CIN (Convective inhibition) - 180-0 mb above ground
- PRMSL (Pressure reduced to MSL) - mean sea level
- VVEL (Vertical velocity) - 850 mb

### 2. GEFS Ensemble CAPE File
**File**: `tests/corpus/large/gefs_ensemble_p01_cape.grib2`
- **Size**: 35M
- **Message count**: 80 messages
- **All messages**: PDT 4.1 (ensemble perturbation member)
- **Parameter**: CAPE (Convective available potential energy)

### 3. ECMWF Ensemble ENSO File
**File**: `tests/corpus/large/ecmwf_ensemble_enso_0h.grib2`
- **Size**: 1.5G
- **Message count**: 2614 messages total
  - 2563 messages: PDT 4.1
  - 51 messages: PDT 11 (Wave products)
  - 1 error: unreadable message
- **Parameters**: 10u (10m u-wind), 10v (10m v-wind), 2t (2m temperature), tp (total precipitation)

### 4. Small Ensemble Test File
**File**: `tests/corpus/small/pdt1_ensemble_3x2.grib2`
- **Size**: 188 bytes
- **Message count**: 1 message
- **PDT**: 4.1

## PDT 4.8 Messages Found (productDefinitionTemplateNumber = 8)

### 1. Accumulation Test File
**File**: `tests/corpus/small/pdt8_accum_3x2.grib2`
- **Size**: 205 bytes
- **Message count**: 1 message
- **PDT**: 4.8 (ensemble forecast with time dimension)
- **Parameter**: tp (total precipitation)
- **Note**: File has some ECCODES errors but PDT 8 is successfully identified

## Summary Statistics

### PDT 4.1 (Ensemble Perturbation Members)
- **Total messages**: 2,713+ messages
- **Files**: 4 files containing PDT 4.1
- **Characteristics**: 
  - ENS=+1 (perturbation member +1)
  - Multiple pressure levels from 10mb to surface
  - Standard meteorological parameters (HGT, TMP, RH, UGRD, VGRD, etc.)
  - Forecast and analysis times

### PDT 4.8 (Ensemble with Time Dimension)
- **Total messages**: 1 message
- **Files**: 1 file containing PDT 4.8  
- **Characteristics**:
  - Time dimension included in product definition
  - Accumulation parameter (tp - total precipitation)

## Key Findings

1. **PDT 4.1 is well-represented**: Multiple large files with thousands of PDT 4.1 messages available for testing
2. **PDT 4.8 is available**: At least one PDT 4.8 message found (though file has some corruption)
3. **Tool compatibility**: Both wgrib2 and grib_ls successfully read PDT 4.1/4.8 messages
4. **Message variety**: Good coverage of different meteorological parameters and pressure levels

## Files Created
- `gefs_pdt41_inventory.txt` - Detailed inventory of PDT 4.1 test file
- `notes/bf-8ghcj/pdt-analysis-summary.md` - This summary document
