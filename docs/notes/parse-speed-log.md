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

## Attempt 4: Left-aligned u64 sliding-window extractor for DRT=3 groups

**Technique:** Replace the per-element `read_bits_at` calls inside each group's inner
loop in `decode_drt3` with a new `extract_group_windowed` function.  The function
maintains a 64-bit buffer that is kept left-aligned (valid bits at the MSB end):

- **Pre-fill:** load `ceil((skip + w) / 8)` bytes upfront so the buffer holds at
  least `(alignment_skip + w)` valid bits before the first extraction.
- **Extraction per element:** one `buf >> (64 - w) & mask` + `buf <<= w` — no
  division, no inner byte loop, no per-element `div_ceil`.
- **Refill:** `while buf_bits < w { load one byte }` runs at most `ceil(w/8) ≤ 4`
  times per element for `w ≤ 32`, amortised ~one byte load per 8 bits consumed.

**Result:**
- DRT=3 (GFS 1-degree, ~65K points): **109.1 MB/s vs 92.8 MB/s baseline → +17.6%**
- grid_points/sec: 149M vs 127M → +17.3%
- Agreement: 100% — no correctness regression.
- New cross-validation test (`extract_group_windowed_matches_read_bits_at`) verifies
  all (skip=0..7, w=1..20) combinations against `read_bits_at` — all 140 cases pass.

**Why it worked:** The original per-element path called `read_bits_at`, which
computed `bytes_needed = div_ceil(bit_start + n_bits, 8)` and then looped over
1–3 bytes to assemble `raw`.  Even with the loop unrolled, the branch over
`bytes_needed` (1 vs 2 vs 3 bytes) caused branch mispredictions.  The windowed
path eliminates this branch entirely: for a given group, the refill pattern is
determined only by `w mod 8`, which the branch predictor can learn quickly (or LLVM
can analyse at compile time).

**Code kept:** `extract_group_windowed` in `crates/gribtract-core/src/decode.rs`.
`read_bits_at` is retained (used by `unpack_n_bits`, `decode_point_drt0`, and the
non-group-value arrays in `decode_drt3`).

**Remaining DRT=3 headroom:** the spatial differencing step (running sum over ~65K
elements with a sequential dependency) is likely the next bottleneck — it cannot
be vectorized and is O(n_points).  Possible next attempts:
- Prefix-sum SIMD approximation (non-trivial correctness risk).
- Avoid the intermediate `Vec<i64>` by combining bit extraction + spatial diff in
  one pass (saves one O(n) allocation and scan).
- Increase group-level parallelism if the GRIB2 encoder writes independent group
  blocks (format investigation needed).

## Attempt 5: Merge spatial-diff pass and f64-scaling pass into one combined loop

**Technique:** `decode_drt3` previously executed two separate O(n) passes after
bit extraction:

1. In-place spatial-differencing over `packed: Vec<i64>` (reading and writing each element).
2. A separate scaling pass (`packed.iter().map(|&x| (r + x as f64 * two_e) / ten_d)`)
   that read each element a second time and wrote to `values: Vec<f64>`.

For a 65 K-point GFS field (`packed` ≈ 520 KB), `packed` exceeds a typical 256 KB L2
cache, so both passes read from L3/RAM.  The merge eliminates the second pass:

- For `order == 0` (DRT=2): one `.iter()` loop — no change in complexity.
- For `order == 1`/`order == 2`: seed values (ival1, ival2) are emitted directly
  from the seed variables rather than overwriting `packed[0..order]`, then each
  subsequent `packed[i]` is read exactly once, the running-sum state is updated
  inline, and the final scaled `f64` is pushed to `values`.  `packed` is never
  mutated after construction.

**Result (measured via `cargo run --manifest-path xtask/Cargo.toml --release -- bench`):**
- DRT=3 (GFS fixture, 65 K-point field): **~128–130 MB/s vs 109.1 MB/s (Attempt-4 baseline) → +17–19%**
  (two consecutive runs: 127.1 MB/s terminal, 129.9 MB/s bench-results.json; run-to-run variation ±2%).
- Compared to pre-windowed baseline (92.8 MB/s): +38–40% cumulative (Attempts 4+5 combined).
- Agreement: 100% — no correctness regression (all tests pass; 100% agreement on DRT=3).

**Why it worked:** The in-place spatial-diff pass wrote ~520 KB of i64 data back to
the same Vec, then the scaling pass read it all again.  With two passes, the CPU
fetched `packed` twice from L3.  One combined pass fetches it once; the write-back
of the intermediate i64 values is eliminated entirely.

**Code kept:** combined spatial-diff + scale loop in `decode_drt3`
(`crates/gribtract-core/src/decode.rs`).  The `packed` Vec remains (bit
extraction requires the intermediate buffer), but its post-construction write path
is removed.

**Next headroom:** The remaining bottleneck in `decode_drt3` is likely:
- The bit-extraction inner loop itself (`extract_group_windowed` refill loop).
- The spatial-diff running sum, which has a strict sequential dependency and
  cannot be SIMD-parallelized directly.
- Eliminating `packed` entirely (streaming from bit extraction directly to f64)
  would save the ~520 KB allocation and the single remaining read pass over it —
  feasible but requires restructuring the group loop to track diff state inline.
