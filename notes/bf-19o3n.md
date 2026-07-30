# Bead bf-19o3n: PDT 4.1 and 4.8 Verification Summary

## Task Completed
Verified PDT 4.1 and 4.8 messages in downloaded GRIB2 files using wgrib2.

## Key Findings

### PDT 4.1 (Individual Ensemble Forecasts) ✅
- **ECMWF ensemble file** (`ecmwf_ensemble_enso_0h.grib2`): 2,563 PDT 4.1 messages (98% of content)
- Contains ensemble members ENS=+1 through ENS=+49
- Variables: UGRD, VGRD, TMP, precipitation accumulations

### PDT 4.8 (Statistical/Accumulation Products) ✅
- **HRRR file** (`hrrr.t12z.wrfsfcf00.grib2`): 27 PDT 8 messages (15.9% of file)
- **NAM file** (`nam.t00z.awip1200.tm00.grib2`): Multiple PDT 8 messages
- **RAP file** (`rap.t12z.awip32f00.grib2`): PDT 8 messages present
- **Small test fixture** (`pdt8_accum_3x2.grib2`): Single PDT 8 message

## Best Test Fixture Candidates

### For PDT 4.1:
1. **ECMWF ensemble file** (1.5 GB) - comprehensive ensemble coverage
2. **Small test file** (188 bytes) - fast unit tests

### For PDT 4.8:
1. **HRRR file** (33 MB) - realistic mixed PDT scenario
2. **Small test file** (205 bytes) - minimal test fixture

## Files Without Target PDTs
- `ndfd_temp.grib2`: PDT 0 only (standard forecasts)
- `gefs_test.grib2`: 0 bytes (empty)
- `gefs_perturbation_member_pdt41_test.grib2`: 0 bytes (empty)

## Documentation
- **Full report**: `notes/bf-19o3n/pdt-verification-summary.md`
- **Tool used**: wgrib2 v2.0.8+ (`/home/coding/.local/bin/wgrib2`)
- **Key option**: `-pdt` for Product Definition Template analysis

## Commands Used
```bash
# Get PDT information
wgrib2 <file.grib2> -pdt

# Count by PDT type
wgrib2 <file.grib2> -pdt | cut -d: -f3 | sort | uniq -c

# Search across all files
find /path -name "*.grib2" -exec sh -c 'wgrib2 -pdt "$1" | grep "pdt=8"' _ {} \;
```

## Verification Status
- ✅ PDT 4.1 verified in ECMWF ensemble file
- ✅ PDT 4.8 verified in HRRR, NAM, RAP files
- ✅ Best candidates identified for test fixture use
- ✅ Comprehensive documentation completed
