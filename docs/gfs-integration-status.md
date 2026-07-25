# GFS Integration Status and Remaining Work

**Document Version:** 2026-07-25  
**Bead:** bf-56y2pd  
**Status:** Partial Integration Complete

## Overview

GFS (Global Forecast System) integration is **partially complete**. Basic GFS fixtures work, but several critical gaps remain in Gaussian-grid support, test coverage, and infrastructure reliability.

## Current Status Summary

### ✅ Working Components

| Component | Status | Notes |
|-----------|--------|-------|
| Basic GFS decoding | ✅ Working | `gfs_anl_t2m_5x5`, `gfs_tmp2m_1deg_anl` pass differential tests |
| GFS provider support | ✅ Implemented | S3, GCS, NOMADS endpoints in `gribtract-fetch/src/provider.rs` |
| Gaussian grid (GDT 3.40) parsing | ✅ Implemented | Core parser in `gribtract-core/src/decode.rs` |
| JPEG2000 (DRT 5.40) | ✅ Implemented | Requires `jpeg2000` feature flag |
| GFS 0.50° CONUS fixture | ✅ Available | `gfs.t00z.pgrb2.0p50.f000` (145 MB, fetched locally) |

### ❌ Critical Issues

| Issue | Severity | Component | Impact |
|-------|----------|-----------|--------|
| CORe Gaussian-grid decode error | 🔴 HIGH | `core_gaussian_gdt40` fixture | Fails with "decode not implemented" |
| Missing golden references | 🟠 MEDIUM | 2 remote fixtures | Cannot verify correctness |
| Hardcoded stale probe dates | 🟠 MEDIUM | Provider probe system | Runtime re-probe fallback broken |
| Rotated lat/lon grid not implemented | 🟡 LOW | Grid projection | Some regional products will fail |

## Known Issues and Limitations

### 1. CORe Gaussian-Grid Fixture Decode Failure

**Fixture:** `core_gaussian_gdt40` (flx.2024011500.grib2, 10.5 MB)  
**Error:** `decode not implemented`  
**Status:** 🔴 CRITICAL

**Details:**
- File is valid GRIB2 (verified via `wgrib2`)
- Contains 30 fields, all using GDT 3.40 (Gaussian Latitude/Longitude grid)
- Grid specification: 512×256 points, N=128, 131,072 data points per field
- wgrib2 successfully parses all fields
- gribtract returns generic "decode not implemented" error

**Evidence:**
```bash
$ wgrib2 -v flx.2024011500.grib2 | head -5
1:0:d=2024011500:DLWRF Downward Long-Wave Rad. Flux [W/m^2]:surface:anl:ens mean
2:174940:d=2024011500:ULWRF Upward Long-Wave Rad. Flux [W/m^2]:surface:anl:ens mean
...

$ cargo test diagnose_core_gaussian_gdt40
thread 'diagnose_core_gaussian_gdt40' panicked at:
decode not implemented
```

**Impact:** CORe archive flux files cannot be decoded, breaking a major NOAA data source.

**Root Cause:** Unknown. GDT 3.40 parsing exists in the codebase, but something in the decode chain is failing for this specific file structure.

### 2. Missing Golden References

**Fixtures Affected:**
- `gfs_gaussian_gdt40_t1534` (gdas.t00z.sfluxgrbf000.grib2, 122 MB)
- `gfs_conus_drt0_0p50` (gfs.t00z.pgrb2.0p50.f000, 145 MB)

**Status:** 🟠 MEDIUM

**Details:**
- Both fixtures are fetched locally and verified (SHA-256 matches)
- Both files are valid GRIB2 (verified via `wgrib2`)
- Golden JSON references do not exist in `tests/corpus/golden/`
- Differential test reports `[no-golden]` and skips these fixtures

**Impact:** Cannot verify correctness of decoding for these fixtures. They are excluded from the differential coverage report, artificially inflating the agreement percentage.

**Work Required:**
1. Generate golden references using eccodes/wgrib2 on internal cluster
2. Add to `tests/corpus/golden/` with corresponding `.json` filenames
3. Wire into differential test suite

### 3. ProviderProbe Hardcoded Stale Date

**Component:** `gribtract-fetch/src/probe.rs`  
**Status:** 🟠 MEDIUM  
**Bead:** bf-15zl

**Issue:** `ProviderProbe::default_probe_files()` uses hardcoded date `20250702` in all probe URLs:
```rust
// Example probe URLs (all use stale 20250702 date):
"https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20250702/conus/hrrr.t00z.wrfsfcf00.grib2.idx"
"https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20250702/00/atmos/gfs.t00z.pgrb2.0p50.f000.idx"
```

**Impact:** 
- All probe URLs now 404 (files outside NOAA retention window)
- Runtime live re-probe fallback silently fails
- System cannot recover from stale or missing probe data
- Contradicts documented reliability: "falls back to re-probing live if stale (>24h)"

**Correct Pattern:** `xtask/src/probe_providers.rs` uses dynamic date computation:
```rust
fn days_ago(days: i64) -> String {
    let date = chrono::Utc::now() - Duration::days(days);
    date.format("%Y%m%d").to_string()
}
```

**Work Required:**
1. Replace hardcoded dates with dynamic computation in `default_probe_files()`
2. Add unit test asserting probe URLs contain recent dates
3. Consider extracting date logic to shared helper

### 4. Rotated Lat/Lon Grid Not Implemented

**Component:** Grid projection support  
**Status:** 🟡 LOW (limited impact)  
**Bead:** bf-15g8

**Issue:** Grid Definition Template 1 (rotated lat/lon) returns `NotImplemented` error:
```rust
// crates/gribtract-core/src/decode.rs
match template_number {
    0 => parse_grid_3_0_latlon(...),
    // ... 
    40 => parse_grid_3_40_gaussian(...),
    _ => Err(DecodeError::NotImplemented), // Template 1 hits this
}
```

**Impact:** Regional models using rotated-pole grids (common in limited-area ensemble products) will fail at decode time.

**Work Required:** Full implementation following existing pattern:
1. Add `RotatedLatLon` variant to `GridProjection` enum
2. Implement template 1 parser in `decode.rs`
3. Add nearest-point-index lookup for rotated coordinates
4. Add synthetic fixture via generator script
5. Wire into differential test suite

## Testing Gaps

### Differential Test Coverage

**Current Differential Results (2026-07-25):**
```
[match]      gfs_anl_t2m_5x5              ✅
[match]      gfs_tmp2m_1deg_anl          ✅
[decode-err] core_gaussian_gdt40         ❌
[no-golden]  gfs_gaussian_gdt40_t1534   ⚠️
[no-golden]  gfs_conus_drt0_0p50        ⚠️
```

**Agreement Floor:** 84.0% (artificially high due to skipped fixtures)

**Missing Tests:**
- No integration test for `core_gaussian_gdt40` decode failure
- No test coverage for runtime probe re-probe fallback
- No test for rotated lat/lon grid (not implemented)

### Open Beads Analysis

**Open GFS-related beads:**
- `bf-1tov`: Source and add real Gaussian-grid fixture to corpus
- `bf-23h38`: Generate golden references for ensemble, Gaussian, and CONUS DRT=0 fixtures
- `bf-2o53`: Expand differential corpus with real remote-stored fixtures
- `bf-2pev44`: Verify GFS fixture integration readiness
- `bf-5tz5p`: Verify cargo test passes for GFS Gaussian-grid fixture
- `bf-91ov1`: Wire GFS Gaussian-grid fixture into differential suite

**Blocked beads (dependencies):**
- `bf-5ysjo`: Add GFS Gaussian-grid fixture to differential.rs (blocked by bf-5lybk, bf-1nnawg)
- `bf-6brqqn`: Add GFS Gaussian-grid test case to differential.rs (blocked by bf-x6om68)
- `bf-x6om68`: Locate and understand GFS Gaussian-grid fixture (blocked by bf-4mlnd3)

**Dependency Chain:** Many beads are blocked waiting for golden references and successful decode of existing fixtures.

## Integration Checklist

### Phase 1: Fix Critical Decode Issues 🔴

- [ ] **Investigate `core_gaussian_gdt40` decode failure**
  - [ ] Add detailed error logging to decode path
  - [ ] Verify GDT 3.40 parser handles all template variations
  - [ ] Check DRT (Data Representation Template) for this fixture
  - [ ] Test with `wgrib2` to identify template specifics
  - [ ] Fix root cause of "decode not implemented" error

- [ ] **Generate missing golden references**
  - [ ] Run eccodes/wgrib2 on `gfs_gaussian_gdt40_t1534` (122 MB)
  - [ ] Run eccodes/wgrib2 on `gfs_conus_drt0_0p50` (145 MB)
  - [ ] Copy golden JSON to `tests/corpus/golden/`
  - [ ] Verify SHA-256 matches fixture manifest

### Phase 2: Fix Infrastructure Issues 🟠

- [ ] **Fix ProviderProbe hardcoded dates**
  - [ ] Replace `20250702` literals with dynamic date computation
  - [ ] Add unit test for recent-date probe URLs
  - [ ] Test runtime re-probe fallback with synthetic staleness
  - [ ] Update documentation to reflect correct behavior

- [ ] **Wire GFS fixtures into differential suite**
  - [ ] Add test cases to `differential.rs` for all 3 GFS fixtures
  - [ ] Update agreement floor after fixtures pass
  - [ ] Remove skip conditions once golden references exist

### Phase 3: Complete Grid Projection Support 🟡

- [ ] **Implement rotated lat/lon grid (GDT 3.1)**
  - [ ] Add `RotatedLatLon` variant to `GridProjection`
  - [ ] Implement template 1 parser
  - [ ] Add rotated-coordinate nearest-point lookup
  - [ ] Generate synthetic fixture
  - [ ] Add to differential suite

### Phase 4: Comprehensive Testing ✅

- [ ] **Add integration test coverage**
  - [ ] Test all GFS fixtures in differential suite
  - [ ] Test provider probe re-probe fallback
  - [ ] Test lazy O(1) point extraction for GFS grids
  - [ ] Benchmark decode performance for large GFS files

## Testing Needed

### Additional Testing Requirements

1. **CORe Decode Failure Root Cause Analysis**
   - Need detailed error logging in decode path
   - Compare byte-level parsing with wgrib2 behavior
   - Identify which section/template is causing failure

2. **Golden Reference Generation**
   - Requires access to internal eccodes cluster (bead bf-23h38)
   - Or: local wgrib2 installation for small fixtures
   - JSON format must match existing golden files

3. **Integration Testing**
   - Test provider probe fallback with simulated staleness
   - Test HTTP error recovery and consecutive-failover trigger
   - End-to-end test: fetch → decode → verify → extract stations

4. **Performance Testing**
   - Large file decode performance (122 MB, 145 MB fixtures)
   - Memory usage during decode
   - Lazy point extraction benchmark for GFS grids

## TODOs and FIXMEs

### Code TODOs (None Found)

No direct TODO/FIXME comments found in GFS-related code. Issues are tracked via beads rather than inline comments.

### Architectural TODOs

1. **Error Messages:** "decode not implemented" is not actionable. Need to specify which template/section failed.
2. **Date Handling:** Provider probe system should use centralized date utilities, not hardcoded strings.
3. **Fixture Management:** Need tooling to automate golden reference generation for new fixtures.

## Appendix: Fixture Details

### `core_gaussian_gdt40`

**Path:** `tests/corpus/large/flx.2024011500.grib2`  
**Size:** 10.5 MB  
**Source:** NOAA CORe Archive (Google Cloud Storage)  
**Grid:** Gaussian 512×256, N=128, 131,072 points  
**Fields:** 30 flux and surface variables  
**Status:** ❌ Decode fails with "decode not implemented"

### `gfs_gaussian_gdt40_t1534`

**Path:** `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`  
**Size:** 122 MB  
**Source:** NOAA NOMADS  
**Grid:** Gaussian 3072×1536, N=768, 4.7M points  
**Fields:** 54 surface flux fields  
**Status:** ⚠️ No golden reference (cannot verify correctness)

### `gfs_conus_drt0_0p50`

**Path:** `tests/corpus/large/gfs.t00z.pgrb2.0p50.f000`  
**Size:** 145 MB  
**Source:** NOAA GFS S3 bucket  
**Grid:** Regular lat/lon 720×361, 0.50° resolution  
**Fields:** 696 GRIB2 records (multiple levels/variables)  
**Status:** ⚠️ No golden reference (cannot verify correctness)

## References

**Related Beads:**
- bf-15g8: Rotated lat/lon grid implementation
- bf-15zl: ProviderProbe hardcoded date bug
- bf-23h38: Generate golden references for missing fixtures
- bf-2o53: Expand differential corpus with remote fixtures

**Documentation:**
- `README.md`: Claims GFS support is complete (outdated)
- `crates/gribtract-fetch/README.md`: Provider probe documentation
- `tests/corpus/manifest.json`: Fixture metadata and provenance

**Test Files:**
- `crates/gribtract/tests/diagnose_gfs_gaussian.rs`: Diagnostic test for GFS Gaussian fixtures
- `crates/gribtract/tests/differential.rs`: Main differential test harness
