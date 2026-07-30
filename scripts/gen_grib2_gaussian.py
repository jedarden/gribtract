#!/usr/bin/env python3
"""
Generate a minimal valid GRIB2 fixture for GFS Gaussian-grid (GDT 3.40) testing.

Message: 2m temperature analysis on a small synthetic Gaussian Latitude/Longitude
grid, PDT 4.0 (analysis/forecast), DRT 5.0 (simple packing), no bitmap.

Gaussian grids (GDT 3.40) share the section-3 layout of GDT 3.0 (lat/lon) except
that octets 68-71 carry N — the number of parallels between pole and equator —
in place of the latitude increment Dj. Longitude spacing (Di) is uniform; latitude
spacing is non-uniform (Gaussian quadrature). gribtract stores lat_first/lat_last
verbatim and leaves dj = 0.0, modeling only N via GaussianLatLonParams.

Grid (synthetic, N=4): nx = 4N = 16 longitudes (di = 22.5°), ny = 2N = 8 latitudes
(±80° bounds). 128 data points. This is the small/inline analogue of the large
remote `core_gaussian_gdt40` (T254) and `gfs_gaussian_gdt40_t1534` (T1534)
fixtures, sized for committed differential testing.

All GRIB2 integers are big-endian (network byte order).
"""

import struct
import hashlib
import json

def u8(v):   return struct.pack('>B', v & 0xFF)
def u16(v):  return struct.pack('>H', v & 0xFFFF)
def u32(v):  return struct.pack('>I', v & 0xFFFFFFFF)
def i16(v):  return struct.pack('>h', v)
def f32(v):  return struct.pack('>f', v)

def latlon_micro(deg):
    """Encode a latitude/longitude in micro-degrees using GRIB2 sign-magnitude:
    bit 31 = sign (1 = S/W), bits 30-0 = magnitude. (NOT two's complement.)
    This matches gribtract's `read_latlon_micro` decoder."""
    magnitude = int(round(abs(deg) * 1_000_000)) & 0x7FFF_FFFF
    if deg < 0:
        magnitude |= 0x8000_0000
    return u32(magnitude)

def section(num, body):
    """Wrap body bytes in a GRIB2 section with 4-byte length header."""
    length = 5 + len(body)
    return u32(length) + u8(num) + body

# ── Section 1: Identification (21 bytes) ─────────────────────────────────────
# Reference time: 2024-06-19T00:00:00Z, analysis (significance=0)
s1_body = (
    u16(7)    +  # originating center: US NCEP
    u16(0)    +  # sub-center: 0
    u8(2)     +  # GRIB Master Tables version
    u8(0)     +  # local tables version (0 = none)
    u8(0)     +  # significance of reference time: 0 = analysis
    u16(2024) +  # year
    u8(6)     +  # month
    u8(19)    +  # day
    u8(0)     +  # hour
    u8(0)     +  # minute
    u8(0)     +  # second
    u8(0)     +  # production status: 0 = operational
    u8(0)        # type of data: 0 = analysis
)
sec1 = section(1, s1_body)
assert len(sec1) == 21, f"section 1 length {len(sec1)}"

# ── Section 3: Grid Definition (72 bytes, template 3.40 = Gaussian lat/lon) ──
# Synthetic N=4 Gaussian grid: nx = 4N = 16, ny = 2N = 8 -> 128 points.
# Longitude: 0°E to 337.5°E, uniform di = 22.5°.
# Latitude: +80°N to -80°N (synthetic near-pole Gaussian bounds; gribtract stores
#   lat_first/lat_last verbatim and approximates interior spacing as uniform).
# Octets 68-71 carry N (parallels pole->equator) instead of Dj.
N_PARALLELS = 4          # N: parallels between pole and equator
NI = 16                  # nx = 4N longitude points
NJ = 8                   # ny = 2N latitude points
N_POINTS = NI * NJ       # 128
s3_body = (
    u8(0)             +   # source: 0 = defined by template
    u32(N_POINTS)     +   # number of data points
    u8(0)             +   # num octets for optional list
    u8(0)             +   # interpretation of optional list
    u16(40)           +   # GDT number: 40 = Gaussian Latitude/Longitude
    # Template 3.40 content:
    u8(6)             +   # shape of Earth: 6 = WGS84
    u8(0)             +   # scale factor of Earth radius (unused for WGS84)
    u32(0)            +   # scaled value of Earth radius (unused)
    u8(0)             +   # scale factor of major axis
    u32(0)            +   # scaled value of major axis (m, unused for WGS84)
    u8(0)             +   # scale factor of minor axis
    u32(0)            +   # scaled value of minor axis (m, unused for WGS84)
    u32(NI)           +   # Ni (number of points along a parallel)
    u32(NJ)           +   # Nj (number of points along a meridian)
    u32(0)            +   # basic angle (0 = use degrees)
    u32(0xFFFFFFFF)   +   # subdivisions of basic angle (missing)
    latlon_micro(80)   +  # La1: 80°N (first/northernmost latitude)
    u32(0)             +  # Lo1: 0°E (first longitude)
    u8(0x30)           +  # resolution/component flags: Δi and Δj increments given
    latlon_micro(-80)  +  # La2: -80°N (last/southernmost latitude, sign-magnitude — matches read_latlon_micro)
    u32(337_500_000)   +  # Lo2: 337.5°E (last longitude)
    u32(22_500_000)    +  # Di: 22.5° longitude increment (uniform)
    u32(N_PARALLELS)  +   # N: number of parallels pole->equator (replaces Dj)
    u8(0x00)              # scanning mode: +i, -j (north→south), no alternating
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

# ── Section 7: Data ───────────────────────────────────────────────────────────
# 128 temperature values: X[i] = i (for i = 0..127)
# Actual value[i] = REF_VALUE + X[i] = 270.0 + i  ->  270.0K .. 397.0K.
packed_data = bytes(range(N_POINTS))  # X = 0,1,2,...,127
s7_body = packed_data
sec7 = section(7, s7_body)

# ── Section 0: Indicator (16 bytes) ──────────────────────────────────────────
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
out_path = '/home/coding/gribtract/tests/corpus/small/gfs_gaussian_gdt40_drt0.grib2'
with open(out_path, 'wb') as f:
    f.write(msg)
print(f"Written: {out_path}")

# Produce manifest entry
manifest_entry = {
    "id": "gfs_gaussian_gdt40_drt0",
    "path": "small/gfs_gaussian_gdt40_drt0.grib2",
    "sha256": sha256,
    "size_bytes": len(msg),
    "storage": "inline",
    "provenance": {
        "source": "synthetic",
        "description": (
            "Minimal GRIB2 fixture: 2m temperature analysis on a synthetic GFS "
            "Gaussian Latitude/Longitude grid (GDT 3.40). "
            "Grid: N=4 (parallels pole->equator), Ni=16 (4N longitudes, di=22.5°), "
            "Nj=8 (2N latitudes), 128 points. Lat bounds +80°N to -80°N, lon 0°E to 337.5°E. "
            "Section 0 discipline=0 (meteorological), edition=2. "
            "Section 1: center=7 (US NCEP), analysis (significance=0), 2024-06-19T00:00:00Z. "
            "Section 3 template 3.40 (Gaussian lat/lon): shape=6 (WGS84), Ni=16, Nj=8, "
            "scanning_mode=0x00 (+i, -j), octets 68-71 carry N=4 (replaces Dj; "
            "latitude spacing is non-uniform Gaussian quadrature). "
            "Section 4 template 4.0: discipline=0/cat=0/num=0 (temperature K), "
            "level type=103 (height above ground), value=2m, forecast_offset=0h. "
            "Section 5 template 5.0 (simple packing): R=270.0, E=0, D=0, nbits=8. "
            "Section 6: no bitmap (indicator=255). "
            "Section 7: 128 packed bytes X=0..127, unpacked values = 270.0+X K. "
            "Small/inline analogue of the large remote core_gaussian_gdt40 (T254) and "
            "gfs_gaussian_gdt40_t1534 (T1534) Gaussian fixtures."
        ),
        "capture_date": "2026-07-26",
        "generated_by": "scripts/gen_grib2_gaussian.py (gribtract project)"
    }
}
print(json.dumps(manifest_entry, indent=2))
