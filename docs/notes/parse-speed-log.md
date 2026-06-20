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

## Attempt 2: SIMD-like unpack via byte-aligned fast paths + loop refactor

**Technique:** Two changes to `decode.rs`:

1. **Byte-aligned fast paths in `unpack_n_bits`** — added `match n_bits { 8 | 16 | 32 => ... }`
   specializations that use direct byte/u16be/u32be reads instead of the general shift/mask
   loop. These expose the inner iterator loops to LLVM auto-vectorization.

2. **DRT=3 group inner loop refactor** — two sub-changes:
   - Zero-width group fast path: when `w == 0`, fill with `gref` × `l` without any bit reads.
   - Precomputed bit offset: replaced the loop-carried `bit_offset` update with
     `start_bit + k * w` so LLVM can treat `k` as the induction variable and potentially
     vectorize the `read_bits_at` calls within a group.

**Result:**
- DRT=0 (5×5 fixture, n_bits=8): 519K msg/s vs 422K baseline → **+23% throughput**, 106 MB/s vs 86 MB/s.
  The n_bits=8 fast path is the driver — direct byte copy, compiler auto-vectorizes.
- DRT=3 (GFS 1-degree, 65160 points): 313 msg/s vs 316 baseline → **no measurable change** (within noise).
  The precomputed-offset refactor did not move the DRT=3 needle: `read_bits_at` itself has
  too much internal branching (variable `bytes_needed`) for LLVM to vectorize it, even when
  the loop induction variable is dependency-free. The zero-width fast path may help on data
  with more w=0 groups, but not on this field.
- Agreement: 100% for DRT=0 and DRT=3 — no correctness regression.

**Why stopped:** DRT=3 improvement requires a fundamentally different approach — either
explicitly unrolling the common group widths (e.g. dedicated extractors for w=12, w=8) or
using `std::arch` SIMD intrinsics with `#[target_feature(enable="avx2")]`. Not attempted here
to keep the change auditable and the correctness bar clear. The DRT=0 win is kept.

**Implication for DRT=3 speed:** To materially speed up the DRT=3 inner loop, the next
attempt should specialize per-group extraction for the most common widths (typically w=0,
4, 8, 12 in GFS complex packing). A dispatch table keyed on `w` returning a closure that
reads values directly (without the `bytes_needed` branch) would remove the last major
scalar bottleneck in `read_bits_at`.

## Attempt 3: Parallelism across forecast-hour messages (rayon par_iter)

**Technique:** Fan the per-forecast-hour `Field` objects across cores using `rayon::par_iter()`
zipped with the geometry cache rows. Each thread handles one field's N-station extractions,
results summed. `Field` is `Send + Sync` (all fields are owned `Vec`s / primitive types).

**Result:**
- nearest serial: 59,829,060 station-hours/s
- nearest-parallel (rayon): 1,344,086 station-hours/s → **44× SLOWER**
- bilinear serial: 16,018,307 station-hours/s
- bilinear-parallel (rayon): 1,272,033 station-hours/s → **12× SLOWER**
- Agreement: 100% nearest, 42.9% bilinear (same as serial — no correctness change)

**Why it failed:** With only 2 fields and 7 stations per field, each rayon task is 7 Vec
index operations (< 100 ns). Rayon's work-stealing queue overhead (microseconds per task)
completely swamps the payload. The parallel version is essentially measuring thread-scheduling
latency, not extraction time.

**When parallelism would help:** With 40+ forecast-hour messages (a full GFS f000-f120 cycle)
AND more expensive per-field work (e.g., DRT=3 decode-on-demand rather than pre-decoded vecs),
the thread overhead becomes negligible. The geometry cache is already read-only and trivially
shared — the architecture is correct, just the current corpus is too thin to benefit.

**Reverted:** No code kept. Rayon not added as dependency.

**Prerequisite for a useful retry:** Real multi-message corpus (40+ messages from one GFS
cycle covering CONUS). Once that corpus exists, re-run with rayon on the pre-decoded
`&[Field]` slice — expected linear speedup to core count for the extraction loop.
