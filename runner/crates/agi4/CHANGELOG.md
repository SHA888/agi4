# agi4 Changelog

## [0.1.0] - 2026-05-26

### Added

#### Library API
- `core::` module: ConjunctStatus, Verdict, Evidence, ConsistencyResult types
- `consistency_check()` function for cross-conjunct validation
- `schema::` module: VerdictOutput, ConjunctReport, and related types
- `thresholds::` module: All SPEC.md §3 threshold constants re-exported
- `render_verdict()` function for Markdown report generation
- `VERSION` constant: Derived from Cargo.toml
- `SPEC_VERSION` constant: "0.1.0", hardcoded to match SPEC.md

#### CLI Binary
- `attest --model <id> --fixture <path>`: Generate verdict from local fixture
- `attest --model <id> --live`: Fetch from live upstream sources (stubbed in v0.1.0)
- `render --input <verdict.json>`: Convert JSON verdict to Markdown
- `schema`: Export JSON Schema for verdict outputs
- `version`: Display agi4 and SPEC versions
- Error handling: Clear error messages on missing arguments, file not found, invalid JSON
- Exit codes: 0 on success, 1 on error

#### Testing
- 4 facade tests verifying re-exports and version constants
- 1 CLI integration test verifying JSON serialization round-trip
- All 116+ workspace tests passing

#### Packaging
- Published to crates.io as `agi4 v0.1.0`
- Includes binary as `agi4` command
- Depends on: agi4-core, agi4-schema, agi4-adapters, agi4-report
- Optional dependencies removed (no default features)

### Design Principles

- **Curated Public API**: Facade re-exports only high-level types and functions
- **Version Alignment**: Crate version, SPEC_VERSION, and runner_version all synchronized
- **Clear Intent**: CLI subcommands map 1:1 to primary operations (attest, render, schema, version)
- **Graceful Degradation**: Live attestation stubbed until Phase 2 (adapters ready)
