# GRIB2 Technical Approach: CLI vs Python Bindings

## Finding: eccodes Python bindings NOT available

The eccodes Python bindings cannot be used in this environment:
- `import eccodes` fails with `ModuleNotFoundError: No module named 'eccodes'`
- `pip`/`pip3` is not available in the environment
- The bindings are not present in the nix profile

**Recommendation: Use CLI tools approach**

## Available CLI Tools

The following eccodes/grib tools are available via `/home/coding/.nix-profile/bin/`:

| Tool | Purpose |
|------|---------|
| `grib_dump` | Full GRIB message dump with JSON output option |
| `grib_get` | Extract specific keys from GRIB messages |
| `grib_ls` | List metadata for all messages in a file |
| `grib_count` | Count messages in a GRIB file |
| `grib_get_data` | Extract data values from GRIB messages |
| `grib_copy` | Copy/filter GRIB messages |
| `grib_set` | Modify keys in GRIB messages |
| `grib_filter` | Filter messages based on key constraints |
| `codes_info` | Display eccodes configuration |

## CLI Approach: grib_dump (JSON mode)

### Example usage:

```bash
# Dump entire file as JSON
grib_dump -j ./samples/nam_awip12_20250115_t00z_f00.grib2
```

**Output structure:**
```json
{
  "messages": [
    [
      {"key": "discipline", "value": 0},
      {"key": "editionNumber", "value": 2},
      {"key": "centre", "value": 7},
      {"key": "dataDate", "value": 20250115},
      {"key": "numberOfDataPoints", "value": 262792},
      {"key": "Ni", "value": 614},
      {"key": "Nj", "value": 428},
      ...
    ],
    ...
  ]
}
```

**Python parsing example:**
```python
import subprocess
import json

def get_grib_metadata(grib_path: str) -> dict:
    """Extract GRIB metadata using grib_dump CLI."""
    result = subprocess.run(
        ['grib_dump', '-j', grib_path],
        capture_output=True,
        text=True,
        check=True
    )
    return json.loads(result.stdout)

# Usage
metadata = get_grib_metadata('./samples/nam_awip12_20250115_t00z_f00.grib2')
print(f"Messages: {len(metadata['messages'])}")
```

## CLI Approach: grib_get (simple key extraction)

### Example usage:

```bash
# Extract specific keys for all messages
grib_get -p name,level,typeOfLevel,centre,dataDate ./samples/nam_awip12_20250115_t00z_f00.grib2
```

**Output (space-separated, one line per message):**
```
Pressure reduced to MSL 0 meanSea kwbc 20250115
Pressure 1 hybrid kwbc 20250115
Rain mixing ratio 1 hybrid kwbc 20250115
...
```

**Python parsing example:**
```python
import subprocess

def list_grib_fields(grib_path: str) -> list[dict]:
    """List all GRIB fields using grib_get CLI."""
    result = subprocess.run(
        ['grib_get', '-p', 'name,level,typeOfLevel,centre,dataDate', grib_path],
        capture_output=True,
        text=True,
        check=True
    )
    
    fields = []
    for line in result.stdout.strip().split('\n'):
        name, level, level_type, centre, date = line.split()
        fields.append({
            'name': name,
            'level': int(level),
            'typeOfLevel': level_type,
            'centre': centre,
            'dataDate': int(date)
        })
    return fields

# Usage
fields = list_grib_fields('./samples/nam_awip12_20250115_t00z_f00.grib2')
print(f"Total fields: {len(fields)}")
```

## CLI Approach: grib_ls (tabular metadata)

### Example usage:

```bash
grib_ls ./samples/nam_awip12_20250115_t00z_f00.grib2
```

**Output (tabular format):**
```
edition      centre       date         dataType     gridType     stepRange    typeOfLevel  level        shortName    packingType  
2            kwbc         20250115     fc           lambert      0            meanSea      0            prmsl        grid_complex_spatial_differencing 
2            kwbc         20250115     fc           lambert      0            hybrid       1            pres         grid_complex_spatial_differencing 
...
```

## CLI Approach: grib_count

```bash
grib_count ./samples/nam_awip12_20250115_t00z_f00.grib2
# Output: 186
```

## Common Keys for GRIB2

Useful keys to extract with `grib_get -p`:

| Key | Description | Example |
|-----|-------------|---------|
| `name` | Parameter name | "Temperature" |
| `shortName` | CF short name | "t" |
| `units` | Units | "K" |
| `level` | Level value | 850 |
| `typeOfLevel` | Level type | "isobaricInhPa" |
| `dataDate` | Analysis date | 20250115 |
| `dataTime` | Analysis time | 0 |
| `centre` | Originating centre | "kwbc" |
| `editionNumber` | GRIB edition | 2 |
| `numberOfDataPoints` | Grid size | 262792 |
| `Ni`, `Nj` | Grid dimensions | 614, 428 |
| `gridType` | Grid type | "lambert" |

## Conclusion

**Use CLI tools.** The eccodes Python bindings are not available in this environment. The CLI approach is:
- Fully functional with JSON output support
- Parseable from any language via subprocess
- Well-documented at https://confluence.ecmwf.int/display/ECC

## Test Results

Test file: `./samples/nam_awip12_20250115_t00z_f00.grib2`
- Messages: 186
- Edition: GRIB2
- Centre: kwbc (NCEP)
- Grid: Lambert Conformal (614x428)
- Date: 2025-01-15 00Z
