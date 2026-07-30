# Ensemble File Verification (bf-3b76o)

## Task
Verify the downloaded GEFS ensemble files decode correctly with grib2 tools.

## Files Verified

### 1. Ensemble Mean File
**File:** `test_data/ensemble/gefs_ensemble_mean_20260723_t00z_f000.grib2`
**Size:** 13,974,676 bytes (~13.3 MB)

**Verification Results:**
- ✅ wgrib2 successfully reads all messages
- ✅ GRIB Edition 2 confirmed
- ✅ PDT 4.2 (Product Definition Template for ensemble-derived products)
- ✅ 26 messages total
- ✅ 30 ensemble members reference
- ✅ Grid: 1440 x 721 lat-lon (0.25° resolution)
- ✅ Data range includes: VIS, GUST, MSLET, PRES, HGT, TMP, DPT, RH, UGRD, VGRD, CAPE, CIN, PWAT, etc.

### 2. Perturbed Member p01 File
**File:** `test_data/ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2`
**Size:** 13,984,963 bytes (~13.3 MB)

**Verification Results:**
- ✅ wgrib2 successfully reads all messages
- ✅ GRIB Edition 2 confirmed
- ✅ PDT 4.1 (Product Definition Template for individual ensemble members)
- ✅ 71 messages total (3D atmospheric fields at multiple pressure levels)
- ✅ 30 ensemble members reference
- ✅ Grid: 720 x 361 lat-lon (0.5° resolution)
- ✅ ENS=+1 indicator for perturbed member +1
- ✅ Variables: HGT, TMP, RH, UGRD, VGRD at pressure levels (10mb through 1000mb)

### 3. Perturbed Member p02 File
**File:** `test_data/ensemble/gefs_perturbed_p02_20260723_t00z_f000.grib2`
**Size:** 13,966,199 bytes (~13.2 MB)

**Verification Results:**
- ✅ wgrib2 successfully reads all messages
- ✅ GRIB Edition 2 confirmed
- ✅ PDT 4.1 (Product Definition Template for individual ensemble members)
- ✅ 71 messages total
- ✅ Same structure as p01 file

## wgrib2 Commands Used

```bash
# Basic inventory
wgrib2 <file>

# Message count
wgrib2 <file> | wc -l

# Product Definition Template (PDT) information
wgrib2 <file> -Sec4

# Grid information
wgrib2 <file> -grid

# Ensemble member count
wgrib2 <file> -N_ens

# GRIB edition
wgrib2 <file> -Sec0
```

## Summary

All three downloaded GEFS ensemble files decode successfully with wgrib2 v3.1.3:

1. **PDT Verification:**
   - Ensemble mean uses PDT 4.2 (correct for derived ensemble products)
   - Perturbed members use PDT 4.1 (correct for individual ensemble members)
   - Both PDT 4.1 and 4.2 are standard WMO code tables for ensemble data

2. **Message Structure:**
   - Ensemble mean: 26 surface/2D variables
   - Perturbed members: 71 messages (3D atmospheric fields at multiple pressure levels)

3. **File Integrity:**
   - All files are valid GRIB2 format
   - No decode errors or corruption
   - Proper grid definitions and metadata

4. **Suitability as Test Fixtures:**
   ✅ Files are confirmed suitable for use as test fixtures
   ✅ Contain both PDT 4.1 and 4.2 examples
   ✅ Include ensemble-specific metadata (ENS=+1, ens mean)
   ✅ Properly formatted and readable by standard grib2 tools

## Date/Time Reference
- Forecast cycle: 2026-07-23 00Z
- Forecast hour: F000 (analysis time)
- Download date: 2026-07-23
