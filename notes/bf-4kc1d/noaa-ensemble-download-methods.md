# NOAA Ensemble Archive Download Methods

## Summary

Successfully identified and tested **public HTTP/HTTPS access** to NOAA GEFS (Global Ensemble Forecast System) archives via **Amazon S3**.

## Working Download Method: Amazon S3 Public Bucket

### Primary Endpoint: `https://noaa-gefs-pds.s3.amazonaws.com/`

**Status:** ✅ **WORKING** - Public access, no authentication required

### File Structure

```
gefs.YYYYMMDD/HH/TYPE/RESOLUTION/MEMBER.FILE
```

**Components:**
- `YYYYMMDD`: Forecast run date (e.g., `20260723`)
- `HH`: Run cycle hour (00, 06, 12, 18)
- `TYPE`: `atmos` (atmospheric data)
- `RESOLUTION`: 
  - `pgrb2ap5` - 0.5° resolution (pgrb2a field set)
  - `pgrb2bp5` - 0.5° resolution (pgrb2b field set)
  - `pgrb2sp5` - 0.5° resolution (pgrb2s field set)
  - `pgrb2ap25` - 0.25° resolution (pgrb2a field set)
  - `pgrb2bp25` - 0.25° resolution (pgrb2b field set)
  - `pgrb2ap25` - 0.25° resolution (pgrb2s field set)

**Member Codes:**
- `gec00` - Control member (c00)
- `gec01` through `gec30` - Perturbation members (p01-p30)
- `geavg` - Ensemble mean
- `gespr` - Ensemble spread
- `gep01` through `gep30` - Probability fields

**File Format:**
```
MEMBER.tHHz.FIELD.RESOLUTION.fXXX[.idx]
```
- `HH`: Run cycle (00, 06, 12, 18)
- `FIELD`: `pgrb2a`, `pgrb2b`, `pgrb2s` (different variable sets)
- `RESOLUTION`: `0p50` (0.5°), `0p25` (0.25°)
- `XXX`: Forecast hour (000, 003, 006, ..., 384)
- `.idx`: Index file for subsetting

### Download Examples

#### 1. **List Available Files**
```bash
# List all files for a specific date and cycle
curl -s "https://noaa-gefs-pds.s3.amazonaws.com/?list-type=2&prefix=gefs.20260723/00/atmos/pgrb2ap5/" \
  | grep -o "<Key>[^<]*</Key>"
```

#### 2. **Download Index File** (for subsetting)
```bash
# Index file shows byte offsets for each variable in the GRIB2 file
curl -o geavg.idx \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000.idx"

# Index format: byte_offset:date:variable:level:type:extra_info
# Example: 1:0:d=2026072300:HGT:10 mb:anl:ENS=low-res ctl
```

#### 3. **Download Complete GRIB2 File**
```bash
# Full file download (~13MB for 0.5° resolution)
curl -o gefs_control_f000.grib2 \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000"
```

#### 4. **Byte-Range Download** (for subsetting)
```bash
# Download specific byte range (useful for extracting individual fields)
# This example downloads the first 1KB
curl -r 0-1023 -o partial.grib2 \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000"

# The server supports Accept-Ranges: bytes
```

#### 5. **Download Ensemble Mean**
```bash
curl -o gefs_ensemble_mean_f000.grib2 \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"
```

#### 6. **Download Specific Perturbation Member**
```bash
# Perturbation member 1 (p01 → gec01)
curl -o gefs_pert1_f000.grib2 \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/gec01.t00z.pgrb2a.0p50.f000"
```

### Available Forecast Hours

**0.5° resolution:** 000, 003, 006, 009, 012, ..., 240, 264, 288, 312, 336, 360, 384 (3-hourly steps)

**0.25° resolution:** 000, 003, 006, ..., 120 (3-hourly steps through 5 days)

### Alternative Access Methods

#### NOMADS (Real-time data only, limited retention)
**Endpoint:** `https://nomads.ncep.noaa.gov/`

**Status:** ⚠️ **LIMITED** - Only recent data available (expiring after ~10 days)

**Notes:** 
- NOMADS filter API returns 403 Forbidden for GEFS
- Better suited for GFS and other models
- For historical GEFS data, use S3 bucket

#### FTP Access
**Endpoint:** `ftp://ftp.ncep.noaa.gov/pub/data/nccf/com/gens/prod/`

**Status:** ❌ **NOT WORKING** - Connection timeouts, likely deprecated

**Recommendation:** Use HTTP via S3 instead

#### NCAR RDA (Research Data Archive)
**Endpoint:** `https://rda.ucar.edu/datasets/ds113.1/`

**Status:** ⚠️ **REQUIRES AUTH** - Requires registration and authentication

**Notes:**
- Contains historical GEFS data
- May require API key or OAuth
- Redirects to gdex.ucar.edu
- Only use if S3 doesn't cover required date range

### Data Formats

#### GRIB2 (.grib2, .grb, no extension)
- Primary format for model output
- Can be read by wgrib2, degrib, cfgrib, pygrib, etc.
- Binary format, requires specialized tools

#### Index Files (.idx)
- Text format, one line per GRIB message
- Format: `record_number:byte_offset:date:variable:level:forecast_type:extra_info`
- Used for byte-range subsetting

#### NetCDF (.nc)
- Alternative format for some data
- Files in `atmos/init/` directory
- Better suited for some applications

### Rate Limits & Restrictions

**Amazon S3 Public Bucket:**
- **No authentication required** ✅
- **No documented rate limits** (standard S3 limits apply)
- **Geographic availability:** Global via HTTPS
- **Concurrent connections:** Supported (use connection pooling for bulk downloads)

**Data Retention:**
- **S3 bucket:** Appears to hold extensive historical data (tested back to 2017)
- **Real-time vs. archive:** Same endpoint for both
- **Data availability:** Most recent cycles appear within hours of model run

### Recommended Download Strategy

1. **For single files:** Use `curl` with HTTP
2. **For subsetting:** Download `.idx` file first, parse for byte offsets, use byte-range requests
3. **For bulk downloads:** Use HTTP with connection pooling, consider AWS SDK for parallel downloads
4. **For processing:** Consider downloading `.idx` files to determine required fields before downloading full GRIB2 files

### Example: Field Subsetting Workflow

```bash
# 1. Get the index file
curl -s "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000.idx" > index.txt

# 2. Parse for desired field (e.g., TMP at 2m)
# Index line format: N:offset:date:var:level:type:extra
# Example: 2:191634:d=2026072300:TMP:2 m:anl:ENS=low-res ctl

# 3. Extract byte range for specific field
# Field 2 starts at offset 191634
# Field 3 starts at offset 320323
# So field 2 spans bytes 191634-320322

curl -r 191634-320322 -o tmp_2m_field.grib2 \
  "https://noaa-gefs-pds.s3.amazonaws.com/gefs.20260723/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000"
```

### Python Example

```python
import requests

# Base URL for NOAA GEFS public S3 bucket
BASE_URL = "https://noaa-gefs-pds.s3.amazonaws.com"

# Download ensemble mean for specific date/time/forecast hour
date = "20260723"
cycle = "00"  # 00Z run
resolution = "pgrb2ap5"  # 0.5°, pgrb2a field set
member = "geavg"  # ensemble mean
fhour = "f000"  # analysis

url = f"{BASE_URL}/gefs.{date}/{cycle}/atmos/{resolution}/{member}.t{cycle}z.pgrb2a.0p50.{fhour}"

response = requests.get(url, stream=True)
with open("gefs_ensemble_mean.grib2", "wb") as f:
    for chunk in response.iter_content(chunk_size=8192):
        f.write(chunk)
```

## Notes

- **GRIB2 format verification:** Files start with "GRIB" magic bytes (tested)
- **Byte-range support:** Server supports `Accept-Ranges: bytes` header
- **Server-side encryption:** Files stored with AES256 encryption on S3
- **Last-Modified header:** Available for cache validation
- **ETag:** Available for integrity checking

## References

- **Product documentation:** https://www.ncei.noaa.gov/products/weather-climate-models/global-ensemble-forecast
- **NOMADS:** https://nomads.ncep.noaa.gov/
- **GRIB2 documentation:** https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/

## Testing Status

✅ HTTP/HTTPS via Amazon S3 (public bucket)
✅ GRIB2 file download
✅ Index file access
✅ Byte-range requests
✅ Multiple resolutions (0.25°, 0.5°)
✅ Multiple field sets (pgrb2a, pgrb2b, pgrb2s)
✅ Ensemble mean and control members
✅ Historical data (tested 2017-2026)

❌ FTP access (timeouts, likely deprecated)
❌ NOMADS filter API for GEFS (403 Forbidden)
⚠️ NCAR RDA (requires authentication)
