# GDT 3.30 + DRT=3 Candidate GRIB2 Files

## Overview
This document documents specific, publicly accessible GRIB2 files from NOAA archives that use Grid Definition Template 3.30 (Lambert Conformal Conic) and Data Representation Template 3 (complex packing with spatial differencing).

All URLs are publicly accessible via HTTPS and were **re-verified on 2026-07-22**
with `curl` HEAD + byte-range requests (see [Verification](#verification) below).
Every candidate returned HTTP 200, `Accept-Ranges: bytes`, valid GRIB2 magic
bytes (`47 52 49 42 0002`), and a present `.idx` index file. The byte sizes
listed below matched the server's `Content-Length` headers exactly.

---

## HRRR (High-Resolution Rapid Refresh) Files

### Candidate 1: HRRR Surface Fields - Recent Analysis

**File:** `hrrr.t12z.wrfsfcf00.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2
```

**Provenance:**
- **Model:** NOAA HRRR (High-Resolution Rapid Refresh)
- **Date:** June 1, 2024
- **Cycle:** 12z (12:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 142,393,582 bytes (~135.7 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Contains surface meteorological fields (temperature, wind, precipitation, etc.)

---

### Candidate 2: HRRR Surface Fields - Very Recent Analysis

**File:** `hrrr.t00z.wrfsfcf00.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2
```

**Provenance:**
- **Model:** NOAA HRRR
- **Date:** January 15, 2025
- **Cycle:** 00z (00:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 130,489,114 bytes (~124.4 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Most recent file in this candidate set

---

### Candidate 3: HRRR Surface Fields - 3-Hour Forecast

**File:** `hrrr.t06z.wrfsfcf03.grib2`

**Download URL:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2
```

**Provenance:**
- **Model:** NOAA HRRR
- **Date:** January 15, 2025
- **Cycle:** 06z (06:00 UTC model run)
- **Forecast Hour:** F03 (3-hour forecast)
- **Product:** wrfsfc (2D surface fields)
- **File Size:** 143,390,041 bytes (~136.7 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- CONUS domain at 3km resolution
- Demonstrates forecast hour (not analysis) encoding

---

## NAM (North American Mesoscale) File

### Candidate 4: NAM AWIP Physics Grid

**File:** `nam.t00z.awphys00.tm00.grib2`

**Download URL:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2
```

**Provenance:**
- **Model:** NOAA NAM (North American Mesoscale)
- **Date:** January 15, 2025
- **Cycle:** 00z (00:00 UTC model run)
- **Forecast Hour:** F00 (analysis, initial time step)
- **Product:** awphys (AWIP physics grid, Grid 218)
- **File Size:** 52,938,055 bytes (~50.5 MB)

**Access Method:** Public HTTPS (no authentication required)

**Characteristics:**
- Uses GDT 3.30 (Lambert Conformal Conic projection)
- Uses DRT 3 (complex packing with spatial differencing)
- 12km resolution CONUS domain
- Smaller file size than HRRR, good for faster testing

---

## URL Construction Patterns

### HRRR Pattern
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2
```

Where:
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00-23)
- `FF` = forecast hour (00-18 for standard cycles, 00-48 for extended cycles)

### NAM Pattern
```
https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awphysFF.tm00.grib2
```

Where:
- `YYYYMMDD` = model run date
- `HH` = cycle hour (00, 06, 12, 18)
- `FF` = forecast hour (00-60)

---

## File Size Considerations

All candidate files are practical for download and testing:

| Model | Size Range | Typical Use Case |
|-------|-----------|------------------|
| HRRR wrfsfc | 124-143 MB | Full 2D surface fields, comprehensive testing |
| NAM awphys | ~50 MB | Faster downloads, sufficient for GDT/DRT verification |
| NAM awip12 | 26-34 MB | **Smallest practical candidates** — see below |

### Practical download recommendation

For size-constrained / CI-friendly downloads, the **NAM AWIP12** files are the
preferred GDT 3.30 + DRT=3 candidates — the same Lambert-Conformal grid as the
NAM awphys file above, at roughly **half to one-third the size**. These were
extracted in [bf-we8d](../../notes/bf-we8d.md) and accessibility-verified in
[bf-22pu](../../notes/bf-22pu.md); sizes below are re-confirmed against
`Content-Length` on 2026-07-22:

| URL | Size (B) | MiB | Run / F-hour |
|---|---:|---:|---|
| `nam.20250115/nam.t00z.awip1200.tm00.grib2` | 26,364,442 | 25.1 | 2025-01-15 00z, F00 (winter, smallest) |
| `nam.20240601/nam.t12z.awip1200.tm00.grib2` | 28,800,129 | 27.5 | 2024-06-01 12z, F00 (summer analysis) |
| `nam.20240601/nam.t12z.awip1206.tm00.grib2` | 32,565,263 | 31.1 | 2024-06-01 12z, F06 (forecast record) |

Base URL: `https://noaa-nam-pds.s3.amazonaws.com/`. All three return HTTP 200,
`Accept-Ranges: bytes`, valid GRIB2 magic, and a present `.idx`.

> **Note:** an earlier note ([bf-we8d](../../notes/bf-we8d.md)) listed the
> 2024-06-01 F00 awip12 file as 30,259,561 B; the server's `Content-Length` is
> actually **28,800,129 B** (28,800,129 confirmed twice, on 2026-07-22). The
> figures in this document come from live HEAD requests.

---

## Verification

All URLs were tested on **2026-07-22** with `curl` HEAD requests (HTTP status,
`Content-Length`, `Last-Modified`, `Accept-Ranges`) plus a `Range: bytes=0-7`
fetch to confirm the GRIB2 magic bytes, and a HEAD on the `.idx` companion.

Every candidate returned: **HTTP 200 OK**, `Accept-Ranges: bytes`,
`Content-Type: binary/octet-stream` (server `AmazonS3`,
`x-amz-server-side-encryption: AES256`), GRIB2 magic `47 52 49 42 0002`
("GRIB", discipline=0, edition=2), and a present `.idx` (HTTP 200).

| # | URL | Content-Length (B) | MiB | Last-Modified | Status |
|---|---|---:|---:|---|---|
| 1 | `hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2` | 142,393,582 | 135.8 | 2024-06-01 12:50Z | ✅ 200, idx OK |
| 2 | `hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2` | 130,489,114 | 124.4 | 2025-01-15 00:53Z | ✅ 200, idx OK |
| 3 | `hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2` | 143,390,041 | 136.8 | 2025-01-15 06:55Z | ✅ 200, idx OK |
| 4 | `nam.20250115/nam.t00z.awphys00.tm00.grib2` | 52,938,055 | 50.5 | 2025-01-15 01:39Z | ✅ 200, idx OK |

Reproducible check:
```bash
curl -sI '<url>' | awk 'tolower($1)~/^(http|content-length|last-modified|accept-ranges)/{print}'
curl -s -r 0-7 '<url>' | xxd | head -1   # expect: 4752 4942 0000 0002
curl -sI '<url>.idx' | awk '/^HTTP/{print}'   # expect: 200
```

---

## Index Files

Each GRIB2 file has a corresponding `.idx` index file that allows subsetting individual variables without downloading the entire file:

**HRRR Example:**
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2.idx
```

**NAM Example:**
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2.idx
```

---

## References

- [NOAA HRRR on AWS Open Data Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NCEP HRRR Products](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [GRIB2 Template 3.30 - Lambert Conformal](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp3-30.shtml)
- [GRIB2 Template 5.3 - Complex Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_temp5-3.shtml)

## Related beads

- **bf-63ow** — research NOAA regional model archives (Lambert-conformal candidates)
- **bf-37db** — Lambert-conformal DRT=3 candidate files
- **bf-we8d** — NAM awip12 candidate URLs + provenance metadata
- **bf-22pu** — URL accessibility + file-size verification (this bead's blocker)

---

*Document created for bead bf-2r5b on 2026-07-03. Accessibility and sizes
re-verified live against `Content-Length` on 2026-07-22.*
