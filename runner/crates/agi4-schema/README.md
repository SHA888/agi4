# agi4-schema

JSON schema and serialization types for AGI/4 verdict outputs.

Defines the canonical JSON structure for verdicts, with automatic schema generation and validation.

## Features

- **VerdictOutput**: Top-level envelope with metadata, conjuncts, consistency check, verdict, reasons
- **Conjunct Reports**: Per-conjunct evaluation results with evidence tables, margins, provenance
- **Evidence Reporting**: Source, measurement ID, value, threshold, pass/fail indicators
- **Margin Analysis**: Min/max margin values for variance-bound consistency checks
- **Provenance Links**: Upstream source metadata (endpoint, refresh cadence, version)
- **JSON Schema Generation**: Automatic schema generation via schemars for external validation
- **Schema Drift Detection**: CI test that fails if generated schema drifts from committed version

## JSON Structure

```json
{
  "spec_version": "0.1.0",
  "runner_version": "0.1.0",
  "run_timestamp": "2026-05-26T12:00:00Z",
  "model": {
    "id": "example-model",
    "provider": "example-org",
    "version_or_date": "2026-05-25"
  },
  "conjuncts": {
    "generality": {
      "status": "pass",
      "evidence": [...],
      "margins": { "min": 0.85, "max": 0.90 }
    },
    ...
  },
  "consistency_check": {
    "status": "pass",
    "failed_rules": [],
    "detail": null
  },
  "verdict": "attested",
  "verdict_reasons": [],
  "known_gaps_acknowledged": [...]
}
```

## Usage

```rust
use agi4_schema::{VerdictOutput, render_to_json};
use serde_json;

// Serialize a verdict to JSON
let json = serde_json::to_string_pretty(&verdict)?;

// Parse JSON into VerdictOutput
let verdict: VerdictOutput = serde_json::from_str(&json)?;

// Generate schema
let schema = agi4_schema::generate_schema();
let schema_str = agi4_schema::schema_json_string()?;
```

## See Also

- [agi4-core](../agi4-core/) — Verdict logic
- [agi4-report](../agi4-report/) — Markdown rendering
- [SPEC.md](../../SPEC.md) — Specification
