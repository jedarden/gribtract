#!/usr/bin/env python3
"""gen_golden.py — Generate gribtract golden reference files using basic GRIB2 parsing.

This script implements basic GRIB2 file parsing from scratch without external dependencies.
It extracts key metadata from GRIB2 messages and outputs JSON structure.

Usage:
    python3 scripts/gen_golden.py <grib2_file> <fixture_id> [--output-dir DIR]

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
# Basic GRIB2 parsing implementation
# ============================================================================

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
        # Bytes 8-11: total length of message
        try:
            discipline = data[offset+6]
            edition = data[offset+7]

            # GRIB2 Section 0 structure:
            # Bytes 0-3: "GRIB" magic
            # Bytes 4-5: Reserved (00 00)
            # Byte 6: Discipline
            # Byte 7: Edition (2 for GRIB2)
            # Bytes 8-11: Additional fields/reserved
            # Bytes 12-15: Total length of message
            section0_length = struct.unpack('>I', data[offset+12:offset+16])[0]

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

            message = self._extract_metadata(sections, discipline)
            next_offset = offset + section0_length

            return message, next_offset

        except (struct.error, ValueError):
            return None

    def _extract_metadata(self, sections, discipline):
        """Extract metadata from parsed sections."""
        if 1 not in sections or 3 not in sections or 4 not in sections:
            return None

        section1 = sections[1]['data']
        section3 = sections[3]['data']
        section4 = sections[4]['data']

        # Section 1: Identification Section
        # Octets: 1-4 (length), 5 (section number), 6-7 (center),
        #         8-9 (subcenter), 10-11 (master tables version), 12-13 (local tables version),
        #         14 (significance of reference time), 15-16 (year), 17 (month), 18 (day),
        #         19 (hour), 20 (minute), 21 (second)
        # Minimum Section 1 length is 21 bytes (including 5-byte header)
        if len(section1) < 21:
            return None

        # Note: section data includes the 5-byte header (length + section number)
        # So octet N in the spec is at index N-1 in the byte array
        center = struct.unpack('>H', section1[5:7])[0]
        subcenter = struct.unpack('>H', section1[7:9])[0]

        # Reference time fields (offsets are 1-indexed octet numbers, convert to 0-indexed)
        significance = section1[13]
        year = struct.unpack('>H', section1[14:16])[0]
        month = section1[16]
        day = section1[17]
        hour = section1[18]
        minute = section1[19]
        second = section1[20] if len(section1) > 20 else 0

        # Section 3: Grid Definition Section
        # Octets: 1-4 (length), 5 (section number), 6-7 (template number)
        if len(section3) < 7:
            return None

        gdt_template = struct.unpack('>H', section3[5:7])[0]

        # Section 4: Product Definition Section
        # Octets: 1-4 (length), 5 (section number), 6-7 (template number),
        #         8 (number of coordinates), then parameter info
        if len(section4) < 15:
            return None

        pdt_template = struct.unpack('>H', section4[5:7])[0]
        num_coords = section4[7]

        # For template 4.0 (analysis/forecast):
        # Octet 10: parameter category
        # Octet 11: parameter number
        param_category = section4[9]
        param_number = section4[10]

        # Get parameter information from lookup tables
        param_info = get_parameter_info(discipline, param_category, param_number)

        # Extract level information from Section 4
        # For PDT 4.0, level info starts at octet 11 (index 10)
        level_info = self._extract_level_info(section4, pdt_template)

        # Extract forecast/time information
        forecast_info = self._extract_forecast_info(section4, pdt_template, year, month, day, hour, minute, second, significance)

        # Build basic message structure
        message = {
            'center': center,
            'subcenter': subcenter,
            'discipline': {
                'code': discipline,
                'name': DISCIPLINE_TABLE.get(discipline, f"Unknown discipline {discipline}")
            },
            'parameter': {
                'discipline': discipline,
                'category': param_category,
                'number': param_number,
                'short_name': param_info['short_name'],
                'long_name': param_info['long_name'],
                'units': param_info['units'],
                'table_source': param_info['source']
            },
            'level': level_info,
            'forecast': forecast_info,
            'gdt_template': gdt_template,
            'pdt_template': pdt_template,
            'sections_found': sorted(sections.keys())
        }

        # Extract grid metadata with error handling
        try:
            grid_info = self._extract_grid_metadata(section3, gdt_template)
            message['grid'] = grid_info
        except (struct.error, ValueError, IndexError) as e:
            # If grid extraction fails, still return the message with basic grid info
            message['grid'] = {
                'template': gdt_template,
                'extraction_error': str(e),
                'ni': None,
                'nj': None,
                'grid_type': self._get_grid_type_name(gdt_template),
                'scanning_mode': None
            }

        return message

    def _extract_level_info(self, section4, pdt_template):
        """Extract level information from Section 4.

        For PDT 4.0 (template 0):
        - Octet 11 (index 10): First level type
        - Octet 12-13 (index 11-12): Scale factor for first level
        - Octet 14-17 (index 13-16): Scaled value for first level
        - Octet 18 (index 17): Second level type
        - Octet 19-20 (index 18-19): Scale factor for second level
        - Octet 21-24 (index 20-23): Scaled value for second level

        Returns:
            Dictionary with level type, value, and description
        """
        level_info = {
            'pdt_template': pdt_template,
            'first_level': None,
            'second_level': None
        }

        try:
            if pdt_template == 0:  # PDT 4.0
                if len(section4) >= 18:
                    # First level
                    level1_type = section4[10]
                    level1_scale = struct.unpack('>h', section4[11:13])[0]  # signed short
                    level1_value = struct.unpack('>i', section4[13:17])[0]  # signed int

                    # Parse scaled value
                    actual_level1 = parse_scaled_value(level1_scale, level1_value)
                    level1_info = get_level_type_info(level1_type)

                    level_info['first_level'] = {
                        'type_code': level1_type,
                        'type_name': level1_info['short_name'],
                        'type_description': level1_info['description'],
                        'scale_factor': level1_scale,
                        'scaled_value': level1_value,
                        'value': actual_level1
                    }

                    # Second level (if present)
                    if len(section4) >= 25:
                        level2_type = section4[17]
                        level2_scale = struct.unpack('>h', section4[18:20])[0]  # signed short
                        level2_value = struct.unpack('>i', section4[20:24])[0]  # signed int

                        # Parse scaled value
                        actual_level2 = parse_scaled_value(level2_scale, level2_value)
                        level2_info = get_level_type_info(level2_type)

                        level_info['second_level'] = {
                            'type_code': level2_type,
                            'type_name': level2_info['short_name'],
                            'type_description': level2_info['description'],
                            'scale_factor': level2_scale,
                            'scaled_value': level2_value,
                            'value': actual_level2
                        }

            else:
                # For other PDT templates, store minimal info
                level_info['extraction_note'] = f'Level extraction for PDT {pdt_template} not fully implemented'
                level_info['pdt_template'] = pdt_template

        except (struct.error, ValueError, IndexError) as e:
            level_info['extraction_error'] = str(e)

        return level_info

    def _extract_forecast_info(self, section4, pdt_template, year, month, day, hour, minute, second, significance):
        """Extract forecast/time information from Section 4.

        For PDT 4.0 (template 0):
        - Forecast time offset is at octets 15-16 (index 14-15)
        - Time range indicator is in some templates

        Returns:
            Dictionary with reference_time, forecast_time, and temporal metadata
        """
        forecast_info = {}

        try:
            # Format reference time
            ref_time = format_reference_time(year, month, day, hour, minute, second, significance)
            forecast_info['reference_time'] = ref_time

            # Extract forecast offset (for PDT 4.0)
            forecast_offset = 0
            forecast_offset_unit = 1  # Default: hours

            if pdt_template == 0 and len(section4) >= 19:
                # Octets 15-16: Forecast time in units defined by octet 19
                forecast_offset = struct.unpack('>H', section4[14:16])[0]
                # Octet 19: Time range indicator/forecast time unit
                if len(section4) >= 22:
                    time_unit_code = section4[21]  # Octet 22
                    # Time unit codes: 0=minute, 1=hour, 2=day, 3=month, 4=year, etc.
                    forecast_offset_unit = time_unit_code
            elif pdt_template == 1 and len(section4) >= 19:
                # PDT 4.1 (ensemble forecast)
                forecast_offset = struct.unpack('>H', section4[14:16])[0]
                if len(section4) >= 22:
                    forecast_offset_unit = section4[21]

            forecast_info['forecast_offset'] = {
                'value': forecast_offset,
                'unit_code': forecast_offset_unit,
                'unit_name': self._get_time_unit_name(forecast_offset_unit)
            }

            # Calculate forecast time if offset is available
            if forecast_offset > 0:
                forecast_info['forecast_time'] = self._calculate_forecast_time(
                    ref_time, forecast_offset, forecast_offset_unit
                )

        except (struct.error, ValueError, IndexError) as e:
            forecast_info['extraction_error'] = str(e)

        return forecast_info

    def _get_time_unit_name(self, unit_code):
        """Get human-readable time unit name from code.

        Time unit codes (Code Table 4.4):
        0: minutes
        1: hours
        2: days
        3: months
        4: years
        5: decades
        6: normals (30 years)
        7: centuries (100 years)
        8-9: reserved
        10: 3 hours
        11: 6 hours
        12: 12 hours
        13: seconds
        """
        time_units = {
            0: "minutes",
            1: "hours",
            2: "days",
            3: "months",
            4: "years",
            5: "decades",
            6: "normals (30 years)",
            7: "centuries",
            10: "3 hours",
            11: "6 hours",
            12: "12 hours",
            13: "seconds"
        }
        return time_units.get(unit_code, f"unknown_unit({unit_code})")

    def _calculate_forecast_time(self, ref_time, offset, unit_code):
        """Calculate forecast time string from reference time and offset.

        Args:
            ref_time: Reference time dictionary from format_reference_time()
            offset: Forecast offset value
            unit_code: Time unit code

        Returns:
            Dictionary with iso_string and offset information
        """
        try:
            from datetime import timedelta

            # Convert offset to hours
            offset_hours = offset
            if unit_code == 0:  # minutes
                offset_hours = offset / 60.0
            elif unit_code == 2:  # days
                offset_hours = offset * 24.0
            elif unit_code == 10:  # 3 hours
                offset_hours = offset * 3.0
            elif unit_code == 11:  # 6 hours
                offset_hours = offset * 6.0
            elif unit_code == 12:  # 12 hours
                offset_hours = offset * 12.0
            elif unit_code == 13:  # seconds
                offset_hours = offset / 3600.0
            elif unit_code in (3, 4, 5, 6, 7):  # months, years, decades, etc.
                # For longer units, can't easily convert to hours
                offset_hours = None

            # Calculate forecast time if possible
            iso_string = None
            if offset_hours is not None:
                try:
                    ref_dt = datetime.fromisoformat(ref_time['iso_string'].replace('Z', '+00:00'))
                    forecast_dt = ref_dt + timedelta(hours=offset_hours)
                    iso_string = forecast_dt.isoformat()
                except (ValueError, OverflowError):
                    pass

            return {
                'iso_string': iso_string,
                'offset': offset,
                'offset_unit_code': unit_code,
                'offset_unit_name': self._get_time_unit_name(unit_code),
                'offset_hours': offset_hours
            }

        except Exception as e:
            return {
                'error': str(e),
                'offset': offset,
                'offset_unit_code': unit_code
            }

    def _extract_grid_metadata(self, section3, gdt_template):
        """Extract grid metadata from Grid Definition Section (Section 3).

        Args:
            section3: Raw bytes of Section 3
            gdt_template: Grid Definition Template number

        Returns:
            Dictionary containing grid metadata

        Raises:
            struct.error: If binary unpacking fails
            ValueError: If section is malformed
            IndexError: If section is too short
        """
        if len(section3) < 7:
            raise ValueError(f"Section 3 too short: {len(section3)} bytes")

        # Initialize grid structure
        grid_info = {
            'template': gdt_template,
            'grid_type': self._get_grid_type_name(gdt_template),
            'ni': None,  # Number of points along latitude (i direction)
            'nj': None,  # Number of points along longitude (j direction)
            'scanning_mode': None
        }

        # Template 3.0: Latitude/Longitude (or Equidistant Cylindrical)
        if gdt_template == 0:
            grid_info.update(self._extract_grid_template_0(section3))
        # Template 3.1: Rotated Latitude/Longitude
        elif gdt_template == 1:
            grid_info.update(self._extract_grid_template_1(section3))
        # Template 3.10: Mercator
        elif gdt_template == 10:
            grid_info.update(self._extract_grid_template_10(section3))
        else:
            # For unsupported templates, still try to extract basic info
            grid_info['extraction_warning'] = f'Grid template {gdt_template} not fully supported'

        return grid_info

    def _extract_grid_template_0(self, section3):
        """Extract grid metadata for Template 3.0 (Latitude/Longitude).

        Template 3.0 structure:
        - Octets 8-9: Ni (number of points along latitude)
        - Octets 10-11: Nj (number of points along longitude)
        - Octet 12: Basic angle of the initial production domain
        - Octets 13-16: Latitude of first point (scaled by 10^-6)
        - Octets 17-20: Longitude of first point (scaled by 10^-6)
        - Octets 21-24: Latitude of last point (scaled by 10^-6)
        - Octets 25-28: Longitude of last point (scaled by 10^-6)
        - Octets 29-32: i direction increment (scaled by 10^-6)
        - Octets 33-36: j direction increment (scaled by 10^-6)
        - Octet 37: Scanning mode

        Returns:
            Dictionary with grid metadata for template 0
        """
        if len(section3) < 37:
            raise ValueError(f"Section 3 too short for template 0: {len(section3)} bytes, need at least 37")

        info = {}

        # Ni and Nj (number of grid points)
        info['ni'] = struct.unpack('>H', section3[7:9])[0]
        info['nj'] = struct.unpack('>H', section3[9:11])[0]

        # Basic angle flags (octet 12)
        basic_angle = section3[11]

        # Latitude/longitude bounds (scaled by 10^-6)
        info['lat_first'] = struct.unpack('>i', section3[12:16])[0] / 1000000.0
        info['lon_first'] = struct.unpack('>i', section3[16:20])[0] / 1000000.0
        info['lat_last'] = struct.unpack('>i', section3[20:24])[0] / 1000000.0
        info['lon_last'] = struct.unpack('>i', section3[24:28])[0] / 1000000.0

        # Grid increments (scaled by 10^-6)
        info['di'] = struct.unpack('>i', section3[28:32])[0] / 1000000.0
        info['dj'] = struct.unpack('>i', section3[32:36])[0] / 1000000.0

        # Scanning mode (octet 37)
        scanning_mode = section3[36]
        info['scanning_mode'] = self._parse_scanning_mode(scanning_mode)

        # Calculate bounds from grid parameters if available
        if info['ni'] and info['nj'] and info['di'] and info['dj']:
            info['bounds_calculated'] = {
                'lat_min': min(info['lat_first'], info['lat_last']),
                'lat_max': max(info['lat_first'], info['lat_last']),
                'lon_min': min(info['lon_first'], info['lon_last']),
                'lon_max': max(info['lon_first'], info['lon_last'])
            }

        return info

    def _extract_grid_template_1(self, section3):
        """Extract grid metadata for Template 3.1 (Rotated Latitude/Longitude).

        Returns minimal info for rotated grids.
        """
        info = {}
        if len(section3) >= 11:
            info['ni'] = struct.unpack('>H', section3[7:9])[0]
            info['nj'] = struct.unpack('>H', section3[9:11])[0]
            info['extraction_note'] = 'Rotated grid - full extraction not implemented'
        return info

    def _extract_grid_template_10(self, section3):
        """Extract grid metadata for Template 3.10 (Mercator).

        Returns minimal info for Mercator grids.
        """
        info = {}
        if len(section3) >= 11:
            info['ni'] = struct.unpack('>H', section3[7:9])[0]
            info['nj'] = struct.unpack('>H', section3[9:11])[0]
            info['extraction_note'] = 'Mercator grid - full extraction not implemented'
        return info

    def _get_grid_type_name(self, gdt_template):
        """Get human-readable grid type name from template number."""
        grid_types = {
            0: 'latitude_longitude',
            1: 'rotated_latitude_longitude',
            2: 'stretched_lat_lon',
            3: 'stretched_rotated_lat_lon',
            5: 'polar_stereographic',
            10: 'mercator',
            12: 'lambert_conformal',
            20: 'gaussian_lat_lon',
            30: 'reduced_gg',
            40: 'rotated_reduced_gg',
            42: 'stretched_gg',
            43: 'stretched_rotated_gg',
            50: 'spectral',
            90: 'space_view_perspective',
            204: 'unstructured',
            32768: 'cross_section',
            1000: 'triangular',
            1100: 'unstructured_hexagonal'
        }
        return grid_types.get(gdt_template, f'unknown_template_{gdt_template}')

    def _parse_scanning_mode(self, mode_byte):
        """Parse scanning mode flags from octet 37 in Template 3.0.

        The scanning mode is a bitmask:
        - Bit 1 (0x80): Scan mode for i points (0: +x direction, 1: -x direction)
        - Bit 2 (0x40): Scan mode for j points (0: +y direction, 1: -y direction)
        - Bit 3 (0x20): Adjacent i points are consecutive (1: yes, 0: no)
        - Bit 4 (0x10): Adjacent rows are consecutive (1: yes, 0: no)

        Returns:
            Dictionary describing the scanning mode
        """
        return {
            'raw_value': mode_byte,
            'i_direction_negative': bool(mode_byte & 0x80),
            'j_direction_negative': bool(mode_byte & 0x40),
            'i_points_consecutive': bool(mode_byte & 0x20),
            'rows_consecutive': bool(mode_byte & 0x10)
        }


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
        'parser_version': 'basic_0.1'
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
