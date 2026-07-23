# Ensemble File Download to Temporary Location - bf-569fv

## Task Summary
The ensemble file was already downloaded to a temporary location from a previous task (bf-3yzzm). This task verified the file integrity and documented the local file path.

## File Location

**Local Path:** `/tmp/grib2-ensemble/gefs_perturbed_p01_20260723_t00z_f000.grib2`

**Source URL:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000
```

## Integrity Verification

### File Size Check
- **Expected size:** 13,984,963 bytes (~13.3 MB)
- **Actual size:** 13,984,963 bytes ✅
- **Status:** Size matches exactly

### File Accessibility
- **Messages:** 71 GRIB2 messages
- **Decode test:** Passes wgrib2 validation ✅
- **Sample content:** HGT (geopotential height), TMP (temperature), RH (relative humidity), UGRD/VGRD (wind components) at various pressure levels
- **Ensemble marker:** ENS=+1 (perturbed member 01)

## File Details
- **Product:** GEFS (Global Ensemble Forecast System)
- **Member:** Perturbed member 01 (p01)
- **Resolution:** 0.5° (pgrb2ap5 field set)
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 000 (analysis time)

## Acceptance Criteria Status

✅ **File downloaded successfully to temp location** - File present at `/tmp/grib2-ensemble/`
✅ **File size verified against expected** - 13,984,963 bytes matches expected size
✅ **Local file path documented** - Documented in this note
✅ **File accessible for decoding** - Verified with wgrib2, all 71 messages decode correctly

## Related Tasks
- **bf-3yzzm:** Original file selection and PDT verification
- **bf-11rzg:** Ensemble GRIB2 test fixture validation
