# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**agi4** is a public, versioned specification for attesting whether an AI system clears the AGI/4 threshold, plus a reference runner that ingests publicly available upstream benchmark data and emits a mechanical verdict.

**Key constraint:** agi4 is an *integration layer*, not a benchmark. The runner composes existing, independently credible measurements from upstream sources. It does not run evaluations or produce capability measurements.

---

## The Specification

The spec defines AGI/4 as the conjunction of four conditions, all of which must pass:

1. **Generality** — broadly competent across most cognitive tasks humans perform
2. **Economic substitutability** — capable enough to displace humans in most economically valuable work
3. **Environmental transfer** — able to pursue goals across a wide range of novel environments
4. **Autonomous long-horizon agency** — autonomous enough to conduct long-horizon work, including AI research itself, without human scaffolding

The verdict is **strictly conjunctive**: failing any single conjunct fails the entire verdict.

See `SPEC.md` §1–3 for the operationalized upstream sources, minimum evidence requirements, and per-conjunct thresholds.

---

## Architecture & Design Principles

The reference runner is a Rust application structured as a Cargo workspace with five member crates:

- **`agi4`** — facade library and CLI binary. Version tracks `SPEC.md` SemVer exactly.
- **`agi4-core`** — pure, zero-dependency verdict logic. Load-bearing; strict SemVer enforcement via `cargo-semver-checks`.
- **`agi4-schema`** — JSON output types and schema validation. Independent versioning.
- **`agi4-adapters`** — per-upstream-source ingestion adapters implementing the `Source` trait.
- **`agi4-report`** — Markdown verdict rendering.

**Five design principles** (see `ARCHITECTURE.md` §1 for details):

1. **Hexagonal / Ports & Adapters** — verdict core is pure (no I/O, no side effects). All I/O lives in adapters.
2. **Parse-Don't-Validate** — each upstream source has its own typed schema; bad data fails at the adapter boundary.
3. **Make-Illegal-States-Unrepresentable** — conjunct and verdict are exhaustive enums; the compiler enforces completeness.
4. **Mechanical verdict** — the verdict function is pure and total (same inputs → same outputs, no panics on valid input).
5. **Spec-to-code traceability** — every threshold, floor, and rule in `SPEC.md` has a named constant or function in the code.

---

## Versioning & Semantic Changes

- **Independent versioning per crate.** Each crate bumps its version independently based on its own changes.
- **`agi4` facade matches `SPEC.md` exactly** (SemVer). Changes to the spec are changes to the runner's API.
- **MAJOR bump** — conjunct definitions or threshold values change. Verdict semantics change; old verdicts are not directly comparable.
- **MINOR bump** — upstream source added/removed with backward-compatible diagnostic output.
- **PATCH bump** — clarifications, wording fixes, source URL updates. No verdict semantics change.

**Critical discipline:** Every spec-affecting PR must include verdict-impact analysis on at least three publicly known frontier models. See `README.md` "Contributing."

---

## Build & Test Commands

Once the Rust codebase is scaffolded (v0.1.0 tasks), these commands apply:

### Build & Format
```bash
cargo build --release
cargo fmt --check          # format check (required by CI)
cargo fmt                  # auto-format
```

### Lint
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Test
```bash
cargo test --workspace                    # all tests
cargo test -p agi4-core                   # core verdict logic only
cargo test -p agi4-adapters --tests       # adapter fixture round-trips
cargo test -p agi4-schema --tests         # schema round-trip validation
```

### SemVer Check (for `agi4-core` changes)
```bash
cargo semver-checks --package agi4-core
```

### Security Audit
```bash
cargo audit                # check for known vulnerabilities
cargo deny check          # check licenses, bans, advisories
```

### Run the CLI
```bash
cargo run -- attest --model <model-id> --fixture <path>  # local fixture
cargo run -- attest --model <model-id> --live            # live upstream (v0.1.1+)
cargo run -- render --input <verdict.json>               # render to Markdown
cargo run -- schema                                       # print output JSON schema
cargo run -- version                                      # show version and matching spec
```

### Install locally
```bash
cargo install --path crates/agi4
agi4 attest --model example --fixture ./tests/fixtures/example/
```

---

## Key Files & Their Purpose

| File | Purpose |
|------|---------|
| `SPEC.md` | The load-bearing specification. Defines conjuncts, upstream sources, thresholds, floors, consistency check, refresh cadence. **Changes here are changes to verdict semantics.** |
| `ARCHITECTURE.md` | Runner design: hexagonal architecture, crate layout, core types, adapter pattern, verdict pipeline, error handling, CI enforcement. |
| `README.md` | Public-facing overview, design principles, scope boundaries, citing, contributing gate. |
| `TODO.md` | Atomic tasks under SemVer milestones (v0.1.0 scaffold → v0.2.0 stable). |
| `NOTICE` | License scope map and attribution. |

---

## Testing Discipline

The runner enforces testing at multiple levels:

1. **Adapter fixture tests** — every adapter has frozen real upstream-data fixtures in `tests/fixtures/<source>/`. Parse + to_evidence round-trip must match expected evidence values.
2. **Unit tests on core logic** — per-conjunct evaluation, consistency check, verdict function.
3. **Verdict invariant tests** — property tests asserting the verdict function is total (no panics). Exhaustive test covering all `4^4 × 2 = 512` input combinations against the `SPEC.md` §5 verdict table.
4. **Schema round-trip tests** — construct a `VerdictOutput`, serialize, deserialize, assert equality.
5. **CI schema-drift check** — regenerate the JSON schema from types and fail if it has drifted from the committed file.

**No `unwrap()` or `expect()` outside tests.** Enforced by `clippy::unwrap_used` denied in CI.

---

## Making Changes to the Spec

If you propose a change to `SPEC.md`:

1. **Open an issue first** with:
   - The proposed conjunct, threshold, or upstream-source change
   - The rationale
   - Expected verdict impact on at least three currently-public frontier models

2. **In the PR**, include:
   - The `SPEC.md` change
   - The corresponding code change in `agi4-core/src/threshold.rs` (or conjunct evaluation if the structure changes)
   - Updated tests reflecting the new thresholds/rules
   - The verdict-impact analysis (what changes for the three models you analyzed?)

3. **Determine the SemVer bump**:
   - Conjunct redefinition or threshold value change? → **MAJOR** (bump `agi4` facade)
   - New upstream source added? → **MINOR** (bump individual library crates)
   - Wording clarification? → **PATCH**

---

## CI Pipeline

The CI runs on every PR and enforces these checks (see `ARCHITECTURE.md` §9):

- `cargo fmt --check` — code formatting
- `cargo clippy --all-targets --all-features -- -D warnings` — linting (no unwrap outside tests)
- `cargo test --workspace` — all test suites
- `cargo semver-checks -p agi4-core` — SemVer compatibility for core crate
- `cargo audit` — known vulnerability scan
- `cargo deny check` — license, ban, advisory checks
- Adapter fixture round-trips — verify upstream data ingest doesn't break
- Schema drift check — verify JSON schema matches types
- Spec-to-code grep check — every threshold constant is referenced

---

## Known Gaps (v0.1.0)

These are intentional limits, tracked in `SPEC.md` §8 and `TODO.md`:

1. **NES (Novel-Environment Subset) underspecified** — the exact set of novel environments for the environmental transfer conjunct is TBD in v0.1.x.
2. **No non-verifiable-domain agency measurement** — current benchmarks (SWE-bench, RE-Bench) test verifiable domains. The "AI research itself" subclause is not yet measurable via public benchmarks.
3. **Partial multimodal coverage** — several upstream sources are text-first. Vision, audio, embodied benchmarks not yet integrated.
4. **Adversarial regeneration assumed** — the spec assumes upstream sources maintain held-out splits; explicit acceptance criteria TBD in v0.1.x.

Each gap is reflected in the verdict output (`known_gaps_acknowledged` field).

---

## Project Status

- **v0.1.0** — calibration phase. Documentation complete. Runner in scaffold stage. No verdicts issued.
- **v0.1.x** — implement adapters, wire live attestation, refine thresholds based on first verdicts.
- **v0.2.0** — lock stable threshold set, address known gaps where possible, publish multiple attestations.

See `TODO.md` for the detailed roadmap.

---

## Contributing

- **Spec changes** require verdict-impact analysis (see "Making Changes to the Spec" above).
- **Adapter implementations** must ship with frozen upstream-data fixtures and round-trip tests.
- **New CI checks** must be justified in terms of the five design principles.
- **Disagreements on verdict semantics** are resolved by forking the spec, not by litigating the measurement. This is a feature.

Maintainer pattern is single-maintainer for v0.x with an open issue tracker.

---

## Quick Reference: Threshold Constants

All thresholds live in `agi4-core/src/threshold.rs` as named `const` values, one per `SPEC.md` value. They are load-bearing for `ARCHITECTURE.md` principle #5 (spec-to-code traceability). Examples:

```
generality::ARC_AGI_2_PASS = 0.85
generality::ARC_AGI_3_FLOOR = 0.05
economic_substitutability::GDPVAL_PASS = 0.85
autonomous_agency::METR_80PCT_PASS_HOURS = 168.0
consistency::MARGIN_VARIANCE_RATIO = 0.5
```

Every one of these must appear in at least one evaluation function in the code. The CI grep-checks this.

---

## Commit Message Guidelines

**Do not include `Co-Authored-By:` trailers in commit messages.** This applies to all assistant-generated commits, including those produced by Claude Code or any other AI tool. Commit attribution stays with the human author. Boilerplate trailers add noise to the history without conveying meaningful authorship and have been retroactively stripped from past commits.

---

## Documentation Language

**Plans.md and all project planning documents must be written in English only.** This ensures clarity and accessibility for all contributors and future reviewers.
