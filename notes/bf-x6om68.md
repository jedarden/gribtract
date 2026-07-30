# GFS Gaussian-Grid Fixture — Locate & Understand (bf-x6om68)

## Task

Locate the GFS Gaussian-grid fixture, document its structure, compare it to existing fixtures, and
confirm integration readiness. **All facts below were re-verified directly against the live workspace
on 2026-07-26** (not transcribed from prior docs) — see §6 for the verification commands and what they
correct.

## TL;DR

- One real GFS Gaussian fixture exists on disk: **`tests/corpus/large/flx.2024011500.grib2`** (10.5 MiB,
  sha256-verified), registered as fixture ID **`core_gaussian_gdt40`**. A second Gaussian fixture
  (`gfs_gaussian_gdt40_t1534`, T1534) is manifest-declared only — its file is a **0-byte placeholder**.
- Grid is **GDT 3.40 (Gaussian Lat/Lon), 512×256, N=128, T254**, 131 072 points. GDT 3.40 is **already
  parsed** (`parse_gdt_40`); DRT 5.3/5.2 are **already decoded**.
- **Integration status: NOT ready end-to-end.** The single blocker is **PDT 4.12** (49 fields): it has
  no arm in `parse_section4` (`crates/gribtract-core/src/decode.rs:703` → `Err(NotImplemented)`), so
  `gribtract::decode()` returns `Err` for the whole file. A golden reference is also **absent**.
- The canonical single-source-of-truth doc is
  [`docs/fixtures/gfs-gaussian-fixture.md`](../docs/fixtures/gfs-gaussian-fixture.md) (consolidated by
  bead bf-4yvt7s). This note is the bead-level pointer into it; refer there for full detail.

## 1. Fixture location

| Fixture ID | Path on disk | Status |
|-----------|--------------|--------|
| `core_gaussian_gdt40` | `tests/corpus/large/flx.2024011500.grib2` | ✅ Present, 10 968 510 B, sha256 matches manifest |
| `gfs_gaussian_gdt40_t1534` | `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` | ⚠️ 0-byte placeholder (never fetched) |

- `storage: remote` — `tests/corpus/large/` is gitignored; fetched + sha256-verified via
  `cargo xtask corpus fetch core_gaussian_gdt40`.
- Source: NOAA **CORe** archive on Google Cloud Storage (public, no auth):
  `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`
  (source object has `.grb` extension but is GRIB2 edition 2 content).
- Registry: `tests/corpus/manifest.json` — entry carries `id`, `path`, `sha256`, `size_bytes`,
  `storage`, `url`, and a full `provenance` block.

## 2. Fixture structure (verified from bytes)

- **Grid (GDT 3.40 — Gaussian Latitude/Longitude):** `grid_template=40`; Gaussian 512×256; number of
  latitudes pole→equator `N=128`; **131 072 points**; La1/La2 = ±89.4629° (Gaussian quadrature
  spacing); Lo1=0° → Lo2=359.297° (uniform, di = 0.703125°); centre `kwbc` (NCEP=7), sub-centre 3;
  discipline 0. **T254** resolution.
- **104 GRIB2 messages** — CORe 3-hourly flux: radiative, heat, land-surface, soil, cloud-layer fields.
- **Packing (DRT):** DRT 5.3 (complex packing **with** spatial differencing) ×102 fields, and DRT 5.2
  (complex packing, no spatial differencing) ×2 fields (the file's last two records). Bitmap indicator
  255 (no bitmap) throughout.
- **Product templates (PDT):** PDT 4.2 (55 fields) + **PDT 4.12 (49 fields)**. The 49 PDT-4.12 fields
  are the time-averaged/accumulated fluxes (DLWRF, ULWRF, DSWRF, LHTFL, SHTFL, GFLUX, PEVPR, PRATE,
  CPRATE, …).

## 3. How it differs from existing fixtures

- **Grid:** GDT 3.40 (Gaussian — uniform longitude, *non-uniform Gaussian* latitude spacing, `dj` is
  null/N-driven) vs. the regular **GDT 3.0** lat/lon fixtures (uniform `di` and `dj`, e.g.
  `gfs_conus_drt0_0p50`, `gfs_anl_t2m_5x5`) and the **GDT 3.30** Lambert fixtures (HRRR/NAM CONUS).
- **Metadata/golden layout:** identical field-object schema to all golden fixtures (`parameter`,
  `forecast`, `level`, `grid`, `packing`, `values`) — see `docs/golden-json-schema.md`.
- **GFS files under `crates/gribtract/fixtures/noaa-samples/` and `gfs_conus_drt0_0p50` are regular
  lat/lon (GDT 3.0), NOT Gaussian** — easy to confuse with the Gaussian fixtures.
- `GaussianLatLonParams { n_parallels }` (in `crates/gribtract-core/src/types.rs`) is the only
  Gaussian-specific state; `nearest_index` approximates latitude spacing as linear between La1/La2
  (exact at corners; true Legendre-zero placement is a noted future optimization).

## 4. Integration readiness — ❌ NOT ready end-to-end

| Component | Status |
|-----------|--------|
| GDT 3.40 grid parsing (`parse_gdt_40`) | ✅ Implemented |
| DRT 5.2 / 5.3 decoding (`decode_drt3`) | ✅ Implemented |
| PDT 4.2 (`parse_section4` → `parse_pdt_0`) | ✅ Implemented (common-header) |
| **PDT 4.12** | ❌ **Not implemented — `decode.rs:703` → `Err(NotImplemented)`** |
| Golden `core_gaussian_gdt40.json` | ❌ Absent (corpus rule: only `small/` fixtures carry goldens) |
| Diagnostic test `diagnose_core_gaussian_gdt40` | ❌ Fails at golden load (line 13), before reaching `decode` |

**Single critical blocker:** add a PDT 4.12 arm to `parse_section4` (likely dispatching through the
`parse_pdt_8` common-header path + a time-range skip). Until it lands, `gribtract::decode()` returns
`Err` with 0 fields decoded for this fixture. Grid and packing are **not** blockers — they work.

## 5. Next steps for full integration

1. Implement PDT 4.12 in `parse_section4` (`crates/gribtract-core/src/decode.rs`).
2. Generate + commit `tests/corpus/golden/core_gaussian_gdt40.json` (via `scripts/gen_golden.py` /
   eccodes) once PDT 4.12 is decodable.
3. Re-run `cargo test -p gribtract --test diagnose_gfs_gaussian`.
4. (Optional) Fetch + sha256-verify the T1534 `gdas.t00z.sfluxgrbf000.grib2` (currently 0-byte).
5. (Optional) Model PDT 4.2/4.12 tail octets for full metadata fidelity.

## 6. Verification performed this session (commands + corrections)

Every claim above was checked directly, not transcribed:

- `ls -la tests/corpus/large/flx.2024011500.grib2` → 10 968 510 B; `ls tests/corpus/golden/` → 8 files,
  **no** `core_gaussian_gdt40.json`.
- Manifest entry in `tests/corpus/manifest.json` → sha256/size/path/url/provenance match.
- `wgrib2 tests/corpus/large/flx.2024011500.grib2 -grid` → `grid_template=40`, Gaussian 512×256,
  N=128, `#points=131072`.
- `wgrib2 … -packing` → 102× complex-with-spatial-differencing + 2× complex.
- `sed -n '672,705p' crates/gribtract-core/src/decode.rs` → `parse_section4` arms for PDT 0/1/2/8/11
  only; `_ => Err(Error::NotImplemented)` ⇒ PDT 4.12 unsupported.

**Corrections to the prior version of this note (commit `4ac2adf`, 2026-07-25):** that version
predated the canonical consolidation (bead bf-4yvt7s) and asserted two claims now refuted by direct
verification — (a) "Golden reference JSON exists, 378.3 MB ✅" (it does **not** exist), and (b) "❌
GDT 3.40 decoder pending" (GDT 3.40 is **already** implemented; the real blocker is PDT 4.12). Both are
fixed here. The fixture is **not** "Ready for Integration" as previously stated; it is blocked on PDT
4.12. See `docs/fixtures/gfs-gaussian-fixture.md` §2–§4 for the same conclusion.
