# Bead bf-3thh: README Claims Verification

## Issue
README.md made three claims that were false at bead creation time:
1. "No C dependencies...no FFI wrappers" / "Pure Rust — no C toolchain, no FFI"
2. CLI section documented commands that weren't implemented
3. "Run benchmarks: cargo bench" (no cargo bench targets exist)

## Verification Status: ✅ ALL FIXED

### 1. Pure Rust / FFI Claim — RESOLVED
**Current README state:**
- Line 5: "JPEG2000 (DRT 5.40) requires the optional `jpeg2000` feature, which pulls in `openjpeg-sys` (C FFI)"
- Line 19: "Pure Rust for core formats — no C toolchain or FFI for DRT 5.0, 5.2, 5.3, 5.41...JPEG2000 (DRT 5.40) requires the optional `jpeg2000` feature, which uses `openjpeg-sys` (C FFI)"

**Cargo.toml:**
```toml
[features]
default = []  # No features by default
jpeg2000 = ["jpeg2k"]
```
**Result:** ✅ Default `cargo build` is pure Rust — no C dependencies unless explicitly enabled

### 2. CLI Implementation — RESOLVED
**Current state:**
- `crates/gribtract-cli/src/main.rs` is fully implemented (269 lines)
- Commands: `decode`, `list`, `dump` — all functional
- Commit `cf21b25` implemented these on 2026-06-22

**Result:** ✅ CLI section documentation is accurate

### 3. Benchmark Command — RESOLVED
**Current README (line 173-174):**
```bash
# Run benchmarks and regenerate bench-results.json
cargo xtask bench
```
**Verified:** Command runs successfully and produces `bench-results.json`
(Uses cargo alias: `xtask = "run --bin xtask --"` in `.cargo/config.toml`)

**Result:** ✅ Correct command documented (not `cargo bench`)

## Acceptance Criteria Verification

✅ **Every claim in README.md is true of a fresh default `cargo build`**
- Default features empty → no C dependencies
- All claims match actual implementation

✅ **Feature flag handling correct**
- DRT=40 fixtures explicitly skipped when `jpeg2000` disabled (`differential.rs:27-33`)
- Differential suite passes with feature enabled (100% agreement)
- Fixtures skipped, not silently failed (clear `[skip-drt40-no-feature]` message)

✅ **Benchmark instructions reproduce bench-results.json**
- Command `cargo run --bin xtask -- bench` generates valid `bench-results.json`
- Dashboard displays results correctly

## Conclusion
All three issues were resolved in prior commits:
- README updated to accurately document optional C dependency
- CLI fully implemented (commit `cf21b25`)
- Benchmark command corrected

No further action required. All acceptance criteria met.
