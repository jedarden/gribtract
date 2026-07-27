# Gaussian-Grid GRIB2 Source Research (bf-1fjrsd)

## Task Objective
Research and identify a specific NOAA Gaussian-grid (GDT 3.40) GRIB2 file from public archives suitable for inclusion in the gribtract test corpus.

## Key Findings

### GDT 3.40 - Gaussian Latitude/Longitude Grid
- **Definition**: Grid Definition Template 3.40 is specifically for Gaussian Latitude/Longitude grids in GRIB2 format
- **Official Documentation**: [NCEP GRIB2 Table 3.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
- **Template Specification**: [Template 3.40 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)

### Gaussian Grid vs. Regular Grid
- Most standard NCEP GFS/GDAS files (0.5°, 0.25°) use **regular latitude-longitude grids (GDT 0)**, not Gaussian grids (GDT 3.40)
- Example: Existing gribtract test file `gfs_20260724_00z_1p00_f000.grib2` uses `grid_template=0`

### Known Gaussian Grid Products

#### 1. GDAS Surface Flux T574 Gaussian Grid
- **Product**: GDAS - Surface Flux T574 Gaussian grid
- **File Naming Convention**: `gdas.tCCz.sfluxgrbfFFF.grib2`
  - CC = model cycle runtime (00, 06, 12, 18)
  - FFF = forecast hour (000-009)
- **Access Methods**:
  - FTP: `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gfs/prod`
  - HTTPS: `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod`
- **Format**: GRIB2
- **Forecast Hours**: FH000-FH009 (up to 9 hours)

#### 2. NCAR RDA Dataset d084004
- **Dataset**: NCEP operational Global Data Assimilation System surface flux grids on a T574 Gaussian global grid
- **Link**: [NCAR GDEX Dataset d084004](https://gdex.ucar.edu/datasets/d084004/)
- **Specifications**:
  - Grid type: T574 Gaussian global grid
  - Spatial resolution: ~0.117° x ~0.117°
  - Grid dimensions: 3072 x 1536 Longitude/Gaussian Latitude
  - Coverage: 0E to 359.883E and 89.91N to 89.91S
  - Format: WMO GRIB2
  - Temporal range: February 22, 2015 to October 14, 2025

#### 3. NOAA AWS S3 Bucket (noaa-gfs-bdp-pds)
- **Bucket**: `s3://noaa-gfs-bdp-pds/`
- **Public Access**: Available via AWS CLI with `--no-sign-request`
- **Content**: Contains `enkfgdas.YYYYMMDD/` directories
- **Issue**: Files appear to be in NEMSIO format, not GRIB2 format

## Access Challenges

### 403 Forbidden Errors
- Direct HTTPS access to `https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gdas.YYYYMMDD/CC/gdas.tCCz.sfluxgrbfFFF.grib2` returns 403 errors
- May require different access method or authentication

### AWS S3 Access
- Files in `s3://noaa-gfs-bdp-pds/` are primarily NEMSIO format (`gdas.t00z.sfcf003.nemsio`)
- No readily accessible GRIB2 files with Gaussian grids found in this bucket

## Alternative Sources

### ECMWF Data
- ECMWF regularly uses **reduced Gaussian grids** (N1280, etc.)
- Documentation: [ECMWF GRIB files](https://confluence.ecmwf.int/plugins/viewsource/viewpagesrc.action?pageId=23693441)
- Download tools: [Herbie ECMWF documentation](https://herbie.readthedocs.io/en/latest/gallery/ecmwf_models/ecmwf.html)
- Note: Reduced Gaussian grids are different from regular Gaussian grids

### NCEP NOMADS
- URL: https://nomads.ncep.noaa.gov/
- Provides public access to GRIB2 files
- Fast download documentation: https://nomads.ncep.noaa.gov/info.php?page=fastdownload

## Recommendation

**Best Identified Source**: NCAR RDA Dataset d084004 (GDAS Surface Flux T574 Gaussian grid)

This dataset:
- Is documented to contain GRIB2 format files on T574 Gaussian global grid
- Covers a long time period (2015-2025) with public access
- Has clear specifications (~0.117° resolution, 3072 x 1536 grid)
- Should contain GDT 3.40 (Gaussian Latitude/Longitude) template

**Next Steps**:
1. Register for NCAR RDA access (requires registration)
2. Download a sample file from the d084004 dataset
3. Verify the file uses GDT 3.40 using wgrib2 or gribtract tools
4. Document specific file URL, size, and metadata

## Sources
- [NCEP GRIB2 Table 3.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
- [NCEP Template 3.40 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)
- [NCEP GFS/GDAS Products Inventory](https://www.nco.ncep.noaa.gov/pmb/products/gfs/)
- [NCAR GDEX Dataset d084004](https://gdex.ucar.edu/datasets/d084004/)
- [NOAA AWS Open Data Registry - GFS](https://registry.opendata.aws/noaa-gfs-bdp-pds/)
- [ECMWF GRIB Files Documentation](https://confluence.ecmwf.int/)
- [NCEP NOMADS](https://nomads.ncep.noaa.gov/)

## Conclusion
While Gaussian grid GRIB2 files (GDT 3.40) are documented and available, accessing specific publicly-accessible files without authentication requires registration at NCAR RDA. The GDAS Surface Flux T574 Gaussian grid dataset appears to be the most promising source for obtaining verified GDT 3.40 files for the gribtract test corpus.