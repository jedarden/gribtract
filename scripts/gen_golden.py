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

        # Forecast time offset (varies by template - typically octet 15-16 for PDT 4.0)
        forecast_offset = 0
        if len(section4) >= 19:
            forecast_offset = struct.unpack('>H', section4[14:16])[0]  # Octets 15-16

        # Build basic message structure
        message = {
            'center': center,
            'subcenter': subcenter,
            'discipline': discipline,
            'parameter': {
                'discipline': discipline,
                'category': param_category,
                'number': param_number
            },
            'forecast': {
                'reference_time': {
                    'year': year,
                    'month': month,
                    'day': day,
                    'hour': hour,
                    'minute': minute,
                    'second': second,
                    'significance': significance
                },
                'forecast_offset': forecast_offset
            },
            'gdt_template': gdt_template,
            'pdt_template': pdt_template,
            'grid': {
                'template': gdt_template
            },
            'sections_found': sorted(sections.keys())
        }

        # Try to extract grid info if available (template 3.0 - lat/lon)
        if gdt_template == 0 and len(section3) >= 32:
            # Template 3.0: Latitude/Longitude
            # Octets 8-9: Ni (nx)
            # Octets 10-11: Nj (ny)
            nx = struct.unpack('>H', section3[7:9])[0]
            ny = struct.unpack('>H', section3[9:11])[0]

            # Basic angle flags (octet 12) should be 0 for lat/lon in degrees

            # Octets 13-16: Latitude of first point (scaled by 10^-6)
            # Octets 17-20: Longitude of first point (scaled by 10^-6)
            lat_first = struct.unpack('>i', section3[12:16])[0] / 1000000.0
            lon_first = struct.unpack('>i', section3[16:20])[0] / 1000000.0

            # Octets 21-24: Latitude of last point
            # Octets 25-28: Longitude of last point
            lat_last = struct.unpack('>i', section3[20:24])[0] / 1000000.0
            lon_last = struct.unpack('>i', section3[24:28])[0] / 1000000.0

            # Octets 29-32: i direction increment
            # Octets 33-36: j direction increment
            di = struct.unpack('>i', section3[28:32])[0] / 1000000.0 if len(section3) >= 32 else 0.0
            dj = struct.unpack('>i', section3[32:36])[0] / 1000000.0 if len(section3) >= 36 else 0.0

            message['grid'].update({
                'nx': nx,
                'ny': ny,
                'lat_first': lat_first,
                'lon_first': lon_first,
                'lat_last': lat_last,
                'lon_last': lon_last,
                'di': di,
                'dj': dj
            })

        return message


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
