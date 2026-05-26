# agi4-core

Pure verdict logic for the AGI/4 specification.

Zero-dependency core library implementing the verdict function, conjunct evaluators, and consistency checks. No I/O, no side effects, no panics on valid input.

## Features

- **Verdict Function**: Combines four conjunct statuses and consistency check result into a final AGI/4 verdict
- **Per-Conjunct Evaluators**: Functions to evaluate Generality, EconomicSubstitutability, EnvironmentalTransfer, AutonomousAgency
- **Consistency Checks**: Three sub-rules from SPEC.md §4 (prevent insufficient_data masking, variance bounds, provenance metadata)
- **Threshold Constants**: All SPEC.md §3 values with spec-to-code traceability
- **Exhaustive Types**: Enums that make illegal states unrepresentable (ConjunctStatus, Verdict)
- **Property Tests**: Full 512-case exhaustive test and invariant property tests

## Usage

```rust
use agi4_core::verdict::{Verdict, verdict};
use agi4_core::conjunct::ConjunctStatus;
use agi4_core::consistency::ConsistencyResult;

let result = verdict(
    ConjunctStatus::Pass,
    ConjunctStatus::Pass,
    ConjunctStatus::Pass,
    ConjunctStatus::Pass,
    ConsistencyResult::Pass,
);

assert_eq!(result, Verdict::Attested);
```

## SemVer Discipline

agi4-core is load-bearing for the spec. Breaking changes (conjunct redefinitions, threshold values, verdict logic) require MAJOR version bumps. Enforced by `cargo-semver-checks` in CI.

## See Also

- [SPEC.md](../../SPEC.md) — Specification with thresholds and verdicts
- [agi4](../agi4/) — Facade library with CLI binary
- [agi4-schema](../agi4-schema/) — JSON output types
