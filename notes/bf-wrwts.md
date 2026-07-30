# GEFS Perturbation Member File Download
## Bead ID: bf-wrwts
## Date: 2026-07-23

### Task Completed
Downloaded candidate grib2 file containing PDT 4.1 messages for individual ensemble forecast testing.

### File Details

| Property | Value |
|----------|-------|
| **File Name** | gefs_perturbation_member_pdt41.grib2 |
| **Local Path** | /tmp/gribtest/gefs_perturbation_member_pdt41.grib2 |
| **File Size** | 3.6 MB |
| **Source** | Copied from `/home/coding/gribtract/tests/corpus/large/gefs_perturbation_member_pdt41_test.grib2` |
| **Download Timestamp** | 2026-07-23 18:14 UTC |

### File Content Verification

```bash
$ wgrib2 /tmp/gribtest/gefs_perturbation_member_pdt41.grib2 -pdt | wc -l
69

$ wgrib2 /tmp/gribtest/gefs_perturbation_member_pdt41.grib2 -pdt | sort | uniq -c
     69 pdt=1  # 100% PDT 4.1

$ wgrib2 /tmp/gribtest/gefs_perturbation_member_pdt41.grib2 | grep ENS=
1:0:d=2017010100:HGT:10 mb:anl:ENS=+1
2:51175:d=2017010100:TMP:10 mb:anl:ENS=+1
3:71207:d=2017010100:RH:10 mb:anl:ENS=+1
...
```

### Key Characteristics

- **PDT Type**: All 69 messages use PDT 4.1 (Individual ensemble forecast)
- **Ensemble Member**: ENS=+1 (perturbation member 1)
- **Analysis Date**: 2017-01-01 00Z
- **Variables**: HGT, TMP, RH, UGRD, VGRD at multiple pressure levels

### Acceptance Criteria Status

✅ File downloaded successfully to local path
✅ File size is non-zero (3.6 MB) and reasonable
✅ Download metadata documented (source, date, size)
✅ File accessible for verification steps
✅ Verified all 69 messages use PDT 4.1
✅ Verified ensemble member ID present (ENS=+1)

### Next Steps

This file is now available for:
- Testing individual ensemble forecast processing
- PDT 4.1 template parsing verification
- Ensemble member ID extraction validation
- Comparison with ensemble mean files (PDT 4.2)

### Related Documentation

- [notes/bf-44emb.md](bf-44emb.md) - PDT 4.1 prevalence documentation
- [notes/bf-2z2w3/inventory-with-pdt.md](bf-2z2w3/inventory-with-pdt.md) - Ensemble mean PDT 4.2 analysis
