# Differential Test Baseline (bf-2b5cy)

## Task
Run differential test suite and capture baseline output after adding GFS Gaussian-grid fixture.

## Results

### Overall Agreement
**91.7%** (11/12 comparable fixtures matched)

### Fixture Breakdown
- **21 total fixtures**
- **12 comparable** (can be tested against golden files)
  - **11 matched** successfully
  - **1 decode error**
- **7 no-golden** (missing golden files, cannot compare yet)
- **2 skipped-feature** (disabled by feature flags)

### Per-Template Results
```
GDT=0 PDT=0 DRT=0: 2/2
GDT=0 PDT=0 DRT=2: 1/1
GDT=0 PDT=0 DRT=3: 1/1
GDT=0 PDT=0 DRT=41: 2/2
GDT=0 PDT=1 DRT=0: 1/1
GDT=0 PDT=1 DRT=3: 71/71
GDT=0 PDT=2 DRT=3: 71/71
GDT=0 PDT=8 DRT=0: 1/1
GDT=30 PDT=0 DRT=3: 187/187
GDT=30 PDT=8 DRT=3: 9/9
```

### Test Execution
- Test duration: ~43 seconds
- Test output saved to: `/tmp/differential_baseline.txt`

## Notes
This establishes the baseline after adding the GFS Gaussian-grid fixture to the manifest (from bf-dag1f). The 91.7% agreement rate indicates strong consistency between the live GRIB decoding and the golden JSON fixtures. The 1 decode error and 7 no-golden fixtures represent areas for improvement in future work.

## Push Status
Commit created locally but push failed due to HTTP 413 (Payload Too Large) error. This is due to 82 accumulated commits including large golden JSON files (1.1G, 361M, 434M) that exceed the server's push size limit. The commit is available locally and will be pushed when the large file issue is resolved.
