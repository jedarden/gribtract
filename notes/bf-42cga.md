# NOAA Ensemble GRIB2 Archive Sources Research

## Task Overview
Research and identify NOAA ensemble/statistical GRIB2 product archive sources for test fixture development.

## Sources Found

### 1. GEFS (Global Ensemble Forecast System) - AWS S3

**Primary Archive:** AWS S3 Public Bucket
- **Bucket Name:** `noaa-gefs-pds`
- **AWS Region:** `us-east-1`
- **ARN:** `arn:aws:s3:::noaa-gefs-pds`
- **Date Range:** 2017 to present
- **Direct Browse URL:** https://noaa-gefs-pds.s3.amazonaws.com/index.html

**Access Methods:**
```bash
# AWS CLI (no authentication required)
aws s3 ls s3://noaa-gefs-pds/ --no-sign-request
aws s3 cp s3://noaa-gefs-pds/[path] [local] --no-sign-request

# Direct HTTPS
https://noaa-gefs-pds.s3.amazonaws.com/[path-to-file]
```

**Directory Structure:**
```
gefs.YYYYMMDD/
├── HH/ (cycle: 00, 06, 12, 18)
    └── atmos/
        ├── pgrb2ap5/ (0.5° resolution - ~13-19 MB)
        ├── pgrb2bp5/ (0.5° resolution full fields - ~93-98 MB)
        └── pgrb2sp25/ (0.25° resolution - ~15-19 MB)
```

**File Naming Convention:**
- `gec00.tHHz.pgrb2a.0p50.fXXX` - Control member (c00)
- `gep01.tHHz.pgrb2a.0p50.fXXX` - Perturbed member 01 (p01-p30)
- `geavg.tHHz.pgrb2a.0p50.fXXX` - Ensemble mean (avg)
- `fXXX` - Forecast hour (f000, f003, f006, etc.)

**File Size Examples (from 2024-10-15):**
- pgrb2ap5: 13-19 MB per file (e.g., `gec00.t00z.pgrb2a.0p50.f000` = 14.2 MB)
- pgrb2sp25: 15-19 MB per file (e.g., `geavg.t00z.pgrb2s.0p25.f000` = 15.6 MB)
- pgrb2bp5: 93-98 MB per file (too large for test fixtures)

**Sample Download URLs:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
```

**Update Cycle:** 4 times daily (00, 06, 12, 18 UTC)
**Ensemble Members:** 31 members total (1 control + 30 perturbed)
**PDT Templates:** PDT 4.1 (individual ensemble forecasts), PDT 4.8 (clustering)

**Documentation:**
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [AWS Registry](https://registry.opendata.aws/noaa-gefs/)
- [Herbie Documentation](https://herbie.readthedocs.io/en/latest/gallery/noaa_models/gefs.html)

---

### 2. SREF (Short Range Ensemble Forecast) - NOMADS

**Primary Archive:** NCEP NOMADS
- **Base URL:** https://nomads.ncep.noaa.gov/
- **Product Page:** https://www.nco.ncep.noaa.gov/pmb/products/sref/
- **SREF GRIB Filter:** https://nomads.ncep.noaa.gov/gribfilter.php?ds=sref
- **HTTPS Access:** https://nomads.ncep.noaa.gov/pub/data/nccf/com/sref/prod/

**Available Datasets:**
1. **SREF CONUS (40km)** - 6-hourly cycles
2. **SREF CONUS (40km) Bias-Corrected** - 6-hourly cycles
3. **SREF North America (32km)** - 6-hourly cycles
4. **SREF North America (16km)** - 6-hourly cycles

**Update Cycle:** 4 times daily (03, 09, 15, 21 UTC)
**Forecast Hours:** 00, 03, 06, ... 87
**File Size:** ~4-5 MB per forecast hour file

**File Naming Convention:**
- `sref.tCCz.pgrb212.ctl.fXX.grib2` (ARW core, control)
- `sref.tCCz.pgrb212.nXX.fXX.grib2` (ensemble member)

**Note:** Data retention is limited (~30-60 days typically)

---

### 3. NAEFS (North American Ensemble Forecast System)

**Primary Archive:** NCEP
- **Product Page:** https://www.nco.ncep.noaa.gov/pmb/products/naefs/
- **NOMADS Access:** https://nomads.ncep.noaa.gov/
- **File Naming:** `naefs.tCCz.ge###`

**Features:**
- Multi-model ensemble (combines NCEP and MSC models)
- Public access via NOMADS
- Contains ensemble data in GRIB2 format

---

## Recommended Test Fixture Candidates

### Candidate 1: GEFS Control Member (pgrb2ap5)
**URL Example:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
```

**Characteristics:**
- Size: ~13-15 MB
- Resolution: 0.5°
- Contains PDT 4.1 messages (ensemble forecast template)
- Permanent archive (data available from 2017)
- No authentication required
- Publicly accessible via HTTPS

**Download Command:**
```bash
wget "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000" -O gefs_control_member.grib2
```

---

### Candidate 2: GEFS Perturbed Member (pgrb2ap5)
**URL Example:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000
```

**Characteristics:**
- Size: ~13-16 MB
- Resolution: 0.5°
- Contains PDT 4.1 messages (ensemble forecast template)
- Permanent archive (data available from 2017)
- No authentication required
- Publicly accessible via HTTPS

**Download Command:**
```bash
wget "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gep01.t00z.pgrb2a.0p50.f000" -O gefs_perturbed_member.grib2
```

---

### Candidate 3: GEFS Ensemble Mean (pgrb2sp25)
**URL Example:**
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20241015/00/atmos/pgrb2sp25/geavg.t00z.pgrb2s.0p25.f000
```

**Characteristics:**
- Size: ~15-19 MB
- Resolution: 0.25° (higher resolution)
- Contains statistical processing results (ensemble mean)
- Permanent archive
- No authentication required
- Publicly accessible via HTTPS

**Download Command:**
```bash
wget "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20241015/00/atmos/pgrb2sp25/geavg.t00z.pgrb2s.0p25.f000" -O gefs_ensemble_mean.grib2
```

---

## Key Resources

### Documentation
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
- [GRIB2 Table 4.1 - Parameter Category](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)
- [GRIB2 Table 4.8 - Clustering Method](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)

### Access Tools
- [NOMADS Fast Download Guide](https://nomads.ncep.noaa.gov/info.php?page=fastdownload)
- [NOMADS Main Portal](https://nomads.ncep.noaa.gov/)

### Historical Data
- **GEFS Reforecast (2000-2019):** `https://noaa-gefs-retrospective.s3.amazonaws.com/index.html`
- [Reforecast Documentation](https://noaa-gefs-retrospective.s3.amazonaws.com/Description_of_reforecast_data.pdf)

---

## Verification Status

**Sources:**
- ✅ GEFS AWS S3 bucket - Verified accessible, no authentication
- ✅ GEFS files contain PDT 4.1 and 4.8 templates (ensemble forecast products)
- ✅ File sizes within acceptable range (<50MB for most products)
- ✅ NOMADS provides public access to ensemble data

**Next Steps:**
1. Download sample files to verify PDT 4.1/4.8 message content
2. Validate GRIB2 decode capabilities with ensemble products
3. Test fixture integration with GEFS control member files

---

## Notes

- All sources are publicly accessible with no authentication required
- GEFS data is permanently archived (2017-present)
- SREF data has limited retention (~30-60 days)
- NAEFS combines multiple meteorological services (NCEP + MSC)
- AWS S3 access is recommended for permanent fixtures
- NOMADS is good for testing recent model runs

## Sources
- [NOAA GEFS AWS Registry](https://registry.opendata.aws/noaa-gefs/)
- [NCEP GEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/gens/)
- [NCEP SREF Products](https://www.nco.ncep.noaa.gov/pmb/products/sref/)
- [NCEP NAEFS Products](https://www.nco.ncep.noaa.gov/pmb/products/naefs/)
- [NOMADS Portal](https://nomads.ncep.noaa.gov/)
- [NCEP GRIB2 Documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
