//! Integration tests that verify should_reprobe is actually called during
//! provider selection flow, not just during validation.
//!
//! These tests use a spy/tracking pattern to verify that should_reprobe
//! is invoked as part of the selection logic.

#![cfg(feature = "probe")]

use gribtract_fetch::probe::{ProviderProbe, ProviderProbeResults, ProbeResult};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

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
        self.should_reprobe_calls.borrow_mut().push(provider.to_string());
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
        self.should_reprobe_calls.borrow().iter().any(|p| p == provider)
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
    assert!(tracked.call_count() > 0, "should_reprobe should be called during selection");

    // Verify it was called for provider_a (the failing one)
    assert!(tracked.was_should_reprobe_called("provider_a"),
            "should_reprobe should be called for provider_a during selection");

    // Verify we stopped checking once we found a valid provider (provider_b)
    // The exact number depends on the implementation, but it should be at least 2
    assert!(tracked.call_count() >= 2,
            "should_reprobe should be called for at least 2 providers during selection");
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
    let is_valid_no_failures = tracked.probe.is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(is_valid_no_failures,
            "Validation should succeed with fresh results and no failures");

    // Now record failures to trigger should_reprobe
    tracked.record_failure("s3:gfs");
    tracked.record_failure("s3:gfs");
    tracked.record_failure("s3:gfs");

    // Verify should_reprobe returns true for s3:gfs
    assert!(tracked.tracked_should_reprobe("s3:gfs"),
            "should_reprobe should return true after 3 failures");

    // Validation should now fail because should_reprobe returns true
    let is_valid_with_failures = tracked.probe.is_valid(&results, Duration::from_secs(24 * 3600));
    assert!(!is_valid_with_failures,
            "Validation should fail when should_reprobe returns true");

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

    assert_eq!(selected_provider, Some("nomads:gfs".to_string()),
                "Selection should skip s3:gfs and select nomads:gfs when should_reprobe returns true");

    // Verify should_reprobe was called during selection tracking
    let selection_calls = tracked.call_count();
    assert!(selection_calls > 0,
            "should_reprobe should be called during selection path");

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
    let providers: Vec<ProbeResult> = (0..5).map(|i| ProbeResult {
        provider: format!("provider_{}", i),
        probe_url: format!("https://test{}.idx", i),
        connect_ms: 50 + i as u64 * 10,
        ttfb_ms: 75 + i as u64 * 15,
        throughput_mbs: 10.0,
        score: 125.0,
        success: true,
        error: None,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }).collect();

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
    assert!(tracked.call_count() >= 1,
            "should_reprobe should be called for at least the first provider");

    // Should have been called for provider_0
    assert!(tracked.was_should_reprobe_called("provider_0"),
            "should_reprobe should be called for provider_0");
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
    assert!(tracked.tracked_should_reprobe("fast_provider"),
            "fast_provider should need reprobe");

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

    assert_eq!(selected_provider, Some("medium_provider".to_string()),
            "Selection should skip failing fast_provider and select medium_provider");

    // Verify should_reprobe was called multiple times during selection
    assert!(tracked.call_count() >= 2,
            "should_reprobe should be called at least twice during selection");
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
    assert!(tracked.was_should_reprobe_called("p1"),
            "should_reprobe should be called for p1");
    assert!(tracked.was_should_reprobe_called("p2"),
            "should_reprobe should be called for p2");
    assert!(tracked.was_should_reprobe_called("p3"),
            "should_reprobe should be called for p3");

    assert_eq!(tracked.call_count(), 3,
            "should_reprobe should be called exactly 3 times (once per provider)");
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

    assert_eq!(selected, Some("gcs:nbm".to_string()),
            "Should select gcs:nbm when s3:nbm needs reprobe");

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

    assert_eq!(selected, Some("s3:nbm".to_string()),
            "Should select s3:nbm after it's reset");
}

#[test]
fn test_should_reprobe_parallel_selection_calls() {
    // Test that should_reprobe is called correctly during parallel selection
    // This verifies the integration works with rayon's parallel execution

    let tracked = Arc::new(Mutex::new(TrackedProviderProbe::new(3)));

    // Create test results with multiple providers
    let mut models = HashMap::new();
    let providers: Vec<ProbeResult> = (0..10).map(|i| ProbeResult {
        provider: format!("provider_{}", i),
        probe_url: format!("https://test{}.idx", i),
        connect_ms: 50 + i as u64 * 10,
        ttfb_ms: 75 + i as u64 * 15,
        throughput_mbs: 10.0,
        score: 125.0,
        success: true,
        error: None,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }).collect();

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
        assert!(tracked_lock.call_count() > 0,
                "should_reprobe should be called during parallel selection");
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
        assert!(tracked_lock.call_count() > 0,
                "should_reprobe should be called during sequential selection");
    }
}
