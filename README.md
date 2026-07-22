# 🌦️ gribtract

**A pure-Rust GRIB2 decoder** — turn NOAA/WMO weather model output into typed fields and gridded numbers, verified field-by-field against eccodes/wgrib2.

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

No C toolchain. No FFI. No shelling out to `wgrib2`. Core decoding (DRT 5.0 / 5.2 / 5.3 / 5.41) is 100% Rust — JPEG2000 (DRT 5.40) is the one optional feature that pulls in a C dependency (`openjpeg-sys`).

---

## 🚀 Quickstart

```bash
# 1. Install the CLI
cargo install --git https://github.com/jedarden/gribtract gribtract-cli

# 2. Grab a real forecast file from NOAA's public S3 bucket (no auth needed)
curl -O "https://noaa-hrrr-bdp-pds.s3.amazonaws.com/hrrr.$(date -u +%Y%m%d)/conus/hrrr.t00z.wrfsfcf01.grib2"

# 3. See what's inside
gribtract list hrrr.t00z.wrfsfcf01.grib2

# 4. Decode everything to JSON
gribtract decode hrrr.t00z.wrfsfcf01.grib2
```

Or as a library:

```toml
[dependencies]
gribtract = { git = "https://github.com/jedarden/gribtract" }
```

```rust
let bytes = std::fs::read("hrrr.t00z.wrfsfcf01.grib2")?;
for field in gribtract::decode(&bytes)? {
    println!(
        "param={}/{}/{}  level={}  +{}h",
        field.parameter.discipline,
        field.parameter.category,
        field.parameter.number,
        field.level.value1(),
        field.forecast_time.offset_seconds() / 3600,
    );
}
```

---

## 🤔 What is GRIB2?

GRIB2 (GRIdded Binary, Edition 2) is the binary format used by NOAA, ECMWF, and other national weather services to distribute model output — temperature, wind, precipitation, pressure fields on global and regional grids. It's the primary output format for **GFS, HRRR, NBM, GEFS, NAM**, and similar numerical weather prediction products.

Decoding it means handling multi-section message framing, four grid geometries, parameter tables indexed by discipline/category/number, five data-packing algorithms, and bitmaps for missing-value masking.

The standard tools (`wgrib2`, `eccodes`) are C programs. If you need decoded GRIB2 values in a Rust project, your options were: shell out, bind through FFI, or give up and parse bytes yourself.

**gribtract is the native Rust alternative.**

## ✨ Features

- 🦀 **Pure Rust core** — no C toolchain or FFI for simple, complex, complex+spatial-differencing, and PNG packing (DRT 5.0 / 5.2 / 5.3 / 5.41); JPEG2000 (5.40) available behind the optional `jpeg2000` feature
- 🏷️ **Typed output** — every decoded field carries structured metadata: parameter ID, level, forecast time, grid geometry
- 🗺️ **Four grid projections** — lat/lon (GDT 3.0), polar stereographic (3.20), Lambert conformal (3.30), Gaussian (3.40)
- ⚡ **Lazy decode path** — parse headers only, defer grid data; extract single grid points in **O(1)** for DRT=0 fields without decoding the full grid
- 📈 **Timeseries extractor** — pull per-station forecast series for a list of lat/lon coordinates in one call
- 🌐 **Byte-range fetching** — `gribtract-fetch` downloads GRIB2 (or slices of it) from NOAA S3, GCS, and NOMADS over HTTP range requests
- 🐍 **Python bindings** — `gribtract-py` (PyO3) exposes the decoder to Python
- ✅ **Oracle-gated correctness** — every release is verified against eccodes/wgrib2 over a corpus of real NOAA files; agreement can only ratchet up

## 📦 Crate Layout

```
gribtract/
├── crates/
│   ├── gribtract-core      # 🔩 Section parser, template decoders, unpacking. No I/O.
│   ├── gribtract           # 📚 High-level API: message iterator, field selection, timeseries
│   ├── gribtract-cli       # 💻 `gribtract decode|list|dump` command-line tool
│   ├── gribtract-fetch     # 🌐 HTTP byte-range fetching from S3 / GCS / NOMADS
│   ├── gribtract-py        # 🐍 Python bindings via PyO3 (excluded from default build)
│   └── gribtract-testutil  # 🧪 Differential harness: corpus, golden fixtures, diff
└── xtask                   # 🛠️ Corpus management, reference-decoder runner, bench dashboard
```

`gribtract-core` is the zero-dependency parsing core. `gribtract` is the crate you depend on for library use. `gribtract-cli` builds the standalone `gribtract` binary.

## 📖 Usage

### Decode all fields from a GRIB2 file

```rust
use gribtract::{decode, Field};

let bytes = std::fs::read("hrrr.t00z.wrfsfcf01.grib2")?;
let fields: Vec<Field> = gribtract::decode(&bytes)?;

for field in &fields {
    println!(
        "param={}/{}/{}  level={}  forecast_offset={}h",
        field.parameter.discipline,
        field.parameter.category,
        field.parameter.number,
        field.level.value1(),
        field.forecast_time.offset_seconds() / 3600,
    );
}
```

### Extract a single grid point — O(1), no full decode

```rust
use gribtract::{decode_lazy, decode_point_drt0};

let fields = gribtract::decode_lazy(&bytes)?;
for lf in &fields {
    if let Some(body) = &lf.section7_raw {
        let idx = lf.grid.nearest_index(40.64, -73.78)?; // JFK
        if let Some(value) = decode_point_drt0(body, &lf.packing, idx) {
            println!("value at JFK: {value}");
        }
    }
}
```

### Extract a forecast timeseries for a list of stations

```rust
use gribtract::timeseries::{extract_timeseries, ParameterRecord, Station, TimeseriesRequest};

// 2m temperature (discipline=0, category=0, number=0), height-above-ground level (type=103)
let request = TimeseriesRequest {
    parameter: ParameterRecord { discipline: 0, category: 0, number: 0 },
    level_type1: 103,
    stations: vec![
        Station { id: "JFK".into(), lat: 40.64,  lon: -73.78  },
        Station { id: "ORD".into(), lat: 41.979, lon: -87.905 },
        Station { id: "LAX".into(), lat: 33.943, lon: -118.408 },
    ],
};

let fields = gribtract::decode(&bytes)?;
let ts = extract_timeseries(&fields, &request);

for (station, rows) in &ts.stations {
    println!("Station: {station}");
    for row in rows {
        println!("  forecast hour {}: {:?}", row.forecast_hour, row.value);
    }
}
```

### Decode a DRT=3 grid (complex + spatial differencing) — extract many stations

DRT=3 requires a full sequential decode; random access isn't possible. The recommended pattern is **decode once, extract many**:

```rust
use gribtract::{decode_lazy, decode_all_drt3};

let lazy_fields = gribtract::decode_lazy(&bytes)?;
for lf in &lazy_fields {
    if let (Some(body), Some(extra)) = (&lf.section7_raw, &lf.complex_extra) {
        let n = lf.grid.num_data_points as usize;
        let grid: Vec<f64> = decode_all_drt3(body, &lf.packing, extra, n)?;

        // Now extract any number of stations by index, O(1) each
        for (lat, lon) in &station_coords {
            let idx = lf.grid.nearest_index(*lat, *lon)?;
            println!("value: {}", grid[idx]);
        }
    }
}
```

### 🐍 Python

```bash
pip install maturin
maturin develop -p gribtract-py
```

```python
import gribtract

fields = gribtract.decode(open("hrrr.t00z.wrfsfcf01.grib2", "rb").read())
for f in fields:
    print(f.parameter, f.level, f.forecast_hour)
```

## 🔬 The Correctness Oracle

gribtract is built **oracle-first**: correctness is defined as agreement with eccodes/wgrib2 over real files, not as passing hand-written unit tests.

```
gribtract decode  ─┐
                   ├─→ compare field-by-field + grid values within derived tolerance
eccodes/wgrib2    ─┘
```

The differential harness in `gribtract-testutil` runs both decoders over a corpus of real NOAA GRIB2 files (GFS, HRRR, NBM, GEFS) and compares:

- **Metadata exactly** — center, parameter ID, level, forecast time, grid geometry
- **Grid values within derived tolerance** — computed from the packing template's own scale factors, never an arbitrary epsilon

Any disagreement becomes a permanent regression fixture. The agreement percentage can only increase; there is no mechanism to silently drop a covered template.

```bash
# Run the differential tests
cargo test -p gribtract -- differential

# Fetch and verify against the full corpus (requires eccodes installed)
cargo xtask corpus fetch
cargo xtask corpus verify
```

## 📊 Performance Dashboard

A self-contained benchmark dashboard (`dashboard.html`) tracks decode throughput on real NOAA files, fed by `bench-results.json`. Speed numbers are never reported without a corresponding correctness verification run.

```bash
cargo xtask bench    # run benchmarks, regenerate bench-results.json
cargo xtask serve    # serve the live dashboard with streaming updates
```

## 🏗️ Building from Source

Requires **Rust 1.75+**.

```bash
git clone https://github.com/jedarden/gribtract
cd gribtract
cargo build --release
```

The `gribtract-py` extension is excluded from the default workspace build (it needs Python headers for PyO3) — build it explicitly with `maturin develop -p gribtract-py`.

## 💻 CLI Reference

```bash
gribtract list   forecast.grib2   # 📋 list all fields (JSON)
gribtract decode forecast.grib2   # 📤 decode all fields + values (JSON)
gribtract dump   forecast.grib2   # 🔍 raw hex dump
```

The CLI is a thin wrapper over the library API, intended for ad-hoc inspection.

## 📜 License

MIT OR Apache-2.0, at your option.
