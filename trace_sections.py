import struct

# Read the file
with open('tests/corpus/small/rotated_latlon_gdt1_drt0.grib2', 'rb') as f:
    data = f.read()

print(f"Total file size: {len(data)} bytes (0x{len(data):02x})")

# Parse Section 0
pos = 0
magic = data[pos:pos+4]
print(f"\nSection 0 (Indicator):")
print(f"  Magic: {magic}")
print(f"  Total length: {struct.unpack('>Q', data[4:12])[0]}")

# Parse sections
pos = 16  # Section 0 is 16 bytes
section_num = 0
while pos < len(data) - 4:  # Leave room for "7777"
    sec_len = struct.unpack('>I', data[pos:pos+4])[0]
    sec_num = data[pos+4]
    section_num += 1
    
    print(f"\nSection {sec_num} (at pos {pos}):")
    print(f"  Length: {sec_len} bytes")
    print(f"  Section number: {sec_num}")
    
    if sec_num == 3:
        print(f"  Section 3 body (first 32 bytes):")
        body_start = pos + 5
        for i in range(0, min(32, sec_len - 5), 4):
            chunk = data[body_start + i:body_start + i + 4]
            print(f"    Offset {i:2d}: {chunk.hex()}")
    
    pos += sec_len
    
print(f"\nEnd marker: {data[pos:pos+4]}")
