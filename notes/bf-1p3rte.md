# Bead bf-1p3rte: Add failure counter data structure to FetchClient

## Summary

Added per-provider failure tracking fields and accessor methods to `FetchClient` in `crates/gribtract-fetch/src/client.rs`.

## Changes

### New Fields Added to FetchClient

1. `consecutive_failures: HashMap<String, u32>` - Maps provider identifier to consecutive failure count
2. `consecutive_failure_threshold: u32` - Threshold for consecutive failures before re-probe trigger (default: 3)

### New Methods

1. `get_failure_count(provider: &str) -> u32` - Get the current consecutive failure count for a provider
2. `set_threshold(threshold: u32)` - Set the consecutive failure threshold
3. `get_threshold() -> u32` - Get the current consecutive failure threshold
4. `record_failure(provider: &str) -> u32` - Record a failure and return the updated count
5. `record_success(provider: &str)` - Reset the failure counter for a provider to zero
6. `should_reprobe(provider: &str) -> bool` - Check if a provider has exceeded the threshold
7. `reset_failures()` - Reset all failure counters

### Updated Constructors

- `FetchClient::new()`, `FetchClient::with_timeout()`, and `FetchClient::from_client()` now initialize the new fields with:
  - Empty HashMap for failures
  - Default threshold of 3

## Tests Added

Added 4 new test functions:
- `test_failure_tracking_basic` - Basic failure tracking workflow
- `test_failure_tracking_multiple_providers` - Independent tracking per provider
- `test_failure_threshold_configurable` - Custom threshold configuration
- `test_reset_failures` - Reset functionality

## Acceptance Criteria Met

- ✅ New fields exist in the relevant struct (HashMap for counter, u32 for threshold)
- ✅ Threshold is configurable with a reasonable default (3)
- ✅ Fields are properly initialized on struct creation
- ✅ cargo test -p gribtract-fetch compiles and all tests pass

## Next Steps

This is a data-only change. Request path logic integration (tracking actual HTTP failures) will be implemented in a future bead.
