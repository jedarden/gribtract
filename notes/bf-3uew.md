# Bead bf-3uew: Generate golden outputs and integrate DRT=3 into differential suite

## Completion Status: ✅ COMPLETE

All acceptance criteria have been met. This work was completed by the dependency beads bf-4p7j0 (end-to-end decode testing) and bf-1cf8 (differential suite validation).

## Verification Summary

### 1. Golden Outputs Generated ✅
- **File**: `tests/corpus/golden/nam_awip12_lambert_drt3.json`
- **Size**: 1.16 GB
- **Content**: 187 fields from nam_awip12_lambert_drt3 fixture
- **Generation**: Created via `scripts/gen_golden.py` using eccodes CLI tools

### 2. Fixture Integrated into Differential Suite ✅
- **Manifest entry**: `nam_awip12_lambert_drt3` in `tests/corpus/manifest.json` (line 122-134)
- **Storage**: `remote` (26.3 MiB file fetched to `tests/corpus/large/`)
- **Test integration**: Automatically included in differential test via corpus::list_fixtures()
- **Test path**: `crates/gribtract/tests/differential.rs::differential_coverage_report()`

### 3. Differential Suite Results ✅
```
=== Differential Harness Coverage ===
Fixtures : 12 total  (8 comparable, 2 no-golden, 2 skipped-feature, 0 skipped-remote-not-fetched)
  matched      : 8
  decode errors: 0
Agreement: 8/8 (100.0%)
Per-template:
  GDT=30 PDT=0 DRT=3: 187/187
  GDT=30 PDT=8 DRT=3: 9/9
```

### 4. All Tests Pass ✅
- **Test result**: `ok. 1 passed; 0 failed`
- **Execution time**: ~52 seconds
- **No ratchet needed**: 100% agreement achieved
- **No integration issues**: All fixtures decode successfully

## Technical Details

### Fixture Characteristics
- **Source**: NOAA NAM awip12 (NCEP Grid 218, Lambert Conformal Conic)
- **Template**: GDT=3.30 (Lambert Conformal) + DRT=3 (complex packing with spatial differencing)
- **Grid**: 614 x 428 points (262,792 total)
- **Messages**: 196 GRIB2 messages (187 tested in differential suite)
- **Performance**: 50 MiB/s decode rate

### Integration Architecture
The differential suite automatically discovers and tests all fixtures in the corpus manifest:
1. `corpus::list_fixtures()` loads `tests/corpus/manifest.json`
2. For each fixture, it loads both the GRIB2 bytes and golden JSON
3. Decodes the GRIB2 using `gribtract::decode()`
4. Compares decoded fields against golden reference using `compare_fixture()`
5. Reports agreement percentage and per-template breakdown

## Dependencies Completed
- ✅ `bf-fouy`: Fetch and verify remote storage for DRT=3 fixture
- ✅ `bf-1cf8`: Run differential suite and ratchet DRT=3 results (100% agreement)
- ✅ `bf-4p7j0`: End-to-end decode testing and documentation

## Notes
- The task description mentioned `tests/differential_main.rs`, but the actual file is `crates/gribtract/tests/differential.rs`
- All dependencies were completed prior to this bead being claimed
- The differential suite already has `AGREEMENT_FLOOR: 100.0` and passes cleanly
- No additional work was required - this bead serves as confirmation that the integration is complete

## Related Documentation
- `docs/bf-4p7j0-nam-lambert-final-state.md` - End-to-end decode verification results
- `tests/corpus/manifest.json` - Fixture manifest with provenance notes
- `tests/corpus/golden/nam_awip12_lambert_drt3.json` - Golden reference output
