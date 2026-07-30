# PDT 4.1/4.8 Analysis for GRIB2 Data

**Task:** Verify downloaded GRIB2 files contain PDT 4.1 or 4.8 messages  
**Date:** 2025-07-23  
**Tools:** wgrib2 v2.0.8+

## Files Analyzed

### 1. HRRR Data
- **File:** `data/hrrr.t12z.wrfsfcf00.grib2` (142 MB)
- **Date:** 2024-06-01 12:00 UTC
- **Total Messages:** 170
- **PDT Distribution:**
  - PDT 4.8 (pdt=8): **27 messages**
  - PDT 0.0 (pdt=0): 143 messages

### 2. NAM Data
- **File:** `data/nam.t00z.awip1200.tm00.grib2` (26 MB)  
- **Date:** 2025-01-15 00:00 UTC
- **Total Messages:** 196
- **PDT Distribution:**
  - PDT 4.8 (pdt=8): **9 messages**
  - PDT 0.0 (pdt=0): 187 messages

## Key Findings

✅ **CONFIRMED:** Both files contain PDT 4.8 messages  
✅ **Total PDT 4.8 messages found:** 36 (27 from HRRR + 9 from NAM)  
✅ **wgrib2 successfully reads both files** without errors

## PDT 4.8 Usage

PDT 4.8 (Product Definition Template 4.8) is typically used for:
- **Ensemble forecast data** (when used with PDT 4.1)
- **Probability forecasts**
- **Forecast error/skill metrics**
- **Model output statistics**

## Message Types with PDT 4.8

### HRRR File (27 messages)
Common PDT 4.8 message types include:
- Max/min temperature forecasts
- Average precipitation accumulations  
- Maximum vertical velocity
- Radar reflectivity maxima
- Various temporal aggregations (max/min/ave over forecast periods)

### NAM File (9 messages)
PDT 4.8 messages found for:
- Specified height level data (11520m, 13056m)
- Multiple forecast time periods

## Inventory Files

Full wgrib2 inventories saved:
- `notes/bf-8ghcj/hrrr_pdt_inventory.txt` - Complete HRRR message list with PDT values
- `notes/bf-8ghcj/nam_pdt_inventory.txt` - Complete NAM message list with PDT values

## Verification Commands Used

```bash
# Check PDT numbers
wgrib2 <file.grib2> -pdt

# Count PDT 4.8 messages
wgrib2 <file.grib2> -pdt | grep -c "pdt=8"

# Full inventory
wgrib2 <file.grib2>
```

## Notes

- PDT values in wgrib2 output: `pdt=8` corresponds to GRIB2 PDT 4.8
- PDT 0.0 (pdt=0) is the standard analysis/forecast template
- Both files successfully processed by wgrib2 without GRIB decoding errors
