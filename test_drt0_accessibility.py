#!/usr/bin/env python3
"""
DRT=0 CONUS File Accessibility Test Script

Tests HTTP/HTTPS accessibility for NOAA DRT=0 CONUS files:
- RTMA 2.5 CONUS (Real-Time Mesoscale Analysis)
- URMA 2.5 CONUS (Unrestricted Mesoscale Analysis)

Verifies:
1. URL accessibility (HTTP HEAD requests)
2. Download capability
3. File integrity (GRIB2 magic bytes)
4. Authentication requirements
5. Rate limiting behavior

Author: bf-14grj accessibility testing
Date: 2026-07-24
"""

import urllib.request
import urllib.error
import hashlib
import time
import json
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass, asdict
from datetime import datetime


@dataclass
class FileTestResult:
    """Results from testing a single file URL"""
    url: str
    dataset: str
    description: str

    # HTTP accessibility
    http_status: Optional[int] = None
    http_headers: Optional[Dict[str, str]] = None
    accessible: bool = False
    auth_required: bool = False

    # Download test
    downloaded: bool = False
    download_size: int = 0
    download_time: float = 0.0
    download_speed: float = 0.0  # bytes per second

    # File integrity
    file_valid_grib2: bool = False
    file_size_match: bool = False
    md5_checksum: Optional[str] = None

    # Errors
    error_message: Optional[str] = None


class DRT0AccessibilityTester:
    """Test accessibility of DRT=0 CONUS files"""

    def __init__(self, timeout: int = 30, max_downloads: int = 3):
        """
        Initialize tester

        Args:
            timeout: HTTP request timeout in seconds
            max_downloads: Maximum number of files to fully download (to avoid excessive bandwidth)
        """
        self.timeout = timeout
        self.max_downloads = max_downloads
        self.results: List[FileTestResult] = []

    def test_http_head(self, url: str) -> Tuple[Optional[int], Dict[str, str], Optional[str]]:
        """
        Test HTTP HEAD request to a URL

        Returns:
            (status_code, headers_dict, error_message)
        """
        try:
            req = urllib.request.Request(url, method='HEAD')
            with urllib.request.urlopen(req, timeout=self.timeout) as response:
                headers = dict(response.headers.items())
                return response.status, headers, None
        except urllib.error.HTTPError as e:
            return e.code, {}, str(e)
        except urllib.error.URLError as e:
            return None, {}, str(e)
        except Exception as e:
            return None, {}, f"Unexpected error: {e}"

    def download_file(self, url: str, local_path: str) -> Tuple[bool, int, float, Optional[str]]:
        """
        Download a file and track performance

        Returns:
            (success, bytes_downloaded, time_seconds, error_message)
        """
        try:
            start_time = time.time()

            with urllib.request.urlopen(url, timeout=self.timeout) as response:
                total_bytes = 0
                chunk_size = 8192

                with open(local_path, 'wb') as f:
                    while True:
                        chunk = response.read(chunk_size)
                        if not chunk:
                            break
                        f.write(chunk)
                        total_bytes += len(chunk)

                elapsed = time.time() - start_time
                return True, total_bytes, elapsed, None

        except urllib.error.HTTPError as e:
            return False, 0, 0.0, f"HTTP Error {e.code}: {e.reason}"
        except urllib.error.URLError as e:
            return False, 0, 0.0, f"URL Error: {e.reason}"
        except Exception as e:
            return False, 0, 0.0, f"Download error: {e}"

    def verify_grib2_integrity(self, file_path: str) -> Tuple[bool, bool, Optional[str]]:
        """
        Verify GRIB2 file integrity

        Returns:
            (valid_grib2, size_ok, error_message)
        """
        try:
            import os

            if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
                return False, False, "File is empty or does not exist"

            with open(file_path, 'rb') as f:
                magic = f.read(16)

            # Check GRIB2 magic bytes
            # GRIB2 starts with "GRIB" + 0x00 0x00 0x00 0x02
            if len(magic) < 8:
                return False, False, "File too small to be valid GRIB2"

            if not magic.startswith(b'GRIB'):
                return False, False, "Missing GRIB magic bytes"

            if magic[7:8] != b'\x02':
                return False, False, f"Not GRIB Edition 2 (Edition={magic[7]})"

            return True, True, None

        except Exception as e:
            return False, False, f"Integrity check error: {e}"

    def calculate_md5(self, file_path: str) -> Optional[str]:
        """Calculate MD5 checksum of a file"""
        try:
            md5_hash = hashlib.md5()
            with open(file_path, 'rb') as f:
                for chunk in iter(lambda: f.read(8192), b''):
                    md5_hash.update(chunk)
            return md5_hash.hexdigest()
        except Exception:
            return None

    def test_file(self, url: str, dataset: str, description: str,
                 do_download: bool = False) -> FileTestResult:
        """
        Test a single file URL

        Args:
            url: File URL to test
            dataset: Dataset name (e.g., "RTMA 2.5 CONUS")
            description: Human-readable description
            do_download: Whether to perform full download test

        Returns:
            FileTestResult with test results
        """
        result = FileTestResult(
            url=url,
            dataset=dataset,
            description=description
        )

        # Test HTTP HEAD
        print(f"  Testing HTTP HEAD: {description}")
        status, headers, error = self.test_http_head(url)

        if status is not None:
            result.http_status = status
            result.http_headers = headers
            result.accessible = (status == 200)

            # Check for authentication requirement
            if status == 401 or status == 403:
                result.auth_required = True

            # Extract content length from headers
            content_length = None
            if headers:
                for key, value in headers.items():
                    if key.lower() == 'content-length':
                        content_length = int(value)
                        break
        else:
            result.error_message = error

        # Perform download test if requested
        if do_download and result.accessible:
            print(f"  Downloading: {description}")
            local_filename = url.split('/')[-1]
            success, size, elapsed, error = self.download_file(url, local_filename)

            if success:
                result.downloaded = True
                result.download_size = size
                result.download_time = elapsed
                result.download_speed = size / elapsed if elapsed > 0 else 0

                # Verify file integrity
                print(f"  Verifying integrity: {description}")
                valid_grib2, size_ok, integrity_error = self.verify_grib2_integrity(local_filename)
                result.file_valid_grib2 = valid_grib2

                if valid_grib2:
                    result.md5_checksum = self.calculate_md5(local_filename)
                else:
                    result.error_message = integrity_error
            else:
                result.error_message = error

        return result

    def test_rate_limiting(self, url: str, num_requests: int = 5) -> Dict[str, any]:
        """
        Test for rate limiting by making multiple requests

        Returns:
            Dict with rate limiting test results
        """
        print(f"  Testing rate limiting with {num_requests} requests...")

        response_times = []
        rate_limited = False
        status_codes = []

        for i in range(num_requests):
            start = time.time()
            status, _, _ = self.test_http_head(url)
            elapsed = time.time() - start

            status_codes.append(status)
            response_times.append(elapsed)

            if status == 429:  # Too Many Requests
                rate_limited = True

            # Small delay between requests
            time.sleep(0.1)

        return {
            'num_requests': num_requests,
            'response_times': response_times,
            'avg_response_time': sum(response_times) / len(response_times),
            'rate_limited': rate_limited,
            'status_codes': status_codes
        }

    def generate_report(self) -> str:
        """Generate a comprehensive test report"""
        report = []
        report.append("=" * 80)
        report.append("DRT=0 CONUS File Accessibility Test Report")
        report.append(f"Generated: {datetime.now().isoformat()}")
        report.append("=" * 80)
        report.append("")

        # Summary
        total = len(self.results)
        accessible = sum(1 for r in self.results if r.accessible)
        downloaded = sum(1 for r in self.results if r.downloaded)
        valid = sum(1 for r in self.results if r.file_valid_grib2)

        report.append("## Summary")
        report.append(f"Total URLs tested: {total}")
        report.append(f"Accessible (HTTP 200): {accessible}/{total} ({100*accessible//total if total > 0 else 0}%)")
        report.append(f"Downloaded successfully: {downloaded}/{accessible} ({100*downloaded//accessible if accessible > 0 else 0}%)")
        report.append(f"Valid GRIB2 format: {valid}/{downloaded} ({100*valid//downloaded if downloaded > 0 else 0}%)")
        report.append(f"Authentication required: {sum(1 for r in self.results if r.auth_required)}")
        report.append("")

        # Detailed results
        report.append("## Detailed Results")
        report.append("")

        for i, result in enumerate(self.results, 1):
            report.append(f"### {i}. {result.description}")
            report.append(f"Dataset: {result.dataset}")
            report.append(f"URL: {result.url}")
            report.append("")

            # HTTP Status
            if result.http_status:
                report.append(f"HTTP Status: {result.http_status} {'✓' if result.accessible else '✗'}")
            else:
                report.append(f"HTTP Status: ERROR - {result.error_message}")

            # Authentication
            if result.auth_required:
                report.append("Authentication Required: ✗ YES (HTTP 401/403)")
            else:
                report.append("Authentication Required: ✓ No")

            # Download results
            if result.downloaded:
                size_mb = result.download_size / (1024 * 1024)
                speed_mbs = result.download_speed / (1024 * 1024)
                report.append(f"Download: ✓ SUCCESS ({size_mb:.2f} MB in {result.download_time:.2f}s)")
                report.append(f"Speed: {speed_mbs:.2f} MB/s")

                # File integrity
                if result.file_valid_grib2:
                    report.append("GRIB2 Format: ✓ Valid")
                    if result.md5_checksum:
                        report.append(f"MD5: {result.md5_checksum}")
                else:
                    report.append("GRIB2 Format: ✗ Invalid")
            elif result.error_message:
                report.append(f"Download: ✗ FAILED - {result.error_message}")

            report.append("")

        return "\n".join(report)

    def save_json_report(self, filepath: str):
        """Save results as JSON"""
        data = {
            'timestamp': datetime.now().isoformat(),
            'summary': {
                'total': len(self.results),
                'accessible': sum(1 for r in self.results if r.accessible),
                'downloaded': sum(1 for r in self.results if r.downloaded),
                'valid_grib2': sum(1 for r in self.results if r.file_valid_grib2)
            },
            'results': [asdict(r) for r in self.results]
        }

        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)


def main():
    """Main test execution"""
    tester = DRT0AccessibilityTester(timeout=30, max_downloads=3)

    print("=" * 80)
    print("DRT=0 CONUS File Accessibility Test")
    print("Testing RTMA 2.5 and URMA 2.5 CONUS files from NOAA NOMADS")
    print("=" * 80)
    print("")

    # Define test files based on documentation from bf-3s515.md and bf-24ma0.md
    test_files = [
        # RTMA 2.5 CONUS - Latest available files
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260724/rtma2p5.t00z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'RTMA 2.5 CONUS',
            'description': 'RTMA 2.5 CONUS - July 24, 2026 00z (DRT=0)',
            'download': True
        },
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260723/rtma2p5.t12z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'RTMA 2.5 CONUS',
            'description': 'RTMA 2.5 CONUS - July 23, 2026 12z (DRT=0)',
            'download': True
        },

        # URMA 2.5 CONUS - Latest available files
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/urma/prod/urma2p5.20260724/urma2p5.t00z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'URMA 2.5 CONUS',
            'description': 'URMA 2.5 CONUS - July 24, 2026 00z (DRT=0)',
            'download': True
        },

        # Additional RTMA cycles (HEAD test only)
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260724/rtma2p5.t06z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'RTMA 2.5 CONUS',
            'description': 'RTMA 2.5 CONUS - July 24, 2026 06z (DRT=0)',
            'download': False
        },
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260724/rtma2p5.t12z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'RTMA 2.5 CONUS',
            'description': 'RTMA 2.5 CONUS - July 24, 2026 12z (DRT=0)',
            'download': False
        },

        # Additional URMA cycles (HEAD test only)
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/urma/prod/urma2p5.20260724/urma2p5.t06z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'URMA 2.5 CONUS',
            'description': 'URMA 2.5 CONUS - July 24, 2026 06z (DRT=0)',
            'download': False
        },
        {
            'url': 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/urma/prod/urma2p5.20260724/urma2p5.t12z.2dvaranl_ndfd.grb2_wexp',
            'dataset': 'URMA 2.5 CONUS',
            'description': 'URMA 2.5 CONUS - July 24, 2026 12z (DRT=0)',
            'download': False
        }
    ]

    # Run tests
    download_count = 0
    for file_spec in test_files:
        print(f"\nTesting: {file_spec['description']}")

        # Limit downloads to conserve bandwidth
        do_download = file_spec['download'] and download_count < tester.max_downloads
        if file_spec['download'] and not do_download:
            print(f"  (HEAD test only - max download limit reached)")

        result = tester.test_file(
            file_spec['url'],
            file_spec['dataset'],
            file_spec['description'],
            do_download=do_download
        )
        tester.results.append(result)

        if result.downloaded:
            download_count += 1

    # Test rate limiting on one URL
    print(f"\nTesting rate limiting behavior...")
    rate_limit_result = tester.test_rate_limiting(
        'https://nomads.ncep.noaa.gov/pub/data/nccf/com/rtma/prod/rtma2p5.20260724/rtma2p5.t00z.2dvaranl_ndfd.grb2_wexp',
        num_requests=5
    )

    # Generate reports
    print(f"\n{tester.generate_report()}")

    # Add rate limiting results to report
    print("\n## Rate Limiting Test Results")
    print(f"Requests made: {rate_limit_result['num_requests']}")
    print(f"Average response time: {rate_limit_result['avg_response_time']:.3f}s")
    print(f"Rate limited (HTTP 429): {'Yes' if rate_limit_result['rate_limited'] else 'No'}")
    print(f"Status codes: {rate_limit_result['status_codes']}")

    # Save reports
    tester.save_json_report('drt0_accessibility_results.json')
    print(f"\nJSON report saved to: drt0_accessibility_results.json")

    # Save text report
    with open('drt0_accessibility_report.txt', 'w') as f:
        f.write(tester.generate_report())
    print(f"Text report saved to: drt0_accessibility_report.txt")

    print(f"\nTest complete!")


if __name__ == '__main__':
    main()
