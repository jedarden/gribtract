# GFS Gaussian-grid Fixture Investigation (bf-49ddt)

## Investigation Summary

Identified and verified two GFS Gaussian-grid fixtures in the corpus manifest:

### 1. core_gaussian_gdt40 ❌ NOT ACCESSIBLE - DECODING NOT IMPLEMENTED

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
- ❌ **gribtract DECODING NOT IMPLEMENTED** - fails with "decode not implemented" error

### 2. gfs_gaussian_gdt40_t1534 ✅ RECOMMENDED - FULLY FUNCTIONAL

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

**Golden JSON**: ⚠️ DOES NOT EXIST (needs generation for differential testing)
- Expected: `tests/corpus/golden/gfs_gaussian_gdt40_t1534.json`
- Status: Must be generated before use in differential tests
- Tools available: grib_dump, wgrib2, scripts/gen_golden.py

**Verification**:
- ✅ SHA256 hash matches manifest
- ✅ File present locally in tests/corpus/large/
- ✅ Grid template confirmed via wgrib2: `grid_template=40`
- ✅ **gribtract DECODING WORKS** - successfully decodes all 54 fields with 4.7M points each
- ✅ URL accessible (HTTP 200 response)
- ⚠️ Golden JSON missing - must be generated for differential testing

## Accessibility Verification (2026-07-25)

Both fixtures are accessible via the standard cargo xtask fetch mechanism:

```bash
cargo xtask corpus list  # Shows both as "remote" with "present: yes"
```

**Fetch test verified:**
```bash
$ cargo xtask corpus fetch --fixture core_gaussian_gdt40
[ok]      core_gaussian_gdt40 (already present, sha256 matches)

corpus fetch: 0 downloaded, 1 already present, 0 failed
```

The files are already downloaded to `tests/corpus/large/` and verified via SHA256:
- `flx.2024011500.grib2`: 11 MB, SHA256 ✅ `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397`
- `gdas.t00z.sfluxgrbf000.grib2`: 122 MB, SHA256 ✅ `f0d63afe6f4ca96ecbd437f962596ec1017b2088569faaba139625b49c471d9e`

## Recommendation

**USE `gfs_gaussian_gdt40_t1534` for GFS Gaussian-grid integration** because:

1. ✅ **DECODING WORKS** - gribtract successfully decodes all 54 fields (bead bf-1qia4 verified)
2. ✅ **FILE ACCESSIBLE** - Both local copy and remote URL available
3. ✅ **GDT 3.40 VERIFIED** - Confirmed by both gribtract and wgrib2
4. ✅ **REAL-WORLD DATA** - Actual GDAS analysis file (not synthetic)
5. ✅ **COMPREHENSIVE COVERAGE** - T1534 grid (4.7M points) provides high-resolution Gaussian grid testing

**DO NOT USE `core_gaussian_gdt40`** - gribtract decoding is not implemented for this CORe archive format (fails with "decode not implemented" error).

**Next Steps for Integration:**

1. **Generate Golden Reference** (required for differential testing):
   ```bash
   cargo run --bin gen_golden -- tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2 > tests/corpus/golden/gfs_gaussian_gdt40_t1534.json
   ```

2. **Verify Golden Schema** - Ensure JSON matches golden schema structure

3. **Automatic Test Integration** - Once golden exists, fixture will be automatically included in differential tests (already in manifest.json)

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

**Fixture to integrate**: `gfs_gaussian_gdt40_t1534`
- GRIB2 file: ✅ Present and verified (122 MB)
- Golden JSON: ⚠️ DOES NOT EXIST - needs generation via scripts/gen_golden.py
- Grid type: ✅ GDT 3.40 Gaussian T1534 (3072x1536, 4.7M points)
- Accessibility: ✅ Fetchable via cargo xtask
- gribtract support: ✅ FULLY FUNCTIONAL - all 54 fields decode correctly (verified bead bf-1qia4)

**DO NOT USE `core_gaussian_gdt40`** - gribtract decoding not implemented (fails with "decode not implemented" error).

## Updated Assessment (2026-07-25)

Based on comprehensive verification of both fixtures:
1. **core_gaussian_gdt40**: Has golden JSON but gribtract cannot decode it
2. **gfs_gaussian_gdt40_t1534**: No golden JSON yet, but gribtract fully decodes all fields

The correct choice is **`gfs_gaussian_gdt40_t1534`** for integration testing.
