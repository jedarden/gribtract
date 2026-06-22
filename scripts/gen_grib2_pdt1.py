#!/usr/bin/env python3
"""
Generate a minimal valid GRIB2 fixture for PDT 4.1 testing.

Message: 2m temperature ensemble member forecast, 3x2 lat/lon grid,
PDT 4.1 (individual ensemble member), DRT 5.0 (simple packing), no bitmap.

Ensemble: type=2 (negatively perturbed member), perturbation=3, n_ens=20.
Level: 500 hPa isobaric (type=100, scale=0, value=50000).
Forecast offset: 6 hours from reference.
"""

import struct
import hashlib
import json

def u8(v):   return struct.pack('>B', v & 0xFF)
def i8(v):   return struct.pack('>b', v)
def u16(v):  return struct.pack('>H', v & 0xFFFF)
def u32(v):  return struct.pack('>I', v & 0xFFFFFFFF)
def i16(v):  return struct.pack('>h', v)
def f32(v):  return struct.pack('>f', v)

def section(num, body):
    """Wrap body bytes in a GRIB2 section with 4-byte length header."""
    length = 5 + len(body)
    return u32(length) + u8(num) + body

# ── Section 1: Identification ─────────────────────────────────────────────────
# Reference time: 2024-01-15T00:00:00Z, start-of-forecast (significance=1)
s1_body = (
    u16(7)    +  # originating center: US NCEP
    u16(0)    +  # sub-center: 0
    u8(2)     +  # GRIB Master Tables version
    u8(0)     +  # local tables version (0 = none)
    u8(1)     +  # significance of reference time: 1 = start of forecast
    u16(2024) +  # year
    u8(1)     +  # month
    u8(15)    +  # day
    u8(0)     +  # hour
    u8(0)     +  # minute
    u8(0)     +  # second
    u8(0)     +  # production status: 0 = operational
    u8(1)        # type of data: 1 = forecast
)
sec1 = section(1, s1_body)
assert len(sec1) == 21, f"section 1 length {len(sec1)}"

# ── Section 3: Grid Definition (72 bytes, template 3.0 = lat/lon) ────────────
# 3x2 grid: 30°N to 20°N, 0°E to 20°E, 10° spacing.
NI = 3
NJ = 2
N_POINTS = NI * NJ  # 6
s3_body = (
    u8(0)           +   # source: 0 = defined by template
    u32(N_POINTS)   +   # number of data points
    u8(0)           +   # num octets for optional list
    u8(0)           +   # interpretation of optional list
    u16(0)          +   # GDT number: 0 = lat/lon
    u8(6)           +   # shape of Earth: 6 = WGS84
    u8(0)           +   # scale factor of Earth radius (unused for WGS84)
    u32(0)          +   # scaled value of Earth radius (unused)
    u8(0)           +   # scale factor of major axis
    u32(0)          +   # scaled value of major axis
    u8(0)           +   # scale factor of minor axis
    u32(0)          +   # scaled value of minor axis
    u32(NI)         +   # Ni
    u32(NJ)         +   # Nj
    u32(0)          +   # basic angle (0 = use degrees)
    u32(0xFFFFFFFF) +   # subdivisions of basic angle (missing)
    u32(30_000_000) +   # La1: 30°N
    u32(0)          +   # Lo1: 0°E
    u8(0x30)        +   # resolution/component flags
    u32(20_000_000) +   # La2: 20°N
    u32(20_000_000) +   # Lo2: 20°E
    u32(10_000_000) +   # Di: 10°
    u32(10_000_000) +   # Dj: 10°
    u8(0x00)            # scanning mode: +i, -j
)
sec3 = section(3, s3_body)
assert len(sec3) == 72, f"section 3 length {len(sec3)}"

# ── Section 4: Product Definition (template 4.1) ──────────────────────────────
# PDT 4.1: individual ensemble member forecast at horizontal level.
# Octs 10-34: same as PDT 4.0
# Octs 35-37: ensemble-specific fields
#
# Parameter: temperature (discipline=0, cat=0, num=0)
# Level: 500 hPa isobaric (type=100, scale=0, value=50000)
# Forecast offset: 6 hours
# Ensemble: type=2 (negatively perturbed), perturbation=3, n_ens=20
s4_body = (
    u16(0)          +   # number of coordinate values (oct 6-7)
    u16(1)          +   # PDT number: 1 = individual ensemble member (oct 8-9)
    # Template 4.1 content (oct 10+):
    u8(0)           +   # parameter category: 0 = temperature (oct 10)
    u8(0)           +   # parameter number: 0 = temperature [K] (oct 11)
    u8(2)           +   # type of generating process: 2 = forecast (oct 12)
    u8(255)         +   # background generating process id: 255 = missing (oct 13)
    u8(255)         +   # forecast gen. process id: 255 = missing (oct 14)
    u16(0)          +   # hours of obs data cutoff (oct 15-16)
    u8(0)           +   # minutes of obs data cutoff (oct 17)
    u8(1)           +   # indicator of unit of time range: 1 = hour (oct 18)
    u32(6)          +   # forecast time: 6 hours (oct 19-22)
    u8(100)         +   # type of first fixed surface: 100 = isobaric level (oct 23)
    u8(0)           +   # scale factor of first fixed surface (oct 24)
    u32(50000)      +   # scaled value of first fixed surface: 50000 Pa = 500 hPa (oct 25-28)
    u8(255)         +   # type of second fixed surface: 255 = missing (oct 29)
    u8(255)         +   # scale factor of second (missing) (oct 30)
    u32(0xFFFFFFFF) +   # scaled value of second (missing) (oct 31-34)
    # Ensemble-specific tail (oct 35-37):
    u8(2)           +   # type of ensemble: 2 = negatively perturbed (oct 35)
    u8(3)           +   # perturbation number: 3 (oct 36)
    u8(20)              # number of forecasts in ensemble: 20 (oct 37)
)
sec4 = section(4, s4_body)
assert len(sec4) == 37, f"section 4 length {len(sec4)} (expected 37)"

# ── Section 5: Data Representation (template 5.0 = simple packing) ───────────
# Packing: R=250.0 K, binary_scale=0, decimal_scale=0, bits=8
# Stored value X -> actual = 250.0 + X
REF_VALUE = 250.0
BINARY_SCALE = 0
DECIMAL_SCALE = 0
BITS_PER_VALUE = 8
s5_body = (
    u32(N_POINTS)     +   # number of packed values
    u16(0)            +   # DRT number: 0 = simple packing
    f32(REF_VALUE)    +   # reference value R
    i16(BINARY_SCALE) +   # binary scale factor E
    i16(DECIMAL_SCALE)+   # decimal scale factor D
    u8(BITS_PER_VALUE)+   # bits per value
    u8(0)                 # type of original field values: 0 = float
)
sec5 = section(5, s5_body)
assert len(sec5) == 21, f"section 5 length {len(sec5)}"

# ── Section 6: Bitmap (no bitmap) ────────────────────────────────────────────
s6_body = u8(255)  # bitmap indicator: 255 = no bitmap
sec6 = section(6, s6_body)

# ── Section 7: Data ───────────────────────────────────────────────────────────
# 6 values: X = 0..5 -> actual = 250+X K
packed_data = bytes(range(N_POINTS))  # X = 0,1,2,3,4,5
s7_body = packed_data
sec7 = section(7, s7_body)

# ── Section 0: Indicator ──────────────────────────────────────────────────────
END = b'7777'
total = 16 + len(sec1) + len(sec3) + len(sec4) + len(sec5) + len(sec6) + len(sec7) + 4
sec0 = (
    b'GRIB'           +
    u8(0) + u8(0)     +
    u8(0)             +  # discipline: 0 = meteorological
    u8(2)             +  # GRIB edition: 2
    struct.pack('>Q', total)
)
assert len(sec0) == 16

# Assemble message
msg = sec0 + sec1 + sec3 + sec4 + sec5 + sec6 + sec7 + END
assert len(msg) == total, f"total length mismatch: {len(msg)} != {total}"
print(f"Message length: {len(msg)} bytes")

assert msg[:4] == b'GRIB'
assert msg[-4:] == b'7777'

sha256 = hashlib.sha256(msg).hexdigest()
print(f"SHA-256: {sha256}")

out_path = '/home/coding/gribtract/tests/corpus/small/pdt1_ensemble_3x2.grib2'
with open(out_path, 'wb') as f:
    f.write(msg)
print(f"Written: {out_path}")

manifest_entry = {
    "id": "pdt1_ensemble_3x2",
    "path": "small/pdt1_ensemble_3x2.grib2",
    "sha256": sha256,
    "size_bytes": len(msg),
    "storage": "inline",
    "provenance": {
        "source": "synthetic",
        "description": (
            "Minimal GRIB2 PDT=1 fixture: individual ensemble member forecast. "
            "3x2 lat/lon grid (30-20N, 0-20E, 10deg spacing). "
            "Section 0 discipline=0 (meteorological), edition=2. "
            "Section 1: center=7 (US NCEP), start-of-forecast (significance=1), 2024-01-15T00:00:00Z. "
            "Section 3 template 3.0 (lat/lon), shape=6 (WGS84), Ni=3, Nj=2. "
            "Section 4 template 4.1 (individual ensemble member): discipline=0/cat=0/num=0 (temperature K), "
            "level type=100 (isobaric), 500 hPa, forecast_offset=6h. "
            "Ensemble: type=2 (negatively perturbed), perturbation_number=3, n_ensemble=20. "
            "Section 5 template 5.0 (simple packing): R=250.0, E=0, D=0, nbits=8. "
            "Section 6: no bitmap. "
            "Section 7: 6 packed bytes X=0..5, unpacked values = 250.0+X K."
        ),
        "capture_date": "2026-06-21",
        "generated_by": "scripts/gen_grib2_pdt1.py (gribtract project)"
    }
}
print(json.dumps(manifest_entry, indent=2))
