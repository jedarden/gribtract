# GFS Gaussian Fixture — Canonical Reference

> **Single source of truth** for the GFS Gaussian fixtures in the gribtract test corpus. This
> document supersedes and replaces the former `gfs-fixtures-complete-reference.md`,
> `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md`, `gfs-gaussian-grid-structure.md`,
> `gfs-gaussian-build-status.md`, `gfs-gaussian-inventory.md`, and
> `gfs-gaussian-convention-audit.md`.
>
> **Provenance.** Consolidated by bead **bf-4yvt7s** (2026-07-26) from three empirically-verified
> sibling artifacts — build/test status (bead `bf-4swew5`, [note](../../notes/bf-4swew5.md)),
> template/dependency inventory (bead `bf-5ydxrm`), and project-convention audit (bead `bf-10ncjt`)
> — plus the structural/schema background in the older detail docs. Every status, template, file,
> and dependency statement below was checked against the live workspace at HEAD `b8661c5` on
> 2026-07-26 (cargo/rustc **1.96.1**). The PDT-dispatch root cause was re-confirmed against
> `crates/gribtract-core/src/decode.rs` during consolidation.
>
> **⚠️ Status correction.** Earlier docs marked these fixtures "✅ Supported" / "verified working"
> and claimed PDT 4.8 support and a committed golden file. Those claims are **inaccurate**; see
> §2 and §3 for the verified ground truth.

---

## TL;DR — status at a glance

| Aspect | Value | Status |
|--------|-------|--------|
| Fixture | `core_gaussian_gdt40` — `flx.2024011500.grib2` (CORe T254 flux) | ✅ Present, sha256-verified |
| Grid | GDT 3.40 Gaussian, 512×256 (T254, N=128), 131 072 pts | ✅ Parsed (`parse_gdt_40`) |
| Packing | DRT 5.3 (102 fields) + DRT 5.2 (2 fields) | ✅ Decoded (`decode_drt3`) |
| Product templates | PDT 4.2 (55 fields) ✅ · **PDT 4.12 (49 fields)** ❌ | ⚠️ Partial |
| **`cargo build -p gribtract`** | exit 0, 3 warnings | ✅ **PASS** |
| **`diagnose_gfs_gaussian` test** | panics `golden loaded` (line 13) | ❌ **FAIL** |
| **`gribtract::decode()` end-to-end** | `Err("decode not implemented")`, 0 fields | ❌ **FAIL** |

**Bottom line.** The workspace builds cleanly. The GFS Gaussian fixture **does not yet decode
end-to-end** — 49 of its 104 fields use **Product Definition Template 4.12**, which `parse_section4`
has no arm for, so `decode()` aborts on the first PDT-4.12 field and returns `Err`. The integration
test fails one step earlier: there is **no golden reference file** for this fixture, so it panics
loading the golden before ever calling `decode`. Implementing PDT 4.12 (§4) is the single change
needed to make the fixture decode.

---

## 1. The fixtures

### 1a. `core_gaussian_gdt40` — primary T254 Gaussian fixture (on disk, verified)

| Attribute | Value |
|-----------|-------|
| Manifest path | `large/flx.2024011500.grib2` → on disk at `tests/corpus/large/flx.2024011500.grib2` |
| Size | 10 968 510 bytes (≈ 10.5 MiB) |
| SHA-256 | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` (matches manifest) |
| Storage | **`remote`** — `tests/corpus/large/` is gitignored; fetched + sha256-verified by `cargo xtask corpus fetch` |
| Grid | GDT 3.40, 512×256, N=128, **T254**, 131 072 points |
| Latitude range | +89.4629° to −89.4629° (Gaussian quadrature spacing) |
| Longitude range | 0° to 359.297° (uniform, di = 0.703125°) |
| Messages | 104 GRIB2 fields |
| Centre / sub-centre | `kwbc` = 7 (NCEP) / 3 |
| Discipline | 0 (Meteorological) |
| Coverage | CORe 3-hourly flux — radiative, heat, land-surface, soil, cloud-layer fields |

### 1b. `gfs_gaussian_gdt40_t1534` — T1534 Gaussian fixture (manifest only, NOT on disk)

| Attribute | Value |
|-----------|-------|
| Manifest path | `large/gdas.t00z.sfluxgrbf000.grib2` → `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` |
| Declared size | 127 659 863 bytes (≈ 122 MiB) |
| Declared SHA-256 | `f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e` |
| On-disk present? | ⚠️ **Placeholder only** — a 0-byte file (SHA-256 `e3b0c44…`, the empty digest). The real file has never been fetched; declared grid facts below are **unverified against bytes**. |
| Declared grid (per manifest) | GDT 3.40, 3072×1536, N=768, **T1534**, 4 718 592 points; La1/La2 = ±89.910324°, Lo1 = 0°, Lo2 = 359.882813°, ~0.117° (~12 km); 54 messages. Source: NOAA GDAS surface-flux. |

### 1c. Data source and download URLs

- **T254 fixture (verified):** NOAA **CORe** archive (NWS/NCEP Climate Data Record of operational
  gridded fluxes, on Google Cloud Storage — public, no auth).
  - `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
  - Pattern: `…/grib/3hour/flx/YYYY/MM/flx.YYYYMMDDHH.grb`
  - The source object has a `.grb` extension but is **GRIB2 edition 2** content. The corpus stores it as `.grib2`.
- **T1534 fixture (manifest-declared):** NOAA GDAS surface-flux via NOMADS.
  - `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2`
- **CORe coverage:** 1950–present (3-hourly flux files).

---

---

## 2. Build & test status — ground truth *(from bf-4swew5)*

Every command below was executed fresh during verification; outputs are quoted verbatim.

### 2a. Toolchain
```
cargo 1.96.1 (356927216 2026-06-26)
rustc 1.96.1 (31fca3adb 2026-06-26)
```

### 2b. Build — **PASS (exit 0)**
`cargo build -p gribtract` finishes with 3 stable warnings (order non-deterministic):
1. `crates/gribtract/Cargo.toml` — `default-features` ignored for `gribtract-core`.
2. `crates/gribtract-cli/Cargo.toml` — `default-features` ignored for `gribtract`.
3. `crates/gribtract-core/src/decode.rs:1184:73` — unused variable `context`.

### 2c. Test — **FAIL (exit 101)**
```
thread 'diagnose_core_gaussian_gdt40' panicked at crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10:
golden loaded
test result: FAILED. 0 passed; 1 failed; …
```
Line 13 is the `.expect("golden loaded")` on the loaded golden reference. The golden file
`tests/corpus/golden/core_gaussian_gdt40.json` **does not exist** (the golden dir contains only 8
files, all for `small/` inline fixtures — none for any `large/` fixture), so `golden::load_golden`
returns `Ok(None)` and the test panics. `gribtract::decode` is **never reached** by this test.

### 2d. Direct decode probe — **FAIL**
Decoding the fixture bytes directly (default features and `--features jpeg2000`, identical result):
```
[probe] id=core_gaussian_gdt40 bytes=10968510
[probe] DECODE_FAILED: decode not implemented
```
`gribtract::decode` returns `Err` whose `Display` is `decode not implemented` (`Error::NotImplemented`).
The `jpeg2000` feature has no effect here — it gates DRT 5.40, which this file does not use.

---

## 3. GRIB2 templates — exercised vs. implemented *(from bf-5ydxrm)*

All template codes are from eccodes `grib_ls` against the on-disk `flx.2024011500.grib2` (104 fields).
Implementing functions are in [`crates/gribtract-core/src/decode.rs`](../../crates/gribtract-core/src/decode.rs).

### 3a. Grid Definition Template 3.40 — Gaussian Latitude/Longitude — ✅ implemented
- Exercised by **all 104 fields** (100% of the fixture).
- `parse_section3` GDT-40 arm (`decode.rs:403`) → `parse_gdt_40` (`decode.rs:628`).
- Builds `GridProjection::GaussianLatLon(GaussianLatLonParams { n_parallels })`. GDT 3.40 differs
  from GDT 3.0 only in octets 68–71, which carry **N** (parallels pole→equator, raw u32) instead of
  Dj; `dj` is left `0.0`.

### 3b. Product Definition Templates — ⚠️ partial (the blocker)

| PDT | Fields in fixture | Implementing function | Status |
|-----|------------------:|-----------------------|--------|
| 4.2 (Derived forecasts) | 55 | dispatched to `parse_pdt_0` at `decode.rs:692`; common header via `parse_pdt_common_header` at `decode.rs:711` | ✅ Implemented (common-header only; tail octets not modelled) |
| **4.12 (Derived accumulation / time-processed)** | **49** | **none** — no match arm in `parse_section4` (`decode.rs:680`); falls to `_ => Err(Error::NotImplemented)` at `decode.rs:703` | ❌ **Not implemented — blocks whole-file decode** |
| 4.8 (Statistical product) | 0 | `parse_pdt_8` at `decode.rs:798` (arm at `decode.rs:695`) | ✅ Implemented, but **absent from this fixture** |

> **Correction to earlier docs / the original task spec:** the fixture does **not** exercise PDT 4.8.
> Its fields split across **PDT 4.2 (55)** and **PDT 4.12 (49)**. The earlier "PDT 4.8 /
> `parse_pdt_48()`" claims (and the "✅ Supported" status) are wrong. The 49 PDT-4.12 fields are the
> time-averaged fluxes (`avg_sdlwrf`, `sulwrf`, `avg_sdswrf`, `suswrf`, `duvb`, `cduvb`, `vbdsf`,
> `vddsf`, …); the 55 PDT-4.2 fields are the instantaneous/analysis fluxes (all flagged `anl:ens
> mean` by wgrib2).

`parse_section4` (`decode.rs:672`) dispatch confirmed against source during consolidation:

```text
decode.rs:680     match template {
decode.rs:681         0  => parse_pdt_0(…)
decode.rs:685         1  => parse_pdt_1(…)
decode.rs:689         2  => parse_pdt_0(…)      // PDT 4.2 reuses the 4.0 parser
decode.rs:695         8  => parse_pdt_8(…)
decode.rs:699        11  => parse_pdt_11(…)
decode.rs:703         _  => Err(Error::NotImplemented)   // ← PDT 4.12 lands here
```

`decode_bytes` bails on the first error via `?` (`decode.rs:214`), so the whole `decode()` returns
`Err` with **zero fields decoded**. Field 56 is the first PDT-4.12 field.

### 3c. Data Representation Templates — ✅ implemented

| DRT | Fields | eccodes `packingType` | Implementing path | Status |
|-----|-------:|-----------------------|-------------------|--------|
| 5.3 — complex packing **with spatial differencing** | 102 | `grid_complex_spatial_differencing` | `parse_section5` arm `decode.rs:862` → `parse_drt_3` `decode.rs:929` → `decode_section7` `decode.rs:1034` → `decode_drt3` `decode.rs:1160` (order-1 `:1304`, order-2 `:1315`) | ✅ |
| 5.2 — complex packing, **no** spatial differencing | 2 | `grid_complex` | `parse_section5` arm `decode.rs:858` → `parse_drt_2` `decode.rs:898` → same `decode_drt3` via `order == 0` branch `decode.rs:1299` | ✅ |

The 2 DRT-2 fields are the file's last two records. `decode_drt3` handles both DRT 2 (`order == 0`)
and DRT 3 (`order` 1 or 2). Section 6 bitmap indicator is 255 (no bitmap) throughout.

### 3d. Decode entry points (DRT 0/2/3)
- `decode_all_drt3` — `decode.rs:1559` — decode the full grid once (decode-once-extract-many pattern).
- `decode_point_drt3` — `decode.rs:1587` — single-point accessor (decodes the whole grid internally; random access is impossible for DRT 3).
- `decode_point_drt0` — `decode.rs:1528` — DRT 0 simple-packing point accessor (not exercised by this fixture).

---

## 4. Root cause & remaining integration work

The fixture is **not blocked by its grid or packing** — GDT 3.40 parsing and DRT 2/3 decoding both
work. The sole end-to-end blocker is the unimplemented **PDT 4.12**. Two independent gaps stand
between the current state and a passing `diagnose_core_gaussian_gdt40` test:

1. **No PDT 4.12 support** (`decode.rs:703` has no arm). PDT 4.12 is the derived/accumulation
   analogue; it likely dispatches through `parse_pdt_8`'s common-header path plus a time-range skip.
   Until added, `gribtract::decode()` returns `Err` for the whole file.
2. **No golden reference** for `core_gaussian_gdt40`. `tests/corpus/golden/` contains no
   `core_gaussian_gdt40.json`, so the diagnostic test cannot pass its `golden::load_golden` step.
   (This is consistent with the corpus-wide rule — only `small/` inline fixtures carry committed
   goldens; **every** `large/` fixture lacks one. A golden for this fixture should be generated
   only once PDT 4.12 is decodable.)

### Remaining work (roadmap)

| # | Task | Blocks | Severity |
|---|------|--------|----------|
| 1 | Implement PDT 4.12 in `parse_section4` (`decode.rs:680`) | whole-file decode | **Critical** |
| 2 | Generate + commit `tests/corpus/golden/core_gaussian_gdt40.json` (via `scripts/gen_golden.py` / eccodes) once #1 lands | `diagnose_core_gaussian_gdt40` test | High |
| 3 | Fetch + sha256-verify the T1534 `gdas.t00z.sfluxgrbf000.grib2` (currently a 0-byte placeholder); confirm its declared grid/template facts against bytes | T1534 coverage | Medium |
| 4 | Add the `diagnose_gfs_gaussian` test to CI once #1–#2 pass | regression coverage | Low |
| 5 | Model the PDT 4.2 / 4.12 tail octets (currently common-header only) | full metadata fidelity | Low |

> Earlier integration roadmaps (the superseded `notes/bf-658687-*.md`) attributed the failure to
> GDT 3.40 / N-parameter / DRT-3 handling. That diagnosis was wrong — those paths work. The real
> cause is the PDT-4.12 gap above.

---

## 5. Data structure schema (golden JSON)

Golden reference fixtures are JSON generated by `scripts/gen_golden.py` from source GRIB2 via
eccodes (`grib_dump -j -d`, preferred) or the eccodes Python bindings.

### 5a. Top-level structure
```json
{
  "fixture_id": "core_gaussian_gdt40",
  "_provenance": "Generated by scripts/gen_golden.py from …",
  "fields": [ { …one object per GRIB2 field… } ]
}
```

### 5b. Field object — Gaussian (GDT 3.40)
```json
{
  "center": 7,
  "subcenter": 3,
  "gdt_template": 40,
  "pdt_template": 2,
  "drt_template": 3,
  "parameter": { "discipline": 0, "category": 5, "number": 3 },
  "forecast": { "reference_time": { … }, "time_range_unit": 1, "forecast_offset": 0 },
  "level":    { "type1": 1, "scale_factor1": 0, "scaled_value1": 0, "type2": 255 },
  "ensemble": null,
  "grid": {
    "template": 40,
    "num_data_points": 131072,
    "nx": 512, "ny": 256,
    "lat_first": 89.4629, "lon_first": 0,
    "lat_last": -89.4629, "lon_last": 359.297,
    "di": 0.703125, "dj": null,
    "scanning_mode": 0, "resolution_flags": 48, "shape_of_earth": 6
  },
  "packing": {
    "reference_value": 270.0, "binary_scale_factor": 0,
    "decimal_scale_factor": 0, "bits_per_value": 8, "original_field_type": 0
  },
  "values": { "Dense": [170.819, 170.827, 170.842, …] }
}
```

### 5c. `grid` field reference (GDT 3.40)

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `template` | int | Grid Definition Template (40 = Gaussian Lat/Lon) | 40 |
| `num_data_points` | int | Total grid points (nx × ny) | 131072 |
| `nx` | int | Longitude points (uniform) | 512 |
| `ny` | int | Latitude points (Gaussian spacing) | 256 |
| `lat_first` / `lat_last` | float | First / last latitude (±90) | 89.4629 / −89.4629 |
| `lon_first` / `lon_last` | float | First / last longitude (0–360) | 0 / 359.297 |
| `di` | float | Longitude increment (uniform) | 0.703125 |
| `dj` | float | **null** for Gaussian — computed from N | null |
| `scanning_mode` | int | Scan direction bit field (0 = +i, −j) | 0 |
| `resolution_flags` | int | Resolution bit flags | 48 |
| `shape_of_earth` | int | Earth shape (6 = WGS84) | 6 |

### 5d. DRT 0 simple-packing formula
`unpacked_value = (R + (packed_value × 2^E)) / 10^D` where R = `reference_value`,
E = `binary_scale_factor`, D = `decimal_scale_factor`.

---

## 6. Gaussian grid parameters

### 6a. T-number resolution table

| T-Number | N (parallels) | nx | ny | Points | Approx resolution |
|----------|---------------|----|----|--------|-------------------|
| T62 | 62 | 192 | 128 | 24 576 | ~1.875° |
| **T254** | **128** | **512** | **256** | **131 072** | **~0.703° (~70 km) — `core_gaussian_gdt40`** |
| T574 | 192 | 768 | 384 | 294 912 | ~0.469° |
| **T1534** | **768** | **3072** | **1536** | **4 718 592** | **~0.117° (~12 km) — `gfs_gaussian_gdt40_t1534`** |

- **N**: number of Gaussian latitudes from equator to pole. **Total latitudes = 2N** (pole to pole).
- **Longitude points**: typically `4N`.
- **Resolution**: ~`360° / nx`.

### 6b. Why Gaussian grids
- **Non-uniform latitude spacing** — latitudes are the roots of the Legendre polynomial P_N(sin φ),
  giving exact numerical integration for spherical harmonics.
- **Spectral-model compatibility** — used by FV3GFS, ECMWF IFS.
- **Pole-to-pole coverage** — extends nearly to the poles (ε ≈ 0.5° for T254), avoiding singularities.

### 6c. Implementation approximation
`GaussianLatLonParams::nearest_index` (in `crates/gribtract-core/src/types.rs`) approximates latitude
spacing as **linear** between `lat_first` and `lat_last` for nearest-point queries. This is exact at
grid corners and reasonable elsewhere. True Gaussian-quadrature placement (computing Legendre zeros)
is a valid future optimization.

```rust
/// Gaussian Latitude/Longitude grid parameters (GDT 3.40)
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianLatLonParams {
    /// N — number of parallels between the Pole and the Equator.
    pub n_parallels: u32,
}
```

---

## 7. Crate dependencies

### 7a. `crates/gribtract-core` (GRIB2 section parser + template decoders)

| Dependency | Manifest spec | Locked (`Cargo.lock`) | Role |
|------------|---------------|------------------------|------|
| `png` | `0.18` | `0.18.1` | DRT 5.41 (PNG) unpacking — `decode_drt41`. **Not used** by the Gaussian fixture. |
| `jpeg2k` | `0.10` (optional, `openjpeg-sys`) | `0.10.1` | DRT 5.40 (JPEG 2000), behind the `jpeg2000` feature. **Not used** by the Gaussian fixture. |
| `openjpeg-sys` | (transitive, via `jpeg2k`) | `1.0.12` | Native JPEG 2000 codec. |

> GDT 3.40 parsing and DRT 2/3 decoding pull in **no third-party crates** — they are pure Rust over
> the raw section bytes. The Gaussian fixture exercises only this dependency-free path.

### 7b. Workspace dependencies (`Cargo.toml`)

| Dependency | Manifest spec | Locked | Used by |
|------------|---------------|--------|---------|
| `serde` | `1` (+ derive) | `1.0.228` | `gribtract` (field JSON) |
| `serde_json` | `1` | `1.0.150` | `gribtract` |
| `sha2` | `0.10` | `0.10.9` | `xtask` corpus sha256 verification |
| `hex` | `0.4` | `0.4.3` | (declared) |
| `pyo3` | `0.22` (+ `extension-module`) | `0.22.6` | `gribtract-py` (excluded from default build) |

### 7c. Fetch path
The `xtask` corpus fetcher (`xtask/src/corpus.rs`) uses the `ureq` crate directly — **not**
`gribtract-fetch`/`reqwest`. It resolves the URL from the manifest `url` field (or
`GRIBTRACT_B2_ENDPOINT`/`GRIBTRACT_B2_BUCKET`), downloads, and verifies with `sha2`
(`verify_sha256`).

---

## 8. Project convention audit *(from bf-10ncjt)*

### 8a. Registered GFS corpus fixtures — ✅ conform
The three GFS manifest entries follow sibling convention exactly (raw NOAA filename under `large/`,
`storage: remote`, full `sha256` + `size_bytes` + `provenance`, `<source/grid>_<descriptor>` id
naming mirroring `nam_awip12_lambert_drt3`, `hrrr_conus_drt3_lambert`, `gefs_member01_pdt41`):

| Fixture ID | Manifest path | storage |
|------------|---------------|---------|
| `core_gaussian_gdt40` | `large/flx.2024011500.grib2` | remote |
| `gfs_gaussian_gdt40_t1534` | `large/gdas.t00z.sfluxgrbf000.grib2` | remote |
| `gfs_conus_drt0_0p50` | `large/gfs.t00z.pgrb2.0p50.f000` | remote |

### 8b. Test organization — ✅ conforms
`crates/gribtract/tests/diagnose_gfs_gaussian.rs` matches the sibling `diagnose_*` template
line-for-line, uses the registry-based `corpus::fixture_entry("core_gaussian_gdt40")` loader (the
modern convention), and the golden-regen hook `regenerate_gfs_gaussian_gdt40_t1534` follows the
`regenerate_<id>` convention. The missing Gaussian golden is **not** a deviation — it is the
corpus-wide rule (only `small/` fixtures carry committed goldens).

### 8c. Deviations found (carried forward for future cleanup)

| # | Axis | Severity | Finding | Fix |
|---|------|----------|---------|-----|
| D-mnr | Naming | Low | `gfs_conus_drt0_0p50`'s path lacks the `.grib2` extension every sibling uses | Rename on-disk + manifest |
| noaa-samples | Placement | High | `crates/gribtract/fixtures/noaa-samples/` is an unmanaged orphan: untracked, gitignored (`*.grib2`), **not** in the manifest, referenced by zero source/test/doc files, contains a 0-byte corrupt file and a byte-identical duplicate of `gfs_conus_drt0_0p50`. Holds only `gfs.*` files (no hrrr/nam/rap). | Delete, or re-add keepers the conventional way (under `tests/corpus/large/` + manifest + `diagnose_*` test) |

> The `noaa-samples/` orphan lives under `crates/gribtract/`, **not** the repo root, so it is out of
> scope for this doc's root-scratch cleanup. It is tracked here so it is not lost.

---

## 9. Commands

```bash
# Fetch + verify the fixture (sha256-checked)
cargo xtask corpus fetch core_gaussian_gdt40

# Build
cargo build -p gribtract

# Run the GFS Gaussian diagnostic test (currently fails — see §2c)
cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture

# Regenerate a golden reference (once PDT 4.12 lands)
python3 scripts/gen_golden.py tests/corpus/large/flx.2024011500.grib2 core_gaussian_gdt40
```

---

## 10. References

- **WMO GRIB2 Edition 2** — Tables 3.40 (Gaussian Lat/Lon), 5.2/5.3 (Data Representation).
- **NOAA NCEP** — GFS Model Grids and Parameters; CORe archive documentation.
- **Gaussian quadrature** — Legendre-polynomial zeros for optimal spherical integration.
- Internal: [`golden-json-schema`](../golden-json-schema.md), [`crates/gribtract-core/src/decode.rs`](../../crates/gribtract-core/src/decode.rs), [`crates/gribtract-core/src/types.rs`](../../crates/gribtract-core/src/types.rs).

---

*Canonical reference consolidated by bead bf-4yvt7s, 2026-07-26. Supersedes the six former GFS detail
docs. All template/file/dependency facts verified against the live workspace; status corrections to
prior docs noted inline (§2, §3b, §4).*
