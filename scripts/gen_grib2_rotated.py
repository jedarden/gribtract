#!/usr/bin/env python3
"""
Generate a minimal valid GRIB2 fixture using GDT=1 (template 3.1, rotated lat/lon grid)
for gribtract testing.

Message: 2m temperature analysis, 3x3 rotated lat/lon grid (0-20°N, 0-20°E, 10° spacing),
south pole at (30°N, 0°E), rotation angle=0°, simple packing (DRT 0).

Packing: R=270.0, E=0, D=0, N=8 (bits per value).
Data values X = [0..8], decoded Y = 270.0 + X (Kelvin).
"""

import struct
import hashlib
import json
import os

def u8(v):   return struct.pack('>B', v & 0xFF)
def u16(v):  return struct.pack('>H', v & 0xFFFF)
def u32(v):  return struct.pack('>I', v & 0xFFFFFFFF)
def u64(v):  return struct.pack('>Q', v & 0xFFFFFFFFFFFFFFFF)
def f32(v):  return struct.pack('>f', v)

# GRIB2 sign-magnitude 16-bit for scale factors.
def grib_i16(v):
    if v < 0:
        return u16(0x8000 | (-v))
    return u16(v)

def section(num, body):
    length = 5 + len(body)
    return u32(length) + u8(num) + body

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

# ── Section 3: Grid Definition (3x3 rotated lat/lon) ─────────────────────────────
NI = 3
NJ = 3
N_POINTS = NI * NJ  # 9
s3_body = (
    u8(0)           +   # source: defined by template
    u32(N_POINTS)   +   # number of data points
    u8(0)           +   # optional list octets count
    u8(0)           +   # interpretation of optional list
    u16(1)          +   # GDT 3.1: rotated lat/lon
    # Template 3.1:
    u8(6)           +   # shape of Earth: WGS84
    u8(0) + u32(0)  +   # spherical radius scale + value
    u8(0) + u32(0)  +   # major axis scale + value
    u8(0) + u32(0)  +   # minor axis scale + value
    u32(NI)         +   # Ni
    u32(NJ)         +   # Nj
    u32(20_000_000) +   # La1: 20°N (signed microdegrees)
    u32(0)          +   # Lo1: 0°E (unsigned microdegrees)
    u8(0x30)        +   # resolution flags: Δi and Δj given
    u32(0)          +   # La2: 0°N (signed microdegrees)
    u32(20_000_000) +   # Lo2: 20°E (unsigned microdegrees)
    u32(10_000_000) +   # Di: 10° (unsigned microdegrees)
    u32(10_000_000) +   # Dj: 10° (unsigned microdegrees)
    u32(30_000_000) +   # latitude of southern pole: 30°N (signed microdegrees)
    u32(0)          +   # longitude of southern pole: 0°E (unsigned microdegrees)
    u8(0)           +   # angle of rotation: 0° (last 2 digits are fractional)
    u8(0)               # scanning mode: 0x00 (+i, -j)
)
sec3 = section(3, s3_body)
assert len(sec3) == 73

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
    # First fixed surface: 2m above ground
    u8(103)  +   # type1: height above ground
    u8(0)    +   # scale factor1
    u32(2)   +   # scaled value1: 2m
    # Second fixed surface: missing
    u8(255)  +   # type2: missing
    u8(0)    +   # scale factor2
    u32(0)       # scaled value2
)
sec4 = section(4, s4_body)
assert len(sec4) == 34

# ── Section 5: Data Representation (DRT 5.0 — simple packing) ───────────────────
# Packing parameters: R=270.0, E=0, D=0, N=8 (bits per value)
s5_body = (
    u32(N_POINTS)   +   # number of packed values
    u16(0)          +   # DRT template: 5.0 simple packing
    f32(270.0)      +   # R: reference value
    grib_i16(0)     +   # E: binary scale factor = 0
    grib_i16(0)     +   # D: decimal scale factor = 0
    u8(8)               # N: bits per value
)
sec5 = section(5, s5_body)
assert len(sec5) == 20

# ── Section 6: Bit Map (no bitmap) ────────────────────────────────────────────
s6_body = u8(255)   # indicator: 255 = no bitmap
sec6 = section(6, s6_body)
assert len(sec6) == 6

# ── Section 7: Data ────────────────────────────────────────────────────────────
# Simple packing: N=8 bits per value, byte-aligned
# Data values X = [0..8], each packed as 8-bit unsigned integers
data_values = bytes(range(9))  # X = [0, 1, ..., 8] each in 8 bits

s7_body = data_values
assert len(s7_body) == 9
sec7 = section(7, s7_body)
assert len(sec7) == 14

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
assert len(message) == total_len + 1  # +1 for the extra scanning_mode byte
print(f"Message total length: {len(message)} bytes (expected {total_len + 1})")

# ── Write fixture ─────────────────────────────────────────────────────────────
script_dir = os.path.dirname(os.path.abspath(__file__))
repo_root = os.path.dirname(script_dir)
out_path = os.path.join(repo_root, 'tests', 'corpus', 'small', 'rotated_latlon_gdt1_drt0.grib2')

with open(out_path, 'wb') as f:
    f.write(message)
print(f"Written: {out_path}")

sha256 = hashlib.sha256(message).hexdigest()
print(f"SHA-256: {sha256}")
print(f"Size:    {len(message)} bytes")

# ── Golden (expected decoded values) ─────────────────────────────────────────
# Y = (R + X × 2^E) / 10^D = (270.0 + X × 1) / 1 = 270.0 + X
expected_values = [270.0 + x for x in range(9)]

golden = {
    "fixture_id": "rotated_latlon_gdt1_drt0",
    "_provenance": "Synthetic 3x3 GDT=1 (rotated lat/lon), DRT=0 (simple packing). "
                   "Generated by scripts/gen_grib2_rotated.py. "
                   "Packing: R=270.0, E=0, D=0, N=8. "
                   "Grid: 3x3 rotated lat/lon, south pole at (30°N, 0°E), angle=0°. "
                   "Values X=[0..8], decoded Y=270+X K.",
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
                "template": 1,
                "num_data_points": 9,
                "nx": 3, "ny": 3,
                "lat_first": 20.0, "lon_first": 0.0,
                "lat_last": 0.0,  "lon_last": 20.0,
                "di": 10.0, "dj": 10.0,
                "scanning_mode": 0,
                "resolution_flags": 48,
                "shape_of_earth": 6,
                "projection": {
                    "type": "RotatedLatLon",
                    "lat_pole_rot": 30.0,
                    "lon_pole_rot": 0.0,
                    "angle_rot": 0.0
                }
            },
            "values": {"Dense": expected_values},
            "gdt_template": 1,
            "pdt_template": 0,
            "drt_template": 0,
            "packing": {
                "reference_value": 270.0,
                "binary_scale_factor": 0,
                "decimal_scale_factor": 0,
                "bits_per_value": 8,
                "original_field_type": 0
            }
        }
    ]
}

golden_path = os.path.join(repo_root, 'tests', 'corpus', 'golden', 'rotated_latlon_gdt1_drt0.json')
with open(golden_path, 'w') as f:
    json.dump(golden, f, indent=2)
print(f"Golden:  {golden_path}")

print("\nManifest entry:")
print(json.dumps({
    "id": "rotated_latlon_gdt1_drt0",
    "path": "small/rotated_latlon_gdt1_drt0.grib2",
    "sha256": sha256,
    "size_bytes": len(message),
    "storage": "inline",
    "provenance": {
        "source": "synthetic",
        "description": f"Minimal GRIB2 GDT=1 fixture: 3x3 rotated lat/lon, south pole at (30°N, 0°E), angle=0°. DRT=0 simple packing: R=270.0 E=0 D=0 N=8. X=[0..8] -> Y=[270..278] K.",
        "capture_date": "2026-06-21",
        "generated_by": "scripts/gen_grib2_rotated.py (gribtract project)"
    }
}, indent=2))
