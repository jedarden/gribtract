#!/usr/bin/env python3
"""
Generate a minimal valid GRIB2 fixture using DRT=41 (template 5.41, PNG data
compression) for gribtract testing.

Message: 2m temperature analysis, 3x2 lat/lon grid (0-20°N, 0-20°E, 10° spacing),
PNG-compressed section 7, no bitmap.

Packing: R=100.0, E=0, D=0, N=8 (8-bit grayscale PNG).
Data values X = [0..5], decoded Y = 100.0 + X (Kelvin-ish stand-in).
"""

import struct
import hashlib
import json
import os
import zlib

def u8(v):   return struct.pack('>B', v & 0xFF)
def u16(v):  return struct.pack('>H', v & 0xFFFF)
def u32(v):  return struct.pack('>I', v & 0xFFFFFFFF)
def u64(v):  return struct.pack('>Q', v & 0xFFFFFFFFFFFFFFFF)
def f32(v):  return struct.pack('>f', v)

def grib_i16(v):
    if v < 0:
        return u16(0x8000 | (-v))
    return u16(v)

def section(num, body):
    length = 5 + len(body)
    return u32(length) + u8(num) + body

def png_chunk(chunk_type, data):
    crc = zlib.crc32(chunk_type + data) & 0xFFFFFFFF
    return struct.pack('>I', len(data)) + chunk_type + data + struct.pack('>I', crc)

def make_grayscale_png(width, height, pixels):
    """Build a minimal 8-bit grayscale PNG from a flat list of pixel values."""
    # PNG signature
    sig = b'\x89PNG\r\n\x1a\n'

    # IHDR: width, height, bit_depth=8, color_type=0 (grayscale),
    #       compression=0, filter=0, interlace=0
    ihdr_data = struct.pack('>II', width, height) + bytes([8, 0, 0, 0, 0])
    ihdr = png_chunk(b'IHDR', ihdr_data)

    # IDAT: each scanline prefixed with filter byte 0x00 (no filter)
    raw = b''
    for row in range(height):
        raw += b'\x00'  # filter type: None
        raw += bytes(pixels[row * width:(row + 1) * width])
    idat = png_chunk(b'IDAT', zlib.compress(raw, level=9))

    # IEND
    iend = png_chunk(b'IEND', b'')

    return sig + ihdr + idat + iend

# ── Grid: 3 wide x 2 tall ─────────────────────────────────────────────────────
NI = 3
NJ = 2
N_POINTS = NI * NJ  # 6

# ── Section 1: Identification ─────────────────────────────────────────────────
s1_body = (
    u16(7)    +  # center: US NCEP
    u16(0)    +  # subcenter
    u8(2)     +  # master tables version
    u8(0)     +  # local tables version
    u8(0)     +  # significance: 0 = analysis
    u16(2026) +  # year
    u8(6)     +  # month
    u8(21)    +  # day
    u8(0)     +  # hour
    u8(0)     +  # minute
    u8(0)     +  # second
    u8(0)     +  # production status
    u8(0)        # type of data
)
sec1 = section(1, s1_body)
assert len(sec1) == 21

# ── Section 3: Grid Definition (3x2 lat/lon) ──────────────────────────────────
s3_body = (
    u8(0)           +   # source: defined by template
    u32(N_POINTS)   +   # number of data points
    u8(0)           +   # optional list octets count
    u8(0)           +   # interpretation of optional list
    u16(0)          +   # GDT 3.0: lat/lon
    # Template 3.0:
    u8(6)           +   # shape of Earth: WGS84
    u8(0) + u32(0)  +   # spherical radius scale + value
    u8(0) + u32(0)  +   # major axis scale + value
    u8(0) + u32(0)  +   # minor axis scale + value
    u32(NI)         +   # Ni
    u32(NJ)         +   # Nj
    u32(0)          +   # basic angle
    u32(0xFFFFFFFF) +   # subdivisions of basic angle
    u32(10_000_000) +   # La1: 10°N
    u32(0)          +   # Lo1: 0°E
    u8(0x30)        +   # resolution flags: Δi and Δj given
    u32(0)          +   # La2: 0°N
    u32(20_000_000) +   # Lo2: 20°E
    u32(10_000_000) +   # Di: 10°
    u32(10_000_000) +   # Dj: 10°
    u8(0x00)            # scanning mode: +i, -j
)
sec3 = section(3, s3_body)
assert len(sec3) == 72

# ── Section 4: Product Definition ────────────────────────────────────────────
s4_body = (
    u16(0)   +   # number of coordinate values
    u16(0)   +   # PDT 4.0
    u8(0)    +   # category: temperature
    u8(0)    +   # number: temperature [K]
    u8(0)    +   # generating process: analysis
    u8(255)  +   # background process id: missing
    u8(255)  +   # forecast gen. process id: missing
    u16(0)   +   # hours cutoff
    u8(0)    +   # minutes cutoff
    u8(1)    +   # time unit: hour
    u32(0)   +   # forecast time: 0h
    u8(103)  +   # type1: height above ground
    u8(0)    +   # scale factor1
    u32(2)   +   # scaled value1: 2m
    u8(255)  +   # type2: missing
    u8(0)    +   # scale factor2
    u32(0)       # scaled value2
)
sec4 = section(4, s4_body)
assert len(sec4) == 34

# ── Section 5: Data Representation (DRT 5.41 — PNG compression) ───────────────
# Template 5.41 uses the common packing header (same as DRT=0):
# R=100.0, E=0, D=0, N=8, type=0
s5_body = (
    u32(N_POINTS)   +   # number of packed values
    u16(41)         +   # DRT template: 5.41 (PNG compression)
    f32(100.0)      +   # R: reference value
    grib_i16(0)     +   # E: binary scale factor = 0
    grib_i16(0)     +   # D: decimal scale factor = 0
    u8(8)           +   # N: bits per value (8-bit PNG)
    u8(0)               # type of original field: 0 = floating point
)
sec5 = section(5, s5_body)

# ── Section 6: Bit Map (no bitmap) ────────────────────────────────────────────
s6_body = u8(255)
sec6 = section(6, s6_body)
assert len(sec6) == 6

# ── Section 7: PNG data ────────────────────────────────────────────────────────
# Pixel values X = [0, 1, 2, 3, 4, 5] for the 3x2 grid.
# Row 0 (lat=10°N): [0, 1, 2]
# Row 1 (lat= 0°N): [3, 4, 5]
pixels = list(range(N_POINTS))
png_bytes = make_grayscale_png(NI, NJ, pixels)

s7_body = png_bytes
sec7 = section(7, s7_body)

# ── Assemble message ──────────────────────────────────────────────────────────
body = sec1 + sec3 + sec4 + sec5 + sec6 + sec7

sec0_len = 16
end_marker = b'7777'
total_len = sec0_len + len(body) + len(end_marker)

sec0 = (
    b'GRIB'          +
    b'\x00\x00'      +   # reserved
    bytes([0])       +   # discipline: 0 = meteorological
    bytes([2])       +   # edition: 2
    u64(total_len)
)
assert len(sec0) == 16

message = sec0 + body + end_marker
assert len(message) == total_len
print(f"Message total length: {total_len} bytes")

# ── Write fixture ─────────────────────────────────────────────────────────────
script_dir = os.path.dirname(os.path.abspath(__file__))
repo_root = os.path.dirname(script_dir)
out_path = os.path.join(repo_root, 'tests', 'corpus', 'small', 'drt41_png_3x2.grib2')

with open(out_path, 'wb') as f:
    f.write(message)
print(f"Written: {out_path}")

sha256 = hashlib.sha256(message).hexdigest()
print(f"SHA-256: {sha256}")
print(f"Size:    {len(message)} bytes")

# ── Golden (expected decoded values) ─────────────────────────────────────────
# Y = (R + X × 2^E) / 10^D = (100.0 + X × 1) / 1 = 100.0 + X
expected_values = [100.0 + x for x in pixels]

golden = {
    "fixture_id": "drt41_png_3x2",
    "_provenance": (
        "Synthetic 3x2 DRT=41 (PNG data compression). "
        "Generated by scripts/gen_grib2_drt41.py. "
        "Packing: R=100.0, E=0, D=0, N=8 (8-bit grayscale PNG). "
        "Pixels X=[0..5], decoded Y=100+X."
    ),
    "fields": [
        {
            "center": 7,
            "subcenter": 0,
            "parameter": {"discipline": 0, "category": 0, "number": 0},
            "forecast": {
                "reference_time": {
                    "year": 2026, "month": 6, "day": 21,
                    "hour": 0, "minute": 0, "second": 0,
                    "significance": 0
                },
                "time_range_unit": 1,
                "forecast_offset": 0
            },
            "level": {
                "type1": 103, "scale_factor1": 0, "scaled_value1": 2,
                "type2": 255, "scale_factor2": 0, "scaled_value2": 0
            },
            "ensemble": None,
            "grid": {
                "template": 0,
                "num_data_points": 6,
                "nx": 3, "ny": 2,
                "lat_first": 10.0, "lon_first": 0.0,
                "lat_last": 0.0,  "lon_last": 20.0,
                "di": 10.0, "dj": 10.0,
                "scanning_mode": 0,
                "resolution_flags": 48,
                "shape_of_earth": 6
            },
            "values": {"Dense": expected_values},
            "gdt_template": 0,
            "pdt_template": 0,
            "drt_template": 41,
            "packing": {
                "reference_value": 100.0,
                "binary_scale_factor": 0,
                "decimal_scale_factor": 0,
                "bits_per_value": 8,
                "original_field_type": 0
            }
        }
    ]
}

golden_path = os.path.join(repo_root, 'tests', 'corpus', 'golden', 'drt41_png_3x2.json')
with open(golden_path, 'w') as f:
    json.dump(golden, f, indent=2)
print(f"Golden:  {golden_path}")

print("\nManifest entry:")
print(json.dumps({
    "id": "drt41_png_3x2",
    "path": "small/drt41_png_3x2.grib2",
    "sha256": sha256,
    "size_bytes": len(message),
    "storage": "inline",
    "provenance": {
        "source": "synthetic",
        "description": (
            f"Minimal GRIB2 DRT=41 fixture: 3x2 lat/lon, PNG compression, "
            f"R=100.0 E=0 D=0 N=8. Pixels X=[0..5] -> Y=[100..105]. "
            f"Section 5 template 5.41 (PNG data compression)."
        ),
        "capture_date": "2026-06-21",
        "generated_by": "scripts/gen_grib2_drt41.py (gribtract project)"
    }
}, indent=2))
