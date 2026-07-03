# NOAA Regional Model GRIB2 Archives Research

**Bead:** bf-3l2y
**Date:** 2026-07-02
**Task:** Research NOAA's public GRIB2 archives for regional weather models

## Summary

This document catalogs NOAA regional models that use GRIB2 encoding with Grid Definition Template (GDT) 3.30 (Lambert-conformal conic projection) and Data Representation Template (DRT) 3 (complex packing and spatial differencing). It includes archive URLs, access patterns, file naming conventions, and typical file sizes.

## Models Using GDT 3.30 (Lambert-Conformal Conic)

### 1. HRRR (High-Resolution Rapid Refresh)

**Description:** Real-time 3-km resolution, hourly-updated, cloud-resolving, convection-allowing atmospheric model over CONUS.

**Grid Specifications:**
- Resolution: 3 km
- Domain: CONUS (Continental United States)
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **AWS Open Data Registry:** https://registry.opendata.aws/noaa-hrrr-pds/
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/hrrr/
- **University of Utah HRRR Archive:** https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/hrrr_download.cgi
- **Microsoft Planetary Computer:** https://planetarycomputer.microsoft.com/dataset/storage/noaa-hrrr

**Run Schedule:**
- **Hourly** model runs (00Z, 01Z, 02Z, ..., 23Z)
- Forecast duration: 
  - All hourly cycles: 18 hours
  - Extended 00Z, 06Z, 12Z, 18Z cycles: 48 hours
- Sub-hourly output available every 15 minutes within each hour

**File Naming Convention:**
- Surface: `hrrr.t[HH]z.wrfsfcf[00-48].grib2`
- Pressure: `hrrr.t[HH]z.wrfprsf[00-48].grib2`

**Typical File Sizes:**
- Surface fields (wrfsfcf): >100 MB per file
- Pressure fields (wrfprsf): >380 MB per file
- Full day of pressure analyses: >9 GB
- Complete 18-hour forecast cycle: ~500 MB total

**Access Method:** HTTPS (public)

**Sources:**
- [HRRR Download Page - University of Utah](https://home.chpc.utah.edu/~u0553130/Brian_Blaylock/cgi-bin/hrrr_download.cgi)
- [AWS Open Data Registry - NOAA HRRR](https://registry.opendata.aws/noaa-hrrr-pds/)
- [NCEP Data Products HRRR](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [Brian Blaylock's HRRR FAQ](https://blaylockbk.github.io/Web-Homepage/hrrr_FAQ.html)

---

### 2. NAM (North American Mesoscale Forecast System)

**Description:** Regional forecast model over North America at multiple resolutions and domains.

**Grid Specifications:**
- NAM 218 AWIPS Grid (CONUS): 12-km resolution
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **AWS Open Data Registry:** https://registry.opendata.aws/noaa-nam/
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/nam/
- **UCAR NAM Archive:** https://data.ucar.edu/dataset/ncep-north-american-mesoscale-nam-12-km-analysis
- **GribStream:** https://gribstream.com/blog/noaa-nam-datasets-now-available

**Run Schedule:**
- **4 times daily** at 00Z, 06Z, 12Z, 18Z
- Each cycle generates 61 GRIB files covering different forecast hours

**File Naming Convention:**
- `nam.t[HH]z.conusnest.hiresf[FF].tm00.grib2`
- `nam.t[HH]z.awip12[FF].tm00.grib2` (12-km CONUS)
- `nam.t[HH]z.awphys[FF].tm00.grib2`

**Typical File Sizes:**
- ~56.5 MB for awphys00 files (example: `nam.t00z.awphys00.tm00.grib2` = 59,241,584 bytes)
- Varies by forecast cycle, time step, and parameters included

**Domains Available:**
- awphys
- awip12 (CONUS 12-km Lambert grid)
- conusnest.hiresf (CONUS high-resolution)
- goes218

**Access Method:** HTTPS (public)

**Historical Availability:** Back to September 16, 2021 (via GribStream)

**Sources:**
- [NCEP Data Products NAM](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [AWS Open Data Registry - NOAA NAM](https://registry.opendata.aws/noaa-nam/)
- [UCAR NAM Archive](https://data.ucar.edu/dataset/ncep-north-american-mesoscale-nam-12-km-analysis)
- [GribStream NAM Blog](https://gribstream.com/blog/noaa-nam-datasets-now-available)

---

### 3. NBM (National Blend of Models)

**Description:** Nationally consistent and skillful suite of calibrated forecast guidance blending multiple models.

**Grid Specifications:**
- CONUS: 2.5-km resolution
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **AWS Open Data Registry:** https://registry.opendata.aws/noaa-nbm/
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/blend/
- **GribStream:** https://gribstream.com/models/nbm

**Run Schedule:**
- **4 times daily** at 00Z, 06Z, 12Z, 18Z (QMD files)

**S3 Path Pattern (AWS):**
- `s3://noaa-nbm-grib2-pds/blend.YYYYMMDD/HH/`

**Access Method:** HTTPS (public S3)

**Documentation:**
- [NOAA VLab - NBM v4.0 GRIB2 Documentation](https://vlab.noaa.gov/web/mdl/nbm-grib2-v4.0)
- [Herbie Documentation - NBM](https://herbie.readthedocs.io/en/2023.12.4/user_guide/_model_notebooks/nbm.html)

**Sources:**
- [NCEP Central Operations - NBM Products](https://www.nco.ncep.noaa.gov/pmb/products/blend/)
- [AWS Open Data Registry - NOAA NBM](https://registry.opendata.aws/noaa-nbm/)
- [GribStream - NBM CONUS Core Grid](https://gribstream.com/models/nbm)

---

### 4. RAP (Rapid Refresh)

**Description:** Continental-scale hourly-updated assimilation/modeling system.

**Grid Specifications:**
- Domain: Continental-scale
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **AWS Open Data Registry:** https://registry.opendata.aws/noaa-rap/
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/rap/
- **Microsoft Planetary Computer:** https://planetarycomputer.microsoft.com/dataset/storage/noaa-rap

**Run Schedule:**
- Hourly updates

**Access Method:** HTTPS (public)

**Sources:**
- [NCEP Data Products RAP](https://www.nco.ncep.noaa.gov/pmb/products/rap/)
- [AWS Open Data Registry - NOAA RAP](https://registry.opendata.aws/noaa-rap/)
- [Herbie Documentation - RAP](https://herbie.readthedocs.io/en/2024.3.0/user_guide/_model_notebooks/rap.html)

---

### 5. RTMA (Real-Time Mesoscale Analysis)

**Description:** High-quality analyses for nowcasting and situational awareness, supporting NDFD operations.

**Grid Specifications:**
- CONUS: 2.5 km Lambert Conformal grid
- Alaska: ~3 km Lambert Conformal grid
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **AWS Open Data Registry:** https://registry.opendata.aws/noaa-rtma/
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/rtma/
- **Google Earth Engine:** https://developers.google.com/earth-engine/datasets/catalog/NOAA_NWS_RTMA

**Run Schedule:**
- **Hourly** analyses

**Access Method:** HTTPS (public)

**Sources:**
- [NCEP Central Operations RTMA/URMA Products](https://www.nco.ncep.noaa.gov/pmb/products/rtma/)
- [AWS Open Data Registry - NOAA RTMA](https://registry.opendata.aws/noaa-rtma/)
- [GribStream RTMA API](https://gribstream.com/models/rtma)

---

### 6. SREF (Short-Range Ensemble Forecast)

**Description:** Regional short-range (0-3 days) ensemble prediction system with 21 members.

**Grid Specifications:**
- AWIPS Grid 212
- Resolution: 40 km
- Grid dimensions: 185×129
- Domain: CONUS
- Projection: Lambert Conformal Conic (GDT 3.30)

**Archive URLs:**
- **NCEP Central Operations:** https://www.nco.ncep.noaa.gov/pmb/products/sref/

**Run Schedule:**
- **4 times daily** at 03Z, 09Z, 15Z, 21Z

**Important Note:** On July 25, 2025, NOAA/NWS issued a **proposal to terminate the SREF system**, with GEFS (Global Ensemble Forecast System) suggested as the replacement.

**Sources:**
- [NCEP Central Operations SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)

---

## GRIB2 Encoding Details

### Grid Definition Template (GDT) 3.30

**Official Documentation:** [NCEP PMB - Grid Definition Template 3.30](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)

**Key Technical Details:**
- Template for Lambert Conformal projections (secant or tangent, conical or bipolar)
- Grid lengths are in units of 10^-3 m at the latitude specified by LaD
- If Latin1 = Latin2, the projection is on a tangent cone; otherwise on a secant cone
- Standard template used at NCEP for regional models

**Related Templates:**
- **Template 3.30:** Lambert conformal (standard for regional models)
- **Template 3.33:** Lambert conformal with modelling subdomains definition (variant of 3.30)

**Additional Documentation:**
- [CPC wgrib2 Documentation - New Grid](https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/new_grid.html)
- [Introduction to Grids (CPC FTP)](https://ftp.cpc.ncep.noaa.gov/wd51we/wgrib2/grib2/intro_grids.doc)

### Data Representation Template (DRT) Details

**Official Documentation:** [NCEP PMB - GRIB2 Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml)

**Key Templates:**
- **DRT 0:** Simple packing
- **DRT 3:** Grid point data - complex packing and spatial differencing
  - Uses spatial differencing as pre-processing before group splitting
  - Reduces size of sufficiently smooth fields
  - Subdivides field of values into NG groups where values in each group have similar sizes
- **DRT 51:** Spectral data with complex packing (Template 5.53)
  - Used for spectral data for limited area models

**Complex Packing Documentation:**
- [NCEP PMB - Grid point data - complex packing and spatial differencing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml)

---

## Common Access Patterns

### HTTPS Direct Download

All major archives support HTTPS direct download:

```bash
# Example HRRR from AWS
curl -O https://noaa-hrrr-pds.s3.amazonaws.com/20250702/00/conus/hrrr.t00z.wrfsfcf00.grib2

# Example NAM from AWS
curl -O https://noaa-nam-pds.s3.amazonaws.com/nam.20250702/00/nam.t00z.awphys00.tm00.grib2
```

### Cloud Storage Access

**Amazon S3 URLs:**
- HRRR: `https://noaa-hrrr-pds.s3.amazonaws.com/`
- NAM: `https://noaa-nam-pds.s3.amazonaws.com/`
- NBM: `https://noaa-nbm-grib2-pds.s3.amazonaws.com/`
- RAP: `https://noaa-rap-pds.s3.amazonaws.com/`
- RTMA: `https://noaa-rtma-pds.s3.amazonaws.com/`

### Python Tools

**Herbie Package:** Python package for downloading NOAA model data
- Documentation: https://herbie.readthedocs.io/
- Supports HRRR, NAM, NBM, RAP, RTMA, and other models
- Automatically handles source selection (AWS, Google Cloud, NOAA)

---

## File Size Summary Table

| Model | Domain | Resolution | Typical File Size | Run Frequency |
|-------|--------|------------|-------------------|----------------|
| HRRR | CONUS | 3 km | ~100-120 MB | Hourly |
| NAM | CONUS | 12 km (awip12) | ~56 MB | 4x daily |
| NBM | CONUS | 2.5 km | Varies | 4x daily |
| RAP | Continental | Varies | Varies | Hourly |
| RTMA | CONUS | 2.5 km | Varies | Hourly |
| SREF | CONUS | 40 km | Varies | 4x daily* |

*SREF proposed for termination July 2025

---

## Archive Availability Patterns

### Retention Periods

- **HRRR:** Historical data available via AWS and Google Cloud (since late 2020)
- **NAM:** Historical data back to September 16, 2021 (via GribStream)
- **NBM:** Continuous archives available via AWS
- **RAP:** Continuous archives available via AWS
- **RTMA:** Continuous archives available via AWS

### THREDDS/OPeNDAP Access

Some archives may provide THREDDS/OPeNDAP services for subsetting and remote access. Check individual archive pages for OPeNDAP endpoints.

---

## Additional Resources

### Documentation

- [NCEP PMB GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [Brian Blaylock's HRRR FAQ](https://blaylockbk.github.io/Web-Homepage/hrrr_FAQ.html)
- [Herbie Documentation](https://herbie.readthedocs.io/)

### Tools

- **wgrib2:** GRIB2 decoder/encoder (CPC): https://www.cpc.ncep.noaa.gov/products/tools/wgrib2/
- **gribstream.com:** API access to multiple NOAA models
- **LuckGrib:** GRIB visualization and download tool: https://luckgrib.com/

---

## Summary of Acceptance Criteria

✅ **Listed 2-3 NOAA regional models that potentially use GDT 3.30**
   - HRRR (3 km CONUS)
   - NAM (12 km CONUS)
   - NBM (2.5 km CONUS)
   - RAP (continental)
   - RTMA (2.5 km CONUS)
   - SREF (40 km CONUS)

✅ **Documented archive URLs and access methods (HTTPS, THREDDS, etc.)**
   - AWS Open Data Registry (HTTPS)
   - NCEP Central Operations (HTTPS)
   - Google Cloud Platform (HTTPS)
   - Microsoft Planetary Computer (HTTPS)
   - University of Utah archives (HTTPS)
   - GribStream API

✅ **Noted typical file sizes for single model output files**
   - HRRR: ~100-120 MB per file
   - NAM: ~56 MB per file (awphys00 example)

✅ **Recorded model run schedules and availability patterns**
   - HRRR: Hourly
   - NAM: 4x daily (00Z, 06Z, 12Z, 18Z)
   - NBM: 4x daily (00Z, 06Z, 12Z, 18Z)
   - RAP: Hourly
   - RTMA: Hourly
   - SREF: 4x daily (03Z, 09Z, 15Z, 21Z) - proposed for termination

---

**End of Research Document**
