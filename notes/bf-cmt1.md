# bf-cmt1: Final Candidate File — Provenance Documentation

Compiles the verified findings of the bf-22pu → bf-2r5b → bf-qa5j chain into a
single self-contained provenance record for [bf-5yoa](bf-5yoa.md)
("Find and validate NOAA Lambert-conformal DRT=3 source file").

The chain located, accessibility-verified, and authoritatively (eccodes) decoded
NOAA GRIB2 files using **Grid Definition Template 3.30 (Lambert Conformal Conic)**
and **Data Representation Template 3 (complex packing + 2nd-order spatial
differencing)**. A primary candidate plus alternates are documented below.

---

## ★ Primary candidate (recommended)

**NAM awip12, winter analysis, smallest file in the set.**

| Field | Value |
|---|---|
| **Direct download URL** | `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2` |
| **Index (.idx) URL** | `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2.idx` |
| **Model** | NOAA NAM (North American Mesoscale) |
| **Run date / cycle** | 2025-01-15, 00z (00:00 UTC) |
| **Forecast hour** | F00 (analysis — initial time step) |
| **Product / grid** | awip12 (AWIP 12-km physics grid, NCEP Grid 218) |
| **File size** | 26,364,442 B (25.1 MiB) |
| **Last-Modified** | Wed, 15 Jan 2025 01:39:19 GMT |
| **Messages** | 196 |

---

## Verification status (as of 2026-07-22)

| Check | Result | Method |
|---|---|---|
| **GDT 3.30** (Lambert Conformal) | ✅ **196/196 messages** | `grib_get gridDefinitionTemplateNumber` → all `30` (eccodes) |
| **DRT 3** (complex packing, spatial differencing) | ✅ **196/196 messages** | `grib_get dataRepresentationTemplateNumber` → all `3`; `packingType=grid_complex_spatial_differencing` (eccodes) |
| **URL accessible** | ✅ HTTP 200, `Accept-Ranges: bytes`, `Content-Type: binary/octet-stream` | `curl -sI` (live, 2026-07-22) |
| **Size matches** | ✅ `Content-Length: 26364442` = documented 26,364,442 B | `curl -sI` |
| **Valid GRIB2** | ✅ magic `4752 4942 0000 0002` ("GRIB", discipline=0, edition=2) | `curl -s -r 0-7 <url> \| xxd` |
| **Index present** | ✅ `.idx` returns HTTP 200 | `curl -sI <url>.idx` |

All four NAM files in the set were decoded the same way — **1046/1046 messages
are GDT 3.30 + DRT=3** (see [bf-qa5j](bf-qa5j.md)).

> **Caveat on the project's own decoder:** `gribtract-cli list` cannot yet read
> these files — DRT=3 unpacking is not implemented (fails with
> `error: buffer too short: need 0, got 262792`). eccodes is the ground-truth
> verifier used here, the same tool the project uses for golden generation.

---

## Grid parameters — NCEP Grid 218 (Lambert Conformal), message 1

Extracted authoritatively via `grib_get` / `grib_dump -j` (eccodes). The `awip12`
and `awphys` NAM files share the identical grid (verified: same GDT/Nx/Ny/La1/Lo1).

| Parameter (GRIB2 key) | Value | Meaning |
|---|---|---|
| `gridDefinitionTemplateNumber` | **30** | Lambert Conformal Conic |
| `gridType` | `lambert` | — |
| `shapeOfTheEarth` | 6 | spherical, R = 6,371,229 m |
| `Nx` (Ni) | 614 | points along a parallel |
| `Ny` (Nj) | 428 | points along a meridian |
| `numberOfDataPoints` | 262,792 | = 614 × 428 |
| `latitudeOfFirstGridPointInDegrees` (**La1**) | 12.19 °N | first grid point lat |
| `longitudeOfFirstGridPointInDegrees` (**Lo1**) | 226.541 °E (= 133.459 °W) | first grid point lon |
| `LaDInDegrees` (**LaD**) | 25 °N | latitude where Dy is specified |
| `LoVInDegrees` (**LoV**) | 265 °E (= 95 °W) | grid orientation longitude |
| **Latin1** / **Latin2** | 25 °N / 25 °N | standard parallels |
| **Dx** / **Dy** | 12,191 m / 12,191 m | **12.191 km** resolution |
| scanning mode byte | `0x40` | +i, +j; `iScansNegatively`=0, `jPointsAreConsecutive`=0 |
| `centre` / `subCentre` | `kwbc` (7) / 0 | NCEP |

> **La2 / Lo2 (last grid point):** eccodes does not expose
> `latitudeOfLastGridPoint` / `longitudeOfLastGridPoint` for Lambert Conformal
> (3.30) — confirmed `<absent>` in `grib_dump -j`. Per the published NCEP Grid 218
> definition the last grid point is **La2 ≈ 47.442 °N, Lo2 ≈ 357.476 °E
> (2.524 °W)**; documented rather than file-derived.

## Packing detail — DRT 3 (template 5.3), message 1

| Key | Value |
|---|---|
| `dataRepresentationTemplateNumber` | **3** (template 5.3) |
| `packingType` | `grid_complex_spatial_differencing` |
| `bitsPerValue` | 15 |
| `orderOfSpatialDifferencing` | 2 (second-order) |
| `referenceValue` (R) | 9.94887 × 10⁶ |
| `binaryScaleFactor` (E) | 4 |
| `decimalScaleFactor` (D) | 2 |

`grid_complex_spatial_differencing` is eccodes' name for GRIB2 template 5.3,
confirming DRT=3 is complex packing **with** spatial differencing (vs. 5.2).

---

## Download method & access requirements

- **Access:** public HTTPS, **no authentication**. Served from AWS Open Data
  (`noaa-nam-pds` S3 bucket, `server: AmazonS3`,
  `x-amz-server-side-encryption: AES256`).
- **Range support:** `Accept-Ranges: bytes` — partial/byte-range downloads work,
  and each `.idx` allows subsetting individual variables without the full file.
- **Reproducible fetch:**
  ```bash
  URL='https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2'
  curl -sI "$URL" | awk 'tolower($1)~/^(http|content-length|last-modified|accept-ranges)/{print}'
  curl -s -r 0-7 "$URL" | xxd        # expect: 4752 4942 0000 0002
  curl -sO "$URL"                    # full download (26 MB)
  ```
- **URL construction pattern:** `https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awip12FF.tm00.grib2`
  where `HH` ∈ {00,06,12,18} and `FF` is the forecast hour.

---

## Alternates (all GDT 3.30 + DRT=3, accessibility-verified 2026-07-22)

For season/cycle/forecast-hour spread, or a fuller message set. All confirmed
GDT 3.30 + DRT=3 by eccodes in [bf-qa5j](bf-qa5j.md).

| File | URL (under `https://noaa-nam-pds.s3.amazonaws.com/`) | Size (B) | Run / F-hour | Msgs |
|---|---|---:|---|---:|
| `nam.t12z.awip1200.tm00.grib2` | `nam.20240601/nam.t12z.awip1200.tm00.grib2` | 28,800,129 | 2024-06-01 12z, F00 (summer) | 196 |
| `nam.t12z.awip1206.tm00.grib2` | `nam.20240601/nam.t12z.awip1206.tm00.grib2` | 32,565,263 | 2024-06-01 12z, F06 (forecast) | 208 |
| `nam.t00z.awphys00.tm00.grib2` | `nam.20250115/nam.t00z.awphys00.tm00.grib2` | 52,938,055 | 2025-01-15 00z, F00 (awphys, fuller) | 446 |
| HRRR `wrfsfcf00/f03` | `noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2` | 130–143 MB | CONUS 3 km | — |

> **bf-we8d data-quality note:** that earlier bead listed the 2024-06-01 F00
> awip12 file as 30,259,561 B; the server's actual `Content-Length` is
> **28,800,129 B** (confirmed twice on 2026-07-22). Figures here come from live
> HEAD requests. HRRR files were not eccodes-decoded in bf-qa5j — the four NAM
> files already gave four independent confirmations at ¼–⅕ the byte cost.

---

## Provenance chain (full audit trail)

| Bead | Contribution |
|---|---|
| [bf-63ow](bf-63ow.md) | Researched NOAA regional model archives for Lambert-conformal candidates |
| [bf-37db](bf-37db.md) | Located Lambert-conformal DRT=3 candidate files |
| [bf-we8d](bf-we8d.md) | Extracted NAM awip12 URLs + metadata (size since corrected) |
| [bf-22pu](bf-22pu.md) | Verified URL accessibility + file sizes (HTTP 200, magic, `.idx`) |
| [bf-2r5b](bf-2r5b.md) | Live-verified all candidate URLs + sizes against `Content-Length` (2026-07-22) |
| [bf-qa5j](bf-qa5j.md) | **Authoritatively decoded** all 1046 msgs via eccodes — confirmed GDT 3.30 + DRT=3, documented grid + packing |
| **bf-cmt1** (this) | Compiles the above into final provenance + comments [bf-5yoa](bf-5yoa.md) |

Research doc: [docs/research/bf-2r5b-gdt330-drt3-candidate-files.md](../docs/research/bf-2r5b-gdt330-drt3-candidate-files.md)

**Date Completed:** 2026-07-22
**Bead ID:** bf-cmt1
