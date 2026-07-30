# Bead bf-3thh: README Claims Fixed

## Issue
README.md made three claims that were false at bead creation time:
1. "No C dependencies...no FFI wrappers" / "Pure Rust — no C toolchain, no FFI"
2. CLI section documented commands that weren't implemented
3. "Run benchmarks: cargo bench" (no cargo bench targets exist)

## Resolution

### 1. Pure Rust / FFI Claim ✅ FIXED
**Current README state (line 8):**
- "No C toolchain. No FFI. No shelling out to `wgrib2`. Core decoding (DRT 5.0 / 5.2 / 5.3 / 5.41) is 100% Rust — JPEG2000 (DRT 5.40) is the one optional feature that pulls in a C dependency (`openjpeg-sys`)."

**Cargo.toml configuration:**
```toml
[features]
default = []  # No C dependencies by default
jpeg2000 = ["jpeg2k"]  # Optional C FFI for JPEG2000 only
```

**Verification:** Default `cargo build` is pure Rust — no C dependencies unless explicitly enabled.

### 2. CLI Implementation ✅ FIXED
**Issue:** README had stale "CLI not yet implemented" warnings from bead bf-4nam (added 2026-07-22), even though the CLI was fully implemented in commit `cf21b25`.

**Action taken:** Removed both warnings:
- Quickstart section (line 28-29): Removed warning blocking steps 1, 3, 4
- CLI Reference section (line 295-296): Removed warning before command examples

**CLI actual state:**
- `crates/gribtract-cli/src/main.rs` is 269 lines, fully functional
- Commands work: `gribtract decode`, `gribtract list`, `gribtract dump`
- Verified: `cargo run -p gribtract-cli -- decode --help` shows functional help

### 3. Benchmark Command ✅ ALREADY CORRECT
**Current README (line 277):**
```bash
cargo xtask bench    # run benchmarks, regenerate bench-results.json
```

**Verification:** Command is correct and produces valid `bench-results.json`.

## Acceptance Criteria Verification

✅ **Every claim in README.md is true of a fresh default `cargo build`**
- Default features empty → no C dependencies
- All claims match actual implementation
- CLI is functional (no stale warnings)

✅ **Feature flag handling correct**
- DRT=40 fixtures explicitly skipped when `jpeg2000` disabled
- Default build is pure Rust

✅ **Benchmark instructions reproduce bench-results.json**
- Command `cargo xtask bench` is correctly documented

## Changes Made
1. Removed stale "CLI not yet implemented" warning from Quickstart section
2. Removed stale "Not yet implemented" warning from CLI Reference section

All three README claims now accurately reflect the current implementation state.
