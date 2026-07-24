# Bead bf-4uu3h: Candidate List Extraction from bf-5eokv

## Task Completed
Extracted candidate list from completed bead bf-5eokv for systematic verification.

## Acceptance Criteria Status
✅ Located the candidate list output from bead bf-5eokv
✅ Parsed and extracted all candidate file URLs/paths into a structured format
✅ Verified the list is complete and matches the search criteria from bf-5eokv
✅ Saved the prepared candidate list to a working file for downstream beads
✅ Documented the total count of candidates to process

## Files Extracted
From bead bf-5eokv:
- `drt_search_results/drt0_candidates.txt` (6 top candidates)
- `drt_search_results/drt0_candidates_full.txt` (9 total candidates)
- `notes/bf-5eokv-drt0-search-results.md` (comprehensive documentation)

## Output Files Created
- `drt_search_results/drt0_candidates_structured.json` - Structured JSON with full metadata
- `drt_search_results/candidates_bf-4uu3h.txt` - Working copy for downstream beads

## Candidate Count Summary
- **Total candidates**: 9 files
- **GFS models**: 6 candidates
  - GFS 0.25° (highest resolution): 2 files
  - GFS 0.50° (medium resolution): 2 files  
  - GFS 1.0° (lowest resolution): 2 files
- **GEFS models**: 3 candidates
  - GEFS ensemble mean at forecast hours 000, 003, 006

## Search Criteria Verification
- ✅ DRT=0 (simple packing) - confirmed via wgrib2
- ✅ CONUS coverage - all global models include CONUS region
- ✅ GRIB2 format - verified
- ✅ Full URLs documented - complete

## Archive Sources
1. **NOMADS** - GFS candidates
   - Base: https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/
   
2. **AWS S3** - GEFS candidates  
   - Bucket: noaa-gefs-pds

## Recommendations for Downstream Beads
- Start with smallest file (GFS 1.0°, 41MB) for testing
- Use structured JSON for programmatic processing
- Reference comprehensive documentation in notes/bf-5eokv-drt0-search-results.md

## Dependencies
- bf-5eokv (completed) - provided candidate search results
- bf-4yv5k (completed) - provided archive structure research
