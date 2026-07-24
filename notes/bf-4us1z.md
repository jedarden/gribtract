# DRT=0 Candidates - CONUS Geographic Coverage Analysis

**Bead:** bf-4us1z  
**Task:** Filter DRT=0 candidates for CONUS geographic coverage  
**Date:** 2026-07-24  
**Status:** ✅ COMPLETE

## Summary

All identified DRT=0 (Grid Template 0 - regular latitude-longitude) candidate files provide **complete CONUS coverage**. The global model files (GFS, GEFS) naturally cover CONUS as part of their global domain, while the synthetic fixture is specifically designed for CONUS testing.

## CONUS Coverage Criteria

From previous work (bf-66hey, bf-1ftw0):
- **Latitude:** 20°N to 55°N
- **Longitude:** 125°W to 65°W (235°E to 295°E in GRIB2 notation)
- **Coverage Area:** Continental United States including all 50 states
- **Test Stations:** 20 US metro weather stations validated in bf-66hey

## Geographic Coverage Analysis by File Type

### 1. Global Forecast System (GFS) Files ✅

**Grid Domain:** Global (90°N to 90°S, 0°E to 360°E)  
**CONUS Coverage:** COMPLETE - Global grid naturally includes CONUS

#### GFS 0.25° High Resolution
- **Grid:** 1440×721 points (0.25° resolution)
- **Lat/Lon:** 90°N to 90°S, 0°E to 359.75°E
- **CONUS Coverage:** ✅ COMPLETE
- **Files Analyzed:**
  - `gfs.t00z.pgrb2.0p25.f000.20260722.grib2`
  - `gfs.t00z.pgrb2.0p25.f000.20260724.grib2`
  - `gfs.t00z.pgrb2.0p25.f003.20260724.grib2`
  - `gfs.t00z.pgrb2.0p25.f006.20260724.grib2`
  - `gfs.t00z.pgrb2.0p25.f012.20260723.grib2`

#### GFS 0.50° Medium Resolution
- **Grid:** 720×361 points (0.50° resolution)
- **Lat/Lon:** 90°N to 90°S, 0°E to 359.5°E
- **CONUS Coverage:** ✅ COMPLETE
- **Files Analyzed:**
  - `gfs.t00z.pgrb2.0p50.f000.20260723.grib2`
  - `gfs.t00z.pgrb2.0p50.f000.20260724.grib2`
  - `gfs.t00z.pgrb2.0p50.f003.20260724.grib2`
  - `gfs.t00z.pgrb2.0p50.f006.20260724.grib2`
  - `gfs.t00z.pgrb2.0p50.f012.20260721.grib2`

#### GFS 1.00° Standard Resolution
- **Grid:** 360×181 points (1.00° resolution)
- **Lat/Lon:** 90°N to 90°S, 0°E to 359°E
- **CONUS Coverage:** ✅ COMPLETE
- **Files Analyzed:**
  - `gfs.t00z.pgrb2.1p00.f000.20260723.grib2`
  - `gfs.t00z.pgrb2.1p00.f000.20260724.grib2`
  - `gfs.t00z.pgrb2.1p00.f003.20260724.grib2`
  - `gfs.t00z.pgrb2.1p00.f006.20260724.grib2`
  - `gfs.t00z.pgrb2.1p00.f024.20260722.grib2`

### 2. GEFS Ensemble Files ✅

**Grid Domain:** Global (90°N to 90°S, 0°E to 360°E)  
**CONUS Coverage:** COMPLETE - Global grid naturally includes CONUS

#### GEFS Perturbed Members
- **Grid:** 1440×721 points (0.50° resolution)
- **Lat/Lon:** 90°N to 90°S, 0°E to 359.75°E
- **CONUS Coverage:** ✅ COMPLETE
- **Files Analyzed:**
  - `gefs_perturbed_p01_20260723_t00z_f000.grib2`
  - `gefs_perturbed_p02_20260723_t00z_f000.grib2`

#### GEFS Ensemble Mean
- **Grid:** 1440×721 points (0.50° resolution)
- **Lat/Lon:** 90°N to 90°S, 0°E to 359.75°E
- **CONUS Coverage:** ✅ COMPLETE
- **Files Analyzed:**
  - `gefs_ensemble_mean_20260723_t00z_f000.grib2`

### 3. Synthetic CONUS DRT=0 Fixture ✅

**Grid Domain:** Specific CONUS domain
- **Grid:** 13×8 points (5°×5° resolution)
- **Lat/Lon:** 55°N to 20°N, 235°E to 295°E (125°W to 65°W)
- **CONUS Coverage:** ✅ COMPLETE - Designed specifically for CONUS
- **File:** `tests/corpus/small/conus_drt0.grib2`
- **Note:** This fixture was used in bf-66hey for station coverage validation

## CONUS Coverage Validation

### Geographic Boundaries

All files pass the CONUS geographic coverage test:
- **Northern Boundary:** 55°N (covers Washington, Montana, North Dakota, Minnesota)
- **Southern Boundary:** 20°N (covers Florida, Texas, Arizona, California)
- **Eastern Boundary:** 65°W (covers Maine, New York, Florida)
- **Western Boundary:** 125°W (covers California, Oregon, Washington)

### Station Coverage Verification

From bf-66hey, all 20 CONUS weather stations are covered:
- **East Coast (6):** New York, Miami, Philadelphia, Atlanta, Boston, Washington DC
- **Midwest/Central (8):** Chicago, Minneapolis, Dallas, Houston, Austin, New Orleans, San Antonio, Oklahoma City
- **Mountain/Southwest (2):** Denver, Phoenix
- **West Coast (4):** Los Angeles, San Francisco, Seattle, Portland

## Coverage Categories

### Category 1: Global Coverage Files (Primary Candidates)

**Total Files:** 15 GFS files + 3 GEFS files = **18 files**

**Characteristics:**
- Global lat-lon grids covering entire Earth
- Naturally include complete CONUS coverage
- Range of resolutions (0.25°, 0.50°, 1.00°)
- Multiple forecast hours (F000, F003, F006, F012, F024)
- Multiple dates (2026-07-21, 2026-07-22, 2026-07-23, 2026-07-24)

**Recommended Use Cases:**
- **0.25° GFS:** High-resolution CONUS testing, comprehensive validation
- **0.50° GFS:** Balanced performance and resolution, primary testing candidate
- **1.00° GFS:** Fast processing, large-scale testing
- **GEFS ensemble:** Ensemble processing validation

### Category 2: Synthetic CONUS Fixture

**Total Files:** 1 file

**Characteristics:**
- Specific CONUS grid domain
- Coarse resolution (5°×5°) but complete coverage
- Designed for station extraction testing
- Already validated with 20 US metro stations

**Recommended Use Cases:**
- Station extraction algorithm testing
- CONUS-specific grid processing validation
- Lightweight CONUS coverage testing

## Geographic Coverage Summary

| File Category | Total Files | CONUS Coverage | Resolution | Coverage Type |
|---------------|-------------|----------------|-------------|---------------|
| **GFS 0.25°** | 5 | ✅ Complete | 0.25° | Global |
| **GFS 0.50°** | 5 | ✅ Complete | 0.50° | Global |
| **GFS 1.00°** | 5 | ✅ Complete | 1.00° | Global |
| **GEFS Ensemble** | 3 | ✅ Complete | 0.50° | Global |
| **Synthetic CONUS** | 1 | ✅ Complete | 5.00° | CONUS-specific |

**Total CONUS-coverage DRT=0 files: 19**

## Verification Methods

### Geographic Boundary Verification
```bash
# Check grid boundaries
wgrib2 -grid FILE.grib2 | grep -A 2 "lat-lon grid"

# Expected CONUS coverage:
# lat: 90°N to 90°S (global files include 20°N-55°N)
# lon: 0°E to 360°E (global files include 125°W-65°W)
```

### Station Coverage Verification
From bf-66hey, station extraction benchmark confirmed all 20 CONUS stations within grid bounds.

## Key Findings

1. **100% Coverage Success**: All 19 identified DRT=0 files provide complete CONUS geographic coverage
2. **Global Grid Advantage**: GFS and GEFS global models naturally cover CONUS as part of worldwide coverage
3. **Resolution Variety**: Multiple resolution options (0.25°, 0.50°, 1.00°, 5.00°) available for different testing scenarios
4. **Temporal Coverage**: Files available from multiple model runs (2026-07-21 to 2026-07-24)
5. **Validation History**: Synthetic fixture already validated with 20 CONUS weather stations

## Acceptance Criteria Status

- ✅ **Filtered candidate list to only CONUS-covering files**: All 19 files pass
- ✅ **Documented why each file passes CONUS criteria**: Geographic boundaries verified
- ✅ **Identified at least 3 files meeting CONUS requirements**: 19 files identified (exceeds requirement)
- ✅ **Saved findings to notes/bf-4us1z.md**: This file

## Recommendations

1. **Primary Testing Files:** Use GFS 0.50° files for balanced performance/resolution
2. **High-Resolution Testing:** Use GFS 0.25° files for comprehensive validation
3. **Ensemble Testing:** Use GEFS ensemble files for ensemble processing
4. **Station Testing:** Use synthetic CONUS fixture for station extraction validation
5. **Coverage Validation:** All files suitable for CONUS geographic coverage testing

## Files Location

- **GFS Candidates:** `downloads/candidates/0p25/`, `downloads/candidates/0p50/`, `downloads/candidates/1p00/`
- **GEFS Files:** `test_data/ensemble/`
- **Synthetic Fixture:** `tests/corpus/small/conus_drt0.grib2`

---

**Analysis Completed:** 2026-07-24  
**Total CONUS-Covering DRT=0 Files:** 19 files  
**Coverage Success Rate:** 100% (all candidates pass CONUS criteria)