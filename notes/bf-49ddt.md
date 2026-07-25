# GFS Gaussian-grid Fixture Investigation (bf-49ddt)

## Investigation Summary

Identified and verified two GFS Gaussian-grid fixtures in the corpus manifest:

### 1. core_gaussian_gdt40 ✅ RECOMMENDED

**Fixture ID**: `core_gaussian_gdt40`

**File**: `tests/corpus/large/flx.2024011500.grib2`

**Source**: NOAA CORe (Climate Data Record) Archive on Google Cloud Storage
- URL: `https://storage.googleapis.com/noaa-nws-ncep-core/grib/3hour/flx/2024/01/flx.2024011500.grb`

**Grid Specifications**:
- Grid Template: GDT 3.40 (Gaussian Latitude/Longitude)
- Dimensions: 512 x 256 grid points
- Total points: 131,072
- Latitudes between pole-equator: 128
- Resolution: ~0.703° longitude increment

**File Details**:
- Size: 10.5 MiB (10,968,510 bytes)
- Storage: remote (committed to gitignore, fetched by cargo xtask)
- SHA256: `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` ✅ VERIFIED

**Golden JSON**: ✅ EXISTS
- File: `tests/corpus/golden/core_gaussian_gdt40.json`
- Size: 361 MB
- Generated: 2024-07-24 22:33

**Verification**:
- ✅ SHA256 hash matches manifest
- ✅ File present locally in tests/corpus/large/
- ✅ Grid template confirmed via wgrib2: `grid_template=40`
- ✅ Golden JSON available for differential testing

### 2. gfs_gaussian_gdt40_t1534 ⚠️ NEEDS GOLDEN

**Fixture ID**: `gfs_gaussian_gdt40_t1534`

**File**: `tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2`

**Source**: NOAA GDAS (Global Data Assimilation System) via NOMADS
- URL: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.20260724/00/atmos/gdas.t00z.sfluxgrbf000.grib2`

**Grid Specifications**:
- Grid Template: GDT 3.40 (Gaussian Latitude/Longitude)
- Dimensions: 3072 x 1536 grid points (T1534)
- Total points: 4,718,592
- Latitudes between pole-equator: 768 (N=768)
- Resolution: ~0.117° (~12 km)

**File Details**:
- Size: 122 MiB (127,659,863 bytes)
- Storage: remote (committed to gitignore, fetched by cargo xtask)
- SHA256: `f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e` ✅ VERIFIED

**Golden JSON**: ❌ DOES NOT EXIST (needs generation)
- Expected: `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`
- Status: Must be generated before use in differential tests

**Verification**:
- ✅ SHA256 hash matches manifest
- ✅ File present locally in tests/corpus/large/
- ✅ Grid template confirmed via wgrib2: `grid_template=40`
- ❌ Golden JSON missing - cannot use for differential testing until generated

## Accessibility Verification

Both fixtures are accessible via the standard cargo xtask fetch mechanism:

```bash
cargo xtask corpus list  # Shows both as "remote" with "present: yes"
```

The files are already downloaded to `tests/corpus/large/` and verified via SHA256.

## Recommendation

**Use `core_gaussian_gdt40` for GFS Gaussian-grid integration** because:

1. ✅ **Golden JSON exists** - Ready for immediate differential testing
2. ✅ **Smaller footprint** - 10.5 MiB vs 122 MiB, faster for CI/CD
3. ✅ **Good coverage** - 512x256 Gaussian grid with 131K points provides representative GDT 3.40 testing
4. ✅ **Stable source** - CORe archive provides long-term Climate Data Record coverage (1950-present)
5. ✅ **Verified** - All integrity checks pass, wgrib2 confirms GDT 3.40

The `gfs_gaussian_gdt40_t1534` fixture (T1534, 4.7M points) provides higher-resolution coverage but requires golden JSON generation before it can be used in differential testing.

## Grid Verification (wgrib2)

### core_gaussian_gdt40:
```
grid_template=40:winds(N/S):
Gaussian grid: (512 x 256) units 1e-06 input WE:NS output WE:SN
number of latitudes between pole-equator=128 #points=131072
```

### gfs_gaussian_gdt40_t1534:
```
grid_template=40:winds(N/S):
Gaussian grid: (3072 x 1536) units 1e-06 input WE:NS output WE:SN
number of latitudes between pole-equator=768 #points=4718592
```

Both confirmed as GDT 3.40 (Gaussian Latitude/Longitude grid) as expected.

## Conclusion

**Fixture to integrate**: `core_gaussian_gdt40`
- GRIB2 file: ✅ Present and verified
- Golden JSON: ✅ Available (361 MB)
- Grid type: ✅ GDT 3.40 Gaussian (512x256)
- Accessibility: ✅ Fetchable via cargo xtask

The fixture is ready for integration into the differential test suite.
