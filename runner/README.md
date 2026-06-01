# agi4 — Rust Workspace

Reference runner implementation for the AGI/4 specification, structured as a Cargo workspace with 5 member crates.

## Crate Overview

```
runner/
├── crates/
│   ├── agi4              — Facade library + CLI binary
│   ├── agi4-core         — Pure verdict logic (zero dependencies)
│   ├── agi4-schema       — JSON output types and schema validation
│   ├── agi4-adapters     — Upstream source adapters (9 sources)
│   └── agi4-report       — Markdown report rendering
└── tests/
    └── fixtures/        — Frozen upstream data snapshots per adapter
```

### agi4 (Facade + CLI)

**Public API** — Library re-exporting verdict logic, types, and utilities.

**CLI Binary** — Subcommands for local and live attestation:
- `agi4 attest --model <id> --fixture <path>` — Fixture-based verdict
- `agi4 attest --model <id> --live` — Live upstream fetch + verdict (v0.1.1+)
- `agi4 render --input <verdict.json>` — JSON to Markdown conversion
- `agi4 schema` — JSON schema export
- `agi4 version` — Show SPEC.md and runner versions

See [`crates/agi4/README.md`](./crates/agi4/README.md)

### agi4-core (Pure Verdict Logic)

**Verdict Function** — Pure, total, exhaustive:
- `evaluate_generality(&evidence) → ConjunctStatus`
- `evaluate_economic_substitutability(&evidence) → ConjunctStatus`
- `evaluate_environmental_transfer(&evidence) → ConjunctStatus`
- `evaluate_autonomous_agency(&evidence) → ConjunctStatus`
- `verdict(g, e, en, a, consistency) → Verdict`

**Consistency Checks** — Three rules from SPEC.md §4:
1. Prevent insufficient_data masking
2. Variance bounds (min_margin ≥ 0.5 × max_margin across all sources)
3. Provenance metadata completeness

**Key Property** — No `unwrap()` or `expect()` outside tests. SemVer-enforced by `cargo-semver-checks`.

See [`crates/agi4-core/README.md`](./crates/agi4-core/README.md)

### agi4-schema (JSON Types)

**Output Types** — Serializable with serde:
- `VerdictOutput` — Top-level verdict envelope
- `ConjunctReport` — Per-conjunct status, evidence, margins
- `EvidenceReport` — Source, measurement, value, threshold, provenance
- `ConsistencyCheckOutput` — Consistency check result

**Schema Management** — JSON Schema generation via schemars, with CI drift detection.

See [`crates/agi4-schema/README.md`](./crates/agi4-schema/README.md)

### agi4-adapters (Upstream Source Adapters)

**Nine Adapters** (v0.1.0 ref + v0.1.1 full):
- `metr` — Autonomous agency (80%-time horizon)
- `arc_prize` — Generality (ARC-AGI-2, ARC-AGI-3)
- `hle` — Generality (Humaneval aggregates)
- `gpqa_diamond` — Generality (GPQA Diamond)
- `gdpval` — Economic substitutability (GDPval, GDPval-AA)
- `rli` — Economic substitutability (Reinforced Learning Index)
- `apex_agents` — Economic substitutability (APEX agents)
- `osworld` — Environmental transfer (OSWorld)
- `re_bench` — Autonomous agency (RE-Bench)
- `swe_bench` — Autonomous agency (SWE-bench Verified pass@5)

**Architecture** — Source trait (pure) + Fetcher trait (I/O abstract):
```rust
pub trait Source {
    fn parse(&self, raw: &str) -> Result<Self::Raw, AdapterError>;
    fn to_evidence(&self, raw: Self::Raw, model: &ModelId) -> Result<Vec<Evidence>, AdapterError>;
}

pub trait Fetcher {
    fn fetch(&self, url: &Url) -> Result<String, Self::Error>;
}
```

**Testing** — Fixture-based with frozen upstream snapshots in `tests/fixtures/<adapter>/`.

See [`crates/agi4-adapters/README.md`](./crates/agi4-adapters/README.md)

### agi4-report (Markdown Rendering)

**Output** — Human-readable Markdown from `VerdictOutput`:
- Evidence tables (source, measurement, value, threshold, status)
- Margin analysis (min/max for consistency bounds)
- Consistency check details
- Verdict summary with reasons
- Known gaps acknowledgments

See [`crates/agi4-report/README.md`](./crates/agi4-report/README.md)

## Build & Test

```bash
# Build all crates
cargo build --release

# Test all crates (265+ tests)
cargo test --workspace

# Test single crate
cargo test -p agi4-core --lib

# Format check (CI gate)
cargo fmt --check

# Lint (CI gate, denies unwrap outside tests)
cargo clippy --all-targets --all-features -- -D warnings

# SemVer check for agi4-core (CI gate)
cargo semver-checks -p agi4-core

# Security audit
cargo audit
cargo deny check
```

## Versioning

- **agi4** — MAJOR/MINOR/PATCH tracks SPEC.md SemVer exactly (facade version = spec version)
- **agi4-core** — Independent MAJOR/MINOR/PATCH; MAJOR bump on verdict semantics change
- **agi4-schema**, **agi4-adapters**, **agi4-report** — Independent versioning

## CI Pipeline

[`.github/workflows/ci.yml`] enforces:
- `cargo fmt --check` — Code formatting
- `cargo clippy` — Linting (no unwrap outside tests)
- `cargo test --workspace` — All test suites
- `cargo semver-checks -p agi4-core` — SemVer compatibility
- `cargo audit` + `cargo deny check` — Security
- Schema drift check — JSON schema matches types
- Adapter fixture validation — All 9+ adapters have fixtures + tests
- Spec-to-code traceability — Every threshold constant referenced

## Key Design Principles

1. **Hexagonal / Ports & Adapters** — Core is pure (no I/O); all I/O lives in adapters
2. **Parse-Don't-Validate** — Each source has typed schema; bad data fails at boundary
3. **Illegal States Unrepresentable** — Conjunct/verdict are exhaustive enums
4. **Mechanical Verdict** — Pure, total function (same inputs → same outputs, no panics)
5. **Spec-to-Code Traceability** — Every threshold has a named constant in `threshold.rs`

See [`ARCHITECTURE.md`](../ARCHITECTURE.md) for detailed design rationale.

## How to Contribute

1. **Spec changes** — Require verdict-impact analysis on 3 frontier models
2. **New adapters** — Ship with frozen fixtures + round-trip tests
3. **Verdict logic changes** — Triggers `cargo semver-checks` (MAJOR bump if threshold/conjunct changes)
4. **CI modifications** — Document rationale in terms of design principles

See [`CLAUDE.md`](../CLAUDE.md) and [`TODO.md`](../TODO.md) for current roadmap.

## Related Documentation

- [`SPEC.md`](../SPEC.md) — Specification (thresholds, sources, consistency rules, known gaps)
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — Design principles, crate layout, error handling
- [`README.md`](../README.md) — Project overview, scope, citing, contributing gate
- [`Plans.md`](../Plans.md) — Task tracking for v0.1.0 → v0.2.0
