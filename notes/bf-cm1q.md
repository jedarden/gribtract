# Golden JSON Format and Eccodes CLI Capabilities Research

## Golden JSON Structure

### Top-Level Structure
```json
{
  "fixture_id": "string",
  "_provenance": "string describing data origin",
  "fields": [ ... ]
}
```

### Field Object Structure
Each field in the `fields` array contains:

#### Core Identification
- `center` (integer): Center code (e.g., 7 = kwbc/NCEP)
- `subcenter` (integer): Subcenter code (typically 0)
- `parameter` (object):
  - `discipline` (integer): GRIB2 discipline (0=meteorological)
  - `category` (integer): Parameter category
  - `number` (integer): Parameter number

#### Forecast Timing
- `forecast` (object):
  - `reference_time` (object):
    - `year`, `month`, `day`, `hour`, `minute`, `second` (integers)
    - `significance` (integer): 0=analysis, 1=forecast
  - `time_range_unit` (integer): Indicator of unit for forecast time
  - `forecast_offset` (integer): Forecast time in specified units

#### Vertical Level
- `level` (object):
  - `type1` (integer): First fixed surface type
  - `scale_factor1` (integer): Scale factor for first surface
  - `scaled_value1` (integer): Scaled value for first surface
  - `type2` (integer): Second fixed surface type (255=missing)
  - `scale_factor2` (integer): Scale factor for second surface
  - `scaled_value2` (integer): Scaled value for second surface

#### Ensemble Information
- `ensemble` (null or object):
  - `member_type` (integer): Type of ensemble member
  - `number` (integer): Member number

#### Grid Definition
- `grid` (object):
  - `template` (integer): Grid definition template number
  - `num_data_points` (integer): Total number of data points
  - `nx`, `ny` (integers): Grid dimensions
  - `lat_first`, `lon_first` (floats): First grid point coordinates
  - `lat_last`, `lon_last` (floats): Last grid point coordinates
  - `di`, `dj` (floats): Grid increments in degrees
  - `scanning_mode` (integer): Bit flags for scanning direction
  - `resolution_flags` (integer): Resolution component flags
  - `shape_of_earth` (integer): Earth shape code

#### Data Values
- `values` (object):
  - `Dense` (array of floats): All data values in row-major order

#### Template Numbers
- `gdt_template` (integer): Grid Definition Template number
- `pdt_template` (integer): Product Definition Template number
- `drt_template` (integer): Data Representation Template number

#### Packing Information
- `packing` (object):
  - `reference_value` (float): Reference value (R)
  - `binary_scale_factor` (integer): Binary scale factor (E)
  - `decimal_scale_factor` (integer): Decimal scale factor (D)
  - `bits_per_value` (integer): Number of bits per value (N)
  - `original_field_type` (integer): Original field type

## Available Eccodes CLI Tools

### Primary Tools
1. **grib_ls**: List GRIB message content
   - `-j`: JSON output
   - `-p`: Specify keys to print
   - `-P`: Add keys to default list
   - `-w`: Where clause for filtering
   - Output format: `{"messages": [{"key": value, ...}]}`

2. **grib_dump**: Dump complete GRIB message
   - `-j`: JSON mode
   - `-d`: Print all data values (works in text mode, not JSON)
   - `-O`: Octet mode (WMO documentation style)
   - `-D`: Debug mode
   - `-p`: Specify keys to dump
   - Output format: Array of `{"key": "...", "value": ...}` objects

3. **grib_get**: Get specific key values
   - `-p`: Keys to retrieve
   - `-F`: C-style format for floats
   - Similar to grib_ls but fails on errors
   - **Cannot extract array-type keys** (like "values")

4. **grib_filter**: Advanced filtering/processing
   - Can handle array-type data
   - Useful for data extraction

5. **codes_info**: System information
   - `-v`: Version
   - `-d`: Definitions path
   - `-s`: Samples path

### Additional Tools
- `grib_copy`: Copy GRIB messages
- `grib_set`: Modify key values in GRIB files
- `grib_compare`: Compare two GRIB files
- `grib_count`: Count messages
- `grib_to_netcdf`: Convert to NetCDF format
- `grib2ppm`: Convert to PPM image
- `grib_get_data`: Extract data at lat/lon points
- `grib_histogram`: Compute histogram of values
- `grib_index_build`: Build indexing file

## Key Finding: CLI Limitations

The eccodes CLI tools **cannot directly produce the golden JSON format**:

1. **grib_dump -j -d** does NOT include data values in JSON output
2. **grib_get** cannot extract array-type keys (error: "Passed array is too small")
3. **grib_ls -j** provides metadata but not data values
4. CLI output format is flat key-value, not nested structure

## Recommended Approach

To generate golden JSON files, use **eccodes Python bindings** (as done in `scripts/gen_golden.py`):

```python
import eccodes as ecc

# Open GRIB file
with open('file.grib2', 'rb') as f:
    msg = ecc.codes_grib_handle_from_file(f)
    
    # Extract all needed fields
    centre = ecc.codes_get(msg, 'centre')
    values = ecc.codes_get_values(msg)
    # ... etc
```

The CLI tools are useful for:
- Quick inspection of GRIB files
- Verifying specific metadata values
- Debugging GRIB structure
- Converting between formats

But for structured output matching the golden JSON schema, the Python bindings are required.
