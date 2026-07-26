# bf-5ysjo — Add GFS Gaussian-grid fixture to differential.rs

**Task:** Add the GFS Gaussian-grid fixture to `tests/differential.rs` to
integrate it into the differential testing suite, following the same pattern as
existing fixtures (e.g., PDT1 ensemble).

**Status:** ✅ ALREADY COMPLETE — no source change needed.

> Note: this file supersedes an earlier (Jul 25) draft that referenced the large
> *remote* `core_gaussian_gdt40` fixture and claimed "GDT 3.40 decoding is not yet
> implemented". Both claims were stale: GDT 3.40 **is** implemented, and the actual
> inline GFS Gaussian fixture (`gfs_gaussian_gdt40_drt0`) was added the next day in
> commit `77aa40b` (bf-6brqqn). This rewrite reflects verified ground truth.

## Why there is nothing to add to differential.rs

`crates/gribtract/tests/differential.rs` is **manifest-driven**: its single test
`differential_coverage_report` iterates `corpus::list_fixtures()` (read from
`tests/corpus/manifest.json`) and compares each fixture against its golden. There
are **no per-fixture inline test cases** in that file. The cited precedent — the
**PDT1 ensemble** fixture (`pdt1_ensemble_3x2`) — is itself just a manifest entry,
not an inline test function. So "adding a fixture to differential.rs" == "register
a manifest entry", which already exists.

This bead is therefore a duplicate of the work done in **bf-6brqqn** (commit
`77aa40b`) and verified in **bf-1nnawg** (current HEAD). Matches the
`bead-autosplit-false-premise` pattern in project memory (auto-split firing on an
already-complete umbrella).

## The fixture is present at all three required locations

| Component | Path / value |
|-----------|--------------|
| Manifest entry | `tests/corpus/manifest.json` → `gfs_gaussian_gdt40_drt0`, `storage: inline` |
| GRIB2 file     | `tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2` (307 bytes) |
| Golden file    | `tests/corpus/golden/gfs_gaussian_gdt40_drt0.json` |

It is a small **synthetic inline** fixture (GDT 3.40 Gaussian lat/lon, PDT 4.0,
DRT 5.0 simple packing, 128 points) — the committed analogue of the two large
remote Gaussian fixtures (`core_gaussian_gdt40`, `gfs_gaussian_gdt40_t1534`).

## Ground-truth verification (run this session)

```
$ ls tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2 tests/corpus/golden/gfs_gaussian_gdt40_drt0.json
  (both present)

$ cargo test --test differential -- --nocapture
test differential_coverage_report ... ok
  matched      : 7
Agreement: 7/8 (87.5%)   >= 84.0% floor  ✓
  GDT=40 PDT=0 DRT=0: 1/1   ← GFS Gaussian grid template, matches its golden
```

The `GDT=40 PDT=0 DRT=0: 1/1` line is unambiguously `gfs_gaussian_gdt40_drt0`:
it is the only **inline** Gaussian fixture, so it is the only GDT=40 fixture that
actually runs (the two remote Gaussian entries are unfetched → skipped). 0 decode
errors, 87.5% agreement above the 84% floor. (GDT 3.40 decoding **is** implemented
— `core_gaussian_gdt40` and `gfs_gaussian_gdt40_t1534` were end-to-end verified in
beads bf-1qia4 / bf-6brqqn; the earlier "not implemented" note was simply stale.)

## Acceptance criteria — all met by existing committed state

- ✅ GFS Gaussian-grid test case integrated into the differential suite (via manifest)
- ✅ Same pattern as PDT1 ensemble (both are manifest entries picked up by the
     manifest-driven harness)
- ✅ Proper naming and organization (`gfs_gaussian_gdt40_drt0`)
- ✅ Code compiles without errors (`cargo test --test differential` passes)

## Conclusion

No source changes were produced. This bead is already-complete (work landed in
`77aa40b`); this note is the commit artifact per task instructions.
