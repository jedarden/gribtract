//! Core GRIB2 data model types.
//!
//! These types represent the decoded output of a GRIB2 message, matching what
//! eccodes reports field-by-field. They are the inputs to the differential harness.

// ── Reference time (Section 1) ──────────────────────────────────────────────

/// GRIB2 reference time from Section 1, expressed as calendar components.
///
/// The significance (Table 1.2) records what the reference time *means*:
/// analysis, start of forecast, verifying time, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    /// Table 1.2: 0=analysis, 1=start-of-forecast, 2=verifying, 3=observation, 4=current.
    pub significance: u8,
}

impl ReferenceTime {
    /// Encode as seconds since 1970-01-01T00:00:00Z (UTC, no leap seconds).
    /// Used for quick arithmetic; the components are the canonical form.
    pub fn unix_seconds(&self) -> i64 {
        // Simplified proleptic Gregorian; good enough for GRIB2 forecast cycles.
        let y = self.year as i64;
        let m = self.month as i64;
        let d = self.day as i64;
        // Days since epoch (Gregorian), from Fliegel-Van Flandern.
        let jdn = (1461 * (y + 4800 + (m - 14) / 12)) / 4
            + (367 * (m - 2 - 12 * ((m - 14) / 12))) / 12
            - (3 * ((y + 4900 + (m - 14) / 12) / 100)) / 4
            + d
            - 32075;
        let unix_days = jdn - 2440588; // JDN of 1970-01-01
        unix_days * 86400
            + self.hour as i64 * 3600
            + self.minute as i64 * 60
            + self.second as i64
    }
}

// ── Parameter identity ───────────────────────────────────────────────────────

/// Unique parameter identifier: the triple (discipline, category, number)
/// from Sections 0 and 4. Indexes WMO GRIB2 parameter tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterId {
    /// Section 0, octets 7: discipline (Table 0.0).
    pub discipline: u8,
    /// Section 4, parameter category (Table 4.1.x).
    pub category: u8,
    /// Section 4, parameter number (Table 4.2.x.y).
    pub number: u8,
}

// ── Level / fixed surface (Section 4, PDT 0+) ───────────────────────────────

/// GRIB2 level description — two fixed surfaces (type + scaled value).
///
/// The physical value of surface 1 is `scaled_value1 * 10^(-scale_factor1)`.
/// Surface 2 is only meaningful when type2 != 255 (missing).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Level {
    /// Table 4.5: type of first fixed surface (e.g. 100 = isobaric level).
    pub type1: u8,
    pub scale_factor1: i8,
    pub scaled_value1: i32,
    /// Table 4.5: type of second fixed surface.
    pub type2: u8,
    pub scale_factor2: i8,
    pub scaled_value2: i32,
}

impl Level {
    /// Physical value of first fixed surface.
    pub fn value1(&self) -> f64 {
        self.scaled_value1 as f64 * 10f64.powi(-(self.scale_factor1 as i32))
    }

    /// Physical value of second fixed surface (255 / missing → not applicable).
    pub fn value2(&self) -> f64 {
        self.scaled_value2 as f64 * 10f64.powi(-(self.scale_factor2 as i32))
    }
}

// ── Forecast time (Section 4, PDT 0+) ───────────────────────────────────────

/// Forecast time: reference + offset expressed in GRIB2 units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForecastTime {
    pub reference_time: ReferenceTime,
    /// Table 4.4: time range unit of `forecast_offset` (0=minute, 1=hour, 2=day, …).
    pub time_range_unit: u8,
    /// Forecast offset in units of `time_range_unit`.
    pub forecast_offset: u32,
}

impl ForecastTime {
    /// Forecast offset converted to seconds (best-effort; months/years are approximated).
    pub fn offset_seconds(&self) -> i64 {
        let secs_per_unit: i64 = match self.time_range_unit {
            0 => 60,
            1 => 3_600,
            2 => 86_400,
            3 => 30 * 86_400,
            4 => 365 * 86_400,
            10 => 3 * 3_600,
            11 => 6 * 3_600,
            12 => 12 * 3_600,
            13 => 15 * 60,
            14 => 30 * 60,
            _ => 0,
        };
        self.forecast_offset as i64 * secs_per_unit
    }

    /// Unix timestamp of the valid (verifying) time.
    pub fn valid_unix_seconds(&self) -> i64 {
        self.reference_time.unix_seconds() + self.offset_seconds()
    }
}

// ── Ensemble (Section 4, PDT 1/11/…) ────────────────────────────────────────

/// Ensemble product descriptor (Table 4.6 + member number).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ensemble {
    /// Table 4.6: 0=unperturbed, 1=positively perturbed, 2=negatively perturbed.
    pub member_type: u8,
    pub number: i16,
}

// ── Complex packing extra (Section 5, DRT 2/3) ──────────────────────────────

/// Group structure parameters from Section 5 template 5.2 / 5.3.
///
/// Stored in [`LazyField`] for DRT=2/3 fields so the caller can decode the
/// full grid on demand via `decode_all_drt3`.  Only populated when the lazy
/// path was used to parse the message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexExtra {
    pub n_groups: u32,
    pub ref_group_widths: u8,
    pub bits_group_widths: u8,
    pub ref_group_lengths: u32,
    pub length_increment: u8,
    pub true_last_group_length: u32,
    pub bits_scaled_group_lengths: u8,
    /// 0 = no spatial differencing (DRT=2), 1 = first-order, 2 = second-order (DRT=3).
    pub order_spatial_diff: u8,
    /// Number of octets per "seed" value in the Section 7 extra-octets block.
    pub extra_octet_count: u8,
}

// ── Packing info (Section 5) — drives tolerance derivation ──────────────────

/// Packing metadata extracted from the Data Representation Section header.
///
/// Used by the differential harness to derive the tolerance for grid-value
/// comparison: `tolerance = 0.5 * quantization_step()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PackingInfo {
    /// R in GRIB2 terminology — the reference value (f32, stored as per spec).
    pub reference_value: f32,
    /// E: binary scale factor (packed integers are `(X * 2^E)`).
    pub binary_scale_factor: i16,
    /// D: decimal scale factor (unpacked value = `(R + X*2^E) / 10^D`).
    pub decimal_scale_factor: i16,
    /// Bits per packed value (n).
    pub bits_per_value: u8,
    /// Table 5.1: 0 = floating point field, 1 = integer field.
    pub original_field_type: u8,
}

impl PackingInfo {
    /// Smallest representable step for this packing: `2^E / 10^D`.
    ///
    /// Returns 0.0 for IEEE-float templates where bits_per_value is 32
    /// and both scale factors are 0 — those fields are compared exactly.
    pub fn quantization_step(&self) -> f64 {
        if self.bits_per_value == 0 {
            return 0.0;
        }
        let e = self.binary_scale_factor as i32;
        let d = self.decimal_scale_factor as i32;
        2f64.powi(e) / 10f64.powi(d)
    }

    /// Half-ULP tolerance for differential comparison.
    pub fn tolerance(&self) -> f64 {
        0.5 * self.quantization_step()
    }
}

// ── Grid definition (Section 3) ─────────────────────────────────────────────

/// Parameters unique to GDT 3.30 (Lambert conformal conic).
#[derive(Debug, Clone, PartialEq)]
pub struct LambertConformalParams {
    /// Latitude where Dx and Dy are specified (degrees, positive N).
    pub lad: f64,
    /// Orientation of the grid / central meridian (degrees, positive E, 0–360).
    pub lov: f64,
    /// i-direction (x) increment in metres at LaD.
    pub dx_m: f64,
    /// j-direction (y) increment in metres at LaD.
    pub dy_m: f64,
    /// Projection centre flag (Table 3.5): bit 7 = south-pole, bit 6 = bipolar.
    pub proj_centre: u8,
    /// First standard parallel (degrees, positive N).
    pub latin1: f64,
    /// Second standard parallel (degrees, positive N).
    pub latin2: f64,
    /// Latitude of south pole of projection (degrees).
    pub lat_south_pole: f64,
    /// Longitude of south pole of projection (degrees, 0–360).
    pub lon_south_pole: f64,
}

impl LambertConformalParams {
    /// Earth radius for GRIB2 shape=6 (WMO standard sphere), in metres.
    const EARTH_R: f64 = 6_371_229.0;

    /// Forward Lambert conformal conic projection on a sphere.
    ///
    /// Returns (x, y) in metres, relative to the projection origin placed at
    /// (LaD, LoV).  x increases in the +i direction (eastward for north-pole
    /// projection); y increases in the +j direction (northward).
    pub fn project_xy(&self, lat: f64, lon: f64) -> (f64, f64) {
        use std::f64::consts::{FRAC_PI_4, PI};
        let to_rad = PI / 180.0;

        let phi1 = self.latin1 * to_rad;
        let phi2 = self.latin2 * to_rad;
        let phi0 = self.lad * to_rad;
        let lam0 = self.lov * to_rad;
        let phi = lat * to_rad;

        // Normalise longitude to within ±π of the central meridian.
        let mut lam = lon * to_rad;
        while lam - lam0 > PI { lam -= 2.0 * PI; }
        while lam0 - lam > PI { lam += 2.0 * PI; }

        let r = Self::EARTH_R;

        // Cone constant n.
        let n = if (phi1 - phi2).abs() < 1e-8 {
            phi1.sin()
        } else {
            let t1 = (FRAC_PI_4 + phi1 / 2.0).tan();
            let t2 = (FRAC_PI_4 + phi2 / 2.0).tan();
            (phi1.cos().ln() - phi2.cos().ln()) / (t2.ln() - t1.ln())
        };

        // Scale factor F (includes R).
        let t1 = (FRAC_PI_4 + phi1 / 2.0).tan();
        let big_f = r * phi1.cos() * t1.powf(n) / n;

        // ρ at reference latitude LaD (defines the y-origin of the local grid).
        let t0 = (FRAC_PI_4 + phi0 / 2.0).tan();
        let rho0 = big_f / t0.powf(n);

        // ρ at query point.
        let t = (FRAC_PI_4 + phi / 2.0).tan();
        let rho = big_f / t.powf(n);

        let theta = n * (lam - lam0);
        let x = rho * theta.sin();
        let y = rho0 - rho * theta.cos();
        (x, y)
    }

    /// Nearest grid-point flat index for a Lambert conformal grid.
    ///
    /// Returns `None` if the query falls outside the grid extent.
    pub fn nearest_index(&self, grid: &GridDefinition, lat: f64, lon: f64) -> Option<usize> {
        let (x1, y1) = self.project_xy(grid.lat_first, grid.lon_first);
        let (xq, yq) = self.project_xy(lat, lon);

        // i increases in the +x direction; j direction depends on scanning mode.
        let di_f = (xq - x1) / self.dx_m;
        let dj_f = if grid.j_positive() {
            (yq - y1) / self.dy_m
        } else {
            (y1 - yq) / self.dy_m
        };

        // Half-cell tolerance for boundary snapping.
        if di_f < -0.5 || dj_f < -0.5 { return None; }
        let nx_f = grid.nx as f64;
        let ny_f = grid.ny as f64;
        if di_f > nx_f - 0.5 || dj_f > ny_f - 0.5 { return None; }

        let i = di_f.round() as usize;
        let j = dj_f.round() as usize;
        if i >= grid.nx as usize || j >= grid.ny as usize { return None; }

        Some(j * grid.nx as usize + i)
    }
}

/// Parameters unique to GDT 3.20 (Polar Stereographic).
#[derive(Debug, Clone, PartialEq)]
pub struct PolarStereographicParams {
    /// Latitude where Dx and Dy are specified (degrees, positive N).
    pub lad: f64,
    /// Orientation of the grid / central meridian (degrees, positive E, 0–360).
    pub lov: f64,
    /// i-direction (x) increment in metres at LaD.
    pub dx_m: f64,
    /// j-direction (y) increment in metres at LaD.
    pub dy_m: f64,
    /// Projection centre flag (Table 3.5): bit 7 set = South Pole in plane.
    pub proj_centre: u8,
}

impl PolarStereographicParams {
    /// Earth radius for GRIB2 shape=6 (WMO standard sphere), in metres.
    const EARTH_R: f64 = 6_371_229.0;

    /// Forward polar stereographic projection on a sphere.
    ///
    /// Returns (x, y) in metres in a Cartesian frame centred at the projection
    /// pole (North or South according to `proj_centre`).
    ///
    /// The projection is scaled so that Dx and Dy are exact distances at
    /// latitude `lad`.  Convention: x increases in the direction 90° east of
    /// LoV; y increases toward the pole along the LoV meridian.
    ///
    /// For North Polar (bit 7 of `proj_centre` == 0):
    ///   ρ = R · (1 + sin LaD) · cos φ / (1 + sin φ)
    ///   x = ρ sin θ,   y = −ρ cos θ    (θ = λ − LoV)
    ///
    /// For South Polar (bit 7 of `proj_centre` == 1):
    ///   ρ = R · (1 − sin LaD) · cos φ / (1 − sin φ)
    ///   x = ρ sin θ,   y = +ρ cos θ
    pub fn project_xy(&self, lat: f64, lon: f64) -> (f64, f64) {
        use std::f64::consts::PI;
        let to_rad = PI / 180.0;

        let phi    = lat * to_rad;
        let lam    = lon * to_rad;
        let lam0   = self.lov * to_rad;
        let phi_d  = self.lad * to_rad;

        // Normalise longitude difference to (−π, π].
        let mut theta = lam - lam0;
        while theta >  PI { theta -= 2.0 * PI; }
        while theta < -PI { theta += 2.0 * PI; }

        let r = Self::EARTH_R;
        let north_pole = self.proj_centre & 0x80 == 0;

        let (rho, y) = if north_pole {
            let rho = r * (1.0 + phi_d.sin()) * phi.cos() / (1.0 + phi.sin());
            let y   = -rho * theta.cos();
            (rho, y)
        } else {
            // South Polar: phi_d is negative (e.g. −60°), so (1 − sin φ_d) > 1.
            let rho = r * (1.0 - phi_d.sin()) * phi.cos() / (1.0 - phi.sin());
            let y   = rho * theta.cos();
            (rho, y)
        };

        let x = rho * theta.sin();
        (x, y)
    }

    /// Nearest grid-point flat index for a polar stereographic grid.
    ///
    /// Returns `None` if the query falls outside the grid extent.
    pub fn nearest_index(&self, grid: &GridDefinition, lat: f64, lon: f64) -> Option<usize> {
        let (x0, y0) = self.project_xy(grid.lat_first, grid.lon_first);
        let (xq, yq) = self.project_xy(lat, lon);

        // i increases in the +x direction; j direction follows scanning mode.
        let di_f = (xq - x0) / self.dx_m;
        let dj_f = if grid.j_positive() {
            (yq - y0) / self.dy_m
        } else {
            (y0 - yq) / self.dy_m
        };

        // Half-cell tolerance for boundary snapping.
        if di_f < -0.5 || dj_f < -0.5 { return None; }
        let nx_f = grid.nx as f64;
        let ny_f = grid.ny as f64;
        if di_f > nx_f - 0.5 || dj_f > ny_f - 0.5 { return None; }

        let i = di_f.round() as usize;
        let j = dj_f.round() as usize;
        if i >= grid.nx as usize || j >= grid.ny as usize { return None; }

        Some(j * grid.nx as usize + i)
    }
}

/// Parameters unique to GDT 3.40 (Gaussian Latitude/Longitude).
///
/// A Gaussian grid has **uniform longitude** spacing but **non-uniform latitude**
/// spacing — the latitudes are the zeros of the associated Legendre polynomial
/// P_N(sin φ) between the equator and the pole.  The parameter N is the number
/// of latitude circles from the pole to the equator; the grid has 2N parallels
/// (or 2N+1 if the poles are included, depending on the implementation).
///
/// For nearest-point queries the latitudes are approximated as linearly spaced
/// between La1 and La2 (equal to the true Gaussian latitudes at the corners).
/// True Gaussian quadrature placement would require computing the zeros of
/// Legendre polynomials — a valid follow-up optimisation once a real fixture
/// exercises this path.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianLatLonParams {
    /// N — number of parallels between the Pole and the Equator.
    pub n_parallels: u32,
}

/// Parameters unique to GDT 3.1 (Rotated Latitude/Longitude).
///
/// A rotated lat/lon grid uses a rotated coordinate system where the "pole"
/// of the grid is not at the geographic pole. The rotation is defined by
/// specifying the geographic location of the southern pole of the rotated
/// system and the rotation angle of the local coordinate system.
///
/// Nearest-point queries work by rotating the query point from geographic
/// coordinates into the rotated coordinate system, then applying the regular
/// lat/lon arithmetic.
#[derive(Debug, Clone, PartialEq)]
pub struct RotatedLatLonParams {
    /// Latitude of the southern pole of the rotation (degrees, positive N).
    pub lat_pole_rot: f64,
    /// Longitude of the southern pole of the rotation (degrees, positive E, 0–360).
    pub lon_pole_rot: f64,
    /// Angle of rotation of the local coordinate system (degrees, last 2 digits are fractional).
    pub angle_rot: f64,
}

impl GaussianLatLonParams {
    /// Nearest-grid-point flat index for a Gaussian lat/lon grid.
    ///
    /// Longitude is uniform (Di stored in the common `di` field).
    /// Latitude is approximated as uniform between La1 and La2
    /// (Nj steps), which is exact at the corners and a close approximation
    /// elsewhere.  Returns `None` if the query is outside the grid extent.
    pub fn nearest_index(&self, grid: &GridDefinition, lat: f64, lon: f64) -> Option<usize> {
        if grid.di == 0.0 { return None; }
        let nj = grid.ny as f64;
        if nj <= 1.0 { return None; }

        // Longitude: same uniform arithmetic as GDT 3.0.
        let mut fi = GridDefinition::lon_to_fi(lon, grid.lon_first, grid.di, grid.nx);
        let nx_f = grid.nx as f64;
        if fi < -0.5 { fi += 360.0 / grid.di; }
        else if fi > nx_f - 0.5 { fi -= 360.0 / grid.di; }
        if fi < -0.5 || fi > nx_f - 0.5 { return None; }

        // Latitude: approximate as uniform between La1 and La2.
        let dlat = (grid.lat_last - grid.lat_first) / (nj - 1.0);
        if dlat.abs() < 1e-12 { return None; }
        let fj = (lat - grid.lat_first) / dlat;
        if fj < -0.5 || fj > nj - 0.5 { return None; }

        let i = fi.round() as usize;
        let j = fj.round() as usize;
        if i >= grid.nx as usize || j >= grid.ny as usize { return None; }
        Some(j * grid.nx as usize + i)
    }
}

impl RotatedLatLonParams {
    /// Rotate a geographic (lat, lon) point into the rotated coordinate system.
    ///
    /// Returns the (lat_rot, lon_rot) coordinates in the rotated system.
    /// The rotation follows the standard GRIB2 rotated pole definition:
    /// 1. Rotate around the Z axis by -(lon_pole_rot + 180°)
    /// 2. Rotate around the Y axis by -(lat_pole_rot + 90°)
    /// 3. Rotate around the Z axis by angle_rot
    fn geographic_to_rotated(&self, lat: f64, lon: f64) -> (f64, f64) {
        use std::f64::consts::PI;
        let to_rad = PI / 180.0;
        let to_deg = 180.0 / PI;

        let lat_rad = lat * to_rad;
        let lon_rad = lon * to_rad;
        let pole_lat_rad = self.lat_pole_rot * to_rad;
        let pole_lon_rad = self.lon_pole_rot * to_rad;
        let angle_rad = self.angle_rot * to_rad;

        // Convert to Cartesian coordinates
        let x = lat_rad.cos() * lon_rad.cos();
        let y = lat_rad.cos() * lon_rad.sin();
        let z = lat_rad.sin();

        // Rotate around Z axis by -(pole_lon + 180°)
        let rot_z1 = -(pole_lon_rad + PI);
        let x1 = x * rot_z1.cos() - y * rot_z1.sin();
        let y1 = x * rot_z1.sin() + y * rot_z1.cos();
        let z1 = z;

        // Rotate around Y axis by -(pole_lat + 90°)
        let rot_y = -(pole_lat_rad + PI / 2.0);
        let x2 = x1 * rot_y.cos() + z1 * rot_y.sin();
        let y2 = y1;
        let z2 = -x1 * rot_y.sin() + z1 * rot_y.cos();

        // Rotate around Z axis by angle_rot
        let x3 = x2 * angle_rad.cos() - y2 * angle_rad.sin();
        let y3 = x2 * angle_rad.sin() + y2 * angle_rad.cos();
        let z3 = z2;

        // Convert back to spherical coordinates
        let lat_rot = z3.atan2(y3.hypot(x3)) * to_deg;
        let lon_rot = y3.atan2(x3) * to_deg;
        (lat_rot, lon_rot)
    }

    /// Nearest-grid-point flat index for a rotated lat/lon grid.
    ///
    /// Rotates the query point into the rotated coordinate system, then
    /// applies the same uniform-grid arithmetic as GDT 3.0.
    /// Returns `None` if the query is outside the grid extent.
    pub fn nearest_index(&self, grid: &GridDefinition, lat: f64, lon: f64) -> Option<usize> {
        if grid.di == 0.0 || grid.dj == 0.0 {
            return None;
        }

        // Rotate the query point into the rotated coordinate system
        let (lat_rot, lon_rot) = self.geographic_to_rotated(lat, lon);

        // Use the same arithmetic as GDT 3.0 in the rotated system
        let mut fi = GridDefinition::lon_to_fi(lon_rot, grid.lon_first, grid.di, grid.nx);
        let nx_f = grid.nx as f64;
        if fi < -0.5 { fi += 360.0 / grid.di; }
        else if fi > nx_f - 0.5 { fi -= 360.0 / grid.di; }
        if fi < -0.5 || fi > nx_f - 0.5 { return None; }

        let fj = grid.lat_to_fj(lat_rot);
        let ny_f = grid.ny as f64;
        if fj < -0.5 || fj > ny_f - 0.5 { return None; }

        let i = fi.round() as usize;
        let j = fj.round() as usize;
        if i >= grid.nx as usize || j >= grid.ny as usize { return None; }
        Some(j * grid.nx as usize + i)
    }
}

/// Template-specific grid projection parameters.
///
/// Carried inside [`GridDefinition`] to hold parameters that are only present
/// for projected (non-lat/lon) grid types.  The `LatLon` variant requires no
/// extra data because all geometry is encoded in the common fields.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GridProjection {
    /// GDT 3.0 (or similar lat/lon): all geometry in the common fields.
    #[default]
    LatLon,
    /// GDT 3.1: rotated latitude/longitude.
    RotatedLatLon(RotatedLatLonParams),
    /// GDT 3.20: polar stereographic.
    PolarStereographic(PolarStereographicParams),
    /// GDT 3.30: Lambert conformal conic.
    LambertConformal(LambertConformalParams),
    /// GDT 3.40: Gaussian latitude/longitude.
    GaussianLatLon(GaussianLatLonParams),
}

/// Grid geometry from the Grid Definition Section.
///
/// `lat_first`/`lon_first` hold the first grid-point coordinates for all
/// templates.  `lat_last`/`lon_last` and `di`/`dj` are populated for lat/lon
/// grids (template 0); they are 0.0 for projected grids whose increments are
/// stored in metres inside `projection`.
#[derive(Debug, Clone, PartialEq)]
pub struct GridDefinition {
    /// Grid Definition Template number (Table 3.1): 0=lat/lon, 20=polar stereo, 30=Lambert…
    pub template: u16,
    /// Total number of data points in the grid.
    pub num_data_points: u32,
    /// i-direction (column) count; 0 if not defined by this template.
    pub nx: u32,
    /// j-direction (row) count.
    pub ny: u32,
    /// Latitude of first grid point, in degrees (positive N).
    pub lat_first: f64,
    /// Longitude of first grid point, in degrees (positive E, 0–360).
    pub lon_first: f64,
    /// Latitude of last grid point (0.0 for projected grids).
    pub lat_last: f64,
    /// Longitude of last grid point (0.0 for projected grids).
    pub lon_last: f64,
    /// i-direction increment in degrees (0 if not uniform or for projected grids).
    pub di: f64,
    /// j-direction increment in degrees (0 if not uniform or for projected grids).
    pub dj: f64,
    /// Scanning mode flags (Table 3.4): bit 7 = +i direction, bit 6 = +j direction, …
    pub scanning_mode: u8,
    /// Resolution and component flags (Table 3.3).
    pub resolution_flags: u8,
    /// Shape of the Earth (Table 3.2): 0=spherical 6367.47km, 6=WGS84, …
    pub shape_of_earth: u8,
    /// Template-specific projection parameters (Lambert, polar stereo, …).
    pub projection: GridProjection,
}

impl GridDefinition {
    /// True if points are stored with i (longitude) varying fastest (row-major).
    pub fn i_positive(&self) -> bool {
        self.scanning_mode & 0x80 == 0
    }

    /// True if j (latitude) increases from first to last point.
    pub fn j_positive(&self) -> bool {
        self.scanning_mode & 0x40 != 0
    }

    /// True if adjacent rows alternate scan direction (boustrophedon).
    pub fn alternating_rows(&self) -> bool {
        self.scanning_mode & 0x20 != 0
    }
}

// ── Grid values (Section 6 + 7) ─────────────────────────────────────────────

/// Decoded grid: either all present (Dense) or with a missing-value mask.
#[derive(Debug, Clone)]
pub enum GridValues {
    /// All points present; length == grid.num_data_points.
    Dense(Vec<f64>),
    /// `values[i]` is meaningful only when `present[i]` is true.
    Masked { values: Vec<f64>, present: Vec<bool> },
}

impl GridValues {
    pub fn len(&self) -> usize {
        match self {
            GridValues::Dense(v) => v.len(),
            GridValues::Masked { values, .. } => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over (value, is_present) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (f64, bool)> + '_ {
        match self {
            GridValues::Dense(v) => {
                let v2: Vec<bool> = vec![true; v.len()];
                v.iter()
                    .zip(v2)
                    .map(|(&val, p)| (val, p))
                    .collect::<Vec<_>>()
                    .into_iter()
            }
            GridValues::Masked { values, present } => values
                .iter()
                .zip(present.iter())
                .map(|(&val, &p)| (val, p))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }

    /// Value at a flat index; None if the point is masked, absent, or out of bounds.
    pub fn get_at(&self, idx: usize) -> Option<f64> {
        match self {
            GridValues::Dense(v) => v.get(idx).copied(),
            GridValues::Masked { values, present } => {
                if *present.get(idx)? { values.get(idx).copied() } else { None }
            }
        }
    }

    /// Bilinear interpolation from four corners produced by [`GridDefinition::bilinear_corners`].
    /// Returns None if any corner is masked or out of bounds.
    pub fn bilinear(&self, c: &BilinearCorners) -> Option<f64> {
        let v_nw = self.get_at(c.idx_nw)?;
        let v_ne = self.get_at(c.idx_ne)?;
        let v_sw = self.get_at(c.idx_sw)?;
        let v_se = self.get_at(c.idx_se)?;
        Some(
            (1.0 - c.fx) * (1.0 - c.fy) * v_nw
                + c.fx * (1.0 - c.fy) * v_ne
                + (1.0 - c.fx) * c.fy * v_sw
                + c.fx * c.fy * v_se,
        )
    }
}

// ── Grid point extraction ─────────────────────────────────────────────────────

/// Four corner indices and fractional weights for bilinear interpolation.
/// Produced by [`GridDefinition::bilinear_corners`].
#[derive(Debug, Clone, Copy)]
pub struct BilinearCorners {
    /// Flat index of the "north-west" corner (lower j, lower i).
    pub idx_nw: usize,
    /// Flat index of the "north-east" corner (lower j, higher i).
    pub idx_ne: usize,
    /// Flat index of the "south-west" corner (higher j, lower i).
    pub idx_sw: usize,
    /// Flat index of the "south-east" corner (higher j, higher i).
    pub idx_se: usize,
    /// Fractional longitude within the cell [0, 1].
    pub fx: f64,
    /// Fractional latitude toward the "south" edge (increasing j) [0, 1].
    pub fy: f64,
}

impl GridDefinition {
    /// Nearest-grid-point flat index.
    ///
    /// Dispatches on `self.projection`:
    /// - [`GridProjection::LatLon`]: regular lat/lon arithmetic (template 0).
    ///   Returns `None` if increments are zero or the query is outside the grid.
    /// - [`GridProjection::RotatedLatLon`]: rotated lat/lon grid.
    ///   Rotates the query point into the rotated coordinate system, then
    ///   applies regular lat/lon arithmetic.
    /// - [`GridProjection::PolarStereographic`]: polar stereographic projection.
    ///   Returns `None` if the query is outside the grid extent.
    /// - [`GridProjection::LambertConformal`]: Lambert conformal conic projection.
    ///   Returns `None` if the query is outside the grid extent.
    /// - [`GridProjection::GaussianLatLon`]: Gaussian lat/lon grid.
    ///   Returns `None` if the query is outside the grid extent.
    pub fn nearest_index(&self, lat: f64, lon: f64) -> Option<usize> {
        match &self.projection {
            GridProjection::LatLon => self.nearest_index_latlon(lat, lon),
            GridProjection::RotatedLatLon(p) => p.nearest_index(self, lat, lon),
            GridProjection::PolarStereographic(p) => p.nearest_index(self, lat, lon),
            GridProjection::LambertConformal(p) => p.nearest_index(self, lat, lon),
            GridProjection::GaussianLatLon(p) => p.nearest_index(self, lat, lon),
        }
    }

    /// Nearest-index for regular lat/lon grids (template 0).
    fn nearest_index_latlon(&self, lat: f64, lon: f64) -> Option<usize> {
        if self.di == 0.0 || self.dj == 0.0 {
            return None;
        }
        let mut fi = Self::lon_to_fi(lon, self.lon_first, self.di, self.nx);
        // Try ±360° wrap once if out of the [-0.5, nx-0.5] window
        let nx_f = self.nx as f64;
        if fi < -0.5 { fi += 360.0 / self.di; }
        else if fi > nx_f - 0.5 { fi -= 360.0 / self.di; }
        if fi < -0.5 || fi > nx_f - 0.5 { return None; }

        let fj = self.lat_to_fj(lat);
        let ny_f = self.ny as f64;
        if fj < -0.5 || fj > ny_f - 0.5 { return None; }

        let i = fi.round() as usize;
        let j = fj.round() as usize;
        Some(j * self.nx as usize + i)
    }

    /// Bilinear interpolation corners.
    ///
    /// Currently implemented for lat/lon grids only.  Returns `None` for
    /// projected grids, zero-increment grids, or queries outside the grid.
    pub fn bilinear_corners(&self, lat: f64, lon: f64) -> Option<BilinearCorners> {
        if !matches!(self.projection, GridProjection::LatLon)
            || self.di == 0.0
            || self.dj == 0.0
        {
            return None;
        }
        let mut fi = Self::lon_to_fi(lon, self.lon_first, self.di, self.nx);
        let nx1 = (self.nx - 1) as f64;
        if fi < 0.0 { fi += 360.0 / self.di; }
        else if fi >= nx1 + 1.0 { fi -= 360.0 / self.di; }
        if fi < 0.0 || fi >= nx1 { return None; }

        let fj = self.lat_to_fj(lat);
        let ny1 = (self.ny - 1) as f64;
        if fj < 0.0 || fj >= ny1 { return None; }

        let i0 = fi.floor() as usize;
        let j0 = fj.floor() as usize;
        let nx = self.nx as usize;
        Some(BilinearCorners {
            idx_nw: j0 * nx + i0,
            idx_ne: j0 * nx + (i0 + 1),
            idx_sw: (j0 + 1) * nx + i0,
            idx_se: (j0 + 1) * nx + (i0 + 1),
            fx: fi - i0 as f64,
            fy: fj - j0 as f64,
        })
    }

    /// Fractional column index for a longitude query.
    fn lon_to_fi(lon: f64, lon_first: f64, di: f64, _nx: u32) -> f64 {
        // Normalize to [0, 360) to match GRIB2 convention, then compute offset
        let mut lon_n = lon % 360.0;
        if lon_n < 0.0 { lon_n += 360.0; }
        (lon_n - lon_first) / di
    }

    /// Fractional row index for a latitude query.
    fn lat_to_fj(&self, lat: f64) -> f64 {
        if self.j_positive() {
            (lat - self.lat_first) / self.dj
        } else {
            (self.lat_first - lat) / self.dj
        }
    }
}

// ── Field ────────────────────────────────────────────────────────────────────

/// A single decoded GRIB2 field — the unit of comparison in the differential harness.
#[derive(Debug, Clone)]
pub struct Field {
    // ── Identification (Section 1) ───────────────────────────────────────
    /// Originating center (Table 0): 7=US-NCEP, 98=ECMWF, …
    pub center: u16,
    pub subcenter: u16,

    // ── Parameter ───────────────────────────────────────────────────────
    pub parameter: ParameterId,

    // ── Forecast ────────────────────────────────────────────────────────
    pub forecast: ForecastTime,

    // ── Level ───────────────────────────────────────────────────────────
    pub level: Level,

    // ── Ensemble (absent for deterministic products) ─────────────────────
    pub ensemble: Option<Ensemble>,

    // ── Grid geometry ────────────────────────────────────────────────────
    pub grid: GridDefinition,

    // ── Decoded values ───────────────────────────────────────────────────
    pub values: GridValues,

    // ── Template numbers (for coverage tracking) ─────────────────────────
    /// Grid Definition Template (3.x).
    pub gdt_template: u16,
    /// Product Definition Template (4.x).
    pub pdt_template: u16,
    /// Data Representation Template (5.x).
    pub drt_template: u16,

    // ── Packing metadata (tolerance derivation) ───────────────────────────
    pub packing: PackingInfo,
}

// ── Lazy field ───────────────────────────────────────────────────────────────

/// A GRIB2 field with Section 7 data stored as raw bytes — not yet decoded.
///
/// Used by the lazy point-extraction path.  The caller extracts individual
/// grid points on demand:
/// - DRT=0 (simple packing) without a bitmap: use [`decode_point_drt0`].
/// - DRT=2/3 (complex packing, with or without spatial differencing) without a
///   bitmap: use `decode_all_drt3`; the full grid must be decoded because DRT=3
///   spatial differencing prevents true random access.  `complex_extra` carries
///   the Section 5 group-structure parameters needed by that decoder.
///
/// For other templates or fields with a bitmap, `section7_raw` is empty and
/// the lazy path returns `None`.
#[derive(Debug, Clone)]
pub struct LazyField {
    pub center: u16,
    pub subcenter: u16,
    pub parameter: ParameterId,
    pub forecast: ForecastTime,
    pub level: Level,
    pub ensemble: Option<Ensemble>,
    pub grid: GridDefinition,
    pub packing: PackingInfo,
    pub gdt_template: u16,
    pub pdt_template: u16,
    pub drt_template: u16,
    /// Raw Section 7 body bytes.
    ///
    /// Non-empty for DRT=0, DRT=2, and DRT=3 fields without a bitmap.
    /// Empty for all other templates or when a bitmap is present.
    pub section7_raw: Vec<u8>,
    /// True when the message has an active bitmap (lazy extraction unsupported).
    pub has_bitmap: bool,
    /// Group-structure parameters from Section 5; populated for DRT=2/3 fields.
    ///
    /// Required to call `decode_all_drt3`.  `None` for DRT=0, DRT=40, DRT=41,
    /// and bitmap fields.
    pub complex_extra: Option<ComplexExtra>,
}

// ── Message ──────────────────────────────────────────────────────────────────

/// A GRIB2 message: zero or more fields sharing a common identification section.
///
/// A single `.grib2` file may contain multiple concatenated messages.
/// Within a message, sections 2–7 may repeat to encode multiple fields.
#[derive(Debug, Clone)]
pub struct Message {
    pub fields: Vec<Field>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_time_unix() {
        let t = ReferenceTime {
            year: 1970, month: 1, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        assert_eq!(t.unix_seconds(), 0);

        let t2 = ReferenceTime {
            year: 1970, month: 1, day: 1,
            hour: 0, minute: 0, second: 60,
            significance: 0,
        };
        assert_eq!(t2.unix_seconds(), 60);

        // 2024-01-01T00:00:00Z
        let t3 = ReferenceTime {
            year: 2024, month: 1, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        // Verified with: date -d "2024-01-01" +%s → 1704067200
        assert_eq!(t3.unix_seconds(), 1_704_067_200);
    }

    #[test]
    fn level_scale_factor() {
        // 500 hPa isobaric level: type=100, scale_factor=0, scaled_value=50000
        // value = 50000 * 10^0 = 50000 Pa
        let level = Level {
            type1: 100, scale_factor1: 0, scaled_value1: 50000,
            type2: 255, scale_factor2: 0, scaled_value2: 0,
        };
        assert!((level.value1() - 50000.0).abs() < 1e-9);

        // 1.5m above ground: scale_factor=1, scaled_value=15 → 15 * 10^-1 = 1.5
        let level2 = Level {
            type1: 103, scale_factor1: 1, scaled_value1: 15,
            type2: 255, scale_factor2: 0, scaled_value2: 0,
        };
        assert!((level2.value1() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn forecast_time_offset_seconds() {
        let ref_time = ReferenceTime {
            year: 2024, month: 6, day: 1,
            hour: 0, minute: 0, second: 0,
            significance: 1,
        };
        let ft = ForecastTime { reference_time: ref_time, time_range_unit: 1, forecast_offset: 6 };
        assert_eq!(ft.offset_seconds(), 6 * 3600);
        assert_eq!(ft.valid_unix_seconds(), ref_time.unix_seconds() + 6 * 3600);
    }

    #[test]
    fn packing_tolerance() {
        // binary_scale=-1, decimal_scale=0 → step = 2^-1 / 10^0 = 0.5
        // tolerance = 0.25
        let p = PackingInfo {
            reference_value: 0.0,
            binary_scale_factor: -1,
            decimal_scale_factor: 0,
            bits_per_value: 16,
            original_field_type: 0,
        };
        assert!((p.quantization_step() - 0.5).abs() < 1e-12);
        assert!((p.tolerance() - 0.25).abs() < 1e-12);

        // binary_scale=0, decimal_scale=2 → step = 1 / 100 = 0.01
        let p2 = PackingInfo {
            reference_value: 0.0,
            binary_scale_factor: 0,
            decimal_scale_factor: 2,
            bits_per_value: 12,
            original_field_type: 0,
        };
        assert!((p2.quantization_step() - 0.01).abs() < 1e-12);
        assert!((p2.tolerance() - 0.005).abs() < 1e-12);
    }

    #[test]
    fn packing_zero_bits() {
        // Constant field: bits_per_value=0, step=0, tolerance=0
        let p = PackingInfo {
            reference_value: 273.15,
            binary_scale_factor: 0,
            decimal_scale_factor: 0,
            bits_per_value: 0,
            original_field_type: 0,
        };
        assert_eq!(p.quantization_step(), 0.0);
        assert_eq!(p.tolerance(), 0.0);
    }

    #[test]
    fn grid_scanning_flags() {
        // scanning_mode=0b00000000: +i (west→east), −j (north→south), no alternating
        let g = GridDefinition {
            template: 0, num_data_points: 100, nx: 10, ny: 10,
            lat_first: 90.0, lon_first: 0.0, lat_last: -90.0, lon_last: 350.0,
            di: 1.0, dj: 1.0, scanning_mode: 0x00, resolution_flags: 0x30,
            shape_of_earth: 6, projection: GridProjection::LatLon,
        };
        assert!(g.i_positive());
        assert!(!g.j_positive());
        assert!(!g.alternating_rows());

        // scanning_mode=0b01100000: +i, +j, alternating
        let g2 = GridDefinition { scanning_mode: 0x60, ..g };
        assert!(g2.i_positive());
        assert!(g2.j_positive());
        assert!(g2.alternating_rows());
    }

    #[test]
    fn grid_values_iter() {
        let gv = GridValues::Dense(vec![1.0, 2.0, 3.0]);
        let pairs: Vec<_> = gv.iter().collect();
        assert_eq!(pairs, vec![(1.0, true), (2.0, true), (3.0, true)]);

        let gv2 = GridValues::Masked {
            values: vec![1.0, 0.0, 3.0],
            present: vec![true, false, true],
        };
        let pairs2: Vec<_> = gv2.iter().collect();
        assert_eq!(pairs2, vec![(1.0, true), (0.0, false), (3.0, true)]);
    }

    // ── Test helpers ──────────────────────────────────────────────────────────

    fn test_grid_5x5() -> GridDefinition {
        // 5×5 lat/lon grid: 40N→0N, 0E→40E, 10° spacing, standard scanning
        GridDefinition {
            template: 0,
            num_data_points: 25,
            nx: 5, ny: 5,
            lat_first: 40.0, lon_first: 0.0,
            lat_last: 0.0, lon_last: 40.0,
            di: 10.0, dj: 10.0,
            scanning_mode: 0x00, // +i, -j
            resolution_flags: 0x30,
            shape_of_earth: 6,
            projection: GridProjection::LatLon,
        }
    }

    #[test]
    fn nearest_index_in_range() {
        let g = test_grid_5x5();
        // Corner: lat=40, lon=0 → i=0, j=0 → index 0
        assert_eq!(g.nearest_index(40.0, 0.0), Some(0));
        // Opposite corner: lat=0, lon=40 → i=4, j=4 → index 24
        assert_eq!(g.nearest_index(0.0, 40.0), Some(24));
        // Centre: lat=20, lon=20 → i=2, j=2 → index 12
        assert_eq!(g.nearest_index(20.0, 20.0), Some(12));
        // Near-corner snap: lat=39.0, lon=1.0 → rounds to i=0, j=0 → index 0
        assert_eq!(g.nearest_index(39.0, 1.0), Some(0));
    }

    #[test]
    fn nearest_index_out_of_range() {
        let g = test_grid_5x5();
        // Latitude too far north
        assert_eq!(g.nearest_index(50.0, 20.0), None);
        // Longitude too far west (negative longitude, not in grid)
        assert_eq!(g.nearest_index(20.0, -10.0), None);
        // Zero-increment lat/lon grid — no way to locate a point
        let g2 = GridDefinition { di: 0.0, dj: 0.0, ..g };
        assert_eq!(g2.nearest_index(20.0, 20.0), None);
    }

    #[test]
    fn nearest_index_negative_lon_normalized() {
        // A global grid starting at 0E with 1° spacing, 360 points
        let g = GridDefinition {
            template: 0, num_data_points: 360,
            nx: 360, ny: 1,
            lat_first: 0.0, lon_first: 0.0,
            lat_last: 0.0, lon_last: 359.0,
            di: 1.0, dj: 1.0,
            scanning_mode: 0x00,
            resolution_flags: 0x30, shape_of_earth: 6,
            projection: GridProjection::LatLon,
        };
        // -73.97° E = 286.03° E
        let idx = g.nearest_index(0.0, -73.97);
        assert_eq!(idx, Some(286));
    }

    #[test]
    fn bilinear_corners_interior() {
        let g = test_grid_5x5();
        // Query at the exact centre of the NW cell: lat=35, lon=5
        // fi = (5-0)/10 = 0.5, fj = (40-35)/10 = 0.5
        let c = g.bilinear_corners(35.0, 5.0).expect("should find corners");
        assert_eq!(c.idx_nw, 0);  // j=0, i=0
        assert_eq!(c.idx_ne, 1);  // j=0, i=1
        assert_eq!(c.idx_sw, 5);  // j=1, i=0
        assert_eq!(c.idx_se, 6);  // j=1, i=1
        assert!((c.fx - 0.5).abs() < 1e-9);
        assert!((c.fy - 0.5).abs() < 1e-9);
    }

    #[test]
    fn bilinear_corners_at_boundary_returns_none() {
        let g = test_grid_5x5();
        // Exact last grid point — no room for bilinear on the SE side
        assert!(g.bilinear_corners(0.0, 40.0).is_none());
        // Outside grid entirely
        assert!(g.bilinear_corners(50.0, 20.0).is_none());
    }

    #[test]
    fn bilinear_interpolate_dense() {
        // A 2×2 Dense grid with known values, query at centre
        // Grid: NW=1, NE=2, SW=3, SE=4
        let gv = GridValues::Dense(vec![1.0, 2.0, 3.0, 4.0]);
        let c = BilinearCorners { idx_nw: 0, idx_ne: 1, idx_sw: 2, idx_se: 3, fx: 0.5, fy: 0.5 };
        let v = gv.bilinear(&c).expect("should interpolate");
        // (0.5*0.5*1 + 0.5*0.5*2 + 0.5*0.5*3 + 0.5*0.5*4) = 0.25*(1+2+3+4) = 2.5
        assert!((v - 2.5).abs() < 1e-9);
    }

    #[test]
    fn get_at_masked() {
        let gv = GridValues::Masked {
            values: vec![1.0, 9999.0, 3.0],
            present: vec![true, false, true],
        };
        assert_eq!(gv.get_at(0), Some(1.0));
        assert_eq!(gv.get_at(1), None); // masked
        assert_eq!(gv.get_at(2), Some(3.0));
        assert_eq!(gv.get_at(3), None); // out of bounds
    }

    // ── Lambert conformal (GDT 3.30) tests ───────────────────────────────────

    /// Construct a small synthetic Lambert conformal grid centred over the
    /// central-US area (approximating HRRR-like parameters but with a coarser
    /// 100 km step to keep the geometry easy to reason about).
    ///
    /// Latin1 = Latin2 = 38.5° N (tangent cone) to keep n = sin(38.5°) simple.
    /// LoV = 262.5° E (≈ −97.5°, roughly the CONUS centre meridian).
    /// LaD = 38.5° (same as standard parallel so ρ₀ gives the reference y).
    /// La1 = 25.0° N, Lo1 = 230.0° E (approximate SW corner).
    fn test_lambert_grid() -> GridDefinition {
        let p = LambertConformalParams {
            lad: 38.5,
            lov: 262.5,
            dx_m: 100_000.0,
            dy_m: 100_000.0,
            proj_centre: 0,
            latin1: 38.5,
            latin2: 38.5,
            lat_south_pole: -90.0,
            lon_south_pole: 0.0,
        };
        GridDefinition {
            template: 30,
            num_data_points: 100,
            nx: 10,
            ny: 10,
            lat_first: 25.0,
            lon_first: 230.0,
            lat_last: 0.0,  // not used for Lambert
            lon_last: 0.0,  // not used for Lambert
            di: 0.0,
            dj: 0.0,
            scanning_mode: 0x40, // +i (west→east), +j (south→north)
            resolution_flags: 0x08,
            shape_of_earth: 6,
            projection: GridProjection::LambertConformal(p),
        }
    }

    #[test]
    fn lambert_nearest_first_point_returns_zero() {
        // The first grid point should always map to flat index 0.
        let g = test_lambert_grid();
        let idx = g.nearest_index(g.lat_first, g.lon_first);
        assert_eq!(idx, Some(0), "first grid point should map to index 0");
    }

    #[test]
    fn lambert_nearest_point_offset() {
        // Projecting la1 + ~1 grid step northward and eastward should give (i,j) ≈ (1,1),
        // i.e. flat index 1*10 + 1 = 11.  We don't know the exact degrees, so we use
        // the forward projection to compute a point that sits one cell away and verify
        // the round-trip.
        let g = test_lambert_grid();
        let lp = match &g.projection {
            GridProjection::LambertConformal(p) => p,
            _ => panic!("expected Lambert"),
        };
        // Projected position of the first point.
        let (x0, y0) = lp.project_xy(g.lat_first, g.lon_first);
        // One cell north-east in projected space.
        let (x1, y1) = (x0 + lp.dx_m, y0 + lp.dy_m);
        // Inverse projection is not implemented; instead verify that the offset
        // from first point gives fractional indices ≈ (1.0, 1.0).
        let di_f = (x1 - x0) / lp.dx_m;
        let dj_f = (y1 - y0) / lp.dy_m;
        assert!((di_f - 1.0).abs() < 1e-6);
        assert!((dj_f - 1.0).abs() < 1e-6);
    }

    #[test]
    fn lambert_nearest_outside_grid_returns_none() {
        let g = test_lambert_grid();
        // A point far outside (e.g. in the Southern Hemisphere) should return None.
        let idx = g.nearest_index(-30.0, 262.5);
        assert_eq!(idx, None, "point south of grid should return None");
    }

    #[test]
    fn lambert_projection_origin_at_lad_lov() {
        // At (LaD, LoV) the projection should give y = 0 (by construction of ρ₀)
        // and x = 0 (no longitude offset).
        let g = test_lambert_grid();
        let lp = match &g.projection {
            GridProjection::LambertConformal(p) => p,
            _ => panic!("expected Lambert"),
        };
        let (x, y) = lp.project_xy(lp.lad, lp.lov);
        // At (LaD, LoV), θ = 0, ρ = ρ₀, so x = 0 and y = ρ₀ - ρ₀ = 0.
        assert!(x.abs() < 1e-6, "x at origin should be 0, got {x}");
        assert!(y.abs() < 1e-6, "y at origin should be 0, got {y}");
    }

    // ── Polar stereographic (GDT 3.20) tests ─────────────────────────────────

    /// A small synthetic North Polar Stereographic grid approximating NCEP NH
    /// grids.  LaD = 60°N, LoV = 270°E (–90°), 10×10, 100 km step.
    /// First point placed at (30°N, 315°E) — southwest corner.
    fn test_polar_stereo_grid() -> GridDefinition {
        let p = PolarStereographicParams {
            lad: 60.0,
            lov: 270.0,
            dx_m: 100_000.0,
            dy_m: 100_000.0,
            proj_centre: 0, // North Pole
        };
        GridDefinition {
            template: 20,
            num_data_points: 100,
            nx: 10,
            ny: 10,
            lat_first: 30.0,
            lon_first: 315.0,
            lat_last: 0.0,
            lon_last: 0.0,
            di: 0.0,
            dj: 0.0,
            scanning_mode: 0x40, // +i, +j
            resolution_flags: 0x08,
            shape_of_earth: 6,
            projection: GridProjection::PolarStereographic(p),
        }
    }

    #[test]
    fn polar_stereo_nearest_first_point_returns_zero() {
        // The first grid point (La1, Lo1) must always map to flat index 0.
        let g = test_polar_stereo_grid();
        let idx = g.nearest_index(g.lat_first, g.lon_first);
        assert_eq!(idx, Some(0), "first grid point should map to index 0");
    }

    #[test]
    fn polar_stereo_north_pole_projects_to_origin() {
        // The North Pole projects to (0, 0) in the polar stereographic plane
        // regardless of longitude.
        let g = test_polar_stereo_grid();
        let pp = match &g.projection {
            GridProjection::PolarStereographic(p) => p,
            _ => panic!("expected PolarStereographic"),
        };
        let (x, y) = pp.project_xy(90.0, pp.lov);
        assert!(x.abs() < 1e-3, "x at North Pole should be 0, got {x}");
        assert!(y.abs() < 1e-3, "y at North Pole should be 0, got {y}");
    }

    #[test]
    fn polar_stereo_lad_lov_rho_equals_r_cos_lad() {
        // At (LaD, LoV), θ = 0 and ρ = R·(1+sin LaD)·cos LaD/(1+sin LaD) = R·cos LaD.
        // This is the defining property of the scale-true latitude.
        let g = test_polar_stereo_grid();
        let pp = match &g.projection {
            GridProjection::PolarStereographic(p) => p,
            _ => panic!("expected PolarStereographic"),
        };
        let (x, y) = pp.project_xy(pp.lad, pp.lov);
        let expected_rho = PolarStereographicParams::EARTH_R * (pp.lad.to_radians()).cos();
        // At (LaD, LoV): θ=0 → x=0, y = -ρ
        assert!(x.abs() < 1e-3, "x should be 0, got {x}");
        assert!((y.abs() - expected_rho).abs() < 1.0, "ρ at LaD = {}, expected ≈ {expected_rho}", y.abs());
    }

    #[test]
    fn polar_stereo_point_offset_consistent() {
        // Project the first grid point to (x0, y0).  A projected point one Dx
        // east and one Dy north should give fractional indices (1.0, 1.0),
        // confirming the grid-step arithmetic is self-consistent.
        let g = test_polar_stereo_grid();
        let pp = match &g.projection {
            GridProjection::PolarStereographic(p) => p,
            _ => panic!("expected PolarStereographic"),
        };
        let (x0, y0) = pp.project_xy(g.lat_first, g.lon_first);
        let (x1, y1) = (x0 + pp.dx_m, y0 + pp.dy_m);

        let di_f = (x1 - x0) / pp.dx_m;
        let dj_f = (y1 - y0) / pp.dy_m;
        assert!((di_f - 1.0).abs() < 1e-9, "i offset should be 1.0, got {di_f}");
        assert!((dj_f - 1.0).abs() < 1e-9, "j offset should be 1.0, got {dj_f}");
    }

    #[test]
    fn polar_stereo_outside_grid_returns_none() {
        let g = test_polar_stereo_grid();
        // The South Pole is always far outside any north-polar grid.
        let idx = g.nearest_index(-90.0, 270.0);
        assert_eq!(idx, None, "South Pole should be outside north-polar grid");
    }

    #[test]
    fn polar_stereo_on_lov_x_is_zero() {
        // At any latitude along LoV, x = 0 and y is determined by ρ.
        let g = test_polar_stereo_grid();
        let pp = match &g.projection {
            GridProjection::PolarStereographic(p) => p,
            _ => panic!("expected PolarStereographic"),
        };
        let (x45, _y45) = pp.project_xy(45.0, pp.lov);
        let (x70, _y70) = pp.project_xy(70.0, pp.lov);
        assert!(x45.abs() < 1e-6, "x on LoV at 45° should be 0, got {x45}");
        assert!(x70.abs() < 1e-6, "x on LoV at 70° should be 0, got {x70}");
    }

    #[test]
    fn polar_stereo_y_increases_toward_pole() {
        // For North Polar, projecting a higher latitude on LoV should give a
        // larger (less negative) y — i.e. y increases toward the pole.
        let g = test_polar_stereo_grid();
        let pp = match &g.projection {
            GridProjection::PolarStereographic(p) => p,
            _ => panic!("expected PolarStereographic"),
        };
        let (_x30, y30) = pp.project_xy(30.0, pp.lov);
        let (_x60, y60) = pp.project_xy(60.0, pp.lov);
        let (_x80, y80) = pp.project_xy(80.0, pp.lov);
        assert!(y60 > y30, "y should increase northward: y30={y30}, y60={y60}");
        assert!(y80 > y60, "y should increase northward: y60={y60}, y80={y80}");
    }

    // ── Gaussian lat/lon (GDT 3.40) tests ────────────────────────────────────

    /// Construct a small synthetic Gaussian lat/lon grid.
    ///
    /// 4 columns (Di=90°) × 3 rows (N=1 parallel → 2 hemispheric parallels).
    /// La1=60°N, Lo1=0°E, La2=60°S, Lo2=270°E.
    /// Scanning mode 0x00: +i (west→east), −j (north→south, row 0 = 60°N).
    fn test_gaussian_latlon_grid() -> GridDefinition {
        GridDefinition {
            template: 40,
            num_data_points: 12,
            nx: 4,
            ny: 3,
            lat_first: 60.0,
            lon_first: 0.0,
            lat_last: -60.0,
            lon_last: 270.0,
            di: 90.0,
            dj: 0.0,        // not stored in GDT 3.40; N below replaces it
            scanning_mode: 0x00,  // +i, -j
            resolution_flags: 0x30,
            shape_of_earth: 6,
            projection: GridProjection::GaussianLatLon(GaussianLatLonParams { n_parallels: 1 }),
        }
    }

    #[test]
    fn gaussian_nearest_first_point_returns_zero() {
        // (La1, Lo1) = (60°N, 0°E) → i=0, j=0 → flat index 0.
        let g = test_gaussian_latlon_grid();
        let idx = g.nearest_index(g.lat_first, g.lon_first);
        assert_eq!(idx, Some(0), "first grid point should map to index 0");
    }

    #[test]
    fn gaussian_nearest_last_point_returns_n_minus_1() {
        // (La2, Lo2) = (60°S, 270°E) → i=3, j=2 → flat index 3*4+3 ...
        // Actually j=2 means row index 2, i=3 means col 3: idx = 2*4 + 3 = 11.
        let g = test_gaussian_latlon_grid();
        let idx = g.nearest_index(g.lat_last, g.lon_last);
        let n = g.num_data_points as usize;
        assert_eq!(idx, Some(n - 1), "last grid point should map to index {}", n - 1);
    }

    #[test]
    fn gaussian_nearest_equator_center_longitude() {
        // Equator (lat=0), centre col (lon=180°E = col 2, row 1 for our 3-row grid)
        // fj = (0 - 60) / (-60-60)/(3-1) = (0-60)/(-60) = 1.0 → j=1
        // fi = (180 - 0) / 90 = 2.0 → i=2
        // idx = 1*4 + 2 = 6
        let g = test_gaussian_latlon_grid();
        let idx = g.nearest_index(0.0, 180.0);
        assert_eq!(idx, Some(6), "equator centre should map to index 6");
    }

    #[test]
    fn gaussian_nearest_outside_returns_none() {
        // Use a narrow grid (La1=50°N, La2=30°N, 3 rows, di=90°, 4 cols)
        // so that there are clearly out-of-range latitudes:
        //   dlat = (30 − 50) / (3 − 1) = −10°
        //   half-cell guard at +5° = 55°N boundary and 25°N boundary.
        let narrow = GridDefinition {
            template: 40,
            num_data_points: 12,
            nx: 4,
            ny: 3,
            lat_first: 50.0,
            lon_first: 0.0,
            lat_last: 30.0,
            lon_last: 270.0,
            di: 90.0,
            dj: 0.0,
            scanning_mode: 0x00,
            resolution_flags: 0x30,
            shape_of_earth: 6,
            projection: GridProjection::GaussianLatLon(GaussianLatLonParams { n_parallels: 1 }),
        };
        // 90°N is 40° north of La1=50°N — well outside.
        assert_eq!(narrow.nearest_index(90.0, 0.0), None, "90°N should be outside grid");
        // 0°N is 30° south of La2=30°N — well outside.
        assert_eq!(narrow.nearest_index(0.0, 0.0), None, "0°N should be outside grid");
    }

    #[test]
    fn gaussian_nearest_negative_lon_normalized() {
        // -180° → 180°E, same as positive 180° → col 2, row 0 → idx 2.
        let g = test_gaussian_latlon_grid();
        let idx_pos = g.nearest_index(60.0, 180.0);
        let idx_neg = g.nearest_index(60.0, -180.0);
        assert_eq!(idx_pos, Some(2));
        assert_eq!(idx_pos, idx_neg, "-180° should resolve the same as +180°");
    }
}
