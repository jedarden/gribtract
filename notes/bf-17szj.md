# Ensemble GRIB2 Download - bf-17szj

## Task Completed: ✅

Successfully downloaded GEFS ensemble mean GRIB2 file to temporary location.

## Download Details

### Source Information
- **Source URL**: https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
- **Provider**: NOAA NCEP (National Centers for Environmental Prediction)
- **Product**: GEFS (Global Ensemble Forecast System) - Ensemble Mean
- **Date**: 2026-07-23 00z cycle, forecast hour F000
- **Resolution**: 0.5° global grid

### Download Location
- **Directory**: `/tmp/gribtract-download/`
- **Filename**: `geavg_20260723_t00z_f000.grib2`
- **Full Path**: `/tmp/gribtract-download/geavg_20260723_t00z_f000.grib2`

### File Integrity Verification ✅
- **File Size**: 13,991,214 bytes (13.4 MB) ✅
- **MD5 Checksum**: `899f1dc141fc98a72a9cc26e72a8c0b4`
- **GRIB2 Structure**: Valid ✅
- **Total Messages**: 71 ✅
- **Format**: GRIB2 Edition 2 ✅

### Verification Commands Used

```bash
# File size verification
ls -lah /tmp/gribtract-download/geavg_20260723_t00z_f000.grib2
stat -c %s /tmp/gribtract-download/geavg_20260723_t00z_f000.grib2

# GRIB2 structure validation
wgrib2 /tmp/gribtract-download/geavg_20260723_t00z_f000.grib2 -s | head -5
wgrib2 /tmp/gribtract-download/geavg_20260723_t00z_f000.grib2 -s | wc -l

# Integrity checksum
md5sum /tmp/gribtract-download/geavg_20260723_t00z_f000.grib2
```

### File Content

The file contains 71 GRIB2 messages with ensemble mean data:
- **PDT Distribution**: All 71 messages use PDT 4.2 (ensemble statistical products)
- **Variables**: HGT, TMP, RH, UGRD, VGRD, VVEL, PRES, PRMSL, TSOIL, SOILW, WEASD, SNOD, ICETK, PWAT, CAPE, CIN
- **Levels**: Multiple pressure levels (10mb through 1000mb) plus surface and atmospheric layers
- **Grid**: Global 0.5° resolution (720 x 361 points)

## Acceptance Criteria - All Met ✅

- ✅ File successfully downloaded to temporary location
- ✅ Downloaded file size matches source (13,991,214 bytes)
- ✅ Source URL and download location documented
- ✅ File is readable and has expected grib2 structure (71 messages confirmed)

## Notes

The downloaded file is identical to the previously validated fixture in `/tmp/geavg_20260723_t00z_f000.grib2` from bead bf-57o2r, confirming the download integrity and source reliability.
