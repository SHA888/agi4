# agi4 Plans

Project task tracking for AGI/4 attestation protocol reference runner.

**Created:** 2026-05-26
**Current Phase:** v0.1.2 (Calibration complete, thresholds verified)
**Latest:** 2026-06-29 — Phase 3 complete, v0.1.2 released to GitHub and crates.io

---

## Phase 4: v0.2.0 — Gap Closure (NES Specification)

**Goal:** Close largest known gap: NES (Novel-Environment Subset).

| Task | Scope | DoD | Depends | Status |
|------|-------|-----|---------|--------|
| 4.1 | Survey candidate benchmarks for NES | Document interactive environments with no training-time analogues | 3.5 | cc:done [research-complete] |
| 4.2 | Define NES acceptance criteria in SPEC.md | Spec §8 updated with NES definition and refresh policy | 4.1 | cc:done [73dabf3] |
| 4.3 | Implement NES adapter(s) | Adapter(s) round-trip test pass | 4.2 | cc:TODO |
| 4.4 | Execute MAJOR spec bump to 0.2.0 (settled by Task 4.2 — conjunct logic + threshold value change) and re-attest | Verdicts under new spec committed | 4.3 | cc:TODO |

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

---

## 📦 Archive

Phase 1 (v0.1.0), Phase 2 (v0.1.1, incl. Code Review Findings remediation and Task 2.26 refactoring pattern), and Phase 3 (v0.1.2) are all complete (all tasks `cc:done`). Full task-level detail with commit hashes moved to [`Plans-archive.md`](Plans-archive.md) to keep this file under the 200-line limit.
