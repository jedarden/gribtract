# GDT 3.30 Lambert Conformal Metadata Verification - bf-ufdir

## Summary

Verified and fixed the GDT 3.30 (Lambert Conformal) grid parsing logic to correctly populate all projection metadata fields for the `nam.t00z.awip1200.tm00.grib2` fixture.

## Bug Found and Fixed

### Issue: Dx/Dy Grid Spacing Values Incorrect by Factor of 1000

**Root Cause:** The parsing code in `parse_gdt_20` and `parse_gdt_30` was not handling GRIB2 Table 3.3 resolution flags correctly. According to the GRIB2 specification:

- **Table 3.3, bit 5 (value 0x20):**
  - 0 = Dx and Dy are in **meters**
  - 1 = Dx and Dy are in **millimeters**

The NAM fixture has `resolution_flags = 56 (0x38)`, which has bit 5 set, indicating Dx/Dy are stored in **millimeters**. However, the parsing code was treating them as meters without conversion.

**Impact:** The parsed grid spacing values were 1000x too large:
- Parsed: 12,191,000 meters (12,191 km)
- Expected: 12,191 meters (12.191 km)

**Fix Applied:** Added millimeter-to-meter conversion in both `parse_gdt_20` and `parse_gdt_30`:

```rust
// Dx and Dy units depend on bit 5 of resolution_flags (Table 3.3):
// 0 = metres, 1 = millimetres
let dx_raw = b.read_u32be()? as f64;   // oct 56–59: Dx (units per resolution flags)
let dy_raw = b.read_u32be()? as f64;   // oct 60–63: Dy (units per resolution flags)
let dx_m = if resolution_flags & 0x20 != 0 { dx_raw / 1000.0 } else { dx_raw };
let dy_m = if resolution_flags & 0x20 != 0 { dy_raw / 1000.0 } else { dy_raw };
```

## Verified Metadata Values

All Lambert Conformal projection parameters are now correctly populated and match wgrib2 output:

| Parameter | Parsed Value | wgrib2 Value | Status |
|-----------|--------------|--------------|--------|
| Grid Template | 30 | 30 | ✅ |
| Nx (columns) | 614 | 614 | ✅ |
| Ny (rows) | 428 | 428 | ✅ |
| Total points | 262,792 | 262,792 | ✅ |
| First latitude | 12.190° N | 12.190° N | ✅ |
| First longitude | 226.541° E | 226.541° E | ✅ |
| LaD (latitude of Dx/Dy) | 25.0° | 25.0° | ✅ |
| LoV (central meridian) | 265.0° | 265.0° | ✅ |
| Dx (grid spacing) | 12.191 km | 12.191 km | ✅ (FIXED) |
| Dy (grid spacing) | 12.191 km | 12.191 km | ✅ (FIXED) |
| Latin1 (standard parallel 1) | 25.0° | 25.0° | ✅ |
| Latin2 (standard parallel 2) | 25.0° | 25.0° | ✅ |
| Projection centre flag | 0 | 0 | ✅ |
| South pole latitude | -90.0° | -90.0° | ✅ |
| South pole longitude | 0.0° | 0.0° | ✅ |
| Scanning mode | 0x40 | 0x40 | ✅ |
| Shape of Earth | 6 | 6 | ✅ |

## Testing

### Test Added
Created `verify_gdt30_lambert_metadata.rs` to validate all GDT 3.30 metadata fields against wgrib2 reference values.

### Tests Passing
- ✅ `verify_lambert_gdt30_metadata_population` - All metadata values validated
- ✅ `integration_nam_lambert_end_to_end` - Full fixture decode with correct grid spacing
- ✅ All existing unit tests (40 passed)

## Files Modified

1. **`crates/gribtract-core/src/decode.rs`**
   - Fixed `parse_gdt_20` (Polar Stereographic) to handle millimeter units
   - Fixed `parse_gdt_30` (Lambert Conformal) to handle millimeter units
   - Updated documentation comments to reflect correct units

2. **`crates/gribtract/tests/verify_gdt30_lambert_metadata.rs`** (new file)
   - Comprehensive verification test for all Lambert Conformal metadata fields

## Reference

- **GRIB2 Table 3.3:** Resolution and Component Flags
- **GRIB2 Table 3.4:** Scanning Mode
- **GRIB2 Grid Definition Template 3.30:** Lambert Conformal Conic
- **NCEP Grid 218:** NAM 12km CONUS grid (614×428, 12.191 km spacing)
