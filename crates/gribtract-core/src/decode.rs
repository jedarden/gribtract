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
    Ensemble, Field, ForecastTime, GridDefinition, GridValues, Level, PackingInfo,
    ParameterId, ReferenceTime,
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
    // From Section 6
    has_bitmap: Option<bool>,
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
                let (drt, packing) = parse_section5(sec_body)?;
                builder.drt_template = Some(drt);
                builder.packing = Some(packing);
            }
            6 => {
                let has_bitmap = parse_section6(sec_body)?;
                builder.has_bitmap = Some(has_bitmap);
            }
            7 => {
                let n_points = builder.grid.as_ref().map(|g| g.num_data_points as usize).unwrap_or(0);
                let packing = builder.packing.as_ref().ok_or(Error::NotImplemented)?;
                let has_bitmap = builder.has_bitmap.unwrap_or(false);
                let values = decode_section7(sec_body, packing, n_points, has_bitmap)?;
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
        _ => Err(Error::NotImplemented),
    }
}

/// Product Definition Template 4.0: Analysis or forecast at horizontal level.
fn parse_pdt_0(
    b: &mut Buf,
    discipline: u8,
) -> Result<(ParameterId, ForecastTime, Level, Option<Ensemble>)> {
    let category = b.read_u8()?;  // oct 10
    let number = b.read_u8()?;    // oct 11
    b.skip(1)?; // generating process type (oct 12)
    b.skip(1)?; // background generating process id (oct 13)
    b.skip(1)?; // analysis/forecast generating process id (oct 14)
    b.skip(2)?; // hours of cutoff after reference time (oct 15-16)
    b.skip(1)?; // minutes of cutoff (oct 17)
    let time_range_unit = b.read_u8()?;   // oct 18
    let forecast_offset = b.read_u32be()?; // oct 19-22

    let type1 = b.read_u8()?;          // oct 23
    let scale_factor1 = b.read_i8()?;  // oct 24
    let scaled_value1 = b.read_u32be()? as i32; // oct 25-28

    let raw_type2 = b.read_u8()?;           // oct 29
    let raw_scale_factor2 = b.read_i8()?;   // oct 30
    let raw_scaled_value2 = b.read_u32be()? as i32; // oct 31-34

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

    Ok((param, fore, lvl, None))
}

// ── Section 5: Data Representation ───────────────────────────────────────────

fn parse_section5(body: &[u8]) -> Result<(u16, PackingInfo)> {
    let mut b = Buf::new(body);
    let _n_values = b.read_u32be()?; // oct 6-9: number of packed values
    let template = b.read_u16be()?;  // oct 10-11: DRT template number

    match template {
        0 => {
            let packing = parse_drt_0(&mut b)?;
            Ok((0, packing))
        }
        _ => Err(Error::NotImplemented),
    }
}

/// Data Representation Template 5.0: Simple packing.
fn parse_drt_0(b: &mut Buf) -> Result<PackingInfo> {
    let reference_value = b.read_f32be()?;          // oct 12-15: R
    let binary_scale_factor = b.read_scale_factor_i16()?; // oct 16-17: E
    let decimal_scale_factor = b.read_scale_factor_i16()?; // oct 18-19: D
    let bits_per_value = b.read_u8()?;              // oct 20: N
    let original_field_type = b.read_u8()?;         // oct 21

    Ok(PackingInfo {
        reference_value,
        binary_scale_factor,
        decimal_scale_factor,
        bits_per_value,
        original_field_type,
    })
}

// ── Section 6: Bit Map ────────────────────────────────────────────────────────

/// Returns `true` if an actual bitmap follows (indicator == 0),
/// `false` if indicator == 255 (no bitmap, all points present).
fn parse_section6(body: &[u8]) -> Result<bool> {
    let mut b = Buf::new(body);
    let indicator = b.read_u8()?; // oct 6
    match indicator {
        255 => Ok(false),
        0 => Ok(true),
        _ => Err(Error::NotImplemented),
    }
}

// ── Section 7: Data ───────────────────────────────────────────────────────────

fn decode_section7(
    body: &[u8],
    packing: &PackingInfo,
    n_points: usize,
    _has_bitmap: bool,
) -> Result<GridValues> {
    // For DRT 5.0 (simple packing), all data is packed after the section header.
    // Section 7 body (after sec_len + sec_num) is raw packed data starting at oct 6.
    match packing.bits_per_value {
        0 => {
            // Constant field: all values equal to R.
            let v = packing.reference_value as f64;
            Ok(GridValues::Dense(vec![v; n_points]))
        }
        n_bits => {
            let packed = unpack_n_bits(body, n_points, n_bits as usize);
            let r = packing.reference_value as f64;
            let e = packing.binary_scale_factor as i32;
            let d = packing.decimal_scale_factor as i32;
            let scale = 2f64.powi(e) / 10f64.powi(d);
            let values: Vec<f64> = packed.iter().map(|&x| r + x as f64 * scale).collect();
            Ok(GridValues::Dense(values))
        }
    }
}

/// Extract `count` unsigned integers each `n_bits` wide from `data`.
/// Bits are packed MSB-first: the first value occupies bits [0..n_bits-1]
/// of the bitstream (bit 0 = MSB of data[0]).
fn unpack_n_bits(data: &[u8], count: usize, n_bits: usize) -> Vec<u64> {
    if n_bits == 0 {
        return vec![0u64; count];
    }
    let mut values = Vec::with_capacity(count);
    let mut bit_offset = 0usize;

    for _ in 0..count {
        let byte_start = bit_offset / 8;
        let bit_start = bit_offset % 8;
        let bits_needed = bit_start + n_bits;
        let bytes_needed = (bits_needed + 7) / 8;

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
        let values = decode_section7(&data, &packing, 25, false).unwrap();
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
}
