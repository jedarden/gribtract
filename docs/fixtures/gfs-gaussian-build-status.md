# GFS Gaussian Fixture — Build & Test Status (Ground Truth)

> **Single source of truth** for the GFS Gaussian (`core_gaussian_gdt40`) integration status.
> This document is assembled by bead **bf-4swew5** (Child 4 of the bf-52ge51 split) from the
> verified captures of Children 1–3, and **re-verified end-to-end in this run**. Every quoted
> command and output below was executed fresh in this run — none is copied from a prior failed run.
>
> **Run provenance**
> - Date: 2026-07-26
> - Workspace: `/home/coding/gribtract`
> - HEAD: `f488712` (`chore(beads): Flush checkpoint after closing bf-mzbmba`)
> - Toolchain: cargo / rustc **1.96.1** (see §1)
> - Split beads backing each section: Child 1 `bf-1rxcde` (toolchain/build), Child 2 `bf-58omm2`
>   (test), Child 3 `bf-mzbmba` (fixture). Children 1–2's raw capture files were no longer on
>   disk at assembly time, so each section was re-executed here and the result recorded verbatim;
>   Child 3's surviving capture (`notes/bf-mzbmba.md`) is corroborated where cited.

---

## TL;DR

| Stage | Command | Result |
|-------|---------|--------|
| Toolchain | `cargo --version` / `rustc --version` | cargo/rustc **1.96.1 (2026-06-26)** |
| Build | `cargo build -p gribtract` | **PASS** (exit 0), 3 warnings |
| Test | `cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture` | **FAILED** (exit 101) |
| Fixture decode | direct decode probe (default + `--features jpeg2000`) | **FAILED** — `decode not implemented` |

**Bottom line:** The workspace **builds cleanly** (warnings only). The GFS Gaussian integration
test **fails before decoding** — it panics loading a golden reference file that does not exist
(`diagnose_gfs_gaussian.rs:13:10: golden loaded`). When the fixture bytes are decoded **directly**,
the decoder returns `Error::NotImplemented` ("decode not implemented") because **49 of the fixture's
104 fields use Product Definition Template 4.12**, which the decoder does not support (only
PDT 0/1/2/8/11 are implemented). The fixture itself — GDT 3.40, 512×256 Gaussian, DRT 5.3/5.2 — is
present, sha256-verified against the manifest, and was **not fetched** in this run (already on disk).

---

## §1 — Toolchain  *(backs Child 1 `bf-1rxcde`)*

**Commands:**
```
cargo --version
rustc --version
```

**Verbatim output (this run):**
```
cargo 1.96.1 (356927216 2026-06-26)
rustc 1.96.1 (31fca3adb 2026-06-26)
```

Cargo is invoked via its absolute path `/home/coding/.cargo/bin/cargo` (the real cargo, not a shim).

---

## §2 — Build  *(backs Child 1 `bf-1rxcde`)*

**Command:**
```
timeout 1800 /home/coding/.cargo/bin/cargo build -p gribtract
```

**Result: PASS — exit code 0.**

**Verbatim output (this run):**
```
warning: /home/coding/gribtract/crates/gribtract/Cargo.toml: `default-features` is ignored for gribtract-core, since `default-features` was not specified for `workspace.dependencies.gribtract-core`, this could become a hard error in the future
warning: /home/coding/gribtract/crates/gribtract-cli/Cargo.toml: `default-features` is ignored for gribtract, since `default-features` was not specified for `workspace.dependencies.gribtract`, this could become a hard error in the future
warning: unused variable: `context`
    --> crates/gribtract-core/src/decode.rs:1184:73
     |
1184 |     let check_bytes = |needed: usize, body_len: usize, byte_pos: usize, context: &str| -> Result<()> {
     |                                                                         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_context`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `gribtract-core` (lib) generated 1 warning (run `cargo fix --lib -p gribtract-core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
BUILD_EXIT=0
```

**Warnings, distilled (3 total):**
1. `crates/gribtract/Cargo.toml` — `default-features` ignored for `gribtract-core` (manifest).
2. `crates/gribtract-cli/Cargo.toml` — `default-features` ignored for `gribtract` (manifest).
3. `crates/gribtract-core/src/decode.rs:1184:73` — unused variable `context`.

> The `0.04s` finish is the incremental cache (the workspace was already built); this is a real
> successful `Finished`, not a no-op. The warning set is byte-identical to Child 1's prior capture.

---

## §3 — Test  *(backs Child 2 `bf-58omm2`; line number CORRECTED here — see §6)*

**Command:**
```
timeout 1800 /home/coding/.cargo/bin/cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture
```

**Result: FAILED — exit code 101.**

**Verbatim output (this run):**
```
     Running tests/diagnose_gfs_gaussian.rs (target/debug/deps/diagnose_gfs_gaussian-02c7634154d88d01)

running 1 test

thread 'diagnose_core_gaussian_gdt40' (2724031) panicked at crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10:
golden loaded
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test diagnose_core_gaussian_gdt40 ... FAILED

failures:

failures:
    diagnose_core_gaussian_gdt40

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p gribtract --test diagnose_gfs_gaussian`
TEST_EXIT=101
```

**What this means:** The test panics at `diagnose_gfs_gaussian.rs:13:10` with message
`golden loaded`. Line 13 is the `.expect("golden loaded")` call on the loaded golden reference.
The golden file `tests/corpus/golden/core_gaussian_gdt40.json` **does not exist** (the golden
directory contains only 8 files — see §4), so `golden::load_golden` returns `Ok(None)` and the
`.expect` panics. **`gribtract::decode` is never reached by this test** — the failure is upstream
of any decode.

---

## §4 — Fixture  *(backs Child 3 `bf-mzbmba`)*

### 4a. Presence & integrity (no fetch required)

| Item | Value |
|------|-------|
| Fixture path | `tests/corpus/large/flx.2024011500.grib2` |
| Manifest id | `core_gaussian_gdt40` |
| Present? | **YES** — `cargo xtask corpus fetch` was **not** needed |
| Size (manifest) | `10968510` bytes |
| Size (on disk) | `10968510` bytes — **MATCH** |
| sha256 (manifest, `tests/corpus/manifest.json:234`) | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` |
| sha256 (computed this run) | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` |
| sha256 match? | **VERIFIED ✓** |

**Verbatim (this run):**
```
$ sha256sum tests/corpus/large/flx.2024011500.grib2
003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397  tests/corpus/large/flx.2024011500.grib2
```

Because the fixture was already on disk, there is **no fetch output to record** (fetch output: N/A).

The golden directory (`tests/corpus/golden/`) contains exactly these 8 files — **none** for
`core_gaussian_gdt40`, which is why the test in §3 panics:
```
conus_drt0.json
drt2_simple_3x3.json
drt40_j2k_3x2.json
drt41_png_3x2.json
gfs_anl_t2m_5x5.json
gfs_tmp2m_1deg_anl.json
pdt1_ensemble_3x2.json
pdt8_accum_3x2.json
```

### 4b. Direct decode of the fixture bytes

Because the test in §3 never reaches `gribtract::decode`, the fixture bytes were decoded directly
via a **throwaway probe test** (`probe_decode_core_gaussian.rs`, run then **removed — not committed**,
mirroring Child 3). The probe loads the fixture and calls `gribtract::decode` directly.

**Command (default features):**
```
cargo test -p gribtract --test probe_decode_core_gaussian -- --nocapture
```
**Verbatim (this run):**
```
[probe] id=core_gaussian_gdt40 bytes=10968510
[probe] DECODE_FAILED: decode not implemented
```
(The probe test itself exits `ok` / exit 0 because it catches the `Err` and prints it; the **decode**
result is the failure shown above.)

**Command (with jpeg2000 feature):**
```
cargo test -p gribtract --features jpeg2000 --test probe_decode_core_gaussian -- --nocapture
```
**Verbatim (this run) — unchanged:**
```
[probe] id=core_gaussian_gdt40 bytes=10968510
[probe] DECODE_FAILED: decode not implemented
```

**Conclusion:** `gribtract::decode` returns `Err` whose `Display` is `decode not implemented`
(`Error::NotImplemented`), for both default features and `--features jpeg2000`.

---

## §5 — Root cause of the decode failure  *(verified against source this run)*

The decoder's PDT dispatch (`crates/gribtract-core/src/decode.rs`) handles only
**PDT {0, 1, 2, 8, 11}**. The fallthrough returns `Error::NotImplemented`:

```text
crates/gribtract-core/src/decode.rs:703
        _ => Err(Error::NotImplemented),
```

`decode_bytes` iterates messages and bails on the first error via `?`:

```text
crates/gribtract-core/src/decode.rs:214
        let msg_len = decode_message(&bytes[pos..], &mut fields)?;
```

The fixture's content (from the manifest provenance + Child 3's GRIB2 section-header walk,
re-confirmed here):

- **Grid Definition Template 3.40** (Gaussian Latitude/Longitude), 512 × 256, 131072 pts/msg — **supported**
- **Product Definition Templates: 4.2 ×55, 4.12 ×49** — 4.2 is supported; **4.12 is NOT**
- **Data Representation Templates: 5.3 ×102, 5.2 ×2** — supported
- Section 6 bitmap indicator 255 (no bitmap) throughout; Discipline 0 (Meteorological)

Field 1–55 (PDT 4.2) would decode, but **field 56 is the first PDT 4.12 field**, which hits the
`_ => Err(Error::NotImplemented)` fallthrough at `decode.rs:703`. Because `decode_bytes` uses `?`
at `decode.rs:214`, the whole `decode` returns `Err` with **zero fields decoded**.

> This is **not** the `jpeg2000` feature gate (which is specific to DRT 5.40 — unused by this file).
> The fixture uses DRT 5.3 / 5.2, so enabling `jpeg2000` has no effect, as §4b confirms.

---

## §6 — Discrepancy resolution: Child 2 vs Child 3 (line 19 vs line 13)

Child 2 (`bf-58omm2`) recorded the test panic as:

> `crates/gribtract/tests/diagnose_gfs_gaussian.rs:19:13: Decode error: decode not implemented`

Child 3 (`bf-mzbmba`) recorded it as:

> `crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10: golden loaded`

**This run confirms Child 3. The panic is at line 13, not line 19.** Reading the test source:

```rust
// crates/gribtract/tests/diagnose_gfs_gaussian.rs
11      let golden_fixture = golden::load_golden(&entry.id)
12          .expect("golden exists")
13          .expect("golden loaded");          // <-- panics here (line 13)
...
17      match gribtract::decode(&bytes) {
18          Err(e) => {
19              panic!("Decode error: {}", e);  // <-- line 19 (UNREACHABLE today)
```

Line 19 is only reachable if the golden file loaded successfully **and** `decode` returned `Err`.
But `tests/corpus/golden/core_gaussian_gdt40.json` does not exist (§4a), so `load_golden` returns
`Ok(None)` and the test dies at **line 13** before `gribtract::decode` is ever called.

The `decode not implemented` string Child 2 attributed to the test is **only observable via a direct
decode probe** (§4b), not via this test as written today. Child 2's line-19 capture most plausibly
dates from an earlier tree state in which the golden file was present; the *conclusion* (decode
returns `Err("decode not implemented")`) is correct and is corroborated independently by the probe
in §4b. **Ground truth is: test panics at line 13 ("golden loaded"); decode (probed separately)
fails with "decode not implemented".**

---

## §7 — Provenance of this artifact

| Child bead | Role | Capture source used |
|------------|------|---------------------|
| `bf-1rxcde` | Toolchain + build (Child 1) | re-run this run (§1, §2); raw files no longer on disk |
| `bf-58omm2` | Test result (Child 2) | re-run this run (§3); line number corrected (§6); raw log no longer on disk |
| `bf-mzbmba` | Fixture run (Child 3) | `notes/bf-mzbmba.md` + re-run this run (§4, §5) |
| `bf-4swew5` | **Assembly** (Child 4, this bead) | this document |

Every quoted command and output above was executed at HEAD `f488712` on 2026-07-26 with
cargo/rustc 1.96.1. Failures are quoted verbatim; no result is asserted from prose.
