# Current Script Output Structure Analysis (bf-1dd2)

## Task Completed

Documented the current `gen_golden.py` script output format and compared it against the documented golden JSON schema from `docs/golden-json-schema.md`.

## Summary

**Overall Structure: ✅ CORRECT**

The current `gen_golden.py` script outputs JSON that **structurally matches** the documented golden JSON schema. All field names, types, and nesting levels are correct.

## Detailed Comparison

### ✅ Matches Documented Schema

**Root Level:**
- ✅ `fixture_id` (string)
- ✅ `_provenance` (string)
- ✅ `fields` (array)

**Field Object:**
- ✅ `center` (integer)
- ✅ `subcenter` (integer)
- ✅ `parameter` {discipline, category, number} (all integers)
- ✅ `forecast` {reference_time, time_range_unit, forecast_offset}
- ✅ `level` {type1, scale_factor1, scaled_value1, type2, scale_factor2, scaled_value2}
- ✅ `ensemble` (object or null)
- ✅ `grid` {template, num_data_points, nx, ny, lat_first, lon_first, lat_last, lon_last, di, dj, scanning_mode, resolution_flags, shape_of_earth}
- ✅ `values` {Dense: [...] or Masked: {...}}
- ✅ `gdt_template` (integer)
- ✅ `pdt_template` (integer)
- ✅ `drt_template` (integer)
- ✅ `packing` {reference_value, binary_scale_factor, decimal_scale_factor, bits_per_value, original_field_type}

**Nesting:**
- ✅ All objects properly nested at correct levels
- ✅ Array structures correct

### ❌ Mismatches Found

**1. Extra Root-Level Field (Non-Structural)**
- **Issue**: Script adds `parser_version` field at root level
- **Expected**: Root should only have `fixture_id`, `_provenance`, `fields`
- **Impact**: This field is not documented in the golden JSON schema
- **Example**: `"parser_version": "basic_0.1"`
- **Severity**: Low - doesn't break schema compatibility, but violates documented spec

**2. Byte Offset Parsing Bugs (Value Issues)**
- **Issue**: Script extracts incorrect values due to wrong byte offsets
- **Impact**: Values are garbage, but structure and types are correct
- **Examples from actual test run**:
  - `reference_time.year`: 1557 (should be 2026)
  - `reference_time.month`: 0 (should be 6)
  - `grid.nx`: 0 (should be 3)
  - `grid.ny`: 2304 (should be 3)
  - `grid.num_data_points`: 0 (should be 9)
- **Severity**: High - makes the output unusable despite correct structure

## Field Type Accuracy

All fields use the **correct data types** as specified in the schema:
- Integers: `center`, `subcenter`, `discipline`, `category`, `number`, template numbers, etc.
- Floats: `lat_first`, `lon_first`, `lat_last`, `lon_last`, `di`, `dj`, `reference_value`, etc.
- Arrays: `fields`, `Dense` values array
- Objects: All nested objects (parameter, forecast, level, grid, packing)
- Null: `ensemble` when not applicable

## Conclusion

The `gen_golden.py` script's **JSON structure is correct** and matches the documented golden JSON schema exactly. The issues are:

1. **One extra undocumented field** (`parser_version`) at root level
2. **Incorrect byte offset extraction** leading to garbage values (but correct field types)

The script outputs the right structure with the right field names and types - it just needs:
1. Removal of the `parser_version` field
2. Fixes to byte offset extraction logic
3. Overall structural design is sound

## Testing Methodology

Tested with: `python3 scripts/gen_golden.py tests/corpus/small/drt2_simple_3x3.grib2 test_output --output-dir /tmp`

Compared output against:
- Documented schema: `docs/golden-json-schema.md`
- Example golden files: `tests/corpus/golden/pdt1_ensemble_3x2.json`, `tests/corpus/golden/drt2_simple_3x3.json`
