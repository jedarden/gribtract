# bf-5tz5p — Verify cargo test passes for GFS Gaussian-grid fixture

**Task:** Run cargo test for the GFS Gaussian-grid fixture, verify 100% agreement
with golden outputs, and ratchet any disagreements. Verification + one test fix.

**Result:** ✅ The GFS Gaussian fixture (`gfs_gaussian_gdt40_drt0`) is at **100%
agreement** with its golden — verified by two independent test paths. There were
**no value disagreements to ratchet**. One standalone test
(`diagnose_gfs_gaussian.rs`) was failing for an unrelated reason (missing golden
for a remote fixture) and was fixed.

## What "the GFS Gaussian fixture" is

The committed, runnable GFS Gaussian fixture is the **inline**
`gfs_gaussian_gdt40_drt0` (307-byte synthetic GRIB2; GDT 3.40 Gaussian lat/lon,
PDT 4.0, DRT 5.0 simple packing, 128 points) with its golden
`tests/corpus/golden/gfs_gaussian_gdt40_drt0.json` (1 field). The two large
*remote* Gaussian entries (`core_gaussian_gdt40`, `gfs_gaussian_gdt40_t1534`)
are not committed (10.5 MiB / 122 MiB, gitignored) and have **no authored
goldens**, so they are skipped by the differential harness and never run inline.

## Verification — 100% agreement, two paths

### 1. Manifest-driven differential harness — PASS (1/1)
```
$ cargo test --test differential -- --nocapture
Agreement: 7/8 (87.5%)
Per-template:
  GDT=40 PDT=0 DRT=0: 1/1        ← gfs_gaussian_gdt40_drt0, matches its golden
test differential_coverage_report ... ok
```
`GDT=40 PDT=0 DRT=0: 1/1` is unambiguously the inline GFS Gaussian fixture — it
is the only inline GDT=40 fixture. 0 decode errors.

### 2. Field-by-field diagnostic — PASS (Field 0: MATCH)
```
$ cargo test --test diagnose_gfs_gaussian -- --nocapture
=== GFS Gaussian-grid GDT40 Differential Analysis (gfs_gaussian_gdt40_drt0) ===
Total fields: actual=1, golden=1
Field 0: MATCH
test diagnose_gfs_gaussian_gdt40 ... ok
```

### No disagreements to ratchet
The single overall differential mismatch (`GDT=0 PDT=1 DRT=0: 0/1`) is the
**PDT1 ensemble** fixture, not GFS Gaussian. It is absorbed by the 84% agreement
floor and is out of scope for this bead.

## Fix applied: `diagnose_gfs_gaussian.rs` retargeted

The standalone `diagnose_gfs_gaussian.rs` (added in `ee49f3a`) was **failing** —
but not from a decode disagreement. It targeted `core_gaussian_gdt40` (remote,
unfetched, **no committed golden**) and hard-`.expect("golden loaded")`-ed that
missing golden, panicking immediately:

```
thread 'diagnose_core_gaussian_gdt40' panicked at diagnose_gfs_gaussian.rs:13:
golden loaded
```

This is the same pre-existing missing-golden pattern as the `diagnose_gefs`
failures (documented in bf-1nnawg). The commit message's stated precedent
(`diagnose_pdt1_ensemble.rs`) does not exist, and its "GDT 3.40 not yet
implemented" premise was already proven stale by bf-5ysjo/bf-6brqqn.

**Fix:** retargeted the diagnostic at the committed inline fixture
`gfs_gaussian_gdt40_drt0`, which has both committed bytes and a golden. This
turns a test that panicked on a missing remote golden into a real, meaningful,
**passing** differential analysis of the GFS Gaussian fixture — printing
`Field 0: MATCH`. The analysis structure (per-field Meta/Values/Length/Mask
comparison) is unchanged and remains useful for future debugging.

- Renamed test fn `diagnose_core_gaussian_gdt40` → `diagnose_gfs_gaussian_gdt40`.
- Updated header comment + output labels to reflect the inline fixture and
  explain why the remote Gaussian fixtures are not run here.

## Full suite status after the fix

| Test | Result |
|------|--------|
| `differential` | ✅ 1 passed (GFS Gaussian 1/1) |
| `diagnose_gfs_gaussian` | ✅ 1 passed (Field 0: MATCH) — **fixed this bead** |
| `gribtract` lib unittests | ✅ 13 passed |
| `diagnose_conus_drt0` | ✅ 1 passed |
| `diagnose_gefs` (2 tests) | ❌ pre-existing, unrelated — missing GEFS goldens for remote fixtures (documented in bf-1nnawg, fails identically on the parent of the GFS Gaussian commit) |

The `diagnose_gefs` failures are out of scope (GEFS, not GFS Gaussian) and were
not touched.

## Acceptance criteria
- ✅ Ran cargo test for the GFS Gaussian-grid fixture (differential + diagnostic)
- ✅ Verified 100% agreement with golden outputs (1/1 + Field 0: MATCH)
- ✅ No disagreements to ratchet (the one overall mismatch is PDT1 ensemble, not GFS Gaussian)
- ✅ GFS Gaussian tests pass successfully
- ✅ Documented the (non-value) disagreement root cause + fix in this note
