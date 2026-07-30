# bf-4swew5 — Assembly + Child 4 re-verification (3rd attempt)

**Bead:** bf-4swew5 (Child 4 of the bf-52ge51 split — the assembly step)
**Role:** Assemble Children 1–3's verified captures into the single ground-truth artifact
`docs/fixtures/gfs-gaussian-build-status.md`.
**Workspace:** /home/coding/gribtract
**Verified:** 2026-07-26

## Status of this attempt

The artifact (`docs/fixtures/gfs-gaussian-build-status.md`) was written and committed by a
prior run of this bead (`dcac7b7`, `0e68d1a`) but that run **timed out before `br close`**
(trace `.beads/traces/bf-4swew5/` shows `exit_code: 124 / outcome: timeout`, 600001 ms). This
3rd attempt **re-verified the artifact end-to-end**, made **no changes** to it (it was already
correct), and closed the bead.

## Independent re-verification performed THIS run

Every runtime claim below was executed fresh in this run against HEAD `0e68d1a`. None is copied
from a prior failed run. Where a claim is "backed by child capture" rather than re-run, the
surviving capture file is cited.

### Toolchain (§1) — re-run
```
cargo 1.96.1 (356927216 2026-06-26)
rustc 1.96.1 (31fca3adb 2026-06-26)
```
`which cargo` → `/home/coding/.local/bin/cargo`; `/home/coding/.cargo/bin/cargo` → `rustup`.
Matches §1 exactly.

### Build (§2) — re-run (emitted by the test compile below)
`timeout 1800 /home/coding/.cargo/bin/cargo test -p gribtract --test diagnose_gfs_gaussian`
compiled the test binary with `Finished … in 0.06s` (incremental cache) and emitted exactly the
3 warnings §2 lists: `gribtract-cli/Cargo.toml` default-features (gribtract),
`gribtract/Cargo.toml` default-features (gribtract-core), and unused variable `context` at
`crates/gribtract-core/src/decode.rs:1184:73`. **Build PASSes** (warnings only).

### Test (§3) — re-run
`timeout 1800 /home/coding/.cargo/bin/cargo test -p gribtract --test diagnose_gfs_gaussian -- --nocapture`
→ **FAILED, exit 101**, panic verbatim:
```
thread 'diagnose_core_gaussian_gdt40' panicked at crates/gribtract/tests/diagnose_gfs_gaussian.rs:13:10:
golden loaded
...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Test-binary hash `diagnose_gfs_gaussian-02c7634154d88d01` matches §3. Matches exactly.

### Fixture (§4a) — re-checked
```
$ sha256sum tests/corpus/large/flx.2024011500.grib2
003a93bfc907c17be3b62891071260569c409a97a0d258e59460a0d013064397  tests/corpus/large/flx.2024011500.grib2
```
Matches manifest `tests/corpus/manifest.json` (id `core_gaussian_gdt40`, size 10968510, same
sha256). Golden dir has exactly 8 files; `core_gaussian_gdt40.json` is **absent** (so the §3 test
panics at line 13 before decode).

### §4b / §5 (direct-decode probe + root cause) — corroborated, source-verified
Child 3's surviving capture `notes/bf-mzbmba.md` §3 records the throwaway-probe result
`decode not implemented` and the PDT distribution `4.2 ×55, 4.12 ×49` / DRT `5.3 ×102, 5.2 ×2`.
This run did **not** re-create the throwaway probe; instead the root cause was re-verified
against source:
- `decode.rs:703` → `_ => Err(Error::NotImplemented)` (PDT dispatch handles only
  templates `{0,1,2,8,11}`; PDT 12 — i.e. "4.12" — is unsupported).
- `decode.rs:214` → `let msg_len = decode_message(&bytes[pos..], &mut fields)?;` (`?` bails on
  first error → whole `decode` returns `Err`, zero fields).

PDT 4.12 (=template 12) ∉ {0,1,2,8,11} ⇒ `Error::NotImplemented` is the necessary, deterministic
consequence. Internally consistent with §4b.

### Source citations (§5, §6) — re-checked against HEAD
- `diagnose_gfs_gaussian.rs` line 13 = `.expect("golden loaded")`, line 19 = the `panic!`
  (`"Decode error: {}"`). §6's line-13-vs-line-19 resolution is correct; line 19 is unreachable
  today because the missing golden file kills the test at line 13.
- `decode.rs:1184` unused `context`; `:214` `?`; `:703` `NotImplemented`. All confirmed.

## Conclusion

The artifact is **accurate and backed by real runs**, not asserted prose. No edit was required.
The 3rd attempt's only contribution is this re-verification trace.

## Publish handling

Worker `main` diverges from `origin/main` (3-ahead/1-behind); `git push origin main` is rejected
(non-fast-forward) and force-push is forbidden by CLAUDE.md. The artifact commit was therefore
published as a clean topic branch off current `origin/main` via the documented gribtract
worker-publish pattern (worktree + cherry-pick), not by pushing divergent main.
