# wgrib2 Availability Verification (Bead bf-5ozc)

## Finding

**wgrib2 is NOT installed** on this system.

## Investigation Results

1. **wgrib2 binary check:**
   - `which wgrib2` → Not found
   - `dpkg -l | grep wgrib` → Not installed via apt
   - No wgrib tools found in `/usr/bin` or `/usr/local/bin`

2. **What IS available:**
   - **cfgrib** CLI tool at `/home/coding/.local/bin/cfgrib`
   - **ecCodes v2.46.0** (confirmed via `cfgrib selfcheck`)
   - Verification was done using eccodes, not wgrib2

3. **Verification file naming issue:**
   - Files `wgrib2_nam_verification.txt` and `wgrib2_hrrr_verification.txt` are **misnamed**
   - These were actually generated using cfgrib/eccodes (confirmed by git commit `9f878ff`)
   - The commit message says "complete GRIB2 file verification with eccodes"

## Project Context

The `README.md` states:
> "verified field-by-field against **eccodes/wgrib2**"

This indicates the project supports either tool as a reference. Currently, **eccodes (via cfgrib)** is the working reference on this system.

## Recommendation

If wgrib2 is specifically required:
1. Install via: `sudo apt-get install wgrib2`
2. Or build from source: https://www.cpc.ncep.noaa.gov/products/wgrib2/

However, **eccodes is already sufficient** for the project's verification needs.

## Tested Commands

```bash
# Check wgrib2
$ which wgrib2
# (not found)

# Check cfgrib/ecCodes
$ cfgrib selfcheck
Found: ecCodes v2.46.0.
Your system is ready.

# List available GRIB tools
$ ls *.grib2
hrrr_sample_20260703.grib2  nam_conus_20260703.grib2
```
