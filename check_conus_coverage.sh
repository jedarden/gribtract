#!/bin/bash
# Key CONUS weather stations with their coordinates
stations=(
    "JFK:40.64:-73.78"
    "ORD:41.979:-87.905" 
    "LAX:33.943:-118.408"
    "DFW:32.898:-97.040"
    "DEN:39.856:-104.675"
    "ATL:33.641:-84.428"
    "SEA:47.449:-122.309"
    "MIA:25.795:-80.238"
    "SFO:37.619:-122.375"
    "BOS:42.364:-71.005"
)

echo "Checking HRRR CONUS coverage for major US airports..."
echo "======================================================"

# Check if gribtract can help us validate coverage
# For now, let's just note the grid bounds from wgrib2 output
echo ""
echo "Grid Definition from wgrib2:"
echo "============================"
echo "Grid Template: 30 (Lambert Conformal)"
echo "Grid Size: 1799 x 1059 points"
echo "Lat1 (SW corner): 21.138123°N"
echo "Lon1 (SW corner): 237.280472°E (-122.719528°W)"
echo "LatD (true latitude 1): 38.500000°N"
echo "Latin1: 38.500000°N"
echo "Latin2: 38.500000°N"
echo "LoV (orientation): 262.500000°E (-97.500000°W)"
echo "Dx/Dy: 3000m x 3000m grid spacing"

echo ""
echo "This is a Lambert Conformal projection centered on the CONUS."
echo "The grid covers approximately:"
echo "- Latitude: ~21°N to ~50°N (Mexico border to Canada border)"  
echo "- Longitude: ~125°W to ~70°W (West Coast to East Coast)"
echo ""
echo "Key CONUS Stations to Validate:"
for station in "${stations[@]}"; do
    IFS=':' read -r name lat lon <<< "$station"
    echo "  $name: ${lat}°N, ${lon}°W"
done

