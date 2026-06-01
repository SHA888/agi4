# Changelog — agi4-adapters

## [0.1.1] - 2026-06-02

### Fixed

- Removed panic-prone `Default` impl from `CachingFetcher` that called `.expect()` in non-test code; replaced with explicit `CachingFetcher::new()` calls in tests
- Fixed error handling in `write_cache`: replaced `unwrap_or_default()` with explicit early return when cache path has no filename component
- Added `AdapterErrorKind` enum to distinguish Parse vs Validation errors, enabling better test coverage and error discrimination without string matching

## [0.1.0] - 2026-05-26

### Added

- Source trait abstraction for upstream data ingestion
- Fetcher trait for I/O abstraction (HTTP, file, in-memory)
- InMemoryFetcher for test fixtures
- METR reference adapter for autonomous agency evidence (simplest schema)
