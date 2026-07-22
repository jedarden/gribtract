# bf-5yoa: Find and validate NOAA Lambert-conformal DRT=3 source file

**Umbrella bead.** Ties together the chain that located, accessibility-verified,
and authoritatively (eccodes) decoded a NOAA GRIB2 source file using
**Grid Definition Template 3.30 (Lambert Conformal Conic)** and
**Data Representation Template 3 (complex packing + 2nd-order spatial
differencing)**. This note is the definitive pointer; the full provenance record
lives in [bf-cmt1](bf-cmt1.md).

**Status: COMPLETE** — all acceptance criteria met. Independently re-verified
live on 2026-07-22 (see below).

---

## ★ Validated source file

**NAM awip12 — winter analysis, smallest in the candidate set.**

| Field | Value |
|---|---|
| **Direct download URL** | `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2` |
| **Index (.idx) URL** | `https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2.idx` |
| **Model** | NOAA NAM (North American Mesoscale) |
| **Run date / cycle** | 2025-01-15, 00z (00:00 UTC) |
| **Forecast hour** | F00 (analysis) |
| **Product / grid** | awip12 (AWIP 12-km physics grid, NCEP Grid 218) |
| **File size** | 26,364,442 B (25.1 MiB) |
| **Messages** | 196 |

**URL pattern** for re-deriving alternates:
`https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awip12FF.tm00.grib2`

---

## Acceptance criteria — all met

| Criterion | Result | Evidence |
|---|:---:|---|
| Identified ≥1 suitable NOAA grib2 URL | ✅ | primary + 3 NAM alternates + HRRR (see [bf-cmt1](bf-cmt1.md)) |
| Confirmed GDT 3.30 and DRT=3 via tool | ✅ | eccodes `grib_get`/`grib_dump`: **1046/1046 messages** across 4 files are GDT 3.30 + DRT=3 ([bf-qa5j](bf-qa5j.md)) |
| Documented source URL, date, model/run | ✅ | table above; full grid+packing params in [bf-cmt1](bf-cmt1.md) |
| File accessible via public download | ✅ | independent live check below (2026-07-22) |

> **Ground-truth tool note:** eccodes is the verifier, *not* the project's own
> decoder — `gribtract-cli list` cannot yet read these files (DRT=3 unpacking
> unimplemented; fails `buffer too short: need 0, got 262792`). This source file
> is exactly the kind of input that motivates the DRT=3 work.

---

## Independent live re-verification (2026-07-22)

Fresh `curl` against the primary candidate, separate from the child beads:

```
HTTP/1.1 200 OK
Last-Modified: Wed, 15 Jan 2025 01:39:19 GMT
Accept-Ranges: bytes
Content-Type: binary/octet-stream
Content-Length: 26364442        # == documented 26,364,442 B ✅

first 8 bytes:  4752 4942 0000 0002   # "GRIB", discipline=0, edition=2 ✅
.idx probe:     HTTP/1.1 200 OK        # variable-level subsetting available ✅
```

Reproducible:
```bash
URL='https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2'
curl -sI "$URL" | awk 'tolower($1)~/^(http|content-length|last-modified|accept-ranges)/{print}'
curl -s -r 0-7 "$URL" | xxd        # expect: 4752 4942 0000 0002
```

---

## Decoded grid + packing (excerpt — message 1)

Authoritatively via eccodes; full table in [bf-cmt1](bf-cmt1.md) / [bf-qa5j](bf-qa5j.md).

- **GDT 3.30** Lambert Conformal: Nx=614, Ny=428 (262,792 pts), La1=12.19°N,
  Lo1=226.541°E, LaD=25°N, LoV=265°E, Latin1/Latin2=25°N/25°N, Dx=Dy=12,191 m
  (12.191 km), shape=6, scan=0x40, centre=kwbc (NCEP).
- **DRT 3** (template 5.3): `packingType=grid_complex_spatial_differencing`,
  bitsPerValue=15, orderOfSpatialDifferencing=2, E=4, D=2.

---

## Provenance chain (audit trail)

| Bead | Contribution |
|---|---|
| [bf-63ow](bf-63ow.md) | Researched NOAA regional model archives |
| [bf-37db](bf-37db.md) | Located Lambert-conformal DRT=3 candidates |
| [bf-we8d](bf-we8d.md) | Extracted NAM awip12 URLs + metadata (size since corrected) |
| [bf-22pu](bf-22pu.md) | Verified URL accessibility + sizes (HTTP 200, magic, `.idx`) |
| [bf-2r5b](bf-2r5b.md) | Live-verified all candidate URLs + sizes vs `Content-Length` |
| [bf-qa5j](bf-qa5j.md) | eccodes-decoded all 1046 msgs — confirmed GDT 3.30 + DRT=3 |
| [bf-cmt1](bf-cmt1.md) | Compiled the above into the final provenance record |

Research doc: [docs/research/bf-2r5b-gdt330-drt3-candidate-files.md](../docs/research/bf-2r5b-gdt330-drt3-candidate-files.md)

**Date Completed:** 2026-07-22
**Verifier:** eccodes (`grib_get` / `grib_dump`) — ground truth; project decoder pending DRT=3
**Bead ID:** bf-5yoa
