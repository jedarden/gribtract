# bf-91ov1 — Wire GFS Gaussian-grid fixture into differential suite

**Task:** Generate golden outputs for the GFS Gaussian-grid fixture with
`scripts/gen_golden.py`, wire it into `tests/differential.rs`, and verify the
differential suite passes with 100% agreement (ratcheting any disagreements).

**Result:** ✅ The GFS Gaussian fixture (`gfs_gaussian_gdt40_drt0`) is fully
wired into the differential suite and the full corpus is at **100% agreement** —
8/8 comparable fixtures by default, 9/9 with the `jpeg2000` feature. The
`AGREEMENT_FLOOR` ratchet is raised from 84.0% → 100.0%.

## State at the start of this bead

All four blocking sub-beads were already closed, and their work was committed:
the fixture (`tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2`), the manifest
entry (`inline`, sha256-verified), the golden
(`tests/corpus/golden/gfs_gaussian_gdt40_drt0.json`), and the generator
(`scripts/gen_grib2_gaussian.py`). The `differential.rs` harness already loops
the manifest, so the fixture was picked up automatically — no source edit was
needed to "add" it to the harness.

What remained uncommitted in the working tree was the **`gen_golden.py`
enhancement** that fixes a latent golden-generation bug, plus the two goldens it
regenerated. Verifying + committing that is this bead's substance.

## The bug the `gen_golden.py` enhancement fixes

`grib_dump -j` (eccodes JSON mode) **omits the Section-5 simple/complex packing
header keys** — `referenceValue`, `binaryScaleFactor`, `decimalScaleFactor`,
`numberOfBitsContainingEachPackedValue`, `typeOfOriginalFieldValues`. The old
`gen_golden.py` therefore emitted goldens with default/placeholder packing
values, e.g. the PDT=1 golden had `reference_value: 0.0, bits_per_value: 0`
instead of the real `250.0 / 8`.

The enhanced script adds `run_grib_ls_keys()`, which fetches those keys via
`grib_ls -p` (one dict per GRIB message) and merges them into the dump data.
Re-running it on the affected fixtures produces correct goldens.

## Verification

### Fixture integrity
```
sha256sum tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2
d9d0814526f2415d2e1c4af28e1e958897e512d003ba32f1ff03b84e3fab0c6a
```
Matches `manifest.json` exactly.

### Golden reproducibility
`gen_golden.py` output for both `gfs_gaussian_gdt40_drt0` and
`pdt1_ensemble_3x2` is byte-for-byte reproducible (regenerated to /tmp,
`json.dumps(..., sort_keys=True)` diff against the working tree = identical).

Only those two goldens needed regeneration: every other committed golden already
carries correct packing values (e.g. `gfs_anl_t2m_5x5` → `reference_value=270.0,
bits_per_value=8`). The PDT=1 golden was the one that had been left with the
bogus `0.0 / 0`.

### Differential harness — 100%, default features
```
$ cargo test -p gribtract --test differential -- --nocapture
=== Differential Harness Coverage ===
Fixtures : 22 total  (8 comparable, 12 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 8
  decode errors: 0
Agreement: 8/8 (100.0%)
Per-template:
  ...
  GDT=40 PDT=0 DRT=0: 1/1        ← gfs_gaussian_gdt40_drt0
```
Running the compiled binary directly surfaces the per-fixture line:
`[match] gfs_gaussian_gdt40_drt0`.

### Differential harness — 100%, `jpeg2000` feature
```
Agreement: 9/9 (100.0%)
  GDT=0 PDT=0 DRT=40: 1/1        ← drt40_j2k_3x2 (previously feature-skipped)
  GDT=40 PDT=0 DRT=0: 1/1        ← gfs_gaussian_gdt40_drt0
```
Both feature configurations are at 100%, so the floor is safe at 100.0%.

## Ratchet

`AGREEMENT_FLOOR` in `crates/gribtract/tests/differential.rs` raised
**84.0% → 100.0%**. The prior comment's partial-match numbers (GDT=30 9/18,
187/374) were stale — they describe an older per-value matching model that no
longer applies; replaced with the current per-fixture state.

## Files

- `scripts/gen_golden.py` — add `run_grib_ls_keys()` + `PACKING_KEYS`; merge
  Section-5 packing keys (sourced from `grib_ls -p`) into each golden message;
  emit `parser_version`.
- `tests/corpus/golden/gfs_gaussian_gdt40_drt0.json` — regenerated (canonical
  key order + `parser_version`; packing was already correct).
- `tests/corpus/golden/pdt1_ensemble_3x2.json` — regenerated (fixes
  `reference_value 0.0→250.0`, `bits_per_value 0→8`; closes the last mismatch).
- `crates/gribtract/tests/differential.rs` — ratchet `AGREEMENT_FLOOR` → 100.0.
