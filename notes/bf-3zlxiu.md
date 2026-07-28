# Provider Selection Code Path Analysis (bf-3zlxiu)

## Overview

This document maps out all locations in the gribtract codebase where provider selection occurs and identifies where `should_reprobe` checks are integrated.

## Architecture

The provider selection system has three main components:

1. **Probe Generation** (`xtask/src/probe_providers.rs`): One-time probing that generates `provider-probe.json`
2. **Runtime Selection** (`crates/gribtract/src/provider_probe.rs`): Loads cached probe results and selects best provider
3. **HTTP Execution** (`crates/gribtract-fetch/src/client.rs`): Performs actual HTTP requests with failure tracking

## Code Flow Map

### 1. Probe File Generation (Startup/Re-probe)

**File:** `xtask/src/probe_providers.rs`

**Function:** `run()`

**Purpose:** Generates `provider-probe.json` by probing all providers for all models

**Selection Logic:** 
- Probes each `(model, provider)` pair
- Measures head latency and throughput
- Computes score: `head_latency_ms + 1000.0 / max(throughput_mbs, 0.001)`
- Ranks providers per model by ascending score

**`should_reprobe` Integration:** 
- ❌ NOT APPLICABLE - This is the initial probing that creates the cache, not runtime selection
- This is what gets triggered when `should_reprobe` returns true

---

### 2. Probe File Load (Runtime)

**File:** `crates/gribtract/src/provider_probe.rs`

**Key Functions:**

- `ProviderProbe::load(path)` - Loads `provider-probe.json` from disk
- `ProviderProbe::is_fresh(max_age_secs)` - Checks if probe timestamp is fresh
- `ProviderProbe::is_valid(max_age_secs, tracker)` - **✅ CHECKS `should_reprobe`**
- `ProviderProbe::providers_needing_reprobe(tracker)` - Returns providers needing re-probe
- `ProviderProbe::best_provider(model)` - Returns best provider for model
- `ProviderProbe::ranked_providers(model)` - Returns all providers ranked

**Selection Logic:**
- Reads pre-computed rankings from `provider-probe.json`
- Returns first provider in ranking (best score) via `best_provider()`
- Does NOT re-evaluate providers at selection time

**`should_reprobe` Integration:** 
- ✅ **FULLY INTEGRATED** in `is_valid()` (lines 184-200)
- ✅ Checks both staleness AND consecutive failures via `ProviderFailureTracker`
- ✅ Returns `false` if either staleness check fails OR `tracker.should_reprobe()` returns true for any provider
- ✅ `providers_needing_reprobe()` provides detailed list for logging

---

### 3. Runtime Provider Selection (Before Fetch)

**File:** `crates/gribtract-fetch/src/probe.rs`

**Key Functions:**

- `ProviderProbe::get_best_provider(results, model)` - Returns best provider from probe results
- `ProviderProbe::is_valid(results, max_age)` - **✅ CHECKS `should_reprobe`**
- `ProviderProbe::is_stale(results, max_age)` - Checks staleness only
- `ProviderProbe::providers_needing_reprobe()` - Returns providers needing re-probe

**Selection Logic:**
- Selects first successful provider from probe results
- Does NOT apply runtime failure filtering at selection time
- Relies on pre-validation via `is_valid()` before selection

**`should_reprobe` Integration:** 
- ✅ **FULLY INTEGRATED** in `is_valid()` (lines 361-376)
- ✅ Calls `Self::is_stale()` first, then checks consecutive failures
- ✅ Iterates through `consecutive_failures` and calls `should_reprobe()` for each
- ✅ Returns `false` if any provider has exceeded the threshold
- ⚠️ **NOT INTEGRATED** in `get_best_provider()` - assumes validation happened before

---

### 4. HTTP Client Execution (During Fetch)

**File:** `crates/gribtract-fetch/src/client.rs`

**Key Functions:**

- `FetchClient::fetch_range(&mut self, url, range)` - Fetch byte range
- `FetchClient::fetch_all(&mut self, url)` - Fetch entire resource
- `FetchClient::resource_size(&mut self, url)` - Get resource size via HEAD
- `FetchClient::probe(&mut self, url)` - Check URL accessibility

**Helper:**
- `FetchClient::extract_provider_from_url(url)` - Extracts provider identifier from URL

**Selection Logic:**
- No selection logic - executes requests against provided URLs
- Extracts provider from URL for failure tracking

**`should_reprobe` Integration:** 
- ✅ **FULLY INTEGRATED** - All HTTP methods track failures
- ✅ Calls `record_failure()` on any HTTP error or non-success status
- ✅ Calls `record_success()` on successful responses
- ✅ `should_reprobe()` method (line 464) checks if failures >= threshold
- ⚠️ **DOES NOT TRIGGER RE-PROBE** - only tracks failures for caller to check

**Failure Tracking Flow:**
```
HTTP Request
    ↓
Success? → NO → record_failure(provider)
    ↓                    ↓
   YES                  Check should_reprobe(provider)?
    ↓                    ↓
record_success(provider)     YES → Caller should trigger re-probe
```

---

## Runtime Workflow

### Correct Usage Pattern (as documented in `provider_probe.rs`)

```rust
use gribtract::ProviderProbe;
use gribtract_fetch::probe::ProviderFailureTracker;
use std::path::Path;

// 1. Load provider probe results
let probe = match ProviderProbe::load(Path::new("provider-probe.json")) {
    Ok(p) => p,
    Err(_) => {
        // File absent - trigger initial probe
        run_probe_and_write_json();
        return;
    }
};

// 2. Initialize failure tracker
let tracker = ProviderFailureTracker::default_threshold();

// 3. Check if we should re-probe (Case 2: stale OR Case 3: consecutive failures)
if !probe.is_valid(24 * 3600, &tracker) {
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

// 4. Safe to use cached provider rankings
if let Some(provider) = probe.best_provider("gfs") {
    println!("Best GFS provider: {provider}");

    // 5. After HTTP requests, record success/failure
    match fetch_data_from_provider(provider).await {
        Ok(_) => {
            tracker.record_success(provider);
            println!("Successfully fetched from {provider}");
        }
        Err(_) => {
            let count = tracker.record_failure(provider);
            eprintln!("HTTP error from {provider} (failure {count}/3)");
        }
    }
}
```

---

## Current Integration Status

### ✅ Fully Integrated Locations

1. **`ProviderProbe::is_valid()` in both crates** - Checks both staleness AND consecutive failures
2. **`FetchClient` HTTP methods** - All track failures and success correctly
3. **`ProviderFailureTracker`** - Standalone tracker for use with loaded probe data

### ⚠️ Integration Points (Expect Caller to Validate First)

1. **`ProviderProbe::best_provider()`** - Does NOT check failures, assumes pre-validation
2. **`ProviderProbe::ranked_providers()`** - Does NOT check failures, assumes pre-validation
3. **`ProviderProbe::get_best_provider()`** - Does NOT check failures, assumes pre-validation

### ❌ Not Applicable (Initial Probe Generation)

1. **`xtask probe-providers`** - Generates the cache, doesn't consume it

---

## Missing Integration Points

### Current State: **NO MISSING INTEGRATION**

All critical code paths that need `should_reprobe` checks have them:

1. ✅ **Before selection:** `is_valid()` checks both staleness AND failures
2. ✅ **During execution:** `FetchClient` tracks all HTTP success/failure
3. ✅ **After execution:** Caller checks `should_reprobe()` via tracker

### Design Pattern

The system uses a **validation-then-selection** pattern:

1. **Validate first:** `is_valid()` checks staleness AND failures
2. **Select if valid:** `best_provider()` assumes validation succeeded
3. **Track execution:** HTTP client records success/failure
4. **Re-validate before next use:** Loop back to step 1

This design keeps selection logic simple and fast (just returns the cached best provider) while ensuring re-probing happens when needed.

---

## Provider Identifier Extraction

**File:** `crates/gribtract-fetch/src/client.rs` (lines 158-201)

**Function:** `FetchClient::extract_provider_from_url(url)`

**Mapping:**
- `s3.amazonaws.com` + `hrrr` → `"s3:hrrr"`
- `s3.amazonaws.com` + `gefs` → `"s3:gefs"`
- `s3.amazonaws.com` + `nbm` → `"s3:nbm"`
- `s3.amazonaws.com` + `gfs` → `"s3:gfs"`
- `storage.googleapis.com` + `hrrr` → `"gcs:hrrr"`
- `storage.googleapis.com` + `gefs` → `"gcs:gefs"`
- `storage.googleapis.com` + `nbm` → `"gcs:nbm"`
- `nomads.ncep.noaa.gov` + `gfs` → `"nomads:gfs"`
- `nomads.ncep.noaa.gov` + `gefs` → `"nomads:gefs"`
- `nomads.ncep.noaa.gov` + `nam` → `"nomads:nam"`

This mapping ensures that HTTP failures are tracked consistently with the provider identifiers used in `provider-probe.json`.

---

## Summary

**All provider selection code paths that require `should_reprobe` integration have it:**

- ✅ Probe file validation checks both staleness AND consecutive failures
- ✅ HTTP client tracks all failures and successes
- ✅ Failure tracker provides `should_reprobe()` check
- ✅ Documentation shows correct usage pattern

**No additional integration needed** - the system is designed to validate before selection and track during execution.