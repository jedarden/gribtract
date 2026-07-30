# bf-8ghcj: PDT Verification for GEFS Ensemble Mean File

## Task
Verify the downloaded GEFS ensemble mean file contains PDT 4.1 or 4.8 messages.

## File Analyzed
- **Path**: `/tmp/geavg_20260723_t00z_f000.grib2`
- **Source**: NOAA GEFS AWS S3 bucket
- **Size**: 14M (13.4MB)
- **Download timestamp**: 2026-07-23 15:39

## Tool Verification
✅ **wgrib2 is available** at `/home/coding/.local/bin/wgrib2`
✅ **wgrib2 successfully reads the file** - inventory completed without errors

## Inventory Results

### Total Messages
**71 GRIB2 messages** found in the file

### Message Characteristics
All messages show:
- **Product type**: `ens mean` (ensemble mean)
- **Date/time**: `d=2026072300` (July 23, 2026, 00 UTC)
- **Forecast hour**: `anl` (analysis, f000)

### Sample Messages (first 10)
```
1:0:d=2026072300:HGT:10 mb:anl:ens mean
2:200935:d=2026072300:TMP:10 mb:anl:ens mean
3:335675:d=2026072300:RH:10 mb:anl:ens mean
4:380565:d=2026072300:UGRD:10 mb:anl:ens mean
5:643932:d=2026072300:VGRD:10 mb:anl:ens mean
6:889634:d=2026072300:HGT:50 mb:anl:ens mean
7:1100263:d=2026072300:TMP:50 mb:anl:ens mean
8:1232498:d=2026072300:RH:50 mb:anl:ens mean
9:1337968:d=2026072300:UGRD:50 mb:anl:ens mean
10:1597251:d=2026072300:VGRD:50 mb:anl:ens mean
```

### Full Inventory
See attached inventory output: `notes/bf-8ghcj_inventory.txt`

## PDT (Product Definition Template) Analysis

### Finding
**PDT 4.2** detected in the file (not PDT 4.1 or 4.8 as expected)

### GRIB2 Section 4 Structure Analysis
```
Section 4 length: 36 bytes
Section number: 4 (Product Definition Section)
Coordinate values: 0
Product Definition Template: 2 → PDT 4.2
```

### PDT 4.2 Template Fields (First Message)
- **Parameter category**: 3 (Temperature)
- **Parameter number**: 5 (Geopotential height)
- **Type of generating process**: 4 (Forecast - ensemble)
- **Background process ID**: 0
- **Analysis process ID**: 107 (GEFS)
- **Forecast time**: 0 hours

### Interpretation
- **PDT 4.2**: Individual ensemble forecast at a horizontal level as a time series
- The file contains ensemble mean (`ens mean`) data
- Type of generating process (4) indicates ensemble-derived product
- All 71 messages use the same PDT 4.2 template

## Conclusion
❌ **Does not contain PDT 4.1 or 4.8 messages**

The file contains **PDT 4.2** messages for ensemble mean data. While PDT 4.2 is a valid ensemble template, it differs from the expected PDT 4.1 (individual ensemble member) or PDT 4.8 (statistical processing) templates.

### Possible Explanations
1. **GEFS encoding**: GEFS may use PDT 4.2 for ensemble mean products
2. **NCEP convention**: NOAA/NCEP may have specific conventions for encoding ensemble means
3. **Template interpretation**: PDT 4.2 may be the appropriate template for this type of ensemble data

### Recommendations
1. Verify PDT requirements for GEFS ensemble mean products
2. Check if PDT 4.2 is acceptable for the intended use case
3. Investigate other GEFS products to confirm PDT patterns

## Files Generated
- `notes/bf-8ghcj.md` (this file)
- `notes/bf-8ghcj_inventory.txt` (full wgrib2 inventory)

## Next Steps
- Confirm if PDT 4.2 is acceptable or if PDT 4.1/4.8 files are required
- May need to source different GEFS products if PDT 4.1/4.8 are specifically needed
