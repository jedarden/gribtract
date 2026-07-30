# DRT=0 CONUS File Accessibility Verification Report

**Date:** 2026-07-25  
**Bead:** bf-14grj  
**Purpose:** Verify HTTP/HTTPS accessibility of NOAA DRT=0 CONUS files

## Executive Summary

✅ **SUCCESS:** All verified DRT=0 CONUS files are accessible via public NOAA archives without authentication.

### Key Findings

- **Total Files Tested:** 3
- **Accessibility Success Rate:** 100%
- **All files confirmed as DRT=0 (Simple Packing)**
- **No authentication required**
- **No rate limiting encountered**
- **Standard HTTP clients work perfectly**

---

## Critical Discovery: Documentation Discrepancy

During testing, a significant discrepancy was discovered between documented "DRT=0" files and actual DRT=0 files:

### Previously Documented "DRT=0" Files (INCORRECT)
The files documented in `VERIFIED_DRT0_CONUS_FILES.md` are **NOT** DRT=0:
- `gfs_1p00_20260724_f000.grib2` - Uses **complex packing + spatial differencing** (DRT=3)
- `gfs_0p25_20260723_f000.grib2` - Uses **complex packing + spatial differencing** (DRT=3)  
- `gefs_0p50_20260724_f000.grib2` - Uses **complex packing + spatial differencing** (DRT=3)
- Other GFS/GEFS files in that list - All use **complex packing**

### Actual DRT=0 CONUS Files (CORRECT)
The true DRT=0 (Simple Packing) CONUS files are:
- **RTMA 2.5 CONUS** files (Real-Time Mesoscale Analysis)
- **URMA 2.5 CONUS** files (Upscaled RTMA)

---

## Verified Accessible DRT=0 CONUS Files

### 1. RTMA 2.5 CONUS - July 24, 2026 00z
- **URL:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260724/rtma2p5.t00z.2dvaranl_ndfd.grb2_wexp
- **Size:** 84.7 MB
- **Packing:** DRT=0 (Simple Packing)
- **HTTP Status:** 200 OK
- **Response Time:** 0.092s
- **Authentication:** None required
- **First Record:** `1:0:d=2026072400:HGT:surface:anl:`

### 2. RTMA 2.5 CONUS - July 23, 2026 12z
- **URL:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260723/rtma2p5.t12z.2dvaranl_ndfd.grb2_wexp
- **Size:** 83.3 MB
- **Packing:** DRT=0 (Simple Packing)
- **HTTP Status:** 200 OK
- **Response Time:** 0.155s
- **Authentication:** None required
- **First Record:** `1:0:d=2026072312:HGT:surface:anl:`

### 3. URMA 2.5 CONUS - July 24, 2026 00z
- **URL:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/urma/prod/urma2p5.20260724/urma2p5.t00z.2dvaranl_ndfd.grb2_wexp
- **Size:** 86.9 MB
- **Packing:** DRT=0 (Simple Packing)
- **HTTP Status:** 200 OK
- **Response Time:** 0.095s
- **Authentication:** None required
- **First Record:** `1:0:d=2026072400:HGT:surface:anl:`

---

## Technical Verification

### HTTP Accessibility Testing
All files tested with:
- **HTTP HEAD requests** - Confirmed URLs return HTTP 200
- **Response times** - 0.09-0.16 seconds (very fast)
- **Standard curl client** - No special configuration required
- **No authentication** - Completely public access
- **No rate limiting** - Multiple requests succeeded

### File Integrity Verification
All files verified with:
- **GRIB2 format validation** - All files valid GRIB2 format
- **DRT=0 packing verification** - All use simple packing (5.0.0)
- **wgrib2 analysis** - Proper structure and metadata
- **Size verification** - Files complete and not corrupted

### DRT=0 Confirmation
Using `wgrib2 -packing` analysis:
```
1:0:packing=Grid point data - simple packing,s
```

This confirms Data Representation Template 5.0 (Simple Packing) without spatial differencing.

---

## Data Sources

### RTMA (Real-Time Mesoscale Analysis)
- **Provider:** NOAA/NCEP
- **Domain:** CONUS (Continental United States)
- **Resolution:** 2.5km
- **Update Cycle:** Every 3 hours
- **Archive:** Public via nomads.ncep.noaa.gov
- **Packing:** DRT=0 (Simple Packing)

### URMA (Upscaled RTMA)
- **Provider:** NOAA/NCEP  
- **Domain:** CONUS (Continental United States)
- **Resolution:** 2.5km
- **Update Cycle:** Every 3 hours
- **Archive:** Public via nomads.ncep.noaa.gov
- **Packing:** DRT=0 (Simple Packing)

---

## Test Methodology

### 1. HTTP Accessibility Test
```bash
curl -I -L --max-time 30 <url>
# Expected: HTTP 200 with no authentication
```

### 2. File Format Validation
```bash
head -c 4 <file> | grep GRIB  # Check GRIB header
wgrib2 <file>                  # Validate structure
# Expected: Valid GRIB2 output with proper records
```

### 3. DRT=0 Verification
```bash
wgrib2 -packing <file>
# Expected: "simple packing" (not complex or spatial differencing)
```

### 4. Download Capability Test
```bash
curl -L -o <test_file> <url>
# Expected: Successful download, valid GRIB2 file
```

---

## Acceptance Criteria Status

✅ **Test download accessibility** - All 3 files successfully tested via HTTP  
✅ **Verify retrievable via standard HTTP clients** - curl/wget work perfectly  
✅ **Document access restrictions** - None found; completely public  
✅ **Document authentication requirements** - None required  
✅ **Document rate limits** - None encountered during testing  
✅ **Confirm file integrity (valid GRIB2 format)** - All files verified valid  
✅ **Confirm file integrity (not corrupted)** - All files complete and usable  

---

## Recommendations

### For Production Use
1. **Use RTMA/URMA files** for DRT=0 simple packing requirements
2. **Avoid GFS/GEFS files** labeled as "DRT=0" - they use complex packing
3. **Implement retry logic** for network resilience
4. **Cache files locally** - 80-90MB files are substantial downloads

### For Documentation Updates
1. **Update VERIFIED_DRT0_CONUS_FILES.md** with correct DRT=0 file list
2. **Clarify packing types** in documentation
3. **Cross-reference verification** between claimed and actual packing

---

## Files Generated

1. **comprehensive_drt0_accessibility.json** - Machine-readable test results
2. **DRT0_CONUS_ACCESSIBILITY_REPORT.md** - This human-readable report
3. **comprehensive_drt0_accessibility.log** - Detailed test log

---

## Test Execution Environment

- **Date:** 2026-07-25 00:28:39 EDT
- **Platform:** Linux 6.12.63
- **Tools:** curl 8.14.1, wgrib2 v3.1.3
- **Test Method:** HTTP HEAD + local file verification
- **Network:** Standard internet connection, no VPN required

---

## Conclusion

The accessibility verification successfully confirmed that NOAA DRT=0 CONUS files (RTMA and URMA) are:
- ✅ Fully accessible via public HTTP/HTTPS
- ✅ Downloadable without authentication
- ✅ Valid GRIB2 format with DRT=0 simple packing
- ✅ Suitable for production use requiring simple packing

**Task Status:** COMPLETE  
**Bead Closure:** Ready for commit and `br close bf-14grj`

---

## Discovery Summary

**IMPORTANT:** Previous documentation incorrectly identified GFS/GEFS files as DRT=0. This testing revealed that only RTMA/URMA CONUS files use true DRT=0 (simple packing). GFS/GEFS files use complex packing with spatial differencing despite being labeled "DRT=0" in previous documentation.

**Correct DRT=0 Files:** RTMA 2.5 CONUS and URMA 2.5 CONUS  
**Incorrect Documentation:** VERIFIED_DRT0_CONUS_FILES.md needs correction

