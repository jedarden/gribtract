mod bench;
mod bench_station;

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let task = raw_args.first().map(|s| s.as_str()).unwrap_or("");
    let rest = if raw_args.len() > 1 { &raw_args[1..] } else { &[][..] };

    match task {
        "bench" => bench::run(rest),
        "corpus" => eprintln!("xtask corpus: not yet implemented"),
        "" => {
            eprintln!("usage: xtask <bench|corpus> [args...]");
            std::process::exit(1);
        }
        other => {
            eprintln!("unknown xtask: {other:?}");
            eprintln!("available: bench, corpus");
            std::process::exit(1);
        }
    }
}
