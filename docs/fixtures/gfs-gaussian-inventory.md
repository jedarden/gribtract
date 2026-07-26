# GFS Gaussian Fixture — Concrete Inventory

**Bead**: bf-5ydxrm
**Purpose**: Authoritative, empirically-verified facts about the GFS Gaussian fixture so the canonical doc stops carrying repeated guesses. Every template, file, dependency, and source below was checked against the actual bytes / build artifacts on 2026-07-26.
**Method**: `wgrib2`, eccodes `grib_ls`/`grib_dump`, the freshly-rebuilt `gribtract` binary, `sha256sum`, `git check-ignore`, and the Cargo manifests/lockfile.

---

## TL;DR — what the fixture actually is

The GFS Gaussian fixture is the CORe 3-hourly flux file **`flx.2024011500.grib2`** (manifest id `core_gaussian_gdt40`). It is a **T254 Gaussian grid** (GDT 3.40, 512×256 = 131 072 points), NCEP centre `kwbc` (7) / sub-centre 3.

**Two corrections to prior docs / this task's spec, both verified:**

1. **The fixture does not exercise PDT 4.8.** Its 104 fields are split across **PDT 4.2 (55 fields)** and **PDT 4.12 (49 fields)**. `gribtract` *does* implement PDT 4.8 (`parse_pdt_8`), but the Gaussian fixture contains no PDT-4.8 message. The task's "(PDT 4.8)" parenthetical is a guess that the bytes do not support.
2. **The fixture is not currently decodable end-to-end by `gribtract`.** `parse_section4` matches PDT 0/1/2/8/11 but has **no arm for PDT 4.12**, so it returns `Error::NotImplemented`. The high-level `gribtract::decode()` aborts on the first PDT-4.12 field and returns `Err("decode not implemented")` for the whole file. Verified empirically: `gribtract list flx…grib2` → `exit 1`, 0 fields; control files (GDT 0/PDT 0/DRT 0 and GDT 0/PDT 0/DRT 3) decode with `exit 0`. The earlier "✅ Supported" status in `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md` is inaccurate.

What *does* work for this fixture in isolation: **GDT 3.40 parsing** (`parse_gdt_40`) and **DRT 3 decoding** (`parse_drt_3` + `decode_drt3`). The block is solely the unimplemented PDT 4.12.

---

## 1. GRIB2 templates exercised vs. implemented

All template codes and packing types below are from eccodes `grib_ls` against the on-disk `flx.2024011500.grib2` (104 fields). Implementing functions are in [`crates/gribtract-core/src/decode.rs`](../../crates/gribtract-core/src/decode.rs).

### 1a. Grid Definition Template 3.40 — Gaussian Latitude/Longitude

- **Exercised by:** all 104 fields (100% of the fixture).
- **Implementing functions:**
  - `parse_section3` dispatch (GDT 40 arm) — `decode.rs:403`
  - `parse_gdt_40` — `decode.rs:628`
- **Status:** ✅ Implemented. Builds `GridProjection::GaussianLatLon(GaussianLatLonParams { n_parallels })`. GDT 3.40 differs from GDT 3.0 only in octets 68–71, which carry **N** (number of parallels pole→equator, raw u32) instead of Dj; `dj` is left `0.0`.

### 1b. Product Definition Templates

| PDT | Fields in fixture | Implementing function | Status |
|-----|------------------:|-----------------------|--------|
| 4.2 (Derived forecasts) | 55 | dispatched to `parse_pdt_0` at `decode.rs:692`; common header in `parse_pdt_common_header` at `decode.rs:711` | ✅ Implemented (common-header-only; tail octets not modelled) |
| 4.12 (Derived accumulation / time-processed) | 49 | **none** — no match arm in `parse_section4` (`decode.rs:680`); falls to `_ => Err(Error::NotImplemented)` at `decode.rs:703` | ❌ **Not implemented** — blocks whole-file decode |
| 4.8 (Statistical product) | 0 | `parse_pdt_8` at `decode.rs:798` (implementing `parse_section4` arm `decode.rs:695`) | ✅ Implemented, but **absent from this fixture** |

The 49 PDT-4.12 fields are the time-averaged fluxes (`avg_sdlwrf`, `sulwrf`, `avg_sdswrf`, `suswrf`, `duvb`, `cduvb`, `vbdsf`, `vddsf`, …). The 55 PDT-4.2 fields are the instantaneous/analysis fluxes (`DLWRF`, `ULWRF`, `DSWRF`, `USWRF`, `UGRD`, …) — all flagged `anl:ens mean` by wgrib2.

### 1c. Data Representation Templates

| DRT | Fields in fixture | eccodes `packingType` | Implementing functions | Status |
|-----|------------------:|-----------------------|------------------------|--------|
| 5.3 — complex packing **with spatial differencing** | 102 | `grid_complex_spatial_differencing` | `parse_section5` arm `decode.rs:862` → `parse_drt_3` `decode.rs:929` → decoded by `decode_section7` `decode.rs:1034` → `decode_drt3` `decode.rs:1160` (order-1 branch `:1304`, order-2 branch `:1315`) | ✅ Implemented |
| 5.2 — complex packing, **no** spatial differencing | 2 | `grid_complex` | `parse_section5` arm `decode.rs:858` → `parse_drt_2` `decode.rs:898` → decoded by the same `decode_drt3` via the `order == 0` branch `decode.rs:1299` | ✅ Implemented |

The 2 DRT-2 fields are the last two records of the file (wgrib2 offsets 10 814 238 and 10 889 671). Note `decode_drt3` handles **both** DRT 2 (`order_spatial_diff == 0`) and DRT 3 (`order` 1 or 2) — DRT 2 is routed there whenever `parse_section5` returns a `ComplexPackingExtra`.

### 1d. Public decode entry points (DRT 2/3)

- `decode_all_drt3` — `decode.rs:1559` — decode the full grid once (used for the decode-once-extract-many station pattern).
- `decode_point_drt3` — `decode.rs:1587` — single-point accessor (decodes the whole grid internally; random access is impossible for DRT 3 because each value depends on all prior values).
- (`decode_point_drt0` — `decode.rs:1528` — DRT 0 simple-packing point accessor; not exercised by this fixture but listed for completeness of the decode surface.)

---

## 2. Fixture files

### 2a. Primary Gaussian fixture — `core_gaussian_gdt40` (T254)

| Attribute | Value |
|-----------|-------|
| Manifest path | `large/flx.2024011500.grib2` → on disk at `tests/corpus/large/flx.2024011500.grib2` |
| Size (bytes) | 10 968 510 (≈ 10.5 MiB) |
| SHA-256 | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` |
| Storage type | **`remote`** (manifest-declared) — `tests/corpus/large/` is gitignored; fetched + sha256-verified by `cargo xtask corpus fetch` (see `xtask/src/corpus.rs`). The source URL is GRIB2 content served with a `.grb` extension. |
| On-disk present? | ✅ Yes — present and SHA-256 matches the manifest (verified 2026-07-26). |
| Git-tracked? | ❌ No — gitignored (`/tests/corpus/large/` and `*.grib2` in `.gitignore`). |

### 2b. T1534 Gaussian fixture — `gfs_gaussian_gdt40_t1534` (manifest only)

| Attribute | Value |
|-----------|-------|
| Manifest path | `large/gdas.t00z.sfluxgrbf000.grib2` → on disk at `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` |
| Declared size (bytes) | 127 659 863 (≈ 122 MiB) |
| Declared SHA-256 | `f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e` |
| Storage type | `remote` (manifest-declared) |
| On-disk present? | ⚠️ **Placeholder only** — a 0-byte file exists (SHA-256 `e3b0c44…`, the empty-file digest), which does **not** match the declared manifest hash. The real file has **not** been fetched; the declared grid/template facts below are unverified against bytes. |

Per-manifest grid facts (T1534, unverified on disk): GDT 3.40, 3072×1536 = 4 718 592 points, N = 768, La1/La2 = ±89.910324°, Lo1 = 0°, Lo2 = 359.882813°, ~0.117° (~12 km) resolution; 54 GRIB2 messages. Source: NOAA GDAS surface-flux.

### 2c. `crates/gribtract/fixtures/noaa-samples/` — NOT Gaussian (listed for completeness)

The task asked to list these paths, but **none of them is a Gaussian fixture**. Every file is a GFS `pgrb2` product on **GDT 0 (regular latitude/longitude)**, confirmed via eccodes (`gridDefinitionTemplateNumber = 0`, PDT 0, `grid_complex_spatial_differencing`). They are **gitignored** (match `*.grib2`; `git check-ignore` confirms) and are **not** in the corpus manifest — they are hand-curated ad-hoc samples, fetched neither by `xtask` nor sha256-pinned.

| File | Size (bytes) | SHA-256 | On disk? |
|------|-------------:|---------|----------|
| `gfs.20260722.t00z.pgrb2.1p00.f000.grib2` | 42 562 250 | `b99cac2ddaf46d94faf1f4c8650fcc6b9673a78a00adf88cff4541cb3ccbcae3` | ✅ |
| `gfs.20260723.t00z.pgrb2.1p00.f000.grib2` | 42 460 488 | `f102a97690fae28cdf6d5c979b1f079e3d2769a05fb49815255bbf464893257f` | ✅ |
| `gfs.20260723.t00z.pgrb2.1p00.f006.grib2` | 45 137 836 | `750752ba65d16ca84c4b03089540cbbae6da6c14a82cc8174353177dca89e12c` | ✅ |
| `gfs.t00z.pgrb2.0p50.f000.grib2` | 152 106 356 | `f2ccb6c8abaeee0a6b0e52f91a096ecdb3c3446384f27da63e5df7fccf3fc302` | ✅ (also mirrored to `tests/corpus/large/` as `gfs.t00z.pgrb2.0p50.f000`, manifest id `gfs_conus_drt0_0p50`) |
| `gfs.t00z.pgrb2.0p50.f003.grib2` | 160 707 640 | `fca55c818ff032d8a3388dc8d1eb78e32cffb2ebc357617e2d0c70d8520bbd06` | ✅ |
| `gfs.t00z.pgrb2.0p50.f006.grib2` | 161 232 968 | `dcce46dd0c5ad6892e067a2e31879335b8ce6c4934a6e8395c860c7a124ba93e` | ✅ |
| `gfs.t00z.pgrb2.1p00.f000.grib2` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | ⚠️ 0-byte placeholder |
| `gfs.t00z.pgrb2.1p00.f003.grib2` | 45 192 652 | `43fa5cf711c99b0f62d2c6693f69ea4bd6cca5afb0a25d20907015a4b8397341` | ✅ |
| `gfs.t00z.pgrb2.1p00.f006.grib2` | 45 332 253 | `d9026e2dbbc98c2f32cb61915b13839f315bd596434ab0726debd6373e494cd8` | ✅ |
| `gfs.t00z.pgrb2.1p00.f012.grib2` | 45 323 689 | `908e1b7dbb7e7d755809d418a45d7944c8e6bb1f23b1c634901b42b4ab41dcc9` | ✅ |

---

## 3. Crate dependencies and pinned versions

Decode-relevant dependencies. "Manifest" = the version spec in the `Cargo.toml`; "Locked" = the resolved version in `Cargo.lock` (the lockfile is gitignored, so this is the on-disk resolution at audit time).

### 3a. `crates/gribtract-core` (the GRIB2 section parser + template decoders)

| Dependency | Manifest spec | Locked (`Cargo.lock`) | Role in decoding |
|------------|---------------|------------------------|------------------|
| `png` | `0.18` | `0.18.1` | DRT 5.41 (PNG) unpacking — `decode_drt41` (`decode.rs:1113`). Not used by the Gaussian fixture. |
| `jpeg2k` | `0.10` (optional, `default-features = false, features = ["openjpeg-sys"]`) | `0.10.1` | DRT 5.40 (JPEG 2000) — `decode_drt40` (`decode.rs:1083`), behind the `jpeg2000` cargo feature. Not used by the Gaussian fixture. |
| `openjpeg-sys` | (transitive, via `jpeg2k`) | `1.0.12` | Native JPEG 2000 codec backing `jpeg2k`. |

> **Note:** GDT 3.40 parsing and DRT 3 decoding pull in **no third-party crates** — they are pure Rust over the raw `&[u8]` section bodies (the `Buf` reader, `unpack_n_bits`, `extract_group_windowed`, `read_sign_magnitude_be`). The Gaussian fixture exercises only this dependency-free path.

### 3b. Workspace dependencies (`Cargo.toml` `[workspace.dependencies]`)

| Dependency | Manifest spec | Locked | Used by |
|------------|---------------|--------|---------|
| `serde` | `1` (+ `derive`) | `1.0.228` | `gribtract` (field JSON) |
| `serde_json` | `1` | `1.0.150` | `gribtract` |
| `sha2` | `0.10` | `0.10.9` | `xtask` corpus sha256 verification |
| `hex` | `0.4` | `0.4.3` | (declared) |
| `pyo3` | `0.22` (+ `extension-module`) | `0.22.6` | `gribtract-py` (excluded from default build) |

### 3c. `crates/gribtract-fetch` (HTTP byte-range fetching — fetches the remote fixture)

| Dependency | Manifest spec | Locked | Role |
|------------|---------------|--------|------|
| `reqwest` | `0.12` (`rustls-tls`, `default-features = false`) | (0.12.x) | HTTP client for S3/GCS/NOMADS byte-range fetch |
| `thiserror` | `2` | — | Error types |
| `url` | `2` | — | URL parsing |
| `bytes` | `1` | — | Buffer type |
| `tokio` | `1` (`optional`) | — | `async` feature only |
| `chrono` | `0.4` (`optional`) | — | `probe` feature only |

> The **`xtask`** corpus fetcher (`xtask/src/corpus.rs`) does **not** use `gribtract-fetch`/`reqwest` — it uses the `ureq` crate directly (see `download()` at `corpus.rs:272`), verifies the downloaded file with `sha2` (`verify_sha256` at `corpus.rs:316`), and resolves the URL from the manifest `url` field or `GRIBTRACT_B2_ENDPOINT`/`GRIBTRACT_B2_BUCKET` env vars (`resolve_url` at `corpus.rs:252`).

---

## 4. Data source and download URL

- **Source:** NOAA **CORe** archive — the NWS/NCEP Climate Data Record of operational gridded fluxes, hosted on Google Cloud Storage (Open Data Dissemination / public, no auth). Originating centre `kwbc` = 7 (NCEP), sub-centre 3.
- **Download URL (T254 fixture):**
  `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
  - URL pattern: `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/YYYY/MM/flx.YYYYMMDDHH.grb`
  - The source object has a `.grb` extension but is **GRIB2 edition 2** content (the `gribtract` edition check passes; eccodes reads it as GRIB2). The corpus stores it as `flx.2024011500.grib2`.
- **T1534 fixture URL (manifest-declared, file not on disk):**
  `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2`
  - Source: NOAA GDAS surface-flux (operational NOMADS).
- **Coverage:** CORe archive spans 1950–present (3-hourly flux files `flx.YYYYMMDDHH`).

---

## 5. Verified header snapshot — `flx.2024011500.grib2` field 1

From eccodes `grib_ls` (authoritative):

| Key | Value |
|-----|-------|
| `centre` | `kwbc` (7 — NCEP) |
| `subCentre` | `3` |
| `discipline` | `0` (Meteorological) |
| `gridDefinitionTemplateNumber` | `40` (Gaussian Lat/Lon) |
| `productDefinitionTemplateNumber` | `2` (field 1); file also has PDT 12 |
| `N` | `128` (parallels pole→equator) → confirms **T254** |
| `latitudeOfFirstGridPointInDegrees` | `89.4629` |
| `latitudeOfLastGridPointInDegrees` | `-89.4629` |
| `longitudeOfFirstGridPointInDegrees` | `0` |
| `longitudeOfLastGridPointInDegrees` | `359.297` |
| `iDirectionIncrementInDegrees` | `0.703125` |
| `Ni` / `Nj` / `numberOfDataPoints` | `512` / `256` / `131072` |
| `packingType` (field 1) | `grid_complex_spatial_differencing` (DRT 5.3) |

Gaussian-grid T-number reference (from `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md`): T62→N62/192×128, **T254→N128/512×256 (this fixture)**, T574→N192/768×384, T1534→N768/3072×1536 (the `gdas` fixture).

---

## 6. Open items surfaced by this inventory

1. **PDT 4.12 unimplemented** (`decode.rs:703` has no arm). Adding it (likely dispatching to `parse_pdt_8`'s common-header path + a time-range skip, since PDT 4.12 is the derived/accumulation analogue) is the single change needed to make the Gaussian fixture decode end-to-end. Until then, the "✅ Supported" claims for `core_gaussian_gdt40` are wrong.
2. **No golden reference exists** for `core_gaussian_gdt40` — `tests/corpus/golden/` contains no `core_gaussian_gdt40.json`, so `diagnose_core_gaussian_gdt40` (in `crates/gribtract/tests/diagnose_gfs_gaussian.rs`) cannot pass its `golden::load_golden` step.
3. **`gdas.t00z.sfluxgrbf000.grib2` is a 0-byte placeholder** on disk; its declared SHA-256/size are unverified against bytes.
4. **`crates/gribtract/fixtures/noaa-samples/`** files are gitignored, unpinned, and not Gaussian (GDT 0). If they are meant to be part of the test corpus they should be moved under `tests/corpus/` and added to the manifest; otherwise they are scratch and could live in `~/scratch/`.

---

*Inventory generated 2026-07-26 for bead bf-5ydxrm. All file/template/dependency facts verified against the live workspace; corrections to prior docs noted inline.*
