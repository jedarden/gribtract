//! GRIB2 message decoder — section splitter + template dispatch.
//!
//! Parses raw GRIB2 bytes into [`Field`] values by:
//!   1. Splitting the byte stream into typed sections (0–8).
//!   2. Dispatching each section to its template-specific parser.
//!   3. Assembling a [`Field`] for each complete 3+4+5+6+7 group.
//!
//! Only the templates required to decode the current corpus are implemented.
//! Unsupported templates return [`Error::NotImplemented`].

use crate::error::{Error, Result};
use crate::types::{
    ComplexExtra, Ensemble, Field, ForecastTime, GaussianLatLonParams, GridDefinition,
    GridProjection, GridValues, LazyField, LambertConformalParams, Level, PackingInfo,
    ParameterId, PolarStereographicParams, ReferenceTime,
};

// ── Byte buffer reader ────────────────────────────────────────────────────────

struct Buf<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Buf<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    fn need(&self, n: usize) -> Result<()> {
        if self.remaining() < n {
            Err(Error::TooShort { needed: n, got: self.remaining() })
        } else {
            Ok(())
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        self.need(1)?;
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_u16be(&mut self) -> Result<u16> {
        self.need(2)?;
        let v = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    fn read_u32be(&mut self) -> Result<u32> {
        self.need(4)?;
        let v = u32::from_be_bytes(
            self.data[self.pos..self.pos + 4].try_into().unwrap(),
        );
        self.pos += 4;
        Ok(v)
    }

    fn read_u64be(&mut self) -> Result<u64> {
        self.need(8)?;
        let v = u64::from_be_bytes(
            self.data[self.pos..self.pos + 8].try_into().unwrap(),
        );
        self.pos += 8;
        Ok(v)
    }

    fn read_f32be(&mut self) -> Result<f32> {
        self.need(4)?;
        let v = f32::from_be_bytes(
            self.data[self.pos..self.pos + 4].try_into().unwrap(),
        );
        self.pos += 4;
        Ok(v)
    }

    // GRIB2 sign-magnitude 16-bit: bit 15 = sign, bits 14-0 = magnitude.
    fn read_scale_factor_i16(&mut self) -> Result<i16> {
        let raw = self.read_u16be()?;
        let magnitude = (raw & 0x7FFF) as i16;
        if raw & 0x8000 != 0 { Ok(-magnitude) } else { Ok(magnitude) }
    }

    // Lat/lon values are signed 32-bit in GRIB2 template 3.0:
    // bit 31 = sign (0=N/E, 1=S/W), bits 30-0 = magnitude in micro-degrees.
    fn read_latlon_micro(&mut self) -> Result<i64> {
        let raw = self.read_u32be()?;
        let magnitude = (raw & 0x7FFF_FFFF) as i64;
        if raw & 0x8000_0000 != 0 { Ok(-magnitude) } else { Ok(magnitude) }
    }

    // Longitude is always 0..360 (unsigned micro-degrees).
    fn read_longi_micro(&mut self) -> Result<u32> {
        self.read_u32be()
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        self.need(n)?;
        self.pos += n;
        Ok(())
    }

    #[allow(dead_code)]
    fn rest(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }
}

// ── Extra parameters for DRT=2/3 (complex packing) ───────────────────────────

/// Private alias for the public `ComplexExtra` type.
///
/// Using the public type everywhere avoids a conversion step when building
/// `LazyField`.  The public type is defined in `crate::types`.
type ComplexPackingExtra = ComplexExtra;

// ── Intermediate parsing state accumulated across sections ────────────────────

#[derive(Default)]
struct FieldBuilder {
    // From Section 1
    center: Option<u16>,
    subcenter: Option<u16>,
    ref_time: Option<ReferenceTime>,
    // From Section 4
    parameter: Option<ParameterId>,
    forecast: Option<ForecastTime>,
    level: Option<Level>,
    ensemble: Option<Option<Ensemble>>,
    pdt_template: Option<u16>,
    // From Section 3
    grid: Option<GridDefinition>,
    gdt_template: Option<u16>,
    // From Section 5
    packing: Option<PackingInfo>,
    drt_template: Option<u16>,
    complex_extra: Option<ComplexPackingExtra>,
    /// Number of packed data values (Section 5 oct 6-9). May be less than
    /// grid.num_data_points when a bitmap is present.
    n_packed: Option<usize>,
    // From Section 6
    has_bitmap: Option<bool>,
    /// Actual bitmap bytes (one bit per grid point, MSB-first). Present only
    /// when Section 6 indicator == 0. Length == ceil(n_grid_points / 8).
    bitmap: Option<Vec<u8>>,
    // From Section 7
    values: Option<GridValues>,
}

impl FieldBuilder {
    fn build(self) -> Option<Field> {
        // All required pieces must be present.
        let center = self.center?;
        let subcenter = self.subcenter?;
        let ref_time = self.ref_time?;
        let parameter = self.parameter?;
        let mut forecast = self.forecast?;
        // Back-fill the reference time into ForecastTime (it's parsed from Section 1).
        forecast.reference_time = ref_time;
        let level = self.level?;
        let ensemble = self.ensemble.unwrap_or(None);
        let grid = self.grid?;
        let gdt_template = self.gdt_template?;
        let pdt_template = self.pdt_template?;
        let packing = self.packing?;
        let drt_template = self.drt_template?;
        let values = self.values?;
        Some(Field {
            center,
            subcenter,
            parameter,
            forecast,
            level,
            ensemble,
            grid,
            values,
            gdt_template,
            pdt_template,
            drt_template,
            packing,
        })
    }
}

// ── Top-level decode entry point ──────────────────────────────────────────────

/// Decode all fields from a concatenated GRIB2 byte buffer.
///
/// Handles multiple messages (GRIB2 files may concatenate them). Within each
/// message, repeated section groups (3–7) produce multiple fields.
pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<Field>> {
    let mut fields = Vec::new();
    let mut pos = 0;

    while pos < bytes.len() {
        // Peek at next 4 bytes: expect "GRIB".
        if bytes.len() - pos < 4 {
            break;
        }
        if &bytes[pos..pos + 4] != b"GRIB" {
            return Err(Error::BadMagic(bytes[pos..pos + 4].try_into().unwrap()));
        }

        let msg_len = decode_message(&bytes[pos..], &mut fields)?;
        pos += msg_len;
    }

    Ok(fields)
}

/// Decode one GRIB2 message (starting at byte 0 of `msg`).
/// Returns the number of bytes consumed (the total message length).
fn decode_message(msg: &[u8], out: &mut Vec<Field>) -> Result<usize> {
    let mut buf = Buf::new(msg);

    // ── Section 0 (Indicator, 16 bytes fixed) ────────────────────────────────
    // oct 1-4: "GRIB"
    let magic = [buf.read_u8()?, buf.read_u8()?, buf.read_u8()?, buf.read_u8()?];
    if &magic != b"GRIB" {
        return Err(Error::BadMagic(magic));
    }
    buf.skip(2)?; // reserved
    let discipline = buf.read_u8()?;
    let edition = buf.read_u8()?;
    if edition != 2 {
        return Err(Error::UnknownEdition(edition));
    }
    let total_len = buf.read_u64be()? as usize;
    // Section 0 is now consumed (16 bytes).

    // Validate total length against buffer.
    if msg.len() < total_len {
        return Err(Error::TooShort { needed: total_len, got: msg.len() });
    }

    // ── Section iterator (sections 1-7 + "7777") ─────────────────────────────
    let mut builder = FieldBuilder::default();

    // The message body ends just before the "7777" end marker.
    let body_end = total_len - 4;

    while buf.pos < body_end {
        // Each section starts with: 4-byte length, 1-byte section number.
        let sec_start = buf.pos;
        let sec_len = buf.read_u32be()? as usize;
        let sec_num = buf.read_u8()?;

        if sec_len < 5 {
            return Err(Error::TooShort { needed: 5, got: sec_len });
        }
        // Remaining bytes in this section (after the 5-byte header).
        let body_start = buf.pos;
        let body_len = sec_len - 5;
        let sec_body = &msg[body_start..body_start + body_len];

        match sec_num {
            1 => {
                let (center, subcenter, ref_time) = parse_section1(sec_body)?;
                builder.center = Some(center);
                builder.subcenter = Some(subcenter);
                builder.ref_time = Some(ref_time);
            }
            2 => { /* Local Use section — skip */ }
            3 => {
                let (gdt, grid) = parse_section3(sec_body)?;
                builder.gdt_template = Some(gdt);
                builder.grid = Some(grid);
            }
            4 => {
                let (pdt, param, fore, lvl, ens) = parse_section4(sec_body, discipline)?;
                builder.pdt_template = Some(pdt);
                builder.parameter = Some(param);
                builder.forecast = Some(fore);
                builder.level = Some(lvl);
                builder.ensemble = Some(ens);
            }
            5 => {
                let (drt, packing, complex_extra, n_packed) = parse_section5(sec_body)?;
                builder.drt_template = Some(drt);
                builder.packing = Some(packing);
                builder.complex_extra = complex_extra;
                builder.n_packed = Some(n_packed);
            }
            6 => {
                let (has_bitmap, bitmap) = parse_section6(sec_body)?;
                builder.has_bitmap = Some(has_bitmap);
                builder.bitmap = bitmap;
            }
            7 => {
                let n_grid = builder.grid.as_ref().map(|g| g.num_data_points as usize).unwrap_or(0);
                let has_bitmap = builder.has_bitmap.unwrap_or(false);
                // When a bitmap is present, Section 5 n_packed < n_grid. Use n_packed
                // for the data unpackers (which only see the "present" subset).
                let n_decode = if has_bitmap {
                    builder.n_packed.unwrap_or(n_grid)
                } else {
                    n_grid
                };
                let packing = builder.packing.as_ref().ok_or(Error::NotImplemented)?;
                let drt = builder.drt_template.unwrap_or(0);
                let packed_values = decode_section7(
                    sec_body, packing, drt, builder.complex_extra.as_ref(), n_decode,
                )?;
                // Expand packed values into the full grid using the bitmap (if present).
                let values = if has_bitmap {
                    expand_bitmap(packed_values, &builder.bitmap, n_grid)
                } else {
                    packed_values
                };
                builder.values = Some(values);

                // Complete field — flush the builder.
                let next_builder = FieldBuilder {
                    center: builder.center,
                    subcenter: builder.subcenter,
                    ref_time: builder.ref_time,
                    ..Default::default()
                };
                if let Some(field) = builder.build() {
                    out.push(field);
                }
                builder = next_builder;
            }
            _ => { /* Unknown section — skip */ }
        }

        // Advance buf.pos to the end of this section.
        buf.pos = sec_start + sec_len;
    }

    // Consume "7777" end marker.
    let end = &msg[body_end..body_end + 4];
    if end != b"7777" {
        return Err(Error::TooShort { needed: 4, got: 0 });
    }

    Ok(total_len)
}

// ── Section 1: Identification ─────────────────────────────────────────────────

fn parse_section1(body: &[u8]) -> Result<(u16, u16, ReferenceTime)> {
    let mut b = Buf::new(body);
    // oct 6-7 of section (indices 0-1 in body): originating center
    let center = b.read_u16be()?;
    // oct 8-9: originating subcenter
    let subcenter = b.read_u16be()?;
    b.skip(1)?; // master tables version
    b.skip(1)?; // local tables version
    let significance = b.read_u8()?;
    let year = b.read_u16be()?;
    let month = b.read_u8()?;
    let day = b.read_u8()?;
    let hour = b.read_u8()?;
    let minute = b.read_u8()?;
    let second = b.read_u8()?;
    // production status and type of data — not stored in Field currently
    Ok((
        center,
        subcenter,
        ReferenceTime { year, month, day, hour, minute, second, significance },
    ))
}

// ── Section 3: Grid Definition ────────────────────────────────────────────────

fn parse_section3(body: &[u8]) -> Result<(u16, GridDefinition)> {
    let mut b = Buf::new(body);
    b.skip(1)?; // source of grid definition (oct 6)
    let num_data_points = b.read_u32be()?; // oct 7-10
    b.skip(1)?; // optional list octets count (oct 11)
    b.skip(1)?; // interpretation of optional list (oct 12)
    let template = b.read_u16be()?; // oct 13-14

    match template {
        0 => {
            let grid = parse_gdt_0(&mut b, num_data_points)?;
            Ok((0, grid))
        }
        20 => {
            let grid = parse_gdt_20(&mut b, num_data_points)?;
            Ok((20, grid))
        }
        30 => {
            let grid = parse_gdt_30(&mut b, num_data_points)?;
            Ok((30, grid))
        }
        40 => {
            let grid = parse_gdt_40(&mut b, num_data_points)?;
            Ok((40, grid))
        }
        _ => Err(Error::NotImplemented),
    }
}

/// Grid Definition Template 3.0: Latitude/Longitude.
fn parse_gdt_0(b: &mut Buf, num_data_points: u32) -> Result<GridDefinition> {
    let shape_of_earth = b.read_u8()?; // oct 15

    // Scale/value for earth shape (spheroid parameters) — not needed for GDT 3.0 geometry.
    b.skip(1 + 4)?; // spherical radius scale + value (oct 16-20)
    b.skip(1 + 4)?; // major axis scale + value (oct 21-25)
    b.skip(1 + 4)?; // minor axis scale + value (oct 26-30)

    let nx = b.read_u32be()?; // oct 31-34: Ni
    let ny = b.read_u32be()?; // oct 35-38: Nj

    b.skip(4)?; // basic angle of production domain (oct 39-42)
    b.skip(4)?; // subdivisions of basic angle (oct 43-46)

    let lat_first = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 47-50
    let lon_first = b.read_longi_micro()? as f64 / 1_000_000.0;   // oct 51-54

    let resolution_flags = b.read_u8()?; // oct 55

    let lat_last = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 56-59
    let lon_last = b.read_longi_micro()? as f64 / 1_000_000.0;   // oct 60-63
    let di = b.read_u32be()? as f64 / 1_000_000.0;               // oct 64-67
    let dj = b.read_u32be()? as f64 / 1_000_000.0;               // oct 68-71

    let scanning_mode = b.read_u8()?; // oct 72

    Ok(GridDefinition {
        template: 0,
        num_data_points,
        nx,
        ny,
        lat_first,
        lon_first,
        lat_last,
        lon_last,
        di,
        dj,
        scanning_mode,
        resolution_flags,
        shape_of_earth,
        projection: GridProjection::LatLon,
    })
}

/// Grid Definition Template 3.20: Polar Stereographic Projection.
///
/// Byte layout (b starts at section octet 15 = first body byte after template number):
///
/// | Octets | Field |
/// |--------|-------|
/// | 15     | shape of earth |
/// | 16–20  | earth radius scale + value (skipped) |
/// | 21–25  | major-axis scale + value (skipped) |
/// | 26–30  | minor-axis scale + value (skipped) |
/// | 31–34  | Nx |
/// | 35–38  | Ny |
/// | 39–42  | La1 (signed microdegrees) |
/// | 43–46  | Lo1 (unsigned microdegrees, 0–360) |
/// | 47     | resolution/component flags |
/// | 48–51  | LaD (signed microdegrees, latitude where Dx/Dy are specified) |
/// | 52–55  | LoV (unsigned microdegrees, orientation / central meridian) |
/// | 56–59  | Dx in metres |
/// | 60–63  | Dy in metres |
/// | 64     | projection centre flag (Table 3.5; bit 7=0 North Pole, bit 7=1 South Pole) |
/// | 65     | scanning mode |
fn parse_gdt_20(b: &mut Buf, num_data_points: u32) -> Result<GridDefinition> {
    let shape_of_earth = b.read_u8()?;    // oct 15
    b.skip(1 + 4)?;                       // oct 16–20: earth radius scale + value
    b.skip(1 + 4)?;                       // oct 21–25: major axis
    b.skip(1 + 4)?;                       // oct 26–30: minor axis

    let nx = b.read_u32be()?;             // oct 31–34: Nx
    let ny = b.read_u32be()?;             // oct 35–38: Ny

    let lat_first = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 39–42: La1
    let lon_first = b.read_longi_micro()? as f64 / 1_000_000.0;  // oct 43–46: Lo1
    let resolution_flags = b.read_u8()?;  // oct 47

    let lad = b.read_latlon_micro()? as f64 / 1_000_000.0;       // oct 48–51: LaD
    let lov = b.read_longi_micro()? as f64 / 1_000_000.0;        // oct 52–55: LoV

    let dx_m = b.read_u32be()? as f64;   // oct 56–59: Dx in metres
    let dy_m = b.read_u32be()? as f64;   // oct 60–63: Dy in metres

    let proj_centre = b.read_u8()?;       // oct 64
    let scanning_mode = b.read_u8()?;     // oct 65

    let projection = GridProjection::PolarStereographic(PolarStereographicParams {
        lad, lov, dx_m, dy_m, proj_centre,
    });

    Ok(GridDefinition {
        template: 20,
        num_data_points,
        nx,
        ny,
        lat_first,
        lon_first,
        lat_last: 0.0,  // not stored in GDT 3.20; derive via projection if needed
        lon_last: 0.0,
        di: 0.0,        // increment in metres, stored in PolarStereographicParams
        dj: 0.0,
        scanning_mode,
        resolution_flags,
        shape_of_earth,
        projection,
    })
}

/// Grid Definition Template 3.30: Lambert Conformal.
///
/// Byte layout (b starts at section octet 15 = first body byte after template number):
///
/// | Octets | Field |
/// |--------|-------|
/// | 15     | shape of earth |
/// | 16–20  | earth radius scale + value (skipped) |
/// | 21–25  | major-axis scale + value (skipped) |
/// | 26–30  | minor-axis scale + value (skipped) |
/// | 31–34  | Nx |
/// | 35–38  | Ny |
/// | 39–42  | La1 (signed microdegrees) |
/// | 43–46  | Lo1 (unsigned microdegrees, 0–360) |
/// | 47     | resolution/component flags |
/// | 48–51  | LaD (signed microdegrees) |
/// | 52–55  | LoV (unsigned microdegrees) |
/// | 56–59  | Dx in metres |
/// | 60–63  | Dy in metres |
/// | 64     | projection centre flag |
/// | 65     | scanning mode |
/// | 66–69  | Latin1 (signed microdegrees) |
/// | 70–73  | Latin2 (signed microdegrees) |
/// | 74–77  | lat south pole (signed microdegrees) |
/// | 78–81  | lon south pole (unsigned microdegrees) |
fn parse_gdt_30(b: &mut Buf, num_data_points: u32) -> Result<GridDefinition> {
    let shape_of_earth = b.read_u8()?;    // oct 15
    b.skip(1 + 4)?;                       // oct 16–20: earth radius scale + value
    b.skip(1 + 4)?;                       // oct 21–25: major axis
    b.skip(1 + 4)?;                       // oct 26–30: minor axis

    let nx = b.read_u32be()?;             // oct 31–34: Nx
    let ny = b.read_u32be()?;             // oct 35–38: Ny

    let lat_first = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 39–42: La1
    let lon_first = b.read_longi_micro()? as f64 / 1_000_000.0;  // oct 43–46: Lo1
    let resolution_flags = b.read_u8()?;  // oct 47

    let lad = b.read_latlon_micro()? as f64 / 1_000_000.0;       // oct 48–51: LaD
    let lov = b.read_longi_micro()? as f64 / 1_000_000.0;        // oct 52–55: LoV

    let dx_m = b.read_u32be()? as f64;   // oct 56–59: Dx in metres
    let dy_m = b.read_u32be()? as f64;   // oct 60–63: Dy in metres

    let proj_centre = b.read_u8()?;       // oct 64
    let scanning_mode = b.read_u8()?;     // oct 65

    let latin1 = b.read_latlon_micro()? as f64 / 1_000_000.0;    // oct 66–69
    let latin2 = b.read_latlon_micro()? as f64 / 1_000_000.0;    // oct 70–73
    let lat_south_pole = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 74–77
    let lon_south_pole = b.read_longi_micro()? as f64 / 1_000_000.0;  // oct 78–81

    let projection = GridProjection::LambertConformal(LambertConformalParams {
        lad, lov, dx_m, dy_m, proj_centre, latin1, latin2, lat_south_pole, lon_south_pole,
    });

    Ok(GridDefinition {
        template: 30,
        num_data_points,
        nx,
        ny,
        lat_first,
        lon_first,
        lat_last: 0.0, // not stored in GDT 3.30; derive via projection if needed
        lon_last: 0.0,
        di: 0.0,       // increment in metres, stored in LambertConformalParams
        dj: 0.0,
        scanning_mode,
        resolution_flags,
        shape_of_earth,
        projection,
    })
}

/// Grid Definition Template 3.40: Gaussian Latitude/Longitude.
///
/// Byte layout (b starts at section octet 15 = first body byte after template number):
///
/// | Octets | Field |
/// |--------|-------|
/// | 15     | shape of earth |
/// | 16–20  | earth radius scale + value (skipped) |
/// | 21–25  | major-axis scale + value (skipped) |
/// | 26–30  | minor-axis scale + value (skipped) |
/// | 31–34  | Nx (Ni) |
/// | 35–38  | Ny (Nj) |
/// | 39–42  | basic angle (skipped) |
/// | 43–46  | basic angle subdivisions (skipped) |
/// | 47–50  | La1 (signed microdegrees) |
/// | 51–54  | Lo1 (unsigned microdegrees, 0–360) |
/// | 55     | resolution/component flags |
/// | 56–59  | La2 (signed microdegrees) |
/// | 60–63  | Lo2 (unsigned microdegrees, 0–360) |
/// | 64–67  | Di (unsigned microdegrees/1e6) |
/// | 68–71  | N (number of parallels between pole and equator) |
/// | 72     | scanning mode |
///
/// GDT 3.40 differs from GDT 3.0 only in octets 68–71: these hold N (a raw
/// unsigned 32-bit integer, not scaled) instead of Dj.  Dj is left as 0.0.
fn parse_gdt_40(b: &mut Buf, num_data_points: u32) -> Result<GridDefinition> {
    let shape_of_earth = b.read_u8()?;    // oct 15
    b.skip(1 + 4)?;                       // oct 16–20: earth radius scale + value
    b.skip(1 + 4)?;                       // oct 21–25: major axis
    b.skip(1 + 4)?;                       // oct 26–30: minor axis

    let nx = b.read_u32be()?;             // oct 31–34
    let ny = b.read_u32be()?;             // oct 35–38

    b.skip(4)?;                           // oct 39–42: basic angle
    b.skip(4)?;                           // oct 43–46: subdivisions

    let lat_first = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 47–50
    let lon_first = b.read_longi_micro()? as f64 / 1_000_000.0;  // oct 51–54
    let resolution_flags = b.read_u8()?;  // oct 55
    let lat_last  = b.read_latlon_micro()? as f64 / 1_000_000.0; // oct 56–59
    let lon_last  = b.read_longi_micro()? as f64 / 1_000_000.0;  // oct 60–63

    let di        = b.read_u32be()? as f64 / 1_000_000.0;        // oct 64–67
    let n_parallels = b.read_u32be()?;                            // oct 68–71 (N, not Dj)
    let scanning_mode = b.read_u8()?;     // oct 72

    let projection = GridProjection::GaussianLatLon(GaussianLatLonParams { n_parallels });

    Ok(GridDefinition {
        template: 40,
        num_data_points,
        nx,
        ny,
        lat_first,
        lon_first,
        lat_last,
        lon_last,
        di,
        dj: 0.0,   // not stored in GDT 3.40; N replaces Dj
        scanning_mode,
        resolution_flags,
        shape_of_earth,
        projection,
    })
}

// ── Section 4: Product Definition ────────────────────────────────────────────

fn parse_section4(
    body: &[u8],
    discipline: u8,
) -> Result<(u16, ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let mut b = Buf::new(body);
    let _n_coord = b.read_u16be()?; // oct 6-7: number of coordinate values
    let template = b.read_u16be()?; // oct 8-9: PDT template number

    match template {
        0 => {
            let (param, fore, lvl, ens) = parse_pdt_0(&mut b, discipline)?;
            Ok((0, param, fore, lvl, ens))
        }
        1 => {
            let (param, fore, lvl, ens) = parse_pdt_1(&mut b, discipline)?;
            Ok((1, param, fore, lvl, ens))
        }
        8 => {
            let (param, fore, lvl, ens) = parse_pdt_8(&mut b, discipline)?;
            Ok((8, param, fore, lvl, ens))
        }
        11 => {
            let (param, fore, lvl, ens) = parse_pdt_11(&mut b, discipline)?;
            Ok((11, param, fore, lvl, ens))
        }
        _ => Err(Error::NotImplemented),
    }
}

/// Parse the common PDT header fields shared by templates 4.0, 4.1, 4.8, 4.11.
///
/// Reads octets 10–34 (parameter, time, and level fields).  The caller reads
/// any template-specific tail octets after calling this function.
fn parse_pdt_common_header(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level)> {
    let category = b.read_u8()?;  // oct 10
    let number = b.read_u8()?;    // oct 11
    b.skip(1)?; // generating process type (oct 12)
    b.skip(1)?; // background generating process id (oct 13)
    b.skip(1)?; // analysis/forecast generating process id (oct 14)
    b.skip(2)?; // hours of cutoff after reference time (oct 15-16)
    b.skip(1)?; // minutes of cutoff (oct 17)
    let time_range_unit = b.read_u8()?;    // oct 18
    let forecast_offset = b.read_u32be()?; // oct 19-22

    let type1 = b.read_u8()?;                     // oct 23
    let scale_factor1 = b.read_i8()?;             // oct 24
    let scaled_value1 = b.read_u32be()? as i32;   // oct 25-28

    let raw_type2 = b.read_u8()?;                     // oct 29
    let raw_scale_factor2 = b.read_i8()?;             // oct 30
    let raw_scaled_value2 = b.read_u32be()? as i32;   // oct 31-34

    // When second surface type is 255 (missing), zero out its scale/value
    // to match eccodes behavior.
    let (type2, scale_factor2, scaled_value2) = if raw_type2 == 255 {
        (255u8, 0i8, 0i32)
    } else {
        (raw_type2, raw_scale_factor2, raw_scaled_value2)
    };

    let param = ParameterId { discipline, category, number };
    // Reference time is filled in from Section 1 by the caller.
    let fore = ForecastTime {
        reference_time: ReferenceTime {
            year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, significance: 0,
        },
        time_range_unit,
        forecast_offset,
    };
    let lvl = Level { type1, scale_factor1, scaled_value1, type2, scale_factor2, scaled_value2 };

    Ok((param, fore, lvl))
}

/// Product Definition Template 4.0: Analysis or forecast at horizontal level.
fn parse_pdt_0(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let (param, fore, lvl) = parse_pdt_common_header(b, discipline)?;
    Ok((param, fore, lvl, None))
}

/// Product Definition Template 4.1: Individual ensemble forecast, control and
/// perturbed, at a horizontal level at a point in time.
///
/// WMO GRIB2 Table 4.1:
/// - Octs 10–34: common header (same as PDT 4.0)
/// - Oct 35: type of ensemble forecast (Table 4.6)
/// - Oct 36: perturbation number
/// - Oct 37: number of forecasts in ensemble
fn parse_pdt_1(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let (param, fore, lvl) = parse_pdt_common_header(b, discipline)?;

    let member_type = b.read_u8()?;          // oct 35: Table 4.6
    let perturbation_number = b.read_u8()?;  // oct 36
    let _n_ensemble = b.read_u8()?;          // oct 37: number in ensemble (unused by harness)

    let ens = Ensemble { member_type, number: perturbation_number as i16 };
    Ok((param, fore, lvl, Some(ens)))
}

/// Product Definition Template 4.8: Average, accumulation, extreme values or
/// other statistically-processed values at a horizontal level in a continuous
/// or non-continuous time interval.
///
/// WMO GRIB2 Table 4.8:
/// - Octs 10–34: common header (same as PDT 4.0)
/// - Oct 35: year (u16), month, day, hour, minute, second of end of interval
/// - Oct 41: number of time range specifications (n)
/// - Octs 42 onward: n × 12-byte time range specifications (skipped)
///
/// Only the common header fields are surfaced; the statistical window
/// information is not yet modelled in `ForecastTime`.
fn parse_pdt_8(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let (param, fore, lvl) = parse_pdt_common_header(b, discipline)?;

    // End-of-interval timestamp and time-range specs: skip entirely.
    // Oct 35-40: end time (year u16 + month + day + hour + min + sec = 7 bytes)
    b.skip(7)?;
    // Oct 41: number of time range specs
    let n_ranges = b.read_u8()? as usize;
    // Each spec is 12 bytes.
    b.skip(12 * n_ranges)?;

    Ok((param, fore, lvl, None))
}

/// Product Definition Template 4.11: Individual ensemble member +
/// time-processed statistical product at a horizontal level.
///
/// WMO GRIB2 Table 4.11:
/// - Octs 10–34: common header (same as PDT 4.0)
/// - Oct 35: type of ensemble forecast (Table 4.6)
/// - Oct 36: perturbation number
/// - Oct 37: number of forecasts in ensemble
/// - Oct 38-43: end-of-interval timestamp (year u16 + month + day + hour + min + sec)
/// - Oct 44: number of time range specifications (n)
/// - Octs 45+: n × 12-byte time range specs (skipped)
fn parse_pdt_11(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let (param, fore, lvl) = parse_pdt_common_header(b, discipline)?;

    let member_type = b.read_u8()?;          // oct 35
    let perturbation_number = b.read_u8()?;  // oct 36
    let _n_ensemble = b.read_u8()?;          // oct 37

    // Oct 38-43: end-of-interval timestamp (7 bytes: year u16 + month + day + hour + min + sec)
    b.skip(7)?;
    // Oct 44: number of time range specs
    let n_ranges = b.read_u8()? as usize;
    b.skip(12 * n_ranges)?;

    let ens = Ensemble { member_type, number: perturbation_number as i16 };
    Ok((param, fore, lvl, Some(ens)))
}

// ── Section 5: Data Representation ───────────────────────────────────────────

fn parse_section5(body: &[u8]) -> Result<(u16, PackingInfo, Option<ComplexPackingExtra>, usize)> {
    let mut b = Buf::new(body);
    let n_values = b.read_u32be()? as usize; // oct 6-9: number of packed values
    let template = b.read_u16be()?;           // oct 10-11: DRT template number

    match template {
        0 => {
            let packing = parse_drt_common(&mut b)?;
            Ok((0, packing, None, n_values))
        }
        2 => {
            let (packing, extra) = parse_drt_2(&mut b)?;
            Ok((2, packing, Some(extra), n_values))
        }
        3 => {
            let (packing, extra) = parse_drt_3(&mut b)?;
            Ok((3, packing, Some(extra), n_values))
        }
        40 => {
            // Template 5.40: JPEG 2000 data compression — same common header as DRT=0.
            let packing = parse_drt_common(&mut b)?;
            Ok((40, packing, None, n_values))
        }
        41 => {
            // Template 5.41: PNG data compression — same common header as DRT=0.
            let packing = parse_drt_common(&mut b)?;
            Ok((41, packing, None, n_values))
        }
        _ => Err(Error::NotImplemented),
    }
}

/// Parse the common packing header shared by DRT templates 0, 2, and 3.
fn parse_drt_common(b: &mut Buf) -> Result<PackingInfo> {
    let reference_value = b.read_f32be()?;               // oct 12-15: R
    let binary_scale_factor = b.read_scale_factor_i16()?; // oct 16-17: E
    let decimal_scale_factor = b.read_scale_factor_i16()?; // oct 18-19: D
    let bits_per_value = b.read_u8()?;                   // oct 20: N
    let original_field_type = b.read_u8()?;              // oct 21

    Ok(PackingInfo {
        reference_value,
        binary_scale_factor,
        decimal_scale_factor,
        bits_per_value,
        original_field_type,
    })
}

/// Data Representation Template 5.2: Complex packing (no spatial differencing).
fn parse_drt_2(b: &mut Buf) -> Result<(PackingInfo, ComplexPackingExtra)> {
    let packing = parse_drt_common(b)?;  // oct 12-21: common header

    b.skip(1)?; // oct 22: group splitting method
    b.skip(1)?; // oct 23: missing value management (0 = none)
    b.skip(4)?; // oct 24-27: primary missing value
    b.skip(4)?; // oct 28-31: secondary missing value

    let n_groups = b.read_u32be()?;              // oct 32-35
    let ref_group_widths = b.read_u8()?;         // oct 36
    let bits_group_widths = b.read_u8()?;        // oct 37
    let ref_group_lengths = b.read_u32be()?;     // oct 38-41
    let length_increment = b.read_u8()?;         // oct 42
    let true_last_group_length = b.read_u32be()?; // oct 43-46
    let bits_scaled_group_lengths = b.read_u8()?; // oct 47
    // Octets 48-49 (order_spatial_diff, extra_octet_count) are absent in template 5.2.

    Ok((packing, ComplexPackingExtra {
        n_groups,
        ref_group_widths,
        bits_group_widths,
        ref_group_lengths,
        length_increment,
        true_last_group_length,
        bits_scaled_group_lengths,
        order_spatial_diff: 0,
        extra_octet_count: 0,
    }))
}

/// Data Representation Template 5.3: Complex packing with spatial differencing.
fn parse_drt_3(b: &mut Buf) -> Result<(PackingInfo, ComplexPackingExtra)> {
    let packing = parse_drt_common(b)?;  // oct 12-21: common header

    b.skip(1)?; // oct 22: group splitting method
    b.skip(1)?; // oct 23: missing value management (0 = none)
    b.skip(4)?; // oct 24-27: primary missing value
    b.skip(4)?; // oct 28-31: secondary missing value

    let n_groups = b.read_u32be()?;              // oct 32-35
    let ref_group_widths = b.read_u8()?;         // oct 36
    let bits_group_widths = b.read_u8()?;        // oct 37
    let ref_group_lengths = b.read_u32be()?;     // oct 38-41
    let length_increment = b.read_u8()?;         // oct 42
    let true_last_group_length = b.read_u32be()?; // oct 43-46
    let bits_scaled_group_lengths = b.read_u8()?; // oct 47
    let order_spatial_diff = b.read_u8()?;       // oct 48
    let extra_octet_count = b.read_u8()?;        // oct 49

    Ok((packing, ComplexPackingExtra {
        n_groups,
        ref_group_widths,
        bits_group_widths,
        ref_group_lengths,
        length_increment,
        true_last_group_length,
        bits_scaled_group_lengths,
        order_spatial_diff,
        extra_octet_count,
    }))
}

// ── Section 6: Bit Map ────────────────────────────────────────────────────────

/// Parse Section 6 (Bit Map).
///
/// Returns `(has_bitmap, bitmap_bytes)`:
/// - `has_bitmap=false, bitmap_bytes=None` when indicator == 255 (all points present).
/// - `has_bitmap=true, bitmap_bytes=Some(bytes)` when indicator == 0: one bit per grid
///   point, MSB-first packed, length = ceil(n_grid_points / 8).
fn parse_section6(body: &[u8]) -> Result<(bool, Option<Vec<u8>>)> {
    let mut b = Buf::new(body);
    let indicator = b.read_u8()?; // oct 6
    match indicator {
        255 => Ok((false, None)),
        0 => {
            // Remaining bytes are the bitmap (1 bit per grid point, MSB first).
            let bitmap = body[1..].to_vec();
            Ok((true, Some(bitmap)))
        }
        _ => Err(Error::NotImplemented),
    }
}

// ── Bitmap expansion ─────────────────────────────────────────────────────────

/// Expand a vector of packed (present-only) values into a full `n_grid`-length
/// masked grid using the bitmap.
///
/// The bitmap is a packed MSB-first bit array (one bit per grid point). Where
/// the bit is 1, the next value from `packed` is used; where 0, the grid point
/// is absent (value = 0.0, present = false). Returns `GridValues::Masked` if
/// a bitmap is provided and `n_packed < n_grid`, otherwise returns the input
/// as a `Dense` grid.
fn expand_bitmap(
    packed: GridValues,
    bitmap: &Option<Vec<u8>>,
    n_grid: usize,
) -> GridValues {
    let bm = match bitmap {
        Some(bm) => bm,
        None => return packed, // no bitmap — pass through
    };

    let packed_vals = match packed {
        GridValues::Dense(v) => v,
        other => return other, // already masked somehow — pass through
    };

    if packed_vals.len() == n_grid {
        // The packed count already matches the grid — no expansion needed.
        return GridValues::Dense(packed_vals);
    }

    let mut values = vec![0.0f64; n_grid];
    let mut present = vec![false; n_grid];
    let mut packed_idx = 0usize;

    for grid_idx in 0..n_grid {
        let byte = grid_idx / 8;
        let bit = 7 - (grid_idx % 8); // MSB first
        let is_present = byte < bm.len() && (bm[byte] >> bit) & 1 == 1;
        if is_present {
            if packed_idx < packed_vals.len() {
                values[grid_idx] = packed_vals[packed_idx];
                packed_idx += 1;
            }
            present[grid_idx] = true;
        }
    }

    GridValues::Masked { values, present }
}

// ── Section 7: Data ───────────────────────────────────────────────────────────

fn decode_section7(
    body: &[u8],
    packing: &PackingInfo,
    drt: u16,
    complex_extra: Option<&ComplexPackingExtra>,
    n_points: usize,
) -> Result<GridValues> {
    if let Some(extra) = complex_extra {
        return decode_drt3(body, packing, extra, n_points);
    }

    if drt == 40 {
        #[cfg(feature = "jpeg2000")]
        return decode_drt40(body, packing, n_points);
        #[cfg(not(feature = "jpeg2000"))]
        return Err(Error::NotImplemented);
    }

    if drt == 41 {
        return decode_drt41(body, packing, n_points);
    }

    // DRT=0: simple packing.
    let r = packing.reference_value as f64;
    let e = packing.binary_scale_factor as i32;
    let d = packing.decimal_scale_factor as i32;
    let two_e = 2f64.powi(e);
    let ten_d = 10f64.powi(d);

    match packing.bits_per_value {
        0 => {
            // Constant field: X=0 for all points, so Y = (R + 0) / 10^D = R / 10^D.
            let v = r / ten_d;
            Ok(GridValues::Dense(vec![v; n_points]))
        }
        n_bits => {
            let packed = unpack_n_bits(body, n_points, n_bits as usize);
            // WMO spec: Y × 10^D = R + X × 2^E  →  Y = (R + X × 2^E) / 10^D
            let values: Vec<f64> = packed.iter().map(|&x| (r + x as f64 * two_e) / ten_d).collect();
            Ok(GridValues::Dense(values))
        }
    }
}

/// Decode Template 5.40: JPEG 2000 data compression.
///
/// Section 7 contains a JPEG 2000 codestream (J2K) or JP2 container. The first
/// component carries packed integers X; decoded values follow: Y = (R + X × 2^E) / 10^D.
#[cfg(feature = "jpeg2000")]
fn decode_drt40(body: &[u8], packing: &PackingInfo, n_points: usize) -> Result<GridValues> {
    let img = jpeg2k::Image::from_bytes(body)
        .map_err(|_| Error::InvalidData("JPEG2000 decode failed"))?;
    let comp = img.components().first()
        .ok_or(Error::InvalidData("J2K: no image components"))?;

    let r = packing.reference_value as f64;
    let e = packing.binary_scale_factor as i32;
    let d = packing.decimal_scale_factor as i32;
    let two_e = 2f64.powi(e);
    let ten_d = 10f64.powi(d);

    let data = comp.data(); // &[i32] — raw packed integer X values
    let values: Vec<f64> = data.iter()
        .take(n_points)
        .map(|&x| (r + x as f64 * two_e) / ten_d)
        .collect();

    if values.len() < n_points {
        return Err(Error::InvalidData("J2K: fewer pixels than grid points"));
    }

    Ok(GridValues::Dense(values))
}

/// Decode Template 5.41: PNG data compression.
///
/// Section 7 contains a raw PNG image. Pixel values are packed integers X;
/// decoded grid values follow the standard formula: Y = (R + X × 2^E) / 10^D.
/// Only 8-bit and 16-bit grayscale PNGs are supported (the common GRIB2 case).
fn decode_drt41(body: &[u8], packing: &PackingInfo, n_points: usize) -> Result<GridValues> {
    let cursor = std::io::Cursor::new(body);
    let decoder = png::Decoder::new(cursor);
    let mut reader = decoder.read_info().map_err(|_| Error::InvalidData("PNG header decode failed"))?;

    let buf_size = reader.output_buffer_size().ok_or(Error::InvalidData("PNG: buffer size unknown"))?;
    let mut img_data = vec![0u8; buf_size];
    let info = reader.next_frame(&mut img_data).map_err(|_| Error::InvalidData("PNG frame decode failed"))?;

    let r = packing.reference_value as f64;
    let e = packing.binary_scale_factor as i32;
    let d = packing.decimal_scale_factor as i32;
    let two_e = 2f64.powi(e);
    let ten_d = 10f64.powi(d);

    let values: Vec<f64> = match (info.bit_depth, info.color_type) {
        (png::BitDepth::Eight, png::ColorType::Grayscale) => {
            img_data[..n_points]
                .iter()
                .map(|&x| (r + x as f64 * two_e) / ten_d)
                .collect()
        }
        (png::BitDepth::Sixteen, png::ColorType::Grayscale) => {
            img_data[..n_points * 2]
                .chunks_exact(2)
                .map(|b| u16::from_be_bytes([b[0], b[1]]) as f64)
                .map(|x| (r + x * two_e) / ten_d)
                .collect()
        }
        _ => return Err(Error::NotImplemented),
    };

    Ok(GridValues::Dense(values))
}

/// Decode Template 5.3: complex packing with spatial differencing.
///
/// The Section 7 body contains:
///   1. `(order+1) × extra_octet_count` bytes: seed values (ival1, [ival2,] minsd) in
///      GRIB2 sign-magnitude big-endian encoding.
///   2. `n_groups × bits_per_value` bits (byte-aligned sub-array): group references.
///   3. `n_groups × bits_group_widths` bits (byte-aligned): group width offsets.
///   4. `n_groups × bits_scaled_group_lengths` bits (byte-aligned): scaled group lengths
///      (last group uses `true_last_group_length` from Section 5 instead).
///   5. Data values packed within groups (per-group bit width).
///
/// All sub-arrays are byte-aligned (padded to the next byte boundary between arrays).
fn decode_drt3(
    body: &[u8],
    packing: &PackingInfo,
    extra: &ComplexPackingExtra,
    n_points: usize,
) -> Result<GridValues> {
    let order = extra.order_spatial_diff as usize;
    let eo = extra.extra_octet_count as usize;
    let n_groups = extra.n_groups as usize;
    let total_seed_bytes = (order + 1) * eo;

    if body.len() < total_seed_bytes {
        return Err(Error::TooShort { needed: total_seed_bytes, got: body.len() });
    }

    // 1. Read seed values from extra octets (sign-magnitude big-endian).
    //    For DRT=2 (order=0, eo=0) there are no extra octets — skip reads entirely.
    let ival1 = if eo > 0 { read_sign_magnitude_be(&body[..eo]) } else { 0i64 };
    let ival2 = if order >= 2 && eo > 0 { read_sign_magnitude_be(&body[eo..2 * eo]) } else { 0i64 };
    let minsd = if eo > 0 { read_sign_magnitude_be(&body[order * eo..total_seed_bytes]) } else { 0i64 };

    let mut byte_pos = total_seed_bytes;

    // 2. Group references: n_groups × bits_per_value bits, byte-aligned.
    let nbits = packing.bits_per_value as usize;
    let group_refs = unpack_n_bits(&body[byte_pos..], n_groups, nbits);
    byte_pos += (n_groups * nbits).div_ceil(8);

    // 3. Group widths: n_groups × bits_group_widths bits, byte-aligned.
    let bw = extra.bits_group_widths as usize;
    let raw_widths = unpack_n_bits(&body[byte_pos..], n_groups, bw);
    let group_widths: Vec<usize> = raw_widths.iter()
        .map(|&w| extra.ref_group_widths as usize + w as usize)
        .collect();
    byte_pos += (n_groups * bw).div_ceil(8);

    // 4. Group lengths: n_groups × bits_scaled_group_lengths bits, byte-aligned.
    //    The last group always uses `true_last_group_length` from Section 5.
    let bl = extra.bits_scaled_group_lengths as usize;
    let raw_lengths = unpack_n_bits(&body[byte_pos..], n_groups, bl);
    let group_lengths: Vec<usize> = raw_lengths.iter().enumerate().map(|(g, &l)| {
        if g == n_groups - 1 {
            extra.true_last_group_length as usize
        } else {
            extra.ref_group_lengths as usize + l as usize * extra.length_increment as usize
        }
    }).collect();
    byte_pos += (n_groups * bl).div_ceil(8);

    // 5. Packed values within groups (variable bit width per group).
    let mut packed = Vec::with_capacity(n_points);
    let mut bit_offset = byte_pos * 8;
    for g in 0..n_groups {
        let w = group_widths[g];
        let l = group_lengths[g];
        let gref = group_refs[g] as i64;
        if w == 0 {
            // Zero-width group: all values equal the group reference.
            // Extending without any bit reads — common in flat regions.
            for _ in 0..l { packed.push(gref); }
            continue;
        }
        // Use the generic windowed extractor (verified correct, no specialized bugs).
        let start_bit = bit_offset;
        extract_group_windowed(body, start_bit, w, l, gref, &mut packed);
        bit_offset = start_bit + w * l;
    }

    if packed.len() != n_points {
        return Err(Error::TooShort { needed: n_points, got: packed.len() });
    }

    // 6+7. Reconstruct integers via spatial differencing and apply the packing
    //    formula in a single combined pass (Attempt 5 in parse-speed-log.md).
    //
    //    Previously: two separate O(n) passes — in-place spatial diff over `packed`
    //    (writing i64), then a scaling pass into `values` (reading i64, writing f64).
    //    For a 65 K-point field each pass reads/writes ~520 KB; with two passes the
    //    vector is read twice from L3/RAM after it exceeds L2.
    //
    //    Now: one combined pass reads each `packed[i]` exactly once, applies the
    //    running-sum state inline, and immediately writes the scaled f64 to `values`.
    //    The `packed` Vec is never mutated after construction; `values` is filled in
    //    a single sequential write — friendlier for the prefetcher.
    //
    //    Seed values (ival1, ival2) are emitted directly from the seed variables
    //    rather than by overwriting packed[0..order], so no in-place writes happen.
    let r = packing.reference_value as f64;
    let two_e = 2f64.powi(packing.binary_scale_factor as i32);
    let ten_d = 10f64.powi(packing.decimal_scale_factor as i32);
    let mut values = Vec::with_capacity(n_points);

    if order == 0 {
        // DRT=2: no spatial differencing — scale directly.
        for &x in &packed {
            values.push((r + x as f64 * two_e) / ten_d);
        }
    } else if order == 1 {
        // Position 0 is the seed ival1; packed[1..] hold first-order differences
        // (with minsd subtracted by the encoder to keep them non-negative).
        if !packed.is_empty() {
            values.push((r + ival1 as f64 * two_e) / ten_d);
        }
        let mut prev = ival1;
        for i in 1..packed.len() {
            prev += packed[i] + minsd;
            values.push((r + prev as f64 * two_e) / ten_d);
        }
    } else if order == 2 {
        // Positions 0 and 1 are seeds ival1/ival2; packed[2..] hold second-order
        // differences. The first-order difference at position 1 is ival2 − ival1.
        if !packed.is_empty() {
            values.push((r + ival1 as f64 * two_e) / ten_d);
        }
        if packed.len() >= 2 {
            values.push((r + ival2 as f64 * two_e) / ten_d);
        }
        let mut delta = ival2 - ival1; // first-order diff at position 1
        let mut prev = ival2;
        for i in 2..packed.len() {
            let second_diff = packed[i] + minsd;
            delta += second_diff;
            prev += delta;
            values.push((r + prev as f64 * two_e) / ten_d);
        }
    }

    Ok(GridValues::Dense(values))
}

/// Read a GRIB2 sign-magnitude big-endian integer from `bytes`.
///
/// The MSB of the first byte is the sign (1 = negative); the remaining bits
/// form the unsigned magnitude.
fn read_sign_magnitude_be(bytes: &[u8]) -> i64 {
    let n = bytes.len();
    debug_assert!(n > 0 && n <= 7, "sign-magnitude: unsupported width {n}");
    let mut raw = 0u64;
    for &b in bytes {
        raw = (raw << 8) | b as u64;
    }
    let sign_bit = 1u64 << (n * 8 - 1);
    let magnitude = (raw & (sign_bit - 1)) as i64;
    if raw & sign_bit != 0 { -magnitude } else { magnitude }
}

/// Extract `count` values each `w` bits wide from `data` starting at `start_bit`,
/// pushing `gref + raw_value` onto `out` for each.
///
/// Uses a left-aligned u64 sliding window (MSB-first, matching GRIB2 bit order)
/// instead of per-element [`read_bits_at`] calls.  The key savings vs. the
/// per-element path:
///
/// * No `div_ceil` per element — `bytes_needed` is computed once on pre-fill.
/// * No inner byte-assembly loop per element — bytes are amortised over a
///   window: one byte loaded per 8 bits consumed on average.
/// * Single `shift + mask` per value instead of 3–9 byte-load + shift operations.
///
/// The left-aligned invariant: valid bits always occupy the MSB positions of
/// `buf`, so the next `w`-bit value is always extracted as `(buf >> (64 - w)) & mask`.
///
/// # Panics (debug only)
/// `w` must satisfy `0 < w ≤ 32`.  The caller is responsible for the w=0 fast
/// path (zero-width groups).
#[inline]
fn extract_group_windowed(
    data: &[u8],
    start_bit: usize,
    w: usize,
    count: usize,
    gref: i64,
    out: &mut Vec<i64>,
) {
    debug_assert!(w > 0 && w <= 32, "w={w} out of range (caller must handle w=0)");
    let mask: u64 = (1u64 << w) - 1;

    // `buf` holds up to 64 bits of stream data, left-aligned (MSB = next bit).
    // `buf_bits` tracks how many valid bits sit at the top of `buf`.
    let mut buf: u64 = 0;
    let mut buf_bits: usize = 0;
    let mut byte_pos = start_bit / 8;
    let skip = start_bit % 8; // alignment bits to discard before the first value

    // Inline loader: place data[byte_pos] at the next available MSB position.
    // Precondition: buf_bits ≤ 56 (so buf_bits + 8 ≤ 64, no overflow).
    // buf_bits stays ≤ 40 in the inner loop (analysis below), so this always holds.
    macro_rules! load_byte {
        () => {
            if byte_pos < data.len() {
                buf |= (data[byte_pos] as u64) << (56 - buf_bits);
                buf_bits += 8;
            }
            // Always advance byte_pos (prevents infinite loop on malformed data).
            byte_pos += 1;
        };
    }

    // Pre-fill: load enough bytes so buf holds at least (skip + w) valid bits.
    // Worst case: skip=7, w=32 → 39 bits → 5 bytes → buf_bits ≤ 40 after pre-fill.
    let init_bytes = (skip + w + 7) / 8;
    for _ in 0..init_bytes {
        load_byte!();
    }

    // Discard the alignment prefix (bits before start_bit within the first byte).
    if skip > 0 {
        buf <<= skip;
        buf_bits -= skip;
    }

    // Main extraction loop.
    // After `buf <<= w`, buf_bits decreases by w.  After at most ceil(w/8)
    // refills (each adding 8 bits), buf_bits ≤ w + 8*ceil(w/8) ≤ 40 for w ≤ 32.
    for _ in 0..count {
        // Refill if needed (at most ceil(w/8) ≤ 4 iterations for w ≤ 32).
        while buf_bits < w {
            load_byte!();
        }

        // Extract the top w bits and advance the window.
        let v = (buf >> (64 - w)) & mask;
        buf <<= w;
        buf_bits -= w;

        out.push(gref + v as i64);
    }
}

/// Read `n_bits` bits from `data` starting at `bit_offset` (MSB-first).
fn read_bits_at(data: &[u8], bit_offset: usize, n_bits: usize) -> u64 {
    if n_bits == 0 {
        return 0;
    }
    let byte_start = bit_offset / 8;
    let bit_start = bit_offset % 8;
    let bits_needed = bit_start + n_bits;
    let bytes_needed = bits_needed.div_ceil(8);
    let mut raw: u64 = 0;
    for i in 0..bytes_needed {
        let byte = if byte_start + i < data.len() { data[byte_start + i] } else { 0 };
        raw = (raw << 8) | byte as u64;
    }
    let total_bits = bytes_needed * 8;
    let shift = total_bits - bit_start - n_bits;
    let mask = (1u64 << n_bits) - 1;
    (raw >> shift) & mask
}

/// Extract `count` unsigned integers each `n_bits` wide from `data`.
/// Bits are packed MSB-first: the first value occupies bits [0..n_bits-1]
/// of the bitstream (bit 0 = MSB of data[0]).
fn unpack_n_bits(data: &[u8], count: usize, n_bits: usize) -> Vec<u64> {
    if n_bits == 0 {
        return vec![0u64; count];
    }
    // Fast paths for byte-aligned widths — avoids per-element shift/mask
    // overhead and exposes the inner loops to compiler auto-vectorization.
    match n_bits {
        8 => {
            let available = count.min(data.len());
            let mut v: Vec<u64> = data[..available].iter().map(|&b| b as u64).collect();
            v.resize(count, 0);
            return v;
        }
        16 => {
            let available = count.min(data.len() / 2);
            let mut v: Vec<u64> = data.chunks_exact(2)
                .take(available)
                .map(|c| u16::from_be_bytes([c[0], c[1]]) as u64)
                .collect();
            v.resize(count, 0);
            return v;
        }
        32 => {
            let available = count.min(data.len() / 4);
            let mut v: Vec<u64> = data.chunks_exact(4)
                .take(available)
                .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]) as u64)
                .collect();
            v.resize(count, 0);
            return v;
        }
        _ => {}
    }

    let mut values = Vec::with_capacity(count);
    let mut bit_offset = 0usize;

    for _ in 0..count {
        let byte_start = bit_offset / 8;
        let bit_start = bit_offset % 8;
        let bits_needed = bit_start + n_bits;
        let bytes_needed = bits_needed.div_ceil(8);

        let mut raw: u64 = 0;
        for i in 0..bytes_needed {
            let byte = if byte_start + i < data.len() { data[byte_start + i] } else { 0 };
            raw = (raw << 8) | byte as u64;
        }

        // raw has bytes_needed * 8 bits, value lives at the top n_bits
        // after shifting past the leading bit_start bits.
        let total_bits = bytes_needed * 8;
        let shift = total_bits - bit_start - n_bits;
        let mask = (1u64 << n_bits) - 1;
        values.push((raw >> shift) & mask);

        bit_offset += n_bits;
    }

    values
}

// ── Lazy / partial decode (DRT=0 only) ───────────────────────────────────────

/// Decode a single grid point from a DRT=0 (simple packing) Section 7 body.
///
/// Returns `None` when `idx` is out of range for the given body length and
/// packing, or when `bits_per_value` is non-zero but the bit span for `idx`
/// exceeds the body.  Returns a value for all indices when `bits_per_value == 0`
/// (constant field).
pub fn decode_point_drt0(body: &[u8], packing: &PackingInfo, idx: usize) -> Option<f64> {
    let r = packing.reference_value as f64;
    let two_e = 2f64.powi(packing.binary_scale_factor as i32);
    let ten_d = 10f64.powi(packing.decimal_scale_factor as i32);

    if packing.bits_per_value == 0 {
        return Some(r / ten_d);
    }

    let n = packing.bits_per_value as usize;
    let bit_offset = idx * n;
    let byte_end = (bit_offset + n).div_ceil(8);
    if byte_end > body.len() {
        return None;
    }
    let x = read_bits_at(body, bit_offset, n);
    Some((r + x as f64 * two_e) / ten_d)
}

/// Decode all grid values from a DRT=2 or DRT=3 (complex packing) Section 7 body.
///
/// This is the public entry point for the "decode-once-extract-many" pattern:
/// decode the full grid once and cache the `Vec<f64>`, then do O(1) index
/// lookups for each station.  For DRT=3 (spatial differencing), random access
/// is impossible without a full decode because each value depends on all prior
/// values in the differencing reversal step.
///
/// `body` is the raw Section 7 body (from `LazyField::section7_raw`).
/// `packing` is from `LazyField::packing`.
/// `extra` is from `LazyField::complex_extra` (must be `Some` for DRT=2/3).
/// `n_points` is `LazyField::grid.num_data_points as usize`.
pub fn decode_all_drt3(
    body: &[u8],
    packing: &PackingInfo,
    extra: &ComplexExtra,
    n_points: usize,
) -> Result<Vec<f64>> {
    match decode_drt3(body, packing, extra, n_points)? {
        GridValues::Dense(v) => Ok(v),
        GridValues::Masked { values, .. } => Ok(values),
    }
}

/// Extract a single grid point from a DRT=2 or DRT=3 (complex packing) Section 7 body.
///
/// Decodes the full grid and returns `values[idx]`.
///
/// **Performance note:** For DRT=3 (spatial differencing), each call decodes
/// the entire grid — random access is impossible because each value depends on
/// all prior values.  When extracting multiple stations from the same field,
/// prefer [`decode_all_drt3`] (decode once, cache, index in O(1)) to avoid
/// paying the full-decode cost N times.
///
/// Returns `None` if `idx >= n_points` or if decoding fails.
///
/// `body` is `LazyField::section7_raw`.
/// `packing` is `LazyField::packing`.
/// `extra` is `LazyField::complex_extra.as_ref().unwrap()` (must be `Some` for DRT=2/3).
/// `n_points` is `LazyField::grid.num_data_points as usize`.
pub fn decode_point_drt3(
    body: &[u8],
    packing: &PackingInfo,
    extra: &ComplexExtra,
    n_points: usize,
    idx: usize,
) -> Option<f64> {
    if idx >= n_points {
        return None;
    }
    let values = decode_all_drt3(body, packing, extra, n_points).ok()?;
    values.get(idx).copied()
}

/// Decode all fields lazily — Section 7 data is stored as raw bytes.
///
/// DRT=0 fields without a bitmap: `section7_raw` is populated; use
/// [`decode_point_drt0`] for random access.
///
/// DRT=2/3 fields without a bitmap: `section7_raw` and `complex_extra` are
/// both populated; use [`decode_all_drt3`] to decode the full grid once.
pub fn decode_bytes_lazy(bytes: &[u8]) -> Result<Vec<LazyField>> {
    let mut fields = Vec::new();
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes.len() - pos < 4 {
            break;
        }
        if &bytes[pos..pos + 4] != b"GRIB" {
            return Err(Error::BadMagic(bytes[pos..pos + 4].try_into().unwrap()));
        }
        let msg_len = decode_lazy_message(&bytes[pos..], &mut fields)?;
        pos += msg_len;
    }
    Ok(fields)
}

// ── Lazy message builder ──────────────────────────────────────────────────────

#[derive(Default)]
struct LazyFieldBuilder {
    center: Option<u16>,
    subcenter: Option<u16>,
    ref_time: Option<ReferenceTime>,
    parameter: Option<ParameterId>,
    forecast: Option<ForecastTime>,
    level: Option<Level>,
    ensemble: Option<Option<Ensemble>>,
    pdt_template: Option<u16>,
    grid: Option<GridDefinition>,
    gdt_template: Option<u16>,
    packing: Option<PackingInfo>,
    drt_template: Option<u16>,
    complex_extra: Option<ComplexPackingExtra>,
    has_bitmap: Option<bool>,
    section7_raw: Option<Vec<u8>>,
}

impl LazyFieldBuilder {
    fn build(self) -> Option<LazyField> {
        let center = self.center?;
        let subcenter = self.subcenter?;
        let ref_time = self.ref_time?;
        let parameter = self.parameter?;
        let mut forecast = self.forecast?;
        forecast.reference_time = ref_time;
        let level = self.level?;
        let ensemble = self.ensemble.unwrap_or(None);
        let grid = self.grid?;
        let gdt_template = self.gdt_template?;
        let pdt_template = self.pdt_template?;
        let packing = self.packing?;
        let drt_template = self.drt_template?;
        let has_bitmap = self.has_bitmap.unwrap_or(false);
        let section7_raw = self.section7_raw.unwrap_or_default();
        let complex_extra = self.complex_extra;
        Some(LazyField {
            center, subcenter, parameter, forecast, level, ensemble,
            grid, packing, gdt_template, pdt_template, drt_template,
            has_bitmap, section7_raw, complex_extra,
        })
    }
}

fn decode_lazy_message(msg: &[u8], out: &mut Vec<LazyField>) -> Result<usize> {
    let mut buf = Buf::new(msg);
    let magic = [buf.read_u8()?, buf.read_u8()?, buf.read_u8()?, buf.read_u8()?];
    if &magic != b"GRIB" {
        return Err(Error::BadMagic(magic));
    }
    buf.skip(2)?;
    let discipline = buf.read_u8()?;
    let edition = buf.read_u8()?;
    if edition != 2 {
        return Err(Error::UnknownEdition(edition));
    }
    let total_len = buf.read_u64be()? as usize;
    if msg.len() < total_len {
        return Err(Error::TooShort { needed: total_len, got: msg.len() });
    }

    let mut builder = LazyFieldBuilder::default();
    let body_end = total_len - 4;

    while buf.pos < body_end {
        let sec_start = buf.pos;
        let sec_len = buf.read_u32be()? as usize;
        let sec_num = buf.read_u8()?;
        if sec_len < 5 {
            return Err(Error::TooShort { needed: 5, got: sec_len });
        }
        let body_start = buf.pos;
        let body_len = sec_len - 5;
        let sec_body = &msg[body_start..body_start + body_len];

        match sec_num {
            1 => {
                let (center, subcenter, ref_time) = parse_section1(sec_body)?;
                builder.center = Some(center);
                builder.subcenter = Some(subcenter);
                builder.ref_time = Some(ref_time);
            }
            2 => {}
            3 => {
                let (gdt, grid) = parse_section3(sec_body)?;
                builder.gdt_template = Some(gdt);
                builder.grid = Some(grid);
            }
            4 => {
                let (pdt, param, fore, lvl, ens) = parse_section4(sec_body, discipline)?;
                builder.pdt_template = Some(pdt);
                builder.parameter = Some(param);
                builder.forecast = Some(fore);
                builder.level = Some(lvl);
                builder.ensemble = Some(ens);
            }
            5 => {
                // Store the full packing info including complex extras for DRT=2/3.
                let (drt, packing, extra, _n_packed) = parse_section5(sec_body)?;
                builder.drt_template = Some(drt);
                builder.packing = Some(packing);
                builder.complex_extra = extra;
            }
            6 => {
                let (has_bitmap, _bitmap) = parse_section6(sec_body)?;
                builder.has_bitmap = Some(has_bitmap);
            }
            7 => {
                // Store raw bytes for DRT=0, DRT=2, and DRT=3 without a bitmap.
                // DRT=0: single-point random access via decode_point_drt0.
                // DRT=2/3: full-grid decode via decode_all_drt3 (spatial differencing
                //          prevents true random access, so the full grid must be decoded).
                let drt = builder.drt_template.unwrap_or(u16::MAX);
                let no_bitmap = !builder.has_bitmap.unwrap_or(false);
                let raw = if (drt == 0 || drt == 2 || drt == 3) && no_bitmap {
                    sec_body.to_vec()
                } else {
                    Vec::new()
                };
                builder.section7_raw = Some(raw);

                let next_builder = LazyFieldBuilder {
                    center: builder.center,
                    subcenter: builder.subcenter,
                    ref_time: builder.ref_time,
                    ..Default::default()
                };
                if let Some(field) = builder.build() {
                    out.push(field);
                }
                builder = next_builder;
            }
            _ => {}
        }
        buf.pos = sec_start + sec_len;
    }

    let end = &msg[body_end..body_end + 4];
    if end != b"7777" {
        return Err(Error::TooShort { needed: 4, got: 0 });
    }
    Ok(total_len)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_8bit_values() {
        let data: Vec<u8> = (0u8..25).collect();
        let values = unpack_n_bits(&data, 25, 8);
        assert_eq!(values.len(), 25);
        for (i, &v) in values.iter().enumerate() {
            assert_eq!(v, i as u64, "mismatch at index {}", i);
        }
    }

    #[test]
    fn unpack_4bit_values() {
        // 4-bit values: pack two per byte.
        // data[0] = 0x01 → values 0, 1
        // data[1] = 0x23 → values 2, 3
        let data = [0x01u8, 0x23];
        let values = unpack_n_bits(&data, 4, 4);
        assert_eq!(values, vec![0, 1, 2, 3]);
    }

    #[test]
    fn unpack_12bit_values() {
        // [0, 1, 2] packed as 12-bit MSB-first values (36 bits → 5 bytes):
        // val0=0x000: 0000 0000 0000
        // val1=0x001: 0000 0000 0001
        // val2=0x002: 0000 0000 0010
        // byte0=0x00 byte1=0x00 byte2=0x01 byte3=0x00 byte4=0x20(+4 padding)
        let data = [0x00u8, 0x00, 0x01, 0x00, 0x20];
        let values = unpack_n_bits(&data, 3, 12);
        assert_eq!(values, vec![0, 1, 2]);
    }

    #[test]
    fn constant_field_zero_bits() {
        let values = unpack_n_bits(&[], 5, 0);
        assert_eq!(values, vec![0; 5]);
    }

    #[test]
    fn decode_section7_simple_packing() {
        let packing = PackingInfo {
            reference_value: 270.0,
            binary_scale_factor: 0,
            decimal_scale_factor: 0,
            bits_per_value: 8,
            original_field_type: 0,
        };
        let data: Vec<u8> = (0u8..25).collect();
        // DRT=0: no complex extra
        let values = decode_section7(&data, &packing, 0, None, 25).unwrap();
        match values {
            GridValues::Dense(v) => {
                assert_eq!(v.len(), 25);
                for (i, &val) in v.iter().enumerate() {
                    assert!((val - (270.0 + i as f64)).abs() < 1e-9, "index {i}: {val}");
                }
            }
            _ => panic!("expected Dense"),
        }
    }

    #[test]
    fn decode_point_drt0_matches_full_decode() {
        // 8-bit packing: value[i] = 270 + i
        let packing = PackingInfo {
            reference_value: 270.0,
            binary_scale_factor: 0,
            decimal_scale_factor: 0,
            bits_per_value: 8,
            original_field_type: 0,
        };
        let data: Vec<u8> = (0u8..25).collect();
        let full = decode_section7(&data, &packing, 0, None, 25).unwrap();
        let GridValues::Dense(full_vals) = full else { panic!() };

        for (idx, &full_val) in full_vals.iter().enumerate().take(25) {
            let lazy_val = decode_point_drt0(&data, &packing, idx).expect("idx in range");
            assert!(
                (lazy_val - full_val).abs() < 1e-9,
                "idx={idx}: lazy={lazy_val} full={full_val}",
            );
        }
        assert!(decode_point_drt0(&data, &packing, 25).is_none()); // out of range
    }

    #[test]
    fn decode_point_drt0_constant_field() {
        // 0-bit packing: all values equal R / 10^D
        let packing = PackingInfo {
            reference_value: 300.0,
            binary_scale_factor: 0,
            decimal_scale_factor: 0,
            bits_per_value: 0,
            original_field_type: 0,
        };
        // Any idx returns 300.0 (constant field, no body bytes needed)
        assert_eq!(decode_point_drt0(&[], &packing, 0), Some(300.0));
        assert_eq!(decode_point_drt0(&[], &packing, 999), Some(300.0));
    }

    #[test]
    fn decode_point_drt3_matches_full_decode() {
        // Verify decode_point_drt3 returns the same value as decode_all_drt3[idx]
        // for the DRT=3 GFS fixture (gfs_tmp2m_1deg_anl).
        let corpus_root = {
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            std::path::Path::new(manifest_dir)
                .join("../..")
                .join("tests/corpus")
        };
        let fixture = corpus_root.join("small/gfs_tmp2m_1deg_anl.grib2");
        let bytes = match std::fs::read(&fixture) {
            Ok(b) => b,
            Err(_) => return, // skip if fixture not present
        };
        let lazy_fields = decode_bytes_lazy(&bytes).expect("lazy decode");
        for lf in &lazy_fields {
            if lf.drt_template != 3 || lf.section7_raw.is_empty() {
                continue;
            }
            let extra = lf.complex_extra.as_ref().expect("DRT=3 must have complex_extra");
            let n_pts = lf.grid.num_data_points as usize;
            let all_vals = decode_all_drt3(&lf.section7_raw, &lf.packing, extra, n_pts)
                .expect("decode_all_drt3 ok");
            // Verify first 5, middle, last index.
            let indices: Vec<usize> = (0..5.min(n_pts))
                .chain([n_pts / 2, n_pts.saturating_sub(1)])
                .collect();
            for idx in indices {
                let point_val = decode_point_drt3(
                    &lf.section7_raw, &lf.packing, extra, n_pts, idx,
                ).expect("in-range idx");
                assert!(
                    (point_val - all_vals[idx]).abs() < 1e-9,
                    "idx={idx}: point={point_val} all={}", all_vals[idx],
                );
            }
            // Out-of-range must return None.
            assert!(
                decode_point_drt3(&lf.section7_raw, &lf.packing, extra, n_pts, n_pts).is_none(),
                "out-of-range idx must return None",
            );
        }
    }

    /// Verify extract_group_windowed matches read_bits_at across many (skip, w) combos.
    #[test]
    fn extract_group_windowed_matches_read_bits_at() {
        // Pseudo-random but deterministic 64-byte data buffer.
        let data: Vec<u8> = (0u8..=255).cycle().take(64).collect();
        let gref = 7i64;
        let count = 10;

        for skip in 0..8usize {
            for w in 1usize..=20 {
                // Reference: per-element read_bits_at
                let mut expected = Vec::with_capacity(count);
                for k in 0..count {
                    let v = read_bits_at(&data, skip + k * w, w) as i64;
                    expected.push(gref + v);
                }

                // Windowed extractor under test
                let mut actual = Vec::with_capacity(count);
                extract_group_windowed(&data, skip, w, count, gref, &mut actual);

                assert_eq!(
                    expected, actual,
                    "skip={skip} w={w}: windowed result differs from read_bits_at",
                );
            }
        }
    }

    #[test]
    fn decode_bytes_lazy_matches_decode_bytes() {
        // Verify lazy parse of the 5×5 synthetic fixture against full decode.
        // We test the structure (grid, packing) and that decode_point_drt0
        // produces values matching the full decode at every index.
        let corpus_root = {
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            std::path::Path::new(manifest_dir)
                .join("../..")
                .join("tests/corpus")
        };
        let fixture = corpus_root.join("small/gfs_anl_t2m_5x5.grib2");
        let bytes = match std::fs::read(&fixture) {
            Ok(b) => b,
            Err(_) => return, // skip if fixture not present (CI may not have corpus)
        };

        let full_fields = decode_bytes(&bytes).expect("full decode");
        let lazy_fields = decode_bytes_lazy(&bytes).expect("lazy decode");

        assert_eq!(full_fields.len(), lazy_fields.len());
        for (ff, lf) in full_fields.iter().zip(lazy_fields.iter()) {
            assert_eq!(ff.grid, lf.grid);
            assert_eq!(ff.packing, lf.packing);
            assert_eq!(ff.drt_template, lf.drt_template);
            assert!(!lf.has_bitmap);
            assert!(!lf.section7_raw.is_empty(), "DRT=0 should have raw bytes");

            let GridValues::Dense(ref full_vals) = ff.values else { panic!("expected Dense") };
            for (idx, &full_val) in full_vals.iter().enumerate() {
                let lazy_val = decode_point_drt0(&lf.section7_raw, &lf.packing, idx)
                    .expect("idx in range");
                let tol = lf.packing.tolerance().max(1e-12);
                assert!(
                    (lazy_val - full_val).abs() <= tol,
                    "idx={idx}: lazy={lazy_val} full={full_val}",
                );
            }
        }
    }
}
