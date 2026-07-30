#!/bin/bash

echo "=== Analyzing Minimal GRIB2 Files ==="
echo ""
echo "1. File sizes and structure:"
echo ""

for file in tests/corpus/small/minimal_underrun*.grib2 tests/corpus/small/rotated_latlon_gdt1_drt0.grib2; do
    if [ -f "$file" ]; then
        echo "File: $file"
        echo "Size: $(stat -c%s "$file") bytes"
        echo "First 64 bytes (hexdump):"
        hexdump -C "$file" | head -4
        echo "Last 16 bytes (hexdump):"
        hexdump -C "$file" | tail -2
        echo ""
    fi
done

echo "=== Section analysis ==="
echo ""
echo "GRIB2 structure:"
echo "- Indicator Section (Section 0): 'GRIB' + edition 2"
echo "- Identification Section (Section 1)"
echo "- Grid Definition Section (Section 3)"
echo "- Product Definition Section (Section 4)"
echo "- Data Representation Section (Section 5)"
echo "- Data Section (Section 6)"
echo "- End Section (Section 7): '7777'"
