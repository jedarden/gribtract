# bf-4gd44: Ensemble Fixture Differential Integration

## Task
Wire ensemble fixture into differential test suite.

## Status: COMPLETE

The ensemble fixture was **already fully integrated** into the differential suite.

## Findings

### Files Referenced in Task
- Task mentioned `tests/differential_main.rs` — **this file does not exist**
- Actual test file: `crates/gribtract/tests/differential.rs`

### Integration Architecture
The differential test (`differential.rs`) uses `corpus::list_fixtures()` to **automatically include all fixtures** from the corpus manifest (`tests/corpus/manifest.json`). There is no explicit "test matrix" to modify.

### Ensemble Fixture Status
✅ **Already integrated** — All components in place:

1. **Corpus manifest entry**: `pdt1_ensemble_3x2` (line 70-81)
   - Storage: `inline` (local file)
   - Path: `small/pdt1_ensemble_3x2.grib2`

2. **GRIB2 fixture file**: `tests/corpus/small/pdt1_ensemble_3x2.grib2`
   - Size: 188 bytes
   - SHA256: efc2bd63db399e592419c1ef1a8f780bb93e78be8a1954e1900e19789350b0fa

3. **Golden reference**: `tests/corpus/golden/pdt1_ensemble_3x2.json`
   - Contains full field metadata with ensemble information
   - PDT 4.1 (individual ensemble member)
   - Ensemble type=2 (negatively perturbed), number=3

4. **Differential test execution**: Shows `[match]` (100% agreement)

### Test Output (2026-07-23)
```
  [match]      pdt1_ensemble_3x2
```

Per-template breakdown:
```
  GDT=0 PDT=1 DRT=0: 1/1
```

## No Code Changes Required

The differential suite architecture means that **any fixture added to the corpus manifest with a golden reference automatically participates** in differential testing. The ensemble fixture was already properly configured.

## Verification
Run: `cargo test --test differential`

The ensemble fixture runs and matches its golden reference at 100% agreement.
