use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

/// Decode GRIB2 files and print field information
#[derive(Parser)]
#[command(name = "gribtract")]
#[command(about = "A pure-Rust GRIB2 decoder CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Decode all fields and print as JSON
    Decode {
        /// GRIB2 file to decode
        file: PathBuf,
    },
    /// List field inventory (metadata only)
    List {
        /// GRIB2 file to list
        file: PathBuf,
    },
    /// Dump raw hex of the file
    Dump {
        /// GRIB2 file to dump
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("gribtract: error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Decode { file } => cmd_decode(file),
        Commands::List { file } => cmd_list(file),
        Commands::Dump { file } => cmd_dump(file),
    }
}

/// Decode all fields and print as JSON
fn cmd_decode(path: PathBuf) -> Result<()> {
    let bytes = fs::read(&path)?;
    let fields = gribtract::decode(&bytes)?;

    println!("[");
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            println!(",");
        }
        print_field_json(field, true);
    }
    println!();
    println!("]");
    Ok(())
}

/// List field inventory (metadata only)
fn cmd_list(path: PathBuf) -> Result<()> {
    let bytes = fs::read(&path)?;
    let fields = gribtract::decode(&bytes)?;

    println!("{{");
    println!("  \"file\": \"{}\",", path.display());
    println!("  \"field_count\": {},", fields.len());
    println!("  \"fields\": [");
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            println!(",");
        }
        print_field_json(field, false);
    }
    println!();
    println!("  ]");
    println!("}}");
    Ok(())
}

/// Dump raw hex of the file
fn cmd_dump(path: PathBuf) -> Result<()> {
    let bytes = fs::read(&path)?;

    // Print in hexdump format: offset + hex bytes + ascii
    let mut i = 0;
    while i < bytes.len() {
        // Print offset
        print!("{:08x}  ", i);

        // Print hex bytes (16 per row)
        let row_end = (i + 16).min(bytes.len());
        for j in i..row_end {
            print!("{:02x} ", bytes[j]);
            if (j - i) == 7 {
                print!(" ");
            }
        }

        // Padding for incomplete rows
        if row_end - i < 16 {
            for j in (row_end - i)..16 {
                print!("   ");
                if j == 7 {
                    print!(" ");
                }
            }
        }

        // Print ASCII representation
        print!(" |");
        for j in i..row_end {
            let c = bytes[j] as char;
            if c.is_ascii_graphic() || c == ' ' {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!("|");

        i = row_end;
    }

    Ok(())
}

/// Print a field as JSON (with values if include_values is true)
fn print_field_json(field: &gribtract::Field, include_values: bool) {
    // Indentation
    let indent = if include_values { "  " } else {    "    " };

    println!("{{");
    println!("{}\"center\": {},", indent, field.center);
    println!("{}\"subcenter\": {},", indent, field.subcenter);
    println!("{}\"parameter\": {{", indent);
    println!("{}  \"discipline\": {},", indent, field.parameter.discipline);
    println!("{}  \"category\": {},", indent, field.parameter.category);
    println!("{}  \"number\": {}", indent, field.parameter.number);
    println!("{}}},", indent);

    println!("{}\"level\": {{", indent);
    println!("{}  \"type1\": {},", indent, field.level.type1);
    println!("{}  \"scale_factor1\": {},", indent, field.level.scale_factor1);
    println!("{}  \"scaled_value1\": {},", indent, field.level.scaled_value1);
    println!("{}  \"type2\": {},", indent, field.level.type2);
    println!("{}  \"scale_factor2\": {},", indent, field.level.scale_factor2);
    println!("{}  \"scaled_value2\": {},", indent, field.level.scaled_value2);
    println!("{}  \"value1\": {}", indent, field.level.value1());
    println!("{}}},", indent);

    println!("{}\"forecast_time\": {{", indent);
    println!("{}  \"reference_time\": {{", indent);
    println!("{}    \"year\": {},", indent, field.forecast.reference_time.year);
    println!("{}    \"month\": {},", indent, field.forecast.reference_time.month);
    println!("{}    \"day\": {},", indent, field.forecast.reference_time.day);
    println!("{}    \"hour\": {},", indent, field.forecast.reference_time.hour);
    println!("{}    \"minute\": {},", indent, field.forecast.reference_time.minute);
    println!("{}    \"second\": {},", indent, field.forecast.reference_time.second);
    println!("{}    \"significance\": {},", indent, field.forecast.reference_time.significance);
    println!("{}    \"unix_seconds\": {}", indent, field.forecast.reference_time.unix_seconds());
    println!("{}  }},", indent);
    println!("{}  \"time_range_unit\": {},", indent, field.forecast.time_range_unit);
    println!("{}  \"forecast_offset\": {},", indent, field.forecast.forecast_offset);
    println!("{}  \"offset_seconds\": {},", indent, field.forecast.offset_seconds());
    println!("{}  \"valid_unix_seconds\": {}", indent, field.forecast.valid_unix_seconds());
    println!("{}}},", indent);

    if let Some(ensemble) = field.ensemble {
        println!("{}\"ensemble\": {{", indent);
        println!("{}  \"member_type\": {},", indent, ensemble.member_type);
        println!("{}  \"number\": {}", indent, ensemble.number);
        println!("{}}},", indent);
    } else {
        println!("{}\"ensemble\": null,", indent);
    }

    println!("{}\"grid\": {{", indent);
    println!("{}  \"template\": {},", indent, field.grid.template);
    println!("{}  \"num_data_points\": {},", indent, field.grid.num_data_points);
    println!("{}  \"nx\": {},", indent, field.grid.nx);
    println!("{}  \"ny\": {},", indent, field.grid.ny);
    println!("{}  \"lat_first\": {},", indent, field.grid.lat_first);
    println!("{}  \"lon_first\": {},", indent, field.grid.lon_first);
    println!("{}  \"lat_last\": {},", indent, field.grid.lat_last);
    println!("{}  \"lon_last\": {},", indent, field.grid.lon_last);
    println!("{}  \"di\": {},", indent, field.grid.di);
    println!("{}  \"dj\": {},", indent, field.grid.dj);
    println!("{}  \"scanning_mode\": {},", indent, field.grid.scanning_mode);
    println!("{}  \"resolution_flags\": {},", indent, field.grid.resolution_flags);
    println!("{}  \"shape_of_earth\": {},", indent, field.grid.shape_of_earth);
    println!("{}  \"projection\": {:?},", indent, field.grid.projection);
    println!("{}}},", indent);

    println!("{}\"templates\": {{", indent);
    println!("{}  \"gdt\": {},", indent, field.gdt_template);
    println!("{}  \"pdt\": {},", indent, field.pdt_template);
    println!("{}  \"drt\": {}", indent, field.drt_template);
    println!("{}}},", indent);

    println!("{}\"packing\": {{", indent);
    println!("{}  \"reference_value\": {},", indent, field.packing.reference_value);
    println!("{}  \"binary_scale_factor\": {},", indent, field.packing.binary_scale_factor);
    println!("{}  \"decimal_scale_factor\": {},", indent, field.packing.decimal_scale_factor);
    println!("{}  \"bits_per_value\": {},", indent, field.packing.bits_per_value);
    println!("{}  \"original_field_type\": {},", indent, field.packing.original_field_type);
    println!("{}  \"quantization_step\": {},", indent, field.packing.quantization_step());
    println!("{}  \"tolerance\": {}", indent, field.packing.tolerance());
    println!("{}}},", indent);

    if include_values {
        println!("{}\"values\": {{", indent);
        match &field.values {
            gribtract::GridValues::Dense(v) => {
                println!("{}  \"type\": \"dense\",", indent);
                println!("{}  \"len\": {},", indent, v.len());
                println!("{}  \"data\": [", indent);
                // Print first 10 values as a preview
                let preview_count = 10.min(v.len());
                for (j, val) in v.iter().take(preview_count).enumerate() {
                    if j > 0 {
                        print!(", ");
                    }
                    print!("{}", val);
                }
                if v.len() > preview_count {
                    print!(" ... ({} more)", v.len() - preview_count);
                }
                println!();
                println!("{}  ]", indent);
            }
            gribtract::GridValues::Masked { values, present } => {
                println!("{}  \"type\": \"masked\",", indent);
                println!("{}  \"len\": {},", indent, values.len());
                println!("{}  \"present_count\": {}", indent, present.iter().filter(|&&p| p).count());
                println!("{}  \"data\": [", indent);
                // Print first 10 values as a preview
                let preview_count = 10.min(values.len());
                for (j, (val, is_present)) in values.iter().zip(present.iter()).take(preview_count).enumerate() {
                    if j > 0 {
                        print!(", ");
                    }
                    if *is_present {
                        print!("{}", val);
                    } else {
                        print!("null");
                    }
                }
                if values.len() > preview_count {
                    print!(" ... ({} more)", values.len() - preview_count);
                }
                println!();
                println!("{}  ]", indent);
            }
        }
        println!("{} }}", indent);
    }

    print!(" }}");
}
