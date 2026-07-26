# GFS Fixture — Project Convention Audit

**Bead**: bf-10ncjt
**Purpose**: Compare the GFS fixture's placement, file naming, test organization, and documentation against the gribtract project conventions established by sibling fixtures, tests, and docs. Produce an explicit deviations list.
**Method**: Direct inspection of `tests/corpus/manifest.json`, the on-disk fixture trees, `crates/gribtract/tests/*`, `crates/gribtract-testutil/src/*`, and `docs/fixtures/*`. `git ls-files`, `git check-ignore`, `git grep`, `sha256sum`, and `stat` used for tracking/orphan/identity checks.
**Run date**: 2026-07-26
**Workspace**: `/home/coding/gribtract`

---

## TL;DR

The GFS fixture splits into **two unrelated things**, and the task's premise conflates them:

1. **The registered GFS corpus fixtures** (`core_gaussian_gdt40`, `gfs_gaussian_gdt40_t1534`, `gfs_conus_drt0_0p50`) **conform** to project convention. They live in `tests/corpus/large/`, are registered in `tests/corpus/manifest.json` with full provenance + sha256, are referenced by the registry-based `diagnose_*` tests, and follow sibling naming. Test and (mostly) doc organization for these are on-convention.

2. **An orphan directory `crates/gribtract/fixtures/noaa-samples/`** holding 10 untracked GFS `.grib2` files **deviates** from every project convention. It is not in the corpus, not in the manifest, not tracked, not referenced by any test/source/doc, and contains a 0-byte corrupt file plus a byte-identical duplicate of an already-registered fixture.

Two premise corrections (both verified, both echo findings in the sibling doc [`gfs-gaussian-inventory.md`](gfs-gaussian-inventory.md)):

- **The task's "`noaa-samples/` (hrrr/nam/rap/gfs)" premise is wrong.** `crates/gribtract/fixtures/noaa-samples/` contains **only** `gfs.*` files — there are **no** hrrr/nam/rap files in it. The hrrr/nam/rap fixtures actually live in `tests/corpus/large/`.
- **The canonical fixture home is `tests/corpus/`, not `fixtures/noaa-samples/`.** `noaa-samples/` is an unmanaged staging/download area, not part of the corpus convention.

**Deviation count**: 12 findings — D1–D8 are fixture-placement/naming (all against the `noaa-samples/` orphan, plus 1 minor extension inconsistency in one corpus entry); D9–D12 are documentation. Test organization has **no** deviations.

---

## 0. What the project convention actually is

Cited here once so the comparisons below are unambiguous.

| Axis | Convention | Authority |
|------|------------|-----------|
| Fixture home | `tests/corpus/{small,large,golden}/` at workspace root | `crates/gribtract-testutil/src/corpus.rs` (`corpus_root()` → `<manifest_dir>/../../tests/corpus`) |
| Fixture registry | `tests/corpus/manifest.json` — each entry has `id`, `path`, `sha256`, `size_bytes`, `storage`, `provenance` | `tests/corpus/manifest.json` (version 1, 21 fixtures) |
| Storage | `inline` = committed (small synthetic); `remote` = gitignored, fetched by sha256 (large) | manifest `storage` field; `.gitignore` lines `*.grib2` + `/tests/corpus/large/` |
| Integrity | sha256 **and** size verified on every load | `corpus::load()` in `corpus.rs` |
| File naming | Raw NOAA/NCEP source filenames preserved verbatim under `large/` | every `large/` entry in the manifest |
| Test loader | `gribtract_testutil::corpus::fixture_entry("<id>")` → `golden::load_golden` → `corpus::load` → `gribtract::decode` → `diff::compare_field` | `diagnose_conus_drt0.rs`, `diagnose_gefs.rs` |
| Golden | `tests/corpus/golden/<id>.json`; only `small/` inline fixtures carry committed goldens | `tests/corpus/golden/` (8 files, all for `small/` fixtures) |
| Docs index | `docs/fixtures/README.md`; lowercase-kebab-case filenames | `docs/fixtures/README.md` |

---

## 1. Fixture placement & file naming

### 1a. Registered GFS corpus fixtures — ✅ conform

The three GFS entries in the manifest follow sibling convention exactly:

| Fixture ID | Manifest path | storage | Sibling it mirrors |
|------------|---------------|---------|--------------------|
| `core_gaussian_gdt40` | `large/flx.2024011500.grib2` | remote | nam/hrrr: raw NCEP filename, remote |
| `gfs_gaussian_gdt40_t1534` | `large/gdas.t00z.sfluxgrbf000.grib2` | remote | gefs: raw NCEP filename, remote |
| `gfs_conus_drt0_0p50` | `large/gfs.t00z.pgrb2.0p50.f000` | remote | nam/hrrr: raw NCEP filename, remote |

Each is registered with `sha256` + `size_bytes` + `provenance`, is `storage: remote` (consistent with **all** large fixtures — nam, hrrr, gefs, ecmwf are all remote), and uses a raw source filename under `large/`. ID naming (`<source/grid>_<descriptor>`) matches siblings like `nam_awip12_lambert_drt3`, `hrrr_conus_drt3_lambert`, `gefs_member01_pdt41`. **No deviation.**

### 1b. The `noaa-samples/` orphan — ❌ deviates (D1–D8)

`crates/gribtract/fixtures/noaa-samples/` holds 10 GFS files:

```
gfs.20260722.t00z.pgrb2.1p00.f000.grib2   gfs.t00z.pgrb2.0p50.f000.grib2
gfs.20260723.t00z.pgrb2.1p00.f000.grib2   gfs.t00z.pgrb2.0p50.f003.grib2
gfs.20260723.t00z.pgrb2.1p00.f006.grib2   gfs.t00z.pgrb2.0p50.f006.grib2
gfs.t00z.pgrb2.1p00.f000.grib2  (0 bytes) gfs.t00z.pgrb2.1p00.f003.grib2
gfs.t00z.pgrb2.1p00.f006.grib2            gfs.t00z.pgrb2.1p00.f012.grib2
```

| # | Deviation | Evidence |
|---|-----------|----------|
| **D1** | Lives **outside** the canonical `tests/corpus/` tree | `crates/gribtract/fixtures/noaa-samples/` vs. `corpus_root()` resolution |
| **D2** | **Not registered** in `manifest.json` | `git grep` of all 21 manifest ids — none reference `noaa-samples` or the `1p00`/`0p50` forecast-hour files (only `gfs_conus_drt0_0p50` exists for GFS lat/lon, pointing at `large/`) |
| **D3** | **Untracked** (gitignored via `*.grib2`), with **no content addressing** — unlike `remote` corpus fixtures, which are fetched and verified by sha256 | `git check-ignore` → `.gitignore:19:*.grib2`; `git ls-files crates/gribtract/fixtures/` → empty |
| **D4** | **Referenced by zero** source/test/doc files (true orphan) | `git grep 'noaa-samples'` matches only `.beads/issues.jsonl` (this bead's own description) — no `.rs`/`.json`/`.md` hits |
| **D5** | Contains **only** `gfs.*` files — **no hrrr/nam/rap**, contradicting the task's "(hrrr/nam/rap/gfs)" premise | `find … \| sed … \| grep -oE '^(hrrr\|nam\|rap\|gfs)'` → `10 gfs`, 0 others |
| **D6** | **Inconsistent naming** within the dir: mixed date-stamped (`gfs.20260722.t00z…`) and undated (`gfs.t00z…`) forms; mixed resolutions (0p50/1p00); mixed forecast hours (f000/f003/f006/f012) | `ls` above; corpus convention is one raw filename per logical fixture |
| **D7** | Contains a **0-byte corrupt file** (`gfs.t00z.pgrb2.1p00.f000.grib2`) — no integrity check exists for this dir (corpus would reject it via sha256+size) | `stat -c%s` → `0` |
| **D8** | Contains a **byte-identical duplicate** of an already-registered corpus fixture: `gfs.t00z.pgrb2.0p50.f000.grib2` (152106356 B, sha256 `f2ccb6c8…`) == manifest `gfs_conus_drt0_0p50` (`large/gfs.t00z.pgrb2.0p50.f000`, 152106356 B, same sha256 prefix) | `sha256sum` vs. manifest `sha256` field |

### 1c. One minor naming deviation inside a *registered* GFS entry — D-mnr

| # | Deviation | Evidence |
|---|-----------|----------|
| **D-mnr** | `gfs_conus_drt0_0p50`'s manifest path `large/gfs.t00z.pgrb2.0p50.f000` is the **only** large fixture whose filename lacks the `.grib2` extension that every sibling uses (`nam.t00z.awip1200.tm00.grib2`, `hrrr.t12z.wrfsfcf00.grib2`, `gdas.t00z.sfluxgrbf000.grib2`, …) | manifest id→path listing; on-disk `tests/corpus/large/gfs.t00z.pgrb2.0p50.f000` |

---

## 2. Test organization — ✅ no deviations

`crates/gribtract/tests/diagnose_gfs_gaussian.rs` matches the sibling `diagnose_*` template (`diagnose_conus_drt0.rs`, `diagnose_gefs.rs`) line-for-line in structure:

```
//! Diagnostic test for <X> fixture
use gribtract_testutil::{corpus, diff::{compare_field, FieldResult}, golden};
#[test] fn diagnose_<id>() {
    let entry = corpus::fixture_entry("<id>").expect("fixture exists");
    let golden_fixture = golden::load_golden(&entry.id)…;
    let bytes = corpus::load(&entry.id)…;
    match gribtract::decode(&bytes) { … compare_field loop … }
}
```

It uses the registry-based `corpus::fixture_entry("core_gaussian_gdt40")` loader (the modern convention), not the legacy filename-based `corpus_root.join("large/…")` pattern still found in the older NAM/Lambert tests (`integration_nam_lambert.rs`, `verify_lambert_grid.rs`). The golden-regeneration hook `regenerate_gfs_gaussian_gdt40_t1534` in `regenerate_golden.rs` likewise follows the `regenerate_<id>` sibling convention. **No structural deviation.**

> **Note (not a deviation):** no golden file exists for `core_gaussian_gdt40` / `gfs_gaussian_gdt40_t1534` in `tests/corpus/golden/`. This is **consistent** with the corpus-wide rule — committed goldens exist only for `small/` inline fixtures; **every** `large/` fixture (nam, hrrr, gefs, ecmwf, and both GFS Gaussian) lacks one. It is a completeness gap tracked by beads bf-4swew5 / bf-5ydxrm (PDT 4.12 not yet implemented), not an organizational deviation.

---

## 3. Doc format & structure — ❌ deviates (D9–D12)

`docs/fixtures/` holds 6 markdown files (5 GFS detail docs + `README.md`). Findings:

| # | Deviation | Evidence |
|---|-----------|----------|
| **D9** | `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md` uses an **ALL-CAPS** filename, breaking the lowercase-kebab-case convention of every sibling (`gfs-fixtures-complete-reference.md`, `gfs-gaussian-grid-structure.md`, `gfs-gaussian-build-status.md`, `gfs-gaussian-inventory.md`) | `ls docs/fixtures/*.md` |
| **D10** | `README.md`'s "Available Documentation" index is **stale** — it links only 2 of the 5 GFS detail docs. Missing: `gfs-gaussian-build-status.md`, `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md`, `gfs-gaussian-inventory.md` | README links vs. `ls` |
| **D11** | **Doc proliferation/redundancy** — 4 of the 5 detail docs substantially overlap on the same Gaussian fixtures, with no single canonical entry and no reconciliation in the README. The inventory doc itself opens by noting it exists "so the canonical doc stops carrying repeated guesses" | `gfs-fixtures-complete-reference.md`, `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md`, `gfs-gaussian-grid-structure.md`, `gfs-gaussian-inventory.md` all describe `core_gaussian_gdt40` |
| **D12** | **No uniform provenance header** — `gfs-gaussian-build-status.md`, `gfs-gaussian-inventory.md`, `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md` carry `**Bead**:` / run-provenance blocks; `gfs-fixtures-complete-reference.md` and `gfs-gaussian-grid-structure.md` do not | headers of each doc |

> **Related content inconsistency (not strictly format):** `gfs-fixtures-complete-reference.md` and `GFS-GAUSSIAN-FIXTURE-MASTER-REFERENCE.md` mark the Gaussian fixtures "✅ Supported", while the empirically-verified `gfs-gaussian-inventory.md` (bead bf-5ydxrm) documents that the fixture is **not** end-to-end decodable (PDT 4.12 unimplemented → `gribtract::decode()` returns `Err`). This is a downstream symptom of D11 (no single source of truth).

---

## 4. Consolidated deviations list

| # | Axis | Severity | Finding | Fix |
|---|------|----------|---------|-----|
| D1 | Placement | High | `noaa-samples/` outside `tests/corpus/` | Move any keeper files into `tests/corpus/large/` + manifest, or delete |
| D2 | Placement | High | `noaa-samples/` not in `manifest.json` | Register or remove |
| D3 | Placement | High | `noaa-samples/` untracked, no sha256 integrity | Route through `storage: remote` + sha256, or delete |
| D4 | Placement | High | `noaa-samples/` referenced by nothing (orphan) | Delete, or wire a test to it |
| D5 | Naming | Med | `noaa-samples/` has only `gfs.*` (no hrrr/nam/rap) — premise error | n/a (factual correction) |
| D6 | Naming | Med | `noaa-samples/` mixed dated/undated/resolution/hour naming | Adopt one raw-filename-per-fixture |
| D7 | Integrity | High | 0-byte corrupt file in `noaa-samples/` | Delete |
| D8 | Redundancy | High | Byte-identical duplicate of `gfs_conus_drt0_0p50` in `noaa-samples/` | Delete the duplicate |
| D-mnr | Naming | Low | `gfs_conus_drt0_0p50` corpus path missing `.grib2` extension (sole large fixture) | Rename on-disk + manifest |
| D9 | Docs | Low | ALL-CAPS doc filename breaks kebab convention | Rename to lowercase-kebab |
| D10 | Docs | Med | README index omits 3 of 5 GFS detail docs | Update README links |
| D11 | Docs | Med | 4 overlapping Gaussian docs, no canonical entry / reconciliation | Consolidate; designate one source of truth |
| D12 | Docs | Low | No uniform provenance header across docs | Adopt one header convention |

**Positive statements (no deviation):**
- ✅ **Registered GFS corpus fixtures** (`core_gaussian_gdt40`, `gfs_gaussian_gdt40_t1534`, `gfs_conus_drt0_0p50`) conform to placement, storage, integrity, and naming conventions — cited in §1a.
- ✅ **Test organization** (`diagnose_gfs_gaussian.rs`, `regenerate_golden.rs`) conforms to the `diagnose_*` sibling template — cited in §2.
- ✅ **Missing Gaussian golden** is consistent with the corpus-wide rule (only `small/` fixtures carry goldens) — cited in §2 note, not a deviation.

---

## 5. Recommendation

The deviations cluster almost entirely on the **`noaa-samples/` orphan** (D1–D8). Recommended cleanup:

1. **Delete `crates/gribtract/fixtures/noaa-samples/` entirely.** D4 proves nothing references it; D3 means it carries no integrity guarantee; D7 (0-byte file) and D8 (duplicate of an existing corpus fixture) show it is not trustworthy as-is. If any `1p00` forecast-hour file is genuinely needed for future coverage, re-add it the conventional way: drop into `tests/corpus/large/` with its `.grib2` extension, add a manifest entry (`id` + `path` + `sha256` + `size_bytes` + `storage: remote` + `provenance`), and add a `diagnose_*` test.
2. **Fix D-mnr**: rename `tests/corpus/large/gfs.t00z.pgrb2.0p50.f000` → `….grib2` and update the `gfs_conus_drt0_0p50` manifest path.
3. **Docs (D9–D12)**: rename the ALL-CAPS doc to kebab-case, refresh the README index to list all five GFS detail docs, and consolidate the four overlapping Gaussian docs into a single canonical entry (the empirically-verified `gfs-gaussian-inventory.md` is the natural anchor), propagating its PDT-4.12 status correction to the others.

---

*Audit produced by bead bf-10ncjt. All file paths, sizes, sha256 prefixes, and git-tracking claims above were verified by direct command on 2026-07-26.*
