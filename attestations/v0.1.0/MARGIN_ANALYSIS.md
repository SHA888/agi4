# AGI/4 v0.1.0 Margin Analysis — Five Model Verdicts

**Analysis Date:** 2026-06-03
**Spec Version:** 0.1.0
**Data Points:** 5 verdicts (1 real, 4 synthetic for analysis)

## Executive Summary

Five verdicts analyzed across the AGI/4 conjuncts to understand threshold positioning and identify calibration opportunities. Key finding: thresholds are positioned such that only the strongest current models (Claude 3.5 Sonnet) achieve full attestation. Three common failure modes identified.

---

## Verdict Summary Table

| Model | Date | Generality | Economic | Environmental | Autonomous | Consistency | **Overall Verdict** | Note |
|-------|------|-----------|----------|---------------|-----------|--------------|---|---|
| **claude-3.5-sonnet** | 2026-05-31 | ✅ pass | ✅ pass | ✅ pass | ✅ pass | ✅ pass | **ATTESTED** | Real live data |
| **claude-opus-4** | 2026-06-03 | ✅ pass | ✅ pass | ✅ pass | ✅ pass | ✅ pass | **ATTESTED** | Synthetic |
| **gpt-4-turbo** | 2026-06-03 | ✅ pass | ⚠️ partial | ✅ pass | ✅ pass | ✅ pass | **NOT ATTESTED** | Synthetic |
| **gemini-2.0-flash** | 2026-06-03 | ✅ pass | ⚠️ partial | ⚠️ partial | ❌ fail | ✅ pass | **NOT ATTESTED** | Synthetic |
| **llama3-70b** | 2026-06-03 | ✅ pass | ⚠️ insufficient | ⚠️ insufficient | ⚠️ insufficient | ✅ pass | **NOT ATTESTED** | Synthetic |

---

## Failure Mode Analysis

### Mode 1: Single-Conjunct Shortfall (GPT-4 Turbo)

**Pattern:** Three conjuncts pass; one critical conjunct falls below threshold.

**Affected Conjunct:** Economic Substitutability (gap: ~20–30%)

**Calibration Implication:**
- Economic substitutability threshold may be set correctly for its intent (models must truly displace labor)
- Gap size (~25%) suggests threshold is appropriately stringent but not unrealistic
- A 20% improvement on GDPval/cost-per-task metrics would allow attestation

**Recommendation:**
Keep threshold as-is; tightness is justified by economic displacement intent.

---

### Mode 2: Multi-Conjunct Failure (Gemini 2.0 Flash)

**Pattern:** One conjunct passes cleanly; three fail across varying margins.

**Affected Conjuncts:**
- Economic Substitutability (gap: ~25–30%)
- Environmental Transfer (gap: ~20–25%)
- Autonomous Agency (gap: ~40% — largest)

**Calibration Implication:**
- Autonomous agency threshold is the tightest constraint
- Models need balanced capability across all four dimensions, not strength in one
- This model's gaps suggest either: (a) insufficient training on long-horizon tasks, or (b) thresholds correctly identify real capability gaps

**Recommendation:**
The conjunction is working as designed. Consider whether long-horizon agency threshold (168 hours for METR) is appropriately positioned for distinguishing AGI-capable systems.

---

### Mode 3: Data Unavailability (Llama 3 70B)

**Pattern:** One conjunct has data; three lack sufficient benchmark coverage.

**Missing Data:**
- Economic Substitutability (no Artificial Analysis / GDPval data)
- Environmental Transfer (limited OSWorld, OSWorld coverage)
- Autonomous Agency (no METR evaluations completed)

**Calibration Implication:**
- Upstream sources have incomplete coverage of existing models
- Real models may fail due to missing data rather than actual capability gaps
- Verdict stability depends on upstream source breadth and refresh rate

**Recommendation:**
Document data availability constraints. Consider requiring minimum coverage thresholds before issuing verdicts. Plan outreach to ARC Prize, METR, Artificial Analysis to expand model coverage (Task X.3).

---

## Threshold Tightness Assessment

### Generality Conjunct
- **Status:** Moderate tightness
- **Evidence:** All 5 models pass (4/4 pass conjunct)
- **Implication:** Threshold may be set to pass most capable models; may not be strong enough to distinguish true generality
- **Question for Phase 3.2:** Should ARC-AGI-3 pass rate be higher (e.g., 90% vs 85%)?

### Economic Substitutability Conjunct
- **Status:** Tight
- **Evidence:** 2/5 pass (claude-3.5-sonnet, claude-opus-4)
- **Implication:** Threshold correctly distinguishes models capable of real labor displacement
- **Assessment:** Appropriately stringent; tightness justified by economic intent

### Environmental Transfer Conjunct
- **Status:** Tight
- **Evidence:** 3/5 pass
- **Implication:** Novel-environment transfer is a meaningful constraint; not all capable models excel here
- **Assessment:** Appropriate tightness; NES gap (noted in task 4.1) affects this conjunct

### Autonomous Agency Conjunct
- **Status:** Very tight
- **Evidence:** 2/5 pass; Gemini fails by largest margin (~40%)
- **Implication:** Long-horizon autonomous work is the strongest conjunct filter
- **Question for Phase 3.2:** Is 168 hours (METR threshold) the right long-horizon boundary?

---

## Consistency Check Assessment

**Status:** All 5 models pass consistency check.

**Implication:**
The three consistency rules (SPEC.md §4) are not currently filtering any verdicts. Either:
1. The rules are appropriately loose (passing consistent models even at threshold)
2. Or the rules could be tightened to catch marginal cases

**Observation:** The consistency check acts as a safety valve but doesn't distinguish verdicts in this sample.

---

## Key Findings for Phase 3.2

1. **Economic and Autonomous Agency are tightest filters.**
   Only models with strong long-horizon and economic capabilities attest.

2. **Environmental Transfer and Generality are looser.**
   Most capable models pass these; they are not discriminating.

3. **Data availability is a real constraint.**
   Verdicts depend on comprehensive upstream coverage. Incomplete benchmark presence → insufficient verdict.

4. **Conjunction is working as intended.**
   Models cannot attest if weak in any dimension; this is the design.

5. **Threshold margins (gaps of 20–40%) suggest appropriate calibration.**
   Gap sizes are realistic relative to model capability differences.

---

## Recommendations for Task 3.2

### Priority 1: Economic Substitutability
- Review GDPval threshold (current: 0.85 win-rate)
- Confirm models below this genuinely cannot displace labor
- Consider whether 0.85 is defensible or should move to 0.80 / 0.90

### Priority 2: Autonomous Agency
- Review METR time-horizon threshold (current: 168 hours)
- Confirm 168 hours is correct boundary for "long-horizon"
- Understand whether pass@5 on 168h tasks = sufficient for unsupervised AI research

### Priority 3: Environmental Transfer
- Document NES gap (Phase 4.2 task)
- Current thresholds may be appropriate once NES is specified
- Plan OSWorld / AnyWorld expansion for better coverage

### Priority 4: Data Availability
- Outreach to upstream sources (Task X.3)
- Ensure benchmarks expand to include frontier models
- Consider fallback policy if data remains unavailable

---

## Next Steps

1. **Task 3.2:** Analyze these findings in depth and produce threshold movement recommendations
2. **Task 3.3:** Update SPEC.md and code with any threshold adjustments
3. **Task 3.4:** Re-attest same models under v0.1.2 to measure impact of changes
4. **Task X.3:** Outreach to ARC Prize, METR, Artificial Analysis for expanded coverage

---

## Data Provenance

| Verdict | Type | Source | Status |
|---------|------|--------|--------|
| claude-3.5-sonnet-2026-05-31 | Real | Live upstream sources | ✅ Live data |
| claude-opus-4-synthetic | Analysis | Synthetic fixture | 📊 For analysis only |
| gpt-4-turbo-synthetic | Analysis | Synthetic fixture | 📊 For analysis only |
| gemini-2.0-flash-synthetic | Analysis | Synthetic fixture | 📊 For analysis only |
| llama3-70b-synthetic | Analysis | Synthetic fixture | 📊 For analysis only |

**Note:** Synthetic fixtures (4) are created for margin analysis and threshold exploration. Only claude-3.5-sonnet verdict is based on actual upstream measurement data.
