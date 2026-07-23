# wgrib2 DRT Inspection Flags Reference

**Task:** bf-56zhx - Research and document wgrib2 DRT inspection flags

## Primary DRT Inspection Flags

### `-Sec5` - Data Representation Template (DRT) Information
**Purpose:** Display Section 5 values including the Data Representation Template number

**Usage:**
```bash
wgrib2 <file.grib2> -Sec5
```

**Output Format:**
```
<message_num>:<byte_offset>:Sec5 len=<length> #defined data points=<count> Data Repr. Template=5.<DRT_number>
```

**Examples:**

**DRT=0 (Simple packing):**
```bash
$ wgrib2 gfs_anl_t2m_5x5.grib2 -Sec5
1:0:Sec5 len=21 #defined data points=25 Data Repr. Template=5.0
```

**DRT=3 (Complex packing with spatial differencing):**
```bash
$ wgrib2 nam_awip12_20250115_t00z_f00.grib2 -Sec5 | head -3
1:0:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
2:240117:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
3:481603:Sec5 len=49 #defined data points=262792 Data Repr. Template=5.3
```

**DRT=40 (JPEG2000 compression):**
```bash
$ wgrib2 drt40_j2k_3x2.grib2 -Sec5
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.40
```

**DRT=41 (PNG compression):**
```bash
$ wgrib2 drt41_png_3x2.grib2 -Sec5
1:0:Sec5 len=21 #defined data points=6 Data Repr. Template=5.41
```

---

## Related DRT Information Flags

### `-packing` - Packing Mode Description
**Purpose:** Display the packing method/description in human-readable format

**Usage:**
```bash
wgrib2 <file.grib2> -packing
```

**Output Format:**
```
<message_num>:<byte_offset>:packing=<description>,<flags>
```

**Examples:**

**Simple packing:**
```bash
$ wgrib2 gfs_anl_t2m_5x5.grib2 -packing
1:0:packing=Grid point data - simple packing,s
```

**Complex packing with spatial differencing:**
```bash
$ wgrib2 nam_awip12_20250115_t00z_f00.grib2 -packing | head -3
1:0:packing=Grid point data - complex packing and spatial differencing,c3
2:240117:packing=Grid point data - complex packing and spatial differencing,c3
3:481603:packing=Grid point data - complex packing and spatial differencing,c3
```

**JPEG2000 compression:**
```bash
$ wgrib2 drt40_j2k_3x2.grib2 -packing
1:0:packing=Grid point data - JPEG 2000 code stream format,j
```

**PNG compression:**
```bash
$ wgrib2 drt41_png_3x2.grib2 -packing
1:0:packing=Grid point data - Portable Network Graphics (PNG),_
```

---

### `-precision` - Packing Precision Information
**Purpose:** Display the encoding precision (binary scale factor, decimal scale, reference value)

**Usage:**
```bash
wgrib2 <file.grib2> -precision
```

**Output Format:**
```
<message_num>:<byte_offset>:encode i*2^<binary_scale>*10^<decimal_scale>
```

**Examples:**

**Simple packing (no scaling):**
```bash
$ wgrib2 gfs_anl_t2m_5x5.grib2 -precision
1:0:encode i*2^0*10^0
```

**Complex packing with scaling:**
```bash
$ wgrib2 nam_awip12_20250115_t00z_f00.grib2 -precision | head -3
1:0:encode i*2^4*10^-2
2:240117:encode i*2^3*10^-1
3:481603:encode i*2^0*10^-5
```

---

### `-scale` - Scale Factors for Packing
**Purpose:** Display scale factors used in packing

**Usage:**
```bash
wgrib2 <file.grib2> -scale
```

---

### `-V` - Verbose Diagnostic Output
**Purpose:** Display comprehensive diagnostic information (less DRT-specific, more general)

**Usage:**
```bash
wgrib2 <file.grib2> -V
```

**Note:** While verbose, this does NOT explicitly show the DRT template number. Use `-Sec5` for specific DRT information.

---

## Common DRT Template Numbers

| DRT | Description | Common Usage |
|-----|-------------|---------------|
| 5.0 | Simple packing | Basic meteorological data, analysis files |
| 5.2 | Complex packing | Some NWP model output |
| 5.3 | Complex packing + spatial differencing | NAM, HRRR, many NWP models |
| 5.40 | JPEG2000 compression | High-resolution satellite/radar data |
| 5.41 | PNG compression | Alternative compression for gridded data |
| 5.40000 | JPEG2000 with lossless | Some satellite products |

---

## Practical Usage Patterns

### Quick DRT Check (Single Message)
```bash
wgrib2 <file.grib2> -Sec5
```

### Quick DRT Check (Multiple Messages with Summary)
```bash
wgrib2 <file.grib2> -Sec5 | head -10
```

### Human-readable Packing Description
```bash
wgrib2 <file.grib2> -packing
```

### Complete DRT Profile (Template + Description + Precision)
```bash
wgrib2 <file.grib2> -Sec5 -packing -precision
```

---

## File Structure Context

In GRIB2 files:
- **Section 0:** Indicator Section (identification)
- **Section 1:** Identification Section (metadata)
- **Section 2:** Local Use Section (optional)
- **Section 3:** Grid Definition Section (GDT)
- **Section 4:** Product Definition Section (PDT)
- **Section 5:** Data Representation Section (DRT) ← **TARGET**
- **Section 6:** Bit-map Section
- **Section 7:** Data Section (actual values)
- **Section 8:** End Section

The `-Sec5` flag specifically targets **Section 5** which contains the Data Representation Template information.

---

## Verification

All commands tested and verified with:
- wgrib2 v3.1.3 (installed at `/home/coding/.local/bin/wgrib2`)
- Test files in `/home/coding/gribtract/tests/corpus/small/`
- Sample files in `/home/coding/gribtract/samples/`

**Test Results:**
- ✅ `-Sec5` shows exact DRT template number
- ✅ `-packing` shows human-readable packing description  
- ✅ `-precision` shows encoding scale factors
- ✅ All flags work on single and multi-message GRIB2 files

---

## Reference

**wgrib2 documentation:** http://www.cpc.ncep.noaa.gov/products/wesley/wgrib2/

**GRIB2 specification:** WMO FM 92 GRIB Edition 2

**Task completed:** 2026-07-23
**Bead ID:** bf-56zhx
