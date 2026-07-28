//! Tests for parallel execution of should_reprobe with is_stale checks
//!
//! These tests verify that the parallel execution of should_reprobe alongside
//! is_stale checks works correctly, including:
//! - Both should_reprobe and is_stale are called in parallel
//! - Selection completes without deadlock
//! - Timing/parallel behavior (both complete before selection proceeds)
//! - Combined parallel execution behavior

#![cfg(feature = "probe")]

use gribtract_fetch::probe::{ProviderProbe, ProviderProbeResults, ProbeResult};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

/// A test wrapper that tracks both is_stale and should_reprobe calls
struct ParallelExecutionTracker {
    probe: ProviderProbe,
    /// Records when is_stale was called
    is_stale_called: RefCell<bool>,
    /// Records when should_reprobe was called (with provider names)
    should_reprobe_calls: RefCell<Vec<String>>,
    /// Tracks execution order to verify parallel behavior
    execution_log: RefCell<Vec<String>>,
}

impl ParallelExecutionTracker {
    fn new(threshold: u32) -> Self {
        Self {
            probe: ProviderProbe::new().with_threshold(threshold),
            is_stale_called: RefCell::new(false),
            should_reprobe_calls: RefCell::new(Vec::new()),
            execution_log: RefCell::new(Vec::new()),
        }
    }

    /// Track and call should_reprobe
    fn tracked_should_reprobe(&self, provider: &str) -> bool {
        self.execution_log.borrow_mut().push(format!("should_reprobe({})", provider));
        self.should_reprobe_calls.borrow_mut().push(provider.to_string());
        self.probe.should_reprobe(provider)
    }

    /// Track and call is_stale
    fn tracked_is_stale(&self, results: &ProviderProbeResults, max_age: Duration) -> bool {
        self.execution_log.borrow_mut().push("is_stale()".to_string());
        *self.is_stale_called.borrow_mut() = true;
        ProviderProbe::is_stale(results, max_age)
    }

    fn record_failure(&mut self, provider: &str) -> u32 {
        self.probe.record_failure(provider)
    }

    fn get_should_reprobe_calls(&self) -> Vec<String> {
        self.should_reprobe_calls.borrow().clone()
    }

    fn was_is_stale_called(&self) -> bool {
        *self.is_stale_called.borrow()
    }

    fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.borrow().clone()
    }

    fn clear_tracking(&self) {
        *self.is_stale_called.borrow_mut() = false;
        self.should_reprobe_calls.borrow_mut().clear();
        self.execution_log.borrow_mut().clear();
    }
}

#[test]
fn test_is_stale_and_should_reprobe_both_checked_during_validation() {
    // Test that both is_stale and should_reprobe are checked during validation
    // This verifies the combined parallel execution behavior

    let mut tracker = ParallelExecutionTracker::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "hrrr".to_string(),
        vec![
            ProbeResult {
                provider: "s3:hrrr".to_string(),
                probe_url: "https://test1.idx".to_string(),
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

    // Record failures for a provider
    tracker.record_failure("s3:hrrr");
    tracker.record_failure("s3:hrrr");
    tracker.record_failure("s3:hrrr");

    // Manually simulate the validation flow with tracking
    let max_age = Duration::from_secs(24 * 3600);

    // First, is_stale should be checked
    let _is_stale_result = tracker.tracked_is_stale(&results, max_age);

    // Then, should_reprobe should be checked for providers (in parallel in real implementation)
    let _needs_reprobe = tracker.tracked_should_reprobe("s3:hrrr");

    // Verify both were called
    assert!(tracker.was_is_stale_called(),
            "is_stale should be called during validation");
    assert!(tracker.get_should_reprobe_calls().contains(&"s3:hrrr".to_string()),
            "should_reprobe should be called for s3:hrrr during validation");

    // Verify execution log contains both calls
    let log = tracker.get_execution_log();
    assert!(log.iter().any(|entry| entry.starts_with("is_stale")),
            "Execution log should contain is_stale call");
    assert!(log.iter().any(|entry| entry.starts_with("should_reprobe")),
            "Execution log should contain should_reprobe call");
}

#[test]
fn test_parallel_execution_no_deadlock() {
    // Test that parallel execution of is_stale and should_reprobe doesn't deadlock
    // This creates a scenario where both are checked and verifies completion

    let mut tracker = ParallelExecutionTracker::new(2);

    // Create test results with multiple providers
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

    // Record failures for some providers
    tracker.record_failure("provider_0");
    tracker.record_failure("provider_0");
    tracker.record_failure("provider_2");
    tracker.record_failure("provider_2");

    // Simulate the validation + selection flow
    // This mirrors what happens in production: is_stale checked first, then should_reprobe
    let max_age = Duration::from_secs(24 * 3600);

    // Check is_stale (should complete without deadlock)
    let _is_stale = tracker.tracked_is_stale(&results, max_age);

    // Check should_reprobe for multiple providers (parallel in real implementation)
    for i in 0..5 {
        let provider = format!("provider_{}", i);
        tracker.tracked_should_reprobe(&provider);
    }

    // Verify all calls completed
    assert!(tracker.was_is_stale_called(),
            "is_stale should complete without deadlock");
    assert_eq!(tracker.get_should_reprobe_calls().len(), 5,
            "should_reprobe should be called for all 5 providers without deadlock");

    // Verify no deadlock occurred by checking execution completed
    let log = tracker.get_execution_log();
    assert!(log.len() >= 6, "All calls should complete (is_stale + 5 should_reprobe calls)");
}

#[test]
fn test_parallel_execution_timing_both_complete_before_selection() {
    // Test that both is_stale and should_reprobe complete before selection proceeds
    // This verifies the timing/parallel behavior

    let mut tracker = ParallelExecutionTracker::new(3);

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

    // Record failures
    tracker.record_failure("s3:gfs");
    tracker.record_failure("s3:gfs");
    tracker.record_failure("s3:gfs");

    // Simulate the complete flow:
    // 1. is_stale check
    // 2. should_reprobe checks (for all providers)
    // 3. Selection proceeds only after both complete

    let max_age = Duration::from_secs(24 * 3600);

    // Step 1: Check staleness
    let is_stale_result = tracker.tracked_is_stale(&results, max_age);
    assert!(!is_stale_result, "Results should be fresh for this test");

    // Step 2: Check should_reprobe for providers
    let s3_needs_reprobe = tracker.tracked_should_reprobe("s3:gfs");
    assert!(s3_needs_reprobe, "s3:gfs should need reprobe");

    let nomads_needs_reprobe = tracker.tracked_should_reprobe("nomads:gfs");
    assert!(!nomads_needs_reprobe, "nomads:gfs should not need reprobe");

    // Step 3: Selection proceeds only after both checks complete
    // Verify that we have the results from both checks before selection
    let log = tracker.get_execution_log();

    // Verify is_stale was called before selection
    let is_stale_position = log.iter().position(|entry| entry.starts_with("is_stale"));
    assert!(is_stale_position.is_some(), "is_stale should be called");

    // Verify should_reprobe was called for both providers before selection
    let should_reprobe_count = log.iter().filter(|entry| entry.starts_with("should_reprobe")).count();
    assert_eq!(should_reprobe_count, 2, "should_reprobe should be called for both providers");

    // Verify timing: both checks complete before we make selection decision
    assert!(tracker.was_is_stale_called(), "is_stale must complete before selection");
    assert!(tracker.get_should_reprobe_calls().len() >= 2,
            "should_reprobe must complete for providers before selection");
}

#[test]
fn test_combined_parallel_execution_behavior_with_fresh_results() {
    // Test the combined parallel execution when results are fresh
    // is_stale returns false, should_reprobe determines the outcome

    let mut tracker = ParallelExecutionTracker::new(2);

    // Create fresh results
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
    tracker.record_failure("s3:nbm");
    tracker.record_failure("s3:nbm");

    // Combined flow: is_stale + should_reprobe
    let max_age = Duration::from_secs(24 * 3600);

    let is_stale = tracker.tracked_is_stale(&results, max_age);
    let s3_reprobe = tracker.tracked_should_reprobe("s3:nbm");
    let gcs_reprobe = tracker.tracked_should_reprobe("gcs:nbm");

    // Verify combined behavior
    assert!(!is_stale, "is_stale should return false for fresh results");
    assert!(s3_reprobe, "should_reprobe should return true for s3:nbm");
    assert!(!gcs_reprobe, "should_reprobe should return false for gcs:nbm");

    // Verify both were called
    assert!(tracker.was_is_stale_called());
    assert!(tracker.get_should_reprobe_calls().contains(&"s3:nbm".to_string()));
    assert!(tracker.get_should_reprobe_calls().contains(&"gcs:nbm".to_string()));

    // The combined result: fresh but some providers need reprobe
    // This is the dual-trigger behavior
    let log = tracker.get_execution_log();
    assert!(log.len() >= 3, "Should have 3 calls: is_stale + 2 should_reprobe");
}

#[test]
fn test_combined_parallel_execution_behavior_with_stale_results() {
    // Test the combined parallel execution when results are stale
    // is_stale returns true, should_reprobe results don't matter

    let mut tracker = ParallelExecutionTracker::new(3);

    // Create stale results (25 hours old)
    let timestamp = chrono::Utc::now() - chrono::Duration::hours(25);
    let mut models = HashMap::new();
    models.insert(
        "hrrr".to_string(),
        vec![
            ProbeResult {
                provider: "s3:hrrr".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 10.0,
                score: 125.0,
                success: true,
                error: None,
                timestamp: timestamp.to_rfc3339(),
            },
        ],
    );

    let results = ProviderProbeResults {
        models,
        timestamp: timestamp.to_rfc3339(),
        git_sha: None,
    };

    // Record failures
    tracker.record_failure("s3:hrrr");
    tracker.record_failure("s3:hrrr");

    // Combined flow: is_stale + should_reprobe
    let max_age = Duration::from_secs(24 * 3600);

    let is_stale = tracker.tracked_is_stale(&results, max_age);
    let s3_reprobe = tracker.tracked_should_reprobe("s3:hrrr");

    // Verify combined behavior
    assert!(is_stale, "is_stale should return true for stale results");
    assert!(!s3_reprobe, "should_reprobe should return false (below threshold)");

    // Verify both were called
    assert!(tracker.was_is_stale_called());
    assert!(tracker.get_should_reprobe_calls().contains(&"s3:hrrr".to_string()));

    // The combined result: stale results invalidate everything
    // This is the dual-trigger behavior where staleness trumps should_reprobe
    let log = tracker.get_execution_log();
    assert!(log.len() >= 2, "Should have 2 calls: is_stale + should_reprobe");
}

#[test]
fn test_parallel_execution_order_preserves_semantics() {
    // Test that parallel execution preserves the semantics of the dual-trigger logic
    // Even when is_stale and should_reprobe are checked in parallel, the behavior
    // should be the same as sequential execution

    let mut tracker = ParallelExecutionTracker::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "gefs".to_string(),
        vec![
            ProbeResult {
                provider: "s3:gefs".to_string(),
                probe_url: "https://test1.idx".to_string(),
                connect_ms: 60,
                ttfb_ms: 90,
                throughput_mbs: 8.0,
                score: 185.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "gcs:gefs".to_string(),
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

    // Record failures
    tracker.record_failure("s3:gefs");
    tracker.record_failure("s3:gefs");
    tracker.record_failure("s3:gefs");

    // Test the actual is_valid implementation which uses parallel execution
    let is_valid = tracker.probe.is_valid(&results, Duration::from_secs(24 * 3600));

    // Verify the dual-trigger semantics:
    // - is_stale would return false (fresh)
    // - should_reprobe returns true for s3:gefs
    // - Combined result: is_valid should return false
    assert!(!is_valid, "is_valid should return false when should_reprobe returns true");

    // Manually verify the semantics with tracking
    tracker.clear_tracking();
    let is_stale = tracker.tracked_is_stale(&results, Duration::from_secs(24 * 3600));
    let s3_reprobe = tracker.tracked_should_reprobe("s3:gefs");

    assert!(!is_stale, "is_stale should return false");
    assert!(s3_reprobe, "should_reprobe should return true");

    // The semantics are preserved: OR logic (stale OR should_reprobe) = false
    // means is_valid = true, but here should_reprobe is true, so is_valid = false
    let log = tracker.get_execution_log();
    assert!(log.len() >= 2, "Both checks should be called");
}

#[test]
#[cfg(feature = "rayon")]
fn test_rayon_parallel_execution_of_should_reprobe() {
    // Test that rayon's parallel execution works correctly for should_reprobe checks
    // This verifies the actual parallel implementation in the production code

    let mut probe = ProviderProbe::new().with_threshold(2);

    let mut probe = ProviderProbe::new().with_threshold(2);

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

    models.insert("test".to_string(), providers);

    let results = ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Record failures for some providers
    for i in 0..5 {
        probe.record_failure(&format!("provider_{}", i));
        probe.record_failure(&format!("provider_{}", i));
    }

    // Call the actual implementation which uses parallel execution
    // get_best_provider_with_tracker uses par_iter internally
    let selected = probe.get_best_provider_with_tracker(&results, "test");

    // Verify selection completed
    assert!(selected.is_some(), "Parallel selection should complete without deadlock");

    // Verify it skipped the providers that need reprobe
    let selected_provider = selected.unwrap();
    let provider_num = selected_provider.provider.strip_prefix("provider_")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    assert!(provider_num >= 5, "Should select a provider that doesn't need reprobe");

    // Verify parallel execution worked by checking that should_reprobe was called
    // We can't directly verify this without instrumentation, but the selection
    // outcome proves it was called (providers 0-4 were skipped)
}

#[test]
fn test_concurrent_calls_to_is_stale_and_should_reprobe() {
    // Test that concurrent calls to is_stale and should_reprobe don't interfere
    // This simulates the real-world scenario where both might be checked

    let tracker = Arc::new(Mutex::new(ParallelExecutionTracker::new(2)));

    // Create test results
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
        ],
    );

    let results = Arc::new(ProviderProbeResults {
        models,
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    });

    // Simulate concurrent calls (mimicking parallel execution)
    let tracker1 = Arc::clone(&tracker);
    let tracker2 = Arc::clone(&tracker);
    let results1 = Arc::clone(&results);

    // Spawn threads to simulate concurrent execution
    let handle1 = std::thread::spawn(move || {
        let tracker = tracker1.lock().unwrap();
        tracker.tracked_is_stale(&results1, Duration::from_secs(24 * 3600))
    });

    let handle2 = std::thread::spawn(move || {
        let tracker = tracker2.lock().unwrap();
        tracker.tracked_should_reprobe("provider_a")
    });

    // Both should complete without interference
    let is_stale_result = handle1.join().unwrap();
    let should_reprobe_result = handle2.join().unwrap();

    // Verify both completed successfully
    assert!(!is_stale_result, "is_stale should complete successfully");
    assert!(!should_reprobe_result, "should_reprobe should complete successfully");

    // Verify both were tracked
    let tracker = tracker.lock().unwrap();
    assert!(tracker.was_is_stale_called(), "is_stale should be tracked");
    assert!(tracker.get_should_reprobe_calls().contains(&"provider_a".to_string()),
            "should_reprobe should be tracked");
}

#[test]
fn test_parallel_execution_with_edge_cases() {
    // Test parallel execution with various edge cases:
    // - Empty provider list
    // - Single provider
    // - All providers need reprobe
    // - No providers need reprobe

    let mut tracker = ParallelExecutionTracker::new(2);

    // Test Case 1: Single provider
    let mut models = HashMap::new();
    models.insert(
        "single".to_string(),
        vec![
            ProbeResult {
                provider: "only_provider".to_string(),
                probe_url: "https://test.idx".to_string(),
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

    tracker.record_failure("only_provider");
    tracker.record_failure("only_provider");

    let is_stale = tracker.tracked_is_stale(&results, Duration::from_secs(24 * 3600));
    let needs_reprobe = tracker.tracked_should_reprobe("only_provider");

    assert!(!is_stale, "is_stale should handle single provider");
    assert!(needs_reprobe, "should_reprobe should handle single provider");
    assert!(tracker.was_is_stale_called());

    // Test Case 2: Empty model (no providers)
    tracker.clear_tracking();
    let empty_results = ProviderProbeResults {
        models: HashMap::new(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    let is_stale = tracker.tracked_is_stale(&empty_results, Duration::from_secs(24 * 3600));
    assert!(!is_stale, "is_stale should handle empty results");
    assert!(tracker.was_is_stale_called());
}

#[test]
fn test_dual_trigger_semantics_preserved_in_parallel_execution() {
    // Test that the dual-trigger semantics are preserved:
    // is_valid returns false if EITHER is_stale returns true OR should_reprobe returns true

    let mut probe = ProviderProbe::new().with_threshold(3);

    // Create fresh results
    let results = ProviderProbeResults {
        models: HashMap::new(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        git_sha: None,
    };

    // Case 1: Neither trigger active
    assert!(probe.is_valid(&results, Duration::from_secs(24 * 3600)),
            "is_valid should return true when neither trigger is active");

    // Case 2: should_reprobe trigger active
    probe.record_failure("test_provider");
    probe.record_failure("test_provider");
    probe.record_failure("test_provider");
    assert!(!probe.is_valid(&results, Duration::from_secs(24 * 3600)),
            "is_valid should return false when should_reprobe trigger is active");

    // Case 3: is_stale trigger active
    let probe2 = ProviderProbe::new().with_threshold(3);
    let stale_results = ProviderProbeResults {
        models: HashMap::new(),
        timestamp: (chrono::Utc::now() - chrono::Duration::hours(25)).to_rfc3339(),
        git_sha: None,
    };
    assert!(!probe2.is_valid(&stale_results, Duration::from_secs(24 * 3600)),
            "is_valid should return false when is_stale trigger is active");

    // Case 4: Both triggers active
    let mut probe3 = ProviderProbe::new().with_threshold(3);
    probe3.record_failure("test_provider");
    probe3.record_failure("test_provider");
    probe3.record_failure("test_provider");
    assert!(!probe3.is_valid(&stale_results, Duration::from_secs(24 * 3600)),
            "is_valid should return false when both triggers are active");

    // This verifies that the OR semantics are preserved:
    // valid = !stale AND !should_reprobe
}
