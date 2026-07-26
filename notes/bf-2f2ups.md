# bf-2f2ups — combined raw toolchain + build output (verbatim)

Split-child of bf-1rxcde (4 of 4). Concatenates the verbatim captured
output from the three prior children into a single raw artifact for
downstream bf-52ge51 assembly. Each captured-output block below is the
RAW BYTES produced by the toolchain/build, passed through unchanged —
no paraphrase. Provenance (raw source files, byte-identical here):
- Child 1 (bf-55h4t3) toolchain:  /tmp/gribtract-toolchain.txt
- Child 2 (bf-yc9ktr) exit code:   /tmp/bf-yc9ktr-exit.txt
- Child 3 (bf-15vpzq) build stderr: /tmp/yc9ktr_build_stderr.txt

============================================================
SECTION 1 — TOOLCHAIN VERSIONS (child 1, bf-55h4t3)
============================================================
=== COMMAND: cargo --version ===
cargo 1.96.1 (356927216 2026-06-26)
=== COMMAND: rustc --version ===
rustc 1.96.1 (31fca3adb 2026-06-26)

============================================================
SECTION 2 — BUILD RESULT / EXIT CODE (child 2, bf-yc9ktr)
============================================================
Exact build command used (child 2, verbatim):
timeout 1800 /home/coding/.cargo/bin/cargo build -p gribtract 2>/tmp/yc9ktr_build_stderr.txt; echo "EXIT_CODE=$?"

Exit-code capture (verbatim, /tmp/bf-yc9ktr-exit.txt):
EXIT_CODE=0

============================================================
SECTION 3 — BUILD WARNINGS (child 3, bf-15vpzq)
============================================================
Build stderr / warnings capture (verbatim, /tmp/yc9ktr_build_stderr.txt):
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
