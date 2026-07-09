# Fix: Differential Coverage Report agreement_pct() Bug (Bead bf-1uqf)

## Problem
The differential coverage report showed `Agreement: 6/4 (150.0%)` - a mathematically impossible ratio.

## Root Cause
- JPEG2000 fixtures were skipped BEFORE `fixtures_total` was incremented
- But they WERE counted in `fixtures_no_golden` 
- `agreement_pct()` computed: `comparable = fixtures_total - fixtures_no_golden`
- This double-subtracted the skipped fixtures, undercounting comparable

## Solution
1. Moved `fixtures_total += 1` to count all inline fixtures (before feature-skip check)
2. Updated `agreement_pct()` and `print_report()` to subtract `fixtures_skipped_feature`
3. Updated stale doc comment (now shows 100% agreement, not 0%)
4. Updated unit test to match new behavior

## Result
Now correctly shows: `Agreement: 6/6 (100.0%)` with `6 comparable` (9 total - 1 no-golden - 2 skipped-feature)

## Verification
- cargo test -p gribtract --test differential -- --nocapture (default features) passes
- cargo test -p gribtract --test differential --features jpeg2000 -- --nocapture passes
- All unit tests in gribtract-testutil pass

## Commit
dcd9b86 "fix(diff): correct agreement_pct() calculation by subtracting fixtures_skipped_feature"
