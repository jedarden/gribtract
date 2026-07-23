# wgrib2 Flags for DRT/Packing Information

Research findings for displaying Data Representation Template (DRT) and packing information using wgrib2.

## Key Flags

### `-Sec5` - Data Representation Section
**Purpose**: Shows Section 5 contents which contains the Data Representation Template (DRT)

**Syntax**:
```bash
wgrib2 file.grib2 -Sec5
```

**Output format**:
```
1:0:Sec5 len=47 #defined data points=9 Data Repr. Template=5.2
```

**What it shows**:
- Section 5 length in bytes
- Number of defined data points
- Data Representation Template number (e.g., 5.2 = DRT 2, 5.40 = DRT 40, 5.41 = DRT 41)

### `-packing` - Packing Mode
**Purpose**: Shows the packing mode/type

**Syntax**:
```bash
wgrib2 file.grib2 -packing        # Basic packing info
wgrib2 file.grib2 -v -packing     # Verbose packing details
```

**Output format (basic)**:
```
1:0:packing=Grid point data - complex packing,c1
```

**Output format (verbose -v)**:
```
1:0:packing=Grid point data - complex packing,c1 val=(100+i*2^0)*10^0, ref=0..255 (#bits=8) group width bits=4 #groups=1
```

**What it shows**:
- Packing type (e.g., "Grid point data - complex packing", "JPEG 2000 code stream format", "Portable Network Graphics (PNG)")
- Scaling/encoding details
- Reference values and bit sizes
- Group information

### `-0xSec 5` - Hex Dump of Section 5
**Purpose**: Hexadecimal dump of Section 5 for raw DRT data

**Syntax**:
```bash
wgrib2 file.grib2 -0xSec 5
```

**Output format**:
```
1:0:Sec5(1..47)=0x0000002f0500000009000242c800000000000008000000000000000000000000000001000400000000010000000904
```

### Other Related Flags

#### `-pdt` - Product Definition Table
```bash
wgrib2 file.grib2 -pdt
# Output: 1:0:pdt=0
```

#### `-gdt` - Grid Definition Template
```bash
wgrib2 file.grib2 -gdt
# Note: Requires g2clib installation
```

#### `-Sec3` - Grid Definition Section
```bash
wgrib2 file.grib2 -Sec3
# Shows GDS (Grid Definition Section) contents
```

#### `-Sec4` - Product Definition Section
```bash
wgrib2 file.grib2 -Sec4
# Shows PDS (Product Definition Section) contents
```

#### `-precision` - Precision of packing
```bash
wgrib2 file.grib2 -precision
# Shows packing precision information
```

## Common DRT Values

The Data Representation Template number indicates the packing scheme:

| DRT Number | Name/Description |
|------------|------------------|
| 5.0 | Grid point data - simple packing |
| 5.2 | Grid point data - complex packing |
| 5.40 | Grid point data - JPEG 2000 code stream format |
| 5.41 | Grid point data - Portable Network Graphics (PNG) |
| 5.40000 | Grid point data - CCSDS (Consultative Committee for Space Data Systems) |

## Examples

### Check DRT on multiple files:
```bash
wgrib2 file1.grib2 -Sec5
wgrib2 file2.grib2 -Sec5
wgrib2 file3.grib2 -Sec5
```

### Detailed packing analysis:
```bash
wgrib2 file.grib2 -v -packing
```

### Combine multiple inventory options:
```bash
wgrib2 file.grib2 -Sec5 -packing -pdt
```

## Usage Notes

- All flags are inventory-type options (inv) - they can be combined
- Use `-v` flag for verbose output with more details
- Section flags (Sec3, Sec4, Sec5, Sec6) show raw GRIB2 section contents
- Template numbers follow format X.Y where X is the section number and Y is the template number within that section

## Binary Location

The wgrib2 binary is located at:
```
./grib2/wgrib2/wgrib2
```

Version: wgrib2 v3.1.3 10/2023
