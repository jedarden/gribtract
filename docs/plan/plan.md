# gribtract Plan

## Overview

A pure-Rust GRIB2 decoder that turns NOAA/WMO GRIB2 messages into typed metadata
and gridded numeric data, gated at every step on field-by-field agreement with a
reference decoder (eccodes / wgrib2) over a corpus of real NOAA products.

This is an **oracle-first** project. The differential harness is built before the
decoder is "done," so that autonomous (marathon) coding sessions always have a
concrete, externally-verifiable hill to climb: *find a message gribtract decodes
wrong → add it as a fixture → fix the decode → harness confirms no regression →
repeat.*

## Design principles

1. **The oracle is the product.** The decoder is only as trustworthy as the
   corpus and the differential check behind it. Harness work is never "overhead."
2. **Real files only.** Fixtures are actual NOAA grib2 messages (GFS, HRRR, NBM,
   GEFS, …), not synthetic. The upstream archive already captures these — seed from it.
3. **Tolerance is derived, not guessed.** Grid-value equality is checked within
   the precision implied by the packing template (e.g. decimal/binary scale
   factors), not a magic epsilon. The tolerance policy is documented and tested.
4. **Every disagreement becomes a permanent fixture.** Monotonic improvement; the
   regression ratchet is the whole point.
5. **Sub-minute verify.** `cargo test` (local, cgroup-limited or iad-ci) must run
   the full differential suite fast enough to drive an unattended loop. Large
   corpora are sharded/sampled in the fast path, exhaustive in CI.

## Architecture

```
                 ┌──────────────────────────────────────────────┐
   .grib2 bytes  │  gribtract-core (no_std-friendly where able)  │
   ───────────►  │                                                │
                 │  Indicator(0) → Identification(1) → Local(2)   │
                 │   → Grid Def(3) → Product Def(4)               │
                 │   → Data Representation(5) → Bitmap(6)          │
                 │   → Data(7) → End(8)                           │
                 │                                                │
                 │  section split → template dispatch → unpack    │
                 └───────────────┬───────────────────────────────┘
                                 │ Vec<Field { meta, grid, values }>
                 ┌───────────────▼───────────────┐
                 │  gribtract (high-level API)    │  iterate messages,
                 │  + CLI (`gribtract decode`)    │  select by key, dump JSON
                 └───────────────┬───────────────┘
                                 │
          ┌──────────────────────▼───────────────────────┐
          │  differential harness (tests/ + xtask)         │
          │  gribtract vs eccodes/wgrib2 over corpus       │
          │  → field-by-field + grid-within-tolerance      │
          └────────────────────────────────────────────────┘
```

### Crate layout (workspace)

- `crates/gribtract-core` — section parsing, template dispatch, unpacking. No I/O.
- `crates/gribtract` — high-level message iterator, field selection, public API.
- `crates/gribtract-cli` — `gribtract decode|list|dump` (JSON/CSV out).
- `xtask` — corpus management, reference-decoder runner, tolerance reports.
- `tests/` — differential conformance suite + proptest/fuzz roundtrip.

## Components

### Section parser
Splits a GRIB2 message into sections 0–8, validates length/number fields, supports
multi-field messages (repeated sections 2–7) and the GRIB2 "edition 2" framing.

### Template registry
GRIB2's complexity lives in three template families. Each gets a registry mapping
template number → decoder:
- **Grid Definition Templates (3.x)** — lat/lon, Lambert conformal, polar
  stereographic, Gaussian, rotated grids. Determines point geometry.
- **Product Definition Templates (4.x)** — what the field *is* (parameter, level,
  forecast time, ensemble member, statistical processing).
- **Data Representation Templates (5.x)** — how values are packed: simple, complex,
  complex+spatial-differencing, IEEE float, JPEG2000, PNG.

### Unpackers
The hard numeric core. Per data-representation template: read reference value,
binary/decimal scale, bit-depth; expand bitmap; apply scaling; (for 5.40/5.41)
JPEG2000/PNG decode; (for 5.3) spatial differencing reversal.

### Differential harness
Runs both decoders over the corpus, compares metadata exactly and grid values
within derived tolerance, emits a per-template coverage + agreement report.

## Data Models

```rust
struct Message<'a> { sections: Sections<'a>, fields: Vec<Field> }

struct Field {
    discipline: u8,
    parameter: ParameterId,      // category + number → name/units
    level: Level,                // type + value(s)
    forecast: ForecastTime,      // reference time + offset + statistical window
    ensemble: Option<Ensemble>,  // member / type
    grid: GridDefinition,        // template 3.x → geometry + point order
    values: GridValues,          // f64 grid + missing mask
}

enum GridValues { Dense(Vec<f64>), Masked { values: Vec<f64>, present: BitVec } }
```

## Implementation Phases

- [ ] **Phase 0 — Oracle harness first.** Stand up the corpus loader, the
  eccodes/wgrib2 reference runner (in internal cluster where the toolchain +
  files live; not the VPS), the field-by-field comparator, and the tolerance
  policy. Wire `cargo test` to run a sampled differential suite. *Deliverable: a
  failing-but-green-framework that can score any decoder against the corpus.*
- [ ] **Phase 1 — Framing + metadata.** Section 0–8 split, identification/grid/
  product metadata. Match all non-value fields exactly for GFS surface temp.
- [ ] **Phase 2 — Simple packing (5.0) + lat/lon grid (3.0).** First end-to-end
  numeric agreement on the most common GFS/NBM fields.
- [ ] **Phase 3 — Complex packing (5.2/5.3) + spatial differencing.** Unlocks most
  HRRR fields. The numerically trickiest unpacker.
- [ ] **Phase 4 — JPEG2000 (5.40) + PNG (5.41).** Compressed grids (common in some
  NOAA products). Pull in a Rust j2k/png path.
- [ ] **Phase 5 — Grid geometry breadth.** Lambert conformal (HRRR native), polar
  stereographic, Gaussian, rotated. Geometry must match for point asof-joins.
- [ ] **Phase 6 — Ensembles + statistical products (GEFS).** Product templates for
  members and time-aggregated fields.
- [ ] **Phase 7 — Publish + integrate.** crates.io, Python bindings, and the
  forecast-timeseries emitter consumed by the downstream backtest join.

## Marathon loop contract

For an autonomous session to make safe, monotonic progress, each iteration must:
1. Pick a corpus message currently failing (or absent) — the harness lists them.
2. Add/confirm it as a fixture (real file, provenance recorded).
3. Implement/repair the relevant template decoder.
4. `cargo test` → full sampled differential suite passes; no regression vs the
   ratcheted fixture set.
5. Commit. The agreement-coverage report (% messages matching, by template) is the
   single north-star metric the loop drives upward.

## Open Questions

- **Reference decoder of record:** eccodes (richer metadata) vs wgrib2 (ubiquitous
  in NOAA tooling)? Likely eccodes primary, wgrib2 as a second cross-check.
- **Tolerance derivation:** confirm that decimal/binary scale factors fully
  determine representable precision for each packing template, or whether some
  templates need an empirically-set band.
- **Corpus governance:** fixtures are real grib2 — size/licensing/storage. Store
  large fixtures out-of-tree (B2) with a manifest + hash, fetched by the harness?
- **JPEG2000 in Rust:** is there a pure-Rust J2K decoder of sufficient fidelity,
  or is a vetted FFI dependency acceptable for template 5.40 only?
- **Where the reference runs:** the differential harness needs the C toolchain;
  run it in internal-cluster (has files + cluster), not the VPS. CI wiring TBD.
- **Scope of metadata naming:** ship a parameter/level name table, or expose raw
  numeric ids and leave naming to the consumer initially?
