# bf-2z0x: Investigate GRIB2 encoding in NOAA models

## Task

Determine which NOAA regional models use **GDT 3.30 (Lambert-conformal conic)**
and **DRT 3 (complex packing)**, with at least 2 sample files each from NAM and
HRRR and a note on variations across forecast hours / levels.

---

## ★ Headline answer

| Model (product) | GDT 3.30 (Lambert)? | DRT 3 (complex packing)? | Verdict |
|---|:---:|:---:|---|
| **NAM** — awip12 / awphys (CONUS 12 km, Grid 218) | **100 %** | **100 %** | Uses **both**, exclusively |
| **HRRR** — wrfsfc (CONUS 3 km) | **100 %** | **~82 % at F00 → ~94 % at F12** | Uses GDT 3.30 always; DRT 3 dominates, **DRT 0 for low-entropy fields** |

Both NAM and HRRR ship their primary CONUS grids as **Lambert-conformal (GDT 3.30)
complex packing with 2nd-order spatial differencing (DRT 3 / template 5.3)**. The
one nuance: **NAM is exclusively DRT 3; HRRR mixes DRT 3 (most fields) with DRT 0
(simple packing) for categorical / accumulated / sparse fields.**

> **Ground-truth tool:** all numbers below are from **eccodes 2.41.0**
> (`nix shell nixpkgs#eccodes`), the same oracle the project uses for golden
> generation. `gribtract-cli list` cannot yet read these files — DRT 3 unpacking
> is unimplemented and it fails with `buffer too short: need 0, got 262792`.
> eccodes `packingType` mapping: `grid_complex_spatial_differencing` = **DRT 3**
> (template 5.3); `grid_simple` = **DRT 0** (template 5.0).

---

## Evidence — per-file GDT / DRT distribution

GDT = `gridDefinitionTemplateNumber`; packing = `packingType`. Every line below
is `count  <GDT> <packingType>` from `grib_ls -p gridDefinitionTemplateNumber,packingType`.

### NAM (CONUS 12 km) — GDT 3.30 + DRT 3, 100 % of every message

| File | Cycle / F-hour | Msgs | GDT 30 | DRT 3 | DRT 0 |
|---|---|---:|---:|---:|---:|
| `nam_awip1200_20250115.grib2` | 00z / **F00** (winter) | 196 | 196 | **196** | 0 |
| `nam_awip1200_20240601.grib2` | 12z / **F00** (summer) | 196 | 196 | **196** | 0 |
| `nam_awip1206_20240601.grib2` | 12z / **F06** (summer) | 208 | 208 | **208** | 0 |
| `nam_awphys00_20250115.grib2` | 00z / **F00** (phys+prs) | 446 | 446 | **446** | 0 |
| **Total** | | **1046** | **1046/1046** | **1046/1046** | **0** |

No variation by forecast hour (F00 vs F06), season (Jan vs Jun), or product
(awip12 vs awphys): **every one of the 1046 NAM CONUS messages is GDT 3.30 + DRT 3.**
(Independently corroborated in [bf-qa5j](bf-qa5j.md).)

### HRRR (CONUS 3 km) — GDT 3.30 always; DRT is a mix

| File | F-hour | Msgs | GDT 30 | DRT 3 | DRT 0 |
|---|---|---:|---:|---:|---:|
| `hrrr.t00z.wrfsfcf00.grib2` | **F00** (analysis) | 170 | 170 | **140 (82.4 %)** | 30 (17.6 %) |
| `hrrr.t00z.wrfsfcf12.grib2` | **F12** | 173 | 173 | **163 (94.2 %)** | 10 (5.8 %) |

**GDT is uniformly 3.30.** DRT is mostly 3, but a minority of fields use **DRT 0
(simple packing)** — and the DRT-3 fraction *rises* with forecast hour
(82 % → 94 %), because the analysis hour (F00) carries extra low-entropy fields.

---

## Grid + packing detail (message 1, authoritative via eccodes)

| Key | NAM awip12 (Grid 218) | HRRR CONUS |
|---|---|---|
| `gridDefinitionTemplateNumber` | **30** (Lambert Conformal) | **30** (Lambert Conformal) |
| `gridType` | `lambert` | `lambert` |
| `Nx` × `Ny` | 614 × 428 = 262 792 pts | 1799 × 1059 = 1 905 141 pts |
| La1 (first lat) | 12.19 °N | 21.1381 °N |
| Lo1 (first lon) | 226.541 °E (133.459 °W) | 237.28 °E (122.72 °W) |
| LaD (std-parallel lat) | 25 °N | 38.5 °N |
| LoV (orient lon) | 265 °E (95 °W) | 262.5 °E (97.5 °W) |
| Latin1 / Latin2 | 25 °N / 25 °N | 38.5 °N / 38.5 °N |
| Dx = Dy (resolution) | 12 191 m (**12.191 km**) | 3 000 m (**3 km**) |
| `packingType` | `grid_complex_spatial_differencing` (**DRT 3**) | `grid_complex_spatial_differencing` (**DRT 3**) |
| `bitsPerValue` | 15 | 12 |

(For NAM, full packing scale factors — R, E=4, D=2, order=2 — are in
[bf-qa5j](bf-qa5j.md).)

---

## Variations — forecast hour / level / parameter / product

### 1. By forecast hour (HRRR)

DRT-3 share grows from F00 (82 %) to F12 (94 %). The *kinds* of DRT-0 fields are
stable across hours; only their count varies.

### 2. By parameter / level — *which* HRRR fields use DRT 0

The ~30 simple-packed (DRT 0) messages in HRRR F00 are not arbitrary — they are
exactly the **low-entropy / categorical / accumulated / sparse** fields where
complex packing buys nothing. Authoritatively via
`grib_ls -p packingType,shortName,typeOfLevel,level`:

| shortName | typeOfLevel | why simple-packed |
|---|---|---|
| `csnow` `cicep` `cfrzr` `crain` | surface | **categorical** (0/1 precip-type flags) |
| `cpofp` | surface | percent frozen (few distinct values) |
| `tp` `prate` `sdwe` `ssrun` `bgrun` | surface | accumulated / rate / sparse |
| `10u` `10v` | heightAboveGround 10 m | 10 m wind (lower resolution need) |
| `ltng` | atmosphere | lightning (very sparse) |
| `hail` | surface | hail (sparse) |
| `max_vo` | heightAboveGroundLayer 1–2 km | time-max vorticity |
| `vucsh` `vvcsh` | heightAboveGroundLayer | vertical shear (layer) |
| `aod` | atmosphereSingleLayer | aerosol optical depth |
| `gh` | cloudBase | cloud-base height |

Everything continuous and smooth aloft (T, RH, geopotential, u/v wind on pressure
levels, reflectivity, visibility, radiation fluxes, CAPE/CIN, precipitable water)
is **DRT 3**. The split is a packing-efficiency decision by NCEP, not a model split.

### 3. By product — NAM is *not* uniformly Lambert

A second NAM product, `nam.t00z.afwaca00.tm00.grib2` (758 messages), is
**`gridType=regular_ll`, GDT 0 (lat/lon), 370×278** — but still **DRT 3** packing
(758/758). So "NAM uses GDT 3.30" is true for the CONUS physics grid (awip12 /
awphys / Grid 218) but **not for every NAM product**; the afwaca product is a
regular lat/lon grid. DRT 3, by contrast, holds across both NAM grid types.

---

## Caveats

- eccodes is the verifier, not the project's own decoder (DRT 3 unpacking still
  unimplemented in `gribtract-core`; that work is the point of these fixtures).
- Acceptance scope is NAM + HRRR. Other NOAA regional models (RAP, HRRR-Alaska /
  NEST, RTMA, HiResWindow) were not sampled here but RAP/HRRR-family are expected
  to share HRRR's Lambert + DRT-3-dominant profile.
- Sample files staged under `~/scratch/bf-2z0x/samples/` (HRRR) and
  `~/scratch/grib-verify/` (NAM) — not committed (tens to hundreds of MiB);
  reproducible below.

---

## Acceptance criteria — checklist

| Criterion | Result |
|---|:---:|
| Tested ≥2 sample files from NAM and HRRR | ✅ NAM 4 files (1046 msgs) / HRRR 2 files (343 msgs) |
| Recorded GDT + DRT per model | ✅ tables above |
| Noted variations between forecast hours / levels | ✅ HRRR F00→F12 DRT-3 share 82 %→94 %; DRT-0 field list; NAM afwaca GDT-0 |

---

## Reproducibility

```bash
EC() { nix shell nixpkgs#eccodes -c "$@"; }

# NAM CONUS 12 km (awip12 / awphys) — Lambert + DRT 3
for d in 20250115 20240601; do
  for prod in awip1200 awip1206 awphys00; do
    url="https://noaa-nam-pds.s3.amazonaws.com/nam.$d/nam.t00z.${prod}.tm00.grib2"
    curl -sI "$url" >/dev/null && echo "== $prod $d ==" &&
    EC grib_ls -p gridDefinitionTemplateNumber,packingType "$url" 2>/dev/null \
      | grep -vE '^(gridDef|/|.*messages)' | sort | uniq -c | sort -rn
  done
done

# HRRR CONUS 3 km (wrfsfc) — Lambert, DRT 3 dominant + DRT 0 minority
for fh in f00 f12; do
  url="https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250101/conus/hrrr.t00z.wrfsfc${fh}.grib2"
  echo "== wrfsfc${fh} ==" &&
  EC grib_ls -p gridDefinitionTemplateNumber,packingType "$url" 2>/dev/null \
    | grep -vE '^(gridDef|/|.*messages)' | sort | uniq -c | sort -rn
done
```
