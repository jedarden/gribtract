# Bead bf-59506: wgrib2 Inventory of GEFS Ensemble Mean File

## Task Completed Successfully

Ran wgrib2 inventory on the downloaded GEFS ensemble mean file from bead bf-54e7p.

## File Information

- **Source**: `/tmp/geavg_20260723_t00z_f000.grib2`
- **Size**: 13.4 MB (13,991,214 bytes)
- **Type**: GEFS Ensemble Mean (PDT 4.8 - statistical processing)
- **Date**: 2026-07-23 00Z (analysis time)
- **Messages**: 71 GRIB2 messages

## Inventory Results

### Message Count
- **71 messages** successfully inventoried
- **0 errors** encountered during inventory
- wgrib2 successfully read all messages from the file

### Parameters Included

**Atmospheric variables:**
- `HGT` - Geopotential height (multiple pressure levels)
- `TMP` - Temperature (multiple pressure levels and 2m above ground)
- `RH` - Relative humidity (multiple pressure levels and 2m above ground)
- `UGRD` - U-component of wind (multiple pressure levels and 10m above ground)
- `VGRD` - V-component of wind (multiple pressure levels and 10m above ground)
- `VVEL` - Vertical velocity (850 mb only)
- `PRES` - Pressure (surface)
- `PRMSL` - Pressure reduced to mean sea level
- `PWAT` - Precipitable water (entire atmosphere)
- `CAPE` - Convective available potential energy
- `CIN` - Convective inhibition

**Surface/soil variables:**
- `TSOIL` - Soil temperature (0-0.1m below ground)
- `SOILW` - Soil water content (0-0.1m below ground)
- `WEASD` - Water equivalent of accumulated snow depth
- `SNOD` - Snow depth
- `ICETK` - Ice thickness

### Vertical Levels

Pressure levels: 10, 50, 100, 200, 250, 300, 400, 500, 700, 850, 925, 1000 mb
Other levels: surface, 2m above ground, 10m above ground, 0-0.1m below ground

### Processing Type

All messages are marked as **"ens mean"** (ensemble mean), confirming this is a statistically processed ensemble product rather than individual member data.

## Inventory Output Location

Full inventory saved to: `.beads/traces/bf-59506/inventory.txt`

## Verification

✅ wgrib2 successfully reads the file
✅ All 71 messages inventoried without errors
✅ Inventory output saved to file
✅ Message count (71) recorded
✅ File contains expected ensemble mean statistical processing (PDT 4.8)
