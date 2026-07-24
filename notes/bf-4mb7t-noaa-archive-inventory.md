# NOAA Archive Sources Inventory and Search Strategy

## Executive Summary

This document inventories all documented NOAA GRIB2 archive sources from the previous catalog (bf-6xddh) and provides detailed search strategies for DRT=0 file discovery across each source.

## Inventory of NOAA GRIB2 Archive Sources

### Priority Tier 1: Highest Likelihood for DRT=0 Files

These sources contain the most comprehensive recent operational model data with well-documented GRIB2 formats.

#### 1. NODD/AWS Big Data Program (Primary Recommendation)

**Source Name:** NOAA Open Data Dissemination Program via Amazon Web Services  
**Base URL:** https://registry.opendata.aws/collab/noaa/  
**Primary S3 Buckets:**
- `noaa-gfs-bdp-pds` - GFS Global Forecast System
- `noaa-nbm-grib2-pds` - National Blend of Models (CONUS)
- `noaa-hrrr-bdp-pds` - High-Resolution Rapid Refresh (CONUS)
- `noaa-gefs-bdp-pds` - Global Ensemble Forecast System

**Access Method:** 
- Direct S3 API (anonymous access)
- HTTPS direct URLs
- AWS CLI with `--no-sign-request`
- Python boto3 with anonymous configuration

**Authentication:** None required (public S3 buckets)

**Expected GRIB2 File Structure:**
```
# GFS structure
gfs.YYYYMMDD/HH/atmos/gfs.tHHz.pgrb2.0p25.fXXX
Example: gfs.20240101/00/atmos/gfs.t00z.pgrb2.0p25.f000

# NBM structure  
nbm.YYYYMMDD/HH/nbm.tHHz.conusnest.02.500mb.grib2

# HRRR structure
hrrr.tHHz.wrfsfcfXX.grib2
```

**Temporal Coverage:**
- GFS: 2019-present (4 cycles/day at 00, 06, 12, 18 UTC)
- NBM: 2021-present (hourly, CONUS focus)
- HRRR: 2015-present (hourly, 3km CONUS)
- GEFS: 2019-present (ensemble members)

**Why High Priority for DRT=0:**
- Most comprehensive recent operational data
- Cloud-optimized access with high bandwidth
- Well-documented GRIB2 formats from NOAA operational models
- CONUS-focused models (NBM, HRRR) likely to contain regional DRT=0 data

**Search Strategy:**
```bash
# 1. Use AWS CLI to explore bucket structure
aws s3 ls s3://noaa-gfs-bdp-pds/ --no-sign-request
aws s3 ls s3://noaa-nbm-grib2-pds/ --no-sign-request

# 2. Download sample files for DRT inspection
aws s3 cp s3://noaa-gfs-bdp-pds/gfs.20240101/00/atmos/gfs.t00z.pgrb2.0p25.f000 ./sample.grib2 --no-sign-request

# 3. Use wgrib2 to scan for DRT values
wgrib2 ./sample.grib2 | grep -E "DRT|drt"

# 4. Implement systematic scan across time periods
# (Write script to iterate through dates/cycles and check DRT values)
```

---

#### 2. NCEI (National Centers for Environmental Information)

**Source Name:** National Centers for Environmental Information  
**Base URL:** https://www.ncei.noaa.gov/  
**API Endpoint:** https://www.ncei.noaa.gov/access/services/data/v1  
**Web Portal:** https://www.ncei.noaa.gov/access/search/dataset-search/

**Access Method:**
- Full REST API with parameter-based queries
- Comprehensive web portal
- Direct HTTPS with query parameters

**Authentication:** None required (public HTTPS)

**Expected GRIB2 File Structure:**
- API returns CSV, JSON, or NetCDF formats
- Original GRIB2 structure preserved in some datasets
- Bounding box spatial selection (bbox=N,W,S,E)
- Date range selection (startDate, endDate)

**Temporal Coverage:**
- Varies by dataset (some dating back to 1901)
- Operational model data typically within last ~1 month
- Long-term climate reanalysis datasets available

**Why High Priority for DRT=0:**
- Long-term archival of NOAA operational models
- Structured API enables targeted geographic searches
- CONUS bounding box queries can filter to regional data
- Comprehensive metadata for dataset discovery

**Search Strategy:**
```bash
# 1. Use REST API to discover available datasets
curl "https://www.ncei.noaa.gov/access/services/data/v1?dataset=gfs-0p25&startDate=2024-01-01&endDate=2024-01-02&bbox=49,-125,25,-65&format=json"

# 2. Query for CONUS coverage specifically
# CONUS bounds: ~49°N to ~25°N, ~125°W to ~65°W
bbox=49,-125,25,-65

# 3. Download GRIB2 files for DRT inspection
# (API returns GRIB2 when format parameter set appropriately)

# 4. Use wgrib2 to scan DRT values in downloaded files
wgrib2 downloaded_file.grib2 | grep -E "DRT|drt"
```

---

#### 3. NOMADS (NOAA Operational Model Archive and Distribution System)

**Source Name:** NOAA Operational Model Archive and Distribution System  
**Base URL:** https://nomads.ncep.noaa.gov/  
**Fast Download Documentation:** https://nomads.ncep.noaa.gov/info.php?page=fastdownload

**Access Method:**
- HTTP random access with index files (.idx)
- Direct HTTPS downloads
- Perl scripts: `get_inv.pl`, `get_grib.pl`
- GRIB filter web application for subsetting

**Authentication:** None required (public HTTP/HTTPS)

**Expected GRIB2 File Structure:**
```
# GFS structure
http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.YYYYMMDD/HH/gfs.tHHz.pgrbfXX.grib2

# Example
http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20240101/00/gfs.t00z.pgrbf12.grib2
```

**Temporal Coverage:**
- Recent data: Up to approximately 1 month
- Older data transitions to NCEI archives
- Operational models retained for ~30 days

**Why High Priority for DRT=0:**
- Direct access to latest operational model runs
- Index files enable efficient subsetting
- GRIB filter tool can filter by specific parameters
- All major NOAA operational models available (GFS, GDAS, GFS Wave)

**Search Strategy:**
```bash
# 1. Use index file approach to inspect available data
INV_URL="http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20240101/00/gfs.t00z.pgrbf12.grib2.idx"
get_inv.pl $INV_URL | head -20

# 2. Use get_grib.pl for targeted downloads
# (Requires perl, curl with HTTP range support)
GRIB_URL="http://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20240101/00/gfs.t00z.pgrbf12.grib2"
get_inv.pl $INV_URL | grep "CONUS" | get_grib.pl $GRIB_URL conus_sample.grb

# 3. Use wgrib2 to scan for DRT values
wgrib2 conus_sample.grb | grep -E "DRT|drt"

# 4. Explore different model runs and forecast hours
# (Cycle through times and forecast lead times)
```

---

### Priority Tier 2: Secondary Sources

#### 4. NCEP Direct Products

**Source Name:** National Centers for Environmental Prediction Direct Products  
**Base URLs:**
- FTP: ftp://ftpprd.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
- HTTP: http://www.nco.ncep.noaa.gov/pmb/data/
- Products Page: https://www.nco.ncep.noaa.gov/pmb/products/gfs/

**Access Method:**
- Anonymous FTP access
- Direct HTTP downloads
- Standard FTP/HTTP clients (wget, curl)

**Authentication:** None required (anonymous FTP/public HTTP)

**Expected GRIB2 File Structure:**
```
# GFS FTP structure
gfs.YYYYMMDD/HH/gfs.tHHz.pgrb2.fXXX
gfs.t00z.pgrb2.0p25.f000  # Analysis
```

**Temporal Coverage:**
- Operational: Real-time, latest model runs
- Archive: Days to weeks
- Long-term: Transitions to NCEI

**Why Secondary Priority:**
- Latest operational data but shorter retention
- Less structured than NODD/NCEI
- No advanced subsetting tools (but file paths are predictable)

**Search Strategy:**
```bash
# 1. Use FTP to explore available data
ftp ftpprd.ncep.noaa.gov
cd /pub/data/nccf/com/gfs/prod/

# 2. Use wget/curl for direct downloads
wget ftp://ftpprd.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20240101/00/gfs.t00z.pgrb2.0p25.f000

# 3. Scan for DRT values
wgrib2 downloaded_file.grib2 | grep -E "DRT|drt"
```

---

#### 5. NOAA READY Archives

**Source Name:** NOAA Air Resources Laboratory READY Archives  
**Base URL:** https://www.ready.noaa.gov/archives.php

**Access Method:**
- Web portal interface
- Direct HTTP downloads
- Reprocessed GRIB format

**Authentication:** None required (public HTTP)

**Expected GRIB2 File Structure:**
- Reprocessed GRIB format (not raw operational output)
- Focus on meteorological data for atmospheric dispersion modeling
- Archive extends beyond NOMADS retention period

**Temporal Coverage:**
- Varies by model
- Focus on historical model output for dispersion modeling
- Longer archive than NOMADS but less structured

**Why Secondary Priority:**
- Reprocessed data (may have different DRT characteristics)
- Less comprehensive than primary sources
- Focus on dispersion modeling use cases

---

### Priority Tier 3: Specialized Sources

#### 6. NOAA Earthdata (NASA Partnership)

**Source Name:** NOAA Earthdata - NCEP GFS 0.25 Degree  
**Base URL:** https://access.earthdata.nasa.gov/collections/C1214110986-SCIOPS

**Access Method:**
- NASA Earthdata system
- API access
- Web portal

**Authentication:** Yes - Requires NASA account registration

**Expected GRIB2 File Structure:**
- Historical GFS 0.25 degree global forecast grids
- Model forecast runs at 00, 06, 12, 18 UTC daily
- Format preserved through NASA processing

**Temporal Coverage:**
- Historical GFS data
- Coverage extends beyond operational retention periods

**Why Lower Priority:**
- Authentication requirement complicates automated access
- NASA portal processing may modify original GRIB2 structure
- Focus on historical data rather than recent operational runs

---

## Comprehensive Search Strategy

### Phase 1: Rapid Discovery (Tier 1 Sources)

**Objective:** Quickly identify DRT=0 files from most promising sources

**Week 1-2: NODD/AWS Primary Search**
1. Script automated scanning of GFS bucket (`noaa-gfs-bdp-pds`)
   - Sample files across different forecast cycles
   - Check recent runs (last 30 days)
   - Focus on CONUS-friendly forecast hours
2. Scan NBM CONUS bucket (`noaa-nbm-grib2-pds`)
   - CONUS-focused model most likely to have regional DRT=0
   - Sample across diurnal cycle
3. Use wgrib2 to catalog DRT values found
4. Document file patterns that contain DRT=0

**Week 3-4: NCEI API Exploration**
1. Use REST API to query CONUS-specific datasets
2. Implement bounding box queries for regional filtering
3. Download sample GRIB2 files from API responses
4. Catalog DRT values in NCEI holdings

**Week 5-6: NOMADS Recent Data**
1. Use index file approach to sample recent operational runs
2. Leverage GRIB filter tool to explore parameter combinations
3. Focus on last 30 days of data (transitions to NCEI)
4. Cross-reference with NODD findings

### Phase 2: Extended Search (Tier 2 Sources)

**Objective:** Expand search to secondary sources if Phase 1 insufficient

1. NCEP FTP exploration of latest runs
2. READY archive for historical context
3. Cross-reference DRT=0 findings across all sources

### Phase 3: Specialized Sources (if needed)

**Objective:** Access specialized sources only if primary sources insufficient**

1. NASA Earthdata registration and access
2. Focus on historical DRT=0 patterns
3. Compare with operational source findings

---

## Technical Implementation Notes

### Tools Required

**For AWS/NODD:**
```bash
# AWS CLI configuration (anonymous access)
aws configure
# Enter dummy credentials, rely on --no-sign-request

# boto3 Python setup
pip install boto3
```

**For NCEI:**
```bash
# Standard curl/wget for REST API
curl "https://www.ncei.noaa.gov/access/services/data/v1?..."

# Python requests for programmatic access
pip install requests
```

**For NOMADS:**
```bash
# Perl scripts for index file processing
# Download from NOMADS documentation page
```

**For GRIB2 Inspection:**
```bash
# wgrib2 for DRT inspection
wgrib2 sample.grib2 | grep -E "DRT|drt"

# Python pygrib for programmatic access
pip install pygrib
```

### Common Search Pattern Across All Sources

1. **Discovery:** Use source-specific tools to identify available files
2. **Sampling:** Download representative files for DRT inspection
3. **Inspection:** Use wgrib2 to catalog DRT values
4. **Pattern Recognition:** Identify file naming patterns that correlate with DRT=0
5. **Systematic Scan:** Apply patterns across full temporal coverage

---

## Expected Outcomes

**High Confidence Sources:**
- NODD/AWS buckets should contain DRT=0 files in operational model data
- NCEI API should provide structured access to DRT=0 files
- NOMADS recent data should contain DRT=0 in current operational runs

**CONUS-Specific Expectations:**
- NBM CONUS data highly likely to contain regional DRT=0
- GFS global forecasts should include DRT=0 for CONUS region
- HRRR CONUS data should contain DRT=0

**Success Criteria:**
- Identification of at least 3 distinct DRT=0 file patterns
- Documentation of access methods for each source
- Prioritized list of most productive sources for continued scanning
- Working scripts for automated DRT=0 discovery

---

## Source Priority Ranking for DRT=0 Discovery

1. **NODD/AWS NBM** - CONUS-focused, recent data, cloud-optimized
2. **NODD/AWS GFS** - Global operational model, comprehensive coverage
3. **NCEI API** - Long-term archival, structured queries
4. **NOMADS** - Recent operational data, subsetting tools
5. **NCEP FTP** - Latest runs, shorter retention
6. **READY Archives** - Historical, reprocessed
7. **Earthdata** - Historical, authentication barrier

---

*Documentation compiled for bead bf-4mb7t: NOAA archive sources inventory and DRT=0 search strategy.*
