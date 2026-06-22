# gribtract

A pure-Rust GRIB2 decoder — turns raw NOAA/WMO GRIB2 bytes into typed fields
and gridded numeric data, verified field-by-field against eccodes/wgrib2.

## Features

- Decodes GFS, HRRR, NBM, and GEFS products
- Data representation templates: simple packing (5.0), complex packing with spatial
  differencing (5.2/5.3), JPEG2000 (5.40), PNG (5.41), IEEE float (5.4)
- Grid geometries: lat/lon (3.0), Lambert conformal (3.20), polar stereographic
  (3.30), Gaussian (3.40)
- Product templates: analysis/forecast (4.0), ensemble members (4.1),
  time-processed statistical (4.8/4.11)
- Lazy decode path for efficient single-point extraction from large grids
- 100% differential agreement vs. eccodes/wgrib2 on the corpus

## Quick start

```toml
[dependencies]
gribtract = "0.1"
```

```rust
use gribtract;

let bytes = std::fs::read("my_file.grib2")?;
let fields = gribtract::decode(&bytes)?;

for field in &fields {
    println!(
        "parameter={:?} level={:?} n_points={}",
        field.parameter,
        field.level,
        field.grid.num_data_points,
    );
}
```

## Point extraction (station time series)

For extracting values at a small set of fixed coordinates across many forecast
hours, use the lazy path to avoid decoding the full grid per station:

```rust
use gribtract::{decode_lazy, decode_point_drt0};

let lazy_fields = decode_lazy(&bytes)?;
for lf in &lazy_fields {
    if let Some(body) = &lf.section7_raw {
        let idx = /* grid index for your station */;
        if let Some(val) = decode_point_drt0(body, &lf.packing, idx) {
            println!("value = {val}");
        }
    }
}
```

## Crate family

- [`gribtract-core`](https://crates.io/crates/gribtract-core) — low-level parsing (no I/O)
- `gribtract` — this crate: high-level API and message iterator
- `gribtract-cli` — `gribtract decode|list|dump` CLI

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
