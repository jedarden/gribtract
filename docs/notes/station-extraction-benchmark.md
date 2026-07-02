# Fixed-Station Point-Extraction Benchmark

A second benchmark workload, distinct from the bulk full-grid decode. It models the
extremely common point-forecast use case: **"I don't need the whole grid — I need
the value at a fixed roster of station coordinates, across the whole forecast
horizon."** This is the parse-speed hot path most downstream consumers actually hit.

## The workload

- **Input:** one model cycle's worth of messages for a single field (e.g. 2 m
  temperature), one message per forecast hour — `f000, f003, … f120`.
- **Task:** for a fixed roster of ~7–10 US metropolitan weather-station points,
  produce a per-station **time series**: the field value at that point for every
  forecast hour. Nearest-grid-point and bilinear-interpolation variants both timed.
- **Output the benchmark measures:** wall time and throughput to go from raw
  `.grib2` bytes → the assembled `stations × hours` matrix.

### Station roster (fixed, representative US metros)

A stable set of major-metro METAR station coordinates so runs are comparable over
time. Coordinates match official observation sites for reproducibility.

| Station | METAR | Lat | Lon | TZ |
|---------|-------|-----|-----|----|
| New York, NY (Central Park) | KNYC | 40.7789 | −73.9692 | ET |
| Miami, FL                   | KMIA | 25.7959 | −80.2870 | ET |
| Philadelphia, PA            | KPHL | 39.8721 | −75.2411 | ET |
| Atlanta, GA                 | KATL | 33.6407 | −84.4277 | ET |
| Boston, MA                  | KBOS | 42.3656 | −71.0096 | ET |
| Washington, DC              | KDCA | 38.8512 | −77.0402 | ET |
| Chicago, IL (Midway)        | KMDW | 41.7868 | −87.7522 | CT |
| Dallas, TX                  | KDFW | 32.8998 | −97.0403 | CT |
| Houston, TX                 | KIAH | 29.9902 | −95.3368 | CT |
| Minneapolis, MN             | KMSP | 44.8820 | −93.2218 | CT |
| Austin, TX                  | KAUS | 30.1945 | −97.6699 | CT |
| New Orleans, LA             | KMSY | 29.9934 | −90.2580 | CT |
| San Antonio, TX             | KSAT | 29.5337 | −98.4698 | CT |
| Oklahoma City, OK           | KOKC | 35.3931 | −97.6007 | CT |
| Denver, CO                  | KDEN | 39.8561 | −104.6737 | MT |
| Phoenix, AZ                 | KPHX | 33.4373 | −112.0078 | AZ |
| Los Angeles, CA             | KLAX | 33.9416 | −118.4085 | PT |
| Las Vegas, NV               | KLAS | 36.0840 | −115.1537 | PT |
| Seattle, WA                 | KSEA | 47.4502 | −122.3088 | PT |
| San Francisco, CA           | KSFO | 37.6189 | −122.3750 | PT |

These are fixed geographic probe points for a reproducible point-extraction benchmark,
covering a broad spread of US grid regions — coastal, interior, high-elevation,
low-latitude, and all four CONUS time zones.

## Why this is a rich optimization target

Decoding the *entire* grid to read 7 points is hugely wasteful, but GRIB2 packing
makes random point access hard — so this is exactly the kind of problem with a long
tail of techniques to try, many of which will be **dead ends**. That is expected and
fine; the correctness gate + dashboard keep every attempt honest. Candidate angles
(non-exhaustive, the marathon should invent more):

- **Lazy / partial unpack** — for simple packing (5.0) the value at point *i* is a
  pure function of the packed bits at offset *i*; decode only the cells covering the
  target points instead of the full grid.
- **Bitmap-aware indexing** — when a bitmap is present, point→packed-index needs a
  rank/select over the bitmap; precompute it once per grid geometry, reuse across
  forecast hours (the grid is identical for every hour in a cycle).
- **Geometry cache** — lat/lon → grid (i,j) is the same for all messages on the same
  grid; compute the station→index map once, reuse for the whole time series.
- **SIMD unpack** of the bit-packed integers for the spans that contain targets.
- **Decode-once-extract-many** — amortize section parsing across all stations.
- **Parallelism** — fan the per-forecast-hour messages across cores.
- **Zero-copy** section slicing to avoid per-message allocation.
- **Complex packing (5.2/5.3) reality check** — grouped/spatial-differenced data may
  not allow cheap random access; partial decode may be impossible and the honest
  answer is "fall back to full decode for these templates." Proving a technique
  *doesn't* work and recording why is a valid, valuable iteration.

## Honesty rules (same as the rest of the project)

- **Correctness gates speed.** Every extracted time series is checked against the
  full-grid decode (and, where available, the reference decoder) at the same points,
  within the derived tolerance. A faster path that changes a value is a regression.
- **Interpolation is explicit.** Nearest-point and bilinear are separate, separately
  verified modes — never silently swapped to win a benchmark.
- **Published to the dashboard.** Station-extraction runs emit their own
  `bench-results.json` records (`workload: "station-extract"`) with `git_sha` +
  `host`, so the `stations × hours / sec` trend is tracked across commits next to
  its agreement check — a speed number is never shown without its correctness proof.

## Definition of done (for the benchmark itself; the *optimization* never "finishes")

- [x] `xtask bench --workload station-extract` builds the per-station time series and
  emits throughput + agreement records.
- [x] Nearest and bilinear modes, each verified against full-grid decode at the points.
- [ ] Dashboard view: `stations × hours / sec` over commits, with the agreement
  companion.
- [x] A short `notes/` running log of techniques tried — including the ones that
  *didn't* help and why — so the loop doesn't re-walk dead ends.
