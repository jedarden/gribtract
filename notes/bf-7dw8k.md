# Bead bf-7dw8k: NOAA Ensemble/Statistical GRIB2 File Download

## Task Completed
Successfully downloaded and verified a real NOAA statistical GRIB2 file containing PDT 4.8 messages from public archives.

## File Downloaded

### NAM (North American Mesoscale) AWIPS Grid File

**Source URL:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20260723/nam.t00z.awip1200.tm00.grib2
```

**File Information:**
- **Product:** NAM (North American Mesoscale Model)
- **Grid:** AWIPS12 (CONUS 12km grid)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** TM00 (analysis time)
- **File Size:** 29 MB ✅ Within <50MB target
- **PDT Content:** PDT 4.8 (statistical/accumulation products)
- **Local Path:** `/tmp/grib2-ensemble-pdt41/nam_awip1200_20260723_t00z_tm00.grib2`

## PDT 4.8 Verification

### File Structure
- **Total Messages:** 196
- **PDT 0:** 187 messages (standard meteorological fields)
- **PDT 8 (4.8):** 9 messages (statistical/accumulation products)

### PDT 8 (PDT 4.8) Messages Found
The 9 PDT 8 messages represent accumulation products:

1. **APCP** - Accumulated Total Precipitation
2. **ACPCP** - Accumulated Convective Precipitation  
3. **WEASD** - Water Equivalent of Accumulated Snow Depth

Example inventory entries:
```
79:12148211:d=2026072300:APCP:surface:0-0 day acc fcst:
80:12148454:d=2026072300:ACPCP:surface:0-0 day acc fcst:
81:12148697:d=2026072300:WEASD:surface:0-0 day acc fcst:
```

## File Validation

### GRIB2 Format ✅
- **Magic Bytes:** `GRIB` (47 52 49 42) - correct
- **Format:** GRIB2
- **Tool:** wgrib2 v2.0.8+

### Decode Test ✅
File decodes successfully with wgrib2:
```bash
wgrib2 nam_awip1200_20260723_t00z_tm00.grib2
1:0:d=2026072300:PRMSL:mean sea level:anl:
2:233335:d=2026072300:PRES:1 hybrid level:anl:
3:473932:d=2026072300:RWMR:1 hybrid level:anl:
```

### PDT Count Verification ✅
```bash
wgrib2 nam_awip1200_20260723_t00z_tm00.grib2 -pdt | cut -d: -f3 | sort | uniq -c
187 pdt=0
  9 pdt=8
```

## Acceptance Criteria Status

✅ **Real NOAA ensemble/statistical GRIB2 file downloaded** - NAM model statistical product  
✅ **File contains PDT 4.1 or 4.8 messages** - Contains 9 PDT 4.8 messages  
✅ **File size suitable for test fixture** - 29 MB, well under 50MB target  
✅ **Source URL and date documented** - URL, cycle time, and file metadata recorded  
✅ **File decodes correctly** - wgrib2 successfully reads and inventories the file  

## About PDT 4.8 (Product Definition Template 4.8)

PDT 4.8 is used for **statistical and/or processing products**, including:
- Accumulation fields (precipitation, snow, etc.)
- Time-averaged products
- Ensemble statistical products (mean, spread, probabilities)

The NAM model uses PDT 4.8 specifically for:
- **APCP**: Total accumulated precipitation
- **ACPCP**: Convective accumulated precipitation  
- **WEASD**: Snow depth accumulation (water equivalent)

## Download Metadata

- **Download Date:** 2026-07-23
- **Model Cycle:** 00 UTC, 2026-07-23
- **Archive Source:** NOAA NOMADS (nomads.ncep.noaa.gov)
- **Archive Type:** Public operational model archive
- **Access Method:** HTTPS wget
- **Transfer Speed:** ~10-12 MB/s
- **Download Time:** ~2.5 seconds

## Notes

This NAM AWIPS12 file is ideal for test fixture development because:
1. It's a real operational NOAA product with realistic data
2. Contains verified PDT 4.8 accumulation messages
3. File size is reasonable (29 MB) for repeated testing
4. Mixes PDT 0 and PDT 4.8 messages, representing real-world GRIB2 files
5. Publicly accessible from NOAA NOMADS archive
6. Uses standard AWIPS grid projection common in NWS operations

## References

- NOAA NAM Model Documentation: https://www.nco.ncep.noaa.gov/pmb/products/nam/
- NOAA NOMADS Archive: https://nomads.ncep.noaa.gov/
- WMO GRIB2 Table 4.8: Statistical/processing product definition template

## Related Work

- bf-19o3n.md: PDT 4.1 and 4.8 verification summary
- bf-1ypv3.md: NOAA ensemble GRIB2 file downloads (GEFS)
- bf-42cga.md: NOAA ensemble GRIB2 archive sources research
