# Changelog

All notable changes to the agi4 project are documented in this file.

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
