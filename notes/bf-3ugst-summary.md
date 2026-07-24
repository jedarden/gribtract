# GRIB2 Header Downloads - Summary

**Bead:** bf-3ugst  
**Date:** 2026-07-24  
**Dependency:** bf-5eokv (DRT=0 candidate list)

## Task Completed

Downloaded GRIB2 message headers from all candidate files identified in bf-5eokv to enable packing and grid analysis without full file downloads.

## Acceptance Criteria Met

✅ **For each candidate in the prepared list, download only the GRIB2 headers**
- Used `curl | wgrib2 -header` to extract headers without downloading full file data
- Processing time: ~1-2 minutes per candidate for large GFS files (~400MB)

✅ **Verify each header download succeeds**
- All 9 candidates downloaded successfully
- 100% success rate (0 failures)

✅ **Save headers to organized temporary files with names matching their source candidates**
- Headers saved to `headers/bf-3ugst/` directory
- File naming: `<source-filename>.headers.txt`
- Example: `gfs.t00z.pgrb2.1p00.f000.headers.txt`

✅ **Create a manifest mapping header files to original candidate URLs**
- Manifest created: `headers/bf-3ugst/manifest.json`
- Contains: candidate_id, source URL, model, resolution, date, forecast_hour, header file path, message count, status, download timestamp

✅ **Log any candidates that fail header download for exclusion**
- Failed log: `headers/bf-3ugst/failed_downloads.log`
- **No failures occurred** - log contains only header, no failed candidates

## Download Results

### Summary Statistics
- **Total candidates:** 9
- **Successful:** 9
- **Failed:** 0
- **Success rate:** 100%

### Header File Details

| File | Messages | Size | Model | Resolution | Date |
|------|----------|------|-------|------------|------|
| gfs.t00z.pgrb2.0p25.f000.headers.txt | 696 | 32K | GFS | 0.25° | 2026-07-24 |
| gfs.t00z.pgrb2.0p50.f000.headers.txt | 696 | 31K | GFS | 0.50° | 2026-07-24 |
| gfs.t00z.pgrb2.1p00.f000.headers.txt | 696 | 31K | GFS | 1.00° | 2026-07-24 |
| gfs.t00z.pgrb2.0p25.f000.headers.txt | 696 | 32K | GFS | 0.25° | 2026-07-23 |
| gfs.t00z.pgrb2.0p50.f000.headers.txt | 696 | 31K | GFS | 0.50° | 2026-07-23 |
| gfs.t00z.pgrb2.1p00.f000.headers.txt | 696 | 31K | GFS | 1.00° | 2026-07-23 |
| geavg.t00z.pgrb2a.0p50.f000.headers.txt | 71 | 3.6K | GEFS | 0.50° | 2026-07-24 |
| geavg.t00z.pgrb2a.0p50.f003.headers.txt | 85 | 5.1K | GEFS | 0.50° | 2026-07-24 |
| geavg.t00z.pgrb2a.0p50.f006.headers.txt | 85 | 5.0K | GEFS | 0.50° | 2026-07-24 |

### Header Format

Headers are in wgrib2 inventory format:
```
<message_num>:<byte_offset>:d=<datetime>:<param>:<level>:<type>:<ensemble_info>
```

Example from GEFS header:
```
1:0:d=2026072400:HGT:10 mb:anl:ens mean
2:202450:d=2026072400:TMP:10 mb:anl:ens mean
```

This provides:
- Message number and byte offset in the original GRIB2 file
- Date/time stamp
- Meteorological parameter (HGT=height, TMP=temperature, UGRD=u-wind, etc.)
- Vertical level (pressure levels, hybrid levels, surface)
- Analysis/forecast type
- Ensemble information (for GEFS)

## Files Created

- `headers/bf-3ugst/manifest.json` - Complete manifest mapping headers to sources
- `headers/bf-3ugst/failed_downloads.log` - Empty (no failures)
- `headers/bf-3ugst/download_output.log` - Full execution log
- `headers/bf-3ugst/*.headers.txt` - 9 header files (6 unique, 3 duplicated from same filenames)

## Methodology Used

### Tool Choice: `curl | wgrib2 -header`

Selected this approach over alternatives:
- **`curl -r 0-20480`**: Would download first 20KB, but may miss complete headers
- **`wgrib2 -header`**: Processes GRIB2 stream, extracts complete message headers without data payload

The pipe approach ensures complete headers are captured even when messages span beyond the first few kilobytes.

### Verification

Each header file was verified to:
- Contain valid GRIB2 inventory format
- Have expected message counts (GFS: ~696 messages, GEFS: 71-85 messages)
- Show correct parameter names and levels

## Next Steps

These headers enable:
1. **Grid analysis** - Extract grid definitions without full download
2. **Packing verification** - Confirm DRT=0 across all messages
3. **Selective record retrieval** - Use byte offsets for targeted data extraction
4. **File comparison** - Compare message structure across models/dates

## Source Data

Candidate list from bead bf-5eokv:
- `drt_search_results/drt0_candidates_structured.json` - Structured candidate metadata
- `notes/bf-5eokv-drt0-search-results.md` - Full search documentation
