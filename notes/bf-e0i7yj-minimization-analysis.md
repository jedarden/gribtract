# Minimization Analysis for Buffer Underrun Test

## Current Test Structure

### File: `rotated_latlon_gdt1_drt0.grib2` (187 bytes)

| Section | Start | Length | Data Len | Content | Status |
|---------|-------|--------|----------|----------|--------|
| 0 | 0 | 16 | - | Indicator "GRIB", edition=2 | **Essential** |
| 1 | 16 | 21 | 16 | Identification metadata | **Essential** |
| 3 | 37 | 72 | 67 | Grid Definition Section | **UNDERRUN TARGET** |
| 4 | 109 | 34 | 29 | Product Definition Template | Reducible |
| 5 | 143 | 20 | 15 | Data Representation (DRT=0) | Reducible |
| 6 | 163 | 6 | 1 | Bitmap | Reducible |
| 7 | 169 | 14 | 9 | Data values | Reducible |

## Buffer Underrun Mechanism

**Location**: Section 3 (Grid Definition Section)
- **Claimed length**: 72 bytes total (67 bytes of body data)
- **Template requirements**: 
  - GDT 0.0: 73 octets minimum
  - GDT 1.0: 84 octets minimum
- **Available**: 67 octets
- **Shortfall**: 6-17 octets
- **Error**: `TooShort { needed: 1, got: 0 }` at octet 72 of GDT 0.0 template

The parser attempts to read GDT template fields and exhausts the 67 available bytes before completing the template parse.

## Key Insight from Existing Minimal Files

### Why `minimal_underrun.grib2` (50 bytes) produces `NotImplemented`:

This file skips Section 3 entirely and jumps straight to Section 7. It hits a different code path because:
- No Section 3 to parse
- Parser expects Grid Definition but finds Data Section
- Returns `NotImplemented` instead of attempting template parse

**Conclusion**: Section 3 must be present to trigger the buffer underrun!

## Minimization Opportunities

### 1. Essential Components (Cannot Remove)
- **Section 0**: Must have "GRIB" + edition 2 + total length
- **Section 1**: Required identification section
- **Section 3**: Must exist and claim length > available data to trigger underrun
- **Section 7**: Required data section to complete message structure

### 2. Reducible Components (Can Minimize)
- **Section 4 (PDT)**: Can reduce template complexity
- **Section 5 (DRT)**: Can use simplest packing (DRT=0, 8-bit values)
- **Section 6 (Bitmap)**: Can use minimal bitmap
- **Section 7 (Data)**: Can reduce grid size and data values
- **Section 3 body**: Can reduce available data while keeping claimed length

### 3. Grid Dimensions
- **Current**: 3×3 grid (9 data points)
- **Minimum viable**: 1×1 grid (1 data point)
  - Reduces data size from 9 bytes to 1 byte
  - Reduces bitmap size

### 4. Data Values
- **Current**: [270.0, 271.0, ..., 278.0]
- **Minimum**: Single value (e.g., 0.0 or 1.0)
  - Simpler encoding
  - Reduces Section 7 size

## Minimization Strategy

### Phase 1: Section 3 Body Reduction
- Keep claimed length at 72 bytes
- Reduce actual data from 67 bytes toward minimum
- Find exact threshold where underrun triggers

### Phase 2: Grid Size Reduction
- Test 2×2 grid (4 points)
- Test 1×1 grid (1 point)
- Verify buffer underrun still occurs

### Phase 3: Section Simplification
- Minimize Section 4 (PDT) to template 0.0
- Keep Section 5 as DRT=0 (simplest)
- Minimize Section 6 bitmap
- Reduce Section 7 to 1 data value

### Phase 4: Complete Message Reduction
- Target: <100 bytes total (from current 187 bytes)
- Maintain Section 3 underrun mechanism
- Preserve exact same error behavior

## Acceptance Criteria Progress

- [x] **Identify minimum message count needed**: 1 message (current test)
- [x] **Identify minimum grid dimensions needed**: Grid size irrelevant to underrun trigger
- [x] **List which GRIB2 sections can be removed**: NONE - all sections S0-S7 required
- [x] **Document which data values are essential**: Only Section 3 structure matters
- [x] **Create a minimization plan**: Complete (see below)

## Complete Minimization Plan

### What Cannot Be Minimized (The Trigger)

**Section 3 (Grid Definition Section) - THE UNDERRUN TRIGGER**
- Current: Claims 72 bytes (67 body), GDT 1.0 needs 84 octets
- Shortfall: 17 octets
- **Must preserve**: Exact claimed/actual length mismatch
- **Cannot reduce body**: Already triggers at current size
- **Cannot remove**: Files without Section 3 produce `NotImplemented`, not `TooShort`

**Section 0 (Indicator)**
- 16 bytes - "GRIB" + edition 2 + total length
- **Fixed format**, already minimal

**Section 1 (Identification)**
- 21 bytes - metadata
- **Already minimal** for valid GRIB2

### What Can Be Minimized

**Section 4 (Product Definition Template)**
- Current: 34 bytes
- **Target**: ~18-20 bytes using PDT 0.0 (simpler template)
- **Reduction**: ~14 bytes

**Section 5 (Data Representation Template)**
- Current: 20 bytes (DRT=0, already minimal)
- **Target**: Keep as-is
- **Reduction**: 0 bytes (already minimal)

**Section 6 (Bitmap)**
- Current: 6 bytes (9 values for 3×3 grid)
- **Target**: 2 bytes (1 value for 1×1 grid)
- **Reduction**: 4 bytes

**Section 7 (Data Values)**
- Current: 14 bytes (9 × 4-byte floats)
- **Target**: 4 bytes (1 × 4-byte float)
- **Reduction**: 10 bytes

### Total Minimization Potential

- **Current**: 187 bytes
- **Theoretical minimum**: ~130-140 bytes
- **Total reduction**: ~47-57 bytes (25-30% smaller)

### Critical Insight

**Grid dimensions are irrelevant to the underrun!**
- The underrun occurs during GDT template parsing in Section 3
- Grid size affects Sections 6-7, which come AFTER the error point
- We can reduce grid to 1×1 without affecting the trigger

### Build Strategy

1. **Copy S0 + S1** (37 bytes) - unchanged
2. **Copy S3** (72 bytes claimed, 67 actual) - unchanged, THE TRIGGER
3. **Build minimal S4** (18-20 bytes):
   - Use PDT 0.0 instead of current template
   - Include only required fields
4. **Copy S5** (20 bytes) - DRT=0, already minimal
5. **Build minimal S6** (2 bytes): 1-bit bitmap for 1 value
6. **Build minimal S7** (4 bytes): 1 float32 value
7. **Update total length** in S0

### Success Criteria

Any minimal file must:
- ✅ Produce `TooShort { needed: N, got: M }` error during Section 3 parsing
- ✅ Include all sections S0-S7 in valid order
- ✅ Have Section 3 with claimed length > actual body
- ✅ Be structurally valid up to the underrun point
- ❌ NOT produce `NotImplemented` (which means missing Section 3)

---
**Analysis**: 2026-07-27
**Task**: bf-e0i7yj - Identify minimizable GRIB2 components
**Status**: ✅ COMPLETE