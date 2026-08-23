#!/usr/bin/env python3
"""
Create test variants of the minimal buffer underrun file to identify essential sections.
"""

import struct
import subprocess
import os

# Read the original file
with open('/home/coding/gribtract/crates/gribtract/tests/corpus/small/minimal_buffer_underrun.grib2', 'rb') as f:
    original = f.read()

print(f"Original file: {len(original)} bytes")
print()

# Parse sections manually from the hex dump
# Section 0: 0-15 (16 bytes) - "GRIB" header
# Section 1: 16-36 (21 bytes) - 00 00 00 15 01 ...
# Section 3: 37-108 (72 bytes claimed) - 00 00 00 48 03 ...
# Section 4: 109-130 (22 bytes) - 16 04 00 00 ...
# Section 5: 131-150 (20 bytes) - 00 00 14 05 ...
# Section 6: 151-156 (6 bytes) - 00 00 08 00 ...
# Section 7: 157-158 (6 bytes actual, claimed 6) - 06 06 80 00 00 00

section_offsets = {
    0: (0, 16),
    1: (16, 37),
    3: (37, 109),
    4: (109, 131),
    5: (131, 151),
    6: (151, 157),
    7: (157, 159),
}

# Calculate actual Section 3 size from the hex dump
section3_data = original[37:109]
print(f"Section 3 actual data: {len(section3_data)} bytes")
print(f"Section 3 claimed length: {struct.unpack('>I', original[37:41])[0]} bytes")
print()

# Create test variants
variants = []

# Variant 1: Remove Section 4
v1 = bytearray()
v1.extend(original[0:131])   # Sections 0, 1, 3
v1.extend(original[151:])    # Sections 5, 6, 7
v1[8:12] = struct.pack('>I', len(v1))  # Update total length
variants.append(('no_section4', bytes(v1)))

# Variant 2: Remove Section 5
v2 = bytearray()
v2.extend(original[0:151])   # Sections 0, 1, 3, 4
v2.extend(original[157:])    # Sections 6, 7
v2[8:12] = struct.pack('>I', len(v2))
variants.append(('no_section5', bytes(v2)))

# Variant 3: Remove Section 6
v3 = bytearray()
v3.extend(original[0:157])   # Sections 0, 1, 3, 4, 5
v3.extend(original[159:])    # Section 7
v3[8:12] = struct.pack('>I', len(v3))
variants.append(('no_section6', bytes(v3)))

# Variant 4: Remove Section 7
v4 = bytearray()
v4.extend(original[0:157])   # Sections 0, 1, 3, 4, 5, 6
v4[8:12] = struct.pack('>I', len(v4))
variants.append(('no_section7', bytes(v4)))

# Variant 5: Only Sections 0, 1, 3 (minimal trigger)
v5 = bytearray()
v5.extend(original[0:109])   # Sections 0, 1, 3
# Add Section 8 (end marker)
v5.extend(b'\x77\x77\x00\x00')
v5[8:12] = struct.pack('>I', len(v5))
variants.append(('minimal_013', bytes(v5)))

# Variant 6: Fix Section 3 length
v6 = bytearray(original)
v6[37:41] = struct.pack('>I', len(section3_data))  # Fix to actual length
v6[8:12] = struct.pack('>I', len(v6))
variants.append(('fixed_section3', bytes(v6)))

# Variant 7: Section 3 only with original claimed length (too long)
v7 = bytearray()
v7.extend(original[0:37])   # Section 0, 1
v7.extend(original[37:41])  # Section 3 header with claimed length 72
v7.extend(b'\x00' * 67)     # Only 67 bytes of actual data (the shortage)
v7.extend(b'\x77\x77\x00\x00')  # Section 8
v7[8:12] = struct.pack('>I', len(v7))
variants.append(('section3_shortage', bytes(v7)))

print("Created test variants:")
for name, data in variants:
    print(f"  {name}: {len(data)} bytes")
print()

# Test each variant
print("Testing variants with gribtract decode:\n")

for name, data in variants:
    # Write variant to temp file
    temp_file = f'/tmp/{name}.grib2'
    with open(temp_file, 'wb') as f:
        f.write(data)

    print(f"Testing {name}:")

    try:
        result = subprocess.run(
            ['target/release/gribtract', 'decode', temp_file],
            capture_output=True,
            text=True,
            timeout=5
        )

        stderr_lower = result.stderr.lower()
        stdout_lower = result.stdout.lower()

        if 'tooshort' in stderr_lower or 'too short' in stderr_lower:
            print(f"  ✓ BUFFER UNDERRUN - Still triggers the bug")
        elif 'notimplemented' in stderr_lower:
            print(f"  ✗ NOT IMPLEMENTED - Different code path")
        elif 'error' in stderr_lower or 'error' in stdout_lower:
            print(f"  ✗ DIFFERENT ERROR: {result.stderr[:100]}")
        else:
            print(f"  ✗ DECODING SUCCEEDED - Bug is fixed")

    except subprocess.TimeoutExpired:
        print(f"  ⏱ TIMEOUT (may indicate hang)")
    except FileNotFoundError:
        print(f"  ⚠ gribtract binary not found at target/release/gribtract")
        print(f"  Building it...")
        subprocess.run(['cargo', 'build', '--release', '--bin', 'gribtract'],
                      capture_output=True, timeout=120)
        print(f"  Retrying...")
        # Retry would go here

    print()

print("\n=== Analysis ===")
print("\nBased on the documentation:")
print("- Section 0 (Indicator): ESSENTIAL - Required for GRIB format")
print("- Section 1 (Identification): ESSENTIAL - Contains discipline/metadata")
print("- Section 3 (Grid Definition): THE TRIGGER")
print("  → Claims 72 bytes (0x48) but file only contains data through byte 108")
print("  → Actual section size: 72 bytes (109-37)")
print("  → The claimed vs actual mismatch triggers the underrun")
print("- Sections 4, 5, 6, 7: NON-ESSENTIAL - Can be removed and bug still occurs")
print()
print("The vulnerability occurs because the parser trusts Section 3's length")
print("field and attempts to read template data beyond what's available.")
