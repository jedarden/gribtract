# GDT 3.40 Gaussian Grid Verification

## Task
Verify GRIB2 files to confirm they use Grid Definition Template 3.40 (Gaussian latitude/longitude grid).

## Tool Used
`grib_ls` from ECCodes (available at `/home/coding/.nix-profile/bin/grib_ls`)

### Command for verification:
```bash
grib_ls -p gridType,gridDefinitionTemplateNumber,Nx,Ny <file.grib2>
```

## Files Confirmed with GDT 3.40

### 1. gfs.t00z.sfluxgrbf000.grib2
- **Location:** `/home/coding/gribtract/gfs.t00z.sfluxgrbf000.grib2`
- **Size:** 121M
- **Grid Type:** regular_gg (Gaussian grid)
- **GDT:** 40 (Grid Definition Template 3.40)
- **Resolution:** 3072 x 1536 (0.117188° longitude spacing)
- **Extent:** 89.9103°N to -89.9103°S, 0° to 359.883°E

### 2. flx.2025011500.grib2
- **Location:** `/home/coding/gribtract/tests/corpus/large/flx.2025011500.grib2`
- **Size:** 11M
- **Grid Type:** regular_gg (Gaussian grid)
- **GDT:** 40 (Grid Definition Template 3.40)
- **Resolution:** 512 x 256

### 3. flx.2024011500.grib2
- **Location:** `/home/coding/gribtract/tests/corpus/large/flx.2024011500.grib2`
- **Size:** 11M
- **Grid Type:** regular_gg (Gaussian grid)
- **GDT:** 40 (Grid Definition Template 3.40)
- **Resolution:** 512 x 256

### 4. gfs_gaussian_gdt40_drt0.grib2
- **Location:** `/home/coding/gribtract/tests/corpus/small/gfs_gaussian_gdt40_drib2`
- **Size:** 307 bytes (test file)
- **Grid Type:** regular_gg (Gaussian grid)
- **GDT:** 40 (Grid Definition Template 3.40)
- **Resolution:** 16 x 8 (minimal test grid)

## Files with Incorrect/Other GDT

### Regular Latitude-Longitude Grid (GDT 0):
- `/home/coding/gribtract/temp_drt_check/gfs_sample.grib2` - 720x361, GDT 0

### Lambert Conformal Conic (GDT 30):
- Multiple HRRR and NBM files use GDT 30 (Lambert projection)

## Summary
✅ **Multiple files confirmed with GDT 3.40 (Gaussian grid)**
✅ **Verification method documented using grib_ls tool**
✅ **Files with other GDTs noted for reference**

All four confirmed files use Grid Definition Template 3.40 as expected for global Gaussian grid datasets commonly used in numerical weather prediction models like GFS.
