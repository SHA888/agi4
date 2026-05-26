# agi4-schema Changelog

## [0.1.0] - 2026-05-26

### Added

- `VerdictOutput`: Top-level verdict envelope type
- `ModelMetadata`: Model identifier, provider, version/date
- `ConjunctsOutput`: Container for four conjunct reports
- `ConjunctReport`: Per-conjunct evaluation with status, evidence, margins
- `EvidenceReport`: Single evidence measurement with source, ID, value, threshold, pass/fail
- `ProvenanceReport`: Upstream source metadata (endpoint, refresh, version)
- `MarginReport`: Min/max margin values for consistency checks
- `ConsistencyCheckOutput`: Consistency check result with status, failed rules, detail
- All types derive `Serialize`, `Deserialize`, `JsonSchema` for automatic schema generation
- Support for optional fields via `skip_serializing_if`
- Comprehensive round-trip serialization tests (11+ test functions)
- JSON Schema v0.1.0 (322 lines, JSON Schema Draft 7 compliant)
- Schema drift detection test: regenerates schema on each run and fails if it drifts
- Canonical schema file committed to git for visibility in diffs

### Design Principles

- **Machine and Human Readable**: JSON for tools, field names for readability
- **Extensibility**: Optional fields allow future additions without breaking existing parsers
- **Validation**: JSON schema enables external validation and IDE support
- **Traceability**: Verdict timestamp and model metadata enable verdict tracking
