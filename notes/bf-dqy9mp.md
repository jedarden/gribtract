# should_reprobe Implementation Exploration

## Task: Explore should_reprobe implementation and call sites

Date: 2026-07-27

## Summary

The `should_reprobe` function is a key component of gribtract's provider health management system. It determines when a specific cloud storage provider needs to be re-probed due to consecutive HTTP failures, working alongside staleness checks to form a dual-trigger re-probe mechanism.

## Function Location and Signature

### Primary Implementation (ProviderProbe)
- **Location**: `crates/gribtract-fetch/src/probe.rs:482`
- **Struct**: `ProviderProbe`
- **Signature**: `pub fn should_reprobe(&self, provider: &str) -> bool`

```rust
pub fn should_reprobe(&self, provider: &str) -> bool {
    self.consecutive_failures
        .get(provider)
        .map(|&count| count >= self.consecutive_failure_threshold)
        .unwrap_or(false)
}
```

### Standalone Implementation (ProviderFailureTracker)
- **Location**: `crates/gribtract-fetch/src/probe.rs:751`
- **Struct**: `ProviderFailureTracker`
- **Signature**: `pub fn should_reprobe(&self, provider: &str) -> bool`

```rust
pub fn should_reprobe(&self, provider: &str) -> bool {
    self.failures
        .get(provider)
        .map(|&count| count >= self.threshold)
        .unwrap_or(false)
}
```

### Client Wrapper
- **Location**: `crates/gribtract-fetch/src/client.rs:464`
- **Struct**: `FetchClient` (delegates to internal ProviderFailureTracker logic)
- **Signature**: `pub fn should_reprobe(&self, provider: &str) -> bool`

### Public API
- **Location**: `crates/gribtract/src/provider_probe.rs`
- Provides documentation and examples of how to use `should_reprobe` in the public API

## Key Implementation Details

- **Default threshold**: 3 consecutive failures
- **State tracking**: Uses `HashMap<String, u32>` to track consecutive failures per provider
- **Return value**: `true` when failure count >= threshold, `false` otherwise
- **Reset behavior**: `record_success()` resets counter to 0; `record_failure()` increments counter

## Call Sites

### 1. Selection Path (Provider Selection Logic)
**Location**: `crates/gribtract-fetch/src/probe.rs:393`

```rust
// INTEGRATION: This is where should_reprobe() is called during provider selection.
model_results.par_iter().find_any(|r| {
    r.success && !self.should_reprobe(&r.provider)
})
```

**Purpose**: Filters out providers that need re-probing during provider selection. If a provider needs re-probing (returns `true`), it's excluded from selection and the next best provider is chosen.

### 2. Selection Path (with ProviderFailureTracker)
**Location**: `crates/gribtract-fetch/src/probe.rs:430`

```rust
model_results.iter().find(|r| {
    r.success && !self.should_reprobe(&r.provider)
})
```

**Purpose**: Same filtering logic but using the standalone `ProviderFailureTracker` struct.

### 3. Validation Path (is_valid method)
**Location**: `crates/gribtract-fetch/src/probe.rs:545` and `:585`

```rust
pub fn is_valid(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
    // First check staleness
    if Self::is_stale(results, max_age) {
        return false;
    }
    
    // Then check if any tracked provider has exceeded the failure threshold
    // INTEGRATION: This is where should_reprobe() is called to implement the
    // failure-tracking half of the dual-trigger re-probe logic.
    // ...
}
```

**Purpose**: Validates that cached probe results are still usable. Returns `false` if ANY provider needs re-probing.

### 4. Provider Filtering (providers_needing_reprobe)
**Location**: `crates/gribtract-fetch/src/probe.rs` (parallel filtering)

```rust
all_providers.par_iter().any(|provider| tracker.should_reprobe(provider))
```

**Purpose**: Collects all providers that need re-probing for logging/debugging.

## Validation vs Selection Call Paths

### Validation Path (Pre-check)
- **When**: Before using cached provider rankings
- **Method**: `is_valid()` 
- **Behavior**: Checks if ANY provider needs re-probing → triggers full re-probe if true
- **Return**: Boolean (valid/invalid)
- **Use case**: "Should I re-run the entire probe process?"

### Selection Path (Runtime provider choice)
- **When**: Choosing which provider to use for a data fetch
- **Method**: `get_best_provider_with_tracker()` and related methods
- **Behavior**: Excludes providers needing re-probe → selects next best healthy provider
- **Return**: `Option<&ProbeResult>` (best available provider)
- **Use case**: "Which provider should I use right now?"

## Key Relationship: Dual-Trigger Re-probe Logic

The `should_reprobe` function implements the "consecutive failures" trigger in a dual-trigger system:

1. **Staleness trigger**: Time-based (file age > max_age)
2. **Consecutive failures trigger**: Error-based (`should_reprobe() == true`)

Either trigger can cause a re-probe, but they serve different purposes:
- Staleness ensures rankings are current
- Consecutive failures ensure unhealthy providers are avoided

## Test Coverage

Comprehensive test coverage exists in:
- **Unit tests**: `crates/gribtract-fetch/src/probe.rs` (lines 780+)
- **Integration tests**: `crates/gribtract-fetch/tests/test_should_reprobe_selection.rs`
  - Tracks actual `should_reprobe` calls during selection
  - Verifies call paths for validation vs selection
  - Tests parallel execution and edge cases

## Conclusion

The `should_reprobe` function is well-integrated into gribtract's provider health management, with clear separation between validation (pre-check) and selection (runtime) call paths. The implementation is straightforward but effectively prevents cascading failures by degrading unhealthy providers before they cause widespread issues.
