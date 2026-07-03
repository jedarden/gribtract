# GRIB2 Grid Definition Template (GDT) Inspection

## Task
Inspect and document Grid Definition Template numbers for GRIB2 files in the gribtract workspace using eccodes (wgrib2 alternative).

## Findings

### HRRR Sample File
**File:** `hrrr_sample_20260703.grib2`

- **Grid Definition Template (GDT):** 30
- **Template designation:** 3.30
- **Template Name:** Lambert Conformal Conic
- **Grid Type:** lambert
- **Centre:** kwbc (NCEP)
- **Grid Dimensions:** 1799 × 1059
- **Parameter:** Maximum/Composite radar reflectivity (refc)

### NAM CONUS File
**File:** `nam_conus_20260703.grib2`

- **Grid Definition Template (GDT):** 30
- **Template designation:** 3.30
- **Template Name:** Lambert Conformal Conic
- **Grid Type:** lambert
- **Centre:** kwbc (NCEP)

### Comparison File (GFS)
**File:** `tests/corpus/small/gfs_tmp2m_1deg_anl.grib2`

- **Grid Definition Template (GDT):** 0
- **Template designation:** 3.0
- **Template Name:** Regular Latitude/Longitude
- **Grid Type:** regular_ll

## Conclusion

The primary HRRR and NAM CONUS files both use **GDT 3.30** (Lambert Conformal Conic), which is the standard projection for CONUS-scale weather prediction models. This is a secant tangent cone projection optimized for mid-latitude regions.

## Tools Used

- **eccodes** Python bindings (pkg-config eccodes) - GRIB2 inspection library
- Equivalent to wgrib2 for this purpose

## Output

Detailed inspection saved to: `docs/bf-tnue-gdt-inspection.txt`
