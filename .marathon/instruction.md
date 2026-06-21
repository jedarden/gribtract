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

## Strand waterfall — evaluate in order, first match wins

Each iteration picks exactly **one** strand to execute, then stops. Evaluate
strands top-to-bottom; skip to the next if the current has nothing to do.

### Strand A — Waterfall (primary work)

1. `cd /home/coding/gribtract`.
2. **Scan for stale in_progress beads.** Run `br list --status in_progress`. For each
   result, check if ALL its blockers are closed — if so, close it immediately with
   `br batch --json '[{"op":"close","id":"<id>"}]'`. This prevents phase beads from
   jamming the waterfall after their child work is complete.
3. `br ready --limit 5000` → if a bead is available, take the highest-priority one.
4. **If the ready bead is a whole phase** (too big for one iteration): decompose it.
   `br create` 2–5 small child beads, `br dep add <child> <phase>` so they block the
   phase, then work the first child this iteration. Do **not** attempt a whole phase
   in one pass.
5. Claim it: `br update <id> --status in_progress`.
6. Do the work. Keep diffs small and verifiable. Match surrounding code style.
7. **Verify before closing:**
   - `cargo test` (auto-offloads to iad-ci when tree is clean+committed, else runs
     locally under cgroup limits — commit first when you can).
   - The differential/oracle suite must pass; agreement coverage must not regress.
   - If you touched decode speed, refresh `bench-results.json` via `xtask bench` and
     confirm the dashboard still renders + stays green.
8. Commit with the repo's configured identity. **Push to `origin` (Forgejo) only** —
   `git push origin main`. Never force-push.
9. Close the bead: `br batch --json '[{"op":"close","id":"<id>"}]'` (`br close` is
   broken everywhere — do not use it).
10. If a **phase** just completed, add a comment to the genesis bead `bf-2wi` and
    tick the phase in `docs/plan/plan.md`.
11. `br sync --flush-only` → JSONL checkpoint. Then **stop**.

*If `br ready` returns empty, skip to Strand B.*

---

### Strand B — Parse-speed (open-ended optimization)

The fixed-station point-extraction benchmark (`docs/notes/station-extraction-benchmark.md`)
measures `stations × hours / sec`. This track is **never "done"** — keep trying new
techniques: lazy/partial unpack, geometry + bitmap-rank caches, SIMD spans,
decode-once-extract-many, parallelism, zero-copy slicing, and ideas not listed here.

- **Expect most attempts to fail** — that's fine. Back out anything that doesn't
  help and append a one-line entry to `notes/parse-speed-log.md`
  (`technique → result → why`) so the loop never re-walks the same dead end.
- **Correctness still gates speed.** Every extraction is verified against the
  full-grid decode within derived tolerance. A faster path that changes a value is a
  regression — revert it.
- Keep the benchmark framed generically (point forecasts at fixed station
  coordinates). Do not describe it as being for any particular downstream consumer.

Make one concrete speed attempt, measure it, keep or revert, append to the log.
Commit, push, flush, **stop**. No separate bead needed for speed work.

*If the speed track is blocked (e.g. a required fixture or toolchain is absent and
all techniques in `notes/parse-speed-log.md` have been tried), skip to Strand C.*

---

### Strand C — Weave (gap analysis → new beads)

Analyze workspace documentation for gaps and create beads to address them:

1. Read `docs/plan/plan.md`, `README.md`, and any files in `docs/notes/` and
   `docs/research/`.
2. Cross-reference against `br list` (all beads, all statuses) to find work that
   the plan calls for but that has no corresponding bead.
3. For each genuine gap: `br create` a focused, actionable bead. Add a
   `weave-generated` label. At most **3 new beads per run**.
4. If beads were created: `br sync --flush-only`, then **stop** (Strand A will pick
   them up next iteration).
5. If no gaps found: skip to Strand D.

---

### Strand D — Unravel (unblock human-labeled beads)

Look for beads labeled `human` (requiring a human decision before an autonomous
agent can proceed):

1. `br list --label human` (or scan `br list` output for `human` in labels).
2. For each `human` bead not yet analyzed this session: read its description and
   think of 1–2 autonomous workarounds that avoid the human dependency.
3. `br create` each workaround as a child bead, with `br dep add <child> <human-bead>`
   so the child blocks the parent. Label the children `unravel-generated`.
4. **Never close or modify the original `human` bead.**
5. If alternatives were created: `br sync --flush-only`, then **stop**.
6. If no `human` beads exist or all have already been analyzed: skip to Strand E.

---

### Strand E — Pulse (codebase health)

Scan the codebase for quality issues and create beads for significant findings:

1. Run `cargo clippy --all-targets 2>&1 | head -80` — collect warnings and errors.
2. Run `cargo test 2>&1 | tail -30` — collect any test failures.
3. For each distinct, actionable issue not already tracked in `br list`:
   `br create` a focused fix bead. Label it `pulse-generated`. At most **3 new beads
   per run**.
4. If beads were created: `br sync --flush-only`, then **stop**.
5. If no new issues found: skip to Strand F.

---

### Strand F — Reflect (meta-learning)

Consolidate learnings from recent closed beads:

1. Check `br list --status closed` for beads closed since the last Reflect run
   (track the last-run date in `notes/reflect-state.md`). Skip if fewer than 5 new
   closes since last run.
2. Read the close comments/bodies of each recently closed bead.
3. Extract cross-bead patterns: recurring friction, techniques that worked,
   approaches that were reverted and why.
4. Append new, non-duplicate insights to `notes/learnings.md`. At most **5 entries
   per run**. Prune entries older than 90 days without reinforcement.
5. If any insight was reinforced 3+ times, promote it to a concrete note in
   `docs/notes/` (e.g. `docs/notes/decode-patterns.md`).
6. Update `notes/reflect-state.md` with today's date and bead count.
7. Commit (`docs/notes/`, `notes/`), push, `br sync --flush-only`, then **stop**.
8. If fewer than 5 closes since last run: skip to the hard stop below.

---

### Hard stop

Only reach here if ALL strands above found nothing to do. This should be rare. If
it happens:

- `br create` a single bead describing precisely what is blocking all six strands.
- `br sync --flush-only`.
- **Stop.**

---

## Parse-speed reference details

See `docs/notes/station-extraction-benchmark.md` for the benchmark spec and
`notes/parse-speed-log.md` for the running techniques log.

## Bootstrapping the oracle (Phase 0 reality check)

The differential harness compares gribtract against **golden reference outputs**
captured from eccodes/wgrib2 — not a live reference decoder in the test loop. So:

- Commit a small set of golden fixtures: real `.grib2` input + the reference-decoded
  output + provenance (source URL/cycle, capture date, sha256). An upstream archive
  already captures real grib2 — seed from there.
- If `eccodes`/`wgrib2` are **not installed on this box**, do not block: build the
  comparator against the committed goldens, and `br create` a bead to generate/refresh
  goldens on the internal cluster (where the toolchain + files live — see the
  gitignored `.marathon/local-env.md` for the concrete cluster/archive names). Keep
  moving.
- Tolerance is derived from each message's packing header (half-ULP of the
  quantization step) — see the design note. Never a global epsilon.
- Large fixtures live out-of-tree (B2) with an in-tree manifest + sha256; the
  harness fetches by hash. `tests/corpus/large/` is gitignored.

## Hard rules

- **One strand per iteration**, then stop. Don't chain strands or beads.
- **Never** edit `.beads/` files directly (issues.jsonl, beads.db) — `br` CLI only.
- **Always** `br sync --flush-only` *before* any `br doctor --repair` (repair
  rebuilds from JSONL and destroys unflushed beads).
- Do **not** run a NEEDLE worker on this repo while the marathon runs — they share
  one git worktree and will collide.
- Stay inside the plan. If you believe the plan is wrong, write a bead proposing the
  change rather than silently diverging.
