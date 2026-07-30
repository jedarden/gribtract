# GDT Findings for NAM AWIP12 CONUS File

**Bead:** bf-4jew
**Date:** 2025-01-15
**Source:** `docs/wgrib2_grid_inspection_bf-1wy7.txt`

---

## Grid Definition Template (GDT) Number

**GDT: 3.30 (Lambert Conformal Conic)** ✓

---

## Summary

All 794 GRIB2 messages in the NAM AWIP12 CONUS file use **GDT 3.30** (Lambert Conformal Conic projection). This matches the expected value for regional NAM CONUS datasets.

---

## Grid Definition Parameters

| Parameter | Value |
|-----------|-------|
| **GDT Number** | 3.30 |
| **Template Name** | Lambert Conformal Conic |
| **Grid Type** | lambert |
| **Grid Dimensions** | 1799 x 1059 points |
| **Data Representation Template (DRT)** | 3 (Complex Packing) |

---

## Verification

- ✓ GDT 3.30 confirmed for all 794 messages (100% consistency)
- ✓ Grid dimensions consistent across all messages
- ✓ DRT=3 (Complex Packing with spatial differencing)
- ✓ Standard NAM CONUS grid configuration

---

## Conclusion

The Grid Definition Template number extracted from the wgrib2 output is **3.30**, which corresponds to the **Lambert Conformal Conic** projection. This is the correct and expected GDT for NAM AWIP12 CONUS GRIB2 files.

---

**Reference:** `docs/wgrib2_grid_inspection_bf-1wy7.txt`, lines 1-246
