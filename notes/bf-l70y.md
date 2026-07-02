# Benchmark Command Fix (bf-l70y)

## Status: Already Complete

This bead requested fixing the benchmark command in README.md from `cargo bench` to `cargo xtask bench`.

## Investigation

Upon inspection, the README.md file (line 174) already contains the correct command:

```bash
cargo xtask bench
```

## Git History

The fix was previously applied in commit `4d1803b`:
```
4d1803b docs(readme): correct benchmark command to 'cargo xtask bench'
```

## Verification

The command `cargo xtask bench` runs successfully and produces benchmark output.

## Conclusion

No changes were needed. The bead's request has already been fulfilled.
