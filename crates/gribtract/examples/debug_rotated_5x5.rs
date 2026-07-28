use std::fs;
use gribtract::decode;

fn main() {
    let bytes = fs::read("tests/corpus/small/rotated_latlon_5x5.grib2")
        .expect("failed to read fixture");

    match decode(&bytes) {
        Ok(fields) => {
            println!("Decoded {} fields:", fields.len());
            for field in &fields {
                println!("{:#?}", field);
            }
        }
        Err(e) => {
            eprintln!("Decode error: {}", e);
        }
    }
}
