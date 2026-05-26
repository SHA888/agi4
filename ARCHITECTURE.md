# agi4 Runner — Architecture

**Scope:** The Rust reference implementation that ingests publicly available upstream benchmark data, applies `SPEC.md` mechanically, and emits a schema-validated JSON verdict.

**Non-scope:** Running benchmarks. Producing capability measurements. Hosting infrastructure. Anything not directly required to ingest, normalize, evaluate, and emit.

---

## 1. Design principles

The runner is the implementation of a spec, not a product. Five principles control every design decision:

1. **Hexagonal / Ports & Adapters.** The verdict core is pure: it takes a typed `EvaluationInput`, returns a typed `Verdict`. All I/O (HTTP fetches, file reads, JSON serialization) lives in adapters at the edge. The core has no dependencies on the network, the filesystem, or the clock.
2. **Parse-Don't-Validate at every boundary.** Each upstream source has its own adapter with its own typed schema. Bad upstream data fails at the adapter boundary with a typed error. The core never sees unstructured input.
3. **Make-Illegal-States-Unrepresentable.** Conjunct status and verdict are exhaustive enums. The compiler refuses to forget a branch. Combinations forbidden by the spec are unrepresentable in the type system, not just guarded by runtime checks.
4. **Mechanical verdict.** The verdict function is pure and total: same inputs, same outputs, no side effects, no panics on valid input. Reproducible to the bit given spec version and ingested data.
5. **Spec-to-code traceability.** Every threshold, floor, and rule in `SPEC.md` has a named constant or function in the runner. A reader can grep the codebase for any spec value and find exactly one definition site.

---

## 2. Crate layout (Cargo workspace)

```
runner/
├── Cargo.toml                          # workspace manifest
├── crates/
│   ├── agi4/                           # facade: lib + bin; version tracks SPEC.md
│   │   ├── src/
│   │   │   ├── lib.rs                  # curated re-exports
│   │   │   └── main.rs                 # CLI entrypoint
│   │   └── Cargo.toml
│   ├── agi4-core/                      # pure verdict logic, no I/O
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── conjunct.rs             # ConjunctStatus, Conjunct enum
│   │   │   ├── evidence.rs             # Evidence, SourceValue types
│   │   │   ├── threshold.rs            # spec thresholds as constants
│   │   │   ├── consistency.rs          # cross-conjunct consistency check
│   │   │   └── verdict.rs              # Verdict, verdict() function
│   │   └── Cargo.toml
│   ├── agi4-schema/                    # JSON schema + serde types for output
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   ├── agi4-adapters/                  # upstream source adapters
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── arc_prize.rs            # ARC-AGI-2, ARC-AGI-3
│   │   │   ├── metr.rs                 # METR time horizon
│   │   │   ├── epoch.rs                # Epoch AI benchmark hub
│   │   │   ├── gdpval.rs               # GDPval / GDPval-AA
│   │   │   ├── hle.rs                  # Humanity's Last Exam
│   │   │   ├── gpqa.rs                 # GPQA Diamond
│   │   │   ├── osworld.rs
│   │   │   ├── rli.rs                  # Remote Labor Index
│   │   │   ├── apex.rs                 # APEX-Agents
│   │   │   ├── rebench.rs
│   │   │   ├── swebench.rs
│   │   │   └── source.rs               # Source trait, common error type
│   │   └── Cargo.toml
│   └── agi4-report/                    # Markdown report renderer
│       ├── src/lib.rs
│       └── Cargo.toml
└── tests/                              # workspace-level integration tests
    ├── fixtures/                       # frozen upstream data snapshots
    └── verdict_invariants.rs
```

**Crate roles:**
- `agi4` — the facade. Library + binary in one crate. Its `lib.rs` re-exports a curated public API. Its `main.rs` is the CLI. Version tracks `SPEC.md`. `cargo install agi4` installs the binary; `cargo add agi4` consumes the library.
- `agi4-core` — load-bearing verdict logic. Zero external dependencies beyond `serde` and `thiserror`. SemVer enforced strictly with `cargo-semver-checks`.
- `agi4-schema` — output JSON types and schema. Independent versioning so downstream consumers can depend on the schema without pulling adapters.
- `agi4-adapters` — per-source ingestion. Depends on `agi4-core` to construct typed `Evidence`. The dependency arrow points inward only.
- `agi4-report` — Markdown rendering. Depends on `agi4-schema`.

**Versioning policy:** independent versions per crate. The `agi4` facade version equals the `SPEC.md` version exactly. Library crates bump on their own changes.

---

## 3. The core types

The type system encodes the spec. These types live in `agi4-core`.

```rust
// Conjunct identity. Exhaustive.
pub enum Conjunct {
    Generality,
    EconomicSubstitutability,
    EnvironmentalTransfer,
    AutonomousAgency,
}

// Per-conjunct status. Exhaustive.
pub enum ConjunctStatus {
    Pass,
    Partial,
    Fail,
    InsufficientData,
}

// Top-level verdict. Exhaustive.
pub enum Verdict {
    Attested,
    NotAttested { reasons: Vec<NotAttestedReason> },
    InsufficientData { missing: Vec<MissingEvidence> },
}

// Why a verdict is not_attested. Exhaustive.
pub enum NotAttestedReason {
    ConjunctNotPassing { conjunct: Conjunct, status: ConjunctStatus },
    ConsistencyCheckFailed { sub_rule: ConsistencyRule, detail: String },
}

// Evidence ingested from one upstream source for one measurement.
pub struct Evidence {
    pub source: SourceId,
    pub measurement: MeasurementId,
    pub value: SourceValue,
    pub reliability_percentile: ReliabilityPercentile,
    pub provenance: Provenance,
}

// The value type is bounded — no arbitrary floats outside [0, 1] or
// time horizons outside [0, T_MAX]. Constructors validate; bad data
// fails at the adapter boundary, never in the core.
pub enum SourceValue {
    Fraction(BoundedFraction),       // 0.0..=1.0
    Hours(NonNegativeHours),         // 0.0..=T_MAX
}

pub struct Provenance {
    pub source_url: Url,
    pub fetch_timestamp: DateTime<Utc>,
    pub source_version: Option<String>,
    pub raw_value: String,           // the verbatim string as ingested
}
```

The `Verdict` enum is the load-bearing piece. Once it is returned by `verdict()`, no further logic exists. Serialization in `agi4-schema` is the only thing that happens between the verdict and the output JSON.

---

## 4. The adapter pattern

Every upstream source implements one trait:

```rust
pub trait Source {
    type Raw: DeserializeOwned;            // the source's native schema
    type Error: std::error::Error + Send + Sync + 'static;

    /// Stable identifier for the source. Matches SourceId in agi4-core.
    fn id(&self) -> SourceId;

    /// The URL or endpoint the adapter ingests from.
    fn endpoint(&self) -> &Url;

    /// Parse raw upstream data into the typed schema for this source.
    /// Fails closed on any malformed input.
    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error>;

    /// Convert validated raw data into agi4-core Evidence values.
    /// One source may produce evidence for multiple conjuncts
    /// (ARC-AGI-3 contributes to both Generality and EnvironmentalTransfer).
    fn to_evidence(&self, raw: Self::Raw, model: &ModelId)
        -> Result<Vec<Evidence>, Self::Error>;
}
```

**Three properties of this trait:**

1. The associated `Raw` type is per-adapter. METR's schema is not ARC Prize's schema. The runner does not have a "common upstream schema" — there is no such thing. Each adapter owns its own typed view.
2. `parse` and `to_evidence` are pure. I/O (HTTP, file read) is done by a separate `Fetcher` abstraction injected at the CLI layer. Adapters can be unit-tested against frozen JSON fixtures with zero network.
3. `to_evidence` returns `Vec<Evidence>` because one source can contribute to multiple conjuncts. ARC-AGI-3 is the canonical case. The adapter, not the core, knows which conjuncts a source feeds.

**Adapter testing:** every adapter ships with `tests/fixtures/*.json` containing real frozen upstream data. The adapter's parse + to_evidence round-trip is asserted against these fixtures. Upstream schema drift fails CI immediately and visibly.

---

## 5. The verdict pipeline

The pipeline is a straight line. Each stage is pure, typed, and individually testable.

```
       ┌─────────────────────────────────────────────────────────┐
       │                  agi4 (facade, bin)                     │
       │  parses args, drives Fetcher, dispatches to adapters    │
       └────────────────────────┬────────────────────────────────┘
                                │
                                ▼
       ┌─────────────────────────────────────────────────────────┐
       │                     agi4-adapters                       │
       │  per-source: fetch → parse → typed Raw → Vec<Evidence>  │
       └────────────────────────┬────────────────────────────────┘
                                │   Vec<Evidence>
                                ▼
       ┌─────────────────────────────────────────────────────────┐
       │                      agi4-core                          │
       │                                                         │
       │  1. group_by_conjunct(Vec<Evidence>)                    │
       │       -> HashMap<Conjunct, Vec<Evidence>>               │
       │                                                         │
       │  2. evaluate_conjunct(Conjunct, &[Evidence])            │
       │       -> ConjunctEvaluation { status, margins, ... }    │
       │     (one function per conjunct, threshold constants     │
       │      pulled from threshold.rs)                          │
       │                                                         │
       │  3. consistency_check(&[ConjunctEvaluation; 4])         │
       │       -> ConsistencyResult                              │
       │                                                         │
       │  4. verdict(&[ConjunctEvaluation; 4], ConsistencyResult)│
       │       -> Verdict                                        │
       └────────────────────────┬────────────────────────────────┘
                                │   Verdict
                                ▼
       ┌─────────────────────────────────────────────────────────┐
       │                     agi4-schema                         │
       │   serialize Verdict + provenance to JSON, schema-check  │
       └────────────────────────┬────────────────────────────────┘
                                │
                                ▼
       ┌─────────────────────────────────────────────────────────┐
       │                     agi4-report                         │
       │   render JSON to Markdown, link provenance              │
       └─────────────────────────────────────────────────────────┘
```

**Two invariants enforced by types:**

1. `evaluate_conjunct` returns `ConjunctEvaluation`, not bare `ConjunctStatus`. The evaluation carries the margins used, which `consistency_check` needs for the 0.5× margin rule (`SPEC.md` §4). The core cannot lose this information between stages.
2. `verdict` takes `&[ConjunctEvaluation; 4]` — a fixed-size array, not a `Vec`. There are exactly four conjuncts. The type enforces it. A future MAJOR spec bump that changes the conjunct count is a type-level change, not a runtime check.

---

## 6. Threshold representation

Every threshold and floor in `SPEC.md` §3 lives in `agi4-core/src/threshold.rs` as a named `const`. No magic numbers in evaluation logic.

```rust
pub mod generality {
    pub const ARC_AGI_2_PASS: f64        = 0.85;
    pub const ARC_AGI_3_PASS: f64        = 0.50;
    pub const ARC_AGI_3_FLOOR: f64       = 0.05;   // fluid-generality floor
    pub const HLE_PASS: f64              = 0.80;
    pub const GPQA_DIAMOND_PASS: f64     = 0.90;
}

pub mod economic_substitutability {
    pub const GDPVAL_PASS: f64           = 0.85;
    pub const RLI_PASS: f64              = 0.60;
    pub const RLI_FLOOR: f64             = 0.10;
    pub const APEX_AGENTS_PASS: f64      = 0.75;
}

pub mod environmental_transfer {
    pub const ARC_AGI_3_PASS: f64        = 0.50;   // cross-listed
    pub const ARC_AGI_3_FLOOR: f64       = 0.05;   // cross-listed
    pub const OSWORLD_PASS: f64          = 0.85;
    // NES thresholds: TBD in v0.1.x
}

pub mod autonomous_agency {
    pub const METR_80PCT_PASS_HOURS: f64     = 168.0;   // one work-week
    pub const METR_80PCT_FLOOR_HOURS: f64    = 8.0;     // one workday
    pub const REBENCH_PASS: f64              = 0.60;
    pub const SWEBENCH_VERIFIED_PASS_AT_5: f64 = 0.85;
}

pub mod consistency {
    pub const MARGIN_VARIANCE_RATIO: f64 = 0.5;
}
```

**Why constants, not config files:** Threshold values are part of the spec. Changing them is a SemVer event, not a runtime configuration. Putting them in a YAML or TOML file would let operators silently change verdict semantics without a version bump. The constants live in source, are SemVer-tracked, and ship with the compiled binary.

---

## 7. JSON output schema

The output schema is defined once in `agi4-schema` and validated on every emission. The runner refuses to emit malformed JSON.

```json
{
  "spec_version": "0.1.0",
  "runner_version": "0.1.0",
  "run_timestamp": "2026-06-30T00:00:00Z",
  "model": {
    "id": "example-model-v1",
    "provider": "example-lab",
    "version_or_date": "2026-06-15"
  },
  "conjuncts": {
    "generality": {
      "status": "partial",
      "evidence": [
        {
          "source": "arc-agi-3",
          "measurement": "interactive-private-pass-at-1",
          "value": { "type": "fraction", "value": 0.01 },
          "threshold": 0.50,
          "floor": 0.05,
          "passes_threshold": false,
          "below_floor": true,
          "reliability_percentile": 80,
          "provenance": {
            "source_url": "https://arcprize.org/leaderboard",
            "fetch_timestamp": "2026-06-30T00:00:00Z",
            "source_version": "ARC-AGI-3 v1.0",
            "raw_value": "0.01"
          }
        }
      ],
      "margins": { "min": -0.49, "max": 0.05 }
    },
    "economic_substitutability": { "...": "..." },
    "environmental_transfer": { "...": "..." },
    "autonomous_agency": { "...": "..." }
  },
  "consistency_check": {
    "status": "fail",
    "failed_rules": ["margin_variance_ratio"],
    "detail": "min/max margin ratio = 0.12, below required 0.5"
  },
  "verdict": "not_attested",
  "verdict_reasons": [
    {
      "type": "conjunct_not_passing",
      "conjunct": "generality",
      "status": "partial"
    }
  ],
  "known_gaps_acknowledged": [
    "nes_underspecified",
    "no_non_verifiable_domain_agency_measurement",
    "partial_multimodal_coverage"
  ]
}
```

**Why `known_gaps_acknowledged` is in the output, not just in the spec:** Every verdict must carry its own confidence bounds. A consumer reading a verdict cannot be assumed to have also read `SPEC.md`. Embedding the gaps in the verdict itself enforces honest reporting.

---

## 8. Error handling discipline

Three error tiers, each handled differently:

1. **Adapter-level errors (network failure, malformed upstream JSON, schema drift).** Typed `thiserror` enums per adapter. Logged with full context. Cause the conjunct to receive `insufficient_data` for the affected source. Do not abort the run.
2. **Core-level errors (impossible — by type construction).** The verdict function is total. There is no `Result` return type on `verdict()`. If a panic occurs in the core, it is a bug, not a runtime condition.
3. **Output-level errors (schema validation failure, write failure).** Aborts the run with a non-zero exit code. The runner refuses to emit a malformed verdict.

**No `unwrap()` outside of tests.** Enforced by `clippy::unwrap_used` denied in CI.

---

## 9. CI enforcement of design principles

Per the meta-rule, every principle has a mechanism. The runner's CI pipeline:

| Principle | Mechanism |
|---|---|
| Format | `cargo fmt --check` |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` |
| Type discipline | compiler (default) |
| No unwrap | `clippy::unwrap_used` denied |
| SemVer on agi4-core | `cargo semver-checks` on every PR touching agi4-core |
| Unit tests | `cargo test --workspace` |
| Adapter fixture round-trips | `cargo test -p agi4-adapters --tests` |
| Schema validation | `cargo test -p agi4-schema --tests` (round-trip JSON against schema) |
| Verdict invariants | property tests via `proptest` on `agi4-core::verdict` |
| Spec-to-code traceability | grep check: every threshold constant must be referenced in at least one evaluation function (script in CI) |
| Security | `cargo audit` and `cargo deny` on every PR |
| Reproducibility | `Cargo.lock` committed; CI runs are pinned to a Rust toolchain version |

The verdict-invariants tests are load-bearing. They assert:

- The verdict function is total (no panics on any valid `[ConjunctEvaluation; 4]` × `ConsistencyResult` combination).
- `Verdict::Attested` is only returned when all four conjuncts are `Pass` and consistency is `Pass`.
- `Verdict::InsufficientData` is never returned when any conjunct is `Fail`.
- The verdict table in `SPEC.md` §5 matches the code's behavior exhaustively. Generated tests cover all `4^4 × 2` input combinations.

---

## 10. What's deferred to v0.2.0

- The NES (Novel-Environment Subset) adapter specification.
- A non-verifiable-domain agency adapter (no public benchmark exists yet).
- Multimodal adapter expansion.
- Adversarial regeneration acceptance criteria for upstream sources.
- A web-facing verdict viewer (if at all — possibly out of scope permanently, since the spec's purpose is the verdict, not the UI).

Each of these has a corresponding entry in `TODO.md`.

---

## 11. What's intentionally not here

- A plugin system for third-party adapters. Adapters must live in-tree so that schema changes are visible in PRs and traceable through SemVer. A plugin system would erode this discipline.
- A configurable threshold mechanism. Per §6, this is by design.
- A daemon mode or scheduler. The runner is invoked, runs, exits. Scheduling is an operator concern (cron, GitHub Actions, etc.), not a runner concern.
- A persistence layer. The runner is stateless. Historical verdicts are stored by the operator (commit JSON outputs to a repo, for instance).
