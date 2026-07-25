# GFS Fixture Integration Roadmap

**Document Version:** 2026-07-25  
**Bead:** bf-658687  
**Purpose:** Detailed roadmap for completing GFS fixture integration

## Current Fixture Status

| Fixture | Size | Golden | Test Status | Blocker | Priority |
|---------|------|--------|------------|---------|----------|
| `gfs_anl_t2m_5x5` | 204B | ✅ Yes | ✅ Pass | None | - |
| `gfs_tmp2m_1deg_anl` | 47KB | ✅ Yes | ✅ Pass | None | - |
| `gfswave_arctic_wind_drt40` | 418KB | ✅ Yes | ✅ Pass | None | - |
| `gfs_conus_drt0_0p50` | 152MB | ❌ No | ⚠️ Skip | No golden | 🔴 HIGH |
| `gfs_gaussian_gdt40_t1534` | 127MB | ✅ Yes | ⚠️ Partial | Validation | 🟠 MEDIUM |

**Integration Progress: 60% complete (3/5 fixtures fully integrated)**

## Immediate Actions Required

### 🔴 CRITICAL: Generate Missing Golden Reference

**Fixture:** `gfs_conus_drt0_0p50`  
**File:** `tests/corpus/large/gfs.t00z.pgrb2.0p50.f000`  
**Size:** 152 MB  
**Fields:** 696 GRIB2 messages  

**Steps:**
```bash
# 1. Verify eccodes is available
which grib_dump

# 2. Generate golden reference (expect 30-60 minutes)
python3 scripts/gen_golden.py \
  tests/corpus/large/gfs.t00z.pgrb2.0p50.f000 \
  gfs_conus_drt0_0p50 \
  --output-dir tests/corpus/golden/

# 3. Verify output
ls -lh tests/corpus/golden/gfs_conus_drt0_0p50.json
# Expected: ~400-500 MB file

# 4. Test integration
cargo test differential --release
```

**Expected Outcome:**
- Golden file created (~400-500 MB)
- Differential test includes fixture (no longer skipped)
- Agreement percentage increases significantly
- All 696 fields decode successfully

**Time Estimate:** 1-2 hours  
**Dependencies:** eccodes CLI tools, Python 3, 500MB disk space

---

### 🟠 MEDIUM: Validate Gaussian Fixture

**Fixture:** `gfs_gaussian_gdt40_t1534`  
**Status:** Golden exists but needs full validation  

**Steps:**
```bash
# 1. Run full differential test
cargo test differential --release -- --nocapture

# 2. Check specific fixture results
grep "gfs_gaussian_gdt40_t1534" test-output.log

# 3. Verify all 54 fields match golden
cargo test differential_gaussian --release
```

**Expected Outcome:**
- All 54 fields pass differential test
- Performance metrics documented
- Integration documentation updated

**Time Estimate:** 1 hour

---

## Technical Implementation Notes

### Golden Reference Generation

**Process:**
1. `grib_dump` extracts metadata and data from GRIB2
2. Python script transforms to golden JSON format
3. Output written to `tests/corpus/golden/<fixture_id>.json`
4. Differential suite auto-discovers and tests fixture

**Format Requirements:**
- Valid JSON matching schema in `docs/golden-json-schema.md`
- All required fields present (parameter, grid, values, etc.)
- Values array in row-major order
- Proper template numbers (GDT, PDT, DRT)

### Dependencies Summary

**Required for All Fixtures:**
- Rust 1.75+ (workspace minimum)
- cargo (build system)
- Standard library only

**Required for Golden Generation:**
- Python 3.x
- eccodes CLI tools (`grib_dump`)
- ~500MB free disk space (for large golden files)

**Optional Dependencies:**
- `openjpeg-sys` (for JPEG2000/DRT=40 fixtures)
- `jpeg2000` feature flag required for `gfswave_arctic_wind_drt40`

### Build System Integration

**Current Build Status:**
```bash
$ cargo build --release
# ✅ PASSING - all 7 workspace crates compile

$ cargo test --lib
# ✅ PASSING - 53 tests pass

$ cargo test differential --release
# ⚠️ REGRESSION - 61.5% vs 84.0% floor
```

**Expected Post-Integration:**
```bash
$ cargo test differential --release
# ✅ PASSING - 85%+ agreement (estimated)
```

---

## Verification Checklist

### Pre-Integration Checklist

- [x] Build system compiles without errors
- [x] Basic GRIB2 decoding works
- [x] Golden generation tool functional
- [x] Eccodes CLI available
- [x] Test harness discovers fixtures correctly
- [x] Manifest metadata accurate
- [x] SHA-256 hashes verified

### Post-Integration Checklist

#### For `gfs_conus_drt0_0p50`:
- [ ] Golden file exists in `tests/corpus/golden/`
- [ ] Golden file is valid JSON
- [ ] Golden file contains all 696 fields
- [ ] Differential test includes fixture (not skipped)
- [ ] All fields decode successfully
- [ ] Agreement percentage increases
- [ ] No memory issues during decode
- [ ] Performance is acceptable

#### For `gfs_gaussian_gdt40_t1534`:
- [ ] All 54 fields pass differential test
- [ ] Performance metrics documented
- [ ] Integration notes updated
- [ ] No decode errors or warnings

---

## Risk Assessment

### Technical Risks

**Low Risk:**
- Core decoders already tested and working
- Golden generation process proven (works for 3 fixtures)
- Build system stable and reliable

**Medium Risk:**
- Large golden files may stress I/O systems
- Memory usage during decode of 152MB file
- Potential timeout in CI for large fixtures

**High Risk:**
- None identified

### Mitigation Strategies

1. **Large File Handling:**
   - Use lazy decode where possible
   - Increase CI timeout if needed
   - Monitor memory usage during testing

2. **Golden Validation:**
   - Verify JSON structure before commit
   - Check file size expectations
   - Test on subset of fields first

3. **CI Integration:**
   - Test locally before pushing
   - Use incremental testing approach
   - Have rollback plan ready

---

## Success Metrics

### Integration Success Criteria

**Phase 1 (GFS CONUS):**
- ✅ Golden file exists and is valid
- ✅ Differential test passes for fixture
- ✅ Agreement ≥ 84% (restores floor)
- ✅ No test failures or timeouts
- ✅ Performance acceptable

**Phase 2 (Gaussian Validation):**
- ✅ All 54 fields pass differential test
- ✅ Performance documented
- ✅ Integration notes updated

**Overall Success:**
- ✅ 5/5 fixtures fully integrated (100%)
- ✅ All differential tests pass
- ✅ Agreement ≥ 85%
- ✅ Documentation complete
- ✅ No known critical issues

### Performance Targets

**Decode Performance:**
- Small fixtures (<1MB): <1 second
- Medium fixtures (1-50MB): <10 seconds
- Large fixtures (50-200MB): <60 seconds

**Memory Usage:**
- Peak memory < 2× file size
- No memory leaks during decode
- Clean deallocation after test

---

## Related Documentation

**Integration Status:**
- `docs/gfs-integration-status.md` - Comprehensive integration overview
- `docs/gfs-fixture-integration-status.md` - Detailed fixture status

**Technical References:**
- `docs/golden-json-schema.md` - Golden file format specification
- `tests/corpus/manifest.json` - Fixture metadata and provenance
- `scripts/gen_golden.py` - Golden generation tool

**Test Infrastructure:**
- `crates/gribtract/tests/differential.rs` - Differential test harness
- `crates/gribtract-testutil/src/corpus.rs` - Fixture loading
- `crates/gribtract-testutil/src/golden.rs` - Golden utilities

---

## Maintenance Notes

### Update Triggers

This roadmap should be updated when:
- Golden references are generated
- Fixtures are added or removed
- Test status changes significantly
- New integration issues discovered
- Performance targets updated

### Review Schedule

- **Weekly:** During active integration work
- **Monthly:** During maintenance phase
- **Quarterly:** For comprehensive review

### Contact Points

- **Technical Issues:** gribtract project maintainers
- **Documentation Updates:** Update this file directly
- **Test Failures:** Check differential test output first

---

**Roadmap Version:** 1.0  
**Last Updated:** 2026-07-25  
**Next Milestone:** Complete `gfs_conus_drt0_0p50` integration (expected: 1-2 hours)