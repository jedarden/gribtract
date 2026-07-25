# GFS Gaussian-grid Fixture Investigation (bf-49ddt)

## Task
Identify which GFS Gaussian-grid fixture to use for integration and verify accessibility.

## Findings

### Available GFS Gaussian-grid Fixtures

Two GFS Gaussian-grid fixtures exist in `tests/corpus/manifest.json`:

#### 1. `core_gaussian_gdt40`
- **ID**: `core_gaussian_gdt40`
- **Path**: `large/flx.2024011500.grib2`
- **Source**: NOAA CORe Archive (Climate Data Record)
- **Size**: 10.5 MB (10,968,510 bytes)
- **Storage**: `remote`
- **URL**: https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb
- **Grid**: GDT 3.40 (Gaussian Lat/Lon), 512 x 256, 131,072 points
- **Fields**: 4 GRIB2 messages (flux fields)
- **Golden JSON**: ✅ EXISTS at `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)
- **Accessibility**: ✅ VERIFIED - `cargo xtask corpus fetch --fixture core_gaussian_gdt40` confirms SHA256 matches
- **Capture Date**: 2026-07-23

#### 2. `gfs_gaussian_gdt40_t1534`
- **ID**: `gfs_gaussian_gdt40_t1534`
- **Path**: `large/gdas.t00z.sfluxgrbf000.grib2`
- **Source**: NOAA GDAS (Global Data Assimilation System) Surface Flux
- **Size**: 122 MB (127,659,863 bytes)
- **Storage**: `remote`
- **URL**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2
- **Grid**: GDT 3.40 (Gaussian Lat/Lon), T1534 (3072 x 1536), N=768, ~0.117° resolution (~12 km)
- **Total Grid Points**: 4,718,592
- **Fields**: 54 GRIB2 messages (surface flux fields)
- **Golden JSON**: ❌ DOES NOT EXIST - needs generation
- **Accessibility**: ✅ VERIFIED - `cargo xtask corpus fetch --fixture gfs_gaussian_gdt40_t1534` confirms SHA256 matches
- **Support Status**: ✅ FULLY SUPPORTED - gribtract's GDT 3.40 decoder verified 2026-07-24 (bead bf-1qia4)
- **Capture Date**: 2026-07-24

## Recommendation

**Use `gfs_gaussian_gdt40_t1534` for integration**

### Rationale:
1. **Better Coverage**: 54 fields vs 4 fields - more comprehensive test coverage
2. **Higher Resolution**: T1534 (3072x1536, ~12km) vs 512x256 - tests larger grid handling
3. **Already Verified**: End-to-end integration testing completed 2026-07-24 (bead bf-1qia4) confirmed gribtract successfully decodes all 54 fields with correct metadata and 4.7M data points per field
4. **Recent**: More recent capture date (2026-07-24 vs 2026-07-23)
5. **Proven Source**: NOAA GDAS is the production system used in operations

### Next Steps for `gfs_gaussian_gdt40_t1534`:
1. Generate golden JSON: `cargo xtask corpus gen-golden --fixture gfs_gaussian_gdt40_t1534`
2. Verify golden JSON is valid and complete
3. Integrate into differential test suite

### Alternative: `core_gaussian_gdt40`
Use only if:
- Smaller file size is a constraint (10.5 MB vs 122 MB)
- Faster golden generation is needed (4 fields vs 54 fields)
- CORe archive flux fields are specifically needed

## Accessibility Verification

Both fixtures are accessible via the corpus fetch mechanism and verified with SHA256:

```bash
# core_gaussian_gdt40
cargo xtask corpus fetch --fixture core_gaussian_gdt40
# Result: [ok] core_gaussian_gdt40 (already present, sha256 matches)
# SHA256 verified: 003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397

# gfs_gaussian_gdt40_t1534
cargo xtask corpus fetch --fixture gfs_gaussian_gdt40_t1534
# Result: [ok] gfs_gaussian_gdt40_t1534 (already present, sha256 matches)
# SHA256 verified: f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e
```

## Golden JSON Status

- ✅ `core_gaussian_gdt40.json`: EXISTS (378 MB, 4 fields)
- ❌ `gfs_gaussian_gdt40_t1534.json`: DOES NOT EXIST - needs generation

## Grid Template Verification

Both fixtures use GDT 3.40 (Gaussian Latitude/Longitude grid) - verified with wgrib2:

```bash
# core_gaussian_gdt40
wgrib2 -grid tests/corpus/large/flx.2024011500.grib2
# Result: grid_template=40:winds(N/S): Gaussian grid: (512 x 256) 
#         number of latitudes between pole-equator=128 #points=131072

# gfs_gaussian_gdt40_t1534  
wgrib2 -grid tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2
# Result: grid_template=40:winds(N/S): Gaussian grid: (3072 x 1536)
#         number of latitudes between pole-equator=768 #points=4718592
```

Grid details:
- `core_gaussian_gdt40`: 512 x 256, N=128 (number of parallels between pole and equator)
- `gfs_gaussian_gdt40_t1534`: 3072 x 1536, N=768 (number of parallels between pole and equator)

## Conclusion

**Fixture ID**: `gfs_gaussian_gdt40_t1534`

**GRIB2 File Path**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`

**Golden JSON Path**: `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json` (to be generated)

**Remote URL**: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2

**Status**: ✅ Accessible, fully supported, golden JSON pending generation
