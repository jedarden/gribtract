# bf-mzbmba — Capture: GFS Gaussian corpus fixture run result

**Bead:** bf-mzbmba (Child 3 of bf-52ge51 split)
**Scope:** Capture-only. Raw result for Child 4 to assemble into the final artifact. Does not write the final artifact.
**Workspace:** /home/coding/gribtract
**Captured:** 2026-07-26

---

## 1. Fixture presence & integrity

| Item | Value |
|------|-------|
| Fixture path | `tests/corpus/large/flx.2024011500.grib2` |
| Manifest id | `core_gaussian_gdt40` |
| Present? | **YES** (already on disk — `cargo xtask corpus fetch` NOT required) |
| Size | 10,968,510 bytes |
| sha256 (computed) | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` |
| sha256 (manifest) | `003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397` |
| sha256 match? | **VERIFIED ✓** |

Verbatim:
```
$ sha256sum tests/corpus/large/flx.2024011500.grib2
003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397  tests/corpus/large/flx.2024011500.grib2
```

Fixture content (parsed from GRIB2 section headers, all 104 messages/fields):
- Grid Definition Template **3.40** (Gaussian Latitude/Longitude), nx=512, ny=256, 131072 points/message
- Product Definition Template distribution: **4.2 ×55, 4.12 ×49**
- Data Representation Template distribution: **5.3 ×102, 5.2 ×2**
- Section 6 bitmap indicator: 255 (no bitmap) throughout
- Discipline 0 (Meteorological); file type flx (flux), ensemble mean

---

## 2. Test that exercises the fixture

**Test:** `crates/gribtract/tests/diagnose_gfs_gaussian.rs::diagnose_core_gaussian_gdt40`
(not `#[ignore]` — runs in the default suite)

**Result: FAILED** — but the failure is **upstream of the fixture decode**.

The test loads the golden reference *before* decoding the fixture bytes:
```rust
let entry = corpus::fixture_entry("core_gaussian_gdt40").expect("fixture exists");
let golden_fixture = golden::load_golden(&entry.id)
    .expect("golden exists")
    .expect("golden loaded");          // <-- panics here (line 13)
let bytes = corpus::load(&entry.id).expect("fixture loaded");
match gribtract::decode(&bytes) { ... } // never reached
```

The golden reference file `tests/corpus/golden/core_gaussian_gdt40.json` **does not exist**
(golden dir contains only: conus_drt0, drt2_simple_3x3, drt40_j2k_3x2, drt41_png_3x2,
gfs_anl_t2m_5x5, gfs_tmp2m_1deg_anl, pdt1_ensemble_3x2, pdt8_accum_3x2).
`load_golden` returns `Ok(None)`, so the second `.expect("golden loaded")` panics.

> Note: `regenerate_golden.rs::regenerate_gfs_gaussian_gdt40_t1534` is `#[ignore]` (manual only)
> and targets a different golden id (`gfs_gaussian_gdt40_t1534`). It does not exercise the fixture
> in the default suite.

Verbatim `cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture`:
```
running 1 test
thread 'diagnose_core_gaussian_gdt40' panicked at crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10:
golden loaded
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test diagnose_core_gaussian_gdt40 ... FAILED

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p gribtract --test diagnose_gfs_gaussian`
```

---

## 3. Direct decode of the fixture bytes

Because the exercising test never reaches `gribtract::decode`, the fixture was exercised directly
(via a throwaway probe test `probe_decode_core_gaussian.rs`, run then removed — not committed).

**Result: `Err("decode not implemented")`** (`Error::NotImplemented`) — for BOTH default features
and `--features jpeg2000`.

Default features verbatim:
```
[probe] fixture id=core_gaussian_gdt40 path=large/flx.2024011500.grib2 storage=remote
[probe] loaded 10968510 bytes
[probe] DECODE_FAILED: decode not implemented
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s
```

With `--features jpeg2000` (unchanged):
```
[probe] fixture id=core_gaussian_gdt40 path=large/flx.2024011500.grib2 storage=remote
[probe] loaded 10968510 bytes
[probe] DECODE_FAILED: decode not implemented
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s
```

### Root cause (definitive)

`gribtract_core::decode::decode_bytes` iterates messages with `decode_message(&bytes[pos..], &mut fields)?`
— the `?` bails on the first message returning `Err`, discarding any fields decoded so far.

`Error::NotImplemented` is returned from the Product Definition Template (PDT) dispatch fallthrough at
`crates/gribtract-core/src/decode.rs:703`. The decoder handles only **PDT {0, 1, 2, 8, 11}**.

The fixture's 104 fields use **PDT 4.2 (×55)** and **PDT 4.12 (×49)**:
- Fields 1–55 use PDT 4.2 (handled) + DRT 5.3 (handled) + GDT 3.40 (handled) → would decode.
- **Field 56 (message 56) is the FIRST PDT 4.12 field** → `Error::NotImplemented` at `decode.rs:703`.
  `decode_bytes` bails here, so `gribtract::decode` returns `Err` with **zero fields**.

Section-header scan (verbatim):
```
Total GRIB2 messages: 104, total fields: 104
PDT distribution across file: {2: 55, 12: 49}
DRT distribution across file: {3: 102, 2: 2}
==> FIRST unhandled field: message 56, field 56: GDT=3.40 PDT=4.12 DRT=5.3 bm=255
    triggers: PDT 4.12
```

This is NOT the `jpeg2000`-feature gate at `decode.rs:1047-1050` (that path is only for DRT 5.40, which
this file does not use — DRT here is 5.3 / 5.2). Enabling `jpeg2000` has no effect, confirming PDT 4.12
is the blocker.

---

## 4. Summary for Child 4

- Fixture is **present and sha256-verified** (no fetch needed). Record verbatim fetch output: N/A.
- The test that nominally exercises it (`diagnose_core_gaussian_gdt40`) **FAILS**, but at the
  golden-load step (`golden loaded` panic, `diagnose_gfs_gaussian.rs:13`) because
  `tests/corpus/golden/core_gaussian_gdt40.json` is missing — **before** the fixture is decoded.
- Decoding the fixture bytes directly **fails** with `Error::NotImplemented` ("decode not implemented").
- **Blocking template:** Product Definition Template **4.12** (unhandled; decoder supports only
  0/1/2/8/11). First PDT 4.12 field is message/field 56 of 104. PDT 4.2 fields (1–55) and GDT 3.40 /
  DRT 5.3 are all handled.
- Enabling the `jpeg2000` feature does not change the result.

**Bottom line:** The GFS Gaussian corpus fixture cannot currently be decoded by gribtract end-to-end,
because 49 of its 104 fields use PDT 4.12, which is not implemented. The fixture's own grid (GDT 3.40,
512×256 Gaussian) and the majority of its fields (PDT 4.2, DRT 5.3) are supported.

---

## 5. Independent re-verification (retry run, 2026-07-26)

This bead was retried because the prior session crashed (trace `outcome: failure`,
`exit_code: 1` — a SessionEnd-hook infrastructure failure, not a task failure; the
note above was already committed in `4e47c04` but the bead was never closed). On the
retry, every claim above was independently re-run from scratch under
`cargo 1.96.1 / rustc 1.96.1`, HEAD `4e47c04`. All confirmed:

| Claim | How re-verified | Result |
|-------|-----------------|--------|
| Fixture present, sha256 matches manifest | `sha256sum` vs `manifest.json` | `003a93bf…064397` == manifest ✓ |
| Exercising test fails at line 13 ("golden loaded") | `cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture` | panic at `diagnose_gfs_gaussian.rs:13:10: golden loaded`, exit 101 ✓ |
| `decode` returns `Err("decode not implemented")` | throwaway probe `probe_decode_core_gaussian.rs` (run, then removed — not committed) | `[probe] DECODE_FAILED: decode not implemented` ✓ |
| `--features jpeg2000` does not change it | same probe with `--features jpeg2000` | identical `DECODE_FAILED: decode not implemented` ✓ |
| PDT 4.2 ×55, 4.12 ×49; DRT 5.3 ×102, 5.2 ×2; GDT 3.40 ×104; first unhandled = msg 56 | independent Python GRIB2 section walker (corrected octet offsets: PDT octets 8-9, DRT octets 10-11, GDT octets 13-14), 0 parse errors across all 104 messages | identical distribution ✓ |

**Discrepancy with Child 2 (bf-58omm2) — RESOLVED.** Child 2's close reason records the
test panic at `diagnose_gfs_gaussian.rs:19:13: Decode error: decode not implemented`. That
is **stale/incorrect**: line 19 (the `Err(e) => panic!("Decode error: …")` branch) is only
reachable if the golden file loaded, but `tests/corpus/golden/core_gaussian_gdt40.json` does
**not** exist (golden dir has 8 files, none for this id), so `load_golden` returns `Ok(None)`
and the test panics at **line 13** (`None.expect("golden loaded")`) — **before** `gribtract::decode`
is ever called. Ground truth (reproduced above) is **line 13, "golden loaded"**. Child 2's panic
string likely dates from an earlier tree state where the golden file was present; the *decode*
outcome Child 2 attributes to the test is in fact only observable via a direct decode probe, not
via this test. The decode-stub conclusion itself (decode returns `Err("decode not implemented")`)
is correct and is corroborated here by the probe.
