# agi4-adapters Changelog

## [0.1.0] - 2026-05-26

### Added

- `Source` trait: Pure parsing abstraction (no I/O)
  - `parse()`: Raw upstream format → typed struct
  - `to_evidence()`: Typed struct + model → Vec<Evidence>
  - `id()`, `endpoint()`: Source metadata
  - Generic over source-specific error type
- `Fetcher` trait: I/O abstraction for testability
  - `fetch(url)`: HTTP/file/memory strategy abstraction
- `InMemoryFetcher`: Test implementation
  - Stores frozen upstream data snapshots
  - Methods: `new()`, `insert()`, `with_data()`
  - Enables unit tests without network calls
- `MetrAdapter`: METR evidence ingestion (autonomous agency)
  - Ingests 80%-time horizon metric (hours)
  - Endpoint: https://metr.org/api/time-horizon
  - Error types: ParseError, ValidationError
  - Flexible endpoint via `with_endpoint()` for testing
- `MetrRaw`: Minimal schema (single f64 field)
  - Serde-compatible for JSON deserialization
  - Validation: hours must be non-negative
- Frozen fixture: `tests/fixtures/metr/time-horizon-168h.json`
- Round-trip test: `metr_round_trip()`

### Design Principles

- **Separation of Concerns**: Parsing (Source) vs. I/O (Fetcher)
- **Dependency Injection**: Fetcher trait enables test doubles
- **Parse-Don't-Validate**: Serde schemas catch bad data early
- **Minimal Schema First**: METR's single f64 establishes pattern
- **Fixture-Driven Testing**: Frozen upstream snapshots for reproducibility
