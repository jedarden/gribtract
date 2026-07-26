# bf-15vpzq — Capture build warnings verbatim

Split-child of bf-1rxcde (3 of 4). Source: build run performed by child 2
(bf-yc9ktr, "Run gribtract build and capture pass/fail result").

## Build command used (child 2, verbatim)

```
timeout 1800 /home/coding/.cargo/bin/cargo build -p gribtract 2>/tmp/yc9ktr_build_stderr.txt; echo "EXIT_CODE=$?"
```

Build result: **PASS**, `EXIT_CODE=0`. (Build result/exit code is child 2's
record; included here only for framing.)

## Captured build output — VERBATIM

Source of truth: the file `/tmp/yc9ktr_build_stderr.txt` written by child 2's
build command above (1124 bytes, 12 lines), read back into child 2's transcript
and confirmed identical on disk. Quoted below exactly, with no paraphrasing:

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

## Summary for child 4 (artifact assembly)

The build PASSED (exit 0) and **emitted warnings**. Three distinct warning
blocks were emitted:

1. Cargo manifest warning: `default-features` ignored for `gribtract-core`
   (`crates/gribtract/Cargo.toml`).
2. Cargo manifest warning: `default-features` ignored for `gribtract`
   (`crates/gribtract-cli/Cargo.toml`).
3. Rust lint warning: unused variable `context` at
   `crates/gribtract-core/src/decode.rs:1184:73`.

Note: cargo's summary line "`gribtract-core` (lib) generated 1 warning" counts
only the Rust lint (#3); the two manifest warnings (#1, #2) are separate.
