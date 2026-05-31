# AGI/4 Attestation Verdict

## Evaluation Metadata

**Model:** claude-3.5-sonnet
**Specification Version:** 0.1.0
**Runner Version:** 0.1.0
**Run Timestamp:** 2026-05-28T01:06:38Z

## Per-Conjunct Evaluation

### Generality

**Status:** `pass`

#### Evidence

| Source | Measurement | Value | Threshold | Passes |
|--------|-------------|-------|-----------|--------|
| arc-agi-2 | pass-at-1 | 0.82 | 0.72 | ✓ |
| arc-agi-3 | pass-at-1-interactive | 0.79 | 0.72 | ✓ |
| hle | overall-accuracy | 0.85 | 0.75 | ✓ |
| gpqa-diamond | accuracy | 0.88 | 0.79 | ✓ |

#### Evidence Provenance

**arc-agi-2:** [pass-at-1](https://example.com/arc-agi-2)
**arc-agi-3:** [pass-at-1-interactive](https://example.com/arc-agi-3)
**hle:** [overall-accuracy](https://example.com/hle)
**gpqa-diamond:** [accuracy](https://example.com/gpqa-diamond)

### Economic Substitutability

**Status:** `pass`

#### Evidence

| Source | Measurement | Value | Threshold | Passes |
|--------|-------------|-------|-----------|--------|
| gdpval | win-tie-rate | 0.81 | 0.80 | ✓ |
| rli | completion-rate | 0.76 | 0.71 | ✓ |
| apex-agents | task-completion-rate | 0.83 | 0.78 | ✓ |

#### Evidence Provenance

**gdpval:** [win-tie-rate](https://example.com/gdpval)
**rli:** [completion-rate](https://example.com/rli)
**apex-agents:** [task-completion-rate](https://example.com/apex-agents)

### Environmental Transfer

**Status:** `pass`

#### Evidence

| Source | Measurement | Value | Threshold | Passes |
|--------|-------------|-------|-----------|--------|
| arc-agi-3 | pass-at-1-interactive | 0.79 | 0.72 | ✓ |
| osworld | task-completion-rate | 0.74 | 0.72 | ✓ |

#### Evidence Provenance

**arc-agi-3:** [pass-at-1-interactive](https://example.com/arc-agi-3)
**osworld:** [task-completion-rate](https://example.com/osworld)

### Autonomous Agency

**Status:** `pass`

#### Evidence

| Source | Measurement | Value | Threshold | Passes |
|--------|-------------|-------|-----------|--------|
| metr-time-horizon | hours-80pct | 336.0 | 24.00 | ✓ |
| re-bench | task-success-rate | 0.72 | 0.72 | ✓ |
| swe-bench-verified | pass-at-5 | 0.49 | 0.40 | ✓ |

#### Evidence Provenance

**metr-time-horizon:** [hours-80pct](https://example.com/metr-time-horizon)
**re-bench:** [task-success-rate](https://example.com/re-bench)
**swe-bench-verified:** [pass-at-5](https://example.com/swe-bench-verified)

## Consistency Check

**Status:** `pass`

## Verdict

**Result:** `ATTESTED`

**Reasons:**

- All four conjuncts passed: generality, economic_substitutability, environmental_transfer, autonomous_agency

## Known Gaps Acknowledged

- v0.1.1 uses synthetic representative values; v0.1.2+ will fetch real upstream data
- NES (Novel-Environment Subset) underspecified; using OSWorld as environmental transfer signal
