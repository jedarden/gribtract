# bf-5yysjb: List and Dump Integration Tests

## Task Summary
Add integration test coverage for the list and dump subcommands using the gfs_anl_t2m_5x5 fixture.

## Findings
The integration tests for `list` and `dump` subcommands were **already implemented** in `crates/gribtract-cli/tests/cli.rs` and meet all acceptance criteria.

## Existing Tests

### List Subcommand Test (`test_list_subcommand_success`, lines 64-77)
- Uses `tests/corpus/small/gfs_anl_t2m_5x5.grib2` fixture
- Asserts exit code is 0 via `.success()`
- Asserts stdout contains `"field_count"` key
- Asserts stdout contains `"file":` key

### Dump Subcommand Test (`test_dump_subcommand_success`, lines 80-93)
- Uses `tests/corpus/small/gfs_anl_t2m_5x5.grib2` fixture
- Asserts exit code is 0 via `.success()`
- Asserts stdout produces hex output via `predicates::str::contains("|")` and `"00000000"` (hex offset markers)

## Verification
- ✅ All 6 CLI integration tests pass (decode/list/dump + nonexistent file error cases)
- ✅ Temporarily breaking the list subcommand (removing `"field_count"` output) causes `test_list_subcommand_success` to fail with clear error message
- ✅ Tests use proper patterns from the decode test (`bf-jzcd9q`)
- ✅ Test infrastructure from `bf-4g9jsz` provides `get_cli_binary()` and `get_test_fixture_path()` helpers

## Conclusion
No code changes were required - the integration tests already existed and passed all acceptance criteria.
