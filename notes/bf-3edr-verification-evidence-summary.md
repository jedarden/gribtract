# NOAA GRIB2 Verification Evidence Summary

**Bead:** bf-3edr  
**Date:** 2026-07-03  
**Purpose:** Extract and compile all verification results from previous beads

---

## Overview

This document compiles all verification evidence related to NOAA GRIB2 files that was gathered across multiple beads in the gribtract workspace. The evidence confirms that the downloaded GRIB2 files use **GDT 3.30 (Lambert Conformal Conic projection)** with **DRT=3 (complex packing)**.

---

## Previous Beads with Verification Evidence

| Bead ID | Title | Status | Evidence Type |
|---------|-------|--------|---------------|
| bf-5ozc | Verify wgrib2 availability | completed | Tool availability check |
| bf-tnue | Inspect and document GDT with wgrib2 | in_progress | GDT 3.30 confirmation |
| bf-3ogm | Verify downloaded GRIB2 file validity | closed | File format validation |
| bf-45h5 | Download and verify GRIB2 file characteristics | open | Full file verification |
| bf-48jz | Document validated NOAA DRT=3 source file | in_progress | Source documentation |
| bf-5c42 | Browse NAM archive dates | in_progress | Archive navigation |
| bf-2rku | Download GRIB2 candidate file from NOAA source | closed | Download record |

---

## 1. Tool Verification (bf-5ozc)

### wgrib2 Availability
**Finding:** wgrib2 is **NOT installed** on this system.

### Alternative Tool: cfgrib/eccodes
**Confirmed:** ecCodes v2.46.0 is available via cfgrib.

```bash
$ cfgrib selfcheck
Found: ecCodes v2.46.0.
Your system is ready.
```

### Key Points
- The files `wgrib2_nam_verification.txt` and `wgrib2_hrrr_verification.txt` are misnamed
- They were actually generated using **cfgrib/eccodes**, not wgrib2
- Git commit `9f878ff` confirms: "complete GRIB2 file verification with eccodes"
- ecCodes is sufficient for all GRIB2 verification needs

---

## 2. GDT 3.30 Confirmation (bf-tnue)

### Primary Evidence Files

#### `docs/bf-tnue-gdt-inspection.txt`
```
Grid Definition Template (GDT): 30
Template designation: 3.30
Template Name: Lambert Conformal Conic
Grid Type: lambert
Centre: kwbc
Grid Dimensions:
  Ni (x-direction): 1799
  Nj (y-direction): 1059
```

### GDT 3.30 Specification
- **Template Number:** 30 (3.30 in GRIB2 notation)
- **Template Name:** Lambert Conformal Conic
- **Grid Type:** lambert
- **Data Source:** kwbc (NCEP/NCAR)
- **Grid Size:** 1799 × 1059 points

---

## 3. DRT=3 Verification Results

### NAM CONUS File (`nam_conus_20260703.grib2`)

**File Size:** 954,566,275 bytes (910 MB)  
**Messages:** 794 total

#### Verification Results
```
SUMMARY:
  Total messages: 794
  GDT 3.30 (Lambert Conformal): 794/794 (100.0%)
  DRT=3 (Complex Packing): 794/794 (100.0%)
```

**Sample Output (first 10 messages):**
```
Message   1: 3.30 ✓, DRT=3 ✓, param=260074, name=Pressure reduced to MSL
            grid_type=lambert, size=1799x1059
Message   2: 3.30 ✓, DRT=3 ✓, param=260389, name=Derived radar reflectivity
            grid_type=lambert, size=1799x1059
Message   3: 3.30 ✓, DRT=3 ✓, param=54, name=Pressure
            grid_type=lambert, size=1799x1059
Message   4: 3.30 ✓, DRT=3 ✓, param=260018, name=Cloud mixing ratio
            grid_type=lambert, size=1799x1059
Message   5: 3.30 ✓, DRT=3 ✓, param=260101, name=Cloud Ice
            grid_type=lambert, size=1799x1059
Message   6: 3.30 ✓, DRT=3 ✓, param=260020, name=Rain mixing ratio
            grid_type=lambert, size=1799x1059
Message   7: 3.30 ✓, DRT=3 ✓, param=260021, name=Snow mixing ratio
            grid_type=lambert, size=1799x1059
Message   8: 3.30 ✓, DRT=3 ✓, param=260389, name=Derived radar reflectivity
            grid_type=lambert, size=1799x1059
Message   9: 3.30 ✓, DRT=3 ✓, param=260017, name=Condensate
            grid_type=lambert, size=1799x1059
Message  10: 3.30 ✓, DRT=3 ✓, param=260040, name=Rime factor
            grid_type=lambert, size=1799x1059
```

### HRRR File (`hrrr_sample_20260703.grib2`)

**File Size:** 144,838,995 bytes (138 MB)  
**Messages:** 104 total

#### Verification Results (excerpt)
```
Message   1: 3.30 ✓, DRT=3 ✓, param=260390, name=Maximum/Composite radar reflectivity
            grid_type=lambert, size=1799x1059
Message   2: 3.30 ✓, DRT=3 ✓, param=0, name=unknown
            grid_type=lambert, size=1799x1059
Message   3: 3.30 ✓, DRT=3 ✓, param=0, name=unknown
            grid_type=lambert, size=1799x1059
Message   4: 3.30 ✓, DRT=3 ✓, param=260136, name=Vertically-integrated liquid
            grid_type=lambert, size=1799x1059
Message   5: 3.30 ✓, DRT=3 ✓, param=3020, name=Visibility
            grid_type=lambert, size=1799x1059
```

**Note:** HRRR also contains DRT=0 (simple packing) messages for certain parameters like:
- Time-maximum vorticity (param=237138)
- Hail (param=260027)
- Lightning (param=260391)
- 10m wind components (param=165, 166)
- Precipitation fields (param=3059, 228228)

---

## 4. File Download Records (bf-2rku)

### HRRR Download Details

| Attribute | Value |
|-----------|-------|
| **Filename** | hrrr.20260703.t00z.wrfsfcf00.grib2 |
| **File Size** | 139 MB (138,879,071 bytes) |
| **File Type** | GRIB2 |
| **Model** | HRRR (High-Resolution Rapid Refresh) |
| **Date** | 2026-07-03 |
| **Cycle** | 00z (analysis) |
| **Forecast Hour** | f00 (analysis) |
| **Product** | wrfsfc (surface fields) |

### Source URL Pattern
```
https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.YYYYMMDD/conus/hrrr.tCCz.wrfsfcfFF.grib2
```

### Verification Checklist
✅ File downloaded to samples/ directory  
✅ File size > 0 bytes (139 MB)  
✅ File has .grib2 extension  
✅ Download timestamp recorded (2026-07-03 09:21 UTC)  
✅ File verified as GRIB2 format using `file` command

---

## 5. Verification Methodology

### Tools Used
- **cfgrib** (Python GRIB reader built on eccodes)
- **ecCodes v2.46.0** (European Centre for Medium-Range Weather Forecasts)
- **file** command (Unix file type detector)

### Verification Steps
1. Downloaded GRIB2 files from NOAA archives
2. Verified file format using `file` command
3. Inspected grid/template information using cfgrib
4. Extracted GDT (Grid Definition Template) values
5. Confirmed DRT (Data Representation Template) values
6. Documented all findings in verification text files

---

## 6. Key Findings Summary

### GDT 3.30 Confirmation
- **CONFIRMED:** Both NAM CONUS and HRRR files use GDT 3.30
- **Projection:** Lambert Conformal Conic
- **Grid Size:** 1799 × 1059 points
- **Coverage:** CONUS (Continental United States)

### DRT=3 Confirmation
- **CONFIRMED:** NAM CONUS file uses DRT=3 for all 794 messages (100%)
- **CONFIRMED:** HRRR file uses DRT=3 for most messages (majority)
- **Note:** HRRR also contains DRT=0 (simple packing) for certain derived parameters

### Tool Availability
- **wgrib2:** NOT installed
- **ecCodes v2.46.0:** AVAILABLE via cfgrib
- **Verification:** All previous verification done with eccodes

---

## 7. Evidence Files

### Verification Output Files
1. `wgrib2_nam_verification.txt` - NAM CONUS GDT/DRT verification (794 messages)
2. `wgrib2_hrrr_verification.txt` - HRRR GDT/DRT verification (104+ messages)
3. `docs/bf-tnue-gdt-inspection.txt` - GDT 3.30 specification
4. `docs/hrrr_gdt_full_inspection.txt` - Complete HRRR GDT inspection
5. `samples/bf-2rku-download-record.md` - Download provenance

### Documentation Files
1. `notes/bf-5ozc.md` - Tool availability verification
2. `docs/gdt_inspection_bf-tnue.md` - GDT inspection details

---

## 8. Conclusions

1. **GDT 3.30 is confirmed** - Both NAM CONUS and HRRR files use Lambert Conformal Conic projection with template 3.30

2. **DRT=3 is confirmed** - Complex packing is the primary encoding for both files:
   - NAM CONUS: 100% of 794 messages use DRT=3
   - HRRR: Majority of messages use DRT=3

3. **Tooling is verified** - ecCodes v2.46.0 is available and functional for GRIB2 verification

4. **File provenance is documented** - Download records confirm files came from official NOAA archives

5. **Evidence is comprehensive** - Multiple verification files provide cross-validation of findings

---

**Compiled by:** bf-3edr (automated evidence extraction)  
**Date:** 2026-07-03  
**Status:** Complete
