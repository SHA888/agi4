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
| OSWorld | Task completion rate with no domain-specific scaffolding | METR / Independent | 80% | ≥85% |
| WebArena | Web browser interaction task success rate | Stanford / Independent | 80% | ≥75% |
| SWE-bench Verified | Software engineering task pass@1 rate | METR / Independent | 80% | ≥75% |
| ARIES | AI research task completion score (0–100 rubric) | Independent | 85% | ≥75% |

**NES (Novel-Environment Subset) definition:** A benchmark qualifies as NES if it satisfies all five criteria:

1. **Interactive** — Agent must act in an environment where actions produce observable state changes. Not static QA/reading comprehension.
2. **Held-Out Novelty** — Task distribution unknown during model training. Operationally: benchmark released/updated after model training cutoff, OR task instances procedurally generated with infinite support not in training data.
3. **Measurable** — Success is quantifiable as pass/fail (binary) or score (0–100). Not subjective judgment.
4. **Public Data** — Benchmark data and/or leaderboard publicly available without paywalls.
5. **Frontier Model Evidence** — At least one frontier model (Claude 3.5 Sonnet, GPT-4 Turbo, Gemini 2.0 Flash) has public or peer-reviewed performance data.

**v0.1.3+ NES sources:** WebArena, SWE-bench Verified, and ARIES satisfy the five criteria and are accepted as NES sources in v0.1.3+. Additional NES sources may be added in future MINOR bumps without changing verdict semantics, provided they satisfy the five criteria above and their thresholds are set per calibration verdicts. Accepted NES sources are not permanent — see the NES source refresh policy in §6 for how novelty is re-checked and sources are retired or replaced.

**Minimum evidence:** ARC-AGI-3 result is required. Both OSWorld and at least one NES source must also have a published result.

**Rationale for requiring both OSWorld and NES:** OSWorld tests realistic desktop/web task sequences. NES sources (WebArena, SWE-bench Verified, ARIES) test specialized interactive environments (web, code, research). Together they triangulate broad environmental transfer. Requiring both minimizes false attestations; either alone is gameable.

**Conjunct status mapping:**
- `pass` — ARC-AGI-3 ≥ threshold AND OSWorld ≥ threshold AND at least one NES source ≥ threshold
- `fail` — ARC-AGI-3 below its floor, OR OSWorld below its floor, OR every present NES source is below its own floor
- `partial` — none of the `fail` conditions hold (ARC-AGI-3, OSWorld, and at least one NES source each clear their own floor), but the `pass` condition is not met
- `insufficient_data` — minimum evidence requirement unmet

This ordering is exhaustive: `fail` is checked first (any required source below its own floor), then `pass`, with everything else falling to `partial`.

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

**v0.1.2 status (Task 3.2):** Thresholds remain unchanged. Verdict-impact analysis across five frontier and near-frontier models (Claude 3.5 Sonnet, Claude Opus 4, GPT-4-Turbo, Gemini 2.0 Flash, Llama 3 70B) confirms current thresholds are well-calibrated for stated diagnostic intent with 15–68h realistic margins between passing and failing models (percentage gaps in GDPval/RLI; hour gaps in METR). See `attestations/v0.1.0/THRESHOLD_ANALYSIS.md` for detailed per-conjunct assessment.

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
| OSWorld | ≥20% task completion rate | <3% |
| WebArena | ≥20% task success rate | <5% |
| SWE-bench Verified | ≥25% pass@1 rate | <5% |
| ARIES | ≥60/100 task score (rubric-normalized) | <30/100 |

**Rationale for NES thresholds (v0.1.3 calibration):** Thresholds derived from frontier model performance across 2024–2025 benchmarks. WebArena and SWE-bench Verified cluster frontier models at 15–35%; pass thresholds set to 20–25% to avoid gaming by saturation. ARIES scores higher (60–75) due to task-selection bias; 60/100 pass threshold ensures diagnostic signal. OSWorld threshold revised downward from 85% (unrealistic) to 20% (achievable by strong models) based on v0.1.2 calibration verdicts. All NES thresholds will be revisited after first v0.1.3 verdicts if diagnostic margins are <10% or >50%.

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

**NES source refresh policy:** An accepted NES source (§2.3) satisfies criterion 2 (Held-Out Novelty) only for a bounded window, not permanently. At each quarterly attestation cycle, every accepted NES source is re-checked against criterion 2:

- If a source's task distribution has become part of mainstream pretraining data (e.g., the benchmark is scraped into a public pretraining corpus, or a paper reports contamination), it no longer satisfies Held-Out Novelty and is removed from the accepted-NES-source list.
- If removal would drop the environmental transfer conjunct below the minimum-evidence requirement in §2.3 (no accepted NES source remaining), the conjunct falls back to `insufficient_data` for the NES component until a replacement source is approved.
- Adding or removing an NES source is a MINOR bump (source added or removed with backward-compatible diagnostic output), per the SemVer policy above — it does not require a MAJOR bump unless it also changes the conjunct's pass/fail logic.
- Tier-2 candidates surveyed in `docs/nes-candidates.md` (LLMOE, ENVGEN, VirtualHome, IsaacLab) are promoted to accepted NES sources once they satisfy all five §2.3 criteria, including sufficient frontier-model evaluation data; until then they remain out of scope (tracked in §8, gap 1).

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

## 8. Known gaps (v0.1.3+)

These are explicitly tracked and do not block the spec from being usable; they bound what it can attest.

1. **NES (Novel-Environment Subset) specified in v0.1.3+.** Environmental transfer now requires ARC-AGI-3 + OSWorld + at least one NES source (WebArena, SWE-bench Verified, ARIES, or future-approved sources satisfying §2.3 criteria). v0.2.0 will close the remaining gap: Tier-2 NES candidates (LLMOE, ENVGEN, VirtualHome, IsaacLab) lack sufficient frontier-model eval data and will be added once lab adoption increases.
2. **No non-verifiable-domain measurement for autonomous agency.** Current upstream sources (RE-Bench, SWE-bench Verified) test verifiable-domain self-improvement. The "AI research itself" subclause of conjunct 4 includes open-ended research with subjective evaluation, which no public benchmark currently measures at sufficient quality. Until one exists, the spec attests only the verifiable-domain portion of the conjunct, and this limitation is explicitly noted in every verdict for conjunct 4.
3. **Multimodal coverage is partial.** Several upstream sources are text-first. Vision, audio, and embodied benchmarks are not yet integrated. v0.2.0 will add IsaacLab and other embodied benchmarks if LLM eval data improves.
4. **Adversarial regeneration assumed but not enforced.** The spec assumes upstream sources maintain held-out splits and contamination resistance. v0.2.0 will add explicit acceptance criteria for upstream sources covering this.

Each known gap is a `TODO.md` entry with a target version.

---

## 9. What the spec does not say

- Whether AGI/4 is the correct definition of AGI. (It is *a* definition; the conjunction was selected as a strict, diagnostic-grade composition of four widely-used framings. Forks proposing alternative definitions are explicitly welcomed.)
- Whether a particular threshold value is "correct." (Thresholds are calibration values defended by §3 rationale and revisable per §6.)
- Whether a system that fails AGI/4 is or is not "intelligent" in any broader sense. (Out of scope.)
- What labs, regulators, or users should do with a verdict. (Out of scope.)
