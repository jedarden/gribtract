# bf-qa5j: Verify GDT 3.30 + DRT=3 in Candidate GRIB2 Files

## Task Summary

Downloaded the candidate GRIB2 files located in [bf-2r5b](bf-2r5b.md) and
**authoritatively verified** — using the **eccodes** decoder (`grib_get` /
`grib_dump`, the same tool the project uses for golden generation, installed via
`nix shell nixpkgs#eccodes`) — that every message uses **Grid Definition Template
3.30 (Lambert Conformal Conic)** and **Data Representation Template 3 (complex
packing with spatial differencing)**.

An independent tool was required because the project's own decoder cannot yet
read these files: `gribtract-cli list` fails on them with
`error: buffer too short: need 0, got 262792` (DRT=3 unpacking is not yet
implemented — see the `gfs_tmp2m_1deg_anl` fixture note in
[tests/corpus/manifest.json](../tests/corpus/manifest.json)). So eccodes is used
as the ground-truth verifier.

---

## Result: PASS — all candidates confirmed GDT 3.30 + DRT=3

| File | Messages | GDT=30 | DRT=3 | GDT/DRT distribution |
|---|---:|---:|---:|---|
| `nam.t00z.awip1200.tm00.grib2` (2025-01-15) | 196 | **196/196** | **196/196** | GDT {30}, DRT {3} |
| `nam.t12z.awip1200.tm00.grib2` (2024-06-01) | 196 | **196/196** | **196/196** | GDT {30}, DRT {3} |
| `nam.t12z.awip1206.tm00.grib2` (2024-06-01) | 208 | **208/208** | **208/208** | GDT {30}, DRT {3} |
| `nam.t00z.awphys00.tm00.grib2` (2025-01-15) | 446 | **446/446** | **446/446** | GDT {30}, DRT {3} |
| **Total** | **1046** | **1046/1046** | **1046/1046** | — |

**Every one of the 1046 messages across all four files is GDT 3.30 + DRT=3.**
No file failed the criteria; nothing excluded. The HRRR `wrfsfc` candidates from
[bf-2r5b](bf-2r5b.md) were not downloaded here — the four NAM files already
provide four independent confirmations at one-quarter to one-fifth the byte cost
(26–51 MiB vs 124–137 MiB).

---

## Download verification (sizes match `Content-Length` exactly)

| File | Expected (B) | Downloaded (B) | Match |
|---|---:|---:|:---:|
| `nam_awip1200_20250115.grib2` | 26,364,442 | 26,364,442 | ✅ |
| `nam_awip1200_20240601.grib2` | 28,800,129 | 28,800,129 | ✅ |
| `nam_awip1206_20240601.grib2` | 32,565,263 | 32,565,263 | ✅ |
| `nam_awphys00_20250115.grib2` | 52,938,055 | 52,938,055 | ✅ |

Files staged at `~/scratch/grib-verify/` (not committed — too large for the
corpus; reproducible via the `curl` commands in [bf-2r5b](bf-2r5b.md)).

---

## Grid parameters — NAM Grid 218 (Lambert Conformal, message 1)

Extracted authoritatively via `grib_get` / `grib_dump -j` (eccodes). The
`awip1200` and `awphys00` files share the identical grid (verified: same
GDT/Nx/Ny/La1/Lo1).

| Parameter (GRIB2 key) | Value | Meaning |
|---|---|---|
| `gridDefinitionTemplateNumber` | **30** | Lambert Conformal Conic |
| `gridType` | `lambert` | — |
| `shapeOfTheEarth` | 6 | Earth assumed spherical, R = 6 371 229 m |
| `Nx` (Ni) | 614 | points along a parallel |
| `Ny` (Nj) | 428 | points along a meridian |
| `numberOfDataPoints` | 262 792 | = 614 × 428 |
| `latitudeOfFirstGridPointInDegrees` (La1) | 12.19 °N | first grid point lat |
| `longitudeOfFirstGridPointInDegrees` (Lo1) | 226.541 °E (= 133.459 °W) | first grid point lon |
| `LaDInDegrees` (LaD) | 25 °N | latitude where Dy is specified |
| `LoVInDegrees` (LoV) | 265 °E (= 95 °W) | grid orientation longitude |
| `Latin1InDegrees` / `Latin2InDegrees` | 25 °N / 25 °N | standard parallels |
| `DxInMetres` / `DyInMetres` | 12 191 m / 12 191 m | **12.191 km** resolution |
| scanning mode byte | `0x40` | +i, +j (j scans positively); `iScansNegatively`=0, `jPointsAreConsecutive`=0, `alternativeRowScanning`=0 |
| `centre` / `subCentre` | `kwbc` (7) / 0 | NCEP |

> **La2 / Lo2 (last grid point):** eccodes does **not** expose
> `latitudeOfLastGridPoint` / `longitudeOfLastGridPoint` for Lambert Conformal
> (3.30) — confirmed `<absent>` in `grib_dump -j` (first-grid-point keys are
> present, last-grid-point keys are not). Per the published NCEP Grid 218
> definition the last grid point is **La2 ≈ 47.442 °N, Lo2 ≈ 357.476 °E
> (2.524 °W)**; flagged as documented rather than file-derived because eccodes
> declines to compute it for this projection.

## Packing detail — DRT 3 (5.3), message 1

| Key | Value |
|---|---|
| `dataRepresentationTemplateNumber` | **3** (template 5.3) |
| `packingType` | `grid_complex_spatial_differencing` |
| `bitsPerValue` | 15 |
| `orderOfSpatialDifferencing` | 2 (second-order) |
| `referenceValue` (R) | 9.94887 × 10⁶ |
| `binaryScaleFactor` (E) | 4 |
| `decimalScaleFactor` (D) | 2 |

`packingType = grid_complex_spatial_differencing` is eccodes' name for GRIB2
template 5.3, confirming DRT=3 is complex packing *with* spatial differencing
(as opposed to 5.2, complex packing without it).

---

## Acceptance Criteria

- ✅ **At least one file verified GDT 3.30** — all 4 files, 1046/1046 messages.
- ✅ **At least one file verified DRT=3** — all 4 files, 1046/1046 messages.
- ✅ **Grid parameters documented** — La1/Lo1/LaD/LoV/Latin1/Latin2/Dx/Dy/Nx/Ny + shape + scanning mode.
- ✅ **Files not meeting criteria noted/excluded** — none; 4/4 pass.

## Reproducibility

```bash
# Inspector (independent of gribtract's own decoder):
nix shell nixpkgs#eccodes -c grib_get \
  -p count,gridDefinitionTemplateNumber,dataRepresentationTemplateNumber \
  <file.grib2>                       # one line per message; all 30 / 3

# Cross-check (expected to FAIL today — DRT=3 not yet implemented):
cargo run -p gribtract-cli -- list <file.grib2>
# -> gribtract: error: buffer too short: need 0, got 262792
```

## Related

- [bf-2r5b](bf-2r5b.md) — located + accessibility-verified these candidate URLs (blocker)
- [bf-22pu](bf-22pu.md), [bf-we8d](bf-we8d.md) — upstream URL/size verification
- Research doc: [docs/research/bf-2r5b-gdt330-drt3-candidate-files.md](../docs/research/bf-2r5b-gdt330-drt3-candidate-files.md)

**Date Completed:** 2026-07-22
**Verifier:** eccodes (`grib_get` / `grib_dump -j`) via NixOS `nix shell nixpkgs#eccodes`
**Bead ID:** bf-qa5j
