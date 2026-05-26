# agi4 Specification

**Version:** 0.1.0
**Status:** Calibration phase. Thresholds and upstream source choices in this version are explicitly subject to revision in v0.1.x patches as upstream data accumulates.
**License:** CC BY 4.0
**SemVer policy:**
- MAJOR — conjunct definitions or threshold values change
- MINOR — upstream source added or removed; threshold tightened with backward-compatible diagnostic output
- PATCH — clarifications, wording fixes, source URL updates

---

## 1. Definition

AGI/4 is the conjunction of four conditions on an AI system:

1. **Generality** — broadly competent across most cognitive tasks humans perform.
2. **Economic substitutability** — capable enough to displace humans in most economically valuable work.
3. **Environmental transfer** — able to pursue goals across a wide range of novel environments.
4. **Autonomous long-horizon agency** — autonomous enough to conduct long-horizon work, including AI research itself, without human scaffolding.

A system is attested as clearing AGI/4 under spec version X.Y.Z if and only if all four conjuncts return status `pass` under the thresholds in §3 **and** the cross-conjunct consistency check in §4 returns `pass`.

The conjunction is strict. A single non-passing conjunct fails the verdict.

---

## 2. Conjunct definitions and upstream sources

Each conjunct is operationalized by one or more upstream measurements. The runner ingests these from publicly available sources. The spec does not define new measurements; it composes existing ones.

For each upstream source, the spec specifies: the measurement, the required reliability percentile, the minimum-evidence requirement, and the saturation watch (the score above which the source is flagged for replacement in a future MINOR bump).

### 2.1 Generality

**Operationalization:** Performance on benchmarks designed to test (a) breadth of knowledge across domains, and (b) fluid reasoning on novel tasks.

| Source | Measurement | Operator | Reliability | Saturation watch |
|---|---|---|---|---|
| ARC-AGI-2 | Private split, pass@1 | ARC Prize Foundation | 95% over evaluation suite | ≥90% |
| ARC-AGI-3 | Interactive task private split, pass@1 | ARC Prize Foundation | 80% (exploration variance) | ≥85% |
| HLE (Humanity's Last Exam) | Overall accuracy | CAIS | 95% over evaluation suite | ≥85% |
| GPQA Diamond | Accuracy | Independent | 95% over evaluation suite | ≥95% |

**Minimum evidence:** At least three of the four sources must have published results for the model under evaluation within the previous two quarters. ARC-AGI-3 result is required (cannot be substituted) — it is the load-bearing fluid-generalization signal.

**Conjunct status mapping:**
- `pass` — all required sources meet their thresholds (defined in §3.1)
- `partial` — at least one source meets threshold and at least one does not
- `fail` — no source meets threshold, or ARC-AGI-3 returns below the fluid-generality floor
- `insufficient_data` — minimum evidence requirement unmet

### 2.2 Economic substitutability

**Operationalization:** Performance on benchmarks designed to test end-to-end completion of economically valuable knowledge work at quality comparable to human professionals.

| Source | Measurement | Operator | Reliability | Saturation watch |
|---|---|---|---|---|
| GDPval gold (220 tasks) | Win+tie rate vs industry experts | OpenAI / Artificial Analysis (GDPval-AA) | 95% over evaluation suite | ≥90% |
| Remote Labor Index (RLI) | Completion rate at expert-comparable quality | Scale AI / METR | 95% over evaluation suite | ≥75% |
| APEX-Agents | Task completion rate | Independent | 95% over evaluation suite | ≥85% |

**Minimum evidence:** Both GDPval and RLI must have published results for the model under evaluation within the previous two quarters. APEX-Agents is supplementary.

**Rationale for requiring both GDPval and RLI:** GDPval tests task-level deliverable quality; RLI tests longer end-to-end labor tasks. Together they triangulate the difference between "can produce expert-quality artifacts" and "can substitute for an expert across a workday." Either alone is gameable.

**Conjunct status mapping:**
- `pass` — both required sources meet their thresholds (defined in §3.2)
- `partial` — one source meets threshold, the other does not
- `fail` — neither required source meets threshold
- `insufficient_data` — minimum evidence requirement unmet

### 2.3 Environmental transfer

**Operationalization:** Performance on benchmarks designed to test adaptation to environments not present in training distribution, with no task family seen during training.

| Source | Measurement | Operator | Reliability | Saturation watch |
|---|---|---|---|---|
| ARC-AGI-3 | Interactive task private split (cross-listed with §2.1) | ARC Prize Foundation | 80% (exploration variance) | ≥85% |
| OSWorld | Task completion rate with no domain-specific scaffolding | Independent | 80% | ≥85% |
| Novel-environment subset (NES) | TBD — held-out interactive environments with no training-time analogues | Spec-designated | 80% | ≥75% |

**Minimum evidence:** ARC-AGI-3 result is required. At least one of OSWorld or NES must also have a published result.

**Note on NES:** The Novel-Environment Subset is currently underspecified in v0.1.0. v0.1.x will specify the exact set of environments accepted as satisfying NES. Until then, NES is treated as `insufficient_data` if no other interactive-novel benchmark is available. This is a known gap, tracked explicitly in `TODO.md`.

**Conjunct status mapping:**
- `pass` — ARC-AGI-3 ≥ threshold AND (OSWorld or NES) ≥ threshold
- `partial` — ARC-AGI-3 above floor but below threshold, OR ARC-AGI-3 passes but no secondary source passes
- `fail` — ARC-AGI-3 below the fluid-transfer floor
- `insufficient_data` — minimum evidence requirement unmet

### 2.4 Autonomous long-horizon agency

**Operationalization:** Ability to complete long-horizon work autonomously, at reliability percentiles meaningful for substitution, including in the specific domain of AI research.

| Source | Measurement | Operator | Reliability | Saturation watch |
|---|---|---|---|---|
| METR Time Horizon | 80%-time horizon, hours | METR | 80% (by definition of the metric) | ≥720h (one month) |
| RE-Bench | AI research engineering task success rate | METR / independent | 80% | ≥80% |
| SWE-bench Verified | pass@k at k ≥ 5 | Independent | 80% at k=5 | ≥95% pass@5 |

**Minimum evidence:** METR 80%-time horizon is required. At least one of RE-Bench or SWE-bench Verified at pass@5 must also have a published result.

**Rationale for the 80% percentile, not 50%:** The 50% percentile is published more widely but is not meaningful for substitution. Human professionals operate at higher reliability than coin-flip. The spec defaults to 80% as the lowest percentile compatible with substitution claims. Future MAJOR versions may tighten this to 95%.

**Conjunct status mapping:**
- `pass` — METR 80%-time horizon ≥ threshold AND (RE-Bench or SWE-bench Verified pass@5) ≥ threshold
- `partial` — METR 80%-time horizon ≥ threshold but no supporting source meets threshold, OR supporting source meets threshold but METR 80%-time horizon below threshold and above floor
- `fail` — METR 80%-time horizon below floor
- `insufficient_data` — minimum evidence requirement unmet

---

## 3. Thresholds

**v0.1.0 status: calibration values.** These numbers are starting points selected from the 2026 landscape diagnosis. They will move in v0.1.x patches as evidence accumulates. v0.2.0 will lock the first stable threshold set.

### 3.1 Generality thresholds

| Source | Pass threshold | Floor (below this = fail) |
|---|---|---|
| ARC-AGI-2 | ≥85% on private split | n/a |
| ARC-AGI-3 | ≥50% on interactive private split | <5% (fluid-generality floor) |
| HLE | ≥80% overall | n/a |
| GPQA Diamond | ≥90% | n/a |

### 3.2 Economic substitutability thresholds

| Source | Pass threshold | Floor |
|---|---|---|
| GDPval | ≥85% win+tie rate vs industry experts | n/a |
| RLI | ≥60% completion at expert-comparable quality | <10% |
| APEX-Agents | ≥75% completion rate | n/a |

**Rationale for asymmetric thresholds:** GDPval is closer to saturation, so its pass threshold is higher in absolute terms. RLI is harder, so its pass threshold is lower in absolute terms but represents a much stronger capability claim.

### 3.3 Environmental transfer thresholds

| Source | Pass threshold | Floor |
|---|---|---|
| ARC-AGI-3 | ≥50% (cross-listed with §3.1) | <5% (fluid-transfer floor) |
| OSWorld | ≥85% with no domain-specific scaffolding | n/a |
| NES | TBD in v0.1.x | TBD |

### 3.4 Autonomous agency thresholds

| Source | Pass threshold | Floor |
|---|---|---|
| METR 80%-time horizon | ≥168h (one work-week) | <8h (one workday) |
| RE-Bench | ≥60% task success | n/a |
| SWE-bench Verified pass@5 | ≥85% | n/a |

**Rationale for 168h (one work-week) at 80% reliability:** This is the lowest horizon at which "autonomous long-horizon work including AI research" is operationally meaningful. Shorter horizons describe assistance, not autonomy. The 720h (one month) saturation watch anticipates v0.2.0 tightening.

---

## 4. Cross-conjunct consistency check

A model with three of four conjuncts marginally passing and one in `insufficient_data` is suspicious — it suggests selective measurement. The consistency check guards against this.

**Rule:** The consistency check returns `pass` if and only if:

1. No conjunct's status is `insufficient_data` when all other conjuncts have status `pass`.
2. The variance across conjunct margins is within a defined bound. Specifically, if all four conjuncts pass, the minimum margin (lowest source-threshold ratio across all sources used) must be at least 0.5× the maximum margin. This prevents a system from clearing AGI/4 by saturating two conjuncts while barely scraping the other two.
3. Every upstream source cited has provenance metadata: source URL, fetch timestamp, source-side version or date stamp.

Failure of any sub-rule returns `fail`.

**Verdict implication:** If all four conjuncts pass but consistency check fails, the verdict is `not_attested`. The runner emits a `consistency_failure_reason` field explaining which sub-rule failed.

---

## 5. Verdict rules

Given the four conjunct statuses and the consistency check result:

| Conjunct statuses | Consistency check | Verdict |
|---|---|---|
| All four `pass` | `pass` | `attested` |
| All four `pass` | `fail` | `not_attested` (with consistency_failure_reason) |
| Any one `partial` or `fail` | any | `not_attested` |
| Any one `insufficient_data`, no `fail` | any | `insufficient_data` |
| Any one `fail` | any | `not_attested` (fail dominates insufficient_data) |

The runner emits the verdict mechanically from this table. No other code path produces a verdict.

---

## 6. Refresh and saturation policy

**Quarterly attestation:** The runner is intended to be executed quarterly against the current public upstream data, with verdicts archived per spec version.

**Saturation-triggered MAJOR bump:** When any upstream source's frontier-model performance crosses its saturation watch threshold (§2), the spec must be revised. Options for the revision: replace the source, supplement it with a harder benchmark, raise the pass threshold, or — if the saturation reflects genuine capability gain — leave it in place and document. The revision is a MAJOR bump because it changes verdict semantics.

**Time-bounded calibration:** v0.1.x is calibration. v0.2.0 must lock a stable threshold set no later than four quarters after v0.1.0, regardless of remaining open issues. The longer v0.1.x extends, the weaker the spec's stability claim.

---

## 7. Provenance requirements

Every verdict must include, for every upstream source cited:

- Source identifier (e.g., `arc-agi-3`, `metr-time-horizon-80pct`)
- Source URL
- Fetch timestamp (ISO 8601, UTC)
- Source-side version or publication date, where available
- The raw value ingested (not just the pass/fail derived from it)

A verdict missing any of these for any cited source is malformed and must not be emitted by a conforming runner.

---

## 8. Known gaps (v0.1.0)

These are explicitly tracked and do not block the spec from being usable; they bound what it can attest.

1. **NES (Novel-Environment Subset) underspecified.** Until v0.1.x specifies the accepted environment set, the environmental transfer conjunct relies on ARC-AGI-3 + OSWorld.
2. **No non-verifiable-domain measurement for autonomous agency.** Current upstream sources (RE-Bench, SWE-bench Verified) test verifiable-domain self-improvement. The "AI research itself" subclause of conjunct 4 includes open-ended research with subjective evaluation, which no public benchmark currently measures at sufficient quality. Until one exists, the spec attests only the verifiable-domain portion of the conjunct, and this limitation is explicitly noted in every verdict for conjunct 4.
3. **Multimodal coverage is partial.** Several upstream sources are text-first. Vision, audio, and embodied benchmarks are not yet integrated.
4. **Adversarial regeneration assumed but not enforced.** The spec assumes upstream sources maintain held-out splits and contamination resistance. v0.1.x will add explicit acceptance criteria for upstream sources covering this.

Each known gap is a `TODO.md` entry with a target version.

---

## 9. What the spec does not say

- Whether AGI/4 is the correct definition of AGI. (It is *a* definition; the conjunction was selected as a strict, diagnostic-grade composition of four widely-used framings. Forks proposing alternative definitions are explicitly welcomed.)
- Whether a particular threshold value is "correct." (Thresholds are calibration values defended by §3 rationale and revisable per §6.)
- Whether a system that fails AGI/4 is or is not "intelligent" in any broader sense. (Out of scope.)
- What labs, regulators, or users should do with a verdict. (Out of scope.)
