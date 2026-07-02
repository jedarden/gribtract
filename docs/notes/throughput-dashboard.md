# Throughput & Proof-of-Speed Dashboard

The question this artifact answers: **"Is gribtract actually decoding at the speed
we claim — on real NOAA files, correctly, and can I reproduce it?"** A README number
is a claim; this dashboard is the proof.

## Design rule: speed is never shown without correctness

Every throughput figure is rendered *next to* its agreement coverage for the same
run. A fast decoder that decodes wrong is worthless, so the dashboard refuses to
present a speed number whose run didn't also pass the differential check. Green
throughput + green agreement, or it's flagged.

## Data source — reproducible, never hand-edited

The dashboard is a pure renderer over `bench-results.json`, emitted by the
benchmark harness (`cargo run --bin xtask -- bench`).
Each record carries provenance so any number is attributable and repeatable:

- `git_sha`, `timestamp`
- `host` (cpu model, cores, mem) — throughput is meaningless without the machine
- `corpus` (name, message count, total bytes) — what was decoded
- per-`decoder` × per-`template_5x`: `messages_per_sec`, `mb_per_sec`,
  `grid_points_per_sec`, `wall_ms`, and (for gribtract) `agreement`

Schema lives in `docs/plan/plan.md` → Data Models → "Benchmark result".

## Views

1. **Headline.** Big numbers for the current build: MB/s, messages/s, grid-points/s,
   and **speedup vs eccodes / wgrib2** (e.g. "1.8× eccodes"). Each annotated with
   the corpus + host + git_sha so the claim is grounded.
2. **Throughput over commits.** A time-series line (x = commit/time, y = MB/s) so
   regressions and gains are visible across the marathon's history. Backed by an
   append-only `bench-history.jsonl`, so the trend is real, not a single snapshot.
3. **By data-representation template.** Grouped bars (gribtract vs eccodes vs wgrib2)
   per 5.x template — surfaces *which* unpacker is slow (5.3 spatial-differencing
   and 5.40 JPEG2000 are the suspects).
4. **Correctness companion.** Agreement coverage by template, same layout — the
   gate that legitimizes every speed bar beside it.

## Live mode — the part that proves it's real, not canned

`gribtract serve --bench <corpus>` (xtask/CLI) starts a tiny local HTTP server that
serves the dashboard and exposes `POST /run`. Clicking **Run benchmark** in the
browser triggers an actual decode of the selected corpus *now*, streams progress
over SSE, and updates the headline numbers live with a wall-clock timer. This
defeats the "the numbers are fabricated" objection: you watch the machine decode N
megabytes in T seconds in front of you. Canned `bench-results.json` is the default
offline view; live mode is the proof on demand.

## Reproducibility contract

- Anyone can run `xtask bench --corpus <name>` and regenerate `bench-results.json`;
  the dashboard renders identically. No hidden state.
- CI runs the benchmark on a pinned runner and publishes the dashboard as
  a build artifact, so the trend line has a stable-hardware baseline alongside
  dev-box runs (each tagged by `host`, so they're never conflated).
- Self-contained: single HTML file + the JSON; no external CDN at runtime (vendor
  any chart lib, or hand-roll canvas/SVG) so it opens offline and from CI artifacts.

## Tooling

- Charts: keep it dependency-light — a single vendored chart lib or plain
  canvas/SVG. The dashboard must open as a static file with zero network.
- UI feedback: mount **Agentation** (`npm i -D agentation`) per the workspace
  standard, so UI tweaks come back as precise DOM selectors rather than prose.

## Definition of done (for the dashboard bead)

- [ ] `xtask bench` produces `bench-results.json` + appends `bench-history.jsonl`.
- [ ] Static dashboard renders all four views from the JSON, offline.
- [ ] `gribtract serve` live mode runs a real decode from the browser, streams it.
- [ ] Every throughput figure is paired with its agreement coverage + provenance.
- [ ] CI publishes the dashboard as an artifact on each build.
