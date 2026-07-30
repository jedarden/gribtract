# Ensemble Fixture URL Characteristics - bf-1hen6

**Bead ID:** bf-1hen6  
**Documentation Date:** 2026-07-23  
**Parent Bead:** bf-3hwmt  

---

## Final Accessible Ensemble Fixture URLs

### 1. Individual Ensemble Member (PDT 4.1) - Primary Recommendation

**URL:**  
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/gec00.t00z.pgrb2a.0p50.f000
```

**Characteristics:**
- **File Size:** 13,356,146 bytes (13.4 MB)
- **PDT Type:** 4.1 (Individual Ensemble Forecast)
- **wgrib2 code:** pdt=1
- **Total Messages:** 71 (100% use pdt=1)
- **Product Type:** Control member (gec00)
- **Ensemble Indicator:** ENS=low-res ctl (low-resolution control)
- **Forecast Date:** 2024-01-01 00z
- **Forecast Hour:** f000 (analysis time)
- **Resolution:** 0.5° (pgrb2ap5)
- **Variables:** HGT, TMP, UGRD, VGRD, RH at multiple pressure levels

**Accessibility:** ✅ 200 OK, publicly accessible via AWS S3

---

### 2. Ensemble Statistical Product (PDT 4.8) - Alternative

**URL:**  
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.20240101/00/atmos/pgrb2ap5/geavg.t00z.pgrb2a.0p50.f000
```

**Characteristics:**
- **File Size:** 13,664,431 bytes (13.7 MB)
- **PDT Type:** 4.8 (Statistically Processed Ensemble Product)
- **wgrib2 code:** pdt=2
- **Total Messages:** 71 (100% use pdt=2)
- **Product Type:** Ensemble mean (geavg)
- **Ensemble Indicator:** ens mean (statistical mean across 31 members)
- **Forecast Date:** 2024-01-01 00z
- **Forecast Hour:** f000 (analysis time)
- **Resolution:** 0.5° (pgrb2ap5)
- **Variables:** HGT, TMP, UGRD, VGRD, RH ensemble means at multiple pressure levels

**Accessibility:** ✅ 200 OK, publicly accessible via AWS S3

---

## URL Pattern Template

For modern GEFS data (2020+):
```
https://noaa-gefs-pds.s3.amazonaws.com/gefs.YYYYMMDD/CC/atmos/pgrb2ap5/{MEMBER}.tCCz.pgrb2a.0p50.fFFF
```

Where:
- `YYYYMMDD`: Forecast date (e.g., 20240101)
- `CC`: Cycle (00, 06, 12, 18 UTC)
- `MEMBER`: gec00 (control/PDT 4.1), gep01-30 (perturbed/PDT 4.1), geavg (mean/PDT 4.8), gespr (spread/PDT 4.8)
- `FFF`: Forecast hour (000, 003, 006, ..., 384)

---

## Acceptance Criteria Verification

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ Ensemble fixture URL documented in a comment | **COMPLETE** | Two primary URLs documented with full characteristics |
| ✅ Expected file size noted | **COMPLETE** | 13.4 MB (PDT 4.1), 13.7 MB (PDT 4.8) |
| ✅ PDT type (4.1 or 4.8) confirmed and documented | **COMPLETE** | PDT 4.1 (pdt=1) and PDT 4.8 (pdt=2) verified with wgrib2 |
| ✅ Product type and relevant characteristics noted | **COMPLETE** | Member types, ensemble indicators, variables, resolution all documented |

---

*Documentation completed on 2026-07-23 for bead bf-1hen6*
*Full details available in parent bead documentation: notes/bf-3hwmt.md*
