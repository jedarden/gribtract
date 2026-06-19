# gribtract

A pure-Rust GRIB2 decoder — decodes NOAA/WMO GRIB2 messages into typed fields and gridded numeric data, verified field-by-field against eccodes/wgrib2.

`gribtract` exists to turn raw GRIB2 files already captured by an upstream weather
pipeline into decoded forecast values that can be asof-joined at backtest time —
without shelling out to a C toolchain. It is built **oracle-first**: every
release is gated on byte-and-number-exact agreement with a reference decoder over
a growing corpus of real NOAA products.

## Why this exists

- There is no high-quality, pure-Rust GRIB2 decoder. The ecosystem shells out to
  `wgrib2`/eccodes (C) or wraps them via FFI.
- A downstream forecast-timeseries pipeline needs to decode GFS/HRRR/NBM/GEFS grids
  in Rust and emit a separate weather/forecast timeseries.
- GRIB2 has a *bounded spec but an unbounded tail* of real-world packing templates
  and grid definitions — exactly the shape that benefits from continuous,
  verification-gated improvement (marathon coding sessions).

## The correctness oracle

The project's center of gravity is not the decoder — it is the **differential
harness**. For every GRIB2 message in the corpus:

```
gribtract decode  ─┐
                   ├─→ assert field-by-field equality (headers + grid values)
eccodes/wgrib2    ─┘
```

Numeric grids are compared within a documented tolerance derived from the packing
template's own precision (not an arbitrary epsilon). Any message that disagrees
becomes a permanent regression fixture. Improvement is therefore monotonic and
externally verifiable — the loop can never silently trade away a capability.

## Structure

- `docs/notes/` — features, constraints, design decisions (spec coverage matrix, tolerance policy)
- `docs/research/` — GRIB2 spec material, template catalogs, reference-decoder notes
- `docs/plan/plan.md` — complete application plan

## Status

Planning. See `docs/plan/plan.md`.
