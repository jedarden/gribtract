# GDT 3.30 Candidate Models Catalog

*Extracted from bead bf-1wwi research on 2026-07-03*

## Primary Candidates (VERIFIED DRT=3)

### 1. HRRR (High-Resolution Rapid Refresh)

- **Model:** HRRR
- **Grid:** Grid #184
- **Resolution:** 3 km (2.54 km effective)
- **Coverage:** CONUS
- **Projection:** Lambert Conformal Conic (GDT 3.30)
- **Central Latitude:** 38.5°N
- **Central Longitude:** 97.5°W (262.5°W)
- **DRT=3 Status:** ✓ VERIFIED (~82-100% of messages)
- **Archive:** [AWS S3: noaa-hrrr-bdp-pds](https://noaa-hrrr-bdp-pds.s3.amazonaws.com/)

### 2. NAM CONUS (North American Mesoscale)

- **Model:** NAM CONUS
- **Grid:** Grid #218 (12 km Lambert CONUS primary)
- **Alternative Grids:** Grid #197 (5 km), Grid #184 (2.5 km), Grid #91 (3 km AK)
- **Resolution:** Multiple: 3 km, 12 km, 29 km
- **Coverage:** CONUS, Alaska, Hawaii, Puerto Rico
- **Projection:** Lambert Conformal Conic (GDT 3.30)
- **DRT=3 Status:** ✓ VERIFIED (100% of awphys messages)
- **Archive:** [AWS S3: noaa-nam-pds](https://noaa-nam-pds.s3.amazonaws.com/)

## Secondary Candidates (Documented Complex Packing)

### 3. NDFD / NBM (National Blend of Models)

- **Model:** NDFD/NBM
- **Grid:** Grid #184 (2.5 km CONUS NDFD), Grid #197 (5 km 16x)
- **Resolution:** 2.5 km, 3 km, 5 km
- **Coverage:** CONUS, Puerto Rico, Hawaii, Guam, Alaska
- **Projection:** Lambert Conformal (GDT 3.30)
- **DRT=3 Status:** Documented complex packing (DRT 5.2/5.3)
- **Archive:** [AWS S3: noaa-nbm-pds](https://noaa-nbm-pds.s3.amazonaws.com/)

### 4. RAP (Rapid Refresh)

- **Model:** RAP
- **Grid:** Grid #130 (13 km Lambert CONUS primary)
- **Alternative Grids:** Grid #236 (40 km), Grid #200 (Puerto Rico 16 km)
- **Resolution:** 13 km (primary), 40 km regional
- **Coverage:** CONUS
- **Projection:** Lambert Conformal Conic (GDT 3.30)
- **DRT=3 Status:** Likely complex packing (requires file verification)
- **Archive:** [AWS S3: noaa-rap-pds](https://noaa-rap-pds.s3.amazonaws.com/)

## Additional Regional Models (DRT=3 Status Unknown)

| Model | Grid | Resolution | Coverage | DRT=3 Status |
|-------|------|------------|----------|--------------|
| RRFS | TBD | 3 km | CONUS | UNKNOWN (in development) |
| HREF | Multiple | 5 km | CONUS, AK, HI | UNKNOWN |
| SREF | Multiple | 32-45 km | CONUS, AK | UNKNOWN |
| RTMA/URMA | #184, others | 2.5 km | CONUS, AK, HI, PR | UNKNOWN |
| HiResW | Multiple | 5 km | Eastern/Western US, AK | UNKNOWN |
| GFS CONUS | #211 | 80 km | CONUS subset | UNKNOWN |

## Technical Reference

- **GDT 3.30:** [NOAA NCEP GRIB2 Template 3.30](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)
- **DRT=3:** [GRIB2 Template 5.3 - Complex Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml)

## Summary

**Primary candidates for DRT=3 with GDT 3.30:**
1. **HRRR** (Grid #184) - VERIFIED DRT=3
2. **NAM CONUS** (Grid #218) - VERIFIED DRT=3

**Secondary candidates requiring verification:**
3. **NDFD/NBM** (Grid #184/#197) - Documented complex packing
4. **RAP** (Grid #130) - Likely complex packing
