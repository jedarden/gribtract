# Unit Test Design for `should_reprobe` Call Verification

## Task Overview

Design the approach for mocking and verifying that `should_reprobe` is called during provider selection, as opposed to validation-only paths.

## Context

From `bf-5w0w0h` findings, there are two distinct call paths:

### Selection Flow Call Site
- **Location**: `gribtract-fetch/src/probe.rs:393` (rayon) or `:430` (non-rayon)
- **Function**: `get_best_provider_with_tracker()`
- **Purpose**: Actively select the best provider that doesn't need re-probing
- **Behavior**: Skips providers where `should_reprobe()` returns `true`

### Validation-Only Call Sites
- **`is_valid()`** (line 560/597): Validates probe data before use
- **`providers_needing_reprobe()`** (line 616/624): Returns list for diagnostics

## Mocking Strategy Analysis

### Option 1: Manual Wrapper/Spy Pattern ✅ **CHOSEN**

**Implementation**: `TrackedProviderProbe` wrapper

**Pros**:
- ✅ Lightweight - no external dependencies
- ✅ Delegates to real implementation - tests actual behavior
- ✅ Simple call tracking with `RefCell<Vec<String>>`
- ✅ Easy to verify call order and arguments
- ✅ Works with both rayon and non-rayon code paths
- ✅ No proc-macro overhead
- ✅ Clear intent - tracking is explicit

**Cons**:
- ❌ Manual wrapper maintenance
- ❌ Limited to call tracking (can't mock return values independently)
- ❌ Requires wrapper for each method tested

**Verdict**: **Best fit** for this use case. The current implementation uses this approach successfully in `test_should_reprobe_selection.rs`.

### Option 2: Mockall (Automated Mocking)

**Implementation**: `#[automock]` on trait

**Pros**:
- ✅ Automated mock generation
- ✅ Can mock return values independently
- ✅ Expectation-based verification
- ✅ Wide ecosystem adoption

**Cons**:
- ❌ Requires extracting `should_reprobe` into a trait
- ❌ Tests mock behavior, not real implementation
- ❌ Mocks may drift from real implementation
- ❌ Proc-macro compilation overhead
- ❌ Overkill for simple call verification
- ❌ Complex setup for `ProviderProbe` (multiple internal fields)

**Verdict**: **Not recommended** - the real implementation is simple and testing it directly is more valuable than mocking it.

### Option 3: Call Counter Pattern

**Implementation**: `Arc<Mutex<AtomicUsize>>` counter

**Pros**:
- ✅ Minimal overhead
- ✅ Thread-safe for parallel execution
- ✅ Simple to implement

**Cons**:
- ❌ Only tracks count, not which providers were checked
- ❌ No argument verification
- ❌ Can't distinguish between selection and validation calls
- ❌ No call order information

**Verdict**: **Insufficient** - we need to verify which providers are checked, not just that calls happened.

### Option 4: Instrumentation/Metric Logging

**Implementation**: Add metrics to `should_reprobe` itself

**Pros**:
- ✅ Production observability
- ✅ No test-only code paths
- ✅ Thread-safe by design

**Cons**:
- ❌ Production code for testing purposes
- ❌ Overhead in production
- ❌ Hard to isolate test-specific verification
- ❌ Requires metrics infrastructure

**Verdict**: **Overkill** - this is a unit test concern, not a production metrics need.

## Test Structure Design

### Current Implementation Structure

The existing test suite in `test_should_reprobe_selection.rs` follows this structure:

```rust
// 1. Test Wrapper
struct TrackedProviderProbe {
    probe: ProviderProbe,
    should_reprobe_calls: RefCell<Vec<String>>,
}

// 2. Test Scenarios
#[test] fn test_should_reprobe_called_during_selection_flow()
#[test] fn test_should_reprobe_selection_vs_validation_paths()
#[test] fn test_should_reprobe_called_for_each_provider_during_selection()
// ... more tests
```

### Test Verification Points

1. **Call Verification**
   - `tracked.call_count() > 0` - at least one call happened
   - `tracked.was_should_reprobe_called("provider_a")` - specific provider checked
   - `tracked.call_count() >= 2` - multiple providers checked

2. **Behavioral Verification**
   - Selection skips providers where `should_reprobe` returns `true`
   - Validation fails when `should_reprobe` returns `true`
   - Reset affects subsequent selection

3. **Path Distinction**
   - `test_should_reprobe_selection_vs_validation_paths` explicitly tests both paths
   - Validation path: `is_valid()` returns `false` when `should_reprobe` is `true`
   - Selection path: providers are skipped when `should_reprobe` is `true`

## Distinguishing Selection from Validation Calls

### Approach 1: Separate Test Functions ✅ **USED**

Each path has its own test:

```rust
// Selection path test
#[test]
fn test_should_reprobe_called_during_selection_flow() {
    // Calls tracked_should_reprobe during manual selection loop
}

// Validation path test
#[test]
fn test_should_reprobe_selection_vs_validation_paths() {
    // Calls probe.is_valid() which internally calls should_reprobe
    // Also tests selection separately
}
```

### Approach 2: Call Context Tracking

Hypothetical enhancement to track call context:

```rust
enum CallContext {
    Selection,
    Validation,
}

struct ContextAwareTrackedProbe {
    // ... existing fields
    call_contexts: RefCell<Vec<(String, CallContext)>>,
}

impl ContextAwareTrackedProbe {
    fn tracked_should_reprobe_selection(&self, provider: &str) -> bool {
        self.call_contexts.borrow_mut().push((provider.to_string(), CallContext::Selection));
        self.probe.should_reprobe(provider)
    }

    fn tracked_should_reprobe_validation(&self, provider: &str) -> bool {
        self.call_contexts.borrow_mut().push((provider.to_string(), CallContext::Validation));
        self.probe.should_reprobe(provider)
    }
}
```

**Verdict**: **Not needed** - the current approach of separate test functions is clearer and sufficient.

### Approach 3: Control Flow Verification

Test that selection logic actually uses the result:

```rust
// Test that selection skips providers when should_reprobe returns true
for provider_result in providers {
    let needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
    if !needs_reprobe {
        selected_provider = Some(provider_result.provider.clone());
        break;
    }
}
assert_eq!(selected_provider, Some("nomads:gfs".to_string()));
```

**Verdict**: **Essential** - already implemented in existing tests.

## Recommendations

### 1. Continue with Manual Wrapper Pattern

The current `TrackedProviderProbe` approach is optimal for this use case:

- Lightweight
- Tests real implementation
- Clear intent
- Sufficient verification capability

### 2. Test Organization Improvements

Consider adding these test categories:

```rust
// Selection-specific tests
mod selection {
    #[test] fn test_should_reprobe_called_during_selection()
    #[test] fn test_should_reprobe_skips_failing_providers()
    #[test] fn test_should_reprobe_all_providers_failing()
}

// Validation-specific tests  
mod validation {
    #[test] fn test_should_reprobe_affects_is_valid()
    #[test] fn test_should_reprobe_in_validation_context()
}

// Integration tests
mod integration {
    #[test] fn test_selection_vs_validation_independence()
    #[test] fn test_should_reprobe_state_changes()
}
```

### 3. Additional Test Coverage

Consider adding:

1. **Thread safety tests** for parallel execution edge cases
2. **Performance tests** to verify call overhead is minimal
3. **Regression tests** for specific bugs found in production

### 4. Documentation

Add doc comments explaining test intent:

```rust
/// Test that should_reprobe is called during provider selection flow.
/// 
/// This verifies the selection path (get_best_provider_with_tracker)
/// as opposed to the validation path (is_valid).
/// 
/// # Selection Flow
/// - Input: ProviderProbeResults with multiple providers
/// - Action: Selection loop calls should_reprobe for each provider
/// - Expected: skips providers where should_reprobe returns true
/// - Output: First provider that doesn't need reprobe
#[test]
fn test_should_reprobe_called_during_selection_flow() {
    // ...
}
```

## Alternative Approaches Not Recommended

### ❌ Mockall
Would require trait extraction and tests mock behavior instead of real implementation.

### ❌ Dependency Injection
Over-engineering for a simple method call verification.

### ❌ Code Instrumentation
Adding test-only code paths to production code.

### ❌ Static Analysis
Cannot verify runtime behavior and call patterns.

## Conclusion

**Recommended Strategy**: Continue with the manual wrapper/spy pattern (`TrackedProviderProbe`).

**Why**:
1. Tests real implementation behavior
2. Minimal overhead and dependencies
3. Clear intent and maintainability
4. Sufficient verification capability
5. Works with both rayon and non-rayon code paths

**Test Coverage**: The existing test suite in `test_should_reprobe_selection.rs` already implements this strategy effectively and covers the key scenarios.

**Next Steps**: Consider test organization improvements and additional edge case coverage as outlined in recommendations.
