#!/usr/bin/env python3
"""gen_golden.py — Generate gribtract golden reference files using eccodes.

Run this script where eccodes is available (e.g., internal cluster with
ECMWF toolchain installed).  The output JSON is committed into the repo and
used by the differential harness for offline comparison.

Usage:
    python3 scripts/gen_golden.py <grib2_file> <fixture_id> [--output-dir DIR]

Output:
    tests/corpus/golden/<fixture_id>.json

The JSON format mirrors GoldenField in crates/gribtract-testutil/src/golden.rs.

Requires: eccodes Python bindings  (pip install eccodes)
"""

import argparse
import json
import sys
from pathlib import Path

try:
    import eccodes
except ImportError:
    print("ERROR: eccodes Python bindings not found.  Install with: pip install eccodes", file=sys.stderr)
    sys.exit(1)


def _get(msg, key, default=None):
    """Retrieve a key from an eccodes message, returning default on error."""
    try:
        return eccodes.codes_get(msg, key)
    except eccodes.KeyValueNotFoundError:
        return default


def decode_message(msg):
    """Extract one GoldenField dict from an open eccodes message handle."""
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


def gen_golden(grib2_path, fixture_id, output_dir):
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
                field = decode_message(msg)
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


def main():
    parser = argparse.ArgumentParser(
        description="Generate gribtract golden reference JSON from a GRIB2 file using eccodes"
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
