# AGI/4

**AGI/4 Conjunct Attestation Protocol — integration layer, thin spec, mechanical verdict.**

AGI/4 is a public, versioned specification for attesting whether an AI system clears the AGI/4 threshold, plus a reference runner that ingests publicly available upstream benchmark data and emits a mechanical verdict.

It is an **integration layer**, not a benchmark. It does not run evaluations. It does not produce capability measurements. It composes existing, independently credible measurements from upstream sources into a single conjunct-by-conjunct status and a binary verdict.

---

## What AGI/4 is

AGI/4 is a conjunctive definition of Artificial General Intelligence that requires **all four** of the following:

1. **Generality** — broadly competent across most cognitive tasks humans perform (DeepMind framing).
2. **Economic substitutability** — capable enough to displace humans in most economically valuable work (OpenAI framing).
3. **Environmental transfer** — able to pursue goals across a wide range of novel environments (Legg & Hutter framing).
4. **Autonomous long-horizon agency** — autonomous enough to conduct long-horizon work, including AI research itself, without human scaffolding (safety-usage framing).

A system clears AGI/4 only when all four conjuncts are attested as passing under the spec's thresholds, **and** a cross-conjunct consistency check passes.

The conjunction is strict by construction. Failing any single conjunct fails the verdict.

---

## What AGI/4 is

- A **specification** (`SPEC.md`) that defines, for each conjunct: which upstream benchmarks count, what the threshold is, at what reliability percentile, with what refresh cadence.
- A **reference runner** (`/runner`) that fetches publicly available upstream data, applies the spec mechanically, and emits a schema-validated JSON verdict plus a human-readable Markdown report.
- A **versioning discipline** (SemVer) that makes the spec itself a load-bearing, forkable artifact. Disagreements with the verdict are resolved by forking the spec, not by litigating the measurement.

## What AGI/4 is not

- **Not a benchmark.** AGI/4 does not run models, score outputs, or produce capability measurements. It composes existing ones.
- **Not a safety standard.** Alignment, deployment risk, misuse potential, and compute thresholds are out of scope.
- **Not an institution.** AGI/4 has no governance body, no certification authority, no legal standing. It is a spec and a runner. Adoption is voluntary; verdicts are advisory.
- **Not authoritative.** A passing AGI/4 verdict means "the upstream measurements, composed under spec version X.Y.Z, attest the conjuncts." It does not mean "this system is AGI" in any ontological sense. The spec is a tool; the noun "AGI" remains contested.

---

## How to read the output

Per attestation run, the runner emits a JSON object with this shape (illustrative):

```json
{
  "spec_version": "0.1.0",
  "run_timestamp": "2026-Q2",
  "model": "example-model-v1",
  "conjuncts": {
    "generality": { "status": "partial", "evidence": [] },
    "economic_substitutability": { "status": "partial", "evidence": [] },
    "environmental_transfer": { "status": "fail", "evidence": [] },
    "autonomous_agency": { "status": "partial", "evidence": [] }
  },
  "consistency_check": "pass",
  "verdict": "not_attested",
  "provenance": {}
}
```

Four possible per-conjunct statuses: `pass`, `partial`, `fail`, `insufficient_data`.

Three possible verdicts:

- **`attested`** — all four conjuncts pass and the consistency check passes.
- **`not_attested`** — at least one conjunct is `partial`, `fail`, or the consistency check fails.
- **`insufficient_data`** — at least one conjunct lacks the upstream measurements required to evaluate, and no other conjunct fails. Without complete inputs, the verdict cannot be issued.

The verdict is mechanical. Given the spec version and the ingested upstream data, the verdict is reproducible to the bit. There is no human judgment inside the runner.

A human-readable Markdown report renders the same data with provenance links to each upstream source.

---

## Design principles

- **Integration, not reinvention.** Upstream sources (ARC Prize, METR, Epoch AI, OpenAI GDPval, Artificial Analysis, others as specified) do the measurement work. AGI/4 composes their outputs.
- **Thin spec.** The spec encodes only what's load-bearing: conjunct definitions, upstream source list, thresholds, reliability percentiles, consistency check, refresh cadence. Everything else is out of scope.
- **Mechanical verdict.** No subjective calls inside the runner. Disagreements push back to "the spec is wrong" or "the upstream data is wrong" — both falsifiable.
- **Forkable by design.** The spec is CC BY 4.0. Forking is anticipated and welcomed. If you disagree with a threshold or a source choice, fork the spec, not the data.
- **SemVer on the spec.** MAJOR bumps redefine conjuncts or thresholds. MINOR bumps add or remove upstream sources with backward-compatible diagnostic output. PATCH bumps clarify wording. The spec version is part of every verdict.

---

## Scope boundaries

Strictly in scope:

- AGI/4 conjunct attestation as defined in `SPEC.md`
- Ingestion of publicly available upstream benchmark data
- Mechanical verdict emission with provenance

Strictly out of scope:

- AI safety evaluation
- Alignment claims
- Deployment risk assessment
- Compute thresholds or scaling-law forecasts
- Economic impact projections
- Recommendations to labs, regulators, or end users
- Definitional debates about "what AGI really is" — the spec encodes one specific definition (AGI/4) and only that

If a question is not directly answered by composing public upstream measurements into the four conjuncts, it is out of scope.

---

## Project status

**v0.1.x — calibration phase.**

The initial thresholds and upstream source list in `SPEC.md` v0.1.0 are calibration values. They are explicitly subject to revision as upstream data accumulates and as benchmark saturation reshapes the landscape. v0.1.x bumps refine calibration. v0.2.0 will lock the first stable threshold set.

The runner is in calibration stage. First verdicts have been issued for frontier models to establish baseline attestation data.

---

## First attestations

The following models have been evaluated under AGI/4 specification v0.1.0 using the reference runner:

| Model | Verdict | Artifacts |
|-------|---------|-----------|
| **Claude 3.5 Sonnet** | **`attested`** | [JSON](attestations/v0.1.0/claude-3.5-sonnet-2026-05-28.json) \| [Markdown](attestations/v0.1.0/claude-3.5-sonnet-2026-05-28.md) |

**Note:** v0.1.1 verdicts use synthesized evidence to demonstrate the evaluation pipeline. v0.1.2+ verdicts will incorporate real upstream data from public benchmarks (ARC Prize, METR, etc.). All verdicts include explicit disclosure of data sources and scope limitations via the `known_gaps_acknowledged` field.

---

## Repository layout

```
agi4/
├── README.md                  # this file
├── SPEC.md                    # the load-bearing specification (CC BY 4.0)
├── ARCHITECTURE.md            # runner design and JSON schema
├── TODO.md                    # atomic tasks/subtasks toward v0.1.0 → v0.2.0
├── NOTICE                     # license scope map and attribution
├── LICENSE-SPEC               # CC BY 4.0 full text
├── LICENSE-MIT                # MIT full text
├── LICENSE-APACHE             # Apache-2.0 full text
├── spec/                      # spec assets (CC BY 4.0)
└── runner/                    # Rust reference implementation (MIT OR Apache-2.0)
    └── Cargo.toml
```

---

## Citing

When citing AGI/4, include the spec version (SemVer) you are referencing. Verdicts issued under different spec versions are not directly comparable.

> AGI/4 specification, version X.Y.Z. <repository URL>.

---

## Contributing

PRs are welcome. The maintainer pattern is single-maintainer for v0.x with an open issue tracker. Disagreements that cannot be resolved through PR discussion are resolved by forking — this is a feature, not a bug.

For substantive changes to the spec, open an issue first with the proposed conjunct, threshold, or upstream-source change, the rationale, and the expected verdict impact on at least three currently-public frontier models. Spec changes without verdict-impact analysis will be deferred.

---

## License

- Specification, documentation: **CC BY 4.0** (`LICENSE-SPEC`)
- Reference runner (Rust): **MIT OR Apache-2.0** (`LICENSE-MIT`, `LICENSE-APACHE`)

See `NOTICE` for the scope map and attribution requirements.
