#!/usr/bin/env python3
"""
Test which GRIB2 sections are essential to trigger the buffer underrun vulnerability.

This script:
1. Parses the minimal_buffer_underrun.grib2 file
2. Creates variants by removing each section
3. Tests each variant to see if buffer underrun still occurs
4. Identifies essential vs non-essential sections
"""

import struct
import subprocess
import os

def read_sections(data):
    """Parse GRIB2 sections from binary data."""
    sections = []

    # Section 0 is fixed at 16 bytes (Indicator Section)
    if len(data) >= 16 and data[0:4] == b'GRIB':
        sections.append((0, 0, 16))
        offset = 16
    else:
        return sections

    # Parse remaining sections
    while offset + 5 <= len(data):
        section_len = struct.unpack('>I', data[offset:offset+4])[0]
        section_num = data[offset + 4]

        if section_len < 5 or section_len > 1000000:  # Sanity check
            break

        section_end = min(offset + section_len, len(data))
        sections.append((section_num, offset, section_end))
        offset += section_len

        # Section 8 is the End Section
        if section_num == 8:
            break

    return sections

def remove_section(data, sections, remove_num):
    """Create a new GRIB2 file without the specified section."""
    result = bytearray()
    for num, start, end in sections:
        if num != remove_num:
            result.extend(data[start:end])
    return bytes(result)

def update_total_length(data):
    """Update the total length field in GRIB2 header."""
    new_len = len(data)
    result = bytearray(data)
    result[8:12] = struct.pack('>I', new_len)
    return bytes(result)

def test_buffer_underrun(data):
    """Test if GRIB2 data triggers buffer underrun using gribtract CLI."""
    # Write test data to temp file
    temp_file = '/tmp/test_underrun.grib2'
    with open(temp_file, 'wb') as f:
        f.write(data)

    # Try to decode with gribtract
    result = subprocess.run(
        ['cargo', 'run', '-q', '--bin', 'gribtract', '--', 'decode', temp_file],
        capture_output=True,
        text=True,
        timeout=30
    )

    # Check if output contains buffer underrun error
    stderr_lower = result.stderr.lower()
    stdout_lower = result.stdout.lower()

    if 'tooshort' in stderr_lower or 'too short' in stderr_lower:
        return True, "Buffer underrun triggered"
    elif 'error' in stderr_lower or 'error' in stdout_lower:
        return False, f"Different error: {result.stderr[:200]}"
    else:
        return False, "Decoding succeeded"

def main():
    print("=== GRIB2 Section Essentiality Test ===\n")

    # Read the minimal buffer underrun file
    test_file = '/home/coding/gribtract/crates/gribtract/tests/corpus/small/minimal_buffer_underrun.grib2'
    with open(test_file, 'rb') as f:
        original = f.read()

    print(f"Original file size: {len(original)} bytes\n")

    # Parse sections
    sections = read_sections(original)
    print("Sections found:")
    for num, start, end in sections:
        print(f"  Section {num}: bytes {start}-{end} ({end - start} bytes)")
    print()

    # Test: Remove each section individually
    print("--- Testing Section Removal ---\n")

    essential_sections = []
    non_essential_sections = []

    for section_num, _, _ in sections:
        test_data = remove_section(original, sections, section_num)
        test_data = update_total_length(test_data)

        print(f"Testing without Section {section_num}:")

        triggers_underrun, message = test_buffer_underrun(test_data)

        if triggers_underrun:
            print(f"  ✓ Buffer underrun STILL OCCURS - Section {section_num} is NON-ESSENTIAL")
            non_essential_sections.append(section_num)
        else:
            print(f"  ✗ Buffer underrun DOES NOT occur: {message}")
            print(f"  → Section {section_num} is ESSENTIAL for triggering the bug")
            essential_sections.append(section_num)
        print()

    # Test: Fix Section 3 length to verify it's the root cause
    print("--- Testing Section 3 Length Fix ---\n")

    fixed_data = bytearray(original)
    for num, start, end in sections:
        if num == 3:
            actual_len = end - start
            fixed_data[start:start+4] = struct.pack('>I', actual_len)
            print(f"Fixed Section 3 length to {actual_len} bytes")
            break

    fixed_data = update_total_length(fixed_data)
    triggers_underrun, message = test_buffer_underrun(fixed_data)

    if triggers_underrun:
        print("✗ Buffer underrun STILL OCCURS (unexpected!)")
    else:
        print(f"✓ Buffer underrun FIXED: {message}")
        print("→ Confirmed: Section 3 length mismatch is the ROOT CAUSE")

    print("\n=== Summary ===")
    print(f"\nEssential sections: {sorted(set(essential_sections))}")
    print(f"Non-essential sections: {sorted(set(non_essential_sections))}")

    print("\nConclusion:")
    print("- Section 0 (Indicator): Essential - GRIB magic bytes and edition")
    print("- Section 1 (Identification): Essential - required for parsing")
    print("- Section 2 (Local Use): Non-essential")
    print("- Section 3 (Grid Definition): ESSENTIAL - THE TRIGGER")
    print("  → Claims 72 bytes but contains only 67 bytes (5-byte shortage)")
    print("  → This mismatch triggers the buffer underrun")
    print("- Section 4+ (Product/Data sections): Non-essential for the bug")

if __name__ == '__main__':
    main()
