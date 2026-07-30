# Bead bf-1r7k3: NOAA CONUS DRT=0 File URL Research

## Task
Identify NOAA GRIB archive URL for a DRT=0 (simple packing) file that covers CONUS (Continental US) domain.

## Acceptance Criteria
- [x] NOAA archive URL identified for a GRIB2 file covering CONUS
- [x] URL is publicly accessible (no authentication required)
- [?] File uses simple packing (DRT=0) - requires verification with wgrib2
- [x] Coverage includes CONUS spatial domain

## Identified URL
**Primary Recommendation:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.20260724/nam.t00z.awphys00.tm00.grib2
```

**URL Pattern:**
```
https://nomads.ncep.noaa.gov/pub/data/nccf/com/nam/prod/nam.YYYYMMDD/nam.tCCz.awphysFF.tm00.grib2
```
Where:
- `YYYYMMDD`: Date directory
- `CC`: Model cycle (00, 06, 12, 18 UTC)
- `FF`: Forecast hour (00-84)

## Product Details
- **Model**: NAM (North American Mesoscale)
- **Product**: awphys (CONUS Upper Air data)
- **Grid**: AWIPS Grid #218 - 12 km Lambert Conformal Conic
- **Domain**: CONUS (Continental United States)
- **Format**: GRIB2

## Verification Status
✅ **Publicly Accessible**: File download confirmed (WebFetch successfully accessed the URL)
✅ **CONUS Coverage**: Confirmed via NOMADS documentation for awphys product on Grid #218
⚠️ **DRT=0 Verification**: Requires inspection with wgrib2 tool

## Recommended Follow-up Verification
To confirm DRT=0 simple packing, run:
```bash
wgrib2 nam.t00z.awphys00.tm00.grib2 -packing
```

This will display the Data Representation Template (DRT) used. Expected output for DRT=0 should show:
- `DRT 5.0` or "simple packing"
- Not "complex packing" (DRT 5.2), "JPEG2000" (DRT 5.40), or other complex methods

## Alternative Data Sources Investigated
- **HRRR**: Uses JPEG2000/complex packing (not DRT=0)
- **GFS**: Available at `/pub/data/nccf/com/gfs/prod/`
- **GDAS**: Available via NOMADS and AWS Open Data Registry

## References
- [NCEP NAM Products](https://www.nco.ncep.noaa.gov/pmb/products/nam/)
- [NOMADS Main Portal](https://nomads.ncep.noaa.gov/)
- [GRIB2 Table 5.0 - Simple Packing](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml)
- [NOMADS NAM Description](https://nomads.ncep.noaa.gov/get_ds_descr.php?file=WRF_NMM_txt.html)

## Notes
While the search confirmed public accessibility and CONUS coverage, the specific Data Representation Template (DRT) value cannot be confirmed without inspecting the actual GRIB2 file with tools like wgrib2 or eccodes. The task requires verification that the file uses DRT=0 simple packing rather than complex packing (DRT 5.2), JPEG2000 (DRT 5.40), or other encoding methods.
