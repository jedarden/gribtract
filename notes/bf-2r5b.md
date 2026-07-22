# bf-2r5b: Locate Specific Candidate Files (GDT 3.30 + DRT=3)

## Task Summary
Located and **live-verified** specific, publicly accessible GRIB2 files that use
Grid Definition Template 3.30 (Lambert Conformal Conic) and Data Representation
Template 3 (complex packing with spatial differencing).

A research document ([docs/research/bf-2r5b-gdt330-drt3-candidate-files.md](../docs/research/bf-2r5b-gdt330-drt3-candidate-files.md))
already existed from a prior pass (2026-07-03), but it carried precise-looking
file sizes with no recorded verification. I re-verified every URL on 2026-07-22
with `curl` HEAD + byte-range requests; all sizes matched the server's
`Content-Length` exactly, and all returned valid GRIB2 magic + `.idx` files.

---

## Verified Candidates (2026-07-22)

All: HTTP 200, `Accept-Ranges: bytes`, `Content-Type: binary/octet-stream`,
GRIB2 magic `47 52 49 42 0002`, `.idx` present.

### HRRR wrfsfc — GDT 3.30 + DRT 3, CONUS 3 km
| URL | Size (B) | MiB | Provenance |
|---|---:|---:|---|
| `noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2` | 142,393,582 | 135.8 | 2024-06-01 12z, F00 |
| `noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t00z.wrfsfcf00.grib2` | 130,489,114 | 124.4 | 2025-01-15 00z, F00 |
| `noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250115/conus/hrrr.t06z.wrfsfcf03.grib2` | 143,390,041 | 136.8 | 2025-01-15 06z, F03 |

### NAM awphys — GDT 3.30 + DRT 3, CONUS 12 km
| URL | Size (B) | MiB | Provenance |
|---|---:|---:|---|
| `noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awphys00.tm00.grib2` | 52,938,055 | 50.5 | 2025-01-15 00z, F00 |

### ★ Recommended small-download candidates — NAM awip12 (26–34 MB)
Extracted in [bf-we8d](bf-we8d.md), verified in [bf-22pu](bf-22pu.md);
re-confirmed against `Content-Length` here. Same Lambert grid as awphys, ~½–⅓ the size.
| URL | Size (B) | MiB | Provenance |
|---|---:|---:|---|
| `noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2` | 26,364,442 | 25.1 | 2025-01-15 00z, F00 (winter, smallest) |
| `noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1200.tm00.grib2` | 28,800,129 | 27.5 | 2024-06-01 12z, F00 (summer) |
| `noaa-nam-pds.s3.amazonaws.com/nam.20240601/nam.t12z.awip1206.tm00.grib2` | 32,565,263 | 31.1 | 2024-06-01 12z, F06 (forecast) |

---

## Data-quality note
The prior [bf-we8d](bf-we8d.md) note listed the 2024-06-01 F00 awip12 file as
30,259,561 B, but the server's `Content-Length` is actually **28,800,129 B**
(confirmed twice on 2026-07-22). All sizes in this note and the research doc
come from live HEAD requests, not estimates.

---

## Acceptance Criteria Verification
- ✅ **At least 2 specific file URLs documented** — 4 HRRR/awphys + 3 awip12 = 7 total.
- ✅ **Provenance (date, model run, forecast hour)** — every URL tagged with YYYYMMDD, cycle, F-hour.
- ✅ **File sizes noted, practical to download** — awip12 candidates 25–31 MiB; awphys 50.5 MiB; HRRR 124–137 MiB.
- ✅ **URLs publicly accessible** — 7/7 return HTTP 200 + range support + GRIB2 magic + `.idx`.

## Related
- [bf-63ow](bf-63ow.md) — NOAA regional model archive research (blocker)
- [bf-22pu](bf-22pu.md) — URL accessibility + size verification (blocker)
- [bf-we8d](bf-we8d.md), [bf-37db](bf-37db.md) — upstream candidate extraction
- Research doc: [docs/research/bf-2r5b-gdt330-drt3-candidate-files.md](../docs/research/bf-2r5b-gdt330-drt3-candidate-files.md)

**Date Completed:** 2026-07-22
**Archive Source:** NOAA HRRR / NAM via AWS Open Data (`noaa-hrrr-bdp-pds`, `noaa-nam-pds`)
**Bead ID:** bf-2r5b
