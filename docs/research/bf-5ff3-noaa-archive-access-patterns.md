# NOAA Regional-Model Archive Access Patterns

**Bead:** bf-5ff3 — *Document NOAA archive access patterns*
**Date verified:** 2026-07-22
**Method:** all endpoints/buckets below were probed live on 2026-07-22 via anonymous
`curl` (HTTP code + directory/catalog XML). URLs marked **✓ verified** responded 200.

This doc consolidates **directory hierarchy, file-naming, and access methods** for the
NOAA regional models gribtract cares about (NAM, HRRR, RAP and neighbors), and — the
gap not covered by earlier beads — locates the **THREDDS catalog endpoints**.

> **Scope split with sibling docs** (do not duplicate here):
> - Raw URL patterns / placeholders for 8 models → [`bf-5gsm-noaa-url-patterns.md`](bf-5gsm-noaa-url-patterns.md)
> - Hosting platforms (NOMADS / NCEI / AWS / GCP / Planetary Computer) →
>   [`notes/bf-2p57.md`](../../notes/bf-2p57.md)
> - Per-model archive URLs + DRT/GDT status for the 10 GDT-3.30 candidates →
>   [`bf-1ot3-gdt330-candidate-archive-urls.md`](bf-1ot3-gdt330-candidate-archive-urls.md)
> - Which models use GDT 3.30 / DRT 3 → [`notes/bf-2z0x.md`](../../notes/bf-2z0x.md)

---

## 1. Two hierarchies, two storage shapes

NOAA regional-model GRIB2 lives in two flat-by-key stores with *different* shapes:

| | **NAM** | **HRRR** |
|---|---|---|
| Bucket | `noaa-nam-pds.s3.amazonaws.com` | `noaa-hrrr-bdp-pds.s3.amazonaws.com` |
| Key shape | `nam.{YYYYMMDD}/nam.t{CC}z.{product}{FF}.tm{TT}.grib2` | `hrrr.{YYYYMMDD}/{region}/hrrr.t{CC}z.{product}f{FF}.grib2` |
| Region subdir? | **No** — products are flat under the date dir | **Yes** — `conus/`, `alaska/` |
| Index sidecar | `{…}.grib2.idx` (wgrib2-style) | `{…}.grib2.idx` |
| Oldest date observed | `nam.20210916/` (2021-09-16) | (rolling long archive) |

Both are anonymous-public S3 — no auth, standard HTTPS GET, and **range-GET** is honored
so the `.idx` sidecar enables byte-range partial download of just the messages you want
(this is the access pattern gribtract's selective extractor targets).

---

## 2. File-name decoding

### NAM — `nam.t{CC}z.{product}{FF}.tm{TT}.grib2`

All fields **zero-padded**. **✓ verified** by enumerating `nam.20250115/` (t00z).

| Token | Meaning | Observed values |
|---|---|---|
| `t{CC}z` | model **cycle / run hour** (UTC) | `t00z t06z t12z t18z` (main 6-hourly); some products also `t03z t09z t15z t21z` |
| `{product}` | **grid / product family** | see table below |
| `{FF}` | **forecast hour** (2-digit) | hourly `00–36`, then **3-hourly** `39,42,…,84` (verified for `awip12`) |
| `tm{TT}` | time marker | `tm00` for standard output; `awip3d` carries `tm00`–`tm06` (10-min sub-hourly) |

**NAM product families** (verified from the `nam.t00z.*` listing of `nam.20250115/`):

| `{product}` | Grid / domain | Notes |
|---|---|---|
| `awip12` | **CONUS 12 km, Grid 218 — Lambert-conformal** | the **DRT=3 / GDT 3.30** grid (see bf-2z0x) |
| `awip20` / `awip32` | CONUS 20 km / 32 km | |
| `awipak` / `awiphi` | Alaska / Hawaii | |
| `afwaca` / `afwahi` | Alaska-Canada / Hawaii FIREwx AWIPS | 00–84 by 3 h |
| `awak3d` | Alaska 3-D | |
| `awp211` / `awp242` | legacy AWIPS grids 211 / 242 | older Lambert grids |
| `alaskanest.hiresf` | Alaska NEST hi-res forecast | 00–60 |
| `awip3d` | 3-D (sub-hourly `tm00`–`tm06`) | plus `_icwf` AWIPS-control variants |

> Within a date dir, GRIB2 files are **flat** (no region subdir); observation BUFR is
> sharded into `bufr.t{CC}z/`, `bufr_{nest}.t{CC}z/` subdirectories instead.

### HRRR — `hrrr.t{CC}z.{product}f{FF}.grib2` (+ `.idx`)

**✓ verified** by enumerating `hrrr.20250101/conus/hrrr.t00z.wrfsfcf00…f14`.

| Token | Meaning | Observed values |
|---|---|---|
| `t{CC}z` | model cycle (UTC) — **hourly** | `t00z … t23z` |
| `{product}` | field set | `wrfsfc` (surface+pressure), `wrfnat` (native), `wrfprs` (pressure levels), `wrfsubh` (sub-hourly) |
| `f{FF}` | forecast hour (**`f` prefix**, 2-digit) | `f00 … f48` (hourly; longer for some cycles) |
| `{region}` | directory | `conus/`, `alaska/` |

> NAM uses a bare 2-digit hour (`00`); HRRR prefixes it (`f00`). NAM suffixes the time
> marker (`.tm00`); HRRR does not. Getting these wrong is the #1 URL-construction bug —
> see bf-5gsm §"Common Errors".

---

## 3. THREDDS catalog endpoints  ✓ (the gap this bead fills)

Earlier beads documented **AWS S3** (anonymous GET/range-GET) and **NOMADS** (HTTP dir +
grib-filter CGI). Neither is a THREDDS server. The two genuine THREDDS Data Servers (TDS)
serving these regional models are below; **both verified live on 2026-07-22**.

### 3a. Unidata TDS — real-time operational catalogs ✓

Top catalog:
`https://thredds.ucar.edu/thredds/catalog/catalog.xml`
→ *Forecast Model Data* → `idd/forecastModels.xml`
→ one `catalogRef` per NCEP model/grid.

**Catalog URL template** (drill directly):
```
https://thredds.ucar.edu/thredds/catalog/grib/NCEP/<MODEL>/<GRID>/catalog.xml
```

**Regional models exposed** (verified `catalogRef` list):
- **HRRR** — `CONUS_2p5km` (Forecast) and `CONUS_2p5km_ANA` (Analysis)
- **NAM** — `CONUS_12km` (+ `CONUS_20/40/80km`, `Alaska_11/45/95km`, `Polar_90km`, `Firewxnest`)
- **RAP** (Rapid Refresh) — `CONUS_13km`, `CONUS_20km`, `CONUS_40km`
- SREF (CONUS 40 km ensembles), WW3 regional wave

Each leaf catalog exposes the standard TDS access services (**OPeNDAP**, **HTTPServer**,
**NetCDFSubset**) plus convenience "Full Collection (Reference/Forecast Time)" and "Best
Time Series" virtual datasets. Example — NAM CONUS 12 km resolves to real datasets and a
live HTML view:

```
catalog: https://thredds.ucar.edu/thredds/catalog/grib/NCEP/NAM/CONUS_12km/catalog.xml   ✓ HTTP 200
browse:  https://thredds.ucar.edu/thredds/catalog/grib/NCEP/NAM/CONUS_12km/catalog.html   ✓ HTTP 200
```

### 3b. NCEI TDS — long-term archive / reanalysis catalogs ✓

Top catalog: `https://www.ncei.noaa.gov/thredds/catalog/catalog.xml`
→ *Model* → `model/model.xml` (34 catalogRefs; **note lowercase `model/`, not `Model/`**).

**Regional-relevant entries** (verified `catalogRef` list of `model/model.xml`):
- **NAM** — North American Mesoscale (NMM) → `nam.xml`
- **NARR** — North American Regional Reanalysis → `narr.xml`
- **RAP / RUC** — Rapid Refresh / Rapid Update Cycle → `rucrap.xml`
- **NDFD** — National Digital Forecast Database → `ndfd.xml`
- (+ GFS, GEFS, CFS, NCEP/DoE Reanalysis II, etc.)

```
catalog index: https://www.ncei.noaa.gov/thredds/catalog/model/model.xml   ✓ HTTP 200
```

### 3c. NOMADS is *not* THREDDS (correction)

`nomads.ncep.noaa.gov` is an **HTTPS directory listing + grib-filter CGI**
(`filter_<model>.pl`), with a short (~10-day) operational retention. It does **not** host
a THREDDS/OPeNDAP catalog. (bf-2p57's summary table loosely grouped NOMADS under
"THREDDS/OPeNDAP" — that is inaccurate; THREDDS access for these models is via Unidata and
NCEI above.) For THREDDS/OPeNDAP on real-time NAM/HRRR/RAP, use the Unidata TDS.

---

## 4. Access-method summary

| Method | Where | Best for | Auth |
|---|---|---|---|
| **S3 GET / range-GET** | `noaa-{model}-pds.s3.amazonaws.com` | whole-file or `.idx`-guided partial fetch (gribtract's selective extractor); long archive | none |
| **S3 list (XML)** | `?list-type=2&prefix=…&delimiter=/` | enumerate date/cycle/product keys | none |
| **NOMADS HTTP dir + grib-filter** | `nomads.ncep.noaa.gov/data/…`, `filter_<model>.pl` | latest ~10 days, server-side variable/level subsetting | none |
| **THREDDS / OPeNDAP** | `thredds.ucar.edu`, `www.ncei.noaa.gov/thredds` | remote subsetting, netCDF/OPeNDAP clients (xarray, IDV) | none |

---

## 5. Reproducibility — how this was verified (2026-07-22)

```bash
# NAM: enumerate product families + confirm flat layout
curl -s "https://noaa-nam-pds.s3.amazonaws.com/?list-type=2&prefix=nam.20250115/nam.t00z.&delimiter=/&max-keys=1000"

# HRRR: confirm region subdir + f-prefixed forecast hours
curl -s "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/?list-type=2&prefix=hrrr.20250101/conus/hrrr.t00z.wrfsfcf&delimiter=/"

# THREDDS catalogs
curl -s "https://thredds.ucar.edu/thredds/catalog/idd/forecastModels.xml"            # Unidata regional-model refs
curl -s "https://thredds.ucar.edu/thredds/catalog/grib/NCEP/NAM/CONUS_12km/catalog.xml"
curl -s "https://www.ncei.noaa.gov/thredds/catalog/model/model.xml"                  # NCEI archive/reanalysis refs
```

## 6. Acceptance criteria — checklist

| Criterion | Result |
|---|:---:|
| File-naming pattern for ≥2 regional models | ✅ NAM (§2) + HRRR (§2), decoded with verified product lists |
| Directory hierarchy described per archive | ✅ §1 + §2 (NAM flat-under-date; HRRR region-subdir) |
| Located THREDDS catalogs / documentation | ✅ §3 — Unidata TDS + NCEI TDS, both verified; NOMADS correctly excluded |
