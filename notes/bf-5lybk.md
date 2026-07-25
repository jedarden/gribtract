# GFS Gaussian-grid Golden Output Generation

## Task
Generate golden JSON outputs for the GFS Gaussian-grid fixture using scripts/gen_golden.py.

## Execution

### Source File
- `tests/corpus/large/flx.2024011500.grib2` (10.9 MB)

### Command
```bash
python3 scripts/gen_golden.py tests/corpus/large/flx.2024011500.grib2 core_gaussian_gdt40 --output-dir tests/corpus/golden
```

### Result
- **Output file**: `tests/corpus/golden/core_gaussian_gdt40.json`
- **Size**: 361 MB
- **Messages**: 104 fields
- **Grid characteristics**:
  - Grid template: 40 (Gaussian grid)
  - Dimensions: 512 x 256
  - Data points: 131,072
  - Grid type: GFS Gaussian grid (GDT 40)

### Validation
- ✅ Valid JSON structure
- ✅ All required fields present (center, subcenter, parameter, forecast, level, grid, gdt_template, pdt_template, drt_template)
- ✅ Grid template correctly set to 40 (Gaussian grid)
- ✅ 104 messages successfully parsed from GRIB2 source

### Status
The golden output generation was successful. The generated file is identical to the existing golden file, indicating that the reference output is already correct and up to date.

## Technical Details

### Grid Characteristics (First Field)
- **NX**: 512
- **NY**: 256  
- **Total points**: 131,072
- **Latitude range**: -89.4629° to 89.4629°
- **Longitude range**: 0° to 359.297°
- **Scanning mode**: 0 (normal)
- **Shape of earth**: 6 (WGS84)

### Source Data
- **Center**: 7 (NCEP)
- **Subcenter**: 3
- **Discipline**: 0 (Meteorological)
- **Parameter category**: 5
- **Parameter number**: 3
- **Reference time**: 2024-01-15 00:00 UTC

## Conclusion
The GFS Gaussian-grid golden output has been successfully generated and validated. The existing golden file is correct and matches the output from the current gen_golden.py script.
