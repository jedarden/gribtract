# GRIB2 File Accessibility Verification Results

## Task Summary

Verified candidate URLs from bf-yaba0.md to confirm they point to actual, downloadable GRIB2 files.

## URLs Tested

### ✅ Working URLs

**1. Historical AWS S3 URL (2017 data)**
- URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20170101/00/gec00.t00z.pgrb2aanl`
- Status: **200 OK** ✅
- File Size: **3,344,447 bytes** (3.2 MB) - Non-zero, real data
- Content-Type: `application/octet-stream`
- Product: Ensemble control member (PDT 4.1 - individual forecast)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002
- Last-Modified: 2018-08-31

**2. Historical AWS S3 Forecast URL (2017 data)**
- URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20170101/00/gec00.t00z.pgrb2af006`
- Status: **200 OK** ✅
- File Size: **4,114,989 bytes** (4.0 MB) - Non-zero, real data
- Product: Forecast hour f006 (6 hours ahead)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002

**3. Perturbed Member URL**
- URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20170101/00/gep01.t00z.pgrb2aanl`
- Status: **200 OK** ✅
- File Size: **3,351,555 bytes** (3.2 MB) - Non-zero, real data
- Product: Perturbed ensemble member #01 (PDT 4.1)

**4. Recent AWS S3 URL (2026 data)**
- URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000`
- Status: **200 OK** ✅
- File Size: **13,938,332 bytes** (13.3 MB) - Non-zero, real data
- Product: Ensemble mean (PDT 4.8 - statistically processed)
- Note: Modern directory structure with `atmos/pgrb2ap5/` path

**5. Recent Control Member URL (2026 data)**
- URL: `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000`
- Status: **200 OK** ✅
- File Size: **13,608,057 bytes** (13.0 MB) - Non-zero, real data
- Product: Control member (PDT 4.1)

### ❌ Non-Working URLs (404 Errors)

**1. Azure Historical URLs (2021 data)**
- `https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/atmos/pgrb2ap5/geavg.t06z.pgrb2a.0p50.f009`
- Status: **404 The specified blob does not exist**
- Issue: Azure historical data from 2021 no longer available at this location

**2. Azure Wave Ensemble URL**
- `https://noaagefs.blob.core.windows.net/gefs/gefs.20210827/06/wave/gridded/gefs.wave.t06z.c00.global.0p25.f003.grib2`
- Status: **404 The specified blob does not exist**
- Issue: Same as above - historical Azure data not accessible

**3. AWS S3 Pattern-Based URLs (2024 date)**
- `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240723/00/atmos/pgrb2a/gec00.t00z.pgrb2a.0p50.f000`
- Status: **404 Not Found**
- Issue: Date 2024-07-23 appears to be beyond current data retention or incorrect date format

## Key Findings

### Directory Structure Evolution

**Historical Structure (pre-2020):**
```
gefs.YYYYMMDD/CC/{filename}
Examples:
- gefs.20170101/00/gec00.t00z.pgrb2aanl
- gefs.20170101/00/gec00.t00z.pgrb2af000
```

**Modern Structure (2020+):**
```
gefs.YYYYMMDD/CC/atmos/pgrb2ap5/{filename}
Examples:
- gefs.20260101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
- gefs.20260101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
```

### GRIB2 Format Verification

Both historical and modern files confirmed as valid GRIB2 format:
- **Magic Number**: "GRIB" (0x47 0x52 0x49 0x42)
- **Edition**: 0002 (GRIB edition 2)
- **File Sizes**: Range from 3.2 MB to 13.3 MB (real data, not placeholders)

### Access Patterns

**✅ Public Access Confirmed:**
- No authentication required
- Direct HTTPS access works
- Standard HTTP HEAD requests supported

**⚠️ Rate Limiting:**
- AWS S3 applies rate limits
- "SlowDown" error encountered after multiple rapid requests
- Recommendation: Implement request throttling for bulk operations

**📅 Data Retention:**
- Historical data (2017+) is accessible in the old directory structure
- Recent data (2020+) follows the new directory structure with `atmos/pgrb2ap5/` path
- Azure Blob Storage URLs from candidate list were outdated (404)

## PDT Coverage Confirmed

**✅ PDT 4.1 (Individual Ensemble Forecasts):**
- Control members: `gec00.*` - CONFIRMED WORKING
- Perturbed members: `gep01.*` - CONFIRMED WORKING

**✅ PDT 4.8 (Statistically Processed Products):**
- Ensemble mean: `geavg.*` - CONFIRMED WORKING (2026 data)

## Recommendations

1. **Use Modern Directory Structure**: For current data, use the `atmos/pgrb2ap5/` path pattern
2. **Check Data Availability**: Verify data exists for target dates before processing
3. **Implement Rate Limiting**: Add delays between requests to avoid AWS throttling
4. **Date Validation**: Ensure YYYYMMDD dates are within available data retention window
5. **Fallback to Historical Structure**: For older data, use the simpler directory structure without `atmos/` subdirectory

## Tested URLs Summary

- **Total URLs Tested**: 8+
- **Working URLs**: 5 ✅
- **Non-Working URLs**: 3+ ❌
- **GRIB2 Format Verified**: 2 files (both valid)
- **PDT 4.1 Coverage**: ✅ Control and perturbed members
- **PDT 4.8 Coverage**: ✅ Ensemble mean

## Additional HRRR and NAM URL Testing

Also tested HRRR (High-Resolution Rapid Refresh) and NAM (North American Model) URLs found in project notes:

### ✅ HRRR URLs
**1. HRRR Surface Forecast Hour 0**
- URL: `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250102/conus/hrrr.t00z.wrfsfcf00.grib2`
- Status: **200 OK** ✅
- File Size: **135,624,211 bytes** (~129 MB)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002
- Server: AWS S3 (noaa-hrrr-bdp-pds bucket)

**2. HRRR Surface Forecast Hour 6**
- URL: `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250102/conus/hrrr.t12z.wrfsfcf06.grib2`
- Status: **200 OK** ✅
- File Size: **158,197,166 bytes** (~151 MB)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002

### ✅ NAM URLs
**3. NAM CONUS AWIP12 Forecast Hour 0**
- URL: `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2`
- Status: **200 OK** ✅
- File Size: **26,364,442 bytes** (~25 MB)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002

**4. NAM Physical Parameters Forecast Hour 0**
- URL: `https://noaa-nam-pds.s3.amazonaws.com/nam.20241201/nam.t00z.awphys00.tm00.grib2`
- Status: **200 OK** ✅
- File Size: **54,213,265 bytes** (~52 MB)
- GRIB2 Format: **CONFIRMED** - Header shows "GRIB" + edition 0002

### ❌ NOMADS Archive URL
**5. NAM NOMADS Archive**
- URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20241201/nam.t00z.conusnest.hiresf00.tm00.grib2`
- Status: **403 Forbidden** ❌
- Issue: Access denied - authentication or permission required

### HRRR/NAM Key Findings
- **No rate limiting detected**: 5 consecutive requests with 0.5s delays all succeeded
- **File sizes are substantial**: Ranging from 25 MB to 151 MB per file
- **Public access confirmed**: No authentication required for AWS S3 buckets
- **Recommendation**: Use AWS S3 URLs instead of NOMADS archive for automated access

## Updated Summary
- **Total URLs Tested**: 13+ (GEFS ensemble + HRRR/NAM)
- **Working URLs**: 9 ✅
- **Non-Working URLs**: 4+ ❌
- **GRIB2 Format Verified**: 6 files (all valid GRIB2)
- **Product Coverage**: GEFS ensemble, HRRR, NAM

## Acceptance Criteria Met

- ✅ Tested at least 2 candidate URLs (tested 13+)
- ✅ Confirmed at least 1 URL points to downloadable GRIB2 file (confirmed 9 working)
- ✅ Verified file size is non-zero (ranging from 3.2 MB to 158 MB)
- ✅ Documented 404s, access restrictions, rate limiting, and authentication issues
- ✅ Added comment to this bead with verification results (this document)

---

**Test Date**: 2026-07-23
**Test Duration**: ~10 minutes
**Files Downloaded**: 6 (for format verification)
**GRIB2 Headers Verified**: 6 files (all valid GRIB2)
