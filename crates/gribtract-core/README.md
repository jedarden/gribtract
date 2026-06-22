# gribtract-core

Low-level GRIB2 section parser and template decoders for the
[gribtract](https://crates.io/crates/gribtract) crate family.

This crate provides:

- Section 0–8 framing and field splitting
- Grid Definition Templates (3.x): lat/lon, Lambert conformal, polar stereographic, Gaussian
- Product Definition Templates (4.x): analysis, forecast, ensemble members, statistical products
- Data Representation Templates (5.x): simple packing, complex packing with spatial differencing,
  JPEG2000, PNG, IEEE float
- Lazy decode path (`decode_bytes_lazy`) for O(1) single-point extraction from DRT=0 fields

Designed to be no-std-friendly where possible. No I/O — operates on `&[u8]` slices.

## Usage

Most users should use the high-level [`gribtract`](https://crates.io/crates/gribtract) crate
instead of calling this crate directly.

```rust
use gribtract_core::decode::decode_bytes;

let bytes: &[u8] = /* raw GRIB2 bytes */;
let fields = decode_bytes(bytes)?;
for field in &fields {
    println!("{:?}", field.parameter);
}
```

## Crate family

- `gribtract-core` — this crate: low-level parsing and template dispatch
- [`gribtract`](https://crates.io/crates/gribtract) — high-level message iterator and public API
- `gribtract-cli` — `gribtract decode|list|dump` CLI tool

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
