#!/usr/bin/env python3
"""
Analyze the GRIB2 structure and create test variants manually.
"""

import struct
import os

# Read the file
with open('/home/coding/gribtract/crates/gribtract/tests/corpus/small/minimal_buffer_underrun.grib2', 'rb') as f:
    data = f.read()

print(f"File size: {len(data)} bytes")
print(f"Hex dump: {data.hex()}")
print()

# Parse sections
offset = 0
sections = []

# Section 0 (Indicator) - fixed 16 bytes
if data[0:4] == b'GRIB':
    total_length = struct.unpack('>I', data[8:12])[0]
    print(f"Section 0 (Indicator): bytes 0-16 (16 bytes)")
    print(f"  Total length claimed: {total_length} bytes")
    print(f"  Edition: {data[7]}")
    sections.append((0, 0, 16))
    offset = 16

# Parse remaining sections
while offset < len(data):
    if offset + 5 > len(data):
        break

    section_len = struct.unpack('>I', data[offset:offset+4])[0]
    section_num = data[offset + 4]

    print(f"Section {section_num}: bytes {offset}-{offset+section_len} ({section_len} bytes)")

    sections.append((section_num, offset, min(offset+section_len, len(data))))
    offset += section_len

    if section_num == 8:
        break

print()
print("Section breakdown:")
for num, start, end in sections:
    print(f"  Section {num}: {start:3d}-{end:3d} ({end-start:3d} bytes)")

print()
print("Creating test variants:")

# Create test directory
os.makedirs('/tmp/gribtest', exist_ok=True)

# Variant 1: Original
with open('/tmp/gribtest/original.grib2', 'wb') as f:
    f.write(data)
print(f"  original.grib2: {len(data)} bytes")

# Variant 2: Remove Section 4 (bytes 109-131)
v2 = bytearray()
for num, start, end in sections:
    if num != 4:
        v2.extend(data[start:end])
# Update total length
v2[8:12] = struct.pack('>I', len(v2))
with open('/tmp/gribtest/no_section4.grib2', 'wb') as f:
    f.write(v2)
print(f"  no_section4.grib2: {len(v2)} bytes (removed Section 4)")

# Variant 3: Remove Section 5 (bytes 131-151)
v3 = bytearray()
for num, start, end in sections:
    if num != 5:
        v3.extend(data[start:end])
v3[8:12] = struct.pack('>I', len(v3))
with open('/tmp/gribtest/no_section5.grib2', 'wb') as f:
    f.write(v3)
print(f"  no_section5.grib2: {len(v3)} bytes (removed Section 5)")

# Variant 4: Remove Section 6 (bytes 151-157)
v4 = bytearray()
for num, start, end in sections:
    if num != 6:
        v4.extend(data[start:end])
v4[8:12] = struct.pack('>I', len(v4))
with open('/tmp/gribtest/no_section6.grib2', 'wb') as f:
    f.write(v4)
print(f"  no_section6.grib2: {len(v4)} bytes (removed Section 6)")

# Variant 5: Remove Section 7 (bytes 157-159)
v5 = bytearray()
for num, start, end in sections:
    if num != 7:
        v5.extend(data[start:end])
v5[8:12] = struct.pack('>I', len(v5))
with open('/tmp/gribtest/no_section7.grib2', 'wb') as f:
    f.write(v5)
print(f"  no_section7.grib2: {len(v5)} bytes (removed Section 7)")

# Variant 6: Minimal - only sections 0, 1, 3 + end marker
v6 = bytearray()
for num, start, end in sections:
    if num in [0, 1, 3]:
        v6.extend(data[start:end])
# Add Section 8 (end section)
v6.extend(b'\x77\x77\x00\x00')
v6[8:12] = struct.pack('>I', len(v6))
with open('/tmp/gribtest/minimal_013.grib2', 'wb') as f:
    f.write(v6)
print(f"  minimal_013.grib2: {len(v6)} bytes (only sections 0, 1, 3)")

# Variant 7: Fix Section 3 length to actual
v7 = bytearray(data)
# Find Section 3
for num, start, end in sections:
    if num == 3:
        actual_len = end - start
        v7[start:start+4] = struct.pack('>I', actual_len)
        print(f"  Fixed Section 3 length from {struct.unpack('>I', data[start:start+4])[0]} to {actual_len}")
        break
v7[8:12] = struct.pack('>I', len(v7))
with open('/tmp/gribtest/fixed_section3.grib2', 'wb') as f:
    f.write(v7)
print(f"  fixed_section3.grib2: {len(v7)} bytes")

print()
print("Test files created in /tmp/gribtest/")
print()
print("=== EXPECTED RESULTS ===")
print()
print("Original file:")
print("  - Should trigger buffer underrun (TooShort error)")
print()
print("Files without sections 4, 5, 6, 7:")
print("  - Should STILL trigger buffer underrun")
print("  - These sections are NON-ESSENTIAL")
print()
print("Minimal file (sections 0, 1, 3 only):")
print("  - Should STILL trigger buffer underrun")
print("  - Proves Section 3 is THE TRIGGER")
print()
print("Fixed Section 3 length:")
print("  - Should NOT trigger buffer underrun")
print("  - Either decodes successfully or produces different error")
print("  - Confirms the length mismatch is the ROOT CAUSE")
