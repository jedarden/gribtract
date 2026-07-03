# NOAA GRIB2 Archive URLs for Lambert-Conformal (GDT 3.30) + Complex Packing (DRT=3)

## Research Findings

### Primary Model: NOAA HRRR (High-Resolution Rapid Refresh)

The HRRR model is confirmed to use:
- **Grid Definition Template (GDT): 3.30** - Lambert Conformal Conic projection
- **Data Representation Template (DRT): 3** - Complex packing with spatial differencing
- **Packing Type:** `grid_complex_spatial_differencing`

#### Archive URL

**AWS S3 Bucket:** `noaa-hrrr-bdp-pds`
- **S3 URL:** `s3://noaa-hrrr-bdp-pds/`
- **HTTPS Access:** `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/`

#### Direct Download URL Pattern

```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tHHz.wrfsfcfFF.grib2
```

#### Example Verified URLs

| Model | Date | Cycle | Forecast Hour | Download URL |
|-------|------|-------|---------------|--------------|
| HRRR | 2024-06-01 | 12z | f00 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2` |
| HRRR | 2024-01-01 | 00z | f00 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240101/conus/hrrr.t00z.wrfsfcf00.grib2` |
| HRRR | 2021-07-21 | 00z | f00 | `https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20210721/conus/hrrr.t00z.wrfsfcf00.grib2` |

#### File Products

- **wrfsfc** - Surface fields (most common)
- **wrfnat** - Native fields
- **wrfprs** - Pressure level fields
- **wrfsubh** - Sub-hourly fields

#### Provenance Details

- **Model Name:** NOAA High-Resolution Rapid Refresh (HRRR)
- **Resolution:** 3 km
- **Domain:** CONUS (Continental United States)
- **Projection:** Lambert Conformal Conic
- **Update Frequency:** Hourly
- **Archive Range:** Available via AWS from approximately 2015-present

---

### Alternative Models with GDT 3.30

#### NAM CONUS (North American Mesoscale Forecast System)

**AWS S3 Bucket:** `noaa-nam-pds`
- **S3 URL:** `s3://noaa-nam-pds/`
- **HTTPS Access:** `https://noaa-nam-pds.s3.amazonaws.com/`

**Product:** NAM AWIPS Grid (218 grid - 12 km resolution)
- **URL Pattern:** `https://noaa-nam-pds.s3.amazonaws.com/nam.YYYYMMDD/nam.tHHz.awphysFF.tm00.grib2`
- **Archive Starts:** September 16, 2021
- **Note:** NCEP documentation confirms NAM CONUS uses GDT 3.30

#### RRFS (Rapid Refresh Forecast System)

**AWS S3 Bucket:** `noaa-rrfs-pds`
- **S3 URL:** `s3://noaa-rrfs-pds/`
- **HTTPS Access:** `https://noaa-rrfs-pds.s3.amazonaws.com/`
- **Resolution:** 3 km CONUS
- **Status:** Prototype/development
- **Note:** Uses Lambert Conformal projection (specific GDT version not verified in this research)

---

## Key Resources

1. **[NOAA HRRR on AWS Open Data Registry](https://registry.opendata.aws/noaa-hrrr-pds/)** - Official registry page
2. **[NCEP HRRR Products](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)** - Official product documentation
3. **[NCEP GRIB2 Seminar](https://www.nco.ncep.noaa.gov/sib/grib2/NCEP_GRIB2_seminar_v2.ppt)** - Confirms NAM uses GDT 3.30
4. **[Herbie Documentation](https://herbie.readthedocs.io/)** - Python package for downloading NOAA GRIB2 data
5. **[ECMWF GRIB2 Code Table 3.1](https://codes.ecmwf.int/grib/format/grib2/ctables/3/1/)** - Confirms template 30 is "Lambert conformal"

---

## Verification

The following command was used to verify HRRR uses GDT 3.30 + DRT=3:

```python
import eccodes
with open('hrrr_sample.grib2', 'rb') as f:
    msg_id = eccodes.codes_grib_new_from_file(f)
    print(f"Grid Type: {eccodes.codes_get(msg_id, 'gridType')}")  # lambert
    print(f"GDT: {eccodes.codes_get(msg_id, 'gridDefinitionTemplateNumber')}")  # 30
    print(f"Packing: {eccodes.codes_get(msg_id, 'packingType')}")  # grid_complex_spatial_differencing
```

**Result:**
- Grid Type: `lambert`
- GDT: `30` (Lambert Conformal Conic)
- Packing: `grid_complex_spatial_differencing` (DRT=3)

---

## Download Examples

```bash
# Download HRRR surface fields for June 1, 2024, 12z cycle, forecast hour 0
curl -O https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2

# Using AWS CLI (if configured)
aws s3 cp s3://noaa-hrrr-bdp-pds/hrrr.20240601/conus/hrrr.t12z.wrfsfcf00.grib2 .

# Using Herbie Python package
herbie fetch 2024-06-01 12:00 model=hrrr product=wrfsfcf00
```

---

## Notes

1. **Complex Packing (DRT=3):** The `grid_complex_spatial_differencing` packing type corresponds to GRIB2 Data Representation Template 3, which uses complex packing with spatial differencing for better compression of smooth meteorological fields.

2. **Lambert Conformal (GDT 3.30):** GRIB2 Grid Definition Template 3.30 is used for Lambert Conformal Conic projections, which are standard for regional weather models over mid-latitude domains like CONUS.

3. **Public Access:** All URLs are publicly accessible via HTTPS without authentication (NOAA Open Data initiative).

4. **File Size:** HRRR wrfsfcf00 files are typically 100-150 MB.

5. **Index Files:** Each GRIB2 file has a corresponding `.idx` file for subsetting individual variables without downloading the entire file.

---

*Research completed for bead bf-13e3 on 2026-07-02*
