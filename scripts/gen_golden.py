#!/usr/bin/env python3
"""gen_golden.py — Generate gribtract golden reference JSON using eccodes CLI tools.

This script uses grib_dump (eccodes CLI) to extract GRIB2 metadata and data values,
then transforms it into the golden JSON format expected by the differential test suite.

Usage:
    python3 scripts/gen_golden.py <grib2_file> <fixture_id> [--output-dir DIR]

Output:
    tests/corpus/golden/<fixture_id>.json

Requirements:
    - eccodes CLI tools (grib_dump) must be installed and in PATH
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path
from datetime import datetime


def run_grib_dump(grib2_path):
    """Run grib_dump with JSON output and return parsed data.

    Args:
        grib2_path: Path to GRIB2 file

    Returns:
        Parsed JSON data from grib_dump

    Raises:
        subprocess.CalledProcessError: If grib_dump fails
        ValueError: If grib_dump output is not valid JSON
    """
    try:
        # Run grib_dump with JSON and data values output
        result = subprocess.run(
            ['grib_dump', '-j', '-d', str(grib2_path)],
            capture_output=True,
            text=True,
            check=True
        )

        # Parse JSON output
        return json.loads(result.stdout)

    except subprocess.CalledProcessError as e:
        print(f"ERROR: grib_dump failed: {e}", file=sys.stderr)
        if e.stderr:
            print(f"grib_dump stderr: {e.stderr}", file=sys.stderr)
        raise

    except json.JSONDecodeError as e:
        print(f"ERROR: Failed to parse grib_dump JSON output: {e}", file=sys.stderr)
        print(f"grib_dump stdout: {result.stdout[:500]}...", file=sys.stderr)
        raise ValueError(f"Invalid JSON from grib_dump: {e}")


# Section-5 packing keys whose values we fetch from `grib_ls -p` because
# `grib_dump -j` (JSON mode) omits them entirely for simple/complex packing.
PACKING_KEYS = [
    'referenceValue',
    'binaryScaleFactor',
    'decimalScaleFactor',
    'numberOfBitsContainingEachPackedValue',
    'typeOfOriginalFieldValues',
]

# Keys whose values are integers (the rest — referenceValue — is a float).
_PACKING_INT_KEYS = {
    'binaryScaleFactor',
    'decimalScaleFactor',
    'numberOfBitsContainingEachPackedValue',
    'typeOfOriginalFieldValues',
}

# Grid and reference-time keys whose raw GRIB values are integers.  The raw
# microdegree coordinate keys preserve the precision that grib_dump's
# human-oriented degree keys may round away.
_GRIB_LS_INT_KEYS = {
    'Ni',
    'Nj',
    'Nx',
    'Ny',
    'latitudeOfFirstGridPoint',
    'longitudeOfFirstGridPoint',
    'latitudeOfLastGridPoint',
    'longitudeOfLastGridPoint',
    'iDirectionIncrement',
    'jDirectionIncrement',
    'resolutionAndComponentFlags',
    'scanningMode',
    'second',
}

GRID_KEYS = [
    'Ni',
    'Nj',
    'Nx',
    'Ny',
    'latitudeOfFirstGridPoint',
    'longitudeOfFirstGridPoint',
    'latitudeOfLastGridPoint',
    'longitudeOfLastGridPoint',
    'iDirectionIncrement',
    'jDirectionIncrement',
    'resolutionAndComponentFlags',
    'scanningMode',
    'second',
]


def _coerce_grib_ls_value(key, raw):
    """Coerce a grib_ls token for packing or grid metadata."""
    if key in PACKING_KEYS or key in _GRIB_LS_INT_KEYS:
        if key in PACKING_KEYS and key not in _PACKING_INT_KEYS:
            return float(raw)
        return int(float(raw))
    raise ValueError(f'unsupported grib_ls key: {key}')


def run_grib_ls_keys(grib2_path, keys):
    """Fetch per-message key values from `grib_ls -p`.

    grib_dump's JSON mode omits several Section-5 packing header keys
    (referenceValue, binary/decimalScaleFactor, numberOfBitsContainingEachPackedValue,
    typeOfOriginalFieldValues). grib_ls surfaces them, so this returns one dict per
    GRIB message (values coerced to int/float) for merging into the dump data.

    Returns:
        List of dicts, one per message. Empty list if grib_ls is unavailable,
        fails, or returns no value rows.
    """
    try:
        result = subprocess.run(
            ['grib_ls', '-p', ','.join(keys), str(grib2_path)],
            capture_output=True,
            text=True,
            check=True,
        )
    except (subprocess.CalledProcessError, FileNotFoundError):
        return []

    lines = result.stdout.splitlines()
    # Drop the leading filename echo line.
    if lines and Path(grib2_path).name in lines[0]:
        lines = lines[1:]

    # Keep only non-empty lines that are not summary lines ("N of M messages ...").
    rows = []
    for line in lines:
        stripped = line.strip()
        if not stripped or 'messages' in stripped:
            continue
        rows.append(stripped.split())

    if len(rows) < 2:
        return []

    # rows[0] is the header (key names); rows[1:] are per-message value rows.
    per_message = []
    for raw_values in rows[1:]:
        entry = {}
        for key, raw in zip(keys, raw_values):
            if raw == 'MISSING':
                continue
            try:
                entry[key] = _coerce_grib_ls_value(key, raw)
            except (TypeError, ValueError):
                continue
        per_message.append(entry)
    return per_message


def key_to_dict(items, key):
    """Convert a list of {key, value} items to a dictionary.

    Args:
        items: List of dictionaries with 'key' and 'value' keys
        key: The key to look up

    Returns:
        Value if found, None otherwise
    """
    for item in items:
        if item.get('key') == key:
            return item.get('value')
    return None


def extract_scanning_mode(data):
    """Extract scanning mode flags from eccodes data dict.

    eccodes provides individual bits, we need to combine them into a single byte.

    Args:
        data: Dictionary containing keys from grib_dump (already converted)

    Returns:
        Integer scanning mode byte
    """
    # Extract individual scanning mode flags
    i_scans_negatively = data.get('iScansNegatively', 0) or 0
    j_scans_positively = data.get('jScansPositively', 0) or 0
    j_points_consecutive = data.get('jPointsAreConsecutive', 0) or 0
    alternative_scanning = data.get('alternativeRowScanning', 0) or 0

    # Combine into the raw scanning-mode byte (Table 3.4).  The ecCodes
    # boolean keys correspond to bits 7..4, not the low four bits.
    scanning_mode = (
        ((i_scans_negatively & 1) << 7) |
        ((j_scans_positively & 1) << 6) |
        ((j_points_consecutive & 1) << 5) |
        ((alternative_scanning & 1) << 4)
    )
    return scanning_mode


def grid_coordinate(data, raw_key, degree_key):
    """Return a raw GRIB microdegree coordinate, falling back to degrees."""
    raw = data.get(raw_key)
    if raw is not None:
        return raw / 1_000_000.0
    return data.get(degree_key)


def grid_increment(data, raw_key, degree_key):
    """Return a raw GRIB microdegree increment, falling back to degrees."""
    raw = data.get(raw_key)
    if raw is not None:
        return raw / 1_000_000.0
    return data.get(degree_key)


def parse_data_date(date_int):
    """Parse dataDate integer (YYYYMMDD) into year, month, day components.

    Args:
        date_int: Date as integer (YYYYMMDD format)

    Returns:
        Tuple of (year, month, day)
    """
    date_str = str(int(date_int))
    if len(date_str) != 8:
        return (0, 0, 0)

    year = int(date_str[0:4])
    month = int(date_str[4:6])
    day = int(date_str[6:8])

    return (year, month, day)


def parse_data_time(time_int, second=0):
    """Parse dataTime integer (HHMM) into hour, minute, second components.

    Args:
        time_int: Time as integer (HHMM format)

    Returns:
        Tuple of (hour, minute, second)
    """
    time_str = str(int(time_int)).zfill(4)
    hour = int(time_str[0:2])
    minute = int(time_str[2:4])
    second = second or 0

    return (hour, minute, second)


def transform_message_to_golden(message_data, packing_extra=None):
    """Transform a single message from grib_dump format to golden format.

    Args:
        message_data: List of {key, value} items from grib_dump
        packing_extra: Optional dict of packing and raw grid keys (sourced
            from `grib_ls -p`) to merge in, since grib_dump -j may omit or
            round them.

    Returns:
        Dictionary in golden JSON format
    """
    # Create a lookup dict for easier access
    data = {item['key']: item['value'] for item in message_data}

    # Merge packing and raw grid keys sourced from grib_ls -p.
    if packing_extra:
        data.update(packing_extra)

    # Extract basic fields
    center = data.get('centre', 0)
    subcenter = data.get('subCentre', 0)

    # Parameter info
    discipline = data.get('discipline', 255)
    param_category = data.get('parameterCategory', 255)
    param_number = data.get('parameterNumber', 255)

    # Parse reference time from dataDate and dataTime
    date_int = data.get('dataDate', 0)
    time_int = data.get('dataTime', 0)
    significance = data.get('significanceOfReferenceTime', 0)

    year, month, day = parse_data_date(date_int)
    hour, minute, second = parse_data_time(time_int, data.get('second', 0))

    # Forecast info
    time_range_unit = data.get('indicatorOfUnitForForecastTime', 1)
    forecast_offset = data.get('forecastTime', 0)

    # Level info
    level_type1 = data.get('typeOfFirstFixedSurface', 255) or 255
    level_scale1 = data.get('scaleFactorOfFirstFixedSurface', 0) or 0
    level_scaled1 = data.get('scaledValueOfFirstFixedSurface', 0) or 0
    level_type2 = data.get('typeOfSecondFixedSurface', 255) or 255
    level_scale2 = data.get('scaleFactorOfSecondFixedSurface', 0) or 0
    level_scaled2 = data.get('scaledValueOfSecondFixedSurface', 0) or 0
    # A missing second surface has reserved scale/value octets in some real
    # products.  The decoder's semantic representation (and eccodes' logical
    # field value) uses zeroes when type=255.
    if level_type2 == 255:
        level_scale2 = 0
        level_scaled2 = 0

    # Grid info
    gdt_template = data.get('gridDefinitionTemplateNumber', 0)
    num_data_points = data.get('numberOfDataPoints')
    # Lat/lon grids use Ni/Nj; projected grids use Nx/Ny.
    nx = data.get('Ni') or data.get('Nx')
    ny = data.get('Nj') or data.get('Ny')
    lat_first = grid_coordinate(
        data, 'latitudeOfFirstGridPoint', 'latitudeOfFirstGridPointInDegrees'
    )
    lon_first = grid_coordinate(
        data, 'longitudeOfFirstGridPoint', 'longitudeOfFirstGridPointInDegrees'
    )
    lat_last = grid_coordinate(
        data, 'latitudeOfLastGridPoint', 'latitudeOfLastGridPointInDegrees'
    )
    lon_last = grid_coordinate(
        data, 'longitudeOfLastGridPoint', 'longitudeOfLastGridPointInDegrees'
    )
    di = grid_increment(data, 'iDirectionIncrement', 'iDirectionIncrementInDegrees')
    dj = grid_increment(data, 'jDirectionIncrement', 'jDirectionIncrementInDegrees')
    shape_of_earth = data.get('shapeOfTheEarth', 6)

    # Extract scanning mode
    scanning_mode = data.get('scanningMode')
    if scanning_mode is None:
        scanning_mode = extract_scanning_mode(data)

    # Product definition template
    pdt_template = data.get('productDefinitionTemplateNumber', 0)

    # Extract ensemble info for PDT 4.1
    ensemble_info = None
    if pdt_template == 1:
        # PDT 4.1 has ensemble forecast info
        # eccodes might not provide all ensemble fields, check what's available
        ensemble_type = data.get('typeOfEnsembleForecast')
        perturbation_number = data.get('perturbationNumber')
        if ensemble_type is not None or perturbation_number is not None:
            ensemble_info = {
                'member_type': ensemble_type if ensemble_type is not None else 0,
                'number': perturbation_number if perturbation_number is not None else 0
            }

    # Data representation template
    packing_type = data.get('packingType', 'grid_simple')

    # Map packing type to DRT template number
    # grid_simple -> 0, grid_complex -> 2, etc.
    drt_map = {
        'grid_simple': 0,
        'grid_complex': 2,
        'grid_jpeg': 40,
        'grid_png': 41,
        'grid_second_simple': 1,
    }
    drt_template = drt_map.get(packing_type, 0)

    # Packing info (eccodes may not provide all fields)
    packing_info = {
        'reference_value': data.get('referenceValue', 0.0),
        'binary_scale_factor': data.get('binaryScaleFactor', 0),
        'decimal_scale_factor': data.get('decimalScaleFactor', 0),
        'bits_per_value': data.get('numberOfBitsContainingEachPackedValue', 0),
        'original_field_type': data.get('typeOfOriginalFieldValues', 0)
    }

    # Values
    values = data.get('values', [])
    # ecCodes occasionally serializes a bitmap-missing point at the end of a
    # partially-filled bitmap as its internal undefined sentinel rather than
    # JSON null.  Treat that sentinel as missing so it compares to the bitmap
    # mask decoded by gribtract.
    values = [None if value == -1e100 else value for value in values]
    values_info = {'Dense': values} if values else None

    # Build golden format message
    golden_message = {
        'center': center,
        'subcenter': subcenter,
        'parameter': {
            'discipline': discipline,
            'category': param_category,
            'number': param_number
        },
        'forecast': {
            'reference_time': {
                'year': year,
                'month': month,
                'day': day,
                'hour': hour,
                'minute': minute,
                'second': second,
                'significance': significance
            },
            'time_range_unit': time_range_unit,
            'forecast_offset': forecast_offset
        },
        'level': {
            'type1': level_type1,
            'scale_factor1': level_scale1,
            'scaled_value1': level_scaled1,
            'type2': level_type2,
            'scale_factor2': level_scale2,
            'scaled_value2': level_scaled2
        },
        'ensemble': ensemble_info,
        'grid': {
            'template': gdt_template,
            'num_data_points': num_data_points,
            'nx': nx,
            'ny': ny,
            'lat_first': lat_first,
            'lon_first': lon_first,
            'lat_last': lat_last,
            'lon_last': lon_last,
            'di': di,
            'dj': dj,
            'scanning_mode': scanning_mode,
            'resolution_flags': data.get('resolutionAndComponentFlags', 48),
            'shape_of_earth': shape_of_earth
        },
        'values': values_info,
        'gdt_template': gdt_template,
        'pdt_template': pdt_template,
        'drt_template': drt_template,
        'packing': packing_info
    }

    return golden_message


def gen_golden_eccodes(grib2_path, fixture_id, output_dir):
    """Generate golden JSON using eccodes CLI tools (grib_dump).

    Args:
        grib2_path: Path to input GRIB2 file
        fixture_id: Fixture ID for output filename
        output_dir: Directory for output JSON

    Raises:
        SystemExit: On file access or parsing errors
    """
    grib2_path = Path(grib2_path)

    if not grib2_path.exists():
        print(f"ERROR: GRIB2 file not found: {grib2_path}", file=sys.stderr)
        sys.exit(1)

    # Run grib_dump and get JSON
    try:
        dump_data = run_grib_dump(grib2_path)
    except (subprocess.CalledProcessError, ValueError) as e:
        print(f"ERROR: Failed to extract data from {grib2_path}: {e}", file=sys.stderr)
        sys.exit(1)

    # Check for messages
    if 'messages' not in dump_data or not dump_data['messages']:
        print(f"ERROR: No messages found in grib_dump output", file=sys.stderr)
        sys.exit(1)

    # Fetch packing and raw grid metadata that grib_dump -j omits or rounds.
    metadata_keys = PACKING_KEYS + GRID_KEYS
    metadata_rows = run_grib_ls_keys(grib2_path, metadata_keys)

    # Transform each message to golden format
    messages = []
    for idx, message_data in enumerate(dump_data['messages']):
        extra = metadata_rows[idx] if idx < len(metadata_rows) else None
        golden_message = transform_message_to_golden(message_data, extra)
        messages.append(golden_message)

    # Build golden file structure
    golden = {
        'fixture_id': fixture_id,
        '_provenance': (
            f'Generated by scripts/gen_golden.py from {grib2_path.name}'
            ' using eccodes CLI tools: grib_dump -j -d for metadata/values,'
            ' grib_ls -p for packing and raw grid keys omitted or rounded by'
            ' grib_dump -j.'
        ),
        'fields': messages,
        'parser_version': 'eccodes_cli_1.0'
    }

    # Write output
    out_path = Path(output_dir) / f'{fixture_id}.json'
    out_path.parent.mkdir(parents=True, exist_ok=True)

    with open(out_path, 'w') as f:
        # These real NOAA fields contain millions of values; compact JSON
        # keeps the checked-in golden references practical without changing
        # their schema or data.
        json.dump(golden, f, separators=(',', ':'))

    print(f'Written: {out_path}  ({len(messages)} message(s))')


def main():
    parser = argparse.ArgumentParser(
        description='Generate gribtract golden reference JSON from a GRIB2 file using eccodes CLI tools'
    )
    parser.add_argument('grib2_file', help='Input GRIB2 file')
    parser.add_argument('fixture_id', help='Fixture ID (becomes the output filename)')
    parser.add_argument(
        '--output-dir',
        default='tests/corpus/golden',
        help='Directory for the output JSON (default: tests/corpus/golden)',
    )
    args = parser.parse_args()

    gen_golden_eccodes(args.grib2_file, args.fixture_id, args.output_dir)


if __name__ == '__main__':
    main()
