# Bead bf-5gsm: NOAA URL Patterns Documentation

## Summary

Documented URL patterns for dates, cycles, and forecast hours across 8 NOAA models to provide a comprehensive reference for constructing archive URLs.

## Work Completed

### 1. Analyzed URL Structure Components

**Date Encoding (YYYYMMDD):**
- All NOAA models use consistent 8-digit format
- YYYY = 4-digit year, MM = 2-digit month, DD = 2-digit day
- Example: `20260703` = July 3, 2026

**Cycle Hour Encoding:**
- Three patterns identified:
  - `HH` (two-digit hour): NAM directory naming
  - `CC` (two-digit hour): RAP, RRFS file naming
  - `tCCz` ("t" + hour + "z"): HRRR, NAM, RAP file naming

**Forecast Hour Encoding:**
- Three patterns identified:
  - `fFF` ("f" + two-digit): HRRR, RAP (f00-f48)
  - `FF` (two-digit, no prefix): NAM (00-60)
  - `FFF` (three-digit): RRFS, NBM (000-084+)

### 2. Documented Model-Specific Patterns

Documented 8 NOAA models with URL patterns and examples:

| Model | Pattern | Cycle Times | Forecast Range |
|-------|---------|-------------|----------------|
| HRRR | `hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2` | Hourly (00z-23z) | f00-f48 |
| NAM | `nam.YYYYMMDD/nam.tCCz.[product]FF.tm00.grib2` | 6-hourly (00,06,12,18z) | 00-60 |
| RAP | `YYYYMMDD/HH/rap.tCCz.awp130pgrbfFF.grib2` | Hourly (00z-23z) | f00-f51 |
| RRFS | `YYYYMMDD/HH/rrfs.tCCz.[product].fFF.grib2` | Hourly (00z-23z) | f000-f084 |
| NBM | `blend.YYYYMMDD/HH/core/blend.tCCz.core.fFFF.co.grib2` | 6-hourly (00,06,12,18z) | f001-f084 |
| HREF | `href.YYYYMMDD/HH/` (NOMADS) | ~4x daily | Varies by member |
| SREF | `sref.YYYYMMDD/HH/` (NOMADS) | 4x daily (03,09,15,21z) | Varies by member |
| RTMA | `YYYYMMDD/HH/rtma.tCCz.2dvaranl.[domain].grib2` | Hourly (00z-23z) | Analysis only |

### 3. Provided Step-by-Step Construction Examples

Created 3 detailed examples showing how to construct URLs from scratch:
- HRRR: December 25, 2025, 18z cycle, 6-hour forecast
- NAM: March 15, 2026, 00z cycle, analysis
- NBM: September 10, 2026, 12z cycle, 24-hour forecast

### 4. Documented Common Errors

Identified 4 common URL construction errors:
1. Missing leading zeros (dates/hours)
2. Incorrect forecast hour format (2-digit vs 3-digit)
3. Wrong product suffix
4. Non-existent cycle times

### 5. Platform Comparison

Documented three archive platforms:
- AWS S3: 6 models (HRRR, NAM, RAP, RRFS, RTMA, NBM)
- NOMADS: 4 models (HREF, SREF, NAM, RAP, HRRR alternative)
- FTP: 2 models (NDFD, WAVEWATCH3)

## Acceptance Criteria Met

✅ **For each model, documented the URL pattern with placeholders**
- All 8 models documented with clear pattern definitions
- Placeholders (YYYYMMDD, CC, FF, FFF, etc.) clearly explained

✅ **Provided 2-3 example URLs per model**
- HRRR: 4 examples
- NAM: 3 examples
- RAP: 3 examples
- RRFS: 3 examples
- NBM: 3 examples
- HREF: 2 examples
- SREF: 2 examples
- RTMA: 3 examples

✅ **Noted variations in URL structure**
- Forecast hour encoding variations (f00 vs 00 vs f000 vs f001)
- Cycle frequency variations (hourly vs 6-hourly vs 4x daily)
- Platform variations (AWS vs NOMADS vs FTP)
- Product type variations documented

## Files Created

1. `/home/coding/gribtract/docs/research/bf-5gsm-noaa-url-patterns.md` - Comprehensive URL pattern documentation (17,256 bytes)
2. `/home/coding/gribtract/notes/bf-5gsm.md` - This summary

## Related Documentation

- `docs/research/bf-1ot3-gdt330-candidate-archive-urls.md` - Archive URLs for GDT 3.30 candidates
- `docs/research/noaa-archive-urls.md` - Primary model archive URLs

## Date Completed

2026-07-03
