#!/usr/bin/env python3
"""
Script to check DRT (Data Representation Type) values from GRIB2 files.
DRT is the grid definition template number stored in GRIB2 Section 3.
"""

import struct
import os
import sys
from pathlib import Path

def get_drt_value(filename):
    """Extract DRT value from a GRIB2 file."""
    try:
        with open(filename, 'rb') as f:
            # Read the header to locate sections
            data = f.read(1000)

            # Check GRIB identifier
            if data[0:4] != b'GRIB':
                return None, 'Not a GRIB file'

            # Check GRIB edition
            edition = data[7]
            if edition != 2:
                return None, f'Not GRIB2 (edition {edition})'

            # Section 1 starts at byte 16 (after Section 0)
            sec1_len = struct.unpack('>I', data[16:20])[0]

            # Section 2 starts at byte 16 + sec1_len
            sec2_offset = 16 + sec1_len
            sec2_len = struct.unpack('>I', data[sec2_offset:sec2_offset+4])[0]

            # Section 3 starts at sec2_offset + sec2_len
            sec3_offset = sec2_offset + sec2_len
            sec3_len = struct.unpack('>I', data[sec3_offset:sec3_offset+4])[0]

            if sec3_len > 0:
                # Grid definition template number is at offset 4 within Section 3
                # Section 3 structure: [4 bytes length][1 byte number][2 bytes template number]...
                grid_template = struct.unpack('>H', data[sec3_offset+5:sec3_offset+7])[0]
                return grid_template, 'OK'
            else:
                return None, 'Section 3 has zero length'

    except Exception as e:
        return None, str(e)

def main():
    samples_dir = Path('samples/grib2-noaa-gfs')

    if not samples_dir.exists():
        print(f"Error: {samples_dir} does not exist")
        sys.exit(1)

    # Find all .f000, .f003, .f006, .f012 files
    grib_files = sorted([f for f in samples_dir.glob('*.f*') if f.is_file()])

    print(f"Checking DRT values for {len(grib_files)} GRIB2 files...")
    print("=" * 80)

    results = []
    drt0_files = []
    drt_other_files = []

    for grib_file in grib_files:
        drt_value, status = get_drt_value(grib_file)

        result = {
            'filename': grib_file.name,
            'drt': drt_value,
            'status': status
        }
        results.append(result)

        if drt_value == 0:
            drt0_files.append(grib_file.name)
        elif drt_value is not None and drt_value > 0:
            drt_other_files.append(grib_file.name)

        # Print individual result
        drt_str = str(drt_value) if drt_value is not None else 'N/A'
        print(f"{grib_file.name:50} DRT={drt_str:5} ({status})")

    print("=" * 80)
    print(f"\nSUMMARY:")
    print(f"  Total files checked: {len(results)}")
    print(f"  DRT=0 files: {len(drt0_files)}")
    print(f"  DRT!=0 files: {len(drt_other_files)}")
    print(f"  Error/unknown: {len([r for r in results if r['drt'] is None])}")

    # Write results to a file
    output_file = Path('notes/drt-check-results.txt')
    output_file.parent.mkdir(exist_ok=True)

    with open(output_file, 'w') as f:
        f.write("DRT Check Results\n")
        f.write("=" * 80 + "\n\n")

        f.write("wgrib2 command used for reference:\n")
        f.write("  wgrib2 -v <filename>  (for verbose inventory)\n")
        f.write("  (Note: DRT extraction requires parsing GRIB2 Section 3 structure)\n\n")

        f.write("Results:\n")
        f.write("-" * 80 + "\n")
        for result in results:
            drt_str = str(result['drt']) if result['drt'] is not None else 'N/A'
            f.write(f"{result['filename']:50} DRT={drt_str:5} ({result['status']})\n")

        f.write("\n" + "=" * 80 + "\n")
        f.write(f"\nSUMMARY:\n")
        f.write(f"  Total files checked: {len(results)}\n")
        f.write(f"  DRT=0 files: {len(drt0_files)}\n")
        f.write(f"  DRT!=0 files: {len(drt_other_files)}\n")
        f.write(f"  Error/unknown: {len([r for r in results if r['drt'] is None])}\n\n")

        if drt0_files:
            f.write(f"\nFiles with DRT=0 ({len(drt0_files)}):\n")
            for filename in drt0_files:
                f.write(f"  - {filename}\n")

        if drt_other_files:
            f.write(f"\nFiles with DRT!=0 ({len(drt_other_files)}):\n")
            for filename in drt_other_files:
                f.write(f"  - {filename}\n")

    print(f"\nResults written to: {output_file}")

if __name__ == '__main__':
    main()
