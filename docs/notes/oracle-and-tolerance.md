# Oracle & Tolerance Policy

The single most important design decision in gribtract: **correctness is defined
by differential agreement with a reference decoder, not by reading the spec.**

## Why differential, not spec-conformance-by-reading

GRIB2's spec is large and its real-world usage is full of NOAA-specific template
combinations. A from-spec implementation is plausibly-correct; a from-spec
implementation that *also matches eccodes field-by-field on 10,000 real messages*
is verifiably-correct. We optimize the latter, because it gives an autonomous loop
an uncheatable signal.

## The comparator

For each message, and each field within it:

1. **Metadata — exact equality.** discipline, parameter (category+number), level
   type+value, reference time, forecast offset, statistical-process window,
   ensemble member/type, grid template number, Nx/Ny, corner lat/lons, scanning
   mode. Any mismatch = hard fail.
2. **Grid values — within derived tolerance.** Compare the decoded `f64` grid
   point-by-point. The missing-value mask must match exactly (bit-for-bit); only
   *present* values are numerically compared.

## Tolerance is derived, never a magic epsilon

GRIB2 packs values with a known precision. For simple/complex packing:

```
value = reference_value + (X * 2^binary_scale) / 10^decimal_scale
```

where `X` is the packed integer of bit-depth `nbits`. The smallest representable
step is therefore `2^binary_scale / 10^decimal_scale`. The agreement tolerance for
a field is **half that step** (the quantization half-ULP) — anything larger means
a genuine decode disagreement, not a rounding artifact. IEEE-float templates (5.4)
are compared exactly (bit-identical). JPEG2000/PNG templates (5.40/5.41) inherit
the integer-quantization tolerance after image decode.

This means tolerance is a *property of each message's packing header*, computed by
the harness, not a global constant. The policy is itself unit-tested against
synthetic packed values with known round-trips.

## The ratchet

Any message that fails (metadata or values) is captured as a permanent fixture
with provenance (source URL/cycle, product, capture date, sha256). The full
fixture set must stay green forever — this is what makes loop progress monotonic.
Fixtures too large for git live out-of-tree (B2) with an in-tree manifest +
hashes; the harness fetches by hash.

## North-star metric

`agreement coverage` = fraction of corpus messages where every field matches
(metadata exact + values within tolerance), broken down by:
- data-representation template (5.x) — tracks unpacker completeness
- grid-definition template (3.x) — tracks geometry completeness
- product-definition template (4.x) — tracks metadata completeness

A marathon session drives this number up. It is the one number that says whether
the decoder got better.

## Non-goals (initially)

- Encoding/writing GRIB2 (decode only).
- GRIB1 (legacy edition).
- Rebuilding a full parameter database — expose raw ids first, add naming tables
  as a separate, independently-testable layer.
