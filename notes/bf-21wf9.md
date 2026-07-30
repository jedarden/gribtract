# wgrib2 Installation Verification - bf-21wf9

## Installation Status
- ✅ **wgrib2 is installed and working**
- **Location:** `/home/coding/.local/bin/wgrib2`
- **Version:** v3.1.3 (October 2023)
- **Authors:** Wesley Ebisuzaki and contributors

## Test Results

Tested on sample GRIB2 file: `samples/grib2-noaa-gfs/gfs.20260724.t00z.pgrb2.1p00.f000`

### Basic Inventory Command
```bash
# List all records in the GRIB2 file (first 20 shown)
wgrib2 samples/grib2-noaa-gfs/gfs.20260724.t00z.pgrb2.1p00.f000 | head -20
```

**Output format:** `record_number:byte_offset:date:parameter:level:type:forecast_time`

Sample output:
```
1:0:d=2026072400:PRMSL:mean sea level:anl:
2:75204:d=2026072400:CLMR:1 hybrid level:anl:
3:87488:d=2026072400:ICMR:1 hybrid level:anl:
...
```

### Parameter Matching
```bash
# Match specific parameters (e.g., sea level pressure)
wgrib2 samples/grib2-noaa-gfs/gfs.20260724.t00z.pgrb2.1p00.f000 -match ':PRMSL:' -s
```

Output: `1:0:d=2026072400:PRMSL:mean sea level:anl:`

### Count Records
```bash
# Count all records in the file
wgrib2 samples/grib2-noaa-gfs/gfs.20260724.t00z.pgrb2.1p00.f000 -count
```

## Working Command Syntax

The basic wgrib2 command syntax is:
```bash
wgrib2 <grib2_file> [options]
```

Common options:
- No options: List all records with inventory
- `-match ':PARAMETER:'`: Match specific parameters
- `-s`: Short inventory format
- `-count`: Count total records
- `-text`: Output data as text
- `-bin`: Output data as binary
- `-nh`: No headers in data output

## Verification Complete

All acceptance criteria met:
- ✅ wgrib2 is installed and accessible in PATH
- ✅ wgrib2 successfully runs on GRIB2 test files
- ✅ Working commands documented in this file
