# bf-5ff3: Document NOAA archive access patterns

## Task
Document file-naming conventions, directory structures, and access patterns
(including THREDDS catalogs) for NOAA regional-model archives.

## Headline result
Filled the one gap the sibling beads left open: **the THREDDS catalog endpoints**.
Two genuine TDS instances serve these models, both verified live on 2026-07-22:

- **Unidata TDS (real-time):** `https://thredds.ucar.edu/thredds/catalog/grib/NCEP/<MODEL>/<GRID>/catalog.xml`
  → HRRR, NAM (many grids), RAP, SREF, WW3. Drilled to a real NAM CONUS 12 km catalog
  (HTTP 200) exposing OPeNDAP / HTTPServer / NetCDFSubset + "Best Time Series" dataset.
- **NCEI TDS (archive/reanalysis):** `https://www.ncei.noaa.gov/thredds/catalog/model/model.xml`
  (note **lowercase `model/`**) → NAM, NARR, RAP/RUC, NDFD + global models.

**Correction recorded:** NOMADS (`nomads.ncep.noaa.gov`) is **not** a THREDDS/OPeNDAP
server — it's an HTTPS directory + grib-filter CGI. bf-2p57's summary table grouped it
under THREDDS/OPeNDAP; that's inaccurate and the new doc flags it.

## Directory hierarchies (verified from live S3 XML listings)
- **NAM** — `noaa-nam-pds.s3.amazonaws.com/nam.{YYYYMMDD}/nam.t{CC}z.{product}{FF}.tm{TT}.grib2`
  — **flat** under the date dir (no region subdir); BUFR obs sharded into `bufr*.t{CC}z/`.
  Oldest date ~2021-09-16. `{product}` families verified: `awip12` (CONUS 12 km Lambert,
  the DRT=3 grid), `awip20/32`, `awipak/hi`, `afwaca/hi`, `awak3d`, `awp211/242`,
  `alaskanest.hiresf`, `awip3d` (sub-hourly `tm00`–`tm06`). Forecast hours hourly 00–36
  then 3-hourly to 84.
- **HRRR** — `noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.{YYYYMMDD}/{conus,alaska}/hrrr.t{CC}z.{product}f{FF}.grib2`
  — hourly cycle `t00z–t23z`, `f00–f48`, products `wrfsfc/wrfnat/wrfprs/wrfsubh`. `.idx`
  sidecars on both → range-GET partial fetch (the pattern gribtract targets).

## Naming gotcha
NAM = bare 2-digit hour + `.tm00` suffix (`awip1200.tm00`); HRRR = `f`-prefixed hour, no
time marker (`wrfsfcf00`). Confusing the two is the top URL-construction bug (bf-5gsm).

## Acceptance criteria — all met
| Criterion | Result |
|---|:---:|
| File-naming pattern for ≥2 regional models | ✅ NAM + HRRR (decoded, verified) |
| Directory hierarchy per archive | ✅ NAM flat-under-date; HRRR region-subdir |
| THREDDS catalogs / documentation located | ✅ Unidata TDS + NCEI TDS (both verified) |

## Files
- `docs/research/bf-5ff3-noaa-archive-access-patterns.md` — consolidated doc (scope-split
  vs bf-5gsm/bf-2p57/bf-1ot3; cross-linked, not duplicated)
- `notes/bf-5ff3.md` — this summary

## Related beads
[bf-2z0x](bf-2z0x.md) (blocking dep, GDT/DRT investigation),
[bf-5gsm](bf-5gsm.md) (URL patterns), [bf-1ot3](bf-1ot3.md) (GDT-3.30 archive URLs),
[bf-2p57](bf-2p57.md) (hosting platforms).

## Date completed
2026-07-22
