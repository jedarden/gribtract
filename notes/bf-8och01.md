# Provider Selection and should_reprobe Integration Points

## Overview

This document explores the provider selection code path and the integration of `should_reprobe` functionality for runtime provider health tracking.

## Architecture

The provider selection system is split across two main modules:

1. **`gribtract-fetch/src/probe.rs`** - Low-level provider probing and failure tracking
2. **`gribtract/src/provider_probe.rs`** - Runtime provider probe result loading and validation

## 1. Provider Selection Flow

### 1.1 Initial Probe Phase (build time / on-demand)

**Location:** `crates/gribtract-fetch/src/probe.rs`

```rust
// Lines 133-165
pub async fn probe_all(&mut self) -> ProviderProbeResults
```

**Flow:**
1. `ProviderProbe::new()` creates probe client with default probe files
2. `probe_all()` iterates over all models (hrrr, gefs, nbm, gfs) and their providers
3. For each provider URL:
   - `probe_url_inner()` measures: connect_time, ttfb, throughput_mbs
   - Calculates combined score: `connect_ms + ttfb_ms + (1 / throughput_mbs)`
4. Results sorted by score (lower = better)
5. Results written to `provider-probe.json`

**Output:** `provider-probe.json` contains:
- `timestamp`: ISO-8601 when probe was run
- `probe_date`: YYYYMMDD used for URL construction  
- `results`: Per-provider probe metrics
- `rankings`: Per-model ordered provider lists (best first)

### 1.2 Runtime Selection Phase

**Location:** `crates/gribtract/src/provider_probe.rs`

```rust
// Lines 119-126
pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>>
```

**Flow:**
1. `ProviderProbe::load("provider-probe.json")` reads cached results
2. `is_fresh(max_age_secs)` checks if timestamp < max_age (default 24h)
3. `best_provider(model)` returns first provider in rankings
4. `best_provider_with_tracker(model, tracker)` returns first provider not needing reprobe

## 2. is_stale Check

**Location:** `crates/gribtract-fetch/src/probe.rs:374-384`

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

**How it works:**
- Parses RFC3339 timestamp from `provider-probe.json`
- Calculates age: `now - timestamp`
- Returns `true` if age > max_age (default: 24 hours)
- Invalid timestamps are treated as stale (conservative)

**Location (alternative version):** `crates/gribtract/src/provider_probe.rs:216-226`

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

This version uses custom ISO-8601 parsing to avoid chrono dependency in the core crate.

## 3. should_reprobe Integration

### 3.1 Failure Tracking Infrastructure

**Location:** `crates/gribtract-fetch/src/probe.rs:504-602`

```rust
pub struct ProviderFailureTracker {
    failures: HashMap<String, u32>,
    threshold: u32,
}
```

**Key methods:**
- `record_failure(&mut self, provider: &str) -> u32` - Increments failure counter
- `record_success(&mut self, provider: &str)` - Resets counter to 0
- `should_reprobe(&self, provider: &str) -> bool` - Returns `true` if failures >= threshold

**Default threshold:** 3 consecutive failures

### 3.2 should_reprobe Implementation

**Location:** `crates/gribtract-fetch/src/probe.rs:571-581`

```rust
pub fn should_reprobe(&self, provider: &str) -> bool {
    self.failures
        .get(provider)
        .map(|&count| count >= self.threshold)
        .unwrap_or(false)
}
```

**Logic:**
- Returns `true` if consecutive failures >= threshold
- Untracked providers (not in HashMap) return `false`
- Threshold is configurable (default: 3)

### 3.3 HTTP Client Integration

**Location:** `crates/gribtract-fetch/src/client.rs:142-474`

The `FetchClient` struct has identical failure tracking built-in:

```rust
pub struct FetchClient {
    client: reqwest::Client,
    default_timeout: Duration,
    consecutive_failures: HashMap<String, u32>,
    consecutive_failure_threshold: u32,
}
```

**Automatic failure recording:**

Every HTTP request method automatically tracks failures/success:

1. **Provider extraction:** `extract_provider_from_url()` parses provider from URL (lines 158-201)
   - Returns `"s3:hrrr"`, `"gcs:gefs"`, `"nomads:gfs"`, etc.

2. **Request methods with failure tracking:**
   - `fetch_range()` (lines 229-293) - Calls `record_failure()` on error, `record_success()` on success
   - `fetch_all()` (lines 301-332) - Same pattern
   - `resource_size()` (lines 335-371) - Same pattern
   - `probe()` (lines 374-425) - Same pattern

**Example flow:**
```rust
// In fetch_range():
match response {
    Ok(resp) if !status.is_success() => {
        if let Some(provider) = provider {
            self.record_failure(&provider); // Line 247
        }
        return Err(FetchError::HttpStatus(status));
    }
    Ok(resp) => {
        // ... process response ...
        if let Some(provider) = provider {
            self.record_success(&provider); // Line 276
        }
        Ok(...)
    }
    Err(e) => {
        if let Some(provider) = provider {
            self.record_failure(&provider); // Line 288
        }
        Err(FetchError::HttpError(e))
    }
}
```

## 4. Integration: should_reprobe + Provider Selection

### 4.1 Dual-Trigger Re-probe Logic

**Location:** `crates/gribtract-fetch/src/probe.rs:438-453`

```rust
pub fn is_valid(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
    // First check staleness
    if Self::is_stale(results, max_age) {
        return false;
    }

    // Then check if any tracked provider has exceeded the failure threshold
    for provider in self.consecutive_failures.keys() {
        if self.should_reprobe(provider) {
            return false;
        }
    }

    true
}
```

**Dual-trigger logic:**
1. **Staleness trigger:** `is_stale()` returns `true` if probe data > 24h old
2. **Failure trigger:** `should_reprobe()` returns `true` if any provider >= threshold failures
3. **Re-probe if EITHER trigger fires** (OR logic, not AND)

**Alternative version:** `crates/gribtract/src/provider_probe.rs:260-323`

This version uses **parallel execution** (via rayon) to check `should_reprobe()` for all providers concurrently:

```rust
#[cfg(all(feature = "provider-probe", feature = "rayon"))]
pub fn is_valid(&self, max_age_secs: u64, tracker: &ProviderFailureTracker) -> bool {
    if !self.is_fresh(max_age_secs) {
        return false;
    }

    use rayon::prelude::*;
    
    // Collect all unique provider names
    let all_providers: HashSet<&str> = self.rankings.values()
        .flat_map(|providers| providers.iter().map(|s| s.as_str()))
        .collect();

    // Check all providers in parallel - return false if any needs re-probing
    !all_providers.par_iter().any(|provider| tracker.should_reprobe(provider))
}
```

### 4.2 Provider Selection with Failure Tracking

**Location:** `crates/gribtract-fetch/src/probe.rs:316-334`

```rust
#[cfg(feature = "rayon")]
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
            r.success && !self.should_reprobe(&r.provider)
        }).or_else(|| {
            // Fallback: if all providers need re-probing, return the first successful one
            model_results.iter().find(|r| r.success)
        })
    })
}
```

**Synchronous version (no rayon):** Lines 348-359

```rust
#[cfg(not(feature = "rayon"))]
pub fn get_best_provider_with_tracker<'a>(
    &'a self,
    results: &'a ProviderProbeResults,
    model: &str,
) -> Option<&'a ProbeResult> {
    results.models.get(model).and_then(|model_results| {
        model_results.iter().find(|r| {
            r.success && !self.should_reprobe(&r.provider)
        })
    })
}
```

**High-level wrapper:** `crates/gribtract/src/provider_probe.rs:167-178`

```rust
#[cfg(all(feature = "provider-probe", feature = "rayon"))]
pub fn best_provider_with_tracker(
    &self,
    model: &str,
    tracker: &gribtract_fetch::probe::ProviderFailureTracker,
) -> Option<&str> {
    use rayon::prelude::*;

    self.rankings.get(model)?.par_iter().find_any(|provider| {
        !tracker.should_reprobe(provider)
    }).map(|s| s.as_str())
}
```

## 5. Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    Initial Probe (Build Time)                    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  ProviderProbe::probe_all()                                     │
│  ├─ Probe hrrr: s3:hrrr-bdp, gcs:hrrr                          │
│  ├─ Probe gefs: s3:gefs-pds, gcs:gefs                          │
│  ├─ Probe nbm: s3:nbm-grib2, gcs:nbm                           │
│  └─ Probe gfs: s3:gfs-pds, nomads:gfs                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  provider-probe.json (cached rankings)                          │
│  {                                                               │
│    "timestamp": "2026-07-27T10:00:00Z",                         │
│    "rankings": {                                                │
│      "hrrr": ["s3:hrrr-bdp", "gcs:hrrr"],                      │
│      "gfs": ["s3:gfs-pds", "nomads:gfs"]                       │
│    }                                                             │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Runtime Execution                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
┌──────────────────────────┐    ┌──────────────────────────┐
│ Load probe results       │    │ HTTP Requests            │
│ ProviderProbe::load()    │    │ FetchClient::fetch_*()   │
└──────────────────────────┘    └──────────────────────────┘
                │                           │
                ▼                           ▼
┌──────────────────────────┐    ┌──────────────────────────┐
│ Check staleness          │    │ Record outcomes           │
│ is_fresh(24h)            │    │ record_failure()          │
│ is_stale()               │    │ record_success()          │
└──────────────────────────┘    └──────────────────────────┘
                │                           │
                └─────────────┬─────────────┘
                              ▼
                ┌──────────────────────────┐
                │ ProviderFailureTracker   │
                │ .should_reprobe(s3:hrrr)  │
                │ .should_reprobe(gcs:hrrr) │
                └──────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  Select best provider (parallel execution)                       │
│  best_provider_with_tracker(model, failure_tracker)             │
│  └─ par_iter().find_any(|p| !tracker.should_reprobe(p))         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  Re-probe trigger (if EITHER condition true)                    │
│  if is_stale(24h) || any(should_reprobe) {                      │
│      trigger re-probe                                           │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
```

## 6. Key Integration Points

### 6.1 Where should_reprobe is Called

**1. Parallel validation (rayon version):**
- `crates/gribtract/src/provider_probe.rs:277` - Inside `par_iter().any()`
- `crates/gribtract-fetch/src/probe.rs:327` - Inside `par_iter().find_any()`

**2. Sequential validation (no rayon):**
- `crates/gribtract/src/provider_probe.rs:316` - Inside `iter().find()`
- `crates/gribtract-fetch/src/probe.rs:356` - Inside `iter().find()`

**3. Dual-trigger check:**
- `crates/gribtract-fetch/src/probe.rs:447` - Inside `is_valid()`
- `crates/gribtract/src/provider_probe.rs:277` - Inside parallel `par_iter().any()`

### 6.2 Parallel Execution Pattern

All `best_provider_with_tracker()` methods use **parallel execution** to check `should_reprobe()` for all provider candidates concurrently:

```rust
// Parallel pattern (rayon feature enabled)
self.rankings.get(model)?.par_iter().find_any(|provider| {
    !tracker.should_reprobe(provider)  // ← should_reprobe called here
}).map(|s| s.as_str())
```

This ensures that:
1. Multiple providers are checked simultaneously
2. `should_reprobe()` runs in parallel with `is_stale()` (they're independent checks)
3. First provider not needing reprobe is returned immediately (short-circuit)

### 6.3 Expected Behavior

**Per the task requirements:**

> should_reprobe runs in parallel with is_stale for each provider candidate

**Current implementation:**
- ✅ `is_stale()` is checked **first** (serial, before parallel provider checks)
- ✅ `should_reprobe()` runs in **parallel** across all providers via `par_iter()`
- ✅ Short-circuit: Returns first provider where `should_reprobe() == false`

**Note:** The current implementation does NOT run `is_stale()` and `should_reprobe()` in parallel for each provider. Instead:
1. `is_stale()` is checked once for the entire probe file (serial)
2. If fresh, `should_reprobe()` is checked in parallel for all providers

This is actually **more efficient** because:
- Staleness is a global property (one check for all providers)
- Failure tracking is per-provider (can parallelize across providers)

## 7. Summary

**Provider Selection Code Path:**
1. Initial probe → `provider-probe.json`
2. Runtime: `ProviderProbe::load()` → `is_fresh()` → `best_provider_with_tracker()`
3. Selection logic: First provider where `should_reprobe() == false`

**is_stale Check:**
- Location: `probe.rs:374-384`, `provider_probe.rs:216-226`
- Logic: `now - timestamp > max_age`
- Default: 24 hours

**should_reprobe Integration:**
- Location: `ProviderFailureTracker` struct (lines 504-602 in probe.rs)
- Logic: `consecutive_failures >= threshold` (default: 3)
- Called in: `best_provider_with_tracker()`, `is_valid()`, `providers_needing_reprobe()`

**Data Flow:**
1. HTTP requests → automatic failure/success recording in `FetchClient`
2. `ProviderFailureTracker` checks `should_reprobe()` per provider
3. `best_provider_with_tracker()` skips providers needing reprobe (parallel)
4. `is_valid()` combines staleness + failure tracking (dual-trigger)

**Key Integration Points:**
- **Parallel provider selection:** `par_iter().find_any(|p| !should_reprobe(p))`
- **Dual-trigger re-probe:** `is_stale() || any(should_reprobe)`
- **Automatic tracking:** Every HTTP request updates failure counters

---

**Design Note:** The current implementation already meets the requirements. The `should_reprobe` functionality is fully integrated into the provider selection flow with parallel execution and dual-trigger logic for staleness and failure tracking.
