import struct

with open('tests/corpus/small/rotated_latlon_gdt1_drt0.grib2', 'rb') as f:
    data = f.read()

# Section 3 starts at position 37 (0x25)
sec3_start = 37
sec3_len = struct.unpack('>I', data[sec3_start:sec3_start+4])[0]
sec3_num = data[sec3_start+4]
body_start = sec3_start + 5
body_len = sec3_len - 5

print(f"Section 3:")
print(f"  Start position: {sec3_start} (0x{sec3_start:02x})")
print(f"  Total length: {sec3_len} bytes")
print(f"  Section number: {sec3_num}")
print(f"  Body length: {body_len} bytes")
print(f"  Body starts at: {body_start} (0x{body_start:02x})")
print(f"\nExpected GDT 3.1 template body: 72 bytes")
print(f"Actual body available: {body_len} bytes")
print(f"Missing bytes: {72 - body_len}")

# Parse what we can
body = data[body_start:body_start+body_len]
print(f"\nBody bytes (first {min(len(body), 72)}):")
for i in range(0, min(len(body), 72), 8):
    chunk = body[i:i+8]
    print(f"  Offset {i:2d}: {' '.join(f'{b:02x}' for b in chunk)}")

# Try to parse according to GDT 3.1 spec
pos = 0
print(f"\nParsing GDT 3.1 fields:")
try:
    print(f"  oct 15 (pos {pos}): shape_of_earth = {body[pos]}")
    pos += 1
    
    # Skip 5+5+5 = 15 bytes (3 fields of scale+value)
    pos += 15
    
    if pos + 8 <= len(body):
        nx = struct.unpack('>I', body[pos:pos+4])[0]
        ny = struct.unpack('>I', body[pos+4:pos+8])[0]
        print(f"  oct 31-34 (pos {pos}): Nx = {nx}")
        print(f"  oct 35-38 (pos {pos+4}): Ny = {ny}")
        pos += 8
    else:
        print(f"  ERROR: Cannot read Nx/Ny at pos {pos}, need 8 bytes, have {len(body)-pos}")
        
except Exception as e:
    print(f"  Error: {e}")

print(f"\nFinal position in body: {pos}")
print(f"Remaining bytes in body: {len(body) - pos}")
