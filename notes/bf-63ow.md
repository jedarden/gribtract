# NOAA Regional Models Research - Lambert Conformal GRIB2 Archives

## Executive Summary

Three NOAA regional NWP models produce Lambert-conformal (GDT 3.30) GRIB2 output suitable for complex packing (DRT=3) applications:

1. **HRRR** (High-Resolution Rapid Refresh) - 3km resolution
2. **NAM** (North American Mesoscale) - Grid 227 at 5km resolution
3. **RAP** (Rapid Refresh) - 13km resolution

All three models use GRIB2 Grid Definition Template 3.30 (Lambert Conformal Conic) and are available via public archives.

---

## HRRR (High-Resolution Rapid Refresh)

### Model Specifications
- **Resolution**: 3-km horizontal resolution
- **Update Frequency**: Hourly model runs
- **Forecast Length**: 18 hours standard; 48 hours for 00, 06, 12, 18 UTC cycles
- **Coverage**: CONUS (Continental United States)
- **Projection**: Lambert Conformal Conic (explicitly confirmed by NCEP)
- **Grid Definition Template**: GDT 3.30

### Archive Locations

#### AWS Open Data Registry (Primary)
- **S3 Bucket**: `s3://noaa-hrrr-bdp-pds/`
- **Explorer**: https://noaa-hrrr-bdp-pds.s3.amazonaws.com/index.html
- **Path Structure**: `hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2`
- **Registry Page**: https://registry.opendata.aws/noaa-hrrr-pds/

#### University of Utah Archive
- **URL**: https://mesowest.utah.edu/html/hrrr/
- **Documentation**: https://blaylockbk.github.io/Web-Homepage/hrrr_FAQ.html

#### NOAA Sources
- **NCEP Products**: https://www.nco.ncep.noaa.gov/pmb/products/hrrr/
- **Official Page**: https://rapidrefresh.noaa.gov/hrrr/

### File Naming Convention
- **Surface (2D)**: `hrrr.tCCz.wrfsfcfFF.grib2` (CC=cycle 00-23, FF=forecast hour 00-18/48)
- **Pressure (3D)**: `hrrr.tCCz.wrfprsfFF.grib2`
- **Native Levels**: `hrrr.tCCz.wrfnatfFF.grib2`
- **Sub-hourly**: `hrrr.tCCz.wrfsubhfFF.grib2` (15-min intervals)

### File Sizes
- **Surface files**: ~120-150 MB each (~146 MB typical)
- **Pressure files**: ~380 MB each
- **Full day of pressure analyses**: >9 GB

### Retention Policy
- AWS archive: Historical data back to 2014
- No explicit retention limit documented (data persists)

---

## NAM (North American Mesoscale)

### Model Specifications
- **Resolution**: Grid 227 at 5-km CONUS resolution
- **Cycles**: 00, 06, 12, 18 UTC
- **Forecast Length**: Up to 60 hours for nest products
- **Coverage**: CONUS via Grid 227
- **Projection**: Lambert Conformal Conic (Grid 227 confirmed)
- **Grid Definition Template**: GDT 3.30

### Archive Locations

#### AWS Open Data Registry
- **S3 Bucket**: `s3://noaa-nam-pds/`
- **HTTPS**: `https://noaa-nam-pds.s3.amazonaws.com/`
- **Registry Page**: https://registry.opendata.aws/noaa-nam/
- **Path Structure**: `nam.YYYYMMDD/nam.tCCz.{product}XX.tm00.grib2`

#### NCEP FTP/HTTPS
- **Products Page**: https://www.nco.ncep.noaa.gov/pmb/products/nam/
- **Protocol**: GRIB2 via FTP and HTTPS
- **Access**: `ftp.ncep.noaa.gov` or HTTPS downloads

#### NOMADS
- **URL**: https://nomads.ncep.noaa.gov/
- **Capabilities**: GRIB filter for selective parameter downloads

### File Naming Convention
- **Grid 227 CONUS Nest**: `nam.tCCz.conusnest.hiresffh.tm00.grib2`
- **Alaska Nest (Grid 198)**: `nam.tCCz.alaskanest.hiresffh.tm00.grib2`
- **Hawaii Nest (Grid 196)**: `nam.tCCz.hawaiinest.hiresffh.tm00.grib2`
- **Puerto Rico Nest (Grid 194)**: `nam.tCCz.priconest.hiresffh.tm00.grib2`

### Related NAM Grids
- **Grid 218**: 12-km AWIPS CONUS
- **Grid 221**: 32-km North American Master
- **Grid 190**: 12-km rotated lat/lon (not Lambert Conformal)

---

## RAP (Rapid Refresh)

### Model Specifications
- **Resolution**: 13-km horizontal resolution
- **Cycles**: Every hour (03/09/15/21 UTC run to 51h; others to 21h)
- **Coverage**: CONUS (Continental United States)
- **Projection**: Lambert Conformal Conic (confirmed by NOAA NCEI metadata)
- **Grid Definition Template**: GDT 3.30

### Archive Locations

#### AWS Open Data Registry
- **S3 Bucket**: `s3://noaa-rap-pds/`
- **Registry Page**: https://registry.opendata.aws/noaa-rap/
- **Documentation**: https://www.nco.ncep.noaa.gov/pmb/products/rap/

#### Google Cloud
- **Marketplace**: https://console.cloud.google.com/marketplace/product/noaa-public/rapid-refresh

#### NOAA Sources
- **NCEP Products**: https://www.nco.ncep.noaa.gov/pmb/products/rap/
- **Official Page**: https://rapidrefresh.noaa.gov/

### File Naming Convention
- **AWIPS 32km**: `rap.tCCz.awip32fFF.grib2`
- **AWIPS Grid 130**: `rap.tCCz.awp130pgrbfFF.grib2`
- **WRF Native**: `rap.tCCz.wrfnatfFF.grib2`
- **WRF MSL**: `rap.tCCz.wrfmslfFF.grib2`

### File Sizes
- **NAM awphys**: ~56 MB per file (e.g., `nam.t00z.awphys00.tm00.grib2` ≈ 59 MB)
- **RAP**: Varies by product (typically smaller than HRRR/NAM due to 13km resolution)

### Retention Policy
- AWS archives: Historical data available (varies by model)
- NCEP servers: Typically 30-90 days retention
- No explicit long-term retention limit documented for AWS

---

## GRIB2 Technical Details

### Grid Definition Template 3.30 (Lambert Conformal)
- **Official Documentation**: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml
- **Parameters**: Central latitude/longitude, standard parallels, grid dimensions, spacing

### Data Representation Types (DRT)
- **DRT=3**: Complex packing (multiple variants: complex1, complex2, complex3)
- **Usage**: NOAA models use complex packing for efficient data compression
- **Trade-off**: Complex3 offers best speed/compression balance for smooth fields

### Access Tools
- **wgrib2**: NOAA's GRIB2 manipulation tool
- **Herbie**: Python package for downloading NOAA model data
- **HRRR-B**: Python package by Brian Blaylock for HRRR specifically

---

## Recommendations for Complex Packing (DRT=3) Testing

### Best Candidates
1. **HRRR**: Most modern, highest resolution (3km), hourly updates, excellent AWS archive access
2. **NAM Grid 227**: 5km resolution, proven CONUS coverage, established archive
3. **RAP**: 13km resolution, hourly updates, good for initial testing (smaller files)

### Sample File Access
```bash
# HRRR from AWS (most accessible)
aws s3 ls s3://noaa-hrrr-bdp-pds/hrrr.20250701/conus/

# NAM from AWS S3
aws s3 ls s3://noaa-nam-pds/nam.20250701/
# Or via HTTPS
curl -O https://noaa-nam-pds.s3.amazonaws.com/nam.20250701/nam.t00z.awphys00.tm00.grib2

# NAM from NCEP (alternative)
wget https://www.nco.ncep.noaa.gov/pmb/products/nam/nam.t00z.conusnest.hiresf00.tm00.grib2

# RAP from AWS
aws s3 ls s3://noaa-rap-pds/
```

---

## References

### Sources
- [NCEP HRRR Products](https://www.nco.ncep.noaa.gov/pmb/products/hrrr/)
- [NCEP NAM Products](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [NCEP RAP Products](https://www.nco.ncep.noaa.gov/pmb/products/rap/)
- [AWS HRRR Registry](https://registry.opendata.aws/noaa-hrrr-pds/)
- [AWS RAP Registry](https://registry.opendata.aws/noaa-rap/)
- [GRIB2 GDT 3.30 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)
- [Brian Blaylock's HRRR FAQ](https://blaylockbk.github.io/Web-Homepage/hrrr_FAQ.html)
- [University of Utah HRRR Archive](https://mesowest.utah.edu/html/hrrr/)
- [NOAA NCEI RAP Metadata](https://www.ncei.noaa.gov/access/metadata/landing-page/bin/iso?id=gov.noaa.ncdc:C00690)

---

## Task Completion

### Acceptance Criteria Met
- ✅ List of 3 NOAA regional models (HRRR, NAM, RAP)
- ✅ Archive base URLs for each model
- ✅ All three confirmed to use Lambert Conformal (GDT 3.30) projection
- ✅ File naming conventions and access patterns documented

### Additional Findings
- File sizes documented (HRRR: ~120-380 MB per file)
- Retention policies (AWS archives persist back to 2014 for HRRR)
- Multiple access methods (AWS S3, NCEP FTP, NOMADS)
- Related tools and documentation (wgrib2, Herbie, HRRR-B)
