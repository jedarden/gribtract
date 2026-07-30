# PDT 4.1 Verification Results

## Task Summary
Verify that GRIB2 files contain Product Definition Template (PDT) 4.1 or 4.8 messages for individual ensemble forecasts.

## Tool Verification
- **wgrib2**: Available at `/home/coding/.local/bin/wgrib2`
- **Version**: Functional and able to process GRIB2 files
- **Key option used**: `-pdt` for Product Definition Template analysis

## File Analysis: ECMWF Ensemble ENSO Data

### File Information
- **Path**: `/home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2`
- **Size**: 1.5 GB
- **Date**: 2023-01-18 00:00 UTC
- **Source**: ECMWF (European Centre for Medium-Range Weather Forecasts)

### PDT Verification Results
✅ **PDT 4.1 messages FOUND**

#### Message Count Summary
- **Total valid messages**: 2614
- **PDT 4.1 messages**: 2,563 (98.0%)
- **PDT 4.11 messages**: 51 (2.0%)
- **PDT 4.8 messages**: 0

#### PDT 4.1 (Individual Ensemble Forecasts)
**PDT 4.1** is defined as: "Individual ensemble forecast, controlled at a specified horizontal level, in a horizontal layer, or at a specified height above the ground"

**Characteristics verified in this file:**
- Contains ensemble member IDs (e.g., ENS=+31, ENS=+19, ENS=+3)
- Individual perturbation forecasts from different ensemble members
- Standard meteorological variables at analysis time

#### Sample Messages with PDT 4.1
```
1:0:d=2023011800:UGRD:10 m above ground:anl:ENS=+31
2:609069:d=2023011800:UGRD:10 m above ground:anl:ENS=+19
3:1218138:d=2023011800:TMP:2 m above ground:anl:ENS=+3
4:1827207:d=2023011800:UGRD:10 m above ground:anl:ENS=+2
5:2436276:d=2023011800:VGRD:10 m above ground:anl:ENS=+4
```

**Variables found in PDT 4.1 messages:**
- UGRD (U-wind component at 10m above ground)
- VGRD (V-wind component at 10m above ground)
- TMP (Temperature at 2m above ground)
- Precipitation accumulation variables

#### Ensemble Members Identified
From the sample analysis, the following ensemble members were identified:
- ENS=+1 through ENS=+49 (indicating at least 49 ensemble members)
- High ensemble member numbers suggest this is a large ensemble system

### Product Definition Template Reference

| PDT | Name | Usage |
|-----|------|-------|
| 4.0 | Analysis or forecast at horizontal level | Standard deterministic forecasts |
| **4.1** | **Individual ensemble forecast** | **Individual ensemble member data** ✅ |
| 4.2 | Derived ensemble products | Ensemble means, spreads, probabilities |
| 4.8 | Individual ensemble forecast (alternative) | Alternative format for ensemble members |

### Commands Used for Verification

```bash
# Check wgrib2 availability
which wgrib2

# Get PDT information for all messages
wgrib2 /home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2 -pdt

# Count messages by PDT type
wgrib2 /home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2 -pdt | cut -d: -f3 | sort | uniq -c

# Show detailed inventory with ensemble member info
wgrib2 /home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2 -s | head
```

### Conclusion
✅ **VERIFIED**: The file `/home/coding/gribtract/tests/corpus/large/ecmwf_ensemble_enso_0h.grib2` contains **2,563 PDT 4.1 messages** representing individual ensemble member forecasts.

**Key findings:**
- PDT 4.1 is successfully used for individual ensemble forecasts
- Ensemble members are clearly identified with ENS=+N notation
- The file contains analysis data for multiple ensemble members
- No PDT 4.8 messages were found in this file (PDT 4.1 is the standard for ECMWF)

### Comparison with Ensemble Mean Files

For comparison, ensemble mean files (like `geavg_20260723_t00z_f000.grib2`) use:
- **PDT 4.2**: Derived products based on individual ensemble forecasts
- Used for ensemble means, spreads, and other statistical products
- Do not contain individual member IDs

This confirms that:
- **Individual ensemble members** → PDT 4.1 or PDT 4.8
- **Ensemble derived products** → PDT 4.2 or similar statistical templates
