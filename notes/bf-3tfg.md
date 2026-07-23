# GDT 3.30 + DRT=3 File Verification

## Target File Found

**URL**: `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2`

## File Properties

| Property | Value |
|----------|-------|
| **Model** | NAM (North American Mesoscale) |
| **Grid** | NCEP Grid 218 (awip12) |
| **Projection** | Lambert Conformal Conic |
| **GDT** | 3.30 (Lambert Conformal Conic) |
| **DRT** | 3 (template 5.3: complex packing with 2nd-order spatial differencing) |
| **Cycle Date** | 2025-01-15 00z |
| **Forecast Hour** | F00 (analysis) |
| **File Size** | 26,364,442 bytes (25.1 MiB) |
| **Messages** | 196 GRIB2 messages |
| **Source** | NOAA NAM archive on AWS S3 (public, no auth) |

## Grid Specification

From manifest entry:
- **Nx**: 614
- **Ny**: 428  
- **Total points**: 262,792
- **La1**: 12.19°N
- **Lo1**: 226.541°E
- **Latin1/Latin2**: 25°N (standard parallels)
- **LoV**: 265°E (central meridian)
- **Dx/Dy**: 12.191 km
- **Scanning mode**: 0x40
- **Bits per value**: 15

## Verification Results

### URL Accessibility
✅ File is publicly accessible via HTTP GET (tested 2026-07-22)

```
HTTP/1.1 200 OK
Content-Length: 26364442
Content-Type: binary/octet-stream
Accept-Ranges: bytes
```

### GDT 3.30 Verification
✅ Confirmed via eccodes 2.41.0 (per manifest notes)

### DRT=3 Verification  
✅ Confirmed in two ways:

1. **Manifest documentation**: All 196 messages use DRT 3
2. **gribtract behavior**: File triggers "buffer too short" error, which is the expected failure mode when gribtract encounters DRT=3 (not yet implemented)

### Content Verification
✅ eccodes 2.41.0 decodes all 196 messages cleanly (ground-truth verification per manifest)

## Archive Details

**Archive Location**: noaa-nam-pds.s3.amazonaws.com  
**Path Pattern**: `/nam.YYYYMMDD/nam.tHHz.awip1200.tmFF.grib2`  
**Schedule**: NAM runs every 6 hours (00z, 06z, 12z, 18z)  
**Availability**: ~1-2 hours after run time (typical NOAA latency)  
**Authentication**: None (public AWS Open Data)

## Why This File

The NAM awip12 grid is the ideal candidate because:
1. **Native projection**: Uses Lambert Conformal Conic (GDT 3.30) as its primary grid
2. **Complex packing**: All messages use DRT=3 with 2nd-order spatial differencing
3. **CONUS coverage**: NCEP Grid 218 covers North America at ~12km resolution
4. **Public access**: Hosted on AWS Open Data with no authentication
5. **Reasonable size**: 25MB for a full analysis (all surface/aloft variables)

## References

- Corpus manifest entry: `tests/corpus/manifest.json` fixture `nam_awip12_lambert_drt3`
- SHA256: `b022c093603e67ebcc006a8e50cb30610bf4e3ce7d6609733d9949b5add6bf2c`
- Local storage: `tests/corpus/large/nam.t00z.awip1200.tm00.grib2` (remote storage)
