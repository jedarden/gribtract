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

    # Combine into scanning mode byte (per GRIB2 spec)
    # Bit 0: iScansNegatively
    # Bit 1: jScansPositively
    # Bit 2: jPointsAreConsecutive
    # Bit 3: alternativeRowScanning
    scanning_mode = (
        (i_scans_negatively & 1) |
        ((j_scans_positively & 1) << 1) |
        ((j_points_consecutive & 1) << 2) |
        ((alternative_scanning & 1) << 3)
    )
    return scanning_mode


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


def parse_data_time(time_int):
    """Parse dataTime integer (HHMM) into hour, minute, second components.

    Args:
        time_int: Time as integer (HHMM format)

    Returns:
        Tuple of (hour, minute, second)
    """
    time_str = str(int(time_int)).zfill(4)
    hour = int(time_str[0:2])
    minute = int(time_str[2:4])
    second = 0

    return (hour, minute, second)


def transform_message_to_golden(message_data):
    """Transform a single message from grib_dump format to golden format.

    Args:
        message_data: List of {key, value} items from grib_dump

    Returns:
        Dictionary in golden JSON format
    """
    # Create a lookup dict for easier access
    data = {item['key']: item['value'] for item in message_data}

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
    hour, minute, second = parse_data_time(time_int)

    # Forecast info
    time_range_unit = data.get('indicatorOfUnitForForecastTime', 1)
    forecast_offset = data.get('forecastTime', 0)

    # Level info
    level_type1 = data.get('typeOfFirstFixedSurface', 255)
    level_scale1 = data.get('scaleFactorOfFirstFixedSurface', 0)
    level_scaled1 = data.get('scaledValueOfFirstFixedSurface', 0)
    level_type2 = data.get('typeOfSecondFixedSurface', 255)
    level_scale2 = data.get('scaleFactorOfSecondFixedSurface', 0)
    level_scaled2 = data.get('scaledValueOfSecondFixedSurface', 0)

    # Grid info
    gdt_template = data.get('gridDefinitionTemplateNumber', 0)
    num_data_points = data.get('numberOfDataPoints')
    nx = data.get('Ni')
    ny = data.get('Nj')
    lat_first = data.get('latitudeOfFirstGridPointInDegrees')
    lon_first = data.get('longitudeOfFirstGridPointInDegrees')
    lat_last = data.get('latitudeOfLastGridPointInDegrees')
    lon_last = data.get('longitudeOfLastGridPointInDegrees')
    di = data.get('iDirectionIncrementInDegrees')
    dj = data.get('jDirectionIncrementInDegrees')
    shape_of_earth = data.get('shapeOfTheEarth', 6)

    # Extract scanning mode
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
            'resolution_flags': 48,  # Default from eccodes
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

    # Transform each message to golden format
    messages = []
    for message_data in dump_data['messages']:
        golden_message = transform_message_to_golden(message_data)
        messages.append(golden_message)

    # Build golden file structure
    golden = {
        'fixture_id': fixture_id,
        '_provenance': (
            f'Generated by scripts/gen_golden.py from {grib2_path.name}'
            ' using eccodes CLI tools (grib_dump -j -d).'
        ),
        'fields': messages,
        'parser_version': 'eccodes_cli_1.0'
    }

    # Write output
    out_path = Path(output_dir) / f'{fixture_id}.json'
    out_path.parent.mkdir(parents=True, exist_ok=True)

    with open(out_path, 'w') as f:
        json.dump(golden, f, indent=4)

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
