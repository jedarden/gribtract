//! `xtask corpus` — manage the GRIB2 fixture corpus.
//!
//! Subcommands:
//!   list   — print all manifest entries with storage type and local presence
//!   fetch  — download remote fixtures from B2 (by sha256 or explicit URL)
//!
//! # Remote-fixture conventions
//!
//! Fixtures with `storage=remote` live in `tests/corpus/large/` (gitignored).
//! The manifest entry must have either:
//!   - a `url` field with the full download URL, or
//!   - no `url` field, in which case `GRIBTRACT_B2_ENDPOINT` and
//!     `GRIBTRACT_B2_BUCKET` env vars are used to build
//!     `<endpoint>/file/<bucket>/<sha256>`.
//!
//! For private buckets set `B2_ACCOUNT_ID` and `B2_APPLICATION_KEY`; for public
//! buckets no auth is required.

use std::io::Write as _;
use std::path::Path;

use sha2::{Digest, Sha256};

use gribtract_testutil::corpus::{corpus_root, list_fixtures, FixtureEntry};

// ── Public entry point ────────────────────────────────────────────────────────

pub fn run(rest: &[String]) {
    let subcommand = rest.first().map(|s| s.as_str()).unwrap_or("");
    match subcommand {
        "list" => cmd_list(),
        "fetch" => cmd_fetch(&rest[1..]),
        "" => {
            eprintln!("usage: xtask corpus <list|fetch> [args...]");
            eprintln!("  list          print all fixtures with storage type and local presence");
            eprintln!("  fetch         download remote fixtures (all, or --fixture <id>)");
            std::process::exit(1);
        }
        other => {
            eprintln!("unknown corpus subcommand: {other:?}");
            eprintln!("available: list, fetch");
            std::process::exit(1);
        }
    }
}

// ── list ──────────────────────────────────────────────────────────────────────

fn cmd_list() {
    let fixtures = match list_fixtures() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("corpus list: {e}");
            std::process::exit(1);
        }
    };
    let root = corpus_root();

    println!(
        "{:<30}  {:<10}  {:<10}  {}",
        "id", "storage", "present", "path"
    );
    println!("{}", "-".repeat(80));

    for f in &fixtures {
        let file_path = root.join(&f.path);
        let present = if file_path.exists() {
            "yes"
        } else {
            "no"
        };
        println!(
            "{:<30}  {:<10}  {:<10}  {}",
            f.id,
            f.storage,
            present,
            file_path.display()
        );
    }

    println!();
    let total = fixtures.len();
    let remote_count = fixtures.iter().filter(|f| f.storage == "remote").count();
    let missing_remote = fixtures
        .iter()
        .filter(|f| f.storage == "remote" && !root.join(&f.path).exists())
        .count();
    println!("{total} total fixtures, {remote_count} remote, {missing_remote} remote missing locally");
}

// ── fetch ─────────────────────────────────────────────────────────────────────

fn cmd_fetch(args: &[String]) {
    // Parse optional --fixture <id>
    let mut specific_id: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fixture" => {
                i += 1;
                specific_id = args.get(i).cloned();
                if specific_id.is_none() {
                    eprintln!("--fixture requires an argument");
                    std::process::exit(1);
                }
            }
            other => {
                eprintln!("unknown flag: {other:?}");
                eprintln!("usage: xtask corpus fetch [--fixture <id>]");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let fixtures = match list_fixtures() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("corpus fetch: cannot load manifest: {e}");
            std::process::exit(1);
        }
    };

    // Select fixtures to process
    let targets: Vec<&FixtureEntry> = if let Some(ref id) = specific_id {
        let matched: Vec<_> = fixtures.iter().filter(|f| &f.id == id).collect();
        if matched.is_empty() {
            eprintln!("corpus fetch: fixture '{id}' not found in manifest");
            std::process::exit(1);
        }
        matched
    } else {
        fixtures.iter().filter(|f| f.storage == "remote").collect()
    };

    if targets.is_empty() {
        println!("corpus fetch: no remote fixtures in manifest — nothing to do");
        return;
    }

    let root = corpus_root();
    let mut fetched = 0usize;
    let mut already_ok = 0usize;
    let mut failed = 0usize;

    for entry in &targets {
        if entry.storage != "remote" {
            // If --fixture was used on a non-remote entry, it's already inline
            let path = root.join(&entry.path);
            if path.exists() {
                println!("[ok]      {} (storage={}, already present)", entry.id, entry.storage);
                already_ok += 1;
            } else {
                eprintln!("[missing] {} (storage={}, but file not found at {})",
                    entry.id, entry.storage, path.display());
                failed += 1;
            }
            continue;
        }

        let dest = root.join(&entry.path);

        // Check if already present and valid
        if dest.exists() {
            match verify_sha256(&dest, &entry.sha256) {
                Ok(true) => {
                    println!("[ok]      {} (already present, sha256 matches)", entry.id);
                    already_ok += 1;
                    continue;
                }
                Ok(false) => {
                    eprintln!("[stale]   {} — sha256 mismatch, re-downloading", entry.id);
                }
                Err(e) => {
                    eprintln!("[warn]    {} — cannot read existing file: {e}; re-downloading", entry.id);
                }
            }
        }

        // Determine download URL
        let url = match resolve_url(entry) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("[error]   {} — {e}", entry.id);
                failed += 1;
                continue;
            }
        };

        println!("[fetch]   {} from {}", entry.id, url);

        // Create parent directory if needed
        if let Some(parent) = dest.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("[error]   {} — cannot create directory {}: {e}", entry.id, parent.display());
                failed += 1;
                continue;
            }
        }

        // Download
        match download(&url, &dest) {
            Ok(bytes_written) => {
                // Verify sha256
                match verify_sha256(&dest, &entry.sha256) {
                    Ok(true) => {
                        println!(
                            "[done]    {} — {:.1} KB written, sha256 ok",
                            entry.id,
                            bytes_written as f64 / 1024.0
                        );
                        fetched += 1;
                    }
                    Ok(false) => {
                        eprintln!(
                            "[error]   {} — downloaded {} bytes but sha256 mismatch (expected {})",
                            entry.id, bytes_written, entry.sha256
                        );
                        // Remove the bad file
                        let _ = std::fs::remove_file(&dest);
                        failed += 1;
                    }
                    Err(e) => {
                        eprintln!("[error]   {} — sha256 verify failed: {e}", entry.id);
                        let _ = std::fs::remove_file(&dest);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("[error]   {} — download failed: {e}", entry.id);
                failed += 1;
            }
        }
    }

    println!();
    println!("corpus fetch: {fetched} downloaded, {already_ok} already present, {failed} failed");
    if failed > 0 {
        std::process::exit(1);
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Resolve the download URL for a remote fixture entry.
///
/// Priority:
/// 1. `url` field in the manifest entry (explicit URL)
/// 2. Env vars `GRIBTRACT_B2_ENDPOINT` + `GRIBTRACT_B2_BUCKET` + sha256 key
///    → `<endpoint>/file/<bucket>/<sha256>`
fn resolve_url(entry: &FixtureEntry) -> Result<String, String> {
    if let Some(ref url) = entry.url {
        return Ok(url.clone());
    }

    let endpoint = std::env::var("GRIBTRACT_B2_ENDPOINT").map_err(|_| {
        "no download URL in manifest entry and GRIBTRACT_B2_ENDPOINT is not set. \
         Set GRIBTRACT_B2_ENDPOINT (e.g. https://f000.backblazeb2.com) and \
         GRIBTRACT_B2_BUCKET, or add a 'url' field to the manifest entry"
            .to_string()
    })?;
    let bucket = std::env::var("GRIBTRACT_B2_BUCKET").map_err(|_| {
        "GRIBTRACT_B2_ENDPOINT is set but GRIBTRACT_B2_BUCKET is not".to_string()
    })?;

    let endpoint = endpoint.trim_end_matches('/');
    Ok(format!("{}/file/{}/{}", endpoint, bucket, entry.sha256))
}

/// Download URL to a file, returning the number of bytes written.
fn download(url: &str, dest: &Path) -> Result<u64, String> {
    // Build request; attach auth if env vars are set
    let mut req = ureq::get(url);

    if let (Ok(account_id), Ok(app_key)) = (
        std::env::var("B2_ACCOUNT_ID"),
        std::env::var("B2_APPLICATION_KEY"),
    ) {
        let creds = format!("{}:{}", account_id, app_key);
        // Base64 encode credentials
        let b64 = base64_encode(creds.as_bytes());
        req = req.set("Authorization", &format!("Basic {b64}"));
    }

    let resp = req.call().map_err(|e| format!("HTTP GET {url}: {e}"))?;
    if resp.status() != 200 {
        return Err(format!(
            "HTTP {} from {url}",
            resp.status()
        ));
    }

    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)
        .map_err(|e| format!("cannot create {}: {e}", dest.display()))?;

    let mut buf = [0u8; 65536];
    let mut total = 0u64;
    loop {
        let n = std::io::Read::read(&mut reader, &mut buf)
            .map_err(|e| format!("read error: {e}"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])
            .map_err(|e| format!("write error: {e}"))?;
        total += n as u64;
    }

    Ok(total)
}

/// Verify a file's SHA-256. Returns `Ok(true)` if it matches, `Ok(false)` if not,
/// `Err` if the file cannot be read.
fn verify_sha256(path: &Path, expected: &str) -> Result<bool, String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    Ok(digest == expected)
}

/// Minimal base64 encoder (RFC 4648, standard alphabet, with padding).
/// Used for B2 Basic-auth header — avoids pulling in a base64 crate.
fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((n >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(n & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encode_roundtrip() {
        // RFC 4648 test vectors
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn resolve_url_uses_explicit_url() {
        let entry = FixtureEntry {
            id: "test".into(),
            path: "large/test.grib2".into(),
            sha256: "abc123".into(),
            size_bytes: 0,
            storage: "remote".into(),
            url: Some("https://example.com/test.grib2".into()),
        };
        let url = resolve_url(&entry).unwrap();
        assert_eq!(url, "https://example.com/test.grib2");
    }

    #[test]
    fn resolve_url_fails_without_env_or_url() {
        // Remove env vars if set
        std::env::remove_var("GRIBTRACT_B2_ENDPOINT");
        std::env::remove_var("GRIBTRACT_B2_BUCKET");
        let entry = FixtureEntry {
            id: "test".into(),
            path: "large/test.grib2".into(),
            sha256: "abc123".into(),
            size_bytes: 0,
            storage: "remote".into(),
            url: None,
        };
        assert!(resolve_url(&entry).is_err());
    }

    #[test]
    fn resolve_url_builds_b2_url_from_env() {
        std::env::set_var("GRIBTRACT_B2_ENDPOINT", "https://f000.backblazeb2.com");
        std::env::set_var("GRIBTRACT_B2_BUCKET", "gribtract-corpus");
        let entry = FixtureEntry {
            id: "test".into(),
            path: "large/test.grib2".into(),
            sha256: "deadbeef".into(),
            size_bytes: 0,
            storage: "remote".into(),
            url: None,
        };
        let url = resolve_url(&entry).unwrap();
        assert_eq!(url, "https://f000.backblazeb2.com/file/gribtract-corpus/deadbeef");
        std::env::remove_var("GRIBTRACT_B2_ENDPOINT");
        std::env::remove_var("GRIBTRACT_B2_BUCKET");
    }

    #[test]
    fn cmd_list_runs_without_panic() {
        // Just verify it doesn't crash; output goes to stdout
        // We can't assert output easily here, but we verify list_fixtures works
        let fixtures = list_fixtures().expect("manifest should be loadable in tests");
        assert!(!fixtures.is_empty());
        for f in &fixtures {
            // All current fixtures are inline or deferred, so no remote present
            assert!(
                f.storage == "inline" || f.storage == "remote" || f.storage == "deferred",
                "unexpected storage value: {}",
                f.storage
            );
        }
    }
}
