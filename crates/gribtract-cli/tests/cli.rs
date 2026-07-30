use assert_cmd::Command;
use serde_json::Value;
use std::path::Path;

fn get_cli_binary() -> Command {
    Command::cargo_bin("gribtract").unwrap()
}

fn get_test_fixture_path(fixture_name: &str) -> String {
    // Navigate from crate directory to workspace root, then to corpus
    // CARGO_MANIFEST_DIR is the crate directory (crates/gribtract-cli)
    // We need to go up two levels to reach workspace root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap().parent().unwrap();
    workspace_root
        .join("tests")
        .join("corpus")
        .join("small")
        .join(fixture_name)
        .to_string_lossy()
        .to_string()
}

#[test]
fn test_decode_subcommand_success() {
    let fixture_path = get_test_fixture_path("gfs_anl_t2m_5x5.grib2");

    // Ensure the fixture exists
    assert!(
        Path::new(&fixture_path).exists(),
        "Test fixture not found: {}",
        fixture_path
    );

    let assertion = get_cli_binary()
        .arg("decode")
        .arg(&fixture_path)
        .assert()
        .success();

    let output = assertion.get_output();
    let stdout = String::from_utf8(output.stdout.clone()).expect("stdout is not valid UTF-8");

    // Parse stdout as valid JSON
    let json: Value = serde_json::from_str(&stdout).expect("stdout is not valid JSON");

    // Verify it's a JSON array
    let fields = json.as_array().expect("decoded output is not a JSON array");
    assert!(
        !fields.is_empty(),
        "decoded output should contain at least one field"
    );

    // Verify first field has expected structure
    let first_field = &fields[0];
    assert!(first_field.is_object(), "field should be a JSON object");

    // Check for required top-level fields
    assert!(
        first_field.get("center").is_some(),
        "field should have 'center'"
    );
    assert!(
        first_field.get("parameter").is_some(),
        "field should have 'parameter'"
    );
    assert!(
        first_field.get("level").is_some(),
        "field should have 'level'"
    );
    assert!(
        first_field.get("forecast_time").is_some(),
        "field should have 'forecast_time'"
    );
    assert!(
        first_field.get("grid").is_some(),
        "field should have 'grid'"
    );
    assert!(
        first_field.get("values").is_some(),
        "field should have 'values'"
    );

    // Verify parameter object has expected fields
    let parameter = first_field.get("parameter").unwrap().as_object().unwrap();
    assert!(
        parameter.contains_key("discipline"),
        "parameter should have 'discipline'"
    );
    assert!(
        parameter.contains_key("category"),
        "parameter should have 'category'"
    );
    assert!(
        parameter.contains_key("number"),
        "parameter should have 'number'"
    );

    // Verify center is expected value (NCEP/EMC for GFS)
    assert_eq!(
        first_field.get("center").unwrap().as_i64(),
        Some(7),
        "center should be 7 (NCEP)"
    );
}

#[test]
fn test_list_subcommand_success() {
    let fixture_path = get_test_fixture_path("gfs_anl_t2m_5x5.grib2");

    // Ensure the fixture exists
    assert!(
        Path::new(&fixture_path).exists(),
        "Test fixture not found: {}",
        fixture_path
    );

    let mut cmd = get_cli_binary();
    cmd.arg("list")
        .arg(&fixture_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("\"field_count\""))
        .stdout(predicates::str::contains("\"file\":"));
}

#[test]
fn test_dump_subcommand_success() {
    let fixture_path = get_test_fixture_path("gfs_anl_t2m_5x5.grib2");

    // Ensure the fixture exists
    assert!(
        Path::new(&fixture_path).exists(),
        "Test fixture not found: {}",
        fixture_path
    );

    let mut cmd = get_cli_binary();
    cmd.arg("dump")
        .arg(&fixture_path)
        .assert()
        .success()
        .stdout(predicates::str::contains("|"))
        .stdout(predicates::str::contains("00000000"));
}

#[test]
fn test_decode_nonexistent_file() {
    let mut cmd = get_cli_binary();
    cmd.arg("decode")
        .arg("/nonexistent/path/to/file.grib2")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}

#[test]
fn test_list_nonexistent_file() {
    let mut cmd = get_cli_binary();
    cmd.arg("list")
        .arg("/nonexistent/path/to/file.grib2")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}

#[test]
fn test_dump_nonexistent_file() {
    let mut cmd = get_cli_binary();
    cmd.arg("dump")
        .arg("/nonexistent/path/to/file.grib2")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}
