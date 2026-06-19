//! GRIB2 section splitter — validates framing, hands back raw section bytes.

pub struct RawSection<'a> {
    pub number: u8,
    pub bytes: &'a [u8],
}
