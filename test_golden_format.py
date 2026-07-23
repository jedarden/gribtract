#!/usr/bin/env python3
"""Test script to verify gen_golden.py output matches expected golden format.

This compares the structure of gen_golden.py output against the actual golden files.
"""

import json
import sys
from pathlib import Path


def load_expected_schema():
    """Load expected schema from existing golden files."""
    golden_dir = Path("tests/corpus/golden")
    schemas = []

    for golden_file in golden_dir.glob("*.json"):
        with open(golden_file) as f:
            data = json.load(f)
            if "fields" in data and len(data["fields"]) > 0:
                schemas.append({
                    "file": golden_file.name,
                    "field_structure": get_field_structure(data["fields"][0])
                })

    return schemas


def get_field_structure(field):
    """Extract structure keys from a field (recursively for nested dicts)."""
    if isinstance(field, dict):
        return {k: get_field_structure(v) for k, v in field.items()}
    elif isinstance(field, list) and len(field) > 0:
        return [get_field_structure(field[0])]
    else:
        return type(field).__name__


def compare_structures(expected, actual, path=""):
    """Compare two structures and report differences."""
    differences = []

    if isinstance(expected, dict) and isinstance(actual, dict):
        # Check for missing keys in actual
        for key in expected.keys():
            if key not in actual:
                differences.append(f"Missing key: {path}.{key}")

        # Check for extra keys in actual
        for key in actual.keys():
            if key not in expected:
                differences.append(f"Extra key: {path}.{key}")

        # Recursively compare common keys
        for key in set(expected.keys()) & set(actual.keys()):
            diffs = compare_structures(expected[key], actual[key], f"{path}.{key}" if path else key)
            differences.extend(diffs)

    elif isinstance(expected, list) and isinstance(actual, list):
        if len(actual) > 0:
            diffs = compare_structures(expected[0], actual[0], f"{path}[0]")
            differences.extend(diffs)

    return differences


def main():
    print("=== Testing Golden Format Compliance ===\n")

    # Load expected schemas from golden files
    expected_schemas = load_expected_schema()
    if not expected_schemas:
        print("ERROR: No golden files found in tests/corpus/golden/")
        sys.exit(1)

    print(f"Loaded {len(expected_schemas)} reference schemas from golden files\n")

    # Show expected structure
    print("=== Expected Field Structure (from golden files) ===")
    print(json.dumps(expected_schemas[0]["field_structure"], indent=2))
    print()

    # Load gen_golden.py output
    gen_golden_output = Path("/tmp/test_gen_golden.json")
    if not gen_golden_output.exists():
        print("ERROR: gen_golden.py output not found at /tmp/test_gen_golden.json")
        print("Run: python3 scripts/gen_golden.py test_data/nam_awip12_drt3.grib2 test_gen_golden --output-dir /tmp")
        sys.exit(1)

    with open(gen_golden_output) as f:
        gen_data = json.load(f)

    if "fields" not in gen_data or len(gen_data["fields"]) == 0:
        print("ERROR: gen_golden.py output has no fields")
        sys.exit(1)

    gen_structure = get_field_structure(gen_data["fields"][0])

    print("=== Generated Field Structure (from gen_golden.py) ===")
    print(json.dumps(gen_structure, indent=2))
    print()

    # Compare structures
    print("=== Structure Comparison ===")
    expected_structure = expected_schemas[0]["field_structure"]
    differences = compare_structures(expected_structure, gen_structure)

    if differences:
        print(f"\n❌ FAILED: {len(differences)} structural differences found:\n")
        for diff in differences:
            print(f"  - {diff}")
        print()
        return 1
    else:
        print("\n✅ PASSED: Structures match!\n")
        return 0


if __name__ == "__main__":
    sys.exit(main())
