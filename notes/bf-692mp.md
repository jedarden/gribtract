# PDT 4.1/4.8 Verification Report

## File Information

**File:** `/tmp/gribtest/gefs_perturbation_member_pdt41.grib2`
**Size:** 3.6 MB
**Messages:** 69 GRIB2 messages
**Download date:** 2026-07-23

## Verification Methodology

Used `grib_get` (from ECMWF GRIB API) and `wgrib2` to inspect Product Definition Template information:

```bash
# Get PDT numbers for all messages
grib_get -p productDefinitionTemplateNumber /tmp/gribtest/gefs_perturbation_member_pdt41.grib2

# Get ensemble configuration details
grib_get -p "productDefinitionTemplateNumber,typeOfProcessedData,perturbationNumber,numberOfForecastsInEnsemble" /tmp/gribtest/gefs_perturbation_member_pdt41.grib2
```

## Verification Results

### Product Definition Template Analysis

✅ **PDT 4.1 CONFIRMED** - All 69 messages use Product Definition Template 4.1

- **Product Definition Template Number:** 1 (corresponds to PDT 4.1 in GRIB2)
- **Data Type:** `pf` (perturbed forecast - ensemble member data)
- **Ensemble Member:** +1 (first perturbation member)
- **Ensemble Size:** 20 members total
- **Processing Type:** Individual ensemble member forecast

### GRIB2 Metadata

- **GRIB Edition:** 2
- **Centre:** kwbc (NCEP/NOAA)
- **Grid Type:** regular_ll (regular latitude-longitude grid)
- **Grid Resolution:** 360 x 181 points (1° x 1°)
- **Packing:** grid_complex_spatial_differencing

### Message Content Distribution

**Variables (15 parameters):**
- CAPE (Convective Available Potential Energy)
- CIN (Convective Inhibition)
- HGT (Geopotential Height)
- PRES (Pressure)
- PRMSL (Pressure reduced to MSL)
- PWAT (Precipitable Water)
- RH (Relative Humidity)
- SNOD (Snow Depth)
- SOILW (Soil Moisture)
- TMP (Temperature)
- TSOIL (Soil Temperature)
- UGRD (U-component of wind)
- VGRD (V-component of wind)
- VVEL (Vertical velocity)
- WEASD (Water equivalent of accumulated snow depth)

**Vertical Levels:**
- Pressure levels: 1000, 925, 850, 700, 500, 400, 300, 250, 200, 100, 50, 10 mb
- Surface levels: 0-0.1m below ground, 2m above ground, 10m above ground
- Atmospheric layers: 180-0mb above ground, entire atmosphere (column)
- Special: mean sea level

**Time:** Analysis (t=0) at 2017-01-01 00:00 UTC

## Conclusion

✅ **PDT 4.1 PRESENT** - This is a verified GEFS ensemble perturbation member file
✅ **All 69 messages consistently use PDT 4.1** for individual ensemble member forecasts
✅ **Ensemble configuration confirmed:** Member +1 from a 20-member ensemble
✅ **File is suitable for ensemble/statistical product testing**

## Product Definition Template 4.1 Definition

**PDT 4.1:** Individual ensemble member forecast (used for perturbation member data)
- Template number 4.1 in GRIB2 Table 4
- Used for individual ensemble member forecasts
- Contains ensemble member number and total ensemble size information

## Product Definition Template 4.8 Status

❌ **PDT 4.8 NOT FOUND** - This file contains no PDT 4.8 (statistical processing) messages
- PDT 4.8 would be used for ensemble mean/standard deviation products
- For PDT 4.8 testing, download ensemble mean or spread files from GEFS archive

## Recommendations

1. ✅ **Use this file for PDT 4.1 testing** - Perfectly suited for ensemble member parsing
2. 🔄 **For PDT 4.8 testing** - Download GEFS ensemble mean files (e.g., `geavg.t00z.pgrb2a.0p50.f000`)
3. 📋 **Track ensemble configuration** - Verify perturbationNumber and numberOfForecastsInEnsemble fields
