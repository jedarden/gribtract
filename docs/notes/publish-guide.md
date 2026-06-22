# crates.io Publish Guide

## Prerequisites

- `cargo login` with a crates.io API token
- All tests pass: `cargo test`
- No uncommitted changes

## Publish sequence

Crates must be published in dependency order. Each must be indexed (usually ~30s)
before the next can be published.

```bash
# 1. Publish the low-level core (no workspace deps)
cargo publish -p gribtract-core

# 2. Wait ~60s for crates.io indexing, then publish the high-level API
cargo publish -p gribtract

# 3. Wait ~60s, then publish the CLI
cargo publish -p gribtract-cli
```

## Dry-run verification

```bash
# Can be verified independently (no workspace deps to resolve)
cargo publish --dry-run -p gribtract-core

# After gribtract-core is live on crates.io:
cargo publish --dry-run -p gribtract
cargo publish --dry-run -p gribtract-cli
```

## Version bump

All crates share `version` via `[workspace.package]` in the root `Cargo.toml`.
To bump: edit `version = "X.Y.Z"` there, run `cargo test`, commit, then publish
in the sequence above.

## Notes

- `gribtract-testutil` and `xtask` are internal crates with `publish = false`
  (or will be — xtask already has it; testutil should get it if not set).
- The `repository` field links to the GitHub mirror; the Forgejo instance is the
  source of truth.
