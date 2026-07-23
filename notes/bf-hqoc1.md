# NOAA Ensemble GRIB2 File Download - bf-hqoc1

## Task Summary

Successfully downloaded a candidate ensemble GRIB2 file from NOAA's GEFS archive via Amazon S3 public bucket.

## Download Details

### Source URL
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
```

### Download Timestamp
- **Date:** 2026-07-23
- **Time:** 18:43 UTC (approximate)
- **Server Time:** Thu, 23 Jul 2026 03:46:18 GMT (file modification time)

### File Information
- **Product:** GEFS (Global Ensemble Forecast System)
- **Resolution:** 0.5° (pgrb2a field set)
- **Member:** Control (c00) - ensemble control member
- **Cycle:** 00 UTC, 2026-07-23
- **Forecast Hour:** 000 (analysis time)
- **Expected PDT:** 4.1 (individual ensemble member)
- **File Size:** 13,476,191 bytes (~12.9 MB) ✅ Within <50MB target
- **MD5 Checksum:** a045f4fe22d3d76669c6983b773c13eb

### File Locations
- **Download Location:** `/tmp/gefs_control_20260723_t00z_f000.grib2`
- **Index File:** `/tmp/gefs_control_20260723_t00z_f000.grib2.idx`

## File Verification

### GRIB2 Format Validation
✅ **Valid GRIB2 file** - File starts with "GRIB" magic bytes

### Index File Statistics
- **Total Fields:** 71 GRIB messages in the file
- **Index Format:** `record_number:byte_offset:date:variable:level:forecast_type:extra_info`

### Sample Fields (from index)
```
1:0:d=2026072300:HGT:10 mb:anl:ENS=low-res ctl
2:191634:d=2026072300:TMP:10 mb:anl:ENS=low-res ctl
3:320323:d=2026072300:RH:10 mb:anl:ENS=low-res ctl
4:361733:d=2026072300:UGRD:10 mb:anl:ENS=low-res ctl
5:618098:d=2026072300:VGRD:10 mb:anl:ENS=low-res ctl
```

Variables include: HGT (Geopotential Height), TMP (Temperature), RH (Relative Humidity), UGRD/VGRD (Wind Components), etc.

## Access Method

### Method: Amazon S3 Public Bucket
- **Endpoint:** `https://noaa-gefs-pds.s3.amazonaws.com/`
- **Authentication:** None required (public access)
- **Server Headers:**
  - HTTP/1.1 200 OK
  - Last-Modified: Thu, 23 Jul 2026 03:46:18 GMT
  - ETag: "a045f4fe22d3d76669c6983b773c13eb"
  - Accept-Ranges: bytes (byte-range requests supported)
  - Content-Type: binary/octet-stream
  - Content-Length: 13476191

## Alternative Candidate Files (Not Downloaded)

Based on bf-5nbaj.md research, the following are additional candidates:

### 1. GEFS 0.25° Perturbed Member
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap25/gec01.t00z.pgrb2a.0p25.f003
```
- Member: Perturbed #1 (p01)
- Resolution: 0.25°
- Forecast Hour: 003

### 2. GEFS Ensemble Mean
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
```
- Member: Ensemble mean
- Resolution: 0.5°
- Forecast Hour: 000

### 3. SREF ARW Control Member (if available)
```
https://nomads.ncep.noaa.gov/sref.20260723/03/pgrb/sref_arw.t03z.pgrb212.ctl.f00.grib2
```
Note: SREF is being decommissioned per NOAA SCN26-48

## Acceptance Criteria Verification

✅ **File downloaded successfully** - 13.5 MB file downloaded without errors
✅ **Source URL and timestamp documented** - URL, download time, and file metadata recorded
✅ **File size noted and within reasonable range** - 13.5 MB is well under 50MB target
✅ **File saved with descriptive name** - `gefs_control_20260723_t00z_f000.grib2` includes product, member, date, cycle, and forecast hour

## Notes

- File contains ensemble control member data, expected to include PDT 4.1 messages
- Index file downloaded for reference (useful for subsetting individual fields)
- Amazon S3 provides the most reliable access method for NOAA ensemble data
- MD5 checksum matches server ETag, confirming download integrity

## References

- bf-5nbaj.md - NOAA Ensemble GRIB2 Archive Sources
- bf-4kc1d/noaa-ensemble-download-methods.md - Download Methods Documentation
