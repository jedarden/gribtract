# Fixture Documentation

Detailed reference documentation for GRIB2 fixtures used in gribtract testing and validation.

## GFS Gaussian Fixture

- **[GFS Gaussian Fixture — Canonical Reference](gfs-gaussian-fixture.md)** — the single source of
  truth for the GFS Gaussian fixtures (`core_gaussian_gdt40`, `gfs_gaussian_gdt40_t1534`). Covers
  fixture identity and provenance, build/test status, the exercised-vs-implemented GRIB2 templates
  (GDT 3.40, PDT 4.2/4.12, DRT 5.2/5.3), the golden JSON schema, Gaussian grid parameters, crate
  dependencies, and the project-convention audit. Includes the remaining integration roadmap
  (the fixture does not yet decode end-to-end — PDT 4.12 is unimplemented).

## Fixture Categories

### GFS (Global Forecast System)
- **Gaussian Grids**: GDT 3.40 — uniform longitude, Gaussian latitude spacing *(see canonical reference above)*
- **Lat/Lon Grids**: GDT 3.0 — regular latitude/longitude grids
- **Lambert Conformal**: GDT 3.30 — regional CONUS coverage

### NAM (North American Mesoscale)
- **Lambert Conformal**: GDT 3.30 — high-resolution regional grids

### GEFS (Global Ensemble Forecast System)
- **Ensemble Products**: PDT 4.1, PDT 4.8 — individual members and ensemble means

## Related Documentation

### Core Reference
- [Golden JSON Schema](../golden-json-schema.md) — test fixture JSON schema reference
- [GDT Inspection Summary](../bf-1357i-grid-definition-reference.md) — Grid Definition Template inspection guide
- [Spatial Extent Extraction](../bf-1357i-spatial-extent-extraction-guide.md) — guide for extracting grid coverage

### Verification
- [NOAA Product Authenticity Verification](../noaa-product-authenticity-verification.md) — methods for verifying NOAA data sources
- [CONUS Coverage Verification Criteria](../conus-coverage-criteria.md) — testing criteria for CONUS coverage

## Usage

Use the canonical reference when adding fixture support, debugging grid-definition parsing,
understanding data-structure conventions, or verifying fixture authenticity and provenance.

All fixtures are indexed in `tests/corpus/manifest.json` with provenance, source URLs, and sha256
verification status.

---

**Last Updated**: 2026-07-26
