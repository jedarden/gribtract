# bf-6brqqn — Add GFS Gaussian-grid test case to the differential suite

**Status:** ✅ COMPLETE
**Workspace:** /home/coding/gribtract
**Date:** 2026-07-26

## Task

> Add the GFS Gaussian-grid test case to `tests/differential.rs` following the documented
> pattern (PDT1 ensemble).

## What "the documented pattern" actually requires

`crates/gribtract/tests/differential.rs` is an **automated harness**: it iterates over every
fixture in `tests/corpus/manifest.json` via `corpus::list_fixtures()` and compares each
decoder output against its golden reference. There is **no per-fixture test case to add** —
this was already established by bead bf-4gd44 for the PDT1 ensemble: *"any fixture added to
the corpus manifest with a golden reference automatically participates."*

So the concrete deliverable is the **golden reference**, which is the one missing piece. The
small synthetic inline fixture `gfs_gaussian_gdt40_drt0` (PDT 4.0 / GDT 3.40 / DRT 5.0, 128
points) had its manifest entry and `.grib2` file but **no golden** — so it was counted as
`no-golden` and never exercised.

> Note: this fixture is distinct from the **large remote** `core_gaussian_gdt40`
> (T254, PDT 4.12 blocker) that the recent bead chain (x6om68 / mzbmba / 4swew5 / 5ydxrm /
> 658687 / 2pev44) concerns. The small synthetic fixture deliberately uses PDT 4.0 / DRT 5.0
> to avoid that blocker, and decodes cleanly.

## Bug found and fixed while authoring the golden

The one-off probe `tests/_probe_gaussian.rs` (created to dump the exact decoded `Field`)
revealed `lat_last = -2067.483648` — a nonsense latitude. Root cause: `gen_grib2_gaussian.py`
encoded La2 as `u32(-80_000_000 & 0xFFFFFFFF)` (**two's complement**), but gribtract's
`read_latlon_micro` reads latitude/longitude as **sign-magnitude** (bit 31 = sign). The
manifest documents the fixture's intent as `Lat bounds +80N to -80N`.

Rather than bake `-2067.483648` into the golden (committing a known-wrong value as reference
truth) or omit the field to hide the bug, the generator's one-line La2 encoding was corrected
to use the sign-magnitude `latlon_micro(-80)` helper the script already defines (and applies
to La1). After regeneration the decoder reports `lat_last = -80`, matching the documented
intent — the same way the PDT1 golden reflects its fixture's true values.

## Files

| File | Change |
|------|--------|
| `scripts/gen_grib2_gaussian.py` | **New** (uncommitted prior). Fix La2 → `latlon_micro(-80)` (sign-magnitude). |
| `tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2` | **New**, force-added past `*.grib2` ignore (same as the other committed small fixtures). Regenerated; 307 B. |
| `tests/corpus/manifest.json` | Add `gfs_gaussian_gdt40_drt0` entry; sha256 = `d9d08145…` (matches regenerated fixture). |
| `tests/corpus/golden/gfs_gaussian_gdt40_drt0.json` | **New** golden, authored bit-exact from the corrected probe output (128 values 270.0..397.0 K). |
| `crates/gribtract/tests/_probe_gaussian.rs` | **Deleted** — its own comment said "Delete after generating the golden." |

New fixture sha256: `d9d0814526f2415d2e1c4af28e1e958897e512d003ba32f1ff03b84e3fab0c6a`
(size 307 B, unchanged by the fix).

## Verification (all run this session)

```
cargo test -p gribtract --test differential
  → test result: ok. 1 passed
  → Per-template: GDT=40 PDT=0 DRT=0: 1/1        ← the Gaussian fixture, matched
  → Agreement: 7/8 (87.5%)  ≥ AGREEMENT_FLOOR 84.0%

cargo test -p gribtract-testutil
  → test result: ok. 12 passed (corpus + golden unit tests, incl. sha256 verification)
```

The Gaussian fixture is the only GDT=40 entry in the corpus, so `GDT=40 PDT=0 DRT=0: 1/1`
confirms it participates and matches. Had it mismatched/errored, agreement would have been
6/8 = 75% < 84% and the suite would have failed.
