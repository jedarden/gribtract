# Task bf-2r93s: Run wgrib2 -pdrt on CONUS GRIB2 file

## Task Attempt

The task requested running `wgrib2 -pdrt` on a CONUS GRIB2 file. However, this revealed several issues:

### Issue 1: Invalid wgrib2 option

The `-pdrt` option does not exist in wgrib2 v3.1.3:
```
*** FATAL ERROR: unknown option -pdrt ***
```

### Issue 2: Correct command for DRT information

To display Data Representation Template (DRT) information, the correct wgrib2 option is `-Sec5`:
```bash
wgrib2 /path/to/file.grib2 -Sec5
```

### Issue 3: Actual DRT values in CONUS file

Running `-Sec5` on the CONUS file (`nam.t12z.afwaca00.tm00.grib2`) shows:
```
1:0:Sec5 len=49 #defined data points=92556 Data Repr. Template=5.3
2:99017:Sec5 len=49 #defined data points=92556 Data Repr. Template=5.3
...
```

All messages use **Data Representation Template 5.3** (complex packing with spatial differencing), **not DRT=0** (simple packing).

## DRT Reference

- **DRT 0.0** = Simple packing
- **DRT 5.3** = Complex packing + spatial differencing (c3b)

## Conclusion

The task goal was to "confirm DRT=0 packing," but the actual CONUS file uses DRT=5.3 (complex packing). The `-pdrt` option does not exist in wgrib2; the equivalent is `-Sec5`.

## Existing Output Files

The notes directory already contains comprehensive wgrib2 output:
- `wgrib2_sec5_output.txt` - DRT information (shows 5.3)
- `wgrib2_packing_output.txt` - Packing details (shows c3b)
- `wgrib2_inventory.txt` - Full inventory
- `wgrib2_pdrt_output.txt` - Failed attempt showing "-pdrt not valid"
