# bf-2pev44 — Verify GFS fixture integration readiness (umbrella)

> **Umbrella bead.** This is the verification + close-out record for bf-2pev44. It records an
> **independent re-verification** run fresh against HEAD, confirms all four blocking dependencies are
> closed, and corrects two earlier bf-2pev44 notes that carried a factually wrong root cause.
>
> **Canonical reference (supersedes all GFS detail docs):**
> [`docs/fixtures/gfs-gaussian-fixture.md`](../docs/fixtures/gfs-gaussian-fixture.md) — consolidated by
> bead bf-4yvt7s. This note does not restate it; it confirms its load-bearing facts against the live
> workspace.

**Status:** VERIFICATION COMPLETE · all 4 dependencies closed · umbrella ready to close.

## Task & acceptance criteria

Task: "Verify that the GFS Gaussian-grid fixture is ready for integration and use."

| Acceptance criterion | How verified (this run) | Outcome |
|---|---|---|
| Fixture compiles/builds successfully | `cargo build -p gribtract` (this run) | ✅ **PASS** — exit 0, 3 stable warnings |
| Check for missing dependencies | `Cargo.lock` versions + manifest §7 | ✅ None missing — GDT 3.40 / DRT 2/3 path is dependency-free pure Rust |
| Confirm fixture follows project conventions | canonical doc §8 + manifest audit | ✅ Conforms (2 low-severity deviations logged, non-blocking) |
| Document remaining integration work needed | canonical doc §4 roadmap | ✅ 5-task roadmap; PDT 4.12 is the sole critical blocker |

## Independent re-verification — run fresh at HEAD `4542a84`, 2026-07-26

Every command below was executed by this run. Toolchain: cargo/rustc **1.96.1**.

### Build — PASS
`cargo build -p gribtract` → **exit 0**, `Finished dev profile in 0.13s`. Exactly the 3 documented
warnings: (1) `crates/gribtract/Cargo.toml` `default-features` ignored for `gribtract-core`, (2)
`crates/gribtract-cli/Cargo.toml` `default-features` ignored for `gribtract`, (3) unused variable
`context` at `crates/gribtract-core/src/decode.rs:1184:73`. None affect the fixture.

### Test — FAIL (exit 101)
`cargo test -p gribtract --test diagnose_gfs_gaussian` → `test result: FAILED. 0 passed; 1 failed`.
The single test `diagnose_core_gaussian_gdt40` panics at
`crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10` (the `.expect("golden loaded")`).

### Fixture on disk — present, sha256-verified
```
tests/corpus/large/flx.2024011500.grib2  →  10 968 510 bytes
sha256 003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397
```
Matches `tests/corpus/manifest.json` entry `core_gaussian_gdt40` (size 10968510, same sha256).

### Golden reference — ABSENT (the reason the test panics at line 13)
`tests/corpus/golden/` contains exactly **8** files, all for `small/` inline fixtures:
`conus_drt0`, `drt2_simple_3x3`, `drt40_j2k_3x2`, `drt41_png_3x2`, `gfs_anl_t2m_5x5`,
`gfs_tmp2m_1deg_anl`, `pdt1_ensemble_3x2`, `pdt8_accum_3x2`. **No** `core_gaussian_gdt40.json`. This is
the corpus-wide rule (only `small/` fixtures carry committed goldens), not a defect. `gribtract::decode`
is never reached by this test — it dies loading the golden first.

### T1534 sibling — placeholder only
`tests/corpus/large/gdas.t00z.sfluxgrbf000.grib2` → **0 bytes** (never fetched; declared size 122 MiB).

## Root cause of end-to-end decode failure — re-confirmed against source

The fixture **does not decode end-to-end**, but the cause is **PDT 4.12**, not DRT 2.

- **`parse_section4`** (`crates/gribtract-core/src/decode.rs:680`) dispatches templates `{0,1,2,8,11}`
  only; the catch-all `_ => Err(Error::NotImplemented)` is at `decode.rs:703`. **No template-12 (PDT
  4.12) arm exists.** The fixture's fields split across PDT 4.2 (×55) and PDT 4.12 (×49); the first
  PDT-4.12 field (field 56) hits the catch-all.
- **`decode()` bails on the first error** via `?` at `decode.rs:214`
  (`let msg_len = decode_message(&bytes[pos..], &mut fields)?;`), so the whole `decode()` returns `Err`
  with **zero fields decoded** — `Display` = `decode not implemented`.

### DRT 2 is NOT the blocker (correcting the earlier notes)
The two earlier bf-2pev44 notes named a "missing DRT 2 decoder" as the critical blocker. That is
**wrong**, verified against source this run:
- `decode_section7` (`decode.rs:1041-1043`) routes **both** DRT 2 and DRT 3 into `decode_drt3` whenever
  `complex_extra` is `Some` (both `parse_drt_2` and `parse_drt_3` produce a `ComplexPackingExtra`).
- `decode_drt3` has an explicit DRT-2 arm at `decode.rs:1299-1303`:
  `if order == 0 { // DRT=2: no spatial differencing — scale directly. }` (order 1/2 = DRT 3 follow).

So DRT 2 (2 fields) and DRT 3 (102 fields) both decode. GDT 3.40 (`decode.rs:403` arm) parses for all
104 fields. The **sole** decode blocker is the missing PDT-4.12 dispatch arm.

## ⚠️ Correction of the two earlier bf-2pev44 notes

[`notes/bf-2pev44-gfs-fixture-integration-verification.md`](bf-2pev44-gfs-fixture-integration-verification.md)
and its report sibling (committed `3b5396c` / `ceb3206`, dated 2026-07-25) carried the pre-correction
diagnosis. Their load-bearing claims are contradicted by the live workspace:

| Claim in the earlier notes | Verified ground truth (this run) |
|---|---|
| "Golden file exists: `tests/corpus/golden/core_gaussian_gdt40.json` (378 MB)" | **False.** No such file; golden dir has 8 files, none for this fixture. |
| "core_gaussian_gdt40 (10.5 MiB, **54** GRIB2 messages)" | **False.** 104 messages (eccodes-verified, per canonical doc §1a). 10.5 MiB size is correct. |
| Critical blocker = "Missing DRT 2 Decoder" | **False.** DRT 2 decodes via `decode_drt3` order==0 branch (`decode.rs:1299`). Real blocker = PDT 4.12. |
| Test panics at "Decode error: decode not implemented" | **False.** Test panics at line 13 `golden loaded`, **before** `decode` is called (no golden). |

The earlier notes have been marked **SUPERSEDED** in-place with a banner pointing here, so a future
reader landing on them via a link sees the correction immediately. Git history preserves the originals.

## Dependencies — all closed (umbrella unblocked)

| Bead | Role | Status |
|---|---|---|
| `bf-619add` | GFS Gaussian fixture pattern analysis | ✅ closed |
| `bf-56y2pd` | Remaining-work summary | ✅ closed |
| `bf-3ogi6i` | GFS integration remaining work | ✅ closed |
| `bf-658687` | GFS fixture integration-status docs (umbrella) | ✅ closed |

## Remaining integration work (handed off — not this bead's scope)

This is a verification umbrella, not an implementation bead. The verified blockers — recorded here so
the next implementer has one starting point — are the canonical doc's §4 roadmap:

1. **Implement PDT 4.12** in `parse_section4` (`decode.rs:680`) — **Critical**, sole decode blocker.
2. Generate + commit `tests/corpus/golden/core_gaussian_gdt40.json` once #1 lands — High.
3. Fetch + sha256-verify the T1534 `gdas…sfluxgrbf000.grib2` (0-byte today) — Medium.
4. Add `diagnose_gfs_gaussian` to CI once #1–#2 pass — Low.
5. Model PDT 4.2 / 4.12 tail octets (common-header only today) — Low.

## Conclusion

The GFS Gaussian fixture is **on disk, sha256-verified, registry-conformant, and the workspace builds
cleanly**. It does **not yet decode end-to-end**, blocked solely by the unimplemented **PDT 4.12**
dispatch arm — a gribtract-core implementation task, out of scope for this verification umbrella. All
four blocking dependencies are closed; integration-readiness verification is complete and the umbrella
closes.
