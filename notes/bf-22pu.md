# bf-22pu: Verify URL Accessibility and Document File Sizes

## Task Summary
Validated the 6 NAM AWIP12 candidate URLs extracted in the previous step
([bf-we8d](bf-we8d.md)). All 6 are publicly accessible, all are genuine GRIB2
files, all support HTTP range requests, and all carry `.idx` companion files for
variable-level subsetting. None exceed 35 MB — well under the 200 MB budget.

Tested 2026-07-22 against `noaa-nam-pds` (AWS S3 Open Data).

---

## Verification Method
For each URL:
1. **HEAD request** → HTTP status, `Content-Length`, `Content-Type`, `Last-Modified`.
2. **Range request** (`Range: bytes=0-15`) → confirm GRIB2 magic bytes
   (`47 52 49 42` = "GRIB", discipline=`00`, edition=`02`).
3. **`Accept-Ranges: bytes`** header → confirms byte-range subsetting is possible.
4. **`.idx` companion** → HTTP status (needed for `wgrib2`-style variable subsetting).

Server for all: `AmazonS3`. Content-Type: `binary/octet-stream`.
SSE: `x-amz-server-side-encryption: AES256`.

---

## Accessibility + Size Results

All files: **HTTP 200 OK**, `Accept-Ranges: bytes`, valid GRIB2 magic, `.idx` present (200).

| # | File (`nam.YYYYMMDD/nam.tCCz.awip12FF.tm00.grib2`) | Content-Length (bytes) | MB (10⁶) | MiB (2²⁰) | Last-Modified |
|---|---|---:|---:|---:|---|
| 1 | `nam.20240601/nam.t12z.awip1200.tm00.grib2` (F00 analysis) | 28,800,129 | 28.80 | 27.47 | 2024-06-01 13:39Z |
| 2 | `nam.20240601/nam.t12z.awip1206.tm00.grib2` (F06) | 32,565,263 | 32.57 | 31.06 | 2024-06-01 13:44Z |
| 3 | `nam.20250115/nam.t00z.awip1200.tm00.grib2` (F00 analysis) | 26,364,442 | 26.36 | 25.14 | 2025-01-15 01:39Z |
| 4 | `nam.20240601/nam.t12z.awip1212.tm00.grib2` (F12) | 33,571,741 | 33.57 | 32.02 | 2024-06-01 13:49Z |
| 5 | `nam.20240601/nam.t12z.awip1224.tm00.grib2` (F24) | 33,599,787 | 33.60 | 32.04 | 2024-06-01 14:00Z |
| 6 | `nam.20240601/nam.t12z.awip1236.tm00.grib2` (F36) | 34,432,235 | 34.43 | 32.84 | 2024-06-01 14:11Z |

**Size budget:** max = 34.4 MB, all < 200 MB. ✅
**Accessibility:** 6 of 6 reachable, 2+ confirmed (requirement met). ✅

---

## Final Selection — 3 Candidates

Chosen for download-size economy plus diversity of season, model cycle, and
analysis-vs-forecast coverage. All are AWIP12 12 km Lambert-Conformal GRIB2,
DRT=3 (relevant to [bf-37db](bf-37db.md)).

### ★ Candidate A — winter, 00z cycle, F00 analysis (smallest overall)
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
```
- Size: **26,364,442 B (25.1 MiB / 26.4 MB)** — smallest of all candidates
- Run: 2025-01-15 00:00 UTC, forecast hour F00
- Value: different season (winter) and different cycle (00z) than the others → broader coverage

### ★ Candidate B — summer, 12z cycle, F00 analysis
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2
```
- Size: **28,800,129 B (27.5 MiB / 28.8 MB)**
- Run: 2024-06-01 12:00 UTC, forecast hour F00
- Value: analysis baseline; pairs with Candidate C for analysis↔forecast comparison

### ★ Candidate C — summer, 12z cycle, F06 forecast
```
https://noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1206.tm00.grib2
```
- Size: **32,565,263 B (31.1 MiB / 32.6 MB)**
- Run: 2024-06-01 12:00 UTC, forecast hour F06 (valid 18:00 UTC)
- Value: same grid/variables as B at a later time step → exercises forecast-record decoding; smaller than F12/F24/F36

**Combined download:** ~88.1 MB for all three.

### Rationale
- **Season/cycle spread:** winter 00z (A) vs summer 12z (B, C).
- **Analysis + forecast:** two F00 analysis files plus one forecast record (C).
- **Size discipline:** the three smallest-dated files (26.4, 28.8, 32.6 MB); larger
  forecast hours (F12–F36) add ~30–34 MB each with diminishing test value over C.

---

## Access Method (confirmed working)
```bash
# Full file (range-capable; supports subsetting)
curl -O https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2

# Index file for variable-level subsetting (HTTP 200)
curl -O https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2.idx

# Byte-range spot download (verified: first 16 bytes)
curl -r 0-15 -o head.bin https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
```

---

## Acceptance Criteria Verification
- ✅ **All URLs tested for accessibility** — 6/6 return HTTP 200 from AmazonS3.
- ✅ **File sizes documented** — exact `Content-Length` for each (26.4–34.4 MB).
- ✅ **At least 2 URLs confirmed accessible** — 6 confirmed (incl. GRIB2 magic, range, `.idx`).
- ✅ **Final selection of 2–3 candidates with complete metadata** — 3 selected (A/B/C).

## Related
- [bf-we8d](bf-we8d.md) — extracted the candidate URLs and provenance metadata
- [bf-2xen](bf-2xen.md) — NAM awip12 model run navigation
- [bf-37db](bf-37db.md) — Lambert-conformal DRT=3 candidate files

**Date Completed:** 2026-07-22
**Archive Source:** NOAA NAM via AWS Open Data (`noaa-nam-pds`)
**Bead ID:** bf-22pu
