# bf-fouy: Fetch & Verify Remote Storage for DRT=3 Fixture

## Task Summary

Verified that `cargo xtask corpus fetch` correctly resolves, downloads, and
sha256-verifies the first `storage=remote` corpus fixture —
`nam_awip12_lambert_drt3` (registered in [bf-4chp](bf-4chp.md)) — from its remote
source. This is the end-to-end remote-fetch validation for the DRT=3 / Lambert
target fixture.

**Result: PASS** — remote download, sha256 verification, size check, and the
corpus manifest test suites all pass.

---

## Remote storage model (why there is no "upload" step)

The manifest entry does **not** use the project's Backblaze B2 path. It carries a
top-level `url` field pointing directly at NOAA's public S3 bucket:

```
storage : remote
url     : https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
```

Per `xtask/src/corpus.rs::resolve_url`, an explicit `url` field takes priority
over the `GRIBTRACT_B2_*` env-var path, so `corpus fetch` pulls straight from the
NOAA Open Data bucket with **no B2 env vars and no upload** required. The fixture
data is public NOAA NAM output that already lives on `noaa-nam-pds`; "remote
storage" for this entry *is* that bucket. (B2 upload would only apply to entries
with no `url` field, which are seeded via `GRIBTRACT_B2_*`.)

---

## Verification evidence

### 1. Remote URL reachable & intact (`curl -I`)

```
HTTP/1.1 200 OK
Content-Length: 26364442          ← matches manifest size_bytes exactly
Accept-Ranges: bytes
Content-Type:   binary/octet-stream
Last-Modified:  Wed, 15 Jan 2025 01:39:19 GMT
```

### 2. Forced re-download from remote (file removed first)

Deleted the local copy under `tests/corpus/large/` so fetch could not
short-circuit on "already present":

```
[fetch]   nam_awip12_lambert_drt3 from https://noaa-nam-pds.s3.amazonaws.com/nam.20250115/nam.t00z.awip1200.tm00.grib2
[done]    nam_awip12_lambert_drt3 — 25746.5 KB written, sha256 ok
corpus fetch: 1 downloaded, 0 already present, 0 failed
```

Real bytes streamed from the S3 URL; xtask's own `Sha256` check reported `ok`.

### 3. Independent sha256 + size verification (system `sha256sum`)

```
actual:   b022c093603e67ebcc006a8e50cb30610bf4e3ce7d6609733d9949b5add6bf2c
manifest: b022c093603e67ebcc006a8e50cb30610bf4e3ce7d6609733d9949b5add6bf2c
✓ SHA-256 MATCH

bytes: 26364442   (manifest expects 26364442)
```

The system hash agrees with the manifest independent of the xtask verifier.

### 4. Default no-arg fetch path (fetches ALL remote fixtures)

```
[ok]      nam_awip12_lambert_drt3 (already present, sha256 matches)
corpus fetch: 0 downloaded, 1 already present, 0 failed
```

### 5. Corpus manifest test suites

```
gribtract-testutil corpus (3 passed)  : manifest_parses, gfs_anl_t2m_5x5_loads_and_verifies, missing_fixture_returns_err
xtask corpus::tests      (5 passed)   : base64_encode_roundtrip, resolve_url_uses_explicit_url,
                                        resolve_url_builds_b2_url_from_env, resolve_url_fails_without_env_or_url,
                                        cmd_list_runs_without_panic
```

`resolve_url_uses_explicit_url` is the unit test covering the `url`-field path
this fixture relies on.

---

## Acceptance criteria

| Criterion | Status |
|---|---|
| `cargo xtask corpus fetch` succeeds | ✅ PASS (1 downloaded, 0 failed) |
| sha256 verification passes | ✅ PASS (xtask + independent `sha256sum` agree) |
| File accessible from remote storage | ✅ PASS (HTTP 200, Accept-Ranges, exact Content-Length) |
| No errors in fetch output | ✅ PASS (0 failed, no `[error]`/`[stale]` lines) |

Fixture is left present locally at `tests/corpus/large/nam.t00z.awip1200.tm00.grib2`
(gitignored — correct for `storage=remote`).

Depends on [bf-4chp](bf-4chp.md) (ADD_DRT3_TO_MANIFEST) — satisfied.
