# Data Representation Template (DRT) Inspection Report

## Task Summary
Inspect and document Data Representation Template (DRT) values from GRIB2 files.

## Tool Availability
**wgrib2**: NOT AVAILABLE - Not installed on this system and not available via nixpkgs package manager.

**Alternative Tool Used**: gribtract CLI (pure-Rust GRIB2 decoder)

## Files Inspected

### 1. drt2_simple_3x3.grib2 - DRT 2 (Simple Packing)
- **File**: `/home/coding/gribtract/tests/corpus/small/drt2_simple_3x3.grib2`
- **DRT Value**: 2
- **Template Type**: Simple Packing (not complex packing)
- **Inspection Tool**: gribtract list command

**Key Metadata**:
```json
"templates": {
  "gdt": 0,
  "pdt": 0,
  "drt": 2
}
"packing": {
  "reference_value": 100,
  "binary_scale_factor": 0,
  "decimal_scale_factor": 0,
  "bits_per_value": 8,
  "original_field_type": 0,
  "quantization_step": 1,
  "tolerance": 0.5
}
```

### 2. gfs_anl_t2m_5x5.grib2 - DRT 0
- **File**: `/home/coding/gribtract/tests/corpus/small/gfs_anl_t2m_5x5.grib2`
- **DRT Value**: 0
- **Template Type**: Grid point data - simple packing

### 3. gfs_tmp2m_1deg_anl.grib2 - DRT 0
- **File**: `/home/coding/gribtract/tests/corpus/small/gfs_tmp2m_1deg_anl.grib2`
- **DRT Value**: 0

### 4. mrms_carib_refl_drt41.grib2 - DRT 41 (PNG Compression)
- **File**: `/home/coding/gribtract/tests/corpus/small/mrms_carib_refl_drt41.grib2`
- **DRT Value**: 41
- **Template Type**: PNG compression for radar data

### 5. pdt1_ensemble_3x2.grib2 - DRT 0 with PDT 1
- **File**: `/home/coding/gribtract/tests/corpus/small/pdt1_ensemble_3x2.grib2`
- **DRT Value**: 0
- **PDT Value**: 1 (ensemble metadata)

### 6. pdt8_accum_3x2.grib2 - DRT 0 with PDT 8
- **File**: `/home/coding/gribtract/tests/corpus/small/pdt8_accum_3x2.grib2`
- **DRT Value**: 0
- **PDT Value**: 8 (accumulation data)

## DRT 3 (Complex Packing) Findings

### Attempted Inspection: nam_awip12_drt3.grib2
- **File**: `/home/coding/gribtract/test_data/nam_awip12_drt3.grib2`
- **Status**: READ ERROR - gribtract encountered buffer error
- **Error**: "buffer too short: need 0, got 262792"

**Additional Files with Potential DRT 3**:
- `/home/coding/gribtract/samples/nam_awip12_20250115_t00z_f00.grib2` (same buffer error)

## Known DRT Values Discovered
- **DRT 0**: Simple packing (grid point data)
- **DRT 2**: Simple packing with spatial differences
- **DRT 3**: Complex packing (expected in NAM AWIP12 files but could not be verified)
- **DRT 40**: JPEG 2000 compression
- **DRT 41**: PNG compression

## Limitations
1. **wgrib2 not available**: Could not use wgrib2 as specified in the task requirements
2. **NAM AWIP12 DRT 3 files**: Could not be inspected due to gribtract buffer errors
3. **DRT 40 files**: Not implemented in gribtract (decode not implemented)

## Conclusion
The inspection successfully identified DRT values for several GRIB2 files using the gribtract CLI tool. DRT 3 (complex packing) is expected in the NAM AWIP12 files based on their naming convention, but could not be directly verified due to tool limitations. The most common DRT values found were 0 (simple packing) and 2 (simple packing with spatial differences), with specialized compression templates (40, 41) for specific data types.

**Date**: 2026-07-22
**Tool**: gribtract CLI (Rust-based GRIB2 decoder)
**Bead ID**: bf-4jpf
