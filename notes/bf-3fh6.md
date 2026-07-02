# Bead bf-3fh6: Per-width specialized DRT=3 extractors - Bug Verification

## Summary

This bead revisited the per-width specialized extractors for DRT=3 (from Attempt 6) and discovered a **critical correctness bug**. The specialized extractors were producing incorrect results and have been removed.

## What was discovered

### Critical Bug Found
The test `extract_group_specialized_matches_generic` **FAILED**, revealing that the specialized w=4 extractor produces incorrect results:

**Test failure (data=[0,1,2,3,...], skip=0, w=4, gref=13):**
- Expected (correct): `[13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13]`
- Actual (buggy): `[13, 13, 13, 14, 13, 15, 13, 16, 13, 17, 13, 18, 13, 19, 13, 20, 13, 21, 13, 22]`

### Root Cause
The specialized extractors broke the **left-alignment invariant** used by the generic windowed extractor:

**Generic (correct):**
```rust
buf |= (data[byte_pos] as u64) << (56 - buf_bits);  // MSB-aligned
```

**Specialized w=4 (buggy):**
```rust
let mut buf = if byte_pos < data.len() { data[byte_pos] as u64 } else { 0 };  // LSB-aligned!
```

This bug affects all specialized extractors (w=4, 8, 12, 16) because they share the same incorrect initialization pattern.

### Why This Matters
Per the plan rules: **"Correctness gates speed: 100% differential agreement must hold; a faster path that changes any value is a regression."**

The specialized extractors were **unusable in their current state** and would have caused data corruption if used in production.

## Resolution

### Code Removed
- `extract_group_w4` (lines 1370-1408)
- `extract_group_w8` (lines 1410-1453)
- `extract_group_w12` (lines 1455-1498)
- `extract_group_w16` (lines 1500-1543)
- `extract_group_specialized` dispatcher (lines 1545-1563)
- Test `extract_group_specialized_matches_generic` (lines 1885-1910)

### Code Updated
- `decode_drt3` now calls `extract_group_windowed` directly (verified correct)

### Performance Impact
**After removing buggy specialized extractors:**
- DRT=3 (template 5.3): 120-124 MB/s (avg ~122 MB/s)
- Agreement: **100%** (all tests pass)

**No performance loss:** The generic extractor is already optimal (Attempts 4+5 gave +38-40% improvement over baseline).

## Why Specialized Extractors Won't Help

Even if the bug were fixed, specialized extractors provide **no measurable benefit**:

1. **Generic is already optimized:** The while-loop refill runs at most `ceil(w/8) ≤ 2` iterations per element for w≤16
2. **Branch prediction:** The predictor learns the refill pattern quickly within each group
3. **Dispatch overhead:** The `match w` dispatcher and code duplication offset any micro-optimizations
4. **Wrong bottleneck:** After Attempts 4+5, bit extraction is no longer the bottleneck — spatial-differencing and f64 scaling dominate

## Conclusion

The specialized extractors had a correctness bug that caused data corruption. They have been removed. The generic `extract_group_windowed` extractor remains:
- **Correct:** 100% test agreement, verified by extensive tests
- **Fast:** 120-124 MB/s (38-40% faster than pre-windowed baseline)
- **Maintainable:** Single code path, no duplication

## Acceptance criteria met

- ✓ New Attempt 7 entry in `docs/notes/parse-speed-log.md` with bug analysis
- ✓ Differential suite passes at **100% agreement** (verified after removing buggy code)
- ✓ Code reverted (specialized extractors removed) with detailed log entry explaining the correctness bug
- ✓ Performance unchanged (generic extractor was already optimal)

## References

- Parse speed log: `docs/notes/parse-speed-log.md` Attempt 7 (bug verification)
- Related beads: bf-3thh (spatial-diff + f64-scale merge), bf-l70y (windowed u64 extractor)
