# GRIB2 Structure — Research Primer

GRIB2 (GRIdded Binary, edition 2) is the WMO format (FM 92) for gridded
meteorological data. A file is a concatenation of self-delimiting **messages**;
each message describes one or more fields on one grid.

> This is orientation material for implementation. The authoritative references are
> the WMO Manual on Codes (WMO-No. 306, Vol I.2) and NCEP's GRIB2 documentation.
> Do not treat this file as normative — verify against the spec + reference decoder.

## Message = 8 numbered sections

| § | Name | Purpose |
|---|------|---------|
| 0 | Indicator | `GRIB` magic, discipline, edition (=2), total message length |
| 1 | Identification | originating center, reference time, production status, data type |
| 2 | Local Use | optional, center-specific (NCEP ensemble info sometimes here) |
| 3 | Grid Definition | **template 3.x** → grid geometry (shape, Nx/Ny, corners, scan mode) |
| 4 | Product Definition | **template 4.x** → what the field is (param, level, time, ensemble) |
| 5 | Data Representation | **template 5.x** → how values are packed |
| 6 | Bitmap | which grid points are present (missing-value mask) |
| 7 | Data | the packed values |
| 8 | End | `7777` terminator |

A **multi-field message** repeats sections 2→7 (or 3→7) before section 8. Sections
2 and 6 are optional. Every section starts with a 4-byte length and a 1-byte
section number — that framing is what makes messages self-delimiting and lets a
parser skip what it can't yet decode.

## The three template families (where the hard work is)

### Grid Definition Templates (3.x) — geometry
- `3.0` Latitude/longitude (equidistant cylindrical) — most common, simplest.
- `3.10` Mercator.
- `3.20` Polar stereographic.
- `3.30` Lambert conformal — **HRRR's native grid.**
- `3.40` Gaussian latitude/longitude — spectral-model grids.
- Rotated variants exist. Geometry must be exact for point/asof-joins, so each
  template needs its corner-point and scanning-mode handling verified.

### Product Definition Templates (4.x) — semantics
- `4.0` analysis/forecast at a horizontal level, single time.
- `4.1` individual ensemble member.
- `4.8` average/accumulation/extreme over a time interval (statistical processing).
- `4.11` individual ensemble member, over a time interval.
- These encode parameter category/number, fixed-surface level type+value, forecast
  time offset, and (for ensembles) member identity.

### Data Representation Templates (5.x) — packing (the numeric core)
- `5.0` **Simple packing** — reference value + scale factors + nbits per point.
- `5.2` **Complex packing** — grouped, per-group references; better compression.
- `5.3` **Complex packing + spatial differencing** — values stored as 1st/2nd-order
  differences, must be reversed. **The trickiest unpacker; common in HRRR.**
- `5.4` IEEE floating point — values stored as raw floats (compare bit-exact).
- `5.40` **JPEG2000** compressed — grid encoded as a J2K image.
- `5.41` **PNG** compressed — grid encoded as a PNG.

## Decode pipeline (per field)

```
section 5 header → choose unpacker (5.x)
section 6 bitmap → build present-mask
section 7 data   → unpack integers X[i]
                 → (5.3) reverse spatial differencing
                 → value[i] = R + (X[i] * 2^E) / 10^D       (E=binary, D=decimal)
                 → place onto grid per section 3 scan mode
```

## Implications for sequencing

1. Framing + metadata first (sections 0–4) — no numbers yet, but unblocks the
   harness's metadata comparison.
2. `5.0` simple packing + `3.0` lat/lon → first real GFS/NBM numeric agreement.
3. `5.2`/`5.3` complex + spatial differencing → unlocks most HRRR fields.
4. `5.40`/`5.41` compressed → needs an image-decode dependency.
5. `3.30` Lambert conformal etc. → HRRR geometry.
6. `4.1`/`4.8`/`4.11` → GEFS ensembles + statistical products.

This ordering is reflected in the plan's phases. Each step is gated by the
differential harness, so "done" always means "agrees with the reference decoder."

## Primary sources to pull into docs/research/ as work proceeds

- WMO Manual on Codes, Vol I.2 (GRIB2 templates) — the normative template catalog.
- NCEP GRIB2 documentation + parameter tables (discipline/category/number → name).
- eccodes definitions tree (template field layouts) — useful cross-check.
- wgrib2 source (NCEP) — pragmatic reference for NOAA product quirks.
