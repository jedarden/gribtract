# GRIB2 Format Validation Summary

## Task: Validate GRIB2 format (bf-346p0)

Date: 2026-07-23

## Files Validated

1. `data/conus_drt0_mxuphl_20260723.grib2` (212 bytes)
2. `data/conus_drt0_mxuphl.grib2` (212 bytes)

## Tool Used
- **wgrib2**: Installed at `/home/coding/.local/bin/wgrib2`

## Validation Results

### conus_drt0_mxuphl_20260723.grib2
```
1:0:vt=2026072312:5000-2000 m above ground:0-0 day max fcst:MXUPHL Hourly Maximum of Updraft Helicity [m^2/s^2]:
    ndata=1905141:undef=0:mean=0:min=0:max=0
    grid_template=30:winds(grid):
	Lambert Conformal: (1799 x 1059) input WE:SN output WE:SN res 8
	Lat1 21.138123 Lon1 237.280472 LoV 262.500000
	LatD 38.500000 Latin1 38.500000 Latin2 38.500000
	LatSP 0.000000 LonSP 0.000000
	North Pole (1799 x 1059) Dx 3000.000000 m Dy 3000.000000 m
```

### conus_drt0_mxuphl.grib2
```
1:0:vt=2024060112:5000-2000 m above ground:0-0 day max fcst:MXUPHL Hourly Maximum of Updraft Helicity [m^2/s^2]:
    ndata=1905141:undef=0:mean=0:min=0:max=0
    grid_template=30:winds(grid):
	Lambert Conformal: (1799 x 1059) input WE:SN output WE:SN res 8
	Lat1 21.138123 Lon1 237.280472 LoV 262.500000
	LatD 38.500000 Latin1 38.500000 Latin2 38.500000
	LatSP 0.000000 LonSP 0.000000
	North Pole (1799 x 1059) Dx 3000.000000 m Dy 3000.000000 m
```

## Acceptance Criteria Met

✓ **wgrib2 confirms valid GRIB2 format** - Both files parsed successfully with full metadata
✓ **No format errors reported** - Clean wgrib2 output with no errors or warnings
✓ **File is parseable by wgrib2** - Complete grid information, data values, and inventory extracted

## File Details

- **Format**: GRIB2 (confirmed by wgrib2 parsing)
- **Parameter**: MXUPHL (Hourly Maximum of Updraft Helicity in m^2/s^2)
- **Grid**: Lambert Conformal projection
- **Resolution**: 3km (3000m)
- **Grid dimensions**: 1799 x 1059 points (1,905,141 total points)
- **Coverage**: CONUS (Continental United States)
- **Data values**: All zeros (test data files)

## Conclusion

Both downloaded GRIB2 files are valid and properly formatted according to wgrib2 validation.
