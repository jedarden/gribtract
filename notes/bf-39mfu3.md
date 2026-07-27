# Bead bf-39mfu3: Provider Probe JSON Loading and Staleness Check Analysis

## Summary

Located and documented the complete flow for provider-probe.json loading and staleness checking.

## Key Files

### 1. Main Loading Function
**File**: `/home/coding/gribtract/crates/gribtract/src/provider_probe.rs`
**Function**: `ProviderProbe::load()` (lines 122-126)

```rust
pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let probe: Self = serde_json::from_str(&contents)?;
    Ok(probe)
}
```

This is the primary runtime function used to load the provider-probe.json file.

### 2. Staleness Check Functions

#### Primary Implementation
**File**: `/home/coding/gribtract/crates/gribtract/src/provider_probe.rs`
**Function**: `is_fresh()` (lines 143-153)

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

This checks if the probe timestamp is less than `max_age_secs` old. Returns `false` for parse failures (conservative).

#### Legacy Implementation
**File**: `/home/coding/gribtract/crates/gribtract-fetch/src/probe.rs`
**Function**: `is_stale()` (lines 297-307)

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

This is the older version using chrono. The newer `is_fresh()` implementation is preferred.

#### Combined Validity Check
**File**: `/home/coding/gribtract/crates/gribtract/src/provider_probe.rs`
**Function**: `is_valid()` (lines 184-200)

```rust
pub fn is_valid(&self, max_age_secs: u64, tracker: &gribtract_fetch::probe::ProviderFailureTracker) -> bool {
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

This combines both staleness checking and consecutive failure tracking.

## Current Flow

### 1. Probe Generation (Build-time/Manual)
- **Command**: `xtask probe-providers`
- **File**: `/home/coding/gribtract/xtask/src/probe_providers.rs`
- **Output**: `provider-probe.json` in workspace root

The xtask:
1. Probes each provider for each model (GFS, HRRR, NBM)
2. Measures head latency and throughput
3. Ranks providers by combined score
4. Writes results with timestamp to `provider-probe.json`

### 2. Runtime Loading
- **Function**: `ProviderProbe::load(path)`
- **Returns**: `Result<ProviderProbe, Box<dyn std::error::Error>>`
- **Error handling**: If file is missing, invalid JSON, or malformed

### 3. Re-probe Triggers
There are three cases that trigger re-probing:

1. **File absent**: `ProviderProbe::load` returns `Err`
2. **File stale**: `is_fresh()` returns `false` (timestamp > 24h old)
3. **Consecutive HTTP errors**: `ProviderFailureTracker::should_reprobe()` returns `true`

## Integration with should_reprobe

The `should_reprobe` integration point is already prepared in the `ProviderProbe` structure:

### ProviderFailureTracker
**File**: `/home/coding/gribtract/crates/gribtract-fetch/src/probe.rs`
**Struct**: `ProviderFailureTracker` (lines 418-486)

Key methods:
- `record_failure(&mut self, provider: &str) -> u32`: Increments failure count
- `record_success(&mut self, provider: &str)`: Resets failure count to 0
- `should_reprobe(&self, provider: &str) -> bool`: Checks if threshold exceeded
- `failure_count(&self, provider: &str) -> u32`: Gets current count

### Example Integration Flow
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
    let needing = probe.providers_needing_reprobe(&tracker);
    
    if !probe.is_fresh(24 * 3600) {
        eprintln!("provider-probe.json is stale (>24h old), triggering re-probe...");
    }
    
    if !needing.is_empty() {
        eprintln!("Re-probe triggered by consecutive failures: {}", needing.join(", "));
    }
    
    run_probe_and_write_json();
    return;
}

// Safe to use cached provider rankings
if let Some(provider) = probe.best_provider("gfs") {
    // After HTTP requests, record success/failure
    match fetch_data_from_provider(provider).await {
        Ok(_) => tracker.record_success(provider),
        Err(_) => {
            let count = tracker.record_failure(provider);
            eprintln!("HTTP error from {provider} (failure {count}/3)");
        }
    }
}
```

## Ready for Integration

The current code structure is well-prepared for integrating `should_reprobe` alongside the existing staleness logic:

1. **Loading**: Single, clean loading function in `provider_probe.rs`
2. **Staleness check**: Well-defined `is_fresh()` method
3. **Failure tracking**: Complete `ProviderFailureTracker` API
4. **Combined check**: `is_valid()` method that checks both conditions
5. **Logging support**: `providers_needing_reprobe()` for diagnostics

The architecture supports both time-based (staleness) and event-based (consecutive failures) re-probe triggers working together seamlessly.
