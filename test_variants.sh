#!/bin/bash
# Test each GRIB2 variant to identify essential sections

echo "=== GRIB2 Section Essentiality Test ==="
echo ""

# Build gribtract first
echo "Building gribtract..."
cargo build --release --bin gribtract --quiet 2>/dev/null
GRIBTRACT="./target/release/gribtract"

if [ ! -f "$GRIBTRACT" ]; then
    echo "ERROR: Could not build gribtract"
    exit 1
fi

echo "Using binary: $GRIBTRACT"
echo ""

# Test each variant
variants=(
    "original"
    "no_section4"
    "no_section5"
    "no_section6"
    "no_section7"
    "minimal_013"
    "fixed_section3"
)

results=()

for variant in "${variants[@]}"; do
    file="/tmp/gribtest/${variant}.grib2"

    if [ ! -f "$file" ]; then
        echo "⚠ $variant: File not found"
        continue
    fi

    echo "Testing $variant:"

    # Run gribtract decode and capture output
    output=$($GRIBTRACT decode "$file" 2>&1)
    exit_code=$?

    # Check if buffer underrun occurred
    if echo "$output" | grep -qi "tooshort\|too short"; then
        echo "  ✓ BUFFER UNDERRUN TRIGGERED"
        results+=("$variant:UNDERRUN")
    elif echo "$output" | grep -qi "notimplemented\|not implemented"; then
        echo "  ✗ NOT IMPLEMENTED (different code path)"
        results+=("$variant:NOT_IMPLEMENTED")
    elif echo "$output" | grep -qi "error"; then
        echo "  ✗ DIFFERENT ERROR:"
        echo "$output" | head -3 | sed 's/^/    /'
        results+=("$variant:OTHER_ERROR")
    elif [ $exit_code -eq 0 ]; then
        echo "  ✗ DECODING SUCCEEDED (bug fixed or variant invalid)"
        results+=("$variant:SUCCESS")
    else
        echo "  ? UNKNOWN RESULT"
        echo "$output" | head -3 | sed 's/^/    /'
        results+=("$variant:UNKNOWN")
    fi

    echo ""
done

echo "=== SUMMARY ==="
echo ""
echo "Essential sections trigger the bug (NON-ESSENTIAL sections can be removed)"
echo ""

for result in "${results[@]}"; do
    variant="${result%:*}"
    status="${result#*:}"

    case "$status" in
        UNDERRUN)
            echo "✓ $variant: Buffer underrun occurs (sections in this variant are sufficient)"
            ;;
        NOT_IMPLEMENTED|OTHER_ERROR|SUCCESS)
            echo "✗ $variant: No buffer underrun (removed ESSENTIAL section)"
            ;;
        *)
            echo "? $variant: Unknown status"
            ;;
    esac
done

echo ""
echo "=== CONCLUSION ==="
echo ""
echo "Based on test results:"
echo "- If removing a section STILL causes underrun → Section is NON-ESSENTIAL"
echo "- If removing a section STOPS causing underrun → Section is ESSENTIAL"
echo ""
echo "Expected findings:"
echo "- Section 0 (Indicator): ESSENTIAL - Required for GRIB format"
echo "- Section 1 (Identification): ESSENTIAL - Contains metadata"
echo "- Section 3 (Grid Definition): ESSENTIAL - THE TRIGGER"
echo "  → Claims 72 bytes, actual data causes shortage"
echo "  → This mismatch triggers buffer underrun"
echo "- Sections 4, 5, 6, 7: NON-ESSENTIAL - Can be removed"
