# Differential Suite Verification — 100% Agreement (bf-2unbv)

## Task

Run the differential test suite with all four new fixtures and resolve any disagreements
between the reference decoder and the gribtract decoder. Acceptance: suite runs, all four
fixtures show 100% agreement, any disagreements fixed or ratcheted.

**All facts below verified directly against the live workspace on 2026-07-26.**

## TL;DR

- ✅ **Differential suite passes at 100.0% agreement.** `cargo test --test differential` →
  8/8 comparable fixtures match (default features). With `--features gribtract/jpeg2000` →
  **9/9** (100.0%). Zero decode errors.
- ✅ **All four target fixtures match their goldens**, each in its own per-template bucket:
  | Fixture | Role | Bucket | Result |
  |---|---|---|---|
  | `conus_drt0` | CONUS DRT=0 | GDT=0 PDT=0 DRT=0 | 2/2 ✓ |
  | `gfs_tmp2m_1deg_anl` | Lambert/global DRT=3 | GDT=0 PDT=0 DRT=3 | 1/1 ✓ |
  | `pdt1_ensemble_3x2` | ensemble | GDT=0 PDT=1 DRT=0 | 1/1 ✓ |
  | `gfs_gaussian_gdt40_drt0` | Gaussian | GDT=40 PDT=0 DRT=0 | 1/1 ✓ |
- ✅ **No disagreements to fix or ratchet.** `AGREEMENT_FLOOR` is already 100.0 (ratcheted in
  commit `30bb967`; `conus_drt0` golden regenerated in `823874f`). No code change was required
  for this bead — it is a pure run-and-verify task.

## 1. What "the differential suite" is

The ratcheted assertion harness is a single test:
**`crates/gribtract/tests/differential.rs::differential_coverage_report`**. It is manifest-driven
(`corpus::list_fixtures()` auto-discovers every entry in `tests/corpus/manifest.json`), so "wiring"
a fixture needs no per-fixture code. It loads each inline (or locally-fetched remote) fixture's
golden via `golden::load_golden`, decodes with `gribtract::decode`, and compares via
`compare_fixture`. `AGREEMENT_FLOOR = 100.0` gates the test.

## 2. Verification commands + output

```text
$ cargo test --test differential -- --nocapture
=== Differential Harness Coverage ===
Fixtures : 22 total  (8 comparable, 12 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 8
  decode errors: 0
Agreement: 8/8 (100.0%)
Per-template:
  GDT=0 PDT=0 DRT=0: 2/2
  GDT=0 PDT=0 DRT=2: 1/1
  GDT=0 PDT=0 DRT=3: 1/1
  GDT=0 PDT=0 DRT=41: 1/1
  GDT=0 PDT=1 DRT=0: 1/1
  GDT=0 PDT=8 DRT=0: 1/1
  GDT=40 PDT=0 DRT=0: 1/1
=====================================
test result: ok. 1 passed
```

With the JPEG2000 feature the two DRT=40 fixtures come back into scope:

```text
$ cargo test --test differential --features gribtract/jpeg2000 -- --nocapture
Fixtures : 22 total  (9 comparable, 13 no-golden, 0 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 9        decode errors: 0
Agreement: 9/9 (100.0%)
```

## 3. Field-level confirmation of the four target fixtures

Beyond the harness counts, field-level diagnostic tests confirm individual agreement:

- `conus_drt0` → `differential_mismatch.rs::diagnose_conus_drt0` **passes** (full per-field
  `compare_field` against golden, all `FieldResult::Match`).
- `gfs_gaussian_gdt40_drt0` → `diagnose_gfs_gaussian.rs::diagnose_gfs_gaussian_gdt40` **passes**.

(`pdt1_ensemble_3x2` and `gfs_tmp2m_1deg_anl` have no standalone diagnose test, but each is the
sole member of its 1/1 per-template bucket in the harness, so a match is a full match.)

## 4. Note on `differential_mismatch.rs` failures (NOT a regression)

`cargo test --test differential_mismatch` reports 4 failures:
`diagnose_nam_awip12_lambert_drt3`, `diagnose_mrms_carib_refl_drt41`,
`diagnose_gefs_ensemble_mean_pdt48`, `diagnose_gefs_member01_pdt41`.

These are **expected and pre-existing** — they are manual diagnostic scaffolding (each panics at
`.expect("golden loaded")` / `.expect("fixture loaded")`), not the ratcheted suite. Each maps to a
fixture with **no golden reference and/or an unfetched remote**:

| Fixture | Storage | Golden |
|---|---|---|
| `nam_awip12_lambert_drt3` | remote | absent |
| `mrms_carib_refl_drt41` | inline | absent |
| `gefs_ensemble_mean_pdt48` | remote | absent |
| `gefs_member01_pdt41` | remote | absent |

None of these is one of the four target fixtures. They fail only because their goldens don't exist
yet — exactly the gap these diagnose tests exist to close when those fixtures are implemented. No
action required for this bead.

## 5. Conclusion

All three acceptance criteria are met: the suite runs successfully, all four fixtures show 100%
agreement (8/8 default, 9/9 with jpeg2000), and there are no open disagreements. No source change
was required; this commit adds only this verification note.
