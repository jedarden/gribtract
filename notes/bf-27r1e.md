# bf-27r1e: CONUS DRT=0 Fixture Verification

## Verification Results

### Corpus Fetch
```bash
cargo xtask corpus fetch
```

All fixtures verified:
- `[ok] hrrr_conus_drt0_lambert_20260723 (already present, sha256 matches)`

### Station Benchmark in_range Results

From `bench-results.json`:
- `interp: nearest, in_range: 40, stations: 20, fields: 8`
- `interp: bilinear, in_range: 40, stations: 20, fields: 8`
- `interp: lazy-nearest, in_range: 40, stations: 20, fields: 8`
- `interp: drt3-cached-nearest, in_range: 20, stations: 20, fields: 8`

All interpolation modes show `in_range > 0`, confirming CONUS DRT=0 fixture provides proper coverage for the station benchmark test suite.

## Acceptance Criteria

- ✅ cargo xtask corpus fetch succeeds for the new fixture
- ✅ SHA256 hash verification passes
- ✅ File is properly cached after fetch
- ✅ Station benchmark shows in_range > 0 for CONUS coverage
