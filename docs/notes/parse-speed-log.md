# Parse-Speed Optimization Log

Running log of techniques tried for the fixed-station point-extraction benchmark.
Format: technique → result → why (to avoid re-walking dead ends).

## Baseline

**Technique:** Full-grid decode (simple packing 5.0) → nearest-grid-point read.  
**Result:** Establishes baseline throughput. With the synthetic 5×5 inline fixture, US
metro stations are outside the grid (0–40N, 0–40E), so `in_range = 0` and
throughput is vacuously fast (no real grid lookups). Meaningful numbers require
real GFS data covering CONUS.  
**Why stopped:** This is the reference, not an optimization. Waiting for real data
(bead `bf-qfj`) before the throughput number is informative.

---

---

## Attempt 1: Lazy/partial unpack for DRT=0 (simple packing)

**Technique:** For template 5.0, point `i` lives at a fixed bit offset `i × bits_per_value`
in the Section 7 body — no inter-point dependencies.  Skip full-grid decode; instead store
raw Section 7 bytes in `LazyField::section7_raw` and call `decode_point_drt0(body, packing, idx)`
per station.

**Result:** Correctness confirmed on 5×5 fixture (all 25 points match full decode to tolerance).
Cannot measure station-hours/s speedup: the only DRT=0 fixture in corpus (0-40N, 0-40E) has
no US metro stations in range.  The GFS 1-degree fixture is DRT=3, which does NOT support
lazy extraction (section7_raw is empty for DRT≠0).

**Full-pipeline benefit (unmeasured):** Lazy path skips decoding the full grid (~65K float ops
for a 1-degree field). Per-station bit extraction adds ~7 read_bits_at calls vs 7 vec[idx]
accesses.  Net win is positive only when we need a small number of points from a large field —
cannot confirm without a DRT=0 CONUS fixture.

**Infrastructure kept:** `decode_point_drt0`, `LazyField`, `decode_bytes_lazy`, and
`run_lazy_nearest` bench are committed.  The bench runner calls `run_lazy_nearest` whenever
DRT=0 data is present, so the speedup will appear automatically when a CONUS DRT=0 fixture
is added to the corpus.

**As expected:** Does NOT generalize to complex packing (DRT=2/3) — group-based coding means
point `i` is NOT at a fixed bit offset; `section7_raw` is intentionally left empty for DRT≠0.

_Add entries here as optimization attempts are made. Include even failed attempts
so the loop doesn't repeat them._
