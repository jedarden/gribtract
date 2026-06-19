# gribtract — Marathon Iteration Instructions

You are an autonomous coding agent building **gribtract**, a pure-Rust GRIB2
decoder. This file is re-read at the start of every iteration. Do **one bead** of
work, verify it, commit it, then stop. The loop will call you again.

## Ground truth (read these; they override anything you remember)

- Repo root: `/home/coding/gribtract` — **cd there first, every iteration.**
- Plan: `docs/plan/plan.md` (the whole plan, 9 phases, the marathon-loop contract).
- Design law: `docs/notes/oracle-and-tolerance.md` (differential correctness +
  derived, never-magic tolerance) and `docs/notes/throughput-dashboard.md`
  (proof-of-speed dashboard spec).

## The one rule that defines this project

**Correctness is the gate; speed is the proof.** A change is only "done" if the
differential harness stays green (agreement coverage did not regress). Throughput
is measured and published, but a speed gain that lowers agreement is a regression,
not progress. Never claim a speed number without a passing decode behind it.

## Per-iteration workflow (exactly one bead)

1. `cd /home/coding/gribtract`.
2. `br ready --limit 5000` → take the highest-priority ready bead.
3. **If the ready bead is a whole phase** (too big for one iteration): decompose it.
   `br create` 2–5 small child beads describing the next concrete steps, `br dep add
   <child> <phase>` so they block the phase, then work the first child this
   iteration. Do **not** attempt a whole phase in one pass.
4. Claim it: `br update <id> --status in_progress`.
5. Do the work. Keep diffs small and verifiable. Match surrounding code style.
6. **Verify before closing:**
   - `cargo test` (it auto-offloads to iad-ci when the tree is clean+committed,
     else runs locally under cgroup limits — so commit first when you can).
   - The differential/oracle suite must pass; agreement coverage must not regress.
   - If you touched decode speed, refresh `bench-results.json` via `xtask bench` and
     confirm the dashboard still renders + stays green.
7. Commit with the repo's configured identity. **Push to `origin` (Forgejo) only** —
   `git push origin main`. The Forgejo→GitHub mirror syncs automatically. **Never
   force-push** either side.
8. Close the bead: `br batch --json '[{"op":"close","id":"<id>"}]'` (plain
   `br close` is broken everywhere — do not use it).
9. If a **phase** just completed, add a comment to the genesis bead `bf-2wi`
   recording it (`br update bf-2wi` can't edit the body — use a comment) and tick
   the phase in `docs/plan/plan.md`.
10. `br sync --flush-only` to checkpoint the db → JSONL. Then stop.

## Parse-speed track (Phase 2c — open-ended, dead ends expected)

Alongside the phase work there is a standing optimization loop: the **fixed-station
point-extraction benchmark** (`docs/notes/station-extraction-benchmark.md`) — extract
a per-station time series at a fixed roster of US metro coordinates and make it fast.

- This track is **never "done."** Keep trying new techniques to raise
  `stations × hours / sec`: lazy/partial unpack, geometry + bitmap-rank caches reused
  across forecast hours, SIMD spans, decode-once-extract-many, parallelism, zero-copy
  slicing, and ideas not listed here.
- **Expect most attempts to fail, regress, or prove impossible — that's expected and
  fine.** Back out anything that doesn't help, and append a one-line entry to a
  techniques log (e.g. `notes/parse-speed-log.md`: technique → result → why) so the
  loop doesn't re-walk the same dead end.
- The one hard rule still holds: **correctness gates speed.** Every extraction is
  verified against the full-grid decode at the same points within derived tolerance.
  A faster path that changes a value is a regression, not progress — revert it.
- When there's no obvious template/correctness bead to pick, **advancing this track
  is always valid work.** Make one concrete speed attempt, measure it on the
  dashboard, keep it if it helps and is still correct, otherwise revert + log.
- Keep this benchmark framed generically (point forecasts at fixed station
  coordinates). Do not describe it as being for any particular downstream consumer.

## Bootstrapping the oracle (Phase 0 reality check)

The differential harness compares gribtract against **golden reference outputs**
captured from eccodes/wgrib2 — not a live reference decoder in the test loop. So:

- Commit a small set of golden fixtures: real `.grib2` input + the reference-decoded
  output + provenance (source URL/cycle, capture date, sha256). The upstream archive
  already captures real grib2 — seed from there.
- If `eccodes`/`wgrib2` are **not installed on this box**, do not block: build the
  comparator against the committed goldens, and `br create` a bead to generate/refresh
  goldens in the internal cluster (where the toolchain + files live). Keep moving.
- Tolerance is derived from each message's packing header (half-ULP of the
  quantization step) — see the design note. Never a global epsilon.
- Large fixtures live out-of-tree (B2) with an in-tree manifest + sha256; the
  harness fetches by hash. `tests/corpus/large/` is gitignored.

## Hard rules

- **One bead per iteration**, then stop. Don't chain beads.
- **Never** edit `.beads/` files directly (issues.jsonl, beads.db) — `br` CLI only.
- **Always** `br sync --flush-only` *before* any `br doctor --repair` (repair
  rebuilds from JSONL and destroys unflushed beads).
- Do **not** run a NEEDLE worker on this repo while the marathon runs — they share
  one git worktree and will collide.
- If no ready bead exists, or you're blocked: `br create` a precise bead describing
  the gap/blocker, flush, and stop. Do not spin or invent unrelated work.
- Stay inside the plan. If you believe the plan is wrong, write a bead proposing the
  change rather than silently diverging.
