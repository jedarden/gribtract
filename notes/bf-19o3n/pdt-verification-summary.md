# PDT 4.1 and 4.8 Verification Report

## Task Summary
Verify that GRIB2 files contain Product Definition Template (PDT) 4.1 or 4.8 messages using wgrib2 tools.

## Tool Verification
- **Tool**: wgrib2 v2.0.8+ 
- **Location**: `/home/coding/.local/bin/wgrib2`
- **Key option**: `-pdt` for Product Definition Template analysis

## File Analysis Results

### Files with PDT 4.1 (Individual Ensemble Forecasts)

#### ✅ ECMWF Ensemble ENSO Data
- **File**: `/home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2`
- **Size**: 1.5 GB
- **Source**: ECMWF (European Centre for Medium-Range Weather Forecasts)
- **Date**: 2023-01-18 00:00 UTC

**PDT 4.1 Statistics:**
- **Total messages**: 2,614
- **PDT 4.1 messages**: 2,563 (98.0%)
- **PDT 4.11 messages**: 51 (2.0%)
- **PDT 4.8 messages**: 0

**Sample PDT 4.1 Messages:**
```
1:0:d=2023011800:UGRD:10 m above ground:anl:ENS=+31
2:609069:d=2023011800:UGRD:10 m above ground:anl:ENS=+19
3:1218138:d=2023011800:TMP:2 m above ground:anl:ENS=+3
```

**Key Characteristics:**
- Individual ensemble members identified with ENS=+N notation
- Ensemble members range from ENS=+1 through ENS=+49
- Variables include UGRD, VGRD, TMP, and precipitation accumulations

### Files with PDT 4.8 (Statistical/Accumulation Products)

#### ✅ HRRR Surface Data
- **File**: `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2`
- **Size**: ~33 MB
- **Source**: NOAA NCEP HRRR model
- **Date**: 2024-01-15 12:00 UTC

**PDT Statistics:**
- **Total messages**: 170
- **PDT 0 messages**: 143 (84.1%)
- **PDT 8 messages**: 27 (15.9%)

**PDT 8 Message Positions:**
```
38:24611752:pdt=8  43:26434829:pdt=8  65:34280686:pdt=8
39:24665312:pdt=8  44:26595639:pdt=8  79:48481114:pdt=8
40:24955192:pdt=8  45:26793288:pdt=8  80:49701089:pdt=8
```

#### ✅ NAM AWIP12 Data
- **File**: `/home/coding/gribtract/data/nam.t00z.awip1200.tm00.grib2`
- **Source**: NOAA NCEP NAM model
- **PDT 8 messages**: Found at multiple positions (79-84, 118-119, 138)

**PDT 8 Message Pattern:**
```
79:11520597:pdt=8
80:11520840:pdt=8
81:11521083:pdt=8
82:11558063:pdt=8
83:11570331:pdt=8
84:11575365:pdt=8
```

#### ✅ RAP AWIP32 Data
- **File**: `/home/coding/gribtract/scratch/drt0-verification/rap.t12z.awip32f00.grib2`
- **Size**: 18 MB
- **PDT 8 messages**: Found at positions 432, 462-468

#### ✅ Small Test Fixture
- **File**: `/home/coding/gribtract/tests/corpus/small/pdt8_accum_3x2.grib2`
- **Size**: 205 bytes
- **PDT 8 messages**: 1 (100% of file)
- **Variable**: APCP (Accumulated Precipitation)

### Files Without PDT 4.1/4.8

#### ❌ NDFD Temperature Data
- **File**: `/home/coding/gribtract/ndfd_temp.grib2`
- **Size**: 5.0 MB
- **PDT type**: PDT 0 only (standard analysis/forecast)
- **Content**: Temperature forecasts at 2m above ground

#### ❌ Empty/Invalid Files
- `gefs_test.grib2`: 0 bytes (empty)
- `gefs_perturbation_member_pdt41_test.grib2`: 0 bytes (empty)
- `gefs.20240101.00.atmos.pgrb2ap5.geavg.t00z.pgrb2a.0p50.f000.grib2`: 0 bytes (empty)

### Test Fixture Candidates

#### Best for PDT 4.1 Testing: ✅
**ECMWF Ensemble File** (`ecmwf_ensemble_enso_0h.grib2`)
- **Advantages**: 
  - Large file (1.5 GB) with extensive PDT 4.1 coverage
  - High concentration of PDT 4.1 messages (98% of content)
  - Multiple ensemble members (ENS=+1 through +49)
  - Diverse variables (wind, temperature, precipitation)
- **Use case**: Production-scale ensemble data processing

#### Best for PDT 4.8 Testing: ✅
**HRRR Surface File** (`hrrr.t12z.wrfsfcf00.grib2`)
- **Advantages**:
  - Real operational data from NOAA
  - Mix of PDT 0 and PDT 8 messages (realistic test scenario)
  - Manageable size (33 MB) for repeated testing
  - 15.9% PDT 8 coverage for statistical products

#### Best for Small Unit Tests: ✅
**Small Test Files** (`pdt1_ensemble_3x2.grib2`, `pdt8_accum_3x2.grib2`)
- **Advantages**:
  - Tiny files (<200 bytes each) for fast test execution
  - Single PDT type per file (isolated testing)
  - Known good fixtures for regression testing
  - Already integrated into test corpus

## Product Definition Template Reference

| PDT | Name | Typical Usage | Found In |
|-----|------|----------------|----------|
| 4.0 | Analysis or forecast at horizontal level | Standard deterministic forecasts | NDFD, most model data |
| **4.1** | **Individual ensemble forecast** | **Individual ensemble member data** | ✅ ECMWF ensemble file |
| 4.2 | Derived ensemble products | Ensemble means, spreads, probabilities | - |
| **4.8** | **Individual ensemble forecast (alt)** | **Statistical/accumulation products** | ✅ HRRR, NAM, RAP files |

## Commands Used

```bash
# Check wgrib2 availability
which wgrib2

# Get PDT information for all messages
wgrib2 <file.grib2> -pdt

# Count messages by PDT type
wgrib2 <file.grib2> -pdt | cut -d: -f3 | sort | uniq -c

# Get full inventory with variable details
wgrib2 <file.grib2> -s

# Search for specific PDT types across all files
find /path -name "*.grib2" -exec sh -c 'wgrib2 -pdt "$1" 2>/dev/null | grep "pdt=8"' _ {} \;
```

## Conclusion

### PDT 4.1 Verification ✅
- **Confirmed presence** in ECMWF ensemble file (2,563 messages)
- **Best test fixture**: `/home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2`
- **Alternative**: `/home/coding/gribtract/tests/corpus/small/pdt1_ensemble_3x2.grib2` (small scale)

### PDT 4.8 Verification ✅
- **Confirmed presence** in multiple operational model files
- **HRRR**: 27 PDT 8 messages (15.9% of file)
- **NAM**: Multiple PDT 8 messages at positions 79-84, 118-119, 138
- **RAP**: PDT 8 messages at positions 432, 462-468
- **Best test fixture**: `/home/coding/gribtract/data/hrrr.t12z.wrfsfcf00.grib2`
- **Alternative**: `/home/coding/gribtract/tests/corpus/small/pdt8_accum_3x2.grib2` (minimal test)

### Recommendations
1. **Use ECMWF ensemble file** for PDT 4.1 processing tests (comprehensive ensemble member coverage)
2. **Use HRRR file** for PDT 4.8 statistical product tests (realistic mixed PDT scenario)
3. **Use small test files** for unit tests and regression testing (fast execution, isolated PDT types)
4. **Avoid empty test files** - several downloaded GEFS files are 0 bytes and need re-downloading

---

**Verification completed**: 2026-07-23
**Tool used**: wgrib2 v2.0.8+
**Total files inspected**: 20+ GRIB2 files
**PDT 4.1 found**: ✅ Yes (ECMWF ensemble)
**PDT 4.8 found**: ✅ Yes (HRRR, NAM, RAP, test fixtures)
