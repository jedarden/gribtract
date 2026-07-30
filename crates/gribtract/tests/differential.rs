//! Differential harness: decodes every inline corpus fixture and compares
//! against its golden reference output.
//!
//! Agreement percentage tracks how many fixtures match their golden references.
//! As templates are implemented and matching improves, the ratchet assertion
//! below (AGREEMENT_FLOOR) is raised.

use gribtract_testutil::corpus;
use gribtract_testutil::diff::{compare_fixture, CoverageReport};
use gribtract_testutil::golden;

/// Minimum acceptable agreement percentage. Raise this as templates are implemented.
/// Ratcheted to 100.0% after wiring the GFS Gaussian fixture (bf-91ov1): every
/// comparable (golden-bearing, feature-available) fixture now matches its golden
/// reference — 8/8 by default, 9/9 with the `jpeg2000` feature. The last gap was
/// `gen_golden.py`'s Section-5 packing-key omission (`grib_dump -j` drops
/// referenceValue/bits-per-value), which left the PDT=1 golden mismatched; the
/// script now pulls those keys via `grib_ls -p`, regenerating the affected goldens.
const AGREEMENT_FLOOR: f64 = 100.0;

#[test]
fn differential_coverage_report() {
    let fixtures = corpus::list_fixtures().expect("corpus manifest must load");

    let mut report = CoverageReport::default();

    for entry in &fixtures {
        // Skip remote fixtures that haven't been fetched locally.
        // Remote fixtures that are present locally (via `cargo xtask corpus fetch`)
        // will participate in the differential test.
        if entry.storage == "remote" && !corpus::is_available_locally(entry) {
            eprintln!("  [skip-remote-not-fetched] {}", entry.id);
            report.fixtures_skipped_remote += 1;
            continue;
        }

        // Count all fixtures that will run (inline + fetched remote) in fixtures_total first,
        // so that fixtures_total - fixtures_no_golden correctly computes the comparable count.
        report.fixtures_total += 1;

        // Skip DRT=40 (JPEG2000) fixtures when jpeg2000 feature is disabled
        #[cfg(not(feature = "jpeg2000"))]
        if entry.id.contains("drt40") {
            eprintln!("  [skip-drt40-no-feature] {}", entry.id);
            report.fixtures_skipped_feature += 1;
            continue;
        }

        // Load golden reference — absence is not an error at this phase.
        let golden_fixture = match golden::load_golden(&entry.id) {
            Ok(Some(g)) => g,
            Ok(None) => {
                eprintln!("  [no-golden] {}", entry.id);
                report.fixtures_no_golden += 1;
                continue;
            }
            Err(e) => panic!("failed to load golden for '{}': {}", entry.id, e),
        };

        // Load raw bytes.
        let bytes = corpus::load(&entry.id)
            .unwrap_or_else(|e| panic!("failed to load fixture '{}': {}", entry.id, e));

        // Attempt to decode.
        match gribtract::decode(&bytes) {
            Err(e) => {
                eprintln!("  [decode-err] {} — {}", entry.id, e);
                report.fixtures_decode_error += 1;
            }
            Ok(actual_fields) => {
                let matched = compare_fixture(&actual_fields, &golden_fixture.fields, &mut report);
                if matched {
                    report.fixtures_matched += 1;
                    eprintln!("  [match]      {}", entry.id);
                } else {
                    eprintln!("  [mismatch]   {}", entry.id);
                }
            }
        }
    }

    report.print_report();

    assert!(
        report.agreement_pct() >= AGREEMENT_FLOOR,
        "agreement regression: {:.1}% < floor {:.1}%",
        report.agreement_pct(),
        AGREEMENT_FLOOR,
    );
}
