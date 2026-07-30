#!/usr/bin/env python3
"""
Generate a minimal GRIB2 fixture with CONUS coverage and DRT=0 simple packing.

Message: 2m temperature analysis, CONUS lat/lon grid (20-55°N, 125-65°W),
simple packing (DRT 5.0), no bitmap.

CONUS approximate bounds:
- Latitude: 20°N to 55°N
- Longitude: 125°W to 65°W (235°E to 295°E in 0-360 convention)
"""

import struct
import hashlib
import json
import os

def u8(v):   return struct.pack('>B', v & 0xFF)
def u16(v):  return struct.pack('>H', v & 0xFFFF)
def u32(v):  return struct.pack('>I', v & 0xFFFFFFFF)
def i16(v):  return struct.pack('>h', v)
def f32(v):  return struct.pack('>f', v)

def section(num, body):
    """Wrap body bytes in a GRIB2 section with 4-byte length header."""
    length = 5 + len(body)
    return u32(length) + u8(num) + body

# CONUS grid parameters
LAT_START = 55  # 55°N
LAT_END = 20    # 20°N
LON_START = 235  # 125°W = 235°E
LON_END = 295    # 65°W = 295°E
LAT_SPACING = 5  # 5° spacing
LON_SPACING = 5  # 5° spacing

# Calculate grid dimensions
NI = (LON_END - LON_START) // LON_SPACING + 1  # 13 points
NJ = (LAT_START - LAT_END) // LAT_SPACING + 1   # 8 points
N_POINTS = NI * NJ  # 104 points

# ── Section 1: Identification (21 bytes) ─────────────────────────────────────
# Reference time: 2024-01-15T00:00:00Z, analysis (significance=0)
s1_body = (
    u16(7)    +  # originating center: US NCEP
    u16(0)    +  # sub-center: 0
    u8(2)     +  # GRIB Master Tables version
    u8(0)     +  # local tables version (0 = none)
    u8(0)     +  # significance of reference time: 0 = analysis
    u16(2024) +  # year
    u8(1)     +  # month
    u8(15)    +  # day
    u8(0)     +  # hour
    u8(0)     +  # minute
    u8(0)     +  # second
    u8(0)     +  # production status: 0 = operational
    u8(0)        # type of data: 0 = analysis
)
sec1 = section(1, s1_body)
assert len(sec1) == 21, f"section 1 length {len(sec1)}"

# ── Section 3: Grid Definition (72 bytes, template 3.0 = lat/lon) ────────────
# Grid: 55°N to 20°N, 235°E to 295°E (125°W to 65°W), 5° spacing.
# Angles stored in millionths of a degree (× 10^6).
# scanning_mode=0x00: +i (west→east), -j (north→south), no alternating.
s3_body = (
    u8(0)           +   # source: 0 = defined by template
    u32(N_POINTS)   +   # number of data points
    u8(0)           +   # num octets for optional list
    u8(0)           +   # interpretation of optional list
    u16(0)          +   # GDT number: 0 = lat/lon
    # Template 3.0 content:
    u8(6)           +   # shape of Earth: 6 = WGS84
    u8(0)           +   # scale factor of Earth radius (unused for WGS84)
    u32(0)          +   # scaled value of Earth radius (unused)
    u8(0)           +   # scale factor of major axis
    u32(0)          +   # scaled value of major axis (m, unused for WGS84)
    u8(0)           +   # scale factor of minor axis
    u32(0)          +   # scaled value of minor axis (m, unused for WGS84)
    u32(NI)         +   # Ni
    u32(NJ)         +   # Nj
    u32(0)          +   # basic angle (0 = use degrees)
    u32(0xFFFFFFFF) +   # subdivisions of basic angle (missing)
    u32(LAT_START * 1_000_000) +   # La1: 55°N = 55,000,000 micro-degrees
    u32(LON_START * 1_000_000) +   # Lo1: 235°E (125°W)
    u8(0x30)        +   # resolution/component flags: Δi and Δj increments given
    u32(LAT_END * 1_000_000) +   # La2: 20°N (south boundary)
    u32(LON_END * 1_000_000) +   # Lo2: 295°E (65°W)
    u32(LON_SPACING * 1_000_000) +   # Di: 5° increment
    u32(LAT_SPACING * 1_000_000) +   # Dj: 5° increment
    u8(0x00)            # scanning mode: +i, -j (north→south), no alternating
)
sec3 = section(3, s3_body)
assert len(sec3) == 72, f"section 3 length {len(sec3)}"

# ── Section 4: Product Definition (34 bytes, template 4.0) ───────────────────
# Parameter: temperature (discipline=0, category=0, number=0)
# Level: 2m above ground (type=103, scale=0, value=2)
# Forecast time: analysis (forecast_offset=0 hours)
s4_body = (
    u16(0)          +   # number of coordinate values
    u16(0)          +   # PDT number: 0 = deterministic forecast/analysis
    # Template 4.0 content:
    u8(0)           +   # parameter category: 0 = temperature
    u8(0)           +   # parameter number: 0 = temperature [K]
    u8(0)           +   # type of generating process: 0 = analysis
    u8(255)         +   # background generating process id: 255 = missing
    u8(255)         +   # analysis/forecast gen. process id: 255 = missing
    u16(0)          +   # hours of obs data cutoff after reference time
    u8(0)           +   # minutes of obs data cutoff
    u8(1)           +   # indicator of unit of time range: 1 = hour
    u32(0)          +   # forecast time: 0 (analysis)
    u8(103)         +   # type of first fixed surface: 103 = specified height above ground
    u8(0)           +   # scale factor of first fixed surface
    u32(2)          +   # scaled value of first fixed surface: 2 m above ground
    u8(255)         +   # type of second fixed surface: 255 = missing
    u8(255)         +   # scale factor of second (missing)
    u32(0xFFFFFFFF)     # scaled value of second (missing)
)
sec4 = section(4, s4_body)
assert len(sec4) == 34, f"section 4 length {len(sec4)}"

# ── Section 5: Data Representation (21 bytes, template 5.0 = simple packing) ─
# Packing: reference=270.0 K, binary_scale=0, decimal_scale=0, bits=8
# Stored value X gives: actual = (270.0 + X * 2^0) / 10^0 = 270.0 + X
REF_VALUE = 270.0
BINARY_SCALE = 0
DECIMAL_SCALE = 0
BITS_PER_VALUE = 8
s5_body = (
    u32(N_POINTS)     +   # number of packed values
    u16(0)            +   # DRT number: 0 = simple packing
    f32(REF_VALUE)    +   # reference value R (IEEE float32)
    i16(BINARY_SCALE) +   # binary scale factor E
    i16(DECIMAL_SCALE)+   # decimal scale factor D
    u8(BITS_PER_VALUE)+   # bits per value
    u8(0)                 # type of original field values: 0 = float
)
sec5 = section(5, s5_body)
assert len(sec5) == 21, f"section 5 length {len(sec5)}"

# ── Section 6: Bitmap (6 bytes, no bitmap) ────────────────────────────────────
s6_body = u8(255)  # bitmap indicator: 255 = no bitmap
sec6 = section(6, s6_body)
assert len(sec6) == 6, f"section 6 length {len(sec6)}"

# ── Section 7: Data (5 + 104 = 109 bytes) ──────────────────────────────────────
# 104 temperature values: X[i] = i (for i = 0..103)
# Actual value[i] = REF_VALUE + X[i] = 270.0 + i
# So data[0]=270K (55°N 125°W), ..., data[103]=373K (20°N 65°W).
packed_data = bytes(range(N_POINTS))  # X = 0,1,2,...,103
s7_body = packed_data
sec7 = section(7, s7_body)
assert len(sec7) == 109, f"section 7 length {len(sec7)}"

# ── Section 0: Indicator (16 bytes) ──────────────────────────────────────────
# Must know total size first.
END = b'7777'
total = 16 + len(sec1) + len(sec3) + len(sec4) + len(sec5) + len(sec6) + len(sec7) + 4
sec0 = (
    b'GRIB'           +  # magic
    u8(0) + u8(0)     +  # reserved
    u8(0)             +  # discipline: 0 = meteorological
    u8(2)             +  # GRIB edition: 2
    struct.pack('>Q', total)  # total message length (8 bytes, uint64)
)
assert len(sec0) == 16, f"section 0 length {len(sec0)}"

# Assemble message
msg = sec0 + sec1 + sec3 + sec4 + sec5 + sec6 + sec7 + END
assert len(msg) == total, f"total length mismatch: {len(msg)} != {total}"
print(f"Message length: {len(msg)} bytes")

# Verify magic
assert msg[:4] == b'GRIB', "missing GRIB magic"
assert msg[-4:] == b'7777', "missing 7777 end marker"

# Compute SHA-256
sha256 = hashlib.sha256(msg).hexdigest()
print(f"SHA-256: {sha256}")

# Write fixture
out_path = '/home/coding/gribtract/tests/corpus/small/conus_drt0.grib2'
with open(out_path, 'wb') as f:
    f.write(msg)
print(f"Written: {out_path}")

# Produce manifest entry
manifest_entry = {
    "id": "conus_drt0",
    "path": "small/conus_drt0.grib2",
    "sha256": sha256,
    "size_bytes": len(msg),
    "storage": "inline",
    "provenance": {
        "source": "synthetic",
        "description": (
            "Minimal GRIB2 fixture: 2m temperature analysis, "
            f"CONUS lat/lon grid ({LAT_START}-{LAT_END}°N, {LON_START-360}°W to {LON_END-360}°W, "
            f"{LON_SPACING}°x{LAT_SPACING}° spacing, {NI}x{NJ}={N_POINTS} points). "
            "Section 0 discipline=0 (meteorological), edition=2. "
            "Section 1: center=7 (US NCEP), analysis (significance=0), 2024-01-15T00:00:00Z. "
            "Section 3 template 3.0 (lat/lon), shape=6 (WGS84), Ni=13, Nj=8, "
            f"scanning_mode=0x00 (+i, -j), lat bounds {LAT_END}-{LAT_START}°N, "
            f"lon bounds {LON_START-360}-{LON_END-360}°W ({LON_START}-{LON_END}°E). "
            "Section 4 template 4.0: discipline=0/cat=0/num=0 (temperature K), "
            "level type=103 (height above ground), value=2m, forecast_offset=0h. "
            "Section 5 template 5.0 (simple packing, DRT=0): R=270.0, E=0, D=0, nbits=8. "
            "Section 6: no bitmap (indicator=255). "
            f"Section 7: {N_POINTS} packed bytes X=0..{N_POINTS-1}, "
            f"unpacked values = 270.0+X K ({REF_VALUE}K at {LAT_START}°N/{LON_START-360}°W .. "
            f"{REF_VALUE+N_POINTS-1}K at {LAT_END}°N/{LON_END-360}°W)."
        ),
        "capture_date": "2026-07-23",
        "generated_by": "scripts/gen_conus_drt0.py (gribtract project)"
    }
}
print(json.dumps(manifest_entry, indent=2))
