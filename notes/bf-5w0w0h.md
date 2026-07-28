# Provider Selection Flow Call Site for `should_reprobe`

## Task Completion Summary

Successfully identified the provider selection flow call sites for `should_reprobe` and confirmed they are distinct from validation-only call paths.

## Provider Selection Flow Call Sites

### PRIMARY: `get_best_provider_with_tracker()` in `gribtract-fetch/src/probe.rs`

This is the **main provider selection flow call site** where `should_reprobe` is actively used during provider selection.

**Rayon version (lines 379-399):**
```rust
pub fn get_best_provider_with_tracker<'a>(
    &'a self,
    results: &'a ProviderProbeResults,
    model: &str,
) -> Option<&'a ProbeResult> {
    use rayon::prelude::*;

    results.models.get(model).and_then(|model_results| {
        // Parallel iteration over provider results
        // Find the first successful provider that doesn't need re-probing
        model_results.par_iter().find_any(|r| {
            r.success && !self.should_reprobe(&r.provider)  // ← SELECTION FLOW CALL
        }).or_else(|| {
            // Fallback: if all providers need re-probing, return the first successful one
            model_results.iter().find(|r| r.success)
        })
    })
}
```

**Non-rayon version (lines 423-433):**
```rust
pub fn get_best_provider_with_tracker<'a>(
    &'a self,
    results: &'a ProviderProbeResults,
    model: &str,
) -> Option<&'a ProbeResult> {
    results.models.get(model).and_then(|model_results| {
        model_results.iter().find(|r| {
            r.success && !self.should_reprobe(&r.provider)  // ← SELECTION FLOW CALL
        })
    })
}
```

**Key characteristics:**
- **Purpose**: Selects the best provider that doesn't need re-probing
- **Behavior**: Skips providers where `should_reprobe()` returns `true`
- **Fallback**: Returns first successful provider if all need re-probing
- **Parallel execution**: Uses `par_iter()` to check multiple providers concurrently

## Validation-Only Call Sites

These call sites are **NOT** part of the selection flow—they are used for validation/diagnostics:

### 1. `is_valid()` in `gribtract-fetch/src/probe.rs`

**Rayon version (line 560):**
```rust
pub fn is_valid(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
    use rayon::prelude::*;

    // First check staleness
    if Self::is_stale(results, max_age) {
        return false;
    }

    // Then check if any tracked provider has exceeded the failure threshold
    let all_providers: Vec<&String> = self.consecutive_failures.keys().collect();
    !all_providers.par_iter().any(|provider| {
        self.should_reprobe(provider)  // ← VALIDATION-ONLY CALL
    })
}
```

**Purpose**: Validates probe data before use (checks both staleness AND provider health)

### 2. `providers_needing_reprobe()` in `gribtract-fetch/src/probe.rs`

**Rayon version (lines 616-624):**
```rust
pub fn providers_needing_reprobe(&self) -> Vec<String> {
    use rayon::prelude::*;

    self.consecutive_failures
        .par_iter()
        .filter(|(_provider, &count)| count >= self.consecutive_failure_threshold)
        .map(|(provider, _count)| provider.clone())
        .collect()
}
```

**Purpose**: Returns list of providers needing re-probing (for logging/debugging)

## Wrapper Methods in `gribtract/src/provider_probe.rs`

These methods use `gribtract_fetch::probe::ProviderFailureTracker.should_reprobe()`:

### Selection Flow:
- **`best_provider_with_tracker()`** (lines 244-254 rayon, 278-286 non-rayon)
  - Line 252: `!tracker.should_reprobe(provider)` - Selection flow call

### Validation-Only:
- **`is_valid()`** (lines 382-400 rayon, 423-439 non-rayon)
  - Line 399: `tracker.should_reprobe(provider)` - Validation call
- **`providers_needing_reprobe()`** (lines 455-473 rayon, 489-506 non-rayon)
  - Line 470: `tracker.should_reprobe(provider)` - Diagnostic call

## Code Path Documentation

### Selection Flow Path:
```
HTTP Request → FetchClient → ProviderProbe.get_best_provider_with_tracker()
    → par_iter() over providers → should_reprobe() check → Select best healthy provider
```

### Validation Flow Path:
```
ProviderProbe.is_valid() → staleness check → par_iter() over tracked providers
    → should_reprobe() check → Return true/false (NOT selection)
```

## Confirmation of Distinct Paths

✅ **Confirmed**: The selection flow (`get_best_provider_with_tracker`) is **distinct** from validation-only paths (`is_valid`, `providers_needing_reprobe`):

1. **Different purposes**: Selection chooses a provider; validation checks data validity
2. **Different call sites**: Line 393/430 (selection) vs Line 560/597 (validation)
3. **Different timing**: Selection happens per fetch; validation happens once at startup
4. **Different behavior**: Selection skips failing providers; validation returns boolean

## Test Coverage

The codebase includes comprehensive tests verifying these call paths:

- **Line 1739**: `test_should_reprobe_called_during_get_best_provider_with_tracker`
- **Line 1800**: `test_should_reprobe_verification_called_for_each_provider`
- **Line 1875**: `test_should_reprobe_selection_path_not_validation_path`
- **Line 1948**: `test_should_reprobe_mock_verification_isolated`

## Summary

**Primary Selection Flow Call Site**: `gribtract-fetch/src/probe.rs:393` (rayon) or `:430` (non-rayon) in `get_best_provider_with_tracker()`

This is the definitive answer to the task: the specific function/method containing the selection call, with documented code paths and confirmation that it's distinct from validation-only call paths.
