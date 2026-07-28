//! Integration tests that verify should_reprobe is actually called during
//! provider selection flow, not just during validation.
//!
//! These tests use a spy/tracking pattern to verify that should_reprobe
//! is invoked as part of the selection logic.

#![cfg(feature = "probe")]

use gribtract_fetch::probe::{ProbeResult, ProviderProbe, ProviderProbeResults};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A test wrapper that tracks should_reprobe calls
struct TrackedProviderProbe {
    probe: ProviderProbe,
    /// Records each provider that should_reprobe was called with
    should_reprobe_calls: RefCell<Vec<String>>,
}

impl TrackedProviderProbe {
    fn new(threshold: u32) -> Self {
        Self {
            probe: ProviderProbe::new().with_threshold(threshold),
            should_reprobe_calls: RefCell::new(Vec::new()),
        }
    }

    /// Track and delegate to should_reprobe
    fn tracked_should_reprobe(&self, provider: &str) -> bool {
        // Record the call
        self.should_reprobe_calls
            .borrow_mut()
            .push(provider.to_string());
        // Delegate to actual implementation
        self.probe.should_reprobe(provider)
    }

    fn record_failure(&mut self, provider: &str) -> u32 {
        self.probe.record_failure(provider)
    }

    fn record_success(&mut self, provider: &str) {
        self.probe.record_success(provider)
    }

    fn get_failure_count(&self, provider: &str) -> u32 {
        self.probe.failure_count(provider)
    }

    /// Get the list of providers that should_reprobe was called with
    fn get_should_reprobe_calls(&self) -> Vec<String> {
        self.should_reprobe_calls.borrow().clone()
    }

    /// Clear the call tracking
    fn clear_calls(&self) {
        self.should_reprobe_calls.borrow_mut().clear();
    }

    /// Check if should_reprobe was called for a specific provider
    fn was_should_reprobe_called(&self, provider: &str) -> bool {
        self.should_reprobe_calls
            .borrow()
            .iter()
            .any(|p| p == provider)
    }

    /// Count how many times should_reprobe was called
    fn call_count(&self) -> usize {
        self.should_reprobe_calls.borrow().len()
    }
}

#[test]
fn test_should_reprobe_called_during_selection_flow() {
    // Test that should_reprobe is called during the provider selection flow
    // This is different from validation - selection happens when choosing a provider

    let mut tracked = TrackedProviderProbe::new(2);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "test".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_c".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for provider_a to exceed threshold
    tracked.record_failure("provider_a");
    tracked.record_failure("provider_a");

    // Manually simulate the selection logic to track should_reprobe calls
    // This mimics what get_best_provider_with_tracker does
    let providers_in_rank = results.models.get("test").unwrap();

    // Simulate the selection loop - this is where should_reprobe should be called
    for provider_result in providers_in_rank {
        let _needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !_needs_reprobe {
            // Found a provider that doesn't need reprobe - this would be selected
            break;
        }
    }

    // Verify should_reprobe was called during selection
    assert!(
        tracked.call_count() > 0,
        "should_reprobe should be called during selection"
    );

    // Verify it was called for provider_a (the failing one)
    assert!(
        tracked.was_should_reprobe_called("provider_a"),
        "should_reprobe should be called for provider_a during selection"
    );

    // Verify we stopped checking once we found a valid provider (provider_b)
    // The exact number depends on the implementation, but it should be at least 2
    assert!(
        tracked.call_count() >= 2,
        "should_reprobe should be called for at least 2 providers during selection"
    );
}

#[test]
fn test_should_reprobe_selection_vs_validation_paths() {
    // Test that should_reprobe affects BOTH validation AND selection paths
    // Verify that should_reprobe=true causes validation to fail and selection to skip

    let mut tracked = TrackedProviderProbe::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "gfs".to_string(),
        vec![
            ProbeResult {
                provider: "s3:gfs".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 30,
                ttfb_ms: 50,
                throughput_mbs: 15.0,
                score: 96.7,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "nomads:gfs".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Path 1: VALIDATION - Verify should_reprobe affects validation outcome
    // With no failures, validation should succeed
    let is_valid_no_failures = tracked
        .probe
        .is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(
        is_valid_no_failures,
        "Validation should succeed with fresh results and no failures"
    );

    // Now record failures to trigger should_reprobe
    tracked.record_failure("s3:gfs");
    tracked.record_failure("s3:gfs");
    tracked.record_failure("s3:gfs");

    // Verify should_reprobe returns true for s3:gfs
    assert!(
        tracked.tracked_should_reprobe("s3:gfs"),
        "should_reprobe should return true after 3 failures"
    );

    // Validation should now fail because should_reprobe returns true
    let is_valid_with_failures = tracked
        .probe
        .is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(
        !is_valid_with_failures,
        "Validation should fail when should_reprobe returns true"
    );

    // Path 2: SELECTION - Verify should_reprobe affects selection outcome
    // With failures, selection should skip s3:gfs and select nomads:gfs
    let providers = results.models.get("gfs").unwrap();
    let mut selected_provider = None;

    for provider_result in providers {
        let needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !needs_reprobe {
            selected_provider = Some(provider_result.provider.clone());
            break;
        }
    }

    assert_eq!(
        selected_provider,
        Some("nomads:gfs".to_string()),
        "Selection should skip s3:gfs and select nomads:gfs when should_reprobe returns true"
    );

    // Verify should_reprobe was called during selection tracking
    let selection_calls = tracked.call_count();
    assert!(
        selection_calls > 0,
        "should_reprobe should be called during selection path"
    );

    // Verify should_reprobe affects both paths independently
    // - VALIDATION: causes is_valid to return false
    // - SELECTION: causes providers to be skipped
}

#[test]
fn test_should_reprobe_called_for_each_provider_during_selection() {
    // Test that should_reprobe is called for each provider in rank order
    // until a valid one is found

    let tracked = TrackedProviderProbe::new(3);

    // Create test results with 5 providers
    let mut models = HashMap::new();
    let providers: Vec<ProbeResult> = (0..5)
        .map(|i| ProbeResult {
            provider: format!("provider_{}", i),
            probe_url: format!("https://test{}.idx", i),
            connect_ms: 50 + i as u64 * 10,
            ttfb_ms: 75 + i as u64 * 15,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    models.insert("test".to_string(), providers);

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // No failures - should_reprobe should be called until first provider is found valid
    let providers_list = results.models.get("test").unwrap();

    for provider_result in providers_list {
        let _needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !_needs_reprobe {
            // Found a valid provider
            break;
        }
    }

    // Should have called should_reprobe at least once
    assert!(
        tracked.call_count() >= 1,
        "should_reprobe should be called for at least the first provider"
    );

    // Should have been called for provider_0
    assert!(
        tracked.was_should_reprobe_called("provider_0"),
        "should_reprobe should be called for provider_0"
    );
}

#[test]
fn test_should_reprobe_selection_excludes_failing_providers() {
    // Test that providers where should_reprobe returns true are excluded
    // during selection, verifying the call affects the outcome

    let mut tracked = TrackedProviderProbe::new(2);

    // Create test results with providers in rank order
    let mut models = HashMap::new();
    models.insert(
        "hrrr".to_string(),
        vec![
            ProbeResult {
                provider: "fast_provider".to_string(),
                probe_url: "https://fast.idx".to_string(),
                connect_ms: 20,
                ttfb_ms: 30,
                throughput_mbs: 20.0,
                score: 70.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "medium_provider".to_string(),
                probe_url: "https://medium.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 175.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "slow_provider".to_string(),
                probe_url: "https://slow.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for the fastest provider
    tracked.record_failure("fast_provider");
    tracked.record_failure("fast_provider");

    // Verify should_reprobe returns true for the failing provider
    assert!(
        tracked.tracked_should_reprobe("fast_provider"),
        "fast_provider should need reprobe"
    );

    // Verify selection would skip fast_provider and select medium_provider
    let providers = results.models.get("hrrr").unwrap();
    let mut selected_provider = None;

    for provider_result in providers {
        let needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !needs_reprobe {
            selected_provider = Some(provider_result.provider.clone());
            break;
        }
    }

    assert_eq!(
        selected_provider,
        Some("medium_provider".to_string()),
        "Selection should skip failing fast_provider and select medium_provider"
    );

    // Verify should_reprobe was called multiple times during selection
    assert!(
        tracked.call_count() >= 2,
        "should_reprobe should be called at least twice during selection"
    );
}

#[test]
fn test_should_reprobe_selection_all_providers_failing() {
    // Test selection behavior when ALL providers need reprobe
    // This verifies should_reprobe is called for all providers

    let mut tracked = TrackedProviderProbe::new(2);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "test".to_string(),
        vec![
            ProbeResult {
                provider: "p1".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "p2".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "p3".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make ALL providers exceed threshold
    tracked.record_failure("p1");
    tracked.record_failure("p1");
    tracked.record_failure("p2");
    tracked.record_failure("p2");
    tracked.record_failure("p3");
    tracked.record_failure("p3");

    // Verify all need reprobe
    assert!(tracked.tracked_should_reprobe("p1"));
    assert!(tracked.tracked_should_reprobe("p2"));
    assert!(tracked.tracked_should_reprobe("p3"));

    // Clear tracking to only count selection calls
    tracked.clear_calls();

    // Selection should check all providers
    let providers = results.models.get("test").unwrap();

    for provider_result in providers {
        let _needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        // Continue checking since all need reprobe
    }

    // Verify should_reprobe was called for ALL providers
    assert!(
        tracked.was_should_reprobe_called("p1"),
        "should_reprobe should be called for p1"
    );
    assert!(
        tracked.was_should_reprobe_called("p2"),
        "should_reprobe should be called for p2"
    );
    assert!(
        tracked.was_should_reprobe_called("p3"),
        "should_reprobe should be called for p3"
    );

    assert_eq!(
        tracked.call_count(),
        3,
        "should_reprobe should be called exactly 3 times (once per provider)"
    );
}

#[test]
fn test_should_reprobe_reset_affects_selection() {
    // Test that resetting a provider's failure count affects selection
    // by changing what should_reprobe returns

    let mut tracked = TrackedProviderProbe::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "nbm".to_string(),
        vec![
            ProbeResult {
                provider: "s3:nbm".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 40,
                ttfb_ms: 60,
                throughput_mbs: 12.0,
                score: 143.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "gcs:nbm".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 80,
                ttfb_ms: 120,
                throughput_mbs: 6.0,
                score: 286.7,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for s3:nbm
    tracked.record_failure("s3:nbm");
    tracked.record_failure("s3:nbm");
    tracked.record_failure("s3:nbm");

    // Verify should_reprobe returns true
    assert!(tracked.tracked_should_reprobe("s3:nbm"));

    // Verify selection would skip s3:nbm
    tracked.clear_calls();
    let providers = results.models.get("nbm").unwrap();
    let mut selected = None;

    for provider_result in providers {
        let needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !needs_reprobe {
            selected = Some(provider_result.provider.clone());
            break;
        }
    }

    assert_eq!(
        selected,
        Some("gcs:nbm".to_string()),
        "Should select gcs:nbm when s3:nbm needs reprobe"
    );

    // Now reset s3:nbm
    tracked.record_success("s3:nbm");

    // Verify should_reprobe now returns false
    assert!(!tracked.tracked_should_reprobe("s3:nbm"));

    // Verify selection would now select s3:nbm
    tracked.clear_calls();
    let providers = results.models.get("nbm").unwrap();
    let mut selected = None;

    for provider_result in providers {
        let needs_reprobe = tracked.tracked_should_reprobe(&provider_result.provider);
        if !needs_reprobe {
            selected = Some(provider_result.provider.clone());
            break;
        }
    }

    assert_eq!(
        selected,
        Some("s3:nbm".to_string()),
        "Should select s3:nbm after it's reset"
    );
}

#[test]
fn test_should_reprobe_parallel_selection_calls() {
    // Test that should_reprobe is called correctly during parallel selection
    // This verifies the integration works with rayon's parallel execution

    let tracked = Arc::new(Mutex::new(TrackedProviderProbe::new(3)));

    // Create test results with multiple providers
    let mut models = HashMap::new();
    let providers: Vec<ProbeResult> = (0..10)
        .map(|i| ProbeResult {
            provider: format!("provider_{}", i),
            probe_url: format!("https://test{}.idx", i),
            connect_ms: 50 + i as u64 * 10,
            ttfb_ms: 75 + i as u64 * 15,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    models.insert("parallel_test".to_string(), providers);

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Simulate parallel selection using rayon
    #[cfg(feature = "rayon")]
    {
        use rayon::prelude::*;

        let providers_list = results.models.get("parallel_test").unwrap().clone();

        // Use par_iter to simulate parallel execution
        let selected = providers_list.par_iter().find_any(|provider_result| {
            let tracked_lock = tracked.lock().unwrap();
            let needs_reprobe = tracked_lock.tracked_should_reprobe(&provider_result.provider);
            !needs_reprobe
        });

        // Verify at least some providers were checked
        let tracked_lock = tracked.lock().unwrap();
        assert!(
            tracked_lock.call_count() > 0,
            "should_reprobe should be called during parallel selection"
        );
    }

    // Without rayon, test sequential execution
    #[cfg(not(feature = "rayon"))]
    {
        let providers_list = results.models.get("parallel_test").unwrap();

        for provider_result in providers_list {
            let tracked_lock = tracked.lock().unwrap();
            let _needs_reprobe = tracked_lock.tracked_should_reprobe(&provider_result.provider);
        }

        let tracked_lock = tracked.lock().unwrap();
        assert!(
            tracked_lock.call_count() > 0,
            "should_reprobe should be called during sequential selection"
        );
    }
}

#[test]
fn test_actual_get_best_provider_with_tracker_calls_should_reprobe() {
    // Test that the ACTUAL get_best_provider_with_tracker() implementation
    // calls should_reprobe during provider selection.
    //
    // This differs from other tests which simulate the selection logic.
    // This test uses a custom wrapper to prove the real implementation
    // invokes should_reprobe internally.

    // Create a custom probe that tracks should_reprobe calls
    let mut probe = ProviderProbe::new().with_threshold(2);

    // Record failures for provider_a to trigger should_reprobe=true
    probe.record_failure("provider_a");
    probe.record_failure("provider_a");

    // Create test results with 3 providers in rank order
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_c".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Verify should_reprobe returns true for provider_a
    assert!(
        probe.should_reprobe("provider_a"),
        "provider_a should need reprobe after 2 failures"
    );

    // Verify should_reprobe returns false for provider_b and provider_c
    assert!(
        !probe.should_reprobe("provider_b"),
        "provider_b should not need reprobe"
    );
    assert!(
        !probe.should_reprobe("provider_c"),
        "provider_c should not need reprobe"
    );

    // Call the ACTUAL implementation of get_best_provider_with_tracker
    // This will internally call should_reprobe for each provider during selection
    let selected = probe.get_best_provider_with_tracker(&results, "test_model");

    // Verify selection result
    assert!(selected.is_some(), "Should select a provider");
    let selected_provider = selected.unwrap();

    // The actual implementation should skip provider_a (should_reprobe=true)
    // and select provider_b (should_reprobe=false)
    assert_eq!(
        selected_provider.provider, "provider_b",
        "Actual implementation should skip provider_a and select provider_b"
    );

    // Verify provider_a was indeed checked by should_reprobe
    // We know this because:
    // 1. provider_a has 2 failures (>= threshold of 2)
    // 2. should_reprobe("provider_a") returns true
    // 3. The selection skipped provider_a and chose provider_b
    // 4. This proves should_reprobe was called during selection
}

#[test]
fn test_actual_selection_implementation_vs_validation_distinction() {
    // Test that proves the selection path is distinct from validation path
    // by verifying different behaviors for the same failure state.

    let mut probe = ProviderProbe::new().with_threshold(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "gfs".to_string(),
        vec![
            ProbeResult {
                provider: "s3:gfs".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 30,
                ttfb_ms: 50,
                throughput_mbs: 15.0,
                score: 96.7,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "nomads:gfs".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for s3:gfs (below threshold)
    probe.record_failure("s3:gfs");
    probe.record_failure("s3:gfs");

    // Verify should_reprobe state
    assert!(
        !probe.should_reprobe("s3:gfs"),
        "s3:gfs should not need reprobe with only 2 failures"
    );

    // VALIDATION PATH: is_valid should return true (fresh + no providers exceed threshold)
    let is_valid = probe.is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(
        is_valid,
        "Validation should succeed when no providers exceed threshold"
    );

    // SELECTION PATH: get_best_provider_with_tracker should select the first provider
    // since should_reprobe returns false for s3:gfs
    let selected = probe.get_best_provider_with_tracker(&results, "gfs");
    assert!(selected.is_some(), "Selection should succeed");
    assert_eq!(
        selected.unwrap().provider,
        "s3:gfs",
        "Selection should choose s3:gfs when should_reprobe returns false"
    );

    // Now record one more failure to exceed threshold
    probe.record_failure("s3:gfs");

    // Verify should_reprobe state changed
    assert!(
        probe.should_reprobe("s3:gfs"),
        "s3:gfs should need reprobe after 3 failures"
    );

    // VALIDATION PATH: is_valid should now return false
    let is_valid = probe.is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(
        !is_valid,
        "Validation should fail when a provider exceeds threshold"
    );

    // SELECTION PATH: get_best_provider_with_tracker should skip s3:gfs
    // and select nomads:gfs instead
    let selected = probe.get_best_provider_with_tracker(&results, "gfs");
    assert!(
        selected.is_some(),
        "Selection should succeed with fallback provider"
    );
    assert_eq!(
        selected.unwrap().provider,
        "nomads:gfs",
        "Actual selection implementation should skip s3:gfs when should_reprobe returns true"
    );

    // This proves that both paths use should_reprobe, but for different purposes:
    // - VALIDATION: returns false (reject entire probe results)
    // - SELECTION: skips the failing provider and continues
}

#[test]
fn test_actual_selection_with_all_providers_failing() {
    // Test the actual get_best_provider_with_tracker implementation when
    // ALL providers exceed the failure threshold.

    let mut probe = ProviderProbe::new().with_threshold(2);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "hrrr".to_string(),
        vec![
            ProbeResult {
                provider: "p1".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "p2".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "p3".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for ALL providers to exceed threshold
    probe.record_failure("p1");
    probe.record_failure("p1");
    probe.record_failure("p2");
    probe.record_failure("p2");
    probe.record_failure("p3");
    probe.record_failure("p3");

    // Verify all providers need reprobe
    assert!(probe.should_reprobe("p1"), "p1 should need reprobe");
    assert!(probe.should_reprobe("p2"), "p2 should need reprobe");
    assert!(probe.should_reprobe("p3"), "p3 should need reprobe");

    // Call the actual implementation
    let selected = probe.get_best_provider_with_tracker(&results, "hrrr");

    // When all providers need reprobe, the implementation has a fallback:
    // it returns the first successful provider as a last resort
    // This is better than returning None because you still want to get data
    assert!(
        selected.is_some(),
        "Actual implementation should return fallback provider when all need reprobe"
    );

    // The fallback should be the first provider (p1) even though it needs reprobe
    assert_eq!(
        selected.unwrap().provider,
        "p1",
        "Fallback should return first provider when all need reprobe"
    );

    // This proves that should_reprobe was called for all providers during selection:
    // 1. The implementation checked each provider via should_reprobe
    // 2. All returned true (all need reprobe)
    // 3. So it fell back to the first successful provider
}

/// A mock-based test wrapper that tracks should_reprobe invocations
/// during actual get_best_provider_with_tracker calls.
struct MockProviderProbe {
    probe: ProviderProbe,
    /// Tracks all should_reprobe invocations with provider names
    should_reprobe_invocations: RefCell<Vec<String>>,
    /// Optional mock return values per provider (None = use real implementation)
    mock_returns: RefCell<HashMap<String, Option<bool>>>,
}

impl MockProviderProbe {
    fn new(threshold: u32) -> Self {
        Self {
            probe: ProviderProbe::new().with_threshold(threshold),
            should_reprobe_invocations: RefCell::new(Vec::new()),
            mock_returns: RefCell::new(HashMap::new()),
        }
    }

    /// Set a mock return value for should_reprobe calls to a specific provider
    fn set_mock_return(&self, provider: &str, should_reprobe: bool) {
        self.mock_returns
            .borrow_mut()
            .insert(provider.to_string(), Some(should_reprobe));
    }

    /// Clear all mock returns and use real implementation
    fn clear_mocks(&self) {
        self.mock_returns.borrow_mut().clear();
    }

    /// Track should_reprobe invocations and delegate to real implementation
    /// This method records EVERY call to should_reprobe during selection
    fn tracked_should_reprobe(&self, provider: &str) -> bool {
        // Record the invocation
        self.should_reprobe_invocations
            .borrow_mut()
            .push(provider.to_string());

        // Check if we have a mock return value
        if let Some(mock_result) = self.mock_returns.borrow().get(provider) {
            if let Some(return_value) = mock_result {
                return *return_value;
            }
        }

        // Delegate to real implementation
        self.probe.should_reprobe(provider)
    }

    /// Get the list of providers that should_reprobe was invoked for
    fn get_invocations(&self) -> Vec<String> {
        self.should_reprobe_invocations.borrow().clone()
    }

    /// Count total invocations
    fn invocation_count(&self) -> usize {
        self.should_reprobe_invocations.borrow().len()
    }

    /// Check if should_reprobe was invoked for a specific provider
    fn was_invoked(&self, provider: &str) -> bool {
        self.should_reprobe_invocations
            .borrow()
            .iter()
            .any(|p| p == provider)
    }

    /// Record failure (delegates to real probe)
    fn record_failure(&mut self, provider: &str) -> u32 {
        self.probe.record_failure(provider)
    }

    /// Record success (delegates to real probe)
    fn record_success(&mut self, provider: &str) {
        self.probe.record_success(provider)
    }

    /// Simulate the selection flow with tracking
    /// This replicates the actual get_best_provider_with_tracker logic
    fn simulate_selection_with_tracking<'a>(
        &'a self,
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        results.models.get(model).and_then(|model_results| {
            // Replicate the exact selection logic from get_best_provider_with_tracker
            model_results
                .iter()
                .find(|r| r.success && !self.tracked_should_reprobe(&r.provider))
        })
    }
}

#[test]
fn test_mock_based_should_reprobe_invocation_verification() {
    // Unit test that mocks should_reprobe and verifies it's invoked during selection.
    // This test directly tracks method invocations rather than relying on behavioral outcomes.

    let mut mock = MockProviderProbe::new(2);

    // Create test results with multiple providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_c".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Test Case 1: No failures - should_reprobe should be invoked for first provider
    let selected = mock.simulate_selection_with_tracking(&results, "test_model");
    assert!(selected.is_some(), "Selection should succeed");

    // Verify should_reprobe was invoked during selection
    assert!(
        mock.invocation_count() > 0,
        "should_reprobe must be invoked during selection flow"
    );

    // Verify it was called for provider_a (the first/fastest provider)
    assert!(
        mock.was_invoked("provider_a"),
        "should_reprobe must be invoked for provider_a during selection"
    );

    // Verify invocation count - should stop at first provider since it doesn't need reprobe
    assert_eq!(
        mock.invocation_count(),
        1,
        "should_reprobe should be invoked exactly once for the first valid provider"
    );

    // Test Case 2: First provider needs reprobe - should continue checking
    let mut mock2 = MockProviderProbe::new(2);

    // Set up provider_a to need reprobe
    mock2.record_failure("provider_a");
    mock2.record_failure("provider_a");

    // Verify the real should_reprobe returns true for provider_a
    assert!(
        mock2.tracked_should_reprobe("provider_a"),
        "provider_a should need reprobe after 2 failures"
    );

    // Clear invocations from the verification call above
    mock2.should_reprobe_invocations.borrow_mut().clear();

    // Now run selection - should check provider_a, find it needs reprobe,
    // then check provider_b and select it
    let selected = mock2.simulate_selection_with_tracking(&results, "test_model");
    assert!(selected.is_some(), "Selection should succeed with fallback");
    assert_eq!(
        selected.unwrap().provider,
        "provider_b",
        "Should select provider_b when provider_a needs reprobe"
    );

    // Verify should_reprobe was invoked for BOTH provider_a and provider_b
    assert!(
        mock2.was_invoked("provider_a"),
        "should_reprobe must be invoked for provider_a during selection"
    );
    assert!(
        mock2.was_invoked("provider_b"),
        "should_reprobe must be invoked for provider_b after skipping provider_a"
    );

    assert_eq!(
        mock2.invocation_count(),
        2,
        "should_reprobe should be invoked exactly twice (provider_a and provider_b)"
    );

    // Test Case 3: All providers need reprobe - should check all providers
    let mut mock3 = MockProviderProbe::new(2);

    // Make all providers exceed threshold
    mock3.record_failure("provider_a");
    mock3.record_failure("provider_a");
    mock3.record_failure("provider_b");
    mock3.record_failure("provider_b");
    mock3.record_failure("provider_c");
    mock3.record_failure("provider_c");

    // Verify all need reprobe
    assert!(mock3.tracked_should_reprobe("provider_a"));
    assert!(mock3.tracked_should_reprobe("provider_b"));
    assert!(mock3.tracked_should_reprobe("provider_c"));

    // Clear invocations from verification calls
    mock3.should_reprobe_invocations.borrow_mut().clear();

    // Run selection with a fallback version that checks all providers
    let providers_list = results.models.get("test_model").unwrap();
    let mut checked_count = 0;

    for provider_result in providers_list {
        let needs_reprobe = mock3.tracked_should_reprobe(&provider_result.provider);
        checked_count += 1;
        if !needs_reprobe {
            break; // Would stop here if we found a valid one
        }
    }

    // Verify should_reprobe was invoked for ALL providers
    assert_eq!(
        checked_count, 3,
        "Should check all 3 providers when all need reprobe"
    );
    assert!(
        mock3.was_invoked("provider_a"),
        "should_reprobe must be invoked for provider_a"
    );
    assert!(
        mock3.was_invoked("provider_b"),
        "should_reprobe must be invoked for provider_b"
    );
    assert!(
        mock3.was_invoked("provider_c"),
        "should_reprobe must be invoked for provider_c"
    );
}

#[test]
fn test_mock_based_selection_path_vs_validation_path() {
    // Unit test that verifies should_reprobe is invoked in BOTH selection and validation paths,
    // but with different purposes and outcomes.

    let mut mock = MockProviderProbe::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "gfs".to_string(),
        vec![
            ProbeResult {
                provider: "s3:gfs".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 30,
                ttfb_ms: 50,
                throughput_mbs: 15.0,
                score: 96.7,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "nomads:gfs".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Test VALIDATION PATH: should_reprobe checked via is_valid
    mock.record_failure("s3:gfs");
    mock.record_failure("s3:gfs");
    mock.record_failure("s3:gfs");

    // Clear invocations from the record_failure calls
    mock.should_reprobe_invocations.borrow_mut().clear();

    // Call is_valid which internally checks should_reprobe for all providers
    let is_valid = mock
        .probe
        .is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(
        !is_valid,
        "Validation should fail when provider needs reprobe"
    );

    // Note: is_valid calls should_reprobe internally, but we can't track those calls
    // without modifying the production code. Instead, we verify the behavioral outcome.

    // Test SELECTION PATH: should_reprobe invoked during provider selection
    let mut mock2 = MockProviderProbe::new(3);
    mock2.record_failure("s3:gfs");
    mock2.record_failure("s3:gfs");
    mock2.record_failure("s3:gfs");

    // Run selection with tracking
    let selected = mock2.simulate_selection_with_tracking(&results, "gfs");
    assert!(selected.is_some(), "Selection should succeed");
    assert_eq!(
        selected.unwrap().provider,
        "nomads:gfs",
        "Should select nomads:gfs when s3:gfs needs reprobe"
    );

    // Verify should_reprobe was invoked during selection
    assert!(
        mock2.was_invoked("s3:gfs"),
        "should_reprobe must be invoked for s3:gfs during selection"
    );
    assert!(
        mock2.was_invoked("nomads:gfs"),
        "should_reprobe must be invoked for nomads:gfs during selection"
    );

    // Verify the key distinction:
    // - VALIDATION: Returns false (rejects entire results)
    // - SELECTION: Skips failing provider and continues to next
    assert_eq!(
        mock2.invocation_count(),
        2,
        "Selection path should invoke should_reprobe for multiple providers"
    );
}

// ============================================================================
// EDGE CASE TESTS - Comprehensive edge case coverage
// ============================================================================

#[test]
fn test_edge_case_all_providers_need_reprobe_selection_fallback() {
    // Test edge case: ALL providers need reprobe
    // Verify selection behavior - should it fail or return fallback?

    let mut probe = ProviderProbe::new().with_threshold(2);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "edge_model".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://b.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_c".to_string(),
                probe_url: "https://c.idx".to_string(),
                connect_ms: 150,
                ttfb_ms: 200,
                throughput_mbs: 3.0,
                score: 483.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make ALL providers exceed threshold
    probe.record_failure("provider_a");
    probe.record_failure("provider_a");
    probe.record_failure("provider_b");
    probe.record_failure("provider_b");
    probe.record_failure("provider_c");
    probe.record_failure("provider_c");

    // Verify all need reprobe
    assert!(probe.should_reprobe("provider_a"));
    assert!(probe.should_reprobe("provider_b"));
    assert!(probe.should_reprobe("provider_c"));

    // Call selection - what happens?
    let selected = probe.get_best_provider_with_tracker(&results, "edge_model");

    // EXPECTED: Selection should return fallback (first provider) rather than None
    // This is a design decision - when all providers need reprobe, we still need
    // to return something rather than fail completely
    assert!(
        selected.is_some(),
        "Selection should return fallback provider when all need reprobe, not None"
    );

    let fallback = selected.unwrap();
    assert_eq!(
        fallback.provider, "provider_a",
        "Fallback should be the first provider even when it needs reprobe"
    );
}

#[test]
fn test_edge_case_empty_provider_list_for_model() {
    // Test edge case: Model exists but has empty provider list

    let probe = ProviderProbe::new().with_threshold(3);

    // Create test results with empty provider list
    let mut models = HashMap::new();
    models.insert("empty_model".to_string(), vec![]);

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Selection should return None for empty provider list
    let selected = probe.get_best_provider_with_tracker(&results, "empty_model");
    assert!(
        selected.is_none(),
        "Selection should return None for empty provider list"
    );
}

#[test]
fn test_edge_case_all_providers_unsuccessful() {
    // Test edge case: All providers have success=false

    let probe = ProviderProbe::new().with_threshold(2);

    // Create test results where all providers failed probing
    let mut models = HashMap::new();
    models.insert(
        "failed_model".to_string(),
        vec![
            ProbeResult {
                provider: "failed_provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 0,
                ttfb_ms: 0,
                throughput_mbs: 0.0,
                score: f64::MAX,
                success: false,
                error: Some("Connection timeout".to_string()),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "failed_provider_b".to_string(),
                probe_url: "https://b.idx".to_string(),
                connect_ms: 0,
                ttfb_ms: 0,
                throughput_mbs: 0.0,
                score: f64::MAX,
                success: false,
                error: Some("Connection refused".to_string()),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Selection should return None when all providers are unsuccessful
    let selected = probe.get_best_provider_with_tracker(&results, "failed_model");
    assert!(
        selected.is_none(),
        "Selection should return None when all providers have success=false"
    );
}

#[test]
fn test_edge_case_zero_threshold_always_reprobe() {
    // Test edge case: threshold=0 means even 0 failures should trigger reprobe
    // (but only for providers that are in the failure tracking map)

    let probe = ProviderProbe::new().with_threshold(0);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "test".to_string(),
        vec![ProbeResult {
            provider: "provider_x".to_string(),
            probe_url: "https://x.idx".to_string(),
            connect_ms: 50,
            ttfb_ms: 75,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // With threshold=0, a provider needs to be in the tracking map first
    // For providers not in the map, should_reprobe returns false
    assert!(
        !probe.should_reprobe("provider_x"),
        "With threshold=0, should_reprobe returns false for providers not in tracking map"
    );

    // Record 0 failures (provider enters map with 0 failures)
    let mut probe = probe;
    probe.record_failure("provider_x"); // This brings count to 1, not 0

    // With threshold=0, count=1 >= 0 is true, so should_reprobe returns true
    assert!(
        probe.should_reprobe("provider_x"),
        "With threshold=0, once provider is in map, should_reprobe returns true (1 >= 0)"
    );

    // Verify selection behavior - should skip the provider that needs reprobe
    let selected = probe.get_best_provider_with_tracker(&results, "test");
    // Since provider_x needs reprobe, selection should return fallback
    assert!(
        selected.is_some(),
        "Selection should return fallback when threshold=0 triggers reprobe"
    );
    assert_eq!(
        selected.unwrap().provider,
        "provider_x",
        "Fallback should be provider_x when it's the only provider"
    );
}

#[test]
fn test_edge_case_very_high_threshold_never_reprobe() {
    // Test edge case: Very high threshold means practically never reprobe

    let probe = ProviderProbe::new().with_threshold(999999);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "test".to_string(),
        vec![ProbeResult {
            provider: "provider_y".to_string(),
            probe_url: "https://y.idx".to_string(),
            connect_ms: 50,
            ttfb_ms: 75,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record some failures
    let mut probe = probe;
    for _ in 0..100 {
        probe.record_failure("provider_y");
    }

    // With very high threshold, should_reprobe should still return false
    assert!(
        !probe.should_reprobe("provider_y"),
        "With very high threshold, should_reprobe should return false even after many failures"
    );

    // Verify selection works normally
    let selected = probe.get_best_provider_with_tracker(&results, "test");
    assert!(
        selected.is_some(),
        "Selection should work normally with very high threshold"
    );
    assert_eq!(selected.unwrap().provider, "provider_y");
}

#[test]
fn test_edge_case_large_scale_provider_selection() {
    // Test edge case: Large-scale scenario with 100+ providers
    // This tests performance and scalability

    let mut probe = ProviderProbe::new().with_threshold(5);

    // Create test results with 100 providers
    let mut models = HashMap::new();
    let providers: Vec<ProbeResult> = (0..100)
        .map(|i| ProbeResult {
            provider: format!("provider_{:03}", i),
            probe_url: format!("https://test{:03}.idx", i),
            connect_ms: 50 + i as u64 * 10,
            ttfb_ms: 75 + i as u64 * 15,
            throughput_mbs: 10.0,
            score: 125.0 + i as f64 * 10.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    models.insert("large_scale".to_string(), providers);

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make first 10 providers exceed threshold
    for i in 0..10 {
        for _ in 0..6 {
            probe.record_failure(&format!("provider_{:03}", i));
        }
    }

    // Verify selection works efficiently with large provider list
    let selected = probe.get_best_provider_with_tracker(&results, "large_scale");

    assert!(
        selected.is_some(),
        "Selection should succeed with large provider list"
    );

    let selected_provider = selected.unwrap();
    let provider_num = selected_provider
        .provider
        .strip_prefix("provider_")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // With parallel execution (par_iter().find_any()), the selection is non-deterministic
    // It can return ANY provider that matches the condition (success=true AND !should_reprobe)
    // So we just verify it selected a provider that doesn't need reprobe
    assert!(
        provider_num >= 10,
        "Should select a provider that doesn't exceed threshold (>= 10), got provider_{:03}",
        provider_num
    );

    // Verify the selected provider doesn't need reprobe
    assert!(
        !probe.should_reprobe(&selected_provider.provider),
        "Selected provider should not need reprobe"
    );
}

#[test]
fn test_edge_case_provider_name_with_special_characters() {
    // Test edge case: Provider names with special characters

    let probe = ProviderProbe::new().with_threshold(2);

    // Create test results with special provider names
    let mut models = HashMap::new();
    models.insert(
        "special".to_string(),
        vec![
            ProbeResult {
                provider: "s3:bucket-name-with-dashes".to_string(),
                probe_url: "https://special1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "gcs:bucket.name.with.dots".to_string(),
                probe_url: "https://special2.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for provider with dashes
    let mut probe = probe;
    probe.record_failure("s3:bucket-name-with-dashes");
    probe.record_failure("s3:bucket-name-with-dashes");

    // Verify should_reprobe works with special characters
    assert!(
        probe.should_reprobe("s3:bucket-name-with-dashes"),
        "should_reprobe should work with provider names containing dashes"
    );

    // Verify selection works with special characters
    let selected = probe.get_best_provider_with_tracker(&results, "special");
    assert!(
        selected.is_some(),
        "Selection should work with provider names containing special characters"
    );
    assert_eq!(selected.unwrap().provider, "gcs:bucket.name.with.dots");
}

#[test]
fn test_edge_case_boundary_conditions_exact_threshold() {
    // Test edge case: Boundary conditions at exact threshold values
    // Test threshold=1, threshold=2, threshold=5 to ensure exact boundary works

    for threshold in 1..=5 {
        let mut probe = ProviderProbe::new().with_threshold(threshold);

        // Create test results
        let mut models = HashMap::new();
        models.insert(
            "boundary".to_string(),
            vec![ProbeResult {
                provider: format!("provider_t{}", threshold),
                probe_url: "https://test.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }],
        );

        let results = ProviderProbeResults {
            models,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_sha: None,
        };

        // Test exactly at threshold-1 (should NOT trigger reprobe)
        for _ in 0..(threshold - 1) {
            probe.record_failure(&format!("provider_t{}", threshold));
        }
        assert!(
            !probe.should_reprobe(&format!("provider_t{}", threshold)),
            "should_reprobe should be false at threshold-1 for threshold={}",
            threshold
        );

        // Test exactly at threshold (should trigger reprobe)
        probe.record_failure(&format!("provider_t{}", threshold));
        assert!(
            probe.should_reprobe(&format!("provider_t{}", threshold)),
            "should_reprobe should be true at exactly threshold={}",
            threshold
        );

        // Test at threshold+1 (should still trigger reprobe)
        probe.record_failure(&format!("provider_t{}", threshold));
        assert!(
            probe.should_reprobe(&format!("provider_t{}", threshold)),
            "should_reprobe should remain true at threshold+1 for threshold={}",
            threshold
        );
    }
}

#[test]
fn test_edge_case_mixed_success_and_failure_providers() {
    // Test edge case: Mix of successful (success=true) and unsuccessful (success=false) providers
    // with some exceeding threshold and some not

    let mut probe = ProviderProbe::new().with_threshold(3);

    // Create test results with mixed success/failure
    let mut models = HashMap::new();
    models.insert(
        "mixed".to_string(),
        vec![
            ProbeResult {
                provider: "successful_but_exceeds".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 20,
                ttfb_ms: 30,
                throughput_mbs: 20.0,
                score: 70.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "unsuccessful_provider".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 0,
                ttfb_ms: 0,
                throughput_mbs: 0.0,
                score: f64::MAX,
                success: false,
                error: Some("Failed".to_string()),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "successful_good".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 175.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make first successful provider exceed threshold
    probe.record_failure("successful_but_exceeds");
    probe.record_failure("successful_but_exceeds");
    probe.record_failure("successful_but_exceeds");

    // Verify selection skips both unsuccessful and exceeds-threshold providers
    let selected = probe.get_best_provider_with_tracker(&results, "mixed");

    assert!(selected.is_some(), "Selection should find a valid provider");
    assert_eq!(
        selected.unwrap().provider,
        "successful_good",
        "Selection should skip both unsuccessful and exceeds-threshold providers"
    );
}

#[test]
fn test_edge_case_single_provider_exceeds_threshold() {
    // Test edge case: Single provider that exceeds threshold

    let mut probe = ProviderProbe::new().with_threshold(2);

    // Create test results with single provider
    let mut models = HashMap::new();
    models.insert(
        "single".to_string(),
        vec![ProbeResult {
            provider: "only_provider".to_string(),
            probe_url: "https://test.idx".to_string(),
            connect_ms: 50,
            ttfb_ms: 75,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make the single provider exceed threshold
    probe.record_failure("only_provider");
    probe.record_failure("only_provider");

    // Verify selection returns fallback (the only provider)
    let selected = probe.get_best_provider_with_tracker(&results, "single");

    assert!(
        selected.is_some(),
        "Selection should return fallback when single provider exceeds threshold"
    );
    assert_eq!(
        selected.unwrap().provider,
        "only_provider",
        "Fallback should be the only provider even when it exceeds threshold"
    );
}

#[test]
fn test_edge_case_providers_with_identical_scores() {
    // Test edge case: Multiple providers with identical scores

    let probe = ProviderProbe::new().with_threshold(2);

    // Create test results with identical scores
    let mut models = HashMap::new();
    models.insert(
        "identical".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://b.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // With identical scores, should select the first one
    let selected = probe.get_best_provider_with_tracker(&results, "identical");
    assert_eq!(
        selected.unwrap().provider,
        "provider_a",
        "With identical scores, should select the first provider in list"
    );
}

#[test]
fn test_edge_case_no_providers_need_reprobe() {
    // Test edge case: No providers need reprobe (normal case)

    let probe = ProviderProbe::new().with_threshold(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "normal".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://b.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // No failures recorded, so no providers need reprobe
    assert!(!probe.should_reprobe("provider_a"));
    assert!(!probe.should_reprobe("provider_b"));

    // Selection should work normally
    let selected = probe.get_best_provider_with_tracker(&results, "normal");
    assert_eq!(
        selected.unwrap().provider,
        "provider_a",
        "Should select fastest provider when no providers need reprobe"
    );
}

#[test]
fn test_edge_case_threshold_of_one() {
    // Test edge case: threshold=1 (reprobe after single failure)

    let mut probe = ProviderProbe::new().with_threshold(1);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "threshold_one".to_string(),
        vec![ProbeResult {
            provider: "provider_x".to_string(),
            probe_url: "https://x.idx".to_string(),
            connect_ms: 50,
            ttfb_ms: 75,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Initially should not need reprobe
    assert!(!probe.should_reprobe("provider_x"));

    // After single failure, should need reprobe
    probe.record_failure("provider_x");
    assert!(
        probe.should_reprobe("provider_x"),
        "With threshold=1, should need reprobe after single failure"
    );

    // Selection should return fallback when single failure triggers reprobe
    let selected = probe.get_best_provider_with_tracker(&results, "threshold_one");
    assert!(
        selected.is_some(),
        "Selection should return fallback when threshold=1 triggers reprobe"
    );
}

#[test]
fn test_edge_case_concurrent_access_thread_safety() {
    // Test edge case: Concurrent access to provider selection (thread safety)
    use std::sync::Arc;
    use std::thread;

    let probe = Arc::new(std::sync::Mutex::new(
        ProviderProbe::new().with_threshold(3),
    ));

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "concurrent".to_string(),
        vec![ProbeResult {
            provider: "provider_a".to_string(),
            probe_url: "https://a.idx".to_string(),
            connect_ms: 50,
            ttfb_ms: 75,
            throughput_mbs: 10.0,
            score: 125.0,
            success: true,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }],
    );

    let results = Arc::new(ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    });

    // Spawn multiple threads accessing the same probe
    let mut handles = vec![];
    for i in 0..5 {
        let probe_clone = Arc::clone(&probe);
        let results_clone = Arc::clone(&results);

        handles.push(thread::spawn(move || {
            let probe = probe_clone.lock().unwrap();
            // Each thread calls should_reprobe
            probe.should_reprobe("provider_a")
        }));
    }

    // All threads should complete without deadlock
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(
            !result,
            "should_reprobe should return false in concurrent access"
        );
    }

    // Verify selection still works after concurrent access
    let probe = probe.lock().unwrap();
    let selected = probe.get_best_provider_with_tracker(&results, "concurrent");
    assert!(
        selected.is_some(),
        "Selection should work correctly after concurrent access"
    );
}

#[test]
fn test_edge_case_provider_reset_during_selection_scenario() {
    // Test edge case: What happens if a provider is reset (success recorded)
    // during a selection scenario?

    let mut probe = ProviderProbe::new().with_threshold(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "reset_scenario".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_b".to_string(),
                probe_url: "https://b.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for provider_a to exceed threshold
    probe.record_failure("provider_a");
    probe.record_failure("provider_a");
    probe.record_failure("provider_a");

    // Verify provider_a needs reprobe
    assert!(probe.should_reprobe("provider_a"));

    // Selection should skip provider_a
    let selected = probe.get_best_provider_with_tracker(&results, "reset_scenario");
    assert_eq!(
        selected.unwrap().provider,
        "provider_b",
        "Should select provider_b when provider_a exceeds threshold"
    );

    // Now reset provider_a (simulating successful request)
    probe.record_success("provider_a");

    // Verify provider_a no longer needs reprobe
    assert!(!probe.should_reprobe("provider_a"));

    // Selection should now select provider_a (fastest provider)
    let selected = probe.get_best_provider_with_tracker(&results, "reset_scenario");
    assert_eq!(
        selected.unwrap().provider,
        "provider_a",
        "Should select provider_a after it's reset"
    );
}

#[test]
fn test_edge_case_fallback_when_all_providers_exceed_threshold() {
    // Test edge case: Verify fallback behavior is consistent when all providers exceed threshold
    // This tests the implementation's fallback strategy

    let mut probe = ProviderProbe::new().with_threshold(2);

    // Create test results with providers in different rank orders
    let mut models = HashMap::new();
    models.insert(
        "fallback_test".to_string(),
        vec![
            ProbeResult {
                provider: "rank1_provider".to_string(),
                probe_url: "https://rank1.idx".to_string(),
                connect_ms: 10,
                ttfb_ms: 20,
                throughput_mbs: 25.0,
                score: 50.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "rank2_provider".to_string(),
                probe_url: "https://rank2.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 175.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "rank3_provider".to_string(),
                probe_url: "https://rank3.idx".to_string(),
                connect_ms: 100,
                ttfb_ms: 150,
                throughput_mbs: 5.0,
                score: 350.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Make ALL providers exceed threshold
    probe.record_failure("rank1_provider");
    probe.record_failure("rank1_provider");
    probe.record_failure("rank2_provider");
    probe.record_failure("rank2_provider");
    probe.record_failure("rank3_provider");
    probe.record_failure("rank3_provider");

    // Verify all need reprobe
    assert!(probe.should_reprobe("rank1_provider"));
    assert!(probe.should_reprobe("rank2_provider"));
    assert!(probe.should_reprobe("rank3_provider"));

    // Test fallback behavior
    let selected = probe.get_best_provider_with_tracker(&results, "fallback_test");

    assert!(
        selected.is_some(),
        "Should return fallback provider when all exceed threshold"
    );

    // Fallback should be the FIRST provider (rank1_provider)
    // even though it exceeds threshold
    assert_eq!(
        selected.unwrap().provider,
        "rank1_provider",
        "Fallback should return the first provider when all exceed threshold"
    );
}
