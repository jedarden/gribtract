//! Unit tests that verify provider exclusion on should_reprobe failure
//!
//! These tests use mocking to verify that providers where should_reprobe returns
//! true are correctly excluded from the candidate list during provider selection.

#![cfg(feature = "probe")]

use gribtract_fetch::probe::{ProbeResult, ProviderProbe, ProviderProbeResults};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A mock wrapper that allows controlling should_reprobe return values
struct MockProviderProbe {
    probe: ProviderProbe,
    /// Mock return values for should_reprobe (provider -> bool)
    mock_should_reprobe: RefCell<HashMap<String, bool>>,
    /// Track actual should_reprobe calls
    should_reprobe_calls: RefCell<Vec<String>>,
}

impl MockProviderProbe {
    fn new(threshold: u32) -> Self {
        Self {
            probe: ProviderProbe::new().with_threshold(threshold),
            mock_should_reprobe: RefCell::new(HashMap::new()),
            should_reprobe_calls: RefCell::new(Vec::new()),
        }
    }

    /// Set a mock return value for should_reprobe
    fn set_mock_should_reprobe(&self, provider: &str, value: bool) {
        self.mock_should_reprobe
            .borrow_mut()
            .insert(provider.to_string(), value);
    }

    /// Clear all mock values
    fn clear_mocks(&self) {
        self.mock_should_reprobe.borrow_mut().clear();
    }

    /// Track should_reprobe calls during selection
    fn track_should_reprobe(&self, provider: &str) -> bool {
        self.should_reprobe_calls
            .borrow_mut()
            .push(provider.to_string());

        // Use mock value if set, otherwise use real implementation
        if let Some(&mock_value) = self.mock_should_reprobe.borrow().get(provider) {
            mock_value
        } else {
            self.probe.should_reprobe(provider)
        }
    }

    /// Get the list of providers that should_reprobe was called for
    fn get_should_reprobe_calls(&self) -> Vec<String> {
        self.should_reprobe_calls.borrow().clone()
    }

    /// Clear the call tracking
    fn clear_calls(&self) {
        self.should_reprobe_calls.borrow_mut().clear();
    }

    /// Record a failure for the given provider
    fn record_failure(&mut self, provider: &str) -> u32 {
        self.probe.record_failure(provider)
    }

    /// Record a success for the given provider
    fn record_success(&mut self, provider: &str) {
        self.probe.record_success(provider)
    }

    /// Simulate selection logic with provider exclusion
    fn simulate_selection_with_exclusion<'a>(
        &'a self,
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Option<&'a ProbeResult> {
        results.models.get(model).and_then(|model_results| {
            model_results
                .iter()
                .find(|r| r.success && !self.track_should_reprobe(&r.provider))
        })
    }

    /// Get the list of candidate providers (those that would NOT be excluded)
    fn get_candidate_providers<'a>(
        &'a self,
        results: &'a ProviderProbeResults,
        model: &str,
    ) -> Vec<&'a ProbeResult> {
        results
            .models
            .get(model)
            .map_or_else(Vec::new, |model_results| {
                model_results
                    .iter()
                    .filter(|r| r.success && !self.track_should_reprobe(&r.provider))
                    .collect()
            })
    }
}

#[test]
fn test_single_provider_exclusion_on_should_reprobe_failure() {
    // Test that a single provider where should_reprobe returns true is excluded
    // from the candidate list during provider selection

    let mock = MockProviderProbe::new(3);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
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

    // Mock should_reprobe to return true for the fastest provider
    mock.set_mock_should_reprobe("fast_provider", true);
    mock.set_mock_should_reprobe("medium_provider", false);
    mock.set_mock_should_reprobe("slow_provider", false);

    // Get candidate providers (should exclude fast_provider)
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify fast_provider is excluded
    assert!(
        !candidates.iter().any(|c| c.provider == "fast_provider"),
        "fast_provider should be excluded when should_reprobe returns true"
    );

    // Verify medium_provider and slow_provider remain as candidates
    assert!(
        candidates.iter().any(|c| c.provider == "medium_provider"),
        "medium_provider should remain a candidate when should_reprobe returns false"
    );
    assert!(
        candidates.iter().any(|c| c.provider == "slow_provider"),
        "slow_provider should remain a candidate when should_reprobe returns false"
    );

    // Clear calls from get_candidate_providers
    mock.clear_calls();

    // Verify selection skips fast_provider and selects medium_provider
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "medium_provider",
        "Selection should skip excluded fast_provider and select medium_provider"
    );
}

#[test]
fn test_multiple_providers_exclusion_on_should_reprobe_failure() {
    // Test that multiple providers where should_reprobe returns true are excluded
    // from the candidate list during provider selection

    let mock = MockProviderProbe::new(2);

    // Create test results with 5 providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "provider_1".to_string(),
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
                provider: "provider_2".to_string(),
                probe_url: "https://test2.idx".to_string(),
                connect_ms: 40,
                ttfb_ms: 60,
                throughput_mbs: 15.0,
                score: 126.7,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_3".to_string(),
                probe_url: "https://test3.idx".to_string(),
                connect_ms: 60,
                ttfb_ms: 90,
                throughput_mbs: 10.0,
                score: 190.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_4".to_string(),
                probe_url: "https://test4.idx".to_string(),
                connect_ms: 80,
                ttfb_ms: 120,
                throughput_mbs: 7.5,
                score: 283.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "provider_5".to_string(),
                probe_url: "https://test5.idx".to_string(),
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

    // Mock should_reprobe to return true for provider_1, provider_2, and provider_3
    mock.set_mock_should_reprobe("provider_1", true);
    mock.set_mock_should_reprobe("provider_2", true);
    mock.set_mock_should_reprobe("provider_3", true);
    mock.set_mock_should_reprobe("provider_4", false);
    mock.set_mock_should_reprobe("provider_5", false);

    // Get candidate providers (should exclude provider_1, provider_2, provider_3)
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify excluded providers are not in candidate list
    assert!(
        !candidates.iter().any(|c| c.provider == "provider_1"),
        "provider_1 should be excluded when should_reprobe returns true"
    );
    assert!(
        !candidates.iter().any(|c| c.provider == "provider_2"),
        "provider_2 should be excluded when should_reprobe returns true"
    );
    assert!(
        !candidates.iter().any(|c| c.provider == "provider_3"),
        "provider_3 should be excluded when should_reprobe returns true"
    );

    // Verify providers passing should_reprobe remain as candidates
    assert!(
        candidates.iter().any(|c| c.provider == "provider_4"),
        "provider_4 should remain a candidate when should_reprobe returns false"
    );
    assert!(
        candidates.iter().any(|c| c.provider == "provider_5"),
        "provider_5 should remain a candidate when should_reprobe returns false"
    );

    // Verify selection selects provider_4 (first non-excluded provider)
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "provider_4",
        "Selection should skip all excluded providers and select provider_4"
    );

    // Clear calls from get_candidate_providers
    mock.clear_calls();

    // Verify selection selects provider_4 (first non-excluded provider)
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "provider_4",
        "Selection should skip all excluded providers and select provider_4"
    );

    // Verify should_reprobe was called for providers in rank order during selection
    let calls = mock.get_should_reprobe_calls();
    assert_eq!(
        calls.len(),
        4,
        "should_reprobe should be called for 4 providers before finding valid one"
    );
    assert_eq!(
        calls[0], "provider_1",
        "First call should be for provider_1"
    );
    assert_eq!(
        calls[1], "provider_2",
        "Second call should be for provider_2"
    );
    assert_eq!(
        calls[2], "provider_3",
        "Third call should be for provider_3"
    );
    assert_eq!(
        calls[3], "provider_4",
        "Fourth call should be for provider_4"
    );
}

#[test]
fn test_all_providers_excluded_when_all_should_reprobe_return_true() {
    // Test behavior when all providers have should_reprobe returning true

    let mock = MockProviderProbe::new(2);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
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

    // Mock all providers to return true for should_reprobe
    mock.set_mock_should_reprobe("provider_a", true);
    mock.set_mock_should_reprobe("provider_b", true);
    mock.set_mock_should_reprobe("provider_c", true);

    // Get candidate providers (should be empty)
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify all providers are excluded
    assert!(
        candidates.is_empty(),
        "All providers should be excluded when should_reprobe returns true for all"
    );

    // Clear calls from get_candidate_providers
    mock.clear_calls();

    // Verify selection returns None
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert!(
        selected.is_none(),
        "Selection should return None when all providers are excluded"
    );

    // Verify should_reprobe was called for all providers
    let calls = mock.get_should_reprobe_calls();
    assert_eq!(
        calls.len(),
        3,
        "should_reprobe should be called for all 3 providers"
    );
    assert!(calls.contains(&"provider_a".to_string()));
    assert!(calls.contains(&"provider_b".to_string()));
    assert!(calls.contains(&"provider_c".to_string()));
}

#[test]
fn test_no_providers_excluded_when_all_should_reprobe_return_false() {
    // Test that no providers are excluded when should_reprobe returns false for all

    let mock = MockProviderProbe::new(3);

    // Create test results with 3 providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
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

    // Mock all providers to return false for should_reprobe
    mock.set_mock_should_reprobe("fast_provider", false);
    mock.set_mock_should_reprobe("medium_provider", false);
    mock.set_mock_should_reprobe("slow_provider", false);

    // Get candidate providers (should include all)
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify all providers remain as candidates
    assert_eq!(
        candidates.len(),
        3,
        "All providers should be candidates when should_reprobe returns false for all"
    );
    assert!(candidates.iter().any(|c| c.provider == "fast_provider"));
    assert!(candidates.iter().any(|c| c.provider == "medium_provider"));
    assert!(candidates.iter().any(|c| c.provider == "slow_provider"));

    // Clear calls from get_candidate_providers
    mock.clear_calls();

    // Verify selection selects the fastest provider (first in rank order)
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "fast_provider",
        "Selection should select fastest provider when none are excluded"
    );

    // Verify should_reprobe was called only once (for first provider)
    let calls = mock.get_should_reprobe_calls();
    assert_eq!(calls.len(), 1, "should_reprobe should be called only once");
    assert_eq!(calls[0], "fast_provider");
}

#[test]
fn test_provider_exclusion_with_mixed_results() {
    // Test provider exclusion when some providers pass and some fail should_reprobe

    let mock = MockProviderProbe::new(2);

    // Create test results with 4 providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "rank1".to_string(),
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
                provider: "rank2".to_string(),
                probe_url: "https://rank2.idx".to_string(),
                connect_ms: 30,
                ttfb_ms: 50,
                throughput_mbs: 18.0,
                score: 105.6,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "rank3".to_string(),
                probe_url: "https://rank3.idx".to_string(),
                connect_ms: 50,
                ttfb_ms: 75,
                throughput_mbs: 12.0,
                score: 178.3,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "rank4".to_string(),
                probe_url: "https://rank4.idx".to_string(),
                connect_ms: 70,
                ttfb_ms: 100,
                throughput_mbs: 8.0,
                score: 255.0,
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

    // Mix of passing and failing should_reprobe
    mock.set_mock_should_reprobe("rank1", true); // excluded
    mock.set_mock_should_reprobe("rank2", false); // candidate
    mock.set_mock_should_reprobe("rank3", true); // excluded
    mock.set_mock_should_reprobe("rank4", false); // candidate

    // Get candidate providers
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify only rank2 and rank4 are candidates
    assert_eq!(candidates.len(), 2, "Should have exactly 2 candidates");
    assert!(candidates.iter().any(|c| c.provider == "rank2"));
    assert!(candidates.iter().any(|c| c.provider == "rank4"));

    // Verify rank1 and rank3 are excluded
    assert!(!candidates.iter().any(|c| c.provider == "rank1"));
    assert!(!candidates.iter().any(|c| c.provider == "rank3"));

    // Clear calls from get_candidate_providers
    mock.clear_calls();

    // Verify selection picks rank2 (first non-excluded provider)
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "rank2",
        "Selection should select rank2 (first non-excluded provider)"
    );

    // Verify should_reprobe was called in rank order during selection
    let calls = mock.get_should_reprobe_calls();
    assert_eq!(
        calls.len(),
        2,
        "should_reprobe should be called for 2 providers during selection"
    );
    assert_eq!(calls[0], "rank1");
    assert_eq!(calls[1], "rank2");
}

#[test]
fn test_provider_exclusion_with_unsuccessful_providers() {
    // Test provider exclusion when some providers have unsuccessful probe results

    let mock = MockProviderProbe::new(2);

    // Create test results with mix of successful and unsuccessful providers
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "successful_but_excluded".to_string(),
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
                error: Some("Connection failed".to_string()),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "successful_and_included".to_string(),
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

    // Mock should_reprobe
    mock.set_mock_should_reprobe("successful_but_excluded", true);
    mock.set_mock_should_reprobe("unsuccessful_provider", false);
    mock.set_mock_should_reprobe("successful_and_included", false);

    // Get candidate providers (should only include successful_and_included)
    let candidates = mock.get_candidate_providers(&results, "test_model");

    // Verify only successful_and_included is a candidate
    assert_eq!(candidates.len(), 1, "Should have exactly 1 candidate");
    assert_eq!(candidates[0].provider, "successful_and_included");

    // Verify unsuccessful_provider is not a candidate (even though should_reprobe returns false)
    assert!(
        !candidates
            .iter()
            .any(|c| c.provider == "unsuccessful_provider"),
        "Unsuccessful providers should not be candidates regardless of should_reprobe"
    );

    // Verify selection picks successful_and_included
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(selected.unwrap().provider, "successful_and_included");
}

#[test]
fn test_provider_exclusion_calls_should_reprobe_for_each_provider() {
    // Test that should_reprobe is called for each provider during exclusion check

    let mock = MockProviderProbe::new(3);

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

    // Mock should_reprobe - first 3 excluded, last 2 included
    for i in 0..3 {
        mock.set_mock_should_reprobe(&format!("provider_{}", i), true);
    }
    for i in 3..5 {
        mock.set_mock_should_reprobe(&format!("provider_{}", i), false);
    }

    // Get candidate providers
    let candidates = mock.get_candidate_providers(&results, "test");

    // Verify candidates
    assert_eq!(candidates.len(), 2, "Should have 2 candidates");

    // Verify should_reprobe was called for all providers
    let calls = mock.get_should_reprobe_calls();
    assert_eq!(
        calls.len(),
        5,
        "should_reprobe should be called for all 5 providers"
    );

    // Verify calls were in rank order
    for i in 0..5 {
        assert_eq!(
            calls[i],
            format!("provider_{}", i),
            "should_reprobe should be called in rank order"
        );
    }
}

#[test]
fn test_provider_exclusion_updates_dynamically() {
    // Test that provider exclusion updates when should_reprobe results change

    let mock = MockProviderProbe::new(2);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "provider_a".to_string(),
                probe_url: "https://a.idx".to_string(),
                connect_ms: 20,
                ttfb_ms: 30,
                throughput_mbs: 20.0,
                score: 70.0,
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

    // Initially, provider_a is excluded
    mock.set_mock_should_reprobe("provider_a", true);
    mock.set_mock_should_reprobe("provider_b", false);

    let candidates = mock.get_candidate_providers(&results, "test_model");
    assert_eq!(candidates.len(), 1, "Should have 1 candidate initially");
    assert_eq!(candidates[0].provider, "provider_b");

    // Update: provider_a is now included, provider_b is excluded
    mock.clear_calls();
    mock.set_mock_should_reprobe("provider_a", false);
    mock.set_mock_should_reprobe("provider_b", true);

    let candidates = mock.get_candidate_providers(&results, "test_model");
    assert_eq!(candidates.len(), 1, "Should have 1 candidate after update");
    assert_eq!(
        candidates[0].provider, "provider_a",
        "Candidate list should update when should_reprobe results change"
    );

    // Update: both providers are included
    mock.clear_calls();
    mock.set_mock_should_reprobe("provider_a", false);
    mock.set_mock_should_reprobe("provider_b", false);

    let candidates = mock.get_candidate_providers(&results, "test_model");
    assert_eq!(
        candidates.len(),
        2,
        "Should have 2 candidates when both are included"
    );

    // Selection should pick provider_a (faster)
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(selected.unwrap().provider, "provider_a");
}

#[test]
fn test_provider_exclusion_with_real_failure_tracking() {
    // Test that provider exclusion works with real failure tracking (not just mocks)

    let mut mock = MockProviderProbe::new(3);

    // Create test results
    let mut models = HashMap::new();
    models.insert(
        "test_model".to_string(),
        vec![
            ProbeResult {
                provider: "s3:provider".to_string(),
                probe_url: "https://s3.idx".to_string(),
                connect_ms: 20,
                ttfb_ms: 30,
                throughput_mbs: 20.0,
                score: 70.0,
                success: true,
                error: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            ProbeResult {
                provider: "gcs:provider".to_string(),
                probe_url: "https://gcs.idx".to_string(),
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

    // Initially, no failures - no providers should be excluded
    mock.clear_mocks(); // Use real implementation
    let candidates = mock.get_candidate_providers(&results, "test_model");
    assert_eq!(
        candidates.len(),
        2,
        "Should have 2 candidates with no failures"
    );

    // Record failures for s3:provider to exceed threshold
    mock.record_failure("s3:provider");
    mock.record_failure("s3:provider");
    mock.record_failure("s3:provider");

    // Clear mocks to use real implementation
    mock.clear_mocks();
    mock.clear_calls();

    // Now s3:provider should be excluded based on real failure tracking
    let candidates = mock.get_candidate_providers(&results, "test_model");
    assert_eq!(
        candidates.len(),
        1,
        "Should have 1 candidate after failures"
    );
    assert_eq!(
        candidates[0].provider, "gcs:provider",
        "gcs:provider should be the only candidate after s3:provider exceeds threshold"
    );

    // Verify selection picks gcs:provider
    let selected = mock.simulate_selection_with_exclusion(&results, "test_model");
    assert_eq!(
        selected.unwrap().provider,
        "gcs:provider",
        "Selection should pick gcs:provider when s3:provider is excluded by real failure tracking"
    );
}
