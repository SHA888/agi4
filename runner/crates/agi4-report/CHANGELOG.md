# agi4-report Changelog

## [0.1.0] - 2026-05-26

### Added

- `render()` function: Converts VerdictOutput to Markdown
- Report sections:
  - Evaluation metadata (model, versions, timestamps)
  - Per-conjunct evaluation with evidence tables
  - Margin analysis (min/max for consistency checks)
  - Consistency check results with detailed failure reasons
  - Verdict summary with reasons for non-attestation
  - Known gaps acknowledgments
- Evidence table rendering:
  - Columns: Source, Measurement ID, Value, Threshold, Pass/Fail indicator
  - Source links to upstream documentation
  - Conditional rendering (only when evidence present)
- Optional field handling:
  - Provider, version_or_date, evidence, detail only rendered when present
  - Graceful fallback for missing data
- Comprehensive test suite:
  - 9 test functions covering structure, sections, evidence tables, optional fields
  - Snapshot tests for regression detection
- Markdown formatting:
  - Headers (h1-h3) for document structure
  - Tables for evidence and margins
  - Bold/italic for emphasis
  - Monospace for technical values and thresholds

### Design Principles

- **Readability First**: Human-consumable format (not just for parsers)
- **Completeness**: All verdict metadata rendered (reasoning, gaps, margins)
- **Extensibility**: Optional sections for future data without breaking layout
- **Linkability**: Provenance links enable source verification
