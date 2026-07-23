# Bead bf-69bu: Local Storage Location for GRIB2 Files

## Decision

**Storage Path:** `/home/coding/gribtract/data`

## Rationale

The `data/` directory at the project root was selected for GRIB2 file storage because:

1. **Already exists** — the directory is present and already in use for GRIB2 data (contains `hrrr.t12z.wrfsfcf00.grib2`)
2. **Appropriate purpose** — logically separate from source code (`crates/`), test fixtures (`test_data/`), and tooling (`grib2/`)
3. **Already gitignored** — `.gitignore` contains patterns `*.grb`, `*.grib2`, `*.grb2` to exclude large operational data files
4. **Writable** — write permissions confirmed via touch test

## Acceptance Criteria

✅ Local storage directory exists: `/home/coding/gribtract/data`
✅ Path is documented (in this file and as a comment on bead bf-69bu)
✅ Directory is writable (verified)

## Git Exclusion

The workspace `.gitignore` already excludes GRIB2 files:

```gitignore
# local operational pointers (kept out of public repo)
*.grb
*.grib2
*.grb2
```

This ensures large operational data files are not committed to the repository.
