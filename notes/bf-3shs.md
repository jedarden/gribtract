# Bead bf-3shs: Output Structure Verification

## Task
Restructure output to match golden format

## Analysis
The output structure of `scripts/gen_golden.py` **already matches** the golden format correctly. All acceptance criteria are met.

## Structure Verification

### Root Level Structure
✓ `fixture_id` - string identifier
✓ `_provenance` - generation description  
✓ `fields` - array of field objects

### Field Object Structure  
✓ `center` - integer (generating center)
✓ `subcenter` - integer (subcenter)
✓ `parameter` - object with discipline, category, number
✓ `forecast` - object with reference_time, time_range_unit, forecast_offset
✓ `level` - object with type1, scale_factor1, scaled_value1, type2, scale_factor2, scaled_value2
✓ `ensemble` - null or {member_type, number}
✓ `grid` - object with grid metadata
✓ `values` - {Dense: [...]} or {Masked: {...}}
✓ `gdt_template` - integer (Grid Definition Template)
✓ `pdt_template` - integer (Product Definition Template)
✓ `drt_template` - integer (Data Representation Template)
✓ `packing` - object with reference_value, binary_scale_factor, decimal_scale_factor, bits_per_value, original_field_type

## Additional Fields
The output includes some extra fields not in golden files:
- `parser_version` - Version tracking (doesn't break compatibility)
- `extraction_error` - Debug info when parsing fails (helpful for debugging)

## Acceptance Criteria Status
- ✓ Output JSON has correct field names matching golden schema
- ✓ Proper nesting structure matches golden files
- ✓ All required fields present with correct data types

## Note
The parsing logic has bugs that cause incorrect values (e.g., reference_time shows year 1555 instead of 2024, grid shows null values), but these are parsing issues, not structural issues. The output structure itself matches the golden format correctly.

## Date
2026-07-23
