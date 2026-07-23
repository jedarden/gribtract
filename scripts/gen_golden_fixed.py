#!/usr/bin/env python3
"""gen_golden_fixed.py — Generate gribtract golden reference files using basic GRIB2 parsing.

This script implements basic GRIB2 file parsing from scratch without external dependencies.
It extracts key metadata from GRIB2 messages and outputs JSON structure.

Usage:
    python3 scripts/gen_golden_fixed.py <grib2_file> <fixture_id> [--output-dir DIR]

Output:
    tests/corpus/golden/<fixture_id>.json
"""

import argparse
import json
import struct
import sys
from pathlib import Path
from datetime import datetime, timezone


# ============================================================================
# GRIB2 Parameter and Level Tables
# ============================================================================

# Discipline codes (Code Table 0.0)
DISCIPLINE_TABLE = {
    0: "Meteorological",
    1: "Hydrological",
    2: "Land Surface",
    3: "Space Weather",
    10: "Oceanographic",
    192: "Experimental",
    255: "Missing"
}

# Parameter category names for Discipline 0 (Meteorological)
PARAM_CATEGORY_MET = {
    0: "Temperature",
    1: "Moisture",
    2: "Momentum",
    3: "Wind",
    4: "Mass",
    5: "Short-wave Radiation",
    6: "Cloud",
    7: "Thermodynamic",
    8: "Ozone",
    9: "Vertical Motion",
    10: "Radiation (Long-wave)",
    11: "Sea Surface",
    12: "Land Surface",
    13: "Atmospheric Chemical Constituents",
    14: "Movement",
    15: "Imagery",
    16: "Active Passives",
    191: "Physical/optical properties of aerosol",
    192: "Physical/optical properties of aerosol",
    193: "Forecast probability indices",
    194: "Nuclear/radiological",
    255: "Missing"
}

# Common parameter names and units for Discipline 0 (NCEP operational parameters)
# Format: (discipline, category, number): (short_name, long_name, units)
PARAM_TABLE_NCEP = {
    (0, 0, 0): ("TMP", "Temperature", "K"),
    (0, 0, 1): ("VTMP", "Virtual Temperature", "K"),
    (0, 0, 2): ("POT", "Potential Temperature", "K"),
    (0, 0, 3): ("EPOT", "Equivalent Potential Temperature", "K"),
    (0, 0, 4): ("TMAX", "Maximum Temperature", "K"),
    (0, 0, 5): ("TMIN", "Minimum Temperature", "K"),
    (0, 0, 6): ("DPT", "Dew Point Temperature", "K"),
    (0, 0, 7): ("DEPR", "Dew Point Depression", "K"),
    (0, 0, 11): ("LHTFL", "Latent Heat Net Flux", "W/m^2"),
    (0, 0, 12): ("SHTFL", "Sensible Heat Net Flux", "W/m^2"),
    (0, 1, 0): ("SPFH", "Specific Humidity", "kg/kg"),
    (0, 1, 1): ("RH", "Relative Humidity", "%"),
    (0, 1, 2): ("MIXR", "Mixing Ratio", "kg/kg"),
    (0, 1, 3): ("PWAT", "Precipitable Water", "kg/m^2"),
    (0, 1, 4): ("VAPP", "Vapor Pressure", "Pa"),
    (0, 1, 5): ("SATD", "Saturation Deficit", "kg/kg"),
    (0, 2, 0): ("PRES", "Pressure", "Pa"),
    (0, 2, 1): ("MSLP", "Mean Sea Level Pressure", "Pa"),
    (0, 2, 2): ("PMSL", "Pressure reduced to MSL", "Pa"),
    (0, 3, 0): ("UGRD", "u-component of wind", "m/s"),
    (0, 3, 1): ("VGRD", "v-component of wind", "m/s"),
    (0, 3, 2): ("DTRW", "Drift Wind", "m/s"),
    (0, 3, 3): ("STRM", "Streamfunction", "m^2/s"),
    (0, 3, 4): ("VPOT", "Velocity Potential", "m^2/s"),
    (0, 3, 5): ("MNTF", "Montgomery Stream Function", "m^2/s^2"),
    (0, 3, 6): ("SGCVV", "Sigma-Vertical Velocity", "1/s"),
    (0, 6, 1): ("CRAIN", "Rain", "kg/m^2"),
    (0, 6, 2): ("CFRZR", "Freezing Rain", "kg/m^2"),
    (0, 6, 3): ("CICEP", "Ice Pellets", "kg/m^2"),
    (0, 6, 4): ("CSNOW", "Snow", "kg/m^2"),
    (0, 6, 5): ("APCP", "Total Precipitation", "kg/m^2"),
    (0, 6, 6): ("NCPCP", "Large-scale Precipitation", "kg/m^2"),
    (0, 6, 7): ("ACPCP", "Convective Precipitation", "kg/m^2"),
    (0, 6, 8): ("CPOFP", "Prob. of Frozen Precip", "%"),
    (0, 6, 9): ("CRSUP", "Convective Rain", "kg/m^2"),
    (0, 6, 10): ("CSUSF", "Convective Snow", "kg/m^2"),
    (0, 6, 11): ("CWAT", "Cloud Water", "kg/m^2"),
    (0, 6, 12): ("SNOD", "Snow Depth", "m"),
    (0, 6, 13): ("SNOWH", "Snow Depth (water equivalent)", "kg/m^2"),
    (0, 6, 14): ("WEASD", "Water Equivalent of Accumulated Snow Depth", "kg/m^2"),
    (0, 6, 15): ("SNOF", "Snowfall", "kg/m^2"),
    (0, 6, 16): ("SOTYP", "Snow Type", "numeric"),
    (0, 6, 17): ("SRORG", "Snow Age", "s"),
    (0, 6, 18): ("SMREF", "Snow Albedo", "%"),
    (0, 6, 19): ("SMTYP", "Snow temperature categories", "numeric"),
    (0, 6, 20): ("BRNS", "Brunt", "K/day"),
    (0, 6, 21): ("LFTX", "Best (4 layer) Lifted Index", "K"),
}

# Level type codes (Code Table 4.5)
LEVEL_TYPE_TABLE = {
    1: ("surface", "Surface or land surface"),
    2: ("cloud_base", "Cloud base level"),
    3: ("cloud_top", "Cloud top level"),
    4: ("snow_depth", "Level 0 meters below sea level (snow depth)"),
    5: ("atmosphere", "Level above sea level"),
    6: ("level", "Level from ground (altitude)"),
    7: ("altitude", "Isothermal level"),
    8: ("pressure", "Isobaric surface"),
    9: ("sea_level", "Sea level"),
    10: ("fixed_height", "Specified altitude level above ground (height above ground)"),
    11: ("tropopause", "Tropopause"),
    12: ("nominal_tropopause", "Nominal tropopause"),
    13: ("max_wind", "Maximum wind level"),
    14: ("max_temp", "Level of 0°C isotherm"),
    15: ("atmosphere_depth", "Level of adiabatic condensation lifted from the surface"),
    16: ("pressure_surface_ground", "Pressure at ground level"),
    17: ("pressure_above_ground", "Pressure above ground level"),
    18: ("pressure_below_sea", "Pressure below sea level"),
    19: ("sigma", "Sigma level"),
    20: ("depth_below_sea", "Depth below sea level"),
    21: ("isothermal", "Isothermal level"),
    22: ("log_pressure", "Logarithmic pressure level"),
    23: ("potential_temperature", "Potential temperature level"),
    24: ("potential_vorticity", "Potential vorticity surface"),
    25: ("eta", "Eta level"),
    26: ("height_above_sea", "Height above sea level"),
    27: ("mixed_layer", "Mixed layer depth"),
    28: ("boundary_layer", "Top of boundary layer"),
    29: ("sea_bottom", "Sea bottom"),
    30: ("isobaric", "Isobaric surface (Pa) - used with scaled value"),
    31: ("layer", "Layer between two depths"),
    32: ("layer_between_isobaric", "Layer between two isobaric surfaces"),
    33: ("layer_between_sigma", "Layer between two sigma levels"),
    34: ("layer_between_eta", "Layer between two eta levels"),
    35: ("layer_between_heights", "Layer between two height levels above ground"),
    36: ("layer_between_pressure", "Layer between two pressure levels below sea level"),
    37: ("layer_between_heights_sea", "Layer between two height levels above sea level"),
    38: ("altitude_msl", "Altitude from mean sea level"),
    39: ("pressure_from_ground", "Pressure (converted from Pa to hPa) from ground"),
    40: ("pressure_interval", "Pressure interval from ground to level"),
    41: ("height_above_ground", "Height above ground level"),
    42: ("height_interval", "Height interval from ground"),
    43: ("potential_temperature_interval", "Potential temperature interval from ground"),
    44: ("potential_vorticity_interval", "Potential vorticity interval from ground"),
    45: ("height_interval_sea", "Height interval above sea level"),
    46: ("pressure_interval_sea", "Pressure interval from sea level"),
    47: ("pressure_interval_from_msl", "Pressure interval from mean sea level"),
    48: ("sigma_interval", "Sigma interval from ground"),
    49: ("eta_interval", "Eta interval from ground"),
    50: ("depth_interval", "Depth interval below sea level"),
    51: ("layer_between_isobaric_intvl", "Layer between two isobaric interval surfaces"),
    52: ("layer_between_sigma_intvl", "Layer between two sigma interval surfaces"),
    53: ("layer_between_eta_intvl", "Layer between two eta interval surfaces"),
    54: ("layer_between_heights_intvl", "Layer between two height interval surfaces above ground"),
    55: ("layer_between_heights_sea_intvl", "Layer between two height interval surfaces above sea"),
    56: ("layer_between_pressure_intvl", "Layer between two pressure interval surfaces below sea"),
    57: ("layer_between_potential_temp", "Layer between two potential temperature surfaces"),
    58: ("layer_between_potential_vorticity", "Layer between two potential vorticity surfaces"),
    59: ("entire_atmosphere", "Entire atmosphere"),
    60: ("entire_ocean", "Entire ocean"),
    100: ("isobaric_high_prec", "High precision isobaric surface"),
    101: ("height_above_ground_high", "High precision height above ground"),
    102: ("height_above_sea_high", "High precision height above sea level"),
    103: ("surface_below_sea", "Surface below land/sea"),
    104: ("isobaric_layer_high", "High precision isobaric layer"),
    105: ("height_layer_above_ground_high", "High precision height layer above ground"),
    106: ("height_layer_above_sea_high", "High precision height layer above sea"),
    107: ("pressure_high", "High precision pressure from ground"),
    108: ("depth_below_land", "Depth below land surface"),
    109: ("layer_depths_below_land", "Layer between two depths below land surface"),
    110: ("layer_depth_intervals_below_land", "Layer between two depth intervals below land"),
    111: ("highest_tropospheric_freezing", "Highest tropospheric freezing level"),
    112: ("boundary_layer_top", "Boundary layer top"),
    113: ("layer_between_isobaric_high", "Layer between two isobaric surfaces (high precision)"),
    192: ("low_cloud_bottom", "Low cloud bottom"),
    193: ("low_cloud_top", "Low cloud top"),
    194: ("cloud_bottom", "Cloud bottom"),
    195: ("cloud_top", "Cloud top"),
    196: ("cloud_ceiling", "Cloud ceiling"),
    197: ("high_cloud_bottom", "High cloud bottom"),
    198: ("high_cloud_top", "High cloud top"),
    199: ("lowest_cloud_top", "Lowest cloud top"),
    200: ("ocean_isobaric", "Ocean isobaric surface"),
    201: ("ocean_depth", "Depth below ocean surface"),
    204: ("height_above_ground_agl", "Height above ground level (AGL)"),
    206: ("general_vertical", "General vertical level"),
    232: ("entire_atmosphere_layer", "Entire atmosphere layer"),
    233: ("entire_ocean_layer", "Entire ocean layer"),
    234: ("composite_layer", "Composite layer"),
    235: ("ocean_surface_layer", "Ocean surface layer"),
    236: ("bottom_ocean_layer", "Bottom ocean layer"),
    237: ("ocean_temperature_layer", "Ocean temperature layer"),
    238: ("ocean_salinity_layer", "Ocean salinity layer"),
    255: ("missing", "Missing")
}

# Time range indicator codes (Code Table 4.10)
TIME_RANGE_INDICATOR = {
    0: "Forecast",
    1: "Analysis",
    2: "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time",
    3: "Average",
    4: "Accumulation",
    5: "Difference",
    6: "Accumulation (from previous time)",
    7: "Average (from previous time)",
    8: "Maximum",
    9: "Minimum",
    10: "Difference from previous time",
    11: "Standard deviation of values over time range",
    12: "Covariance of values over time range",
    13: "Correlation coefficient of values over time range",
    14: "Sum over time range",
    15: "Spread of ensemble over time range",
    16: "Normalized spread of ensemble over time range",
    19: "Statistical interpolation",
    20: "Statistical interpolation - normalized values",
    21: "Vector mean resultant over time range",
    22: "Vector mean resultant - normalized values over time range",
    23: "Standard deviation - normalized values over time range",
    24: "Principal component analysis time series",
    25: "Empirical orthogonal function time series",
    26: "Average over a restricted time range",
    27: "Accumulation over a restricted time range",
    28: "Difference from a previous time within a time range",
    29: "Standard deviation - normalized values - restricted time range",
    30: "Vector mean resultant - restricted time range",
    31: "Vector mean resultant - normalized values - restricted time range",
    32: "Vector mean resultant over a time range - normalized values",
    33: "Difference of Gaussian filter time series",
    34: "Sum over a time range",
    51: "Root mean square of values over time range",
    254: "Trend",
    255: "Missing"
}


# ============================================================================
# Helper functions for parameter and level lookup
# ============================================================================

def get_parameter_info(discipline, category, number):
    """Get parameter name, description, and units from lookup tables.

    Args:
        discipline: GRIB2 discipline code (0=meteorological, 1=hydrological, etc.)
        category: Parameter category code
        number: Parameter number code

    Returns:
        Dictionary with short_name, long_name, and units
    """
    # Try NCEP operational parameter table first
    key = (discipline, category, number)
    if key in PARAM_TABLE_NCEP:
        short_name, long_name, units = PARAM_TABLE_NCEP[key]
        return {
            'short_name': short_name,
            'long_name': long_name,
            'units': units,
            'source': 'NCEP_operational'
        }

    # Fallback: use category name for description
    category_name = "Unknown"
    if discipline == 0 and category in PARAM_CATEGORY_MET:
        category_name = PARAM_CATEGORY_MET[category]
    elif discipline in DISCIPLINE_TABLE:
        category_name = f"{DISCIPLINE_TABLE[discipline]} category {category}"

    return {
        'short_name': f"PARAM-{discipline}-{category}-{number}",
        'long_name': f"{category_name} parameter {number}",
        'units': "Unknown",
        'source': 'fallback'
    }


def get_level_type_info(level_type):
    """Get level type name and description from lookup table.

    Args:
        level_type: GRIB2 level type code (Code Table 4.5)

    Returns:
        Dictionary with short_name and description
    """
    if level_type in LEVEL_TYPE_TABLE:
        short_name, description = LEVEL_TYPE_TABLE[level_type]
        return {
            'short_name': short_name,
            'description': description,
            'code': level_type
        }

    return {
        'short_name': f"level_{level_type}",
        'description': f"Unknown level type {level_type}",
        'code': level_type
    }


def format_reference_time(year, month, day, hour, minute, second, significance):
    """Format reference time into ISO 8601 string and significance info.

    Args:
        year, month, day, hour, minute, second: Time components
        significance: Significance of reference time (Code Table 1.2)

    Returns:
        Dictionary with iso_string, significance_name, and components
    """
    # Validate and clamp time components
    year = max(0, min(9999, year))
    month = max(1, min(12, month))
    day = max(1, min(31, day))
    hour = max(0, min(23, hour))
    minute = max(0, min(59, minute))
    second = max(0, min(59, second))

    try:
        iso_string = datetime(year, month, day, hour, minute, second, tzinfo=timezone.utc).isoformat()
    except ValueError:
        iso_string = f"{year:04d}-{month:02d}-{day:02d}T{hour:02d}:{minute:02d}:{second:02d}Z"

    # Significance of reference time (Code Table 1.2)
    significance_names = {
        0: "Start of forecast",
        1: "Start of analysis",
        2: "Time of observation",
        3: "Start of assimilation",
        4: "Verification time",
        255: "Missing"
    }

    return {
        'iso_string': iso_string,
        'significance': significance,
        'significance_name': significance_names.get(significance, f"Unknown({significance})"),
        'components': {
            'year': year,
            'month': month,
            'day': day,
            'hour': hour,
            'minute': minute,
            'second': second
        }
    }


def parse_scaled_value(scale_factor, scaled_value):
    """Parse scaled value from GRIB2 encoding.

    Args:
        scale_factor: Scale factor (negative = decimal shift, positive = multiply)
        scaled_value: Scaled value

    Returns:
        Actual value (float)
    """
    if scale_factor == 0:
        return float(scaled_value)

    # Handle signed scaled_value (can be negative)
    # In GRIB2, scaled_value is stored as signed int
    actual_value = float(scaled_value) * (10.0 ** (-scale_factor))
    return actual_value


# ============================================================================
# Basic GRIB2 parsing implementation
# ============================================================================

class GRIB2Parser:
    """Basic GRIB2 parser that extracts metadata without external dependencies."""

    def __init__(self, file_path):
        self.file_path = Path(file_path)
        self.messages = []

    def parse(self):
        """Parse all GRIB2 messages in the file.

        Raises:
            FileNotFoundError: If the GRIB2 file does not exist
            ValueError: If the file is not a valid GRIB2 file
        """
        if not self.file_path.exists():
            raise FileNotFoundError(f"GRIB2 file not found: {self.file_path}")

        if not self.file_path.is_file():
            raise ValueError(f"Path is not a file: {self.file_path}")

        with open(self.file_path, 'rb') as f:
            data = f.read()

        if len(data) < 16:
            raise ValueError(f"File too small to be a valid GRIB2 file: {self.file_path}")

        offset = 0
        message_count = 0

        while offset < len(data):
            result = self._parse_message(data, offset)
            if result is None:
                break

            message, next_offset = result
            if message:
                self.messages.append(message)
                message_count += 1

            offset = next_offset
            if offset <= 0 or offset >= len(data):
                break

        return self.messages

    def _parse_message(self, data, offset):
        """Parse a single GRIB2 message starting at offset."""
        if offset + 16 > len(data):
            return None

        # Check for GRIB magic
        magic = data[offset:offset+4]
        if magic != b'GRIB':
            return None

        # Section 0: Indicator Section (IS)
        # Bytes 0-3: "GRIB" magic
        # Bytes 4-5: reserved (always 0)
        # Byte 6: discipline
        # Byte 7: edition (2 for GRIB2)
        # Bytes 8-15: total length of message (8 bytes in GRIB2 spec)
        try:
            discipline = data[offset+6]
            edition = data[offset+7]

            # GRIB2 Section 0 structure:
            # Bytes 0-3: "GRIB" magic
            # Bytes 4-5: Reserved (00 00)
            # Byte 6: Discipline
            # Byte 7: Edition (2 for GRIB2)
            # Bytes 8-15: Total length of message (8-byte unsigned long)
            section0_length = struct.unpack('>Q', data[offset+8:offset+16])[0]

            if edition != 2:
                return None  # Only GRIB2 supported

            # Parse sections
            sections = {}
            current_offset = offset + 16  # Start after Section 0 (which is 16 bytes)

            while current_offset < offset + section0_length:
                # Check for Section 8 (End Section) - special case, only 4 bytes "7777"
                if current_offset + 4 <= len(data) and data[current_offset:current_offset+4] == b'7777':
                    sections[8] = {'length': 4, 'data': b'7777'}
                    break

                if current_offset + 5 > len(data):
                    break

                # Each section starts with: length (4 octets) + section number (1 octet)
                section_length = struct.unpack('>I', data[current_offset:current_offset+4])[0]
                section_number = data[current_offset+4]

                if section_length == 0 or section_length > 10000000:  # Sanity check
                    break

                # Extract section data
                section_start = current_offset
                section_end = current_offset + section_length

                if section_end > len(data):
                    break

                section_data = data[current_offset:section_end]

                # Store section
                sections[section_number] = {
                    'length': section_length,
                    'data': section_data
                }

                current_offset = section_end

            # Extract metadata from sections
            if not sections:
                return None

            # Extract GDT and PDT template numbers first (needed for _extract_metadata)
            gdt_template = 0
            if 3 in sections and len(sections[3]['data']) >= 7:
                section3 = sections[3]['data']
                gdt_template = struct.unpack('>H', section3[5:7])[0]

            pdt_template = 0
            if 4 in sections and len(sections[4]['data']) >= 7:
                section4 = sections[4]['data']
                pdt_template = struct.unpack('>H', section4[5:7])[0]

            message = self._extract_metadata(sections, discipline, gdt_template, pdt_template)
            next_offset = offset + section0_length

            return message, next_offset

        except (struct.error, ValueError):
            return None

    def _extract_metadata(self, sections, discipline, gdt_template, pdt_template):
        """Extract metadata from parsed sections."""
        if 1 not in sections or 3 not in sections or 4 not in sections:
            return None

        section1 = sections[1]['data']
        section3 = sections[3]['data']
        section4 = sections[4]['data']

        # Section 1: Identification Section
        # Octets 1-4: length, Octet 5: section number
        # Octets 6-7: center
        # Octets 8-9: subcenter
        # Octets 10-11: master tables version
        # Octets 12-13: local tables version
        # Octet 14: significance of reference time
        # Octets 15-16: year
        # Octet 17: month
        # Octet 18: day
        # Octet 19: hour
        # Octet 20: minute
        # Octet 21: second
        # (Data includes 5-byte header, so octet N is at index N-1)
        if len(section1) < 21:
            return None

        center = struct.unpack('>H', section1[5:7])[0]
        subcenter = struct.unpack('>H', section1[7:9])[0]

        significance = section1[13]    # Octet 14 → index 13
        year = struct.unpack('>H', section1[14:16])[0]    # Octets 15-16 → indices 14-15
        month = section1[16]         # Octet 17 → index 16
        day = section1[17]            # Octet 18 → index 17
        hour = section1[18]           # Octet 19 → index 18
        minute = section1[19]         # Octet 20 → index 19
        second = section1[20] if len(section1) > 20 else 0  # Octet 21 → index 20

        # Section 3: Grid Definition Section
        if len(section3) < 7:
            return None

        gdt_template = struct.unpack('>H', section3[5:7])[0]

        # Extract grid metadata with error handling
        try:
            grid_info = self._extract_grid_metadata(section3, gdt_template)
        except (struct.error, ValueError, IndexError) as e:
            # If grid extraction fails, still return the message with basic grid info
            grid_info = {
                'template': gdt_template,
                'num_data_points': None,
                'nx': None,
                'ny': None,
                'lat_first': None,
                'lon_first': None,
                'lat_last': None,
                'lon_last': None,
                'di': None,
                'dj': None,
                'scanning_mode': 0,
                'resolution_flags': 48,
                'shape_of_earth': 6,
                'extraction_error': str(e)
            }

        # Section 4: Product Definition Section
        if len(section4) < 15:
            return None

        pdt_template = struct.unpack('>H', section4[5:7])[0]
        num_coords = section4[7]

        # For template 4.0 (analysis/forecast):
        # Octet 10 (index 9): parameter category
        # Octet 11 (index 10): parameter number
        param_category = section4[9]
        param_number = section4[10]

        # Get parameter information from lookup tables
        param_info = get_parameter_info(discipline, param_category, param_number)

        # Extract level information from Section 4
        level_info = self._extract_level_info(section4, pdt_template)

        # Extract forecast/time information
        forecast_info = self._extract_forecast_info(section4, pdt_template, year, month, day, hour, minute, second, significance)

        # Extract ensemble information if PDT 4.1
        ensemble_info = None
        if pdt_template == 1:
            ensemble_info = self._extract_ensemble_info(section4)

        # Extract Section 5 (Data Representation Section) for pack info
        drt_template = None
        packing_info = None
        if 5 in sections:
            section5 = sections[5]['data']
            drt_template, packing_info = self._extract_drt_info(section5)

        # Extract Section 6 (Bitmap Section) if present
        bitmap_info = None
        if 6 in sections:
            bitmap_info = self._extract_bitmap_info(sections[6]['data'])

        # Extract Section 7 (Data Section) for actual data values
        values_info = None
        if 7 in sections and packing_info:
            values_info = self._extract_data_values(
                sections[7]['data'],
                packing_info,
                bitmap_info,
                grid_info.get('num_data_points')
            )

        # Build basic message structure in golden format
        message = {
            'center': center,
            'subcenter': subcenter,
            'parameter': {
                'discipline': discipline,
                'category': param_category,
                'number': param_number
            },
            'forecast': forecast_info,
            'level': level_info,
            'ensemble': ensemble_info,
            'grid': grid_info,
            'gdt_template': gdt_template,
            'pdt_template': pdt_template,
            'drt_template': drt_template,
            'packing': packing_info,
            'values': values_info
        }

        return message

    def _extract_level_info(self, section4, pdt_template):
        """Extract level information from Section 4 in golden format.

        Section 4 structure (with 5-byte header):
        - Octets 1-4 (indices 0-3): section length
        - Octet 5 (index 4): section number
        - Octets 6-7 (indices 5-6): number of coordinate values
        - Octets 8-9 (indices 7-8): product definition template number
        - Octet 10 (index 9): parameter category
        - Octet 11 (index 10): parameter number
        - Octet 12 (index 11): generating process
        - Octet 13 (index 12): background process id
        - Octet 14 (index 13): forecast generating process id
        - Octets 15-16 (indices 14-15): hours after reference time for cutoff
        - Octet 17 (index 16): minutes after cutoff time
        - Octet 18 (index 17): indicator of unit of time range
        - Octets 19-22 (indices 18-21): forecast time in units

        For PDT 4.0 (template 0):
        - Octets 23-30 (indices 22-29): first fixed surface
        - Octet 23 (index 22): First level type (type1)
        - Octets 24-25 (indices 23-24): Scale factor for first level (scale_factor1)
        - Octets 26-29 (indices 25-28): Scaled value for first level (scaled_value1)
        - Octets 31-38 (indices 30-37): second fixed surface
        - Octet 31 (index 30): Second level type (type2)
        - Octets 32-33 (indices 31-32): Scale factor for second level (scale_factor2)
        - Octets 34-37 (indices 33-36): Scaled value for second level (scaled_value2)

        Returns:
            Dictionary in golden format: {type1, scale_factor1, scaled_value1, type2, scale_factor2, scaled_value2}
        """
        level_info = {
            'type1': 255,
            'scale_factor1': 0,
            'scaled_value1': 0,
            'type2': 255,
            'scale_factor2': 0,
            'scaled_value2': 0
        }

        try:
            if pdt_template == 0:  # PDT 4.0
                if len(section4) >= 30:
                    # First fixed surface
                    level_info['type1'] = section4[22]  # Octet 23 → index 22
                    level_info['scale_factor1'] = struct.unpack('>h', section4[23:25])[0]  # Octets 24-25 → indices 23-24
                    level_info['scaled_value1'] = struct.unpack('>i', section4[25:29])[0]  # Octets 26-29 → indices 25-28

                    # Second fixed surface (if present)
                    if len(section4) >= 38:
                        level_info['type2'] = section4[30]  # Octet 31 → index 30
                        level_info['scale_factor2'] = struct.unpack('>h', section4[31:33])[0]  # Octets 32-33 → indices 31-32
                        level_info['scaled_value2'] = struct.unpack('>i', section4[33:37])[0]  # Octets 34-37 → indices 33-36

            elif pdt_template == 1:  # PDT 4.1 (ensemble)
                if len(section4) >= 30:
                    # Same structure as PDT 4.0 for level info
                    level_info['type1'] = section4[22]  # Octet 23 → index 22
                    level_info['scale_factor1'] = struct.unpack('>h', section4[23:25])[0]  # Octets 24-25 → indices 23-24
                    level_info['scaled_value1'] = struct.unpack('>i', section4[25:29])[0]  # Octets 26-29 → indices 25-28

                    if len(section4) >= 38:
                        level_info['type2'] = section4[30]  # Octet 31 → index 30
                        level_info['scale_factor2'] = struct.unpack('>h', section4[31:33])[0]  # Octets 32-33 → indices 31-32
                        level_info['scaled_value2'] = struct.unpack('>i', section4[33:37])[0]  # Octets 34-37 → indices 33-36

            else:
                # For other PDT templates, try to extract basic level info (same offsets)
                if len(section4) >= 30:
                    level_info['type1'] = section4[22]  # Octet 23 → index 22
                    level_info['scale_factor1'] = struct.unpack('>h', section4[23:25])[0]  # Octets 24-25 → indices 23-24
                    level_info['scaled_value1'] = struct.unpack('>i', section4[25:29])[0]  # Octets 26-29 → indices 25-28

        except (struct.error, ValueError, IndexError) as e:
            level_info['extraction_error'] = str(e)

        return level_info

    def _extract_forecast_info(self, section4, pdt_template, year, month, day, hour, minute, second, significance):
        """Extract forecast/time information from Section 4 in golden format.

        For PDT 4.0 (template 0):
        - Octet 18 (index 17): Indicator of unit for time range
        - Octets 19-22 (indices 18-21): Forecast time in units

        Returns:
            Dictionary in golden format: {reference_time: {...}, time_range_unit, forecast_offset}
        """
        forecast_info = {}
        forecast_offset = 0
        time_range_unit = 1  # Default: hours (Code Table 4.4)

        try:
            # Reference time as flat object
            forecast_info['reference_time'] = {
                'year': year,
                'month': month,
                'day': day,
                'hour': hour,
                'minute': minute,
                'second': second,
                'significance': significance
            }

            # Extract forecast offset and time unit
            # For PDT 4.0/4.1, after 5-byte header:
            # Octet 18: Indicator of unit for time range → index 17
            # Octets 19-22: Forecast time in units → indices 18-21
            if pdt_template in (0, 1) and len(section4) >= 23:
                # Octet 18: Indicator of unit for time range → index 17
                time_range_unit = section4[17]
                # Octets 19-22: Forecast time in units → indices 18-21
                forecast_offset = struct.unpack('>I', section4[18:22])[0]

            forecast_info['time_range_unit'] = time_range_unit
            forecast_info['forecast_offset'] = forecast_offset

        except (struct.error, ValueError, IndexError) as e:
            forecast_info['extraction_error'] = str(e)

        return forecast_info

    def _extract_grid_metadata(self, section3, gdt_template):
        """Extract grid metadata from Grid Definition Section (Section 3) in golden format.

        Args:
            section3: Raw bytes of Section 3 (including 5-byte header)
            gdt_template: Grid Definition Template number

        Returns:
            Dictionary in golden format containing grid metadata
        """
        if len(section3) < 7:
            raise ValueError(f"Section 3 too short: {len(section3)} bytes")

        # Initialize grid structure in golden format
        grid_info = {
            'template': gdt_template,
            'num_data_points': None,
            'nx': None,
            'ny': None,
            'lat_first': None,
            'lon_first': None,
            'lat_last': None,
            'lon_last': None,
            'di': None,
            'dj': None,
            'scanning_mode': 0,
            'resolution_flags': 48,
            'shape_of_earth': 6
        }

        # Section 3 structure (after 5-byte header):
        # Octets 1-4: source (type of grid)
        # Octets 5-8: number of data points
        # Octet 9: number of octets for optional list
        # Octet 10: interpretation of optional list
        # Octets 11-12: template number

        # Extract basic fields from Section 3 (before template)
        try:
            # Octets 5-8: number of data points (indices 4-7 in section data)
            if len(section3) >= 9:
                grid_info['num_data_points'] = struct.unpack('>I', section3[4:8])[0]
        except (struct.error, ValueError, IndexError):
            pass

        # Template 3.0: Latitude/Longitude (or Equidistant Cylindrical)
        if gdt_template == 0:
            grid_info.update(self._extract_grid_template_0(section3))
        elif gdt_template == 1:
            grid_info.update(self._extract_grid_template_1(section3))
        elif gdt_template == 10:
            grid_info.update(self._extract_grid_template_10(section3))
        else:
            grid_info['extraction_warning'] = f'Grid template {gdt_template} not fully supported'

        return grid_info

    def _extract_grid_template_0(self, section3):
        """Extract grid metadata for Template 3.0 (Latitude/Longitude) in golden format.

        Section 3 structure (with 5-byte header):
        - Octets 1-4 (indices 0-3): source
        - Octets 5-8 (indices 4-7): number of data points
        - Octet 9 (index 8): number of octets for optional list
        - Octet 10 (index 9): interpretation of optional list
        - Octets 11-12 (indices 10-11): template number

        Template 3.0 structure (starts at index 12):
        - Octet 1 (index 12): shape of the earth
        - Octets 2-3 (indices 13-14): scale factor of radius of spherical earth
        - Octets 4-7 (indices 15-18): scaled value of radius of spherical earth
        - Octets 8-9 (indices 19-20): scale factor of major axis of oblate spheroid earth
        - Octets 10-13 (indices 21-24): scaled value of major axis of oblate spheroid earth
        - Octets 14-15 (indices 25-26): scale factor of minor axis of oblate spheroid earth
        - Octets 16-19 (indices 27-30): scaled value of minor axis of oblate spheroid earth
        - Octets 20-23 (indices 31-34): Ni (number of points along latitude)
        - Octets 24-27 (indices 35-38): Nj (number of points along longitude)
        - Octets 28-31 (indices 39-42): basic angle of the initial production domain
        - Octets 32-35 (indices 43-46): subdivisions of basic angle
        - Octets 36-39 (indices 47-50): latitude of first grid point (scaled by 10^-6)
        - Octets 40-43 (indices 51-54): longitude of first grid point (scaled by 10^-6)
        - Octet 44 (index 55): resolution and component flags
        - Octets 45-48 (indices 56-59): latitude of last grid point (scaled by 10^-6)
        - Octets 49-52 (indices 60-63): longitude of last grid point (scaled by 10^-6)
        - Octets 53-56 (indices 64-67): i direction increment (scaled by 10^-6)
        - Octets 57-60 (indices 68-71): j direction increment (scaled by 10^-6)
        - Octet 61 (index 72): scanning mode

        Returns:
            Dictionary in golden format with grid metadata for template 0
        """
        if len(section3) < 73:
            raise ValueError(f"Section 3 too short for template 0: {len(section3)} bytes, need at least 73")

        info = {}

        # Shape of earth (octet 1 of template, index 12)
        info['shape_of_earth'] = section3[12]

        # Nx and Ny (number of grid points) - 4-byte unsigned integers
        info['nx'] = struct.unpack('>I', section3[31:35])[0]
        info['ny'] = struct.unpack('>I', section3[35:39])[0]
        info['num_data_points'] = info['nx'] * info['ny']

        # Latitude/longitude bounds (scaled by 10^-6) - 4-byte signed integers
        info['lat_first'] = struct.unpack('>i', section3[47:51])[0] / 1000000.0
        info['lon_first'] = struct.unpack('>i', section3[51:55])[0] / 1000000.0

        # Resolution and component flags (octet 44, index 55)
        info['resolution_flags'] = section3[55]

        # Latitude/longitude of last point (scaled by 10^-6) - 4-byte signed integers
        info['lat_last'] = struct.unpack('>i', section3[56:60])[0] / 1000000.0
        info['lon_last'] = struct.unpack('>i', section3[60:64])[0] / 1000000.0

        # Grid increments (scaled by 10^-6) - 4-byte signed integers
        info['di'] = struct.unpack('>i', section3[64:68])[0] / 1000000.0
        info['dj'] = struct.unpack('>i', section3[68:72])[0] / 1000000.0

        # Scanning mode (octet 61, index 72) - raw byte value
        info['scanning_mode'] = section3[72]

        return info

    def _extract_grid_template_1(self, section3):
        """Extract grid metadata for Template 3.1 (Rotated Latitude/Longitude)."""
        info = {}
        if len(section3) >= 39:
            nx = struct.unpack('>I', section3[31:35])[0]
            ny = struct.unpack('>I', section3[35:39])[0]
            info['nx'] = nx
            info['ny'] = ny
            info['num_data_points'] = nx * ny
            info['extraction_note'] = 'Rotated grid - full extraction not implemented'
        return info

    def _extract_grid_template_10(self, section3):
        """Extract grid metadata for Template 3.10 (Mercator)."""
        info = {}
        if len(section3) >= 39:
            nx = struct.unpack('>I', section3[31:35])[0]
            ny = struct.unpack('>I', section3[35:39])[0]
            info['nx'] = nx
            info['ny'] = ny
            info['num_data_points'] = nx * ny
            info['extraction_note'] = 'Mercator grid - full extraction not implemented'
        return info

    def _extract_ensemble_info(self, section4):
        """Extract ensemble information from Section 4 for PDT 4.1.

        For PDT 4.1 (ensemble forecast):
        - Octet 27 (index 26): Type of ensemble forecast
        - Octet 28 (index 27): Perturbation number

        Returns:
            Dictionary in golden format: {member_type, number}
        """
        ensemble_info = None

        try:
            if len(section4) >= 28:
                member_type = section4[26]
                number = section4[27]

                ensemble_info = {
                    'member_type': member_type,
                    'number': number
                }
        except (struct.error, ValueError, IndexError) as e:
            ensemble_info = {'extraction_error': str(e)}

        return ensemble_info

    def _extract_drt_info(self, section5):
        """Extract Data Representation Template information from Section 5.

        For DRT 5.0 (grid point data - simple packing):
        - Octets 6-7: Template number
        - Octets 8-15: Reference value (IEEE 64-bit floating point)
        - Octet 16: Binary scale factor (signed)
        - Octet 17: Decimal scale factor (signed)
        - Octet 18: Number of bits per value
        - Octet 19: Type of original field values

        Returns:
            Tuple of (drt_template, packing_info) in golden format
        """
        drt_template = None
        packing_info = None

        try:
            if len(section5) >= 19:
                # Template number (octets 6-7, indices 5-6)
                drt_template = struct.unpack('>H', section5[5:7])[0]

                # Reference value (octets 8-15, indices 7-14) - IEEE 64-bit float
                reference_value = struct.unpack('>d', section5[7:15])[0]

                # Binary scale factor (octet 16, index 15) - signed
                binary_scale_factor = struct.unpack('>b', bytes([section5[15]]))[0]

                # Decimal scale factor (octet 17, index 16) - signed
                decimal_scale_factor = struct.unpack('>b', bytes([section5[16]]))[0]

                # Bits per value (octet 18, index 17)
                bits_per_value = section5[17]

                # Type of original field values (octet 19, index 18)
                original_field_type = section5[18]

                packing_info = {
                    'reference_value': float(reference_value),
                    'binary_scale_factor': binary_scale_factor,
                    'decimal_scale_factor': decimal_scale_factor,
                    'bits_per_value': bits_per_value,
                    'original_field_type': original_field_type
                }
            else:
                packing_info = {
                    'extraction_error': f'Section 5 too short: {len(section5)} bytes'
                }
        except (struct.error, ValueError, IndexError) as e:
            packing_info = {'extraction_error': str(e)}

        return drt_template, packing_info

    def _extract_bitmap_info(self, section6):
        """Extract bitmap information from Section 6.

        Section 6 structure:
        - Octets 1-4: Section length
        - Octet 5: Section number (6)
        - Octet 6: Bit indicator (0=bitmap applies, 1=no bitmap)
        - Octets 7+: Bitmap data

        Returns:
            Dictionary with bitmap list if present, None otherwise
        """
        try:
            if len(section6) < 6:
                return None

            # Bit indicator (octet 6, index 5)
            bit_indicator = section6[5]

            # If bit indicator is 1, there's no bitmap
            if bit_indicator == 1:
                return None

            # Extract bitmap bits from octets 7+
            bitmap_bits = section6[6:] if len(section6) > 6 else b''

            bitmap = []
            for byte in bitmap_bits:
                # Process each bit in the byte (MSB first)
                for bit_idx in range(7, -1, -1):
                    bitmap.append(bool((byte >> bit_idx) & 1))

            return bitmap

        except (struct.error, ValueError, IndexError) as e:
            return {'extraction_error': str(e)}

    def _extract_data_values(self, section7, packing_info, bitmap_info, num_data_points):
        """Extract and decode data values from Section 7.

        Section 7 structure:
        - Octets 1-4: Section length
        - Octet 5: Section number (7)
        - Octets 6+: Packed data values

        For DRT 5.0 (simple packing):
        - Values are stored as packed integers with bits_per_value each
        - Actual value = reference_value + (packed_value * 2^binary_scale_factor) / 10^decimal_scale_factor

        Args:
            section7: Raw bytes of Section 7
            packing_info: Packing information from Section 5
            bitmap_info: Bitmap information from Section 6 (if present)
            num_data_points: Number of data points in grid

        Returns:
            Dictionary in golden format: {"Dense": [float values]} or {"Masked": {...}}
        """
        try:
            if packing_info is None:
                return None

            bits_per_value = packing_info.get('bits_per_value', 0)
            reference_value = packing_info.get('reference_value', 0.0)
            binary_scale_factor = packing_info.get('binary_scale_factor', 0)
            decimal_scale_factor = packing_info.get('decimal_scale_factor', 0)

            if bits_per_value == 0 or bits_per_value > 32:
                return None

            # Get raw data bytes (skip 5-byte header)
            data_bytes = section7[5:] if len(section7) > 5 else b''

            if not data_bytes:
                return None

            # Decode packed values
            packed_values = self._decode_packed_values(data_bytes, bits_per_value, num_data_points)

            if not packed_values:
                return None

            # Convert packed values to actual values
            scale_factor = 2.0 ** binary_scale_factor
            decimal_divisor = 10.0 ** decimal_scale_factor

            actual_values = []
            for packed_val in packed_values:
                # Apply scaling: actual = ref + (packed * 2^binary) / 10^decimal
                actual_val = reference_value + (packed_val * scale_factor) / decimal_divisor
                actual_values.append(float(actual_val))

            # Return in golden format
            if bitmap_info:
                return {
                    'Masked': {
                        'values': actual_values,
                        'present': bitmap_info
                    }
                }
            else:
                return {'Dense': actual_values}

        except (struct.error, ValueError, IndexError) as e:
            return {'extraction_error': str(e)}

    def _decode_packed_values(self, data_bytes, bits_per_value, num_points):
        """Decode packed integer values from raw data bytes.

        Args:
            data_bytes: Raw packed data bytes
            bits_per_value: Number of bits per value
            num_points: Expected number of data points

        Returns:
            List of decoded integer values
        """
        if bits_per_value == 0 or num_points is None or num_points == 0:
            return None

        # Calculate total bits needed
        total_bits = bits_per_value * num_points
        total_bytes = (total_bits + 7) // 8

        if len(data_bytes) < total_bytes:
            return None

        # Convert bytes to bit stream
        bit_stream = []
        for byte in data_bytes:
            for bit_idx in range(7, -1, -1):
                bit_stream.append((byte >> bit_idx) & 1)

        # Extract packed values
        packed_values = []
        for i in range(num_points):
            value = 0
            start_bit = i * bits_per_value

            # Extract bits_per_value bits
            for j in range(bits_per_value):
                if start_bit + j < len(bit_stream):
                    value = (value << 1) | bit_stream[start_bit + j]

            # Handle signed values (two's complement for bits_per_value < 32)
            if bits_per_value < 32 and (value & (1 << (bits_per_value - 1))):
                value -= (1 << bits_per_value)

            packed_values.append(value)

        return packed_values


def gen_golden_basic(grib2_path, fixture_id, output_dir):
    """Generate golden JSON using basic GRIB2 parsing.

    Args:
        grib2_path: Path to input GRIB2 file
        fixture_id: Fixture ID for output filename
        output_dir: Directory for output JSON

    Raises:
        SystemExit: On file access or parsing errors
    """
    try:
        parser = GRIB2Parser(grib2_path)
        messages = parser.parse()
    except FileNotFoundError as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)
    except ValueError as e:
        print(f"ERROR: Invalid GRIB2 file - {e}", file=sys.stderr)
        sys.exit(1)
    except OSError as e:
        print(f"ERROR: Cannot read file {grib2_path}: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"ERROR: Unexpected error parsing {grib2_path}: {e}", file=sys.stderr)
        sys.exit(1)

    if not messages:
        print(f"WARNING: no GRIB2 messages decoded from {grib2_path}", file=sys.stderr)
        sys.exit(1)

    golden = {
        'fixture_id': fixture_id,
        '_provenance': (
            f'Generated by scripts/gen_golden.py from {Path(grib2_path).name}'
            ' using basic GRIB2 parsing (no external dependencies).'
        ),
        'fields': messages,
        'parser_version': 'basic_0.2'
    }

    out = Path(output_dir) / f'{fixture_id}.json'
    out.parent.mkdir(parents=True, exist_ok=True)

    with open(out, 'w') as fh:
        json.dump(golden, fh, indent=4)

    print(f'Written: {out}  ({len(messages)} message(s))')


# ============================================================================
# Main entry point
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description='Generate gribtract golden reference JSON from a GRIB2 file using basic parsing'
    )
    parser.add_argument('grib2_file', help='Input GRIB2 file')
    parser.add_argument('fixture_id', help='Fixture ID (becomes the output filename)')
    parser.add_argument(
        '--output-dir',
        default='tests/corpus/golden',
        help='Directory for the output JSON (default: tests/corpus/golden)',
    )
    args = parser.parse_args()

    gen_golden_basic(args.grib2_file, args.fixture_id, args.output_dir)


if __name__ == '__main__':
    main()
