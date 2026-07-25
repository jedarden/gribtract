# Differential Test Results Verification - bf-4rrq1

## Test Execution
Date: 2026-07-25
Command: `cargo test differential_coverage_report -- --nocapture`

## Results Summary
- **Test Status**: ✅ PASSED
- **Test Duration**: ~45 seconds
- **No failures or panics**

## Coverage Details
- **Total fixtures**: 21
- **Comparable fixtures** (have golden reference): 12
- **Matched fixtures**: 11
- **Decode errors**: 1 (non-Gaussian-grid fixture)
- **Overall agreement**: 91.7% (11/12)

## GFS Gaussian-grid Fixture Results (GDT=30)
GFS Gaussian-grid fixtures use Grid Definition Template 30 (GDT=30). All GDT=30 fixtures show **100% agreement**:

| Template Combination | Fields | Agreement |
|---------------------|--------|-----------|
| GDT=30 PDT=0 DRT=3  | 187/187 | 100% ✅ |
| GDT=30 PDT=8 DRT=3  | 9/9     | 100% ✅ |
| **Total GDT=30**    | **196/196** | **100% ✅** |

## Per-Template Breakdown
```
GDT=0 PDT=0 DRT=0: 2/2 (100%)
GDT=0 PDT=0 DRT=2: 1/1 (100%)
GDT=0 PDT=0 DRT=3: 1/1 (100%)
GDT=0 PDT=0 DRT=41: 2/2 (100%)
GDT=0 PDT=1 DRT=0: 1/1 (100%)
GDT=0 PDT=1 DRT=3: 71/71 (100%)
GDT=0 PDT=2 DRT=3: 71/71 (100%)
GDT=0 PDT=8 DRT=0: 1/1 (100%)
GDT=30 PDT=0 DRT=3: 187/187 (100%) ← GFS Gaussian-grid
GDT=30 PDT=8 DRT=3: 9/9 (100%)     ← GFS Gaussian-grid
```

## Acceptance Criteria Verification
- ✅ cargo test completes successfully
- ✅ Test output shows 100% agreement for the GFS Gaussian-grid fixtures (196/196 fields)
- ✅ No test failures or panics

## Notes
The single decode error (causing the 91.7% overall agreement) is in a non-Gaussian-grid fixture and does not affect the GFS Gaussian-grid fixture test results, which are the focus of this verification.
