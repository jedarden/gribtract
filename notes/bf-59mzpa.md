# Bead bf-59mzpa: Manual Test Workflow Artifact Verification

## Workflow Submitted
**Workflow Name:** `gribtract-ci-manual-wzpr6`
**Submitted:** 2026-07-29 01:14 UTC
**Status:** Failed (exit code 1)

## Investigation Results

The workflow failed during the `cargo test` phase. Local investigation revealed:

1. **Root Cause:** Missing golden test fixture files for GEFS tests
   - Tests in `crates/gribtract/tests/diagnose_gefs.rs` expect golden files at `tests/corpus/golden/`
   - Required files: `gefs_member01_pdt41.json` and `gefs_ensemble_mean_pdt48.json`
   - These files do not exist in the repository

2. **Test Failure Details:**
   ```
   thread 'diagnose_gefs_member01_pdt41' panicked at crates/gribtract/tests/diagnose_gefs.rs:13:10:
   golden loaded

   thread 'diagnose_gefs_ensemble_mean_pdt48' panicked at crates/gribtract/tests/diagnose_gefs.rs:102:10:
   golden loaded
   ```

3. **Impact:** The workflow cannot complete successfully because `cargo test` fails before it reaches the artifact generation phase (`cargo run --bin xtask -- bench`)

## Local Artifact Generation
Despite the CI failure, the xtask bench command works locally and produces valid artifacts:
- `bench-results.json` (2.7 KB)
- `dashboard.html` (121 KB)

## Next Steps
To make the workflow succeed:
1. Generate missing golden files for GEFS fixtures
2. Verify all golden files are checked into the repository
3. Re-run the workflow to complete artifact generation and verification

## Workflow Template Reference
The workflow template used is `gribtract-ci` in the `iad-ci` cluster, which:
1. Installs Rust toolchain
2. Clones gribtract repository
3. Runs `cargo test` (currently failing)
4. Runs `cargo run --bin xtask -- bench` to generate artifacts
5. Verifies artifacts exist and outputs them
