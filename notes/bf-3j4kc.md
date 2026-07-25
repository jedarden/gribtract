# CONUS DRT=0 Corpus Fetch Verification

## Task
Verify the new CONUS DRT=0 fixture downloads and hash-checks successfully.

## Execution

### Command Run
```bash
cargo xtask corpus fetch
```

### Results
- ✅ **Command completed successfully**: 1 downloaded, 9 already present, 0 failed
- ✅ **File downloaded**: `gfs_conus_drt0_0p50` from remote URL
  - Source: https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260724/00/atmos/gfs.t00z.pgrb2.0p50.f000
  - Size: 148541.4 KB (146 MB)
- ✅ **SHA256 hash verification passed**: `sha256 ok`
- ✅ **File stored in correct location**: `tests/corpus/large/gfs.t00z.pgrb2.0p50.f000`
- ✅ **Hash matches manifest**: `f2ccb6c8abaeee0a6b0e52f91a096ecdb3c3446384f27da63e5df7fccf3fc302`

## Acceptance Criteria Status
All acceptance criteria met:
1. ✅ cargo xtask corpus fetch completes successfully
2. ✅ File downloads from the remote URL
3. ✅ SHA256 hash verification passes
4. ✅ File is stored in the correct local corpus location
5. ✅ No errors or warnings related to the new fixture

## Conclusion
The CONUS DRT=0 fixture (gfs_conus_drt0_0p50) is successfully integrated into the corpus fetch system and downloads correctly with proper hash verification.
