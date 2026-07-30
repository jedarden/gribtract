# DRT=3 Fixture Validation Report

Generated: 2026-07-23
Task: bf-10gw3 - Validate DRT=3 fixtures pass correctly

## Summary

✅ **All DRT=3 fixtures with golden references passed with 100% agreement**

## Test Execution

Command: `cargo test --test differential -- --nocapture`
Test Duration: 23.45s
Overall Result: **PASS** (100% agreement across all comparable fixtures)

## DRT=3 Fixtures Tested

### 1. ✅ gfs_tmp2m_1deg_anl (Inline Fixture)

- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 0 (lat/lon grid)
- **PDT**: 0 (analysis)
- **Grid**: 360x181 global lat/lon (65160 points), 1° spacing
- **Messages**: 1
- **Source**: NOAA NCEP GFS (2026-06-18 cycle)
- **Result**: **PASS** - 1/1 messages matched
- **Status**: ✅ First real-world DRT=3 fixture to pass (per bf-54alb inventory)

### 2. ✅ nam_awip12_lambert_drt3 (Multi-Message Remote Fixture)

- **DRT**: 3 (complex packing with spatial differencing)
- **GDT**: 30 (Lambert Conformal Conf)
- **PDT**: 0/8 (mixed analysis and time-processed)
- **Grid**: 614x428 (262,792 points), NCEP Grid 218
- **Storage**: Remote (25.1 MiB, fetched via cargo xtask corpus fetch)
- **Golden**: ✅ Yes (generated from eccodes)
- **Messages**: 196 total
  - 187 messages: GDT 3.30 + PDT 0 + DRT 3 → **187/187 matched**
  - 9 messages: GDT 3.30 + PDT 8 + DRT 3 → **9/9 matched**
- **Source**: NOAA NAM awip12 (2025-01-15 00z)
- **Result**: **PASS** - 196/196 messages matched (100%)
- **Status**: ✅ Large multi-message fixture tests DRT=3 + Lambert decoder coverage

## DRT=3 Fixtures Not Yet Tested

The following DRT=3 fixtures have golden references but were not included in this validation:

### ⚠️ nam_awip12_lambert_drt3_20250120
- **DRT**: 3 (complex packing with spatial differencing)
- **Storage**: Remote (26.3 MiB)
- **Golden**: ❌ No golden generated yet
- **Messages**: 196 GRIB2 messages
- **Status**: Cannot test until golden reference is generated

### ⚠️ hrrr_conus_drt3_lambert
- **DRT**: 3 (complex packing with spatial differencing)
- **Storage**: Remote (135 MiB)
- **Golden**: ❌ No golden generated yet
- **Messages**: 170 GRIB2 messages
- **Status**: Cannot test until golden reference is generated

## Overall Test Results

```
=== Differential Harness Coverage ===
Fixtures : 12 total
  Comparable fixtures: 8
  Matched: 8/8 (100.0%)
  No-golden: 2
  Skipped-feature: 2 (DRT=40 fixtures without jpeg2000 feature)
  
Per-template breakdown:
  GDT=0 PDT=0 DRT=3: 1/1     ← gfs_tmp2m_1deg_anl
  GDT=30 PDT=0 DRT=3: 187/187 ← nam_awip12_lambert_drt3 (PDT=0 messages)
  GDT=30 PDT=8 DRT=3: 9/9     ← nam_awip12_lambert_drt3 (PDT=8 messages)
=====================================
```

## Conclusion

✅ **All testable DRT=3 fixtures pass with 100% agreement**

- **Total DRT=3 messages tested**: 197 (1 + 187 + 9)
- **Messages matched**: 197/197 (100%)
- **Fixtures tested**: 2 (both passed)
- **Fixtures awaiting golden references**: 2

The DRT=3 implementation is working correctly for both:
1. Simple lat/lon grids (GDT=0)
2. Complex Lambert Conformal Conic grids (GDT=30)
3. Both analysis (PDT=0) and time-processed (PDT=8) products
4. Single and multi-message fixtures

## Recommendations

1. Generate golden references for the 2 remaining DRT=3 fixtures (`nam_awip12_lambert_drt3_20250120` and `hrrr_conus_drt3_lambert`)
2. Once golden references are available, run validation on those fixtures to complete DRT=3 coverage
3. Consider adding additional DRT=3 fixtures with different grid templates (e.g., GDT=20 polar stereographic) for comprehensive coverage
