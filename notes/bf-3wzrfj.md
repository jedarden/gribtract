# Added Comprehensive Tests for should_reprobe Integration (bf-3wzrfj)

## Summary

Added comprehensive tests to verify that the `should_reprobe` integration works correctly in the `is_valid` method in `crates/gribtract-fetch/src/probe.rs`.

## Test Cases Added

1. **test_is_valid_returns_false_when_should_reprobe_returns_true**
   - Verifies that `is_valid` returns false when `should_reprobe` returns true
   - Tests that consecutive failures properly invalidate fresh probe results

2. **test_is_valid_returns_true_when_should_reprobe_false_and_fresh**
   - Verifies that `is_valid` returns true when both conditions are met:
     - `should_reprobe` returns false
     - File is fresh (not stale)
   - Tests normal operation with failures below threshold

3. **test_is_valid_returns_false_when_stale_regardless_of_should_reprobe**
   - Verifies that staleness trumps everything
   - Tests both cases: stale with no failures AND stale with failures
   - Ensures staleness check has priority

4. **test_is_valid_handles_multiple_providers_correctly**
   - Verifies that `is_valid` correctly handles multiple providers with different failure states
   - Tests that ANY provider exceeding threshold makes results invalid
   - Tests that results become valid again after all failing providers reset
   - Comprehensive multi-provider scenario testing

5. **test_integration_triggers_reprobe_on_consecutive_http_errors**
   - Tests the full lifecycle of consecutive HTTP error tracking
   - Simulates consecutive errors and verifies counter increments
   - Verifies that threshold crossing triggers reprobe
   - Tests that successful requests reset the counter
   - End-to-end integration test

6. **test_is_valid_dual_trigger_logic**
   - Tests the dual-trigger logic (stale OR should_reprobe)
   - Covers all combinations:
     - Fresh, no failures → valid
     - Stale, no failures → invalid (staleness trigger)
     - Fresh, with failures → invalid (should_reprobe trigger)
     - Stale, with failures → invalid (both triggers)
   - Verifies the OR condition works correctly

7. **test_should_reprobe_boundary_conditions**
   - Tests boundary conditions for different threshold values
   - Verifies that threshold-1 failures don't trigger
   - Verifies that exactly threshold failures do trigger
   - Tests thresholds from 1 to 5

8. **test_is_valid_with_no_tracked_providers**
   - Tests behavior when no providers are tracked
   - Verifies `should_reprobe` returns false for untracked providers
   - Ensures `is_valid` only depends on staleness in this case

## Test Coverage

All new tests cover both staleness and consecutive failure conditions, verifying the dual-trigger logic:

- **Staleness trigger**: Tests verify stale results invalidate regardless of failure state
- **Consecutive failure trigger**: Tests verify that exceeding threshold invalidates fresh results
- **Integration**: Tests verify the OR condition works correctly (either trigger invalidates)

## Running the Tests

```bash
cargo test -p gribtract-fetch --features probe --lib probe::
```

All 28 tests pass (1 network-dependent test is ignored).

## Files Modified

- `crates/gribtract-fetch/src/probe.rs`: Added 8 new comprehensive test functions
