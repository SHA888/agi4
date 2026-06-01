# Changelog — agi4

## [0.1.1] - 2026-06-02

### Fixed

- Fixed `attest_from_fixture`: evidence vectors in VerdictOutput were always empty; now correctly populates conjunct evidence by filtering source associations and converting core Evidence to schema EvidenceReport types
- Fixed `load_evidence_from_fixtures`: JSON read errors now use `continue` instead of `?` operator, consistent with other error paths and allowing attestation to proceed with valid sources even if one fixture file is unreadable

## [0.1.0] - 2026-05-26

### Added

- Public library API re-exporting verdict logic, types, and utilities
- CLI binary with subcommands: attest, render, schema, version
- attest --fixture for local fixture-based attestation
- render --input for verdict JSON to Markdown conversion
- schema subcommand for JSON schema export
- version subcommand for SPEC.md version tracking
