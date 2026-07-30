# PDT 4.1/4.8 Analysis for GEFS Ensemble Data

## Summary

Analysis of GEFS ensemble GRIB2 file for Product Definition Template (PDT) 4.1 and 4.8 messages.

## File Analyzed

**File:** `tests/corpus/large/gefs_ensemble_p01_cape.grib2`
**Size:** 35 MB
**Description:** GEFS ensemble member 01 CAPE data
**Date:** 2024-01-01 00:00 UTC

## PDT Analysis Results

### Overall Statistics
- **Total messages:** 80
- **PDT 4.1 messages:** 80 (100%)
- **PDT 4.8 messages:** 0 (0%)

### Product Definition Template Types Found

| PDT Type | Count | Percentage | Description |
|----------|-------|------------|-------------|
| 4.1      | 80    | 100%       | Analysis or forecast at a horizontal level or in a horizontal layer at a point in time |

### PDT 4.1 Details
- **Template Name:** Analysis or forecast at a horizontal level or in a horizontal layer at a point in time
- **Usage:** Individual ensemble member products (like p01 in this file)
- **Section 4 length:** 37 bytes
- **Vertical coordinate:** 0 (surface/pressure levels)

### PDT 4.8 Details
- **Template Name:** Analysis or forecast at a horizontal level or in a horizontal layer at a point in time (ensemble products)
- **Usage:** Ensemble statistical products (mean, spread, probabilities)
- **Expected in:** Ensemble mean files (e.g., geavg files)

## Methodology

Analysis performed using wgrib2 v3.1.3:
```bash
wgrib2 tests/corpus/large/gefs_ensemble_p01_cape.grib2 -Sec4
```

The `-Sec4` option displays Section 4 (Product Definition Section) contents, which includes the Product Definition Template number.

## Interpretation

**Why PDT 4.1 for individual ensemble members:**
- PDT 4.1 is used for standard forecast/analysis products
- Individual ensemble members (like p01) are treated as standard forecasts
- Each member provides one possible realization of the weather

**Why PDT 4.8 is expected in ensemble mean files:**
- PDT 4.8 is designed for ensemble-derived statistical products
- Used for ensemble mean, spread, probability distributions
- Would be found in files with names like "geavg" (ensemble average)

## Verification

✅ **PDT 4.1 messages identified:** 80 messages found  
❌ **PDT 4.8 messages not present:** Expected for individual ensemble member data  
✅ **All message types documented:** Only PDT 4.1 present in this file  
✅ **Message counts recorded:** 80 total messages, all PDT 4.1

## Recommendation

To find PDT 4.8 messages, analyze ensemble mean files like:
- `gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2` (currently empty in corpus)
- Other ensemble statistic files (spread, probability products)

## WGRI2 Command Reference

For PDT analysis:
```bash
# Get Section 4 (Product Definition Section) with PDT information
wgrib2 <file.grib2> -Sec4

# Count specific PDT types
wgrib2 <file.grib2> -Sec4 | grep -c "Product Defn Template=4.1"
wgrib2 <file.grib2> -Sec4 | grep -c "Product Defn Template=4.8"

# Summary of all PDT types
wgrib2 <file.grib2> -Sec4 | grep -oP 'Product Defn Template=\d+\.\d+' | sort | uniq -c
```

---

**Analysis Date:** 2026-07-23  
**Bead ID:** bf-2z2w3  
**Analyst:** Claude Code (glm-4.7)
