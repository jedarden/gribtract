mod bench;
mod bench_station;
mod corpus;
mod probe_providers;
mod serve;

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let task = raw_args.first().map(|s| s.as_str()).unwrap_or("");
    let rest = if raw_args.len() > 1 { &raw_args[1..] } else { &[][..] };

    match task {
        "bench" => bench::run(rest),
        "corpus" => corpus::run(rest),
        "probe-providers" => probe_providers::run(rest),
        "serve" => serve::run(rest),
        "" => {
            eprintln!("usage: xtask <bench|corpus|probe-providers|serve> [args...]");
            eprintln!("  bench              measure decode throughput → bench-results.json + dashboard.html");
            eprintln!("  corpus list        print all fixtures with storage type and local presence");
            eprintln!("  corpus fetch       download remote fixtures from B2 by sha256");
            eprintln!("  probe-providers    probe NOAA provider latency → provider-probe.json");
            eprintln!("  serve [--port N]   start live dashboard HTTP server (default port 7777)");
            std::process::exit(1);
        }
        other => {
            eprintln!("unknown xtask: {other:?}");
            eprintln!("available: bench, corpus, probe-providers, serve");
            std::process::exit(1);
        }
    }
}
