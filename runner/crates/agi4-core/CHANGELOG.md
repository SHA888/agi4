# Changelog — agi4-core

## [0.1.1] - 2026-06-02

### Fixed

- Variance bounds check now correctly implements SPEC.md §4 rule 2: compares all sources in a single pool (Fraction + Hours together) rather than splitting by type, preventing spurious failures on strong long-horizon models

## [0.1.0] - 2026-05-26

### Added

- Verdict logic and type system with four conjuncts: Generality, EconomicSubstitutability, EnvironmentalTransfer, AutonomousAgency
- Per-conjunct evaluation functions for each measurement source
- Consistency check implementation with three sub-rules (prevent insufficient_data masking, variance bounds, provenance metadata)
- Verdict function combining conjunct statuses and consistency results
- Verdict invariant property tests covering determinism, totality, and dominance rules
- Threshold constants for all SPEC.md §3 values, with spec-to-code traceability
