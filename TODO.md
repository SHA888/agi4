# agi4 — Roadmap

**Format:** Atomic tasks under SemVer-tagged milestones. Each task is sized to a single PR. Tasks are checkboxed for tracking; subtasks are dash-bulleted for scope.

**Versioning policy (recap):**
- Independent versions per crate.
- The `agi4` facade crate version matches the `SPEC.md` version exactly.
- `cargo install agi4@X.Y.Z` ⇔ runner that implements spec `X.Y.Z`.

**SDLC discipline:**
- Every task lands behind a passing CI pipeline (fmt, clippy, test, semver-check, audit, deny).
- Every spec-affecting task updates `SPEC.md` and the corresponding test in the same PR.
- No PR without a verdict-impact analysis where the spec changes (per README contribution gate).

---

## Milestone v0.1.0 — Scaffold and first verdict

**Goal:** Repository initialized, license files in place, all five crates compile, the verdict pipeline runs end-to-end against frozen fixture data for one model, emits valid JSON. No real upstream fetches yet.

### Repository initialization

- [ ] **Initialize workspace.**
  - Create `Cargo.toml` workspace manifest with the five member crates.
  - Pin Rust toolchain via `rust-toolchain.toml` (latest stable at init).
  - Commit `.gitignore`, `.editorconfig`.
- [ ] **Commit license files verbatim.**
  - Fetch CC BY 4.0 legal code from `creativecommons.org` and commit as `LICENSE-SPEC`.
  - Fetch Apache-2.0 legal code from `apache.org` and commit as `LICENSE-APACHE`.
  - Commit `LICENSE-MIT` from the text in Stage 1.
  - Commit `NOTICE` from Stage 1.
- [ ] **Commit Stage 1–4 documents.**
  - `README.md`, `SPEC.md`, `ARCHITECTURE.md`, `TODO.md`.
  - Verify all cross-references (README → SPEC sections, SPEC → ARCHITECTURE concepts) resolve.

### `agi4-core` crate — the verdict logic

- [ ] **Define core enums.**
  - `Conjunct`, `ConjunctStatus`, `Verdict`, `NotAttestedReason`, `MissingEvidence`, `ConsistencyRule`.
  - Derive `Debug`, `Clone`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`.
- [ ] **Define `Evidence` and bounded value types.**
  - `BoundedFraction` (0.0..=1.0, validated constructor).
  - `NonNegativeHours` (0.0..=T_MAX, validated constructor).
  - `SourceValue`, `Provenance`, `Evidence`.
- [ ] **Define `SourceId` and `MeasurementId`.**
  - String-newtype pattern. Stable identifiers matching SPEC.md §2 tables.
- [ ] **Encode thresholds as constants.**
  - Per the `threshold.rs` module structure in ARCHITECTURE §6.
  - One constant per spec value. No magic numbers in evaluation logic.
- [ ] **Implement per-conjunct evaluation functions.**
  - `evaluate_generality(&[Evidence]) -> ConjunctEvaluation`
  - `evaluate_economic_substitutability(&[Evidence]) -> ConjunctEvaluation`
  - `evaluate_environmental_transfer(&[Evidence]) -> ConjunctEvaluation`
  - `evaluate_autonomous_agency(&[Evidence]) -> ConjunctEvaluation`
  - Each returns status + margins + per-source pass/fail/below-floor flags.
- [ ] **Implement consistency check.**
  - `consistency_check(&[ConjunctEvaluation; 4]) -> ConsistencyResult`.
  - All three sub-rules from SPEC.md §4 implemented.
- [ ] **Implement verdict function.**
  - `verdict(&[ConjunctEvaluation; 4], ConsistencyResult) -> Verdict`.
  - Total function. No `Result`. No panics on any input.
- [ ] **Verdict invariant tests.**
  - Property test with `proptest`: verdict is total.
  - Exhaustive test: `4^4 × 2 = 512` input combinations enumerated and asserted against the SPEC.md §5 verdict table.
  - Test: `Verdict::Attested` only when all four `Pass` and consistency `Pass`.
  - Test: `Verdict::InsufficientData` never returned when any conjunct is `Fail`.
- [ ] **SemVer baseline.**
  - Add `cargo-semver-checks` to CI for `agi4-core` only.
  - Tag baseline at v0.1.0.

### `agi4-schema` crate — output JSON schema

- [ ] **Define output types matching ARCHITECTURE §7.**
  - `VerdictOutput`, `ConjunctReport`, `EvidenceReport`, `ConsistencyReport`, `ModelMeta`.
- [ ] **Round-trip serialization tests.**
  - Construct a representative `VerdictOutput`, serialize, deserialize, assert equality.
- [ ] **JSON schema export.**
  - Generate JSON Schema (Draft 2020-12) from the Rust types using `schemars`.
  - Commit the generated schema to `/schema/agi4-output-v0.1.0.json`.
  - CI step: regenerate schema and fail if it has drifted from the committed file.
- [ ] **Known-gaps embedding.**
  - Hardcoded list of v0.1.0 known gaps from SPEC.md §8.
  - Every emitted verdict includes the gaps list.

### `agi4-adapters` crate — Source trait, no real adapters yet

- [ ] **Define `Source` trait per ARCHITECTURE §4.**
  - Associated types: `Raw`, `Error`.
  - Methods: `id`, `endpoint`, `parse`, `to_evidence`.
- [ ] **Define common error type and `Fetcher` abstraction.**
  - `Fetcher` trait for HTTP I/O, with a default `reqwest`-based implementation behind a feature flag.
  - In-memory `Fetcher` for tests.
- [ ] **One reference adapter: `metr` (the simplest schema).**
  - Implement `Source` for METR time horizon.
  - Frozen fixture: a real recent METR JSON snapshot in `tests/fixtures/metr/`.
  - Round-trip test: parse fixture → emit `Vec<Evidence>` → assert evidence values match expected.
- [ ] **CI step: adapter fixture validation.**
  - Every adapter must have at least one fixture and one round-trip test.

### `agi4-report` crate — Markdown rendering

- [ ] **Implement `render(&VerdictOutput) -> String`.**
  - Header: model, spec version, run timestamp, verdict.
  - Per-conjunct section: status, evidence table, margins.
  - Consistency section: status, sub-rule outcomes.
  - Provenance section: every source URL and fetch timestamp.
  - Known gaps section: reproduced from the verdict's `known_gaps_acknowledged`.
- [ ] **Snapshot test.**
  - Render a fixture `VerdictOutput`, assert the Markdown matches a committed golden file.

### `agi4` facade crate — library + binary

- [ ] **Create `agi4` crate with `[lib]` and `[[bin]]` targets.**
  - Version: `0.1.0` (matches spec).
- [ ] **Curated public API in `lib.rs`.**
  - Re-export from `agi4-core`: `Conjunct`, `ConjunctStatus`, `Verdict`, `Evidence`, `SourceValue`, `Provenance`, threshold constants module.
  - Re-export from `agi4-schema`: `VerdictOutput` and child types.
  - Do not re-export `agi4-adapters` internals or `agi4-report` internals.
- [ ] **CLI binary in `main.rs`.**
  - Subcommands (using `clap`):
    - `agi4 attest --model <id> --fixture <path>` — runs the pipeline against a local fixture directory, emits JSON to stdout.
    - `agi4 attest --model <id> --live` — runs the pipeline against live upstream sources. *Stubbed in v0.1.0 — returns error "live attestation not yet wired."*
    - `agi4 render --input <path>` — reads a verdict JSON, renders Markdown to stdout.
    - `agi4 schema` — prints the output JSON schema.
    - `agi4 version` — prints crate version and matching spec version.
- [ ] **End-to-end test.**
  - `cargo run -- attest --model example --fixture tests/fixtures/example/ | jq .` produces valid JSON conforming to the committed schema.

### CI pipeline

- [ ] **Add `.github/workflows/ci.yml` (or equivalent) per ARCHITECTURE §9.**
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check` (licenses, bans, advisories)
  - `cargo audit`
  - `cargo semver-checks` (on `agi4-core` only initially)
- [ ] **Pin clippy lints in `clippy.toml`.**
  - Deny `unwrap_used`, `expect_used` (outside tests).
- [ ] **Schema-drift check.**
  - CI step regenerates `/schema/agi4-output-v0.1.0.json` and fails if it differs from committed.
- [ ] **Spec-to-code grep check.**
  - Script that asserts every constant in `threshold.rs` is referenced in at least one evaluation function.
- [ ] **Reproducibility.**
  - Commit `Cargo.lock`.
  - CI uses the pinned toolchain.

### v0.1.0 release gate

- [ ] All tasks above complete.
- [ ] `cargo install --path crates/agi4` succeeds locally.
- [ ] `agi4 attest --model example --fixture tests/fixtures/example/` produces a valid JSON verdict.
- [ ] `agi4 render --input <that JSON>` produces a readable Markdown report.
- [ ] Tag `v0.1.0` on the `agi4` crate. Other crates tagged at their own current versions.
- [ ] Publish to crates.io: `agi4-core`, `agi4-schema`, `agi4-adapters`, `agi4-report`, `agi4`.

---

## Milestone v0.1.1 — Real adapters, first live attestation

**Goal:** Implement adapters for every upstream source named in SPEC.md §2. Run the first live attestation against a real, publicly known frontier model. No spec changes.

### Adapter implementation

- [ ] **Adapter: ARC Prize (ARC-AGI-2, ARC-AGI-3).**
  - Single adapter, emits evidence for both Generality and EnvironmentalTransfer.
  - Frozen fixture from the ARC Prize public leaderboard.
- [ ] **Adapter: HLE.**
- [ ] **Adapter: GPQA Diamond.**
  - Via Epoch AI benchmark hub if direct source unavailable.
- [ ] **Adapter: GDPval / GDPval-AA.**
  - Prefer Artificial Analysis's third-party implementation for verifiability.
- [ ] **Adapter: RLI (Remote Labor Index).**
- [ ] **Adapter: APEX-Agents.**
- [ ] **Adapter: OSWorld.**
- [ ] **Adapter: RE-Bench.**
- [ ] **Adapter: SWE-bench Verified (pass@5).**
  - Note: most published results are pass@1. The adapter must reject pass@1-only data and return `insufficient_data` for that source.

### Live attestation

- [ ] **Wire `agi4 attest --live` to actually fetch.**
  - Concurrent fetch via `tokio` + `reqwest`.
  - Per-source timeout and retry policy.
  - Cache layer (local filesystem) to avoid hammering upstream sources.
- [ ] **First public verdict.**
  - Pick one frontier model with sufficient public upstream data.
  - Run live attestation.
  - Commit the resulting JSON + Markdown report to `/attestations/v0.1.0/<model>-<date>.{json,md}`.
- [ ] **README update.**
  - Add a "First attestations" section linking to the committed verdicts.

### v0.1.1 release gate

- [ ] All adapters implemented with fixtures and round-trip tests.
- [ ] At least one live attestation committed for a current frontier model.
- [ ] No spec changes from v0.1.0.
- [ ] `agi4` facade crate stays at `0.1.0` (spec unchanged). Individual library crates bump per their own changes.

---

## Milestone v0.1.2 — Calibration based on first attestation

**Goal:** Refine thresholds based on what the first live attestations reveal. This is a calibration patch — spec adjustments only, no architectural changes.

- [ ] **Run attestation against three to five frontier models.**
  - Commit each verdict.
  - Tabulate where thresholds are too loose or too tight relative to the diagnostic intent.
- [ ] **Spec calibration PR.**
  - Per the README contribution gate, include verdict-impact analysis on the attested models.
  - Adjust thresholds in SPEC.md §3 as needed.
  - Bump `agi4` facade to `0.1.2`.
  - Update threshold constants in `agi4-core`.
- [ ] **Re-attest the same models under v0.1.2.**
  - Commit new verdicts alongside the old.
  - Document threshold movement and verdict deltas in `CHANGELOG.md`.

### v0.1.2 release gate

- [ ] At least three live attestations under v0.1.2.
- [ ] Threshold movements documented with rationale.
- [ ] No new upstream sources (defer to v0.1.3).

---

## Milestone v0.1.3 — Gap closure (NES specification)

**Goal:** Close the largest known gap from SPEC.md §8: NES (Novel-Environment Subset).

- [ ] **Survey candidate benchmarks for NES.**
  - Interactive environments with no training-time analogues.
  - Candidates: ARC-AGI-3 (already used), held-out subset of OSWorld, newer interactive benchmarks released since v0.1.0.
- [ ] **Define NES acceptance criteria in SPEC.md.**
  - What makes a benchmark eligible for inclusion in NES.
  - Refresh policy when an environment is no longer novel.
- [ ] **Implement NES adapter(s).**
- [ ] **Spec bump.**
  - MINOR bump (`agi4` to `0.2.0`? or `0.1.3` if backward-compatible — decide based on whether NES becomes required evidence or remains supplementary).
- [ ] **Re-attest existing models under the new spec.**

### v0.1.3 release gate

- [ ] NES specified and at least one NES adapter implemented.
- [ ] Re-attestation of prior models complete.
- [ ] CHANGELOG documents whether environmental transfer conjunct now requires NES.

---

## Milestone v0.2.0 — First stable threshold set

**Goal:** Lock the calibration phase. v0.2.0 commits to a stable threshold set that will only change via explicit MAJOR bumps thereafter.

- [ ] **Review all v0.1.x calibration deltas.**
  - Document the calibration journey in `CHANGELOG.md`.
- [ ] **Lock thresholds for v0.2.0.**
  - SPEC.md §3 values frozen.
  - Any future threshold change is a MAJOR bump (v0.3.0+) per SemVer policy.
- [ ] **Address known gaps where possible.**
  - Multimodal coverage: at least one multimodal adapter (vision or audio).
  - Non-verifiable-domain agency: if a public benchmark exists by v0.2.0, integrate it; otherwise document the continued gap.
  - Adversarial regeneration: explicit acceptance criteria for upstream sources.
- [ ] **Governance documentation.**
  - `GOVERNANCE.md` — maintainer pattern, fork policy, dispute resolution.
  - Explicit non-goal: institutional governance.
- [ ] **Stable JSON schema.**
  - `/schema/agi4-output-v0.2.0.json` — committed and frozen.
  - Future schema changes follow JSON Schema versioning alongside spec SemVer.

### v0.2.0 release gate

- [ ] All v0.1.x calibration absorbed.
- [ ] At least five frontier-model attestations under v0.2.0 published.
- [ ] No open `known_gaps` of the "blocks meaningful attestation" kind.
- [ ] `agi4@0.2.0` published to crates.io.

---

## Cross-cutting tasks (any milestone)

- [ ] **Documentation: per-conjunct rationale.**
  - For each conjunct, write a short doc explaining *why* the selected upstream sources triangulate it.
- [ ] **Documentation: reading a verdict.**
  - A short guide for non-implementers on how to interpret an `agi4` JSON or Markdown verdict.
- [ ] **Public dashboard (optional, low priority).**
  - Static site rendering the most recent attestations.
  - Generated from committed JSON files; no server.
- [ ] **Community signaling.**
  - Open issues with `help-wanted` and `good-first-issue` tags for adapter work.
  - Outreach to ARC Prize, METR, Epoch AI only after v0.1.1 demonstrates working live attestation — never before. (Build → Works → Community.)

---

## Explicitly deferred to v0.3.0+

- Web-facing verdict viewer.
- Historical verdict diffing across spec versions.
- Multi-model batch attestation as a first-class CLI mode.
- Non-English upstream sources (if any emerge).
- Embodied / robotics benchmarks.
- Compute-cost reporting alongside capability verdicts.

---

## Explicitly never in scope

- Plugin system for third-party adapters (per ARCHITECTURE §11).
- Configurable thresholds at runtime (per ARCHITECTURE §6).
- Daemon mode (per ARCHITECTURE §11).
- Hosted attestation service.
- Lab partnerships that grant privileged data access.
- AGI definitions other than AGI/4 (forks welcome; not in-tree).
