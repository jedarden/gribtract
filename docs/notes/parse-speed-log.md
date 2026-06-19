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

_Add entries here as optimization attempts are made. Include even failed attempts
so the loop doesn't repeat them._
