//! Corpus fixture loader.
//!
//! Reads `tests/corpus/manifest.json` in the workspace root, verifies each
//! fixture's SHA-256 on first load, and returns the raw bytes.
//!
//! # Finding the workspace root
//!
//! `CARGO_MANIFEST_DIR` for this crate is `<workspace>/crates/gribtract-testutil`.
//! We walk two levels up to find the workspace root. This is fragile only if the
//! crate moves within the workspace; callers can override with `GRIBTRACT_CORPUS_ROOT`.

use std::path::{Path, PathBuf};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
struct Manifest {
    version: u32,
    fixtures: Vec<FixtureEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FixtureEntry {
    pub id: String,
    pub path: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub storage: String,
}

/// Locate the `tests/corpus/` directory for this workspace.
///
/// Checks `GRIBTRACT_CORPUS_ROOT` env var first; falls back to
/// `<CARGO_MANIFEST_DIR>/../../tests/corpus`.
pub fn corpus_root() -> PathBuf {
    if let Ok(root) = std::env::var("GRIBTRACT_CORPUS_ROOT") {
        return PathBuf::from(root);
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("../..")
        .join("tests/corpus")
}

/// Load and parse the fixture manifest.
fn load_manifest() -> Result<Manifest, String> {
    let manifest_path = corpus_root().join("manifest.json");
    let json = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("cannot read manifest {}: {}", manifest_path.display(), e))?;
    let manifest: Manifest = serde_json::from_str(&json)
        .map_err(|e| format!("cannot parse manifest: {}", e))?;
    if manifest.version != 1 {
        return Err(format!("unsupported manifest version: {}", manifest.version));
    }
    Ok(manifest)
}

/// Return all entries from the manifest.
pub fn list_fixtures() -> Result<Vec<FixtureEntry>, String> {
    Ok(load_manifest()?.fixtures)
}

/// Find the manifest entry for a fixture by id.
pub fn fixture_entry(id: &str) -> Result<FixtureEntry, String> {
    let manifest = load_manifest()?;
    manifest
        .fixtures
        .into_iter()
        .find(|f| f.id == id)
        .ok_or_else(|| format!("fixture '{}' not found in manifest", id))
}

/// Load fixture bytes by id, verifying SHA-256.
///
/// Returns `Err` if the fixture is not in the manifest, the file is missing,
/// or the digest does not match.
pub fn load(id: &str) -> Result<Vec<u8>, String> {
    let entry = fixture_entry(id)?;

    if entry.storage == "remote" {
        return Err(format!(
            "fixture '{}' has storage=remote — run `cargo xtask corpus fetch {}` first",
            id, id
        ));
    }

    let file_path = corpus_root().join(&entry.path);
    let bytes = std::fs::read(&file_path)
        .map_err(|e| format!("cannot read fixture {}: {}", file_path.display(), e))?;

    // Verify size
    if bytes.len() as u64 != entry.size_bytes {
        return Err(format!(
            "fixture '{}' size mismatch: manifest says {} bytes, file is {}",
            id, entry.size_bytes, bytes.len()
        ));
    }

    // Verify SHA-256
    let digest = format!("{:x}", Sha256::digest(&bytes));
    if digest != entry.sha256 {
        return Err(format!(
            "fixture '{}' SHA-256 mismatch:\n  expected {}\n  got      {}",
            id, entry.sha256, digest
        ));
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_parses() {
        let manifest = load_manifest().expect("manifest should parse");
        assert_eq!(manifest.version, 1);
        assert!(!manifest.fixtures.is_empty(), "manifest should have at least one fixture");
    }

    #[test]
    fn gfs_anl_t2m_5x5_loads_and_verifies() {
        let bytes = load("gfs_anl_t2m_5x5").expect("fixture should load");

        // GRIB2 magic at start
        assert_eq!(&bytes[..4], b"GRIB", "missing GRIB magic");

        // Edition 2 at byte 7
        assert_eq!(bytes[7], 2, "expected GRIB edition 2");

        // Total length encoded in bytes 8-15 (big-endian u64)
        let msg_len = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        assert_eq!(msg_len as usize, bytes.len(), "encoded length should match file size");

        // End marker "7777"
        assert_eq!(&bytes[bytes.len() - 4..], b"7777", "missing 7777 end marker");

        // Discipline = 0 (meteorological products)
        assert_eq!(bytes[6], 0, "discipline should be 0 (meteorological)");
    }

    #[test]
    fn missing_fixture_returns_err() {
        assert!(load("does_not_exist").is_err());
    }
}
