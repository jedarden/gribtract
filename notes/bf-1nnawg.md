# bf-1nnawg — Verify compilation and run differential tests

**Task:** Verify the GFS Gaussian-grid integration compiles and the differential
tests pass. Verification only — no source changes were made.

## Results

### 1. `cargo build` — PASS
Workspace (default members) compiles cleanly, exit 0, no errors. Only pre-existing
warnings (unused `context`/imports, `default-features` workspace note) — none new.

### 2. Differential test (`cargo test --test differential`) — PASS

```
=== Differential Harness Coverage ===
Fixtures : 22 total  (8 comparable, 12 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 7
  decode errors: 0
Agreement: 7/8 (87.5%)   >= 84.0% floor  ✓
Per-template:
  GDT=40 PDT=0 DRT=0: 1/1   ← GFS Gaussian grid template
=====================================
test differential_coverage_report ... ok
```

### 3. GFS Gaussian-grid fixture (`gfs_gaussian_gdt40_drt0`) — MATCH
- The new inline fixture is the **only** inline GDT=40 fixture (the other two
  Gaussian entries, `core_gaussian_gdt40` and `gfs_gaussian_gdt40_t1534`, are
  `remote` and unfetched → skipped). So `GDT=40 PDT=0 DRT=0: 1/1` is
  unambiguously this fixture matching its golden (`tests/corpus/golden/
  gfs_gaussian_gdt40_drt0.json`).
- 0 decode errors. Id contains `drt0` (not `drt40`), so it is not skipped by the
  `jpeg2000` feature gate.

### 4. Full workspace `cargo test`
- `gribtract` lib unittests: 13 passed
- `gribtract-testutil` lib unittests: 13 passed
- `diagnose_conus_drt0`: 1 passed
- `differential`: 1 passed
- `diagnose_gefs`: **2 failed — pre-existing, unrelated**

## `diagnose_gefs` failures are NOT a regression

`diagnose_gefs_member01_pdt41` and `diagnose_gefs_ensemble_mean_pdt48` panic at
`.expect("golden loaded")` because their golden references were never authored
(`tests/corpus/golden/gefs_*` does not exist). The ratcheted differential harness
handles missing goldens gracefully (counted as "12 no-golden"); these two
standalone diagnostic tests hard-`.expect()` them.

Proof it is unrelated to the GFS Gaussian work:
- The GFS Gaussian commit (`77aa40b`) touched **no** GEFS files — only an additive
  manifest entry (+13 lines), the fixture `.grib2`, its golden `.json`, a generator
  script, and a note.
- The same 2 failures reproduce identically on `77aa40b^` (parent, before any GFS
  Gaussian work): `test result: FAILED. 0 passed; 2 failed`.
- Manifest still parses (valid JSON, 22 fixtures, GEFS entries intact).

## Conclusion
The GFS Gaussian-grid integration compiles and the differential test passes; the
new fixture matches its golden (`GDT=40 PDT=0 DRT=0: 1/1`). The only failing tests
are pre-existing `diagnose_gefs` failures from never-authored GEFS goldens,
independent of this change.
