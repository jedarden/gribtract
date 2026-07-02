# gribtract

A pure-Rust GRIB2 decoder — decodes NOAA/WMO GRIB2 messages into typed fields and gridded numeric data, verified field-by-field against eccodes/wgrib2.

No C dependencies for core GRIB2 decoding (DRT 5.0, 5.2, 5.3, 5.41). No shelling out to `wgrib2`. JPEG2000 (DRT 5.40) requires the optional `jpeg2000` feature, which pulls in `openjpeg-sys` (C FFI).

## What is GRIB2?

GRIB2 (GRIdded Binary, Edition 2) is the binary file format used by NOAA, ECMWF, and other national weather services to distribute model output — temperature, wind, precipitation, pressure fields on global and regional grids. It is the primary output format for GFS, HRRR, NBM, GEFS, NAM, and similar NWP (numerical weather prediction) products.

Decoding GRIB2 requires handling: multi-section message framing, grid geometry definitions (lat/lon, Lambert conformal, polar stereographic, Gaussian), parameter tables indexed by discipline/category/number, five or more data packing algorithms (simple, complex, spatial-differencing, JPEG2000, PNG), and bitmaps for missing-value masking.

The standard tools (`wgrib2`, `eccodes`) are C programs and libraries. If you need to decode GRIB2 in a Rust project, your options have been to shell out to them, call their C APIs through FFI, or skip decoded values and work with raw bytes.

gribtract is a native Rust alternative.

## Features

- **Pure Rust for core formats** — no C toolchain or FFI for DRT 5.0, 5.2, 5.3, 5.41 (simple, complex, complex+spatial differencing, PNG). JPEG2000 (DRT 5.40) requires the optional `jpeg2000` feature, which uses `openjpeg-sys` (C FFI)
- **Typed output** — decoded fields carry structured metadata: parameter ID, level, forecast time, grid geometry
- **Multiple packing formats** — DRT 5.0 (simple), 5.2 (complex), 5.3 (complex+spatial differencing), 5.40 (JPEG2000), 5.41 (PNG)
- **Multiple grid projections** — lat/lon (GDT 3.0), polar stereographic (3.20), Lambert conformal (3.30), Gaussian (3.40)
- **Lazy decode path** — parse headers and defer grid data; extract individual grid points in O(1) for DRT=0 fields without full decode
- **Timeseries extractor** — given a decoded GRIB2 message set, extract per-station forecasts for a list of lat/lon coordinates
- **Python bindings** — `gribtract-py` (PyO3) exposes the Rust decoder to Python
- **Oracle-gated correctness** — every release is verified against eccodes/wgrib2 over a corpus of real NOAA files

## Crate Layout

```
gribtract/
├── crates/
│   ├── gribtract-core     # Section parser, template decoders, unpacking. No I/O.
│   ├── gribtract          # High-level API: message iterator, field selection, timeseries
│   ├── gribtract-cli      # `gribtract decode|list|dump` command-line tool
│   ├── gribtract-py       # Python bindings via PyO3 (excluded from default build)
│   └── gribtract-testutil # Differential harness utilities: corpus, golden, diff
└── xtask                  # Corpus management, reference-decoder runner, bench dashboard
```

`gribtract-core` is the zero-dependency parsing core. `gribtract` is the crate you depend on for library use. `gribtract-cli` provides a standalone binary.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
gribtract = "0.1"
```

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

### Extract a single grid point (DRT=0, O(1))

```rust
use gribtract::{decode_lazy, decode_point_drt0};

let fields = gribtract::decode_lazy(&bytes)?;
for lf in &fields {
    if let Some(body) = &lf.section7_raw {
        // idx is the flat grid index for your target lat/lon
        let idx = lf.grid.nearest_index(40.64, -73.78)?;
        if let Some(value) = decode_point_drt0(body, &lf.packing, idx) {
            println!("value at JFK: {}", value);
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
        Station { id: "JFK".into(),  lat: 40.64,  lon: -73.78  },
        Station { id: "ORD".into(),  lat: 41.979, lon: -87.905 },
        Station { id: "LAX".into(),  lat: 33.943, lon: -118.408 },
    ],
};

let fields = gribtract::decode(&bytes)?;
let ts = extract_timeseries(&fields, &request);

for (station, rows) in &ts.stations {
    println!("Station: {}", station);
    for row in rows {
        println!("  forecast hour {}: {:?}", row.forecast_hour, row.value);
    }
}
```

### Decode a DRT=3 (complex+spatial differencing) grid — extract many stations

DRT=3 requires a full sequential decode; random access is not possible. The recommended pattern is decode-once-extract-many:

```rust
use gribtract::{decode_lazy, decode_all_drt3};

let lazy_fields = gribtract::decode_lazy(&bytes)?;
for lf in &lazy_fields {
    if let (Some(body), Some(extra)) = (&lf.section7_raw, &lf.complex_extra) {
        let n = lf.grid.num_data_points as usize;
        let grid: Vec<f64> = decode_all_drt3(body, &lf.packing, extra, n)?;

        // Now extract any number of stations by index in O(1) each
        for (lat, lon) in &station_coords {
            let idx = lf.grid.nearest_index(*lat, *lon)?;
            println!("value: {}", grid[idx]);
        }
    }
}
```

## The Correctness Oracle

gribtract is built **oracle-first**: correctness is defined as agreement with eccodes/wgrib2, not as passing hand-written unit tests.

The differential harness in `gribtract-testutil` runs both decoders over a corpus of real NOAA GRIB2 files (GFS, HRRR, NBM, GEFS), then compares:

- **Metadata exactly** — center, parameter ID, level, forecast time, grid geometry
- **Grid values within derived tolerance** — tolerance is computed from the packing template's own scale factors, not an arbitrary epsilon

Any message where gribtract disagrees with the reference decoder becomes a permanent regression fixture. The agreement percentage can only increase; there is no mechanism to silently remove a covered template.

```
gribtract decode  ─┐
                   ├─→ compare field-by-field + grid within tolerance
eccodes/wgrib2    ─┘
```

Run the harness:

```bash
cargo test -p gribtract -- differential
```

Fetch and run against the full corpus (requires eccodes installed):

```bash
cargo xtask corpus fetch
cargo xtask corpus verify
```

## Performance Dashboard

A self-contained benchmark dashboard (`dashboard.html`) tracks decode throughput on real NOAA files, fed by `bench-results.json`. Speed numbers are never reported without a corresponding correctness verification run.

```bash
# Run benchmarks and regenerate bench-results.json
cargo xtask bench

# Serve the live dashboard with streaming benchmark updates
cargo xtask serve
```

## Building

Requires Rust 1.75+.

```bash
git clone https://github.com/jedarden/gribtract
cd gribtract
cargo build --release
```

The `gribtract-py` Python extension is excluded from the default workspace build (it requires Python headers for PyO3). Build it explicitly:

```bash
pip install maturin
maturin develop -p gribtract-py
```

## CLI

```bash
# List all fields in a GRIB2 file (JSON output)
gribtract list forecast.grib2

# Decode all fields and print as JSON
gribtract decode forecast.grib2

# Dump raw hex of the file
gribtract dump forecast.grib2
```

The CLI binary name is `gribtract` (built from `crates/gribtract-cli`). It provides a thin wrapper over the library API for ad-hoc inspection rather than production use.

## License

MIT OR Apache-2.0
