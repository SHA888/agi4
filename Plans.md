# agi4 Plans

Project task tracking for AGI/4 attestation protocol reference runner.

**Created:** 2026-05-26
**Current Phase:** v0.1.0 (Scaffold and first verdict)

---

## Phase 1: v0.1.0 — Scaffold and First Verdict

**Goal:** Repository initialized, all five crates compile, verdict pipeline runs end-to-end against frozen fixture data, emits valid JSON. No real upstream fetches yet.

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 1.1 | Initialize Cargo workspace with five member crates | `cargo build` succeeds, `Cargo.toml` manifest exists | - | cc:done [f9ae554] |
| 1.2 | Pin Rust toolchain via `rust-toolchain.toml` | Toolchain file committed, `rustc --version` matches | 1.1 | cc:done [7437ca6] |
| 1.3 | Implement `agi4-core` type system (enums, bounded types) | All core types compile, `Debug`, `Clone`, `Serialize` derive work | 1.1 | cc:done [2597f22] |
| 1.4 | Implement per-conjunct evaluation functions (generality, economic_substitutability, environmental_transfer, autonomous_agency) | All four evaluation functions compile and unit tests pass | 1.3 | cc:done [2d0b679] |
| 1.5 | Implement consistency check logic (all three sub-rules from SPEC.md §4) | `consistency_check` function total, all sub-rules tested | 1.3 | cc:done [57c918e] |
| 1.6 | Implement verdict function (pure, total, spec-compliant) | `verdict()` is total (no panics), exhaustive verdict table test passes | 1.4, 1.5 | cc:done [1d64c29] |
| 1.7 | Implement verdict invariant property tests | Property tests assert verdict is total, exhaustive 512-case test passes | 1.6 | cc:done [3315ed9] |
| 1.8 | Implement `Source` trait and `Fetcher` abstraction in `agi4-adapters` | Trait compiles, in-memory test fetcher works | 1.1 | cc:done [34616e3] |
| 1.9 | Implement METR reference adapter (simplest schema) | Adapter parses frozen METR fixture, round-trip test passes | 1.8 | cc:done [6e5b85e] |
| 1.10 | Define output JSON types in `agi4-schema` | `VerdictOutput`, `ConjunctReport` types serialize/deserialize | 1.1 | cc:done [04fdbc2] |
| 1.11 | Implement JSON schema export and validation | `schemars` generates schema, schema drift check works in CI | 1.10 | cc:done [36e81f7] |
| 1.12 | Implement Markdown report renderer in `agi4-report` | `render()` function produces valid Markdown, snapshot test passes | 1.10 | cc:done [b06b876] |
| 1.13 | Create `agi4` facade crate with curated public API | `lib.rs` re-exports work, `cargo test -p agi4` passes | 1.3, 1.10, 1.12 | cc:done [6bcede5] |
| 1.14 | Implement CLI binary with subcommands (attest --fixture, render, schema, version) | `cargo run -- attest --model example --fixture ./tests/fixtures/example/` produces valid JSON | 1.13 | cc:done [f13cdba] |
| 1.15 | End-to-end integration test with frozen fixture | CLI produces valid verdict JSON against frozen fixture data | 1.14 | cc:done [ba58267] |
| 1.16 | Set up CI pipeline (fmt, clippy, test, semver-check, audit, deny) | `.github/workflows/ci.yml` passes all checks on every commit | 1.7, 1.14 | cc:done [889c065] |
| 1.17 | Add adapter fixture validation step to CI | CI fails if any adapter lacks fixture or round-trip test | 1.9, 1.16 | cc:done [3693479] |
| 1.18 | Verify `cargo install --path crates/agi4` works locally | Binary installs and `agi4 version` returns `0.1.0` | 1.14 | cc:done [448ac65] |
| 1.19 | Tag v0.1.0 and publish to crates.io | Crates `agi4-core@0.1.0`, `agi4@0.1.0` published | 1.18 | cc:done [8b8ec30] |

---

## Phase 2: v0.1.1 — Real Adapters, First Live Attestation

**Goal:** Implement all nine upstream source adapters, run first live attestation, commit verdicts.

**Tracked issues** ([milestone v0.1.1](https://github.com/SHA888/agi4/milestone/1)):
- [#1](https://github.com/SHA888/agi4/issues/1) — Source-ID drift between SWE-bench evaluator and consistency check (blocks Task 2.9)
- [#2](https://github.com/SHA888/agi4/issues/2) — `evaluate_environmental_transfer` treats NES more permissively than SPEC.md §2.3

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 2.0a | Centralize source-ID constants in `agi4-core`, consume from evaluators + consistency check ([#1](https://github.com/SHA888/agi4/issues/1)) | One canonical `SourceId` per upstream source; both code paths reference the same constant | 1.9 | cc:done [d8ef8f6] |
| 2.0b | Tighten `evaluate_environmental_transfer` NES handling to match SPEC.md §2.3 ([#2](https://github.com/SHA888/agi4/issues/2)) | NES alone cannot move conjunct to Pass under v0.1.x; unit test added | 1.9 | cc:done [967d8d7] |
| 2.1 | Implement ARC Prize adapter (ARC-AGI-2, ARC-AGI-3) | Adapter parses leaderboard data, emits evidence for Generality and EnvironmentalTransfer | 2.0a | cc:done [4b2dd6e] |
| 2.2 | Implement HLE adapter | Adapter round-trip test passes with frozen fixture | 2.0a | cc:done [48e561e] |
| 2.3 | Implement GPQA Diamond adapter | Adapter round-trip test passes | 2.0a | cc:done [5913efb] |
| 2.4 | Implement GDPval/GDPval-AA adapter | Adapter round-trip test passes (prefer Artificial Analysis source) | 2.0a | cc:done [02796e2] |
| 2.5 | Implement RLI adapter | Adapter round-trip test passes | 2.0a | cc:done [795b558] |
| 2.6 | Implement APEX-Agents adapter | Adapter round-trip test passes | 2.0a | cc:TODO |
| 2.7 | Implement OSWorld adapter | Adapter round-trip test passes | 2.0a | cc:TODO |
| 2.8 | Implement RE-Bench adapter | Adapter round-trip test passes | 2.0a | cc:TODO |
| 2.9 | Implement SWE-bench Verified pass@5 adapter ([#1](https://github.com/SHA888/agi4/issues/1)) | Adapter rejects pass@1-only, round-trip test passes | 2.0a | cc:TODO |
| 2.10 | Wire `attest --live` to fetch upstream sources concurrently | `cargo run -- attest --model example --live` fetches with timeout and retry | 2.1-2.9 | cc:TODO |
| 2.11 | Add cache layer (local filesystem) to avoid hammering upstream | Cache hit/miss behavior tested, concurrent fetches deduplicate | 2.10 | cc:TODO |
| 2.12 | Run first live attestation on a frontier model | Verdict JSON + Markdown report committed to `attestations/v0.1.0/<model>-<date>.{json,md}` | 2.11 | cc:TODO |
| 2.13 | Update README with "First attestations" section | README links to committed verdicts | 2.12 | cc:TODO |

---

## Phase 3: v0.1.2 — Calibration Based on First Attestation

**Goal:** Refine thresholds based on first verdicts, recalibrate spec.

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 3.1 | Run attestation on three to five frontier models | Five verdicts committed, tabulate margin analysis | 2.12 | cc:TODO |
| 3.2 | Analyze threshold looseness/tightness vs. diagnostic intent | Tabulation complete, threshold movement recommendations documented | 3.1 | cc:TODO |
| 3.3 | Update SPEC.md thresholds and corresponding code constants | Spec and `threshold.rs` updated in sync, verdict-impact analysis included | 3.2 | cc:TODO |
| 3.4 | Re-attest same models under v0.1.2, commit new verdicts | Five verdicts committed alongside v0.1.0 verdicts | 3.3 | cc:TODO |
| 3.5 | Document threshold movements in CHANGELOG.md | CHANGELOG records rationale for each threshold change | 3.4 | cc:TODO |

---

## Phase 4: v0.1.3 — Gap Closure (NES Specification)

**Goal:** Close largest known gap: NES (Novel-Environment Subset).

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 4.1 | Survey candidate benchmarks for NES | Document interactive environments with no training-time analogues | 3.5 | cc:TODO |
| 4.2 | Define NES acceptance criteria in SPEC.md | Spec §8 updated with NES definition and refresh policy | 4.1 | cc:TODO |
| 4.3 | Implement NES adapter(s) | Adapter(s) round-trip test pass | 4.2 | cc:TODO |
| 4.4 | Decide spec bump (0.1.3 vs 0.2.0 early) and re-attest | Verdicts under new spec committed | 4.3 | cc:TODO |

---

## Phase 5: v0.2.0 — First Stable Threshold Set

**Goal:** Lock calibration, publish multiple attestations, address remaining gaps.

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 5.1 | Review all v0.1.x calibration deltas | Calibration journey documented in CHANGELOG.md | 3.5, 4.4 | cc:TODO |
| 5.2 | Lock thresholds for v0.2.0 (no future changes without MAJOR bump) | SPEC.md §3 frozen, decision documented | 5.1 | cc:TODO |
| 5.3 | Address known gaps: multimodal coverage, non-verifiable-domain agency, adversarial regeneration | At least one gap closure or documented continued limitation | 5.2 | cc:TODO |
| 5.4 | Create GOVERNANCE.md (maintainer pattern, fork policy, dispute resolution) | GOVERNANCE.md committed | 5.2 | cc:TODO |
| 5.5 | Export and commit stable JSON schema v0.2.0 | `/schema/agi4-output-v0.2.0.json` committed and frozen | 5.2 | cc:TODO |
| 5.6 | Publish at least five frontier-model attestations under v0.2.0 | Verdicts committed | 5.5 | cc:TODO |
| 5.7 | Publish v0.2.0 to crates.io | All crates published at stable versions | 5.6 | cc:TODO |

---

## Cross-Cutting Tasks (Any Phase)

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| X.1 | Document per-conjunct rationale (why these sources triangulate) | One-page rationale per conjunct in `docs/conjuncts/` | - | cc:TODO |
| X.2 | Document how to read a verdict (for non-implementers) | `docs/reading-verdicts.md` complete and clear | - | cc:TODO |
| X.3 | Outreach to ARC Prize, METR, Epoch AI (after v0.1.1 works) | Issue or email sent only after live attestation proven | - | cc:TODO |

---

## Deferred / Out of Scope

- Web-facing verdict viewer (v0.3.0+)
- Historical verdict diffing (v0.3.0+)
- Multi-model batch attestation (v0.3.0+)
- Plugin system for third-party adapters (never, by design)
- Configurable thresholds at runtime (never, by design)
- Daemon mode (never, by design)
