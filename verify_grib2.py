#!/usr/bin/env python3
"""
GRIB2 File Verification Script
Verifies GDT 3.30 (Lambert Conformal) and DRT=3 (Complex Packing)
"""

import sys
import eccodes

def inspect_grib2(filepath):
    """
    Inspect a GRIB2 file and report GDT and DRT information for all messages.
    """
    print(f"\n{'='*70}")
    print(f"FILE: {filepath}")
    print('='*70)

    message_count = 0
    gdt_30_count = 0
    drt_3_count = 0
    gdt_unknown = 0
    drt_unknown = 0

    try:
        with open(filepath, 'rb') as f:
            while True:
                # Read next message
                msg_id = eccodes.codes_grib_new_from_file(f)
                if msg_id is None:
                    break

                message_count += 1

                # Get template numbers
                try:
                    gdt = eccodes.codes_get(msg_id, 'gridDefinitionTemplateNumber')
                    drt = eccodes.codes_get(msg_id, 'dataRepresentationTemplateNumber')

                    # Get parameter info for context
                    try:
                        param = eccodes.codes_get(msg_id, 'paramId')
                        name = eccodes.codes_get(msg_id, 'name')
                    except:
                        param = "unknown"
                        name = "unknown"

                    # Get grid type info
                    try:
                        grid_type = eccodes.codes_get(msg_id, 'gridType')
                    except:
                        grid_type = "unknown"

                    # Get grid size
                    try:
                        nx = eccodes.codes_get(msg_id, 'Nx')
                        ny = eccodes.codes_get(msg_id, 'Ny')
                    except:
                        nx = "unknown"
                        ny = "unknown"

                    # Count GDT 3.30 (gdt=30) and DRT=3
                    if gdt == 30:
                        gdt_30_count += 1
                    else:
                        gdt_unknown += 1

                    if drt == 3:
                        drt_3_count += 1
                    else:
                        drt_unknown += 1

                    # Print message info
                    gdt_str = "3.30 ✓" if gdt == 30 else f"GDT={gdt}"
                    drt_str = "DRT=3 ✓" if drt == 3 else f"DRT={drt}"

                    print(f"Message {message_count:3d}: {gdt_str}, {drt_str}, param={param}, name={name}")
                    print(f"            grid_type={grid_type}, size={nx}x{ny}")

                except Exception as e:
                    print(f"Message {message_count:3d}: ERROR - {e}")

                # Release message
                eccodes.codes_release(msg_id)

    except Exception as e:
        print(f"ERROR reading file: {e}")
        return

    # Print summary
    print(f"\n{'-'*70}")
    print("SUMMARY:")
    print(f"  Total messages: {message_count}")
    print(f"  GDT 3.30 (Lambert Conformal): {gdt_30_count}/{message_count} ({100*gdt_30_count/message_count if message_count > 0 else 0:.1f}%)")
    print(f"  DRT=3 (Complex Packing): {drt_3_count}/{message_count} ({100*drt_3_count/message_count if message_count > 0 else 0:.1f}%)")

    if gdt_unknown > 0:
        print(f"  ⚠️  Non-GDT 3.30 messages: {gdt_unknown}")
    if drt_unknown > 0:
        print(f"  ⚠️  Non-DRT=3 messages: {drt_unknown}")

    print('='*70)

def main():
    if len(sys.argv) < 2:
        print("Usage: verify_grib2.py <file.grib2> [file2.grib2 ...]")
        sys.exit(1)

    for filepath in sys.argv[1:]:
        inspect_grib2(filepath)

    print("\n" + "="*70)
    print("NOTES:")
    print("  - GDT=30 corresponds to GDT 3.30 (Lambert Conformal Conic)")
    print("  - DRT=3 corresponds to Data Representation Template 3 (Complex packing)")
    print("  - These files contain multiple messages, one per parameter/level")
    print("="*70)

if __name__ == "__main__":
    main()
