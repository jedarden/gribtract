# Fixture Documentation

This directory contains detailed reference documentation for GRIB2 fixtures used in gribtract testing and validation.

## Available Documentation

### Grid Definition References
- [GFS Fixtures — Complete Reference](gfs-fixtures-complete-reference.md) — Comprehensive reference for all GFS fixtures, including Gaussian grids (GDT 40), regular lat/lon grids (GDT 0), rotated lat/lon grids (GDT 1), data representation templates (DRT 0, 3), and parameter categories.
- [GFS Gaussian-Grid Structure](gfs-gaussian-grid-structure.md) — Detailed documentation for GFS Gaussian Latitude/Longitude grids (GDT 3.40), including data structure schema, key components, and implementation details.

## Related Documentation

### Core Reference Documentation
- [Golden JSON Schema](../golden-json-schema.md) — Test fixture JSON schema reference
- [GDT Inspection Summary](../bf-1357i-grid-definition-reference.md) — Grid Definition Template inspection guide
- [Spatial Extent Extraction](../bf-1357i-spatial-extent-extraction-guide.md) — Guide for extracting grid coverage information

### Verification Documentation
- [NOAA Product Authenticity Verification](../noaa-product-authenticity-verification.md) — Methods for verifying NOAA data sources
- [CONUS Coverage Verification Criteria](../conus-coverage-verification-criteria.md) — Testing criteria for CONUS coverage

## Fixture Categories

### GFS (Global Forecast System)
- **Gaussian Grids**: GDT 3.40 — Uniform longitude, Gaussian latitude spacing
- **Lat/Lon Grids**: GDT 3.0 — Regular latitude/longitude grids
- **Lambert Conformal**: GDT 3.30 — Regional CONUS coverage

### NAM (North American Mesoscale)
- **Lambert Conformal**: GDT 3.30 — High-resolution regional grids

### GEFS (Global Ensemble Forecast System)
- **Ensemble Products**: PDT 4.1, PDT 4.8 — Individual members and ensemble means

## Usage

Use these documents as reference when:
- Adding new fixture support to gribtract
- Debugging grid definition parsing
- Understanding data structure conventions
- Verifying fixture authenticity and provenance

## Fixture Manifest

All fixtures are indexed in `tests/corpus/manifest.json` with provenance, source URLs, and verification status.

---

**Last Updated**: 2026-07-25
