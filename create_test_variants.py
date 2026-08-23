#!/usr/bin/env python3
"""
Manually create GRIB2 test variants based on known structure.

From the hex dump analysis:
- Section 0: bytes 0-15 (16 bytes) - GRIB header
- Section 1: bytes 16-36 (21 bytes) - length 0x15=21, section 01
- Section 3: bytes 37-108 (72 bytes) - length 0x48=72, section 03
- Section 4: bytes 109-130 (22 bytes) - length 0x16=22, section 04
- Section 5: bytes 131-150 (20 bytes) - length 0x14=20, section 05
- Section 6: bytes 151-156 (6 bytes) - length 0x08=8, section 06 (but 6 bytes body)
- Section 7: bytes 157-158 (2 bytes) - actually the file ends here

Total: 159 bytes
"""

import struct
import os

# Read the original file
with open('/home/coding/gribtract/crates/gribtract/tests/corpus/small/minimal_buffer_underrun.grib2', 'rb') as f:
    data = f.read()

print(f"Original file: {len(data)} bytes")
print()

# Manual section boundaries based on analysis
sections = {
    0: (0, 16),    # Indicator Section (fixed)
    1: (16, 37),   # Identification Section (21 bytes)
    3: (37, 109),  # Grid Definition Section (72 bytes)
    4: (109, 131), # Product Definition Section (22 bytes)
    5: (131, 151), # Data Representation Section (20 bytes)
    6: (151, 157), # Bit-map Section (6 bytes)
    7: (157, 159), # Data Section (2 bytes)
}

os.makedirs('/tmp/gribtest', exist_ok=True)

def update_length(data):
    """Update total length in GRIB header."""
    result = bytearray(data)
    result[8:12] = struct.pack('>I', len(result))
    return bytes(result)

# Create variants
variants = []

# Variant 1: Original (control)
variants.append(('original', data))

# Variant 2: No Section 4
v2 = bytearray()
v2.extend(data[0:109])  # Sections 0, 1, 3
v2.extend(data[131:])   # Sections 5, 6, 7
v2 = update_length(v2)
variants.append(('no_section4', v2))

# Variant 3: No Section 5
v3 = bytearray()
v3.extend(data[0:131])  # Sections 0, 1, 3, 4
v3.extend(data[151:])   # Sections 6, 7
v3 = update_length(v3)
variants.append(('no_section5', v3))

# Variant 4: No Section 6
v4 = bytearray()
v4.extend(data[0:151])  # Sections 0, 1, 3, 4, 5
v4.extend(data[157:])    # Section 7
v4 = update_length(v4)
variants.append(('no_section6', v4))

# Variant 5: No Section 7
v5 = bytearray()
v5.extend(data[0:157])  # Sections 0, 1, 3, 4, 5, 6
v5 = update_length(v5)
variants.append(('no_section7', v5))

# Variant 6: Minimal (Sections 0, 1, 3 only) + Section 8 end marker
v6 = bytearray()
v6.extend(data[0:109])  # Sections 0, 1, 3
v6.extend(b'\x77\x77\x00\x00')  # Section 8
v6 = update_length(v6)
variants.append(('minimal_013', v6))

# Variant 7: Fixed Section 3 length
v7 = bytearray(data)
# Section 3 claims 72 bytes (0x48) at offset 37, actually has 72 bytes
# But we need to check if the bug is about claimed vs available
# Let's check the hex at offset 37:
print(f"Section 3 header bytes: {data[37:41].hex()}")
print(f"Section 3 claimed length: {struct.unpack('>I', data[37:41])[0]}")
print(f"Section 3 actual bytes: {109-37}")
# The bug might be that the template WITHIN Section 3 claims more data
variants.append(('fixed_section3', v7))

# Write all variants
for name, variant_data in variants:
    path = f'/tmp/gribtest/{name}.grib2'
    with open(path, 'wb') as f:
        f.write(variant_data)
    print(f"Created {name}: {len(variant_data)} bytes")

print()
print("=== Test Commands ===")
print()
print("To test each variant:")
print("  cargo run --quiet --bin gribtract -- decode /tmp/gribtest/VARIANT.grib2")
print()
print("Expected results:")
print("  original:        BUFFER UNDERRUN ✓")
print("  no_section4:     BUFFER UNDERRUN ✓ (Section 4 non-essential)")
print("  no_section5:     BUFFER UNDERRUN ✓ (Section 5 non-essential)")
print("  no_section6:     BUFFER UNDERRUN ✓ (Section 6 non-essential)")
print("  no_section7:     BUFFER UNDERRUN ✓ (Section 7 non-essential)")
print("  minimal_013:     BUFFER UNDERRUN ✓ (Section 3 is THE TRIGGER)")
print("  fixed_section3:  NO UNDERRUN or different error")
