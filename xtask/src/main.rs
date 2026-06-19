fn main() {
    let task = std::env::args().nth(1).unwrap_or_default();
    match task.as_str() {
        "bench" => eprintln!("xtask bench: not yet implemented"),
        "corpus" => eprintln!("xtask corpus: not yet implemented"),
        other => {
            eprintln!("unknown xtask: {other:?}");
            eprintln!("available: bench, corpus");
            std::process::exit(1);
        }
    }
}
