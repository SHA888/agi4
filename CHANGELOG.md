# Changelog

All notable changes to the agi4 project are documented in this file.

## [Unreleased]

## [0.1.2] - 2026-06-19

### Changed

#### Thresholds (Calibration Phase 3 Complete)
- All threshold values reviewed and **confirmed stable** under v0.1.2. No movements justified by verdict analysis across five frontier and near-frontier models
  - Generality conjunct thresholds (ARC-AGI-2, ARC-AGI-3, HLE, GPQA-Diamond) remain unchanged; all models exceed thresholds by 20–40% margins
  - Economic substitutability thresholds (GDPval, RLI, APEX-Agents) remain unchanged; correctly discriminate frontier-capable (Claude 3.5 Sonnet, Claude Opus 4) from capable-but-not-frontier models
  - Environmental transfer and autonomous agency thresholds remain unchanged; margins adequate for future capability growth
  - See `attestations/v0.1.2/THRESHOLD_ANALYSIS.md` for detailed margin analysis and rationale

#### Verdict Logic (Code Review Fixes)
- Verdict enum now correctly returns "insufficient_data" when any conjunct lacks evidence (previously returned "not_attested", conflating failure with no-evidence case)
- ConjunctStatus serialization now uses snake_case naming per JSON schema: "insufficient_data" (previously serialized as "insufficientdata" without underscore)
- Consistency check now includes detail message when passing vacuously (zero evidence), clarifying that the check was skipped rather than meaningfully validated

#### Markdown Reporting
- Verdict reasons list now excludes passing conditions (only includes failing/insufficient), preventing contradictory statements like "Consistency Check: pass" under a non-attested verdict

### Added

#### Runner Improvements
- Model metadata now populated for known frontier models: provider (Anthropic, OpenAI, Google, Meta) and version_or_date for all five models in v0.1.2 attestations
- v0.1.2 attestations committed for five frontier models alongside v0.1.0 verdicts
  - Claude 3.5 Sonnet (Anthropic, 2024-06)
  - Claude Opus 4 (Anthropic, 2024-11)
  - GPT-4-Turbo (OpenAI, 2024-04)
  - Gemini 2.0 Flash (Google, 2024-12)
  - Llama3-70B (Meta, 2024-04)
  - See `attestations/v0.1.2/` for v0.1.2 verdicts with corrected output format

### Fixed

#### Schema Compliance (Blocking Code Review Findings)
- Fixed ConjunctStatus enum serialization to emit "insufficient_data" with underscore (schema-compliant) instead of "insufficientdata" via explicit match statement
- Fixed verdict output to use correct top-level verdict enum: "insufficient_data" for no-evidence case (previously conflated with "not_attested")

#### Version Alignment
- Facade crate `agi4` version bumped from 0.1.1 to 0.1.2, now matching v0.1.2 attestation outputs and spec calibration phase
- All verdicts and runner versions now report 0.1.2 consistently

---

## [0.1.1] - 2026-06-02

### Fixed

#### Core (agi4-core)
- Variance bounds check now correctly implements SPEC.md §4 rule 2: compares all sources in a single pool (Fraction + Hours together) rather than splitting by type, preventing spurious failures on strong long-horizon models

#### Adapters (agi4-adapters)
- Removed panic-prone `Default` impl from `CachingFetcher` that called `.expect()` in non-test code; replaced with explicit `CachingFetcher::new()` calls in tests
- Fixed error handling in `write_cache`: replaced `unwrap_or_default()` with explicit early return when cache path has no filename component
- Added `AdapterErrorKind` enum to distinguish Parse vs Validation errors, enabling better test coverage and error discrimination without string matching
  - New constructors: `AdapterError::parse()`, `AdapterError::validation()`, `AdapterError::with_kind()`
  - `kind()` method to query error category

#### Facade (agi4)
- Fixed `attest_from_fixture`: evidence vectors in VerdictOutput were always empty; now correctly populates conjunct evidence by filtering source associations and converting core Evidence to schema EvidenceReport types
- Fixed `load_evidence_from_fixtures`: JSON read errors now use `continue` instead of `?` operator, consistent with other error paths and allowing attestation to proceed with valid sources even if one fixture file is unreadable

#### Testing
- Updated variance bound regression tests to reflect single-pool semantics per SPEC.md §4 rule 2

## [0.1.0] - 2026-05-26

### Added

#### Core (agi4-core)
- Verdict logic and type system with four conjuncts: Generality, EconomicSubstitutability, EnvironmentalTransfer, AutonomousAgency
- Per-conjunct evaluation functions for each measurement source
- Consistency check implementation with three sub-rules (prevent insufficient_data masking, variance bounds, provenance metadata)
- Verdict function combining conjunct statuses and consistency results
- Verdict invariant property tests covering determinism, totality, and dominance rules
- Threshold constants for all SPEC.md §3 values, with spec-to-code traceability

#### Schema (agi4-schema)
- VerdictOutput, ModelMetadata, ConjunctsOutput, ConjunctReport types
- EvidenceReport, ProvenanceReport, MarginReport, ConsistencyCheckOutput types
- JSON schema generation and validation via schemars
- Schema drift detection test in CI
- Canonical JSON Schema v0.1.0 for verdict output validation

#### Adapters (agi4-adapters)
- Source trait abstraction for upstream data ingestion
- Fetcher trait for I/O abstraction (HTTP, file, in-memory)
- InMemoryFetcher for test fixtures
- METR reference adapter for autonomous agency evidence (simplest schema)

#### Report (agi4-report)
- Markdown report rendering from VerdictOutput
- Per-conjunct evidence tables with source, measurement, value, threshold, pass/fail
- Provenance links to upstream sources
- Margin analysis with min/max values
- Consistency check detail sections
- Verdict summary with reasons for non-attestation
- Known gaps acknowledgments

#### Facade (agi4)
- Public library API re-exporting verdict logic, types, and utilities
- CLI binary with subcommands: attest, render, schema, version
- attest --fixture for local fixture-based attestation
- render --input for verdict JSON to Markdown conversion
- schema subcommand for JSON schema export
- version subcommand for SPEC.md version tracking

#### CI/Testing
- Comprehensive CI pipeline with 9 checks: format, lint, test, semver, security-audit, deny, schema-validation, adapter-fixtures, adapter-fixture-validation, spec-traceability
- All tests passing: 116+ workspace tests, property tests covering 512 verdict input combinations
- End-to-end integration test with frozen METR fixture
- Fixture round-trip tests for adapter validation
- cargo-semver-checks enforcement on agi4-core

#### Documentation
- SPEC.md defining conjuncts, upstream sources, thresholds, consistency rules (§1-§5)
- ARCHITECTURE.md documenting hexagonal architecture, crate layout, verdict pipeline
- README.md with project overview, design principles, scope boundaries
- CLAUDE.md with developer guidance
- Plans.md with Phase 1-5 roadmap and task tracking

### Publishing
- All crates published to crates.io: agi4-core, agi4-schema, agi4-adapters, agi4-report, agi4
- Git tag v0.1.0 for stable release

---

## Phases

- **Phase 1 (v0.1.0)**: Scaffold and first verdict (completed)
- **Phase 2 (v0.1.1)**: Real adapters, first live attestation (todo)
- **Phase 3 (v0.1.2)**: Calibration based on first attestation (todo)
- **Phase 4 (v0.1.3)**: Gap closure - NES specification (todo)
- **Phase 5 (v0.2.0)**: First stable threshold set (todo)

---

[Unreleased]: https://github.com/SHA888/agi4/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/SHA888/agi4/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/SHA888/agi4/releases/tag/v0.1.0
