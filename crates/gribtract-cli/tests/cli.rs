use assert_cmd::Command;
use predicates::str::contains;
use serde_json::{json, Value};
use std::path::PathBuf;

const FIXTURE_NAME: &str = "gfs_anl_t2m_5x5.grib2";
const MISSING_FIXTURE_NAME: &str = "gribtract-cli-nonexistent.grib2";

fn cli() -> Command {
    Command::cargo_bin("gribtract").expect("gribtract binary should be built for integration tests")
}

fn corpus_file(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/corpus/small")
        .join(name)
}

fn fixture() -> PathBuf {
    let path = corpus_file(FIXTURE_NAME);
    assert!(path.is_file(), "test fixture not found: {}", path.display());
    path
}

fn assert_missing_file_error(subcommand: &str) {
    let path = corpus_file(MISSING_FIXTURE_NAME);
    assert!(!path.exists(), "missing-file test path unexpectedly exists");

    let assertion = cli()
        .arg(subcommand)
        .arg(path)
        .assert()
        .failure()
        .stderr(contains("gribtract: error:"));

    let stderr = String::from_utf8_lossy(&assertion.get_output().stderr);
    assert!(!stderr.contains("panicked"), "CLI panicked: {stderr}");
    assert!(
        !stderr.contains("thread 'main'"),
        "CLI emitted a panic report: {stderr}"
    );
}

#[test]
fn decode_prints_expected_field_as_json() {
    let assertion = cli().arg("decode").arg(fixture()).assert().success();
    let output: Value = serde_json::from_slice(&assertion.get_output().stdout)
        .expect("decode stdout should be valid JSON");

    let fields = output
        .as_array()
        .expect("decode output should be a JSON array");
    assert_eq!(
        fields.len(),
        1,
        "fixture should decode to exactly one field"
    );
    assert_eq!(
        fields[0]["parameter"],
        json!({"discipline": 0, "category": 0, "number": 0})
    );
}

#[test]
fn list_prints_field_count() {
    cli()
        .arg("list")
        .arg(fixture())
        .assert()
        .success()
        .stdout(contains("\"field_count\": 1"));
}

#[test]
fn dump_prints_grib_header_as_hex() {
    cli()
        .arg("dump")
        .arg(fixture())
        .assert()
        .success()
        .stdout(contains("00000000  47 52 49 42"))
        .stdout(contains("|GRIB"));
}

#[test]
fn decode_reports_a_missing_file_without_panicking() {
    assert_missing_file_error("decode");
}

#[test]
fn list_reports_a_missing_file_without_panicking() {
    assert_missing_file_error("list");
}

#[test]
fn dump_reports_a_missing_file_without_panicking() {
    assert_missing_file_error("dump");
}
