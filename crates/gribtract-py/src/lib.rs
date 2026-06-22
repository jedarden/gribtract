//! Python bindings for gribtract — pure-Rust GRIB2 decoder.
//!
//! # Usage
//!
//! ```python
//! import gribtract
//!
//! with open("sample.grib2", "rb") as f:
//!     data = f.read()
//!
//! fields = gribtract.decode(data)
//! for field in fields:
//!     print(field.discipline, field.category, field.number,
//!           "->", len(field.values), "grid points")
//! ```

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use gribtract_core::types::{Field, GridValues};
use ::gribtract::decode as gribtract_decode;

/// GRIB2 decoded field exposed to Python.
///
/// Attributes correspond to the WMO GRIB2 table numbers cited in the field
/// docs; see the gribtract-core source for definitions.
#[pyclass(name = "Field")]
struct PyField {
    inner: Field,
}

#[pymethods]
impl PyField {
    // ── Identification ──────────────────────────────────────────────────

    /// Originating centre (Table 0.0; 7 = NCEP/NWS).
    #[getter]
    fn center(&self) -> u16 {
        self.inner.center
    }

    /// Originating sub-centre.
    #[getter]
    fn subcenter(&self) -> u16 {
        self.inner.subcenter
    }

    // ── Parameter ───────────────────────────────────────────────────────

    /// Discipline (Table 0.0): 0 = meteorological, 1 = hydrological, 10 = oceanographic.
    #[getter]
    fn discipline(&self) -> u8 {
        self.inner.parameter.discipline
    }

    /// Parameter category within the discipline (Table 4.1.x).
    #[getter]
    fn category(&self) -> u8 {
        self.inner.parameter.category
    }

    /// Parameter number within the category (Table 4.2.x.y).
    #[getter]
    fn number(&self) -> u8 {
        self.inner.parameter.number
    }

    // ── Reference time ──────────────────────────────────────────────────

    /// Reference time as a `(year, month, day, hour, minute, second)` tuple.
    #[getter]
    fn ref_time(&self) -> (u16, u8, u8, u8, u8, u8) {
        let rt = &self.inner.forecast.reference_time;
        (rt.year, rt.month, rt.day, rt.hour, rt.minute, rt.second)
    }

    /// Unix timestamp of the reference time (seconds since 1970-01-01T00:00:00Z).
    #[getter]
    fn ref_time_unix(&self) -> i64 {
        self.inner.forecast.reference_time.unix_seconds()
    }

    /// Forecast offset in seconds.
    #[getter]
    fn forecast_offset_seconds(&self) -> i64 {
        self.inner.forecast.offset_seconds()
    }

    /// Unix timestamp of the valid (verifying) time.
    #[getter]
    fn valid_time_unix(&self) -> i64 {
        self.inner.forecast.valid_unix_seconds()
    }

    // ── Level ───────────────────────────────────────────────────────────

    /// Fixed surface 1 type code (Table 4.5; 100 = isobaric, 103 = height above MSL, …).
    #[getter]
    fn level_type1(&self) -> u8 {
        self.inner.level.type1
    }

    /// Physical value of fixed surface 1.
    #[getter]
    fn level_value1(&self) -> f64 {
        self.inner.level.value1()
    }

    /// Fixed surface 2 type code (255 = not applicable / missing).
    #[getter]
    fn level_type2(&self) -> u8 {
        self.inner.level.type2
    }

    /// Physical value of fixed surface 2.
    #[getter]
    fn level_value2(&self) -> f64 {
        self.inner.level.value2()
    }

    // ── Ensemble ────────────────────────────────────────────────────────

    /// Ensemble member type (Table 4.6): 0 = unperturbed, 1 = pos-perturbed, 2 = neg-perturbed.
    /// `None` for deterministic products.
    #[getter]
    fn ensemble_member_type(&self) -> Option<u8> {
        self.inner.ensemble.map(|e| e.member_type)
    }

    /// Ensemble member number. `None` for deterministic products.
    #[getter]
    fn ensemble_member(&self) -> Option<i16> {
        self.inner.ensemble.map(|e| e.number)
    }

    // ── Grid geometry ────────────────────────────────────────────────────

    /// Grid Definition Template number (Table 3.1): 0=lat/lon, 20=polar stereo, 30=Lambert.
    #[getter]
    fn gdt_template(&self) -> u16 {
        self.inner.gdt_template
    }

    /// Product Definition Template number (Table 4.0).
    #[getter]
    fn pdt_template(&self) -> u16 {
        self.inner.pdt_template
    }

    /// Data Representation Template number (Table 5.0):
    /// 0=simple, 2=complex, 3=spatial-diff, 40=JPEG2000, 41=PNG.
    #[getter]
    fn drt_template(&self) -> u16 {
        self.inner.drt_template
    }

    /// Total number of grid data points (including masked / missing points).
    #[getter]
    fn num_data_points(&self) -> u32 {
        self.inner.grid.num_data_points
    }

    /// Number of columns (i-direction points).
    #[getter]
    fn nx(&self) -> u32 {
        self.inner.grid.nx
    }

    /// Number of rows (j-direction points).
    #[getter]
    fn ny(&self) -> u32 {
        self.inner.grid.ny
    }

    /// Latitude of the first grid point (degrees, positive North).
    #[getter]
    fn lat_first(&self) -> f64 {
        self.inner.grid.lat_first
    }

    /// Longitude of the first grid point (degrees, positive East, 0–360).
    #[getter]
    fn lon_first(&self) -> f64 {
        self.inner.grid.lon_first
    }

    /// Latitude of the last grid point.
    #[getter]
    fn lat_last(&self) -> f64 {
        self.inner.grid.lat_last
    }

    /// Longitude of the last grid point.
    #[getter]
    fn lon_last(&self) -> f64 {
        self.inner.grid.lon_last
    }

    /// i-direction (longitude) grid spacing in degrees. 0 for projected grids.
    #[getter]
    fn di(&self) -> f64 {
        self.inner.grid.di
    }

    /// j-direction (latitude) grid spacing in degrees. 0 for projected grids.
    #[getter]
    fn dj(&self) -> f64 {
        self.inner.grid.dj
    }

    // ── Decoded values ───────────────────────────────────────────────────

    /// Decoded grid values as a Python list of floats.
    ///
    /// For masked grids (`GridValues::Masked`), returns only the *present*
    /// values in scan order. Use `values_with_mask()` to obtain separate
    /// value and mask arrays.
    #[getter]
    fn values(&self) -> Vec<f64> {
        match &self.inner.values {
            GridValues::Dense(v) => v.clone(),
            GridValues::Masked { values, present } => {
                // Return only present values
                values
                    .iter()
                    .zip(present.iter())
                    .filter_map(|(v, p)| if *p { Some(*v) } else { None })
                    .collect()
            }
        }
    }

    /// Returns `(values, mask)` where both lists have length `num_data_points`.
    ///
    /// For dense grids the mask is all-True. For masked grids, a `False` entry
    /// means the corresponding point is missing / not decoded.
    fn values_with_mask(&self) -> (Vec<f64>, Vec<bool>) {
        match &self.inner.values {
            GridValues::Dense(v) => {
                let mask = vec![true; v.len()];
                (v.clone(), mask)
            }
            GridValues::Masked { values, present } => {
                let mask: Vec<bool> = present.iter().copied().collect();
                (values.clone(), mask)
            }
        }
    }

    /// True when all grid points are present (no bitmap / missing values).
    #[getter]
    fn is_dense(&self) -> bool {
        matches!(&self.inner.values, GridValues::Dense(_))
    }

    fn __repr__(&self) -> String {
        let rt = self.ref_time();
        let n = match &self.inner.values {
            GridValues::Dense(v) => v.len(),
            GridValues::Masked { values, .. } => values.len(),
        };
        format!(
            "Field(discipline={}, category={}, number={}, \
             level_type1={}, level_value1={:.1}, \
             valid_time={:04}-{:02}-{:02}T{:02}:{:02}Z, \
             drt={}, nx={}, ny={}, n_values={})",
            self.discipline(),
            self.category(),
            self.number(),
            self.level_type1(),
            self.level_value1(),
            rt.0, rt.1, rt.2, rt.3, rt.4,
            self.drt_template(),
            self.nx(),
            self.ny(),
            n,
        )
    }
}

// ── Module ───────────────────────────────────────────────────────────────────

/// Decode all GRIB2 fields from raw bytes.
///
/// Parameters
/// ----------
/// data : bytes
///     Raw GRIB2 bytes (may contain multiple concatenated messages).
///
/// Returns
/// -------
/// list[Field]
///     Decoded fields in message order.
///
/// Raises
/// ------
/// ValueError
///     If the bytes are not valid GRIB2.
#[pyfunction]
fn decode(data: &[u8]) -> PyResult<Vec<PyField>> {
    gribtract_decode(data)
        .map(|fields| fields.into_iter().map(|f| PyField { inner: f }).collect())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// gribtract — pure-Rust GRIB2 decoder Python bindings.
///
/// Quick start
/// -----------
/// ```python
/// import gribtract
///
/// with open("sample.grib2", "rb") as f:
///     fields = gribtract.decode(f.read())
///
/// for f in fields:
///     print(f)
/// ```
#[pymodule]
#[pyo3(name = "gribtract")]
fn gribtract_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyField>()?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}

// ── Rust-side unit tests ─────────────────────────────────────────────────────
// These tests run via `cargo test` and verify that the Rust decode path
// called by the Python bindings produces correct results on the small
// test fixtures shipped in the workspace corpus.

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_path(name: &str) -> std::path::PathBuf {
        // Fixture lives at <workspace_root>/tests/corpus/small/<name>
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()  // crates/
            .unwrap()
            .parent()  // workspace root
            .unwrap()
            .to_path_buf();
        root.join("tests/corpus/small").join(name)
    }

    #[test]
    fn decode_drt0_returns_fields() {
        let path = corpus_path("gfs_anl_t2m_5x5.grib2");
        let data = std::fs::read(&path)
            .unwrap_or_else(|_| panic!("missing fixture: {}", path.display()));
        let fields = gribtract_decode(&data).expect("decode failed");
        assert!(!fields.is_empty(), "expected at least one field");
        let f = &fields[0];
        // DRT=0 simple packing — values should be dense
        assert!(matches!(f.values, GridValues::Dense(_)),
            "expected Dense values for DRT=0 fixture");
        // Grid should have points
        assert!(f.grid.num_data_points > 0);
    }

    #[test]
    fn decode_drt40_jpeg2000_returns_fields() {
        let path = corpus_path("drt40_j2k_3x2.grib2");
        let data = std::fs::read(&path)
            .unwrap_or_else(|_| panic!("missing fixture: {}", path.display()));
        let fields = gribtract_decode(&data).expect("decode failed");
        assert!(!fields.is_empty(), "expected at least one field from DRT=40 fixture");
        assert_eq!(fields[0].drt_template, 40, "expected DRT=40");
    }

    #[test]
    fn decode_drt41_png_returns_fields() {
        let path = corpus_path("drt41_png_3x2.grib2");
        let data = std::fs::read(&path)
            .unwrap_or_else(|_| panic!("missing fixture: {}", path.display()));
        let fields = gribtract_decode(&data).expect("decode failed");
        assert!(!fields.is_empty(), "expected at least one field from DRT=41 fixture");
        assert_eq!(fields[0].drt_template, 41, "expected DRT=41");
    }

    #[test]
    fn pyfiled_repr_does_not_panic() {
        // Smoke test: build a PyField from a decoded field and call __repr__
        let path = corpus_path("gfs_anl_t2m_5x5.grib2");
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(_) => return, // skip if fixture missing
        };
        let fields = gribtract_decode(&data).expect("decode failed");
        let py_field = PyField { inner: fields.into_iter().next().unwrap() };
        let repr = py_field.__repr__();
        assert!(repr.contains("Field("), "repr should start with Field(");
    }
}
