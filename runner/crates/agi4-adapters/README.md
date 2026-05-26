# agi4-adapters

Upstream source adapters for AGI/4 evidence ingestion.

Implements pluggable adapters for nine upstream measurement sources, with pure parsing, fixture-based testing, and testable I/O abstraction.

## Architecture

### Source Trait

Pure parsing abstraction, no I/O:

```rust
pub trait Source {
    type Raw: FromStr;
    type Error: Error + Send + Sync;

    fn parse(&self, raw: String) -> Result<Self::Raw, Self::Error>;
    fn to_evidence(&self, raw: Self::Raw, model: &str) -> Vec<Evidence>;
    fn id(&self) -> SourceId;
    fn endpoint(&self) -> &str;
}
```

### Fetcher Trait

I/O abstraction for testability:

```rust
pub trait Fetcher {
    fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>>;
}
```

## Adapters (v0.1.0)

- **METR**: Autonomous agency evidence (80%-time horizon). Simplest schema (single f64).

## Adapters (v0.1.1 and beyond)

- ARC Prize (Generality)
- HLE (Generality)
- GPQA Diamond (Generality)
- GDPval / GDPval-AA (EconomicSubstitutability)
- RLI (EnvironmentalTransfer)
- APEX-Agents (EnvironmentalTransfer)
- OSWorld (EnvironmentalTransfer)
- RE-Bench (AutonomousAgency)
- SWE-bench Verified pass@5 (AutonomousAgency)

## Testing

All adapters include:

- **Fixture**: Frozen upstream data snapshot in `tests/fixtures/<adapter>/`
- **Round-Trip Test**: Parse fixture → evidence → serialize → verify

Example fixture: METR's `tests/fixtures/metr/time-horizon-168h.json`

## See Also

- [agi4-core](../agi4-core/) — Verdict logic
- [Source Trait Documentation](../agi4-adapters/src/lib.rs) — API reference
- [Phase 2 Plan](../../Plans.md#phase-2-v011--real-adapters-first-live-attestation) — Adapter roadmap
