# Gaussian-grid GRIB2 Source Research (bead bf-1fjrsd)

## Summary
Successfully identified and verified a publicly-accessible NOAA GRIB2 file using Grid Definition Template 3.40 (Gaussian Latitude/Longitude grid).

## File Details

**File URL:**
```
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2
```

**Alternative S3 path:**
```
s3://noaa-gfs-bdp-pds/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2
```

### Metadata
- **Product:** GFS T1534 Semi-Lagrangian grid (surface flux)
- **File name pattern:** `gfs.tCCz.sfluxgrbfFFF.grib2`
  - `CC` = forecast cycle (00, 06, 12, 18)
  - `FFF` = forecast hour (000-384)
- **Model run:** 2026-07-27 00Z
- **Forecast hour:** F000 (analysis)
- **File size:** 121,155,368 bytes (~116 MB)
- **Grid definition:** GDT 3.40 (Gaussian Latitude/Longitude)
- **Grid dimensions:** 3072 x 1536 points (longitude x latitude)
- **Gaussian N parameter:** 768 (number of latitudes between pole and equator)
- **Resolution:** ~0.117° longitude (~13 km at equator)

## Grid Verification

Using wgrib2 v1.9.5 or later:
```bash
wgrib2 gfs.t00z.sfluxgrbf000.grib2 -V -match ""
```

Output confirms:
```
grid_template=40:winds(N/S):
Gaussian grid: (3072 x 1536) units 1e-06 input WE:NS output WE:SN
number of latitudes between pole-equator=768 #points=4718592
lat 89.910324 to -89.910324
lon 0.000000 to 359.882813 by 0.117188
```

Key: `grid_template=40` = GDT 3.40 (Gaussian Latitude/Longitude)

## Access Methods

### 1. AWS S3 (Public, no authentication required)
```bash
aws s3 ls --no-sign-request s3://noaa-gfs-bdp-pds/gfs.20260727/00/atmos/
aws s3 cp --no-sign-request \
  s3://noaa-gfs-bdp-pds/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2 .
```

### 2. Direct HTTPS download
```bash
curl -O https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2
```

### 3. wget
```bash
wget https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260727/00/atmos/gfs.t00z.sfluxgrbf000.grib2
```

## Public Archive

**AWS S3 Bucket:** `noaa-gfs-bdp-pds` (NOAA Big Data Program)
- **Region:** us-east-1
- **Access:** Public, no authentication required
- **Registry:** https://registry.opendata.aws/noaa-gfs-bdp-pds/

**Availability:** 30-day rolling window

Data is updated 4 times daily (00Z, 06Z, 12Z, 18Z cycles). Files for each cycle include forecast hours F000-F384.

## Additional Gaussian Grid Products

### GFS T1534 Semi-Lagrangian (surface flux)
- Pattern: `gfs.tCCz.sfluxgrbfFFF.grib2`
- Grid: Gaussian T1534 (~0.117° resolution)
- Available for all forecast cycles

### GDAS T574 Gaussian grid (surface flux)  
- Pattern: `gdas.tCCz.sfluxgrbfFFF.grib2`
- Grid: Gaussian T574
- Note: GDAS files may be in different directory structures

## Sources

- [NCEP GFS Products Page](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [GRIB2 Table 3.40 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)
- [NOAA GFS Big Data Program Registry](https://registry.opendata.aws/noaa-gfs-bdp-pds/)
- [AWS CLI S3 Access](https://noaa-gfs-bdp-pds.s3.amazonaws.com/index.html)

## Notes

- The standard GFS 0.5° products (gfs.tCCz.pgrb2.0p50.fFFF) use regular latitude-longitude grids, NOT Gaussian grids
- Gaussian grids in GFS output are primarily used for surface flux products (sfluxgrb)
- GDT 3.40 is the GRIB2 template for Gaussian Latitude/Longitude grids
- The T1534 Gaussian grid has N=768, which corresponds to approximately 1536 latitude points (2N) and 3072 longitude points (4N for reduced Gaussian)
