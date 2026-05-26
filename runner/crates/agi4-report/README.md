# agi4-report

Markdown report rendering for AGI/4 verdicts.

Converts `VerdictOutput` JSON to human-readable Markdown reports with evidence tables, margin analysis, and reasoning.

## Features

- **Evidence Tables**: Per-conjunct tables with source, measurement, value, threshold, status
- **Provenance Links**: Clickable links to upstream sources and documentation
- **Margin Analysis**: Min/max margin values for consistency bounds checking
- **Consistency Details**: Explanation of passed/failed consistency rules
- **Verdict Summary**: Readable verdict statement with reasons for non-attestation
- **Known Gaps**: Acknowledgments of intentional limitations
- **Optional Fields**: Graceful rendering when provider, version, or evidence absent

## Usage

```rust
use agi4_report::render;
use agi4_schema::VerdictOutput;
use serde_json;

let verdict_json = r#"{ ... }"#;
let verdict: VerdictOutput = serde_json::from_str(verdict_json)?;
let markdown = render(&verdict);
println!("{}", markdown);
```

## Output Example

```markdown
# AGI/4 Attestation Report

**Model**: example-model (example-org, 2026-05-25)
**Spec**: 0.1.0 | **Runner**: 0.1.0 | **Generated**: 2026-05-26T12:00:00Z

## Verdict: ATTESTED

All four conjuncts pass with consistent evidence.

## Conjunct Details

### Generality
| Source | Measurement | Value | Threshold | Status |
| --- | --- | --- | --- | --- |
| ARC Prize | ARC-AGI-2 | 0.87 | 0.85 | ✓ pass |

...

## Consistency Check
Status: **PASS**
Margins: Min 0.85, Max 0.90 (ratio 0.944 > 0.5) ✓

## Known Gaps
- NES (Novel-Environment Subset) underspecified
- Non-verifiable-domain agency not yet measurable
```

## See Also

- [agi4-schema](../agi4-schema/) — JSON output types
- [agi4](../agi4/) — CLI binary
- [SPEC.md](../../SPEC.md) — Specification
