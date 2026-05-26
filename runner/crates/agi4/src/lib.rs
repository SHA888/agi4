//! AGI/4 specification and reference runner.
//!
//! The `agi4` facade crate provides a curated public API for attestation logic,
//! verdict types, and report rendering. Version tracks SPEC.md exactly.

/// Core verdict logic and types.
pub mod core {
    pub use agi4_core::conjunct::{Conjunct, ConjunctStatus};
    pub use agi4_core::consistency::ConsistencyResult;
    pub use agi4_core::evidence::{Evidence, MeasurementId, SourceId, SourceValue};
    pub use agi4_core::verdict::Verdict;
}

/// Re-export evaluation and consistency functions for direct use.
pub use agi4_core::conjunct::ConjunctStatus;
pub use agi4_core::consistency::consistency_check;
pub use agi4_core::evidence::{Evidence, MeasurementId, SourceId};
pub use agi4_core::verdict::Verdict;

/// Schema and serialization types for verdict outputs.
pub mod schema {
    pub use agi4_schema::{
        ConjunctReport, ConjunctsOutput, ConsistencyCheckOutput, EvidenceReport, MarginReport,
        ModelMetadata, ProvenanceReport, VerdictOutput,
    };
}

/// Re-export commonly-used schema types at top level.
pub use agi4_schema::{ConjunctReport, VerdictOutput};

/// Report rendering for verdicts.
pub use agi4_report::render as render_verdict;

/// Threshold constants used in verdict evaluation.
pub mod thresholds {
    pub use agi4_core::threshold::*;
}

/// Crate and specification metadata.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SPEC_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_re_exports_core_types() {
        // Verify ConjunctStatus is re-exported
        let status = ConjunctStatus::Pass;
        assert_eq!(status, ConjunctStatus::Pass);
    }

    #[test]
    fn facade_re_exports_schema_types() {
        // Verify VerdictOutput is re-exported at top level
        // VerdictOutput is importable as: use agi4::VerdictOutput;
        let _ = std::any::type_name::<VerdictOutput>();
    }

    #[test]
    fn facade_provides_version_constants() {
        // Verify version constants are accessible
        assert_eq!(SPEC_VERSION, "0.1.0");
        // VERSION is derived from Cargo.toml, always populated at compile time
        let _ = VERSION;
    }

    #[test]
    fn facade_modules_are_accessible() {
        // Verify submodules are accessible for more detailed imports
        let _ = std::any::type_name::<core::ConjunctStatus>();
        let _ = std::any::type_name::<schema::VerdictOutput>();
    }
}

#[cfg(test)]
mod cli_tests {
    #[test]
    fn cli_attest_produces_valid_json() {
        // Test that the attest subcommand works (indirectly via library)
        use crate::VerdictOutput;

        // Verify that VerdictOutput can be serialized/deserialized
        let verdict = serde_json::json!({
            "spec_version": "0.1.0",
            "runner_version": "0.1.0",
            "run_timestamp": "2026-05-26T00:00:00Z",
            "model": {"id": "test", "provider": null, "version_or_date": null},
            "conjuncts": {
                "generality": {"status": "pass", "margins": null},
                "economic_substitutability": {"status": "pass", "margins": null},
                "environmental_transfer": {"status": "pass", "margins": null},
                "autonomous_agency": {"status": "pass", "margins": null}
            },
            "consistency_check": {"status": "pass", "failed_rules": [], "detail": null},
            "verdict": "attested",
            "verdict_reasons": [],
            "known_gaps_acknowledged": []
        });

        let verdict_output: VerdictOutput =
            serde_json::from_value(verdict).expect("valid JSON should deserialize");

        assert_eq!(verdict_output.model.id, "test");
        assert_eq!(verdict_output.verdict, "attested");
    }
}
