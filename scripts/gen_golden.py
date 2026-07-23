#!/usr/bin/env python3
"""gen_golden.py — Generate gribtract golden reference files.

This script supports both eccodes Python bindings and CLI tools.
Python bindings are preferred when available; falls back to CLI tools otherwise.

Usage:
    python3 scripts/gen_golden.py <grib2_file> <fixture_id> [--output-dir DIR]

Output:
    tests/corpus/golden/<fixture_id>.json
"""

import argparse
import json
import sys
import subprocess
from pathlib import Path

# Try to import eccodes Python bindings
try:
    import eccodes
    ECCODES_AVAILABLE = True
except ImportError:
    ECCODES_AVAILABLE = False

# Check if CLI tools are available
def check_cli_tools():
    """Check if eccodes CLI tools are available."""
    try:
        # Test with a simple metadata query instead of --help
        result = subprocess.run(
            ["grib_get", "-p", "editionNumber", "/dev/null"],
            capture_output=True,
            timeout=2
        )
        # Command may fail on /dev/null but that's OK - we just want to see if the binary exists
        return True
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False

CLI_TOOLS_AVAILABLE = check_cli_tools()

if not ECCODES_AVAILABLE and not CLI_TOOLS_AVAILABLE:
    print(
        "ERROR: No eccodes installation found. "
        "Install either: pip install eccodes OR eccodes CLI tools (grib_dump, grib_get)",
        file=sys.stderr
    )
    sys.exit(1)


# ============================================================================
# Python bindings implementation (preferred)
# ============================================================================

def _get(msg, key, default=None):
    """Retrieve a key from an eccodes message, returning default on error."""
    try:
        return eccodes.codes_get(msg, key)
    except eccodes.KeyValueNotFoundError:
        return default


def decode_message_python(msg):
    """Extract one field dict from an open eccodes message handle using Python bindings."""
    edition = _get(msg, "edition")
    if edition != 2:
        return None

    center    = _get(msg, "centre", 0)
    subcenter = _get(msg, "subCentre", 0)

    discipline = _get(msg, "discipline", 0)
    category   = _get(msg, "parameterCategory", 0)
    number     = _get(msg, "parameterNumber", 0)

    year        = _get(msg, "year", 0)
    month       = _get(msg, "month", 0)
    day         = _get(msg, "day", 0)
    hour        = _get(msg, "hour", 0)
    minute      = _get(msg, "minute", 0)
    second      = _get(msg, "second", 0)
    significance = _get(msg, "significanceOfReferenceTime", 0)

    time_range_unit = _get(msg, "indicatorOfUnitOfTimeRange", 1)
    forecast_offset = _get(msg, "forecastTime", 0)

    type1         = _get(msg, "typeOfFirstFixedSurface", 255)
    scale_factor1 = _get(msg, "scaleFactorOfFirstFixedSurface", 0)
    scaled_value1 = _get(msg, "scaledValueOfFirstFixedSurface", 0)
    type2         = _get(msg, "typeOfSecondFixedSurface", 255)
    scale_factor2 = _get(msg, "scaleFactorOfSecondFixedSurface", 0)
    scaled_value2 = _get(msg, "scaledValueOfSecondFixedSurface", 0)

    pdt = _get(msg, "productDefinitionTemplateNumber", 0)
    ensemble = None
    if pdt in (1, 11):
        ensemble = {
            "member_type": _get(msg, "typeOfEnsembleForecast", 0),
            "number": _get(msg, "perturbationNumber", 0),
        }

    gdt             = _get(msg, "gridDefinitionTemplateNumber", 0)
    num_data_points = _get(msg, "numberOfDataPoints", 0)
    nx              = _get(msg, "Ni", 0)
    ny              = _get(msg, "Nj", 0)
    lat_first       = _get(msg, "latitudeOfFirstGridPointInDegrees", 0.0)
    lon_first       = _get(msg, "longitudeOfFirstGridPointInDegrees", 0.0)
    lat_last        = _get(msg, "latitudeOfLastGridPointInDegrees", 0.0)
    lon_last        = _get(msg, "longitudeOfLastGridPointInDegrees", 0.0)
    di              = _get(msg, "iDirectionIncrementInDegrees", 0.0)
    dj              = _get(msg, "jDirectionIncrementInDegrees", 0.0)
    scanning_mode   = _get(msg, "scanningMode", 0)
    resolution_flags = _get(msg, "resolutionAndComponentFlags", 48)
    shape_of_earth  = _get(msg, "shapeOfTheEarth", 6)

    drt                  = _get(msg, "dataRepresentationTemplateNumber", 0)
    reference_value      = float(_get(msg, "referenceValue", 0.0))
    binary_scale_factor  = _get(msg, "binaryScaleFactor", 0)
    decimal_scale_factor = _get(msg, "decimalScaleFactor", 0)
    bits_per_value       = _get(msg, "bitsPerValue", 0)
    original_field_type  = _get(msg, "typeOfOriginalFieldValues", 0)

    values = eccodes.codes_get_values(msg).tolist()
    bitmap_present = _get(msg, "bitmapPresent", 0)
    if bitmap_present:
        bitmap = eccodes.codes_get_array(msg, "bitmap").tolist()
        grid_values = {
            "Masked": {
                "values": values,
                "present": [bool(b) for b in bitmap],
            }
        }
    else:
        grid_values = {"Dense": values}

    return {
        "center": center,
        "subcenter": subcenter,
        "parameter": {
            "discipline": discipline,
            "category": category,
            "number": number,
        },
        "forecast": {
            "reference_time": {
                "year": year,
                "month": month,
                "day": day,
                "hour": hour,
                "minute": minute,
                "second": second,
                "significance": significance,
            },
            "time_range_unit": time_range_unit,
            "forecast_offset": forecast_offset,
        },
        "level": {
            "type1": type1,
            "scale_factor1": scale_factor1,
            "scaled_value1": scaled_value1,
            "type2": type2,
            "scale_factor2": scale_factor2,
            "scaled_value2": scaled_value2,
        },
        "ensemble": ensemble,
        "grid": {
            "template": gdt,
            "num_data_points": num_data_points,
            "nx": nx,
            "ny": ny,
            "lat_first": lat_first,
            "lon_first": lon_first,
            "lat_last": lat_last,
            "lon_last": lon_last,
            "di": di,
            "dj": dj,
            "scanning_mode": scanning_mode,
            "resolution_flags": resolution_flags,
            "shape_of_earth": shape_of_earth,
        },
        "values": grid_values,
        "gdt_template": gdt,
        "pdt_template": pdt,
        "drt_template": drt,
        "packing": {
            "reference_value": reference_value,
            "binary_scale_factor": binary_scale_factor,
            "decimal_scale_factor": decimal_scale_factor,
            "bits_per_value": bits_per_value,
            "original_field_type": original_field_type,
        },
    }


def gen_golden_python(grib2_path, fixture_id, output_dir):
    """Generate golden JSON using eccodes Python bindings."""
    fields = []
    with open(grib2_path, "rb") as f:
        while True:
            try:
                msg = eccodes.codes_grib_new_from_file(f)
            except Exception:
                break
            if msg is None:
                break
            try:
                field = decode_message_python(msg)
                if field is not None:
                    fields.append(field)
            finally:
                eccodes.codes_release(msg)

    if not fields:
        print(f"WARNING: no GRIB2 fields decoded from {grib2_path}", file=sys.stderr)
        sys.exit(1)

    golden = {
        "fixture_id": fixture_id,
        "_provenance": (
            f"Generated by scripts/gen_golden.py from {Path(grib2_path).name}"
            " using eccodes Python bindings."
        ),
        "fields": fields,
    }

    out = Path(output_dir) / f"{fixture_id}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    with open(out, "w") as fh:
        json.dump(golden, fh, indent=4)
    print(f"Written: {out}  ({len(fields)} field(s))")


# ============================================================================
# CLI tools implementation (fallback)
# ============================================================================

def run_grib_dump(grib2_path):
    """Run grib_dump with JSON output and return parsed result."""
    result = subprocess.run(
        ["grib_dump", "-j", grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        raise RuntimeError(f"grib_dump failed: {result.stderr}")
    return json.loads(result.stdout)


def run_grib_get(grib2_path, key):
    """Run grib_get to extract a single key value."""
    result = subprocess.run(
        ["grib_get", "-p", key, grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        return None
    lines = result.stdout.strip().split('\n')
    if not lines or not lines[0].strip():
        return None
    return lines[0].strip()


def extract_values_array(grib2_path):
    """Extract the data values array from GRIB file.

    Note: eccodes CLI tools (grib_get) cannot directly extract array-type keys.
    For full values extraction, either:
    - Build wgrib2 from source (wgrib2.tgz in repo)
    - Use eccodes Python bindings (pip install eccodes)
    - Use a custom C decoder

    Returns empty list for now - basic metadata extraction is the priority.
    """
    # TODO: Implement values extraction via wgrib2 or C binding
    return []


def extract_bitmap(grib2_path):
    """Extract bitmap if present.

    Note: eccodes CLI tools cannot directly extract array-type keys like bitmap.
    Returns None for now - bitmap detection is handled in metadata.
    """
    return None


def key_list_to_dict(key_list):
    """Convert grib_dump key list format to dict."""
    result = {}
    for item in key_list:
        if "key" in item and "value" in item:
            result[item["key"]] = item["value"]
    return result


def decode_message_cli(data_dict):
    """Extract one field dict from parsed grib_dump data using CLI tools."""
    edition = data_dict.get("editionNumber")
    if edition != 2:
        return None

    center = data_dict.get("centre", 0)
    subcenter = data_dict.get("subCentre", 0)

    discipline = data_dict.get("discipline", 0)
    category = data_dict.get("parameterCategory", 0)
    number = data_dict.get("parameterNumber", 0)

    # Parse date from YYYYMMDD format
    data_date = data_dict.get("dataDate", 0)
    if data_date:
        date_str = str(int(data_date))
        year = int(date_str[:4]) if len(date_str) >= 4 else 0
        month = int(date_str[4:6]) if len(date_str) >= 6 else 0
        day = int(date_str[6:8]) if len(date_str) >= 8 else 0
    else:
        year = month = day = 0

    # Parse time from HHMM format
    data_time = data_dict.get("dataTime", 0)
    if data_time:
        time_str = str(int(data_time)).zfill(4)
        hour = int(time_str[:2]) if len(time_str) >= 2 else 0
        minute = int(time_str[2:4]) if len(time_str) >= 4 else 0
    else:
        hour = minute = 0
    second = data_dict.get("second", 0)
    if second is None:
        second = 0

    significance = data_dict.get("significanceOfReferenceTime", 0)

    time_range_unit = data_dict.get("indicatorOfUnitForForecastTime", 1)
    forecast_offset = data_dict.get("forecastTime", 0)

    type1 = data_dict.get("typeOfFirstFixedSurface", 255)
    scale_factor1 = data_dict.get("scaleFactorOfFirstFixedSurface", 0)
    scaled_value1 = data_dict.get("scaledValueOfFirstFixedSurface", 0)
    type2 = data_dict.get("typeOfSecondFixedSurface", 255)
    scale_factor2 = data_dict.get("scaleFactorOfSecondFixedSurface", 0)
    scaled_value2 = data_dict.get("scaledValueOfSecondFixedSurface", 0)

    pdt = data_dict.get("productDefinitionTemplateNumber", 0)
    ensemble = None
    if pdt in (1, 11):
        ensemble = {
            "member_type": data_dict.get("typeOfEnsembleForecast", 0),
            "number": data_dict.get("perturbationNumber", 0),
        }

    gdt = data_dict.get("gridDefinitionTemplateNumber", 0)
    num_data_points = data_dict.get("numberOfDataPoints", 0)
    nx = data_dict.get("Ni", 0)
    ny = data_dict.get("Nj", 0)
    lat_first = data_dict.get("latitudeOfFirstGridPointInDegrees", 0.0)
    lon_first = data_dict.get("longitudeOfFirstGridPointInDegrees", 0.0)
    lat_last = data_dict.get("latitudeOfLastGridPointInDegrees", 0.0)
    lon_last = data_dict.get("longitudeOfLastGridPointInDegrees", 0.0)
    di = data_dict.get("iDirectionIncrementInDegrees", 0.0)
    dj = data_dict.get("jDirectionIncrementInDegrees", 0.0)

    # Compute scanning mode from individual flags
    scanning_mode = 0
    if data_dict.get("iScansNegatively", 0):
        scanning_mode |= 0x80
    if data_dict.get("jScansPositively", 0):
        scanning_mode |= 0x40
    if data_dict.get("jPointsAreConsecutive", 0):
        scanning_mode |= 0x20
    if data_dict.get("alternativeRowScanning", 0):
        scanning_mode |= 0x10

    resolution_flags = data_dict.get("resolutionAndComponentFlags", 48)
    shape_of_earth = data_dict.get("shapeOfTheEarth", 6)

    drt = data_dict.get("dataRepresentationTemplateNumber", 0)
    reference_value = float(data_dict.get("referenceValue", 0.0))
    binary_scale_factor = data_dict.get("binaryScaleFactor", 0)
    decimal_scale_factor = data_dict.get("decimalScaleFactor", 0)
    bits_per_value = data_dict.get("bitsPerValue", 0)
    original_field_type = data_dict.get("typeOfOriginalFieldValues", 0)

    return {
        "center": center,
        "subcenter": subcenter,
        "parameter": {
            "discipline": discipline,
            "category": category,
            "number": number,
        },
        "forecast": {
            "reference_time": {
                "year": year,
                "month": month,
                "day": day,
                "hour": hour,
                "minute": minute,
                "second": second,
                "significance": significance,
            },
            "time_range_unit": time_range_unit,
            "forecast_offset": forecast_offset,
        },
        "level": {
            "type1": type1,
            "scale_factor1": scale_factor1,
            "scaled_value1": scaled_value1,
            "type2": type2,
            "scale_factor2": scale_factor2,
            "scaled_value2": scaled_value2,
        },
        "ensemble": ensemble,
        "grid": {
            "template": gdt,
            "num_data_points": num_data_points,
            "nx": nx,
            "ny": ny,
            "lat_first": lat_first,
            "lon_first": lon_first,
            "lat_last": lat_last,
            "lon_last": lon_last,
            "di": di,
            "dj": dj,
            "scanning_mode": scanning_mode,
            "resolution_flags": resolution_flags,
            "shape_of_earth": shape_of_earth,
        },
        "gdt_template": gdt,
        "pdt_template": pdt,
        "drt_template": drt,
        "packing": {
            "reference_value": reference_value,
            "binary_scale_factor": binary_scale_factor,
            "decimal_scale_factor": decimal_scale_factor,
            "bits_per_value": bits_per_value,
            "original_field_type": original_field_type,
        },
    }


def gen_golden_cli(grib2_path, fixture_id, output_dir):
    """Generate golden JSON using eccodes CLI tools."""
    # Get metadata from grib_dump
    dump_data = run_grib_dump(grib2_path)
    messages = dump_data.get("messages", [])

    if not messages:
        print(f"WARNING: no GRIB2 messages decoded from {grib2_path}", file=sys.stderr)
        sys.exit(1)

    # Extract values array and bitmap
    values = extract_values_array(grib2_path)
    bitmap = extract_bitmap(grib2_path)

    # Process all messages
    fields = []
    for msg_list in messages:
        msg_dict = key_list_to_dict(msg_list)
        field = decode_message_cli(msg_dict)
        if field is not None:
            # Add values to field
            if bitmap:
                field["values"] = {
                    "Masked": {
                        "values": values,
                        "present": bitmap,
                    }
                }
            else:
                field["values"] = {"Dense": values}
            fields.append(field)

    if not fields:
        print(f"WARNING: failed to decode GRIB2 messages from {grib2_path}", file=sys.stderr)
        sys.exit(1)

    golden = {
        "fixture_id": fixture_id,
        "_provenance": (
            f"Generated by scripts/gen_golden.py from {Path(grib2_path).name}"
            " using eccodes CLI tools."
        ),
        "fields": fields,
    }

    out = Path(output_dir) / f"{fixture_id}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    with open(out, "w") as fh:
        json.dump(golden, fh, indent=4)
    print(f"Written: {out}  ({len(fields)} field(s))")


# ============================================================================
# Main entry point
# ============================================================================

def gen_golden(grib2_path, fixture_id, output_dir):
    """Generate golden JSON from a GRIB2 file.

    Uses eccodes Python bindings if available, otherwise falls back to CLI tools.
    """
    if ECCODES_AVAILABLE:
        print(f"Using eccodes Python bindings...")
        gen_golden_python(grib2_path, fixture_id, output_dir)
    else:
        print(f"Using eccodes CLI tools...")
        gen_golden_cli(grib2_path, fixture_id, output_dir)


def main():
    parser = argparse.ArgumentParser(
        description="Generate gribtract golden reference JSON from a GRIB2 file"
    )
    parser.add_argument("grib2_file", help="Input GRIB2 file")
    parser.add_argument("fixture_id", help="Fixture ID (becomes the output filename)")
    parser.add_argument(
        "--output-dir",
        default="tests/corpus/golden",
        help="Directory for the output JSON (default: tests/corpus/golden)",
    )
    args = parser.parse_args()
    gen_golden(args.grib2_file, args.fixture_id, args.output_dir)


if __name__ == "__main__":
    main()
