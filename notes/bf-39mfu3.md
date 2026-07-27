# Provider-Probe.json Loading and Staleness Check Analysis

## Task Summary
Located and analyzed where `provider-probe.json` is loaded and where the `is_stale` check happens in the runtime provider-selection code.

## Key Findings

### 1. Where provider-probe.json is Loaded

There are **two main locations** where the file is loaded:

#### A. Runtime Loading (Primary Consumer)
**File:** `crates/gribtract/src/provider_probe.rs`  
**Function:** `ProviderProbe::load()` (lines 119-126)

```rust
pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let probe: Self = serde_json::from_str(&contents)?;
    Ok(probe)
}
```

This is the main runtime entry point. It:
- Reads the JSON file from disk
- Deserializes it into a `ProviderProbe` struct
- Returns `Err` if file is missing, unreadable, or malformed

#### B. Internal Library Loading
**File:** `crates/gribtract-fetch/src/probe.rs`  
**Function:** `ProviderProbe::load_results()` (lines 290-295)

This is used internally by the fetch library for probe operations.

---

### 2. Where is_stale Check Happens

The staleness check exists in **THREE places**:

#### A. In gribtract-fetch::probe module
**File:** `crates/gribtract-fetch/src/probe.rs`  
**Function:** `ProviderProbe::is_stale()` (lines 298-307)

```rust
pub fn is_stale(results: &ProviderProbeResults, max_age: Duration) -> bool {
    match chrono::DateTime::parse_from_rfc3339(&results.timestamp) {
        Ok(timestamp) => {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(timestamp);
            age.to_std().unwrap_or(Duration::ZERO) > max_age
        }
        Err(_) => true, // Invalid timestamp means stale
    }
}
```

This checks the `ProviderProbeResults` struct (the older format used by fetch library).

#### B. In gribtract provider_probe module
**File:** `crates/gribtract/src/provider_probe.rs`  
**Function:** `ProviderProbe::is_fresh()` (lines 139-153)

```rust
pub fn is_fresh(&self, max_age_secs: u64) -> bool {
    parse_iso8601_secs(&self.timestamp)
        .map(|probe_secs| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now.saturating_sub(probe_secs) < max_age_secs
        })
        .unwrap_or(false)
}
```

This is the **main runtime staleness check** that uses the newer `ProviderProbe` struct format. Note:
- It's named `is_fresh()` (inverse of `is_stale()`)
- Uses a custom ISO-8601 parser (no external chrono dependency in gribtract crate)
- Conservative: returns `false` (stale) if timestamp parsing fails

#### C. Integrated check (combines staleness + failures)
**File:** `crates/gribtract/src/provider_probe.rs`  
**Function:** `ProviderProbe::is_valid()` (lines 183-200)

```rust
pub fn is_valid(&self, max_age_secs: u64, tracker: &ProviderFailureTracker) -> bool {
    // First check staleness
    if !self.is_fresh(max_age_secs) {
        return false;
    }

    // Then check if any tracked provider has exceeded the failure threshold
    for (_model, providers) in &self.rankings {
        for provider in providers {
            if tracker.should_reprobe(provider) {
                return false;
            }
        }
    }

    true
}
```

This is the **comprehensive validity check** that combines both triggers.

---

### 3. Current Re-Probe Trigger Flow

The complete flow involves **THREE re-probe triggers**:

1. **File absent**: `ProviderProbe::load()` returns `Err`
2. **File stale**: `is_fresh()` returns `false` (timestamp > max_age)
3. **Consecutive failures**: `ProviderFailureTracker::should_reprobe()` returns `true`

#### Where should_reprobe() is defined

**A. In gribtract-fetch::probe::ProviderFailureTracker**
**File:** `crates/gribtract-fetch/src/probe.rs`  
**Function:** `ProviderFailureTracker::should_reprobe()` (lines 460-465)

```rust
pub fn should_reprobe(&self, provider: &str) -> bool {
    self.failures
        .get(provider)
        .map(|&count| count >= self.threshold)
        .unwrap_or(false)
}
```

**B. In gribtract-fetch::client::FetchClient**
**File:** `crates/gribtract-fetch/src/client.rs`  
**Function:** `FetchClient::should_reprobe()` (lines 464-469)

This is identical logic but embedded in the HTTP client.

#### How failures are tracked

The `ProviderFailureTracker` struct:
- Tracks consecutive failures per provider (HashMap<String, u32>)
- Default threshold: 3 consecutive failures
- `record_failure()`: increments counter
- `record_success()`: resets counter to zero
- `should_reprobe()`: returns true when count >= threshold

#### Where failures are recorded

**File:** `crates/gribtract-fetch/src/client.rs`  
In `FetchClient` HTTP methods:
- `fetch_range()` - lines 246-247 (failure), 275-277 (success)
- `fetch_all()` - lines 311-312 (failure), 318-320 (success)
- `resource_size()` - lines 344-348 (failure), 353-355 (success)
- `probe()` - lines 384-387 (failure), 406-408 (success)

Provider is extracted via `extract_provider_from_url()` (lines 162-201).

---

### 4. Helper Function: check_needs_reprobe()

**File:** `crates/gribtract-fetch/src/probe.rs`  
**Function:** `check_needs_reprobe()` (lines 518-555)

This integrates BOTH triggers (staleness AND failures):

```rust
pub fn check_needs_reprobe(
    probe_path: &std::path::Path,
    max_age: Duration,
    tracker: &ProviderFailureTracker,
) -> Result<bool, Box<dyn std::error::Error>> {
    use gribtract::ProviderProbe as GribtractProviderProbe;

    // Try to load the probe file
    let probe = match GribtractProviderProbe::load(probe_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("provider-probe.json load failed: {}", e);
            return Ok(true); // File absent/malformed - needs re-probe
        }
    };

    // Check staleness
    let max_age_secs = max_age.as_secs() as u64;
    let is_stale = !probe.is_fresh(max_age_secs);

    if is_stale {
        eprintln!("provider-probe.json is stale (>{}s old), triggering re-probe", max_age_secs);
    }

    // Check consecutive failures
    let needing_reprobe = probe.providers_needing_reprobe(tracker);
    let has_failures = !needing_reprobe.is_empty();

    if has_failures {
        eprintln!("Re-probe triggered by consecutive failures: {}",
            needing_reprobe.join(", "));
    }

    Ok(is_stale || has_failures)
}
```

**Returns:**
- `Ok(true)` - Re-probe needed (stale OR failures)
- `Ok(false)` - No re-probe needed (fresh AND no failures)
- `Err(_)` - Error loading file (but internally returns Ok(true) for missing/malformed)

---

### 5. Recommended Runtime Workflow (from docs)

**File:** `crates/gribtract/src/provider_probe.rs`  
**Lines 24-80 (module-level example)**

```rust
// Load provider probe results
let probe = match ProviderProbe::load(Path::new("provider-probe.json")) {
    Ok(p) => p,
    Err(_) => {
        // Case 1: File absent — trigger re-probe
        eprintln!("provider-probe.json not found, running initial probe...");
        run_probe_and_write_json();
        return;
    }
};

// Initialize failure tracker (threshold = 3 consecutive errors)
let tracker = ProviderFailureTracker::default_threshold();

// Check if we should re-probe (Case 2 OR Case 3)
// is_valid() checks both: file staleness AND consecutive failures
if !probe.is_valid(24 * 3600, &tracker) {
    // Get specific providers that need re-probing for logging
    let needing = probe.providers_needing_reprobe(&tracker);

    if !probe.is_fresh(24 * 3600) {
        eprintln!("provider-probe.json is stale (>24h old), triggering re-probe...");
    }

    if !needing.is_empty() {
        eprintln!("Re-probe triggered by consecutive failures: {}",
            needing.join(", "));
    }

    run_probe_and_write_json();
    return;
}

// Safe to use cached provider rankings
if let Some(provider) = probe.best_provider("gfs") {
    match fetch_data_from_provider(provider).await {
        Ok(_) => {
            tracker.record_success(provider);
        }
        Err(_) => {
            let count = tracker.record_failure(provider);
            eprintln!("HTTP error from {provider} (failure {count}/3)");
        }
    }
}
```

---

## Architecture Summary

```
┌─────────────────────────────────────────────────────────────┐
│                    Runtime Application                        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
        ┌──────────────────────────────────────────┐
        │  ProviderProbe::load("provider-probe.json") │
        └──────────────────────────────────────────┘
                            │
                            ├─→ Err (file absent/malformed)
                            │       └─→ Trigger re-probe
                            │
                            ▼
        ┌──────────────────────────────────────────┐
        │     ProviderProbe + FailureTracker       │
        └──────────────────────────────────────────┘
                            │
                            ▼
        ┌──────────────────────────────────────────┐
        │  probe.is_valid(max_age, &tracker)?      │
        └──────────────────────────────────────────┘
                   │                    │
            false: STALE            true: VALID
                   │                    │
                   ▼                    ▼
        ┌──────────────────┐    ┌──────────────────┐
        │ Trigger re-probe │    │ Use cached       │
        │ (write new JSON) │    │ providers        │
        └──────────────────┘    └──────────────────┘

        During HTTP requests:
        ┌──────────────────────────────────────────┐
        │ FetchClient (records success/failure)     │
        └──────────────────────────────────────────┘
                           │
                ┌──────────┴──────────┐
                ▼                     ▼
        Success: record_success()  Failure: record_failure()
                │                     │
                │                     ▼
                │         Check if count >= threshold
                │                     │
                └─────────────────────┤
                                    ▼
                        If threshold exceeded:
                        Next is_valid() check will fail
```

---

## Key Functions for Integration

| Function | File | Purpose |
|----------|------|---------|
| `ProviderProbe::load()` | `gribtract/src/provider_probe.rs:119-126` | Load JSON file |
| `ProviderProbe::is_fresh()` | `gribtract/src/provider_probe.rs:139-153` | Check staleness |
| `ProviderProbe::is_valid()` | `gribtract/src/provider_probe.rs:183-200` | Combined check |
| `ProviderProbe::providers_needing_reprobe()` | `gribtract/src/provider_probe.rs:212-230` | List failing providers |
| `ProviderFailureTracker::should_reprobe()` | `gribtract-fetch/src/probe.rs:460-465` | Check failure threshold |
| `check_needs_reprobe()` | `gribtract-fetch/src/probe.rs:518-555` | Integrated helper |
| `FetchClient::record_failure()` | `gribtract-fetch/src/client.rs:448-452` | Record HTTP failure |
| `FetchClient::record_success()` | `gribtract-fetch/src/client.rs:457-459` | Record HTTP success |

---

## Data Structures

### ProviderProbe (main runtime struct)
```rust
pub struct ProviderProbe {
    pub timestamp: String,        // ISO-8601 timestamp
    pub probe_date: String,       // YYYYMMDD
    pub results: Vec<ProviderResult>,
    pub rankings: HashMap<String, Vec<String>>,  // model → [providers]
}
```

### ProviderFailureTracker
```rust
pub struct ProviderFailureTracker {
    failures: HashMap<String, u32>,  // provider → consecutive count
    threshold: u32,                   // default: 3
}
```

### ProviderProbeResults (older fetch library format)
```rust
pub struct ProviderProbeResults {
    pub models: HashMap<String, Vec<ProbeResult>>,
    pub timestamp: String,
    pub git_sha: Option<String>,
}
```

---

## Test Coverage

All components have comprehensive test coverage:
- `is_fresh()` tests with various timestamps
- `ProviderFailureTracker` tests with threshold boundaries
- `is_valid()` tests combining staleness and failures
- `check_needs_reprobe()` integration tests
- `FetchClient` failure tracking tests

See individual files for test implementations.

---

## Conclusion

The current architecture provides a **dual-trigger system** for re-probing:

1. **Time-based**: File staleness (24-hour TTL by default)
2. **Failure-based**: Consecutive HTTP errors per provider (threshold = 3)

Both triggers are integrated through:
- `ProviderProbe::is_valid()` - combines checks for validity
- `check_needs_reprobe()` - helper function for integrated checking
- `providers_needing_reprobe()` - diagnostic helper for logging

The system is well-documented with comprehensive examples in the module-level documentation of `gribtract/src/provider_probe.rs`.
