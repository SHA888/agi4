# agi4

AGI/4 specification and reference runner.

Public facade library and CLI binary for issuing AGI/4 attestations based on upstream benchmark sources.

## Library API

```rust
use agi4::{Verdict, render_verdict};
use agi4::schema::VerdictOutput;

// Core verdict logic
let verdict = agi4::verdict(/*...*/);

// Render to Markdown
let markdown = render_verdict(&verdict_output);

// Access thresholds
let threshold = agi4::thresholds::generality::ARC_AGI_2_PASS;
```

## CLI

```bash
# Attest using local fixture
agi4 attest --model example --fixture ./tests/fixtures/example/

# Render verdict to Markdown
agi4 render --input verdict.json

# Export JSON schema
agi4 schema

# Show version
agi4 version
```

## Features

- **Verdict Curation**: Re-exports core types (Verdict, ConjunctStatus, Evidence, VerdictOutput)
- **Consistency Checks**: Cross-conjunct validation function
- **Report Rendering**: Markdown report generation from verdicts
- **Threshold Constants**: All SPEC.md §3 values accessible at `agi4::thresholds::`
- **CLI Subcommands**: attest, render, schema, version
- **JSON Output**: Valid JSON Schema Draft 7 output
- **Version Tracking**: VERSION and SPEC_VERSION constants track SPEC.md exactly

## Versions

- **agi4 crate version**: Tracks SPEC.md SemVer exactly
- **spec_version** in verdict JSON: Identifies specification version
- **runner_version** in verdict JSON: Identifies runner implementation version

## See Also

- [SPEC.md](../../SPEC.md) — Specification with thresholds and verdicts
- [agi4-core](../agi4-core/) — Verdict logic (pure, no I/O)
- [agi4-schema](../agi4-schema/) — JSON output types
- [agi4-adapters](../agi4-adapters/) — Upstream source ingestion
- [agi4-report](../agi4-report/) — Markdown rendering
- [README.md](../../README.md) — Project overview
