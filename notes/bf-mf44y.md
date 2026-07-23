# Specific GEFS Ensemble GRIB2 Files for Test Fixtures

## Task Completed: bf-mf44y

Located specific downloadable GRIB2 files from NOAA GEFS ensemble archives.

## Files Identified

### 1. Individual Ensemble Control Member (PDT 4.1/4.8)

**URL:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000`

**File Details:**
- **Size:** 13,476,191 bytes (~12.9 MB) ✓ Suitable for test fixture
- **Last Modified:** 2026-07-23T03:46:18.000Z
- **Forecast Run:** 2026-07-23 00Z
- **Forecast Hour:** f000 (analysis time)
- **Resolution:** 0.5° (pgrb2ap5)
- **Member:** gec00 (control member)
- **Expected PDT:** 4.1 or 4.8 (individual ensemble member)

**Index File:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000.idx` (4,085 bytes)

### 2. Ensemble Mean File (PDT 4.2)

**URL:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000`

**File Details:**
- **Size:** 13,991,214 bytes (~13.3 MB) ✓ Suitable for test fixture
- **Last Modified:** 2026-07-23T03:48:28.000Z
- **Forecast Run:** 2026-07-23 00Z
- **Forecast Hour:** f000 (analysis time)
- **Resolution:** 0.5° (pgrb2ap5)
- **Product:** geavg (ensemble average/mean)
- **Expected PDT:** 4.2 (derived product - ensemble mean)

**Index File:** `https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000.idx` (3,589 bytes)

## File Naming Convention

Pattern: `gefs.YYYYMMDD/HH/atmos/pgrb2ap5/PRODUCT.tHZz.pgrb2a.0p50.fFFF`

- `YYYYMMDD`: Forecast run date
- `HH`: Forecast cycle (00, 06, 12, 18Z)
- `pgrb2ap5`: GRIB2 product at 0.5° resolution
- `PRODUCT`: 
  - `gec00`: Control member (individual ensemble)
  - `gep01` through `gep30`: Perturbation members
  - `geavg`: Ensemble average (derived statistical product)
- `tHZz`: Cycle time (t00z = 00Z cycle)
- `fFFF`: Forecast hour (f000 = analysis, f003 = 3-hour forecast, etc.)

## Archive Source

**Base URL:** `https://noaa-gefs-pds.s3.amazonaws.com/`

- **Provider:** NOAA NCEP via Amazon S3 Public Data Set
- **Access Method:** HTTPS (public, no authentication required)
- **Availability:** Real-time and recent forecast data
- **Structure:** `gefs.YYYYMMDD/HH/atmos/pgrb2[ab]p#5/`

## Verification

Both files verified accessible with:
- HTTP 200 OK response
- Valid GRIB2 format confirmed
- Reasonable file sizes for test fixtures
- Index files available for subsetting

## Download Example

```bash
# Download individual ensemble member file
curl -o gec00_f000.grib2 "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000"

# Download ensemble mean file
curl -o geavg_f000.grib2 "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"

# Download index file for subsetting
curl -o gec00_f000.idx "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000.idx"
```

## Recommendation

For PDT 4.1/4.8 testing, use the **gec00** file (individual ensemble control member).
For PDT 4.2 testing, use the **geavg** file (ensemble mean derived product).

Both files are suitable as test fixtures due to their manageable size (~13MB) and recent availability.
