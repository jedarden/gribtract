# Bead bf-5jxt: Verify README benchmark command correction

## Status: Complete

The benchmark command in README.md was already corrected in commit `4d1803b`.

## Verification

1. ✅ README.md line 174 correctly documents `cargo xtask bench`
2. ✅ No instances of incorrect `cargo bench` command found in README
3. ✅ Command `cargo xtask bench` executes successfully and produces benchmark results

## Command Output

```
=== xtask bench summary ===
corpus: 9 messages, 76921 bytes
  template 5.0 | decoder=gribtract | 99.5 MB/s | 500167 msg/s | 6168723 gpts/s | 0.01 ms | agreement=100.0%
  template 5.2 | decoder=gribtract | 88.6 MB/s | 408163 msg/s | 3673469 gpts/s | 0.00 ms | agreement=100.0%
  template 5.3 | decoder=gribtract | 25.9 MB/s | 543 msg/s | 35411658 gpts/s | 1.84 ms | agreement=0.0%
  template 5.41 | decoder=gribtract | 0.1 MB/s | 9 msg/s | 20177122 gpts/s | 223.03 ms | agreement=100.0%
  station-extract | interp=nearest | 20 stations × 7 fields → 37105751 station-hours/s | agreement=100.0%
  station-extract | interp=bilinear | 20 stations × 7 fields → 10643960 station-hours/s | agreement=100.0%
  station-extract | interp=lazy-nearest | 20 stations × 7 fields → 599 station-hours/s | agreement=100.0%
  station-extract | interp=drt3-cached-nearest | 20 stations × 7 fields → 12067 station-hours/s | agreement=100.0%
```

All acceptance criteria met.
