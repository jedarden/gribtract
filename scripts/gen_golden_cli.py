#!/usr/bin/env python3
"""gen_golden_cli.py — Generate gribtract golden reference files using eccodes CLI tools.

This script is an adapted version of gen_golden.py that uses the eccodes CLI tools
instead of Python bindings, which are not available in this environment.

Usage:
    python3 scripts/gen_golden_cli.py <grib2_file> <fixture_id> [--output-dir DIR]

Output:
    tests/corpus/golden/<fixture_id>.json

The JSON format mirrors GoldenField in crates/gribtract-testutil/src/golden.rs.

Requires: eccodes CLI tools (grib_dump, grib_get) available in PATH
"""

import argparse
import json
import sys
import subprocess
from pathlib import Path


def run_grib_dump(grib2_path):
    """Run grib_dump with JSON output and return parsed result.

    Requests all keys needed for golden format including packing metadata.
    """
    # Request all keys needed for golden format (packing + sections 1-4)
    keys_needed = ",".join([
        "discipline", "editionNumber", "centre", "subCentre",
        "significanceOfReferenceTime", "dataDate", "dataTime", "second",
        "productionStatusOfProcessedData", "typeOfProcessedData",
        "numberOfDataPoints", "interpretationOfNumberOfPoints",
        "gridDefinitionTemplateNumber", "shapeOfTheEarth",
        "Ni", "Nj", "iScansNegatively", "jScansPositively",
        "jPointsAreConsecutive", "alternativeRowScanning",
        "latitudeOfFirstGridPointInDegrees", "longitudeOfFirstGridPointInDegrees",
        "latitudeOfLastGridPointInDegrees", "longitudeOfLastGridPointInDegrees",
        "iDirectionIncrementInDegrees", "jDirectionIncrementInDegrees",
        "resolutionAndComponentFlags",
        "productDefinitionTemplateNumber", "parameterCategory", "parameterNumber",
        "typeOfFirstFixedSurface", "scaleFactorOfFirstFixedSurface", "scaledValueOfFirstFixedSurface",
        "typeOfSecondFixedSurface", "scaleFactorOfSecondFixedSurface", "scaledValueOfSecondFixedSurface",
        "typeOfEnsembleForecast", "perturbationNumber",
        "indicatorOfUnitForForecastTime", "forecastTime",
        "dataRepresentationTemplateNumber", "referenceValue",
        "binaryScaleFactor", "decimalScaleFactor", "bitsPerValue", "typeOfOriginalFieldValues"
    ])

    result = subprocess.run(
        ["grib_dump", "-j", "-p", keys_needed, grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        raise RuntimeError(f"grib_dump failed: {result.stderr}")

    # grib_dump with -p outputs multiple JSON objects, wrap in brackets to make array
    json_str = "[" + result.stdout + "]"
    return json.loads(json_str)


def run_grib_get(grib2_path, key, dtype="s"):
    """Run grib_get to extract a single key value."""
    result = subprocess.run(
        ["grib_get", f"-p{key}:{dtype}", grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        return None
    # Parse output format: "value"
    lines = result.stdout.strip().split('\n')
    if not lines or not lines[0].strip():
        return None
    return lines[0].strip()


def extract_values_array(grib2_path):
    """Extract the data values array from GRIB file.

    Uses grib_dump -j -p values to get values as JSON array.
    """
    result = subprocess.run(
        ["grib_dump", "-j", "-p", "values", grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        raise RuntimeError(f"grib_dump values failed: {result.stderr}")

    # Parse JSON output - should be {"key": "values", "value": [v1, v2, ...]}
    try:
        data = json.loads(result.stdout)
        if "value" in data and isinstance(data["value"], list):
            # Convert integers to floats if needed
            return [float(v) for v in data["value"]]
    except (json.JSONDecodeError, KeyError, TypeError) as e:
        raise RuntimeError(f"Failed to parse values JSON: {e}")

    return []


def extract_bitmap(grib2_path):
    """Extract bitmap if present."""
    # First check if bitmap is present
    bitmap_present = run_grib_get(grib2_path, "bitmapPresent", "i")
    if bitmap_present is None or int(bitmap_present) == 0:
        return None

    result = subprocess.run(
        ["grib_get", "-p", "bitmap", grib2_path],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        return None

    # Parse bitmap values
    bitmap_text = result.stdout.strip()
    if not bitmap_text:
        return None

    bitmap = []
    for val_str in bitmap_text.split():
        try:
            bitmap.append(bool(int(val_str)))
        except ValueError:
            continue

    return bitmap


def key_value_to_dict(messages):
    """Convert grib_dump key-value format to dict."""
    if not messages or len(messages) == 0:
        return {}

    # grib_dump JSON format: {"messages": [[{"key": "...", "value": ...}, ...]]}
    # Extract the first message's keys
    msg_data = messages[0]
    result = {}

    for item in msg_data:
        if "key" in item and "value" in item:
            result[item["key"]] = item["value"]

    return result


def decode_message_from_dict(data):
    """Extract one GoldenField dict from parsed grib_dump data."""
    edition = data.get("editionNumber")
    if edition != 2:
        return None

    center = data.get("centre", 0)
    subcenter = data.get("subCentre", 0)

    discipline = data.get("discipline", 0)
    category = data.get("parameterCategory", 0)
    number = data.get("parameterNumber", 0)

    # Parse date from YYYYMMDD format
    data_date = data.get("dataDate", 0)
    if data_date:
        date_str = str(int(data_date))
        year = int(date_str[:4]) if len(date_str) >= 4 else 0
        month = int(date_str[4:6]) if len(date_str) >= 6 else 0
        day = int(date_str[6:8]) if len(date_str) >= 8 else 0
    else:
        year = month = day = 0

    # Parse time from HHMM format
    data_time = data.get("dataTime", 0)
    if data_time:
        time_str = str(int(data_time)).zfill(4)
        hour = int(time_str[:2]) if len(time_str) >= 2 else 0
        minute = int(time_str[2:4]) if len(time_str) >= 4 else 0
    else:
        hour = minute = 0
    second = data.get("second", 0)
    if second is None:
        second = 0

    significance = data.get("significanceOfReferenceTime", 0)

    time_range_unit = data.get("indicatorOfUnitForForecastTime", 1)
    forecast_offset = data.get("forecastTime", 0)

    type1 = data.get("typeOfFirstFixedSurface", 255)
    scale_factor1 = data.get("scaleFactorOfFirstFixedSurface", 0)
    scaled_value1 = data.get("scaledValueOfFirstFixedSurface", 0)
    type2 = data.get("typeOfSecondFixedSurface", 255)
    scale_factor2 = data.get("scaleFactorOfSecondFixedSurface", 0)
    scaled_value2 = data.get("scaledValueOfSecondFixedSurface", 0)

    pdt = data.get("productDefinitionTemplateNumber", 0)
    ensemble = None
    if pdt in (1, 11):
        ensemble = {
            "member_type": data.get("typeOfEnsembleForecast", 0),
            "number": data.get("perturbationNumber", 0),
        }

    gdt = data.get("gridDefinitionTemplateNumber", 0)
    num_data_points = data.get("numberOfDataPoints", 0)
    nx = data.get("Ni", 0)
    ny = data.get("Nj", 0)
    lat_first = data.get("latitudeOfFirstGridPointInDegrees", 0.0)
    lon_first = data.get("longitudeOfFirstGridPointInDegrees", 0.0)
    lat_last = data.get("latitudeOfLastGridPointInDegrees", 0.0)
    lon_last = data.get("longitudeOfLastGridPointInDegrees", 0.0)
    di = data.get("iDirectionIncrementInDegrees", 0.0)
    dj = data.get("jDirectionIncrementInDegrees", 0.0)

    # Compute scanning mode from individual flags
    scanning_mode = 0
    if data.get("iScansNegatively", 0):
        scanning_mode |= 0x80
    if data.get("jScansPositively", 0):
        scanning_mode |= 0x40
    if data.get("jPointsAreConsecutive", 0):
        scanning_mode |= 0x20
    if data.get("alternativeRowScanning", 0):
        scanning_mode |= 0x10

    resolution_flags = data.get("resolutionAndComponentFlags", 48)
    shape_of_earth = data.get("shapeOfTheEarth", 6)

    drt = data.get("dataRepresentationTemplateNumber", 0)
    reference_value = float(data.get("referenceValue", 0.0))
    binary_scale_factor = data.get("binaryScaleFactor", 0)
    decimal_scale_factor = data.get("decimalScaleFactor", 0)
    bits_per_value = data.get("bitsPerValue", 0)
    original_field_type = data.get("typeOfOriginalFieldValues", 0)

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


def gen_golden(grib2_path, fixture_id, output_dir):
    """Generate golden JSON from GRIB2 file using CLI tools."""
    # Get metadata from grib_dump
    dump_data = run_grib_dump(grib2_path)

    # dump_data is now an array of key-value objects directly
    if not dump_data or len(dump_data) == 0:
        print(f"WARNING: no GRIB2 messages decoded from {grib2_path}", file=sys.stderr)
        sys.exit(1)

    # Extract values array and bitmap
    values = extract_values_array(grib2_path)
    bitmap = extract_bitmap(grib2_path)

    # Process the flat array of key-value objects
    field = decode_message_from_dict(key_value_to_dict(dump_data))

    if field is None:
        print(f"WARNING: failed to decode GRIB2 message from {grib2_path}", file=sys.stderr)
        sys.exit(1)

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

    golden = {
        "fixture_id": fixture_id,
        "_provenance": (
            f"Generated by scripts/gen_golden_cli.py from {Path(grib2_path).name}"
            " using eccodes CLI tools."
        ),
        "fields": [field],
    }

    out = Path(output_dir) / f"{fixture_id}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    with open(out, "w") as fh:
        json.dump(golden, fh, indent=4)
    print(f"Written: {out}")


def main():
    parser = argparse.ArgumentParser(
        description="Generate gribtract golden reference JSON from a GRIB2 file using eccodes CLI tools"
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
