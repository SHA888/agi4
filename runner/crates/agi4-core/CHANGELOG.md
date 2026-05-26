# agi4-core Changelog

## [0.1.0] - 2026-05-26

### Added

- Core verdict types: `Verdict` enum with Attested, NotAttested, InsufficientData variants
- `ConjunctStatus` enum: Pass, Fail, Partial, InsufficientData (exhaustive)
- `Evidence`, `SourceId`, `MeasurementId`, `SourceValue` types for evidence representation
- `verdict()` function: Pure, total verdict combiner per SPEC.md §5
- Per-conjunct evaluation functions:
  - `generality::evaluate()`
  - `economic_substitutability::evaluate()`
  - `environmental_transfer::evaluate()`
  - `autonomous_agency::evaluate()`
- `consistency_check()` function implementing three sub-rules:
  1. Prevent insufficient_data masking (3 Pass + 1 InsufficientData blocks Attested)
  2. Variance bounds (min_margin ≥ 0.5 × max_margin when all pass)
  3. Provenance metadata completeness
- Threshold constants in `threshold.rs` module:
  - Generality: ARC_AGI_2_PASS (0.85), ARC_AGI_3_FLOOR (0.05)
  - EconomicSubstitutability: GDPVAL_PASS (0.85), GDPVAL_AA_FLOOR (0.05)
  - EnvironmentalTransfer: OSWorld_PASS (0.85), RLI_FLOOR (0.10)
  - AutonomousAgency: METR_PASS_HOURS (168.0), SWEBENCH_VERIFIED_PASS@5 (0.75)
  - Consistency: MARGIN_VARIANCE_RATIO (0.5)
- Comprehensive test suite:
  - 10 unit tests on verdict function
  - 19 tests on consistency check rules
  - 6+ per-conjunct evaluation tests
  - Exhaustive 512-case property test
  - Property tests for determinism, totality, fail/partial/insufficient_data dominance
- SemVer compliance enforcement via `cargo-semver-checks` in CI

### Design Principles

- **Hexagonal Architecture**: Pure logic, zero I/O, all effects at boundaries
- **Parse-Don't-Validate**: Illegal states unrepresentable at type level
- **Make-Illegal-States-Unrepresentable**: Exhaustive enums, no Option/Result for verdict
- **Mechanical Verdict**: Pure, total, deterministic (no panics on valid input)
- **Spec-to-Code Traceability**: Every SPEC.md threshold and rule has a named constant or function
