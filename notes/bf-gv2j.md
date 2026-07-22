# bf-gv2j — Differential suite verification of specialized extractors

## Task

Run the full differential test suite to verify that the DRT=3 specialized group
extractors (w = 4/8/12/16) maintain 100% agreement with the reference
implementation. Correctness gates speed per the plan rules.

## Outcome

All acceptance criteria met. **No code changes were required** — the specialized
extractors already maintain full agreement, so this bead is a verification pass.

### 1. cargo test passes — full workspace, 0 failures

```
41 passed  (gribtract-core lib, includes decode::tests)
13 passed  (gribtract lib)
12 passed  (gribtract-testutil lib: corpus/golden/diff)
11 passed  (xtask: corpus/probe_providers)
 3 passed  (gribtract-fetch client)
 2 passed  (gribtract doctests: provider_probe, timeseries)
 2 passed  (station_extraction integration test)
 1 passed  (differential_coverage_report)
 0 ×4     (empty targets)
─────────
85 passed; 0 failed; 0 ignored; 0 errors
```

### 2. Differential suite reports 100% agreement

```
=== Differential Harness Coverage ===
Fixtures : 10 total  (6 comparable, 2 no-golden, 2 skipped-feature)
  matched      : 6
  decode errors: 0
Agreement: 6/6 (100.0%)
Per-template:
  GDT=0 PDT=0 DRT=0: 1/1
  GDT=0 PDT=0 DRT=2: 1/1
  GDT=0 PDT=0 DRT=3: 1/1   ← specialized extractors (real GFS fixture)
  GDT=0 PDT=0 DRT=41: 1/1
  GDT=0 PDT=1 DRT=0: 1/1
  GDT=0 PDT=8 DRT=0: 1/1
```

`AGREEMENT_FLOOR` is set to `100.0`; the assertion passes.

### 3. No correctness regressions

The two dedicated cross-validation tests for the extractors both pass:

- `extract_group_specialized_matches_generic` — for every (skip ∈ 0..=7,
  w ∈ {4,8,12,16}, count ∈ 1..=20), the specialized dispatcher
  (`extract_group_specialized`) produces element-for-element identical output
  to the generic `extract_group_windowed`, including matching output length.
  Uses a high-entropy xorshift32 buffer so every bit/nibble position takes
  varied values.
- `extract_group_windowed_matches_read_bits_at` — the generic windowed
  extractor matches the per-element `read_bits_at` reference across
  (skip ∈ 0..7, w ∈ 1..=20).

The end-to-end DRT=3 path is additionally covered by
`decode_point_drt3_matches_full_decode` on the real GFS 1° fixture
(`gfs_tmp2m_1deg_anl`, 65 160 points), which passed.

## Coverage notes

- 2 fixtures are `no-golden` (golden references not yet generated) and 2 are
  `skipped-feature` (DRT=40 JPEG2000; the `jpeg2000` cargo feature is off by
  default). Both categories are excluded from the comparable denominator by
  `CoverageReport::agreement_pct()`, so neither masks a regression.
- The specialized extractors are only reachable through the DRT=3 decode path;
  DRT=3 coverage is exercised by the `gfs_tmp2m_1deg_anl` fixture plus the
  exhaustive parameter-matrix unit test above.

## Files

No source changes. This note is the sole artifact of the bead.
