//! JSON schema and serialization types for AGI/4 verdicts.
//!
//! This crate defines the output types that serialize to JSON conforming to
//! SPEC.md §7 provenance requirements and ARCHITECTURE.md §7 schema.
//! JSON schema is exported and validated against committed schema files in CI.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The top-level verdict output JSON.
///
/// Contains the complete verdict result with evidence, consistency check results,
/// and verdict reasons. Serializes to JSON matching ARCHITECTURE.md §7 schema.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VerdictOutput {
    /// AGI/4 specification version (SemVer).
    pub spec_version: String,

    /// Runner version (SemVer).
    pub runner_version: String,

    /// ISO 8601 timestamp of verdict computation.
    pub run_timestamp: String,

    /// Model being evaluated.
    pub model: ModelMetadata,

    /// Per-conjunct evaluation results.
    pub conjuncts: ConjunctsOutput,

    /// Cross-conjunct consistency check result.
    pub consistency_check: ConsistencyCheckOutput,

    /// The final verdict: "attested", "not_attested", or "insufficient_data".
    pub verdict: String,

    /// Reasons why verdict is not attested (if applicable).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub verdict_reasons: Vec<String>,

    /// Known gaps in the specification and measurements.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub known_gaps_acknowledged: Vec<String>,
}

/// Model identification metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelMetadata {
    /// Model identifier.
    pub id: String,

    /// Organization/lab that created the model.
    pub provider: Option<String>,

    /// Model version or release date.
    pub version_or_date: Option<String>,
}

/// Output for all four conjuncts.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConjunctsOutput {
    /// Generality conjunct evaluation.
    pub generality: ConjunctReport,

    /// Economic substitutability conjunct evaluation.
    pub economic_substitutability: ConjunctReport,

    /// Environmental transfer conjunct evaluation.
    pub environmental_transfer: ConjunctReport,

    /// Autonomous agency conjunct evaluation.
    pub autonomous_agency: ConjunctReport,
}

/// Output for a single conjunct (aliased as ConjunctReport per DoD).
///
/// Reports the evaluation status, evidence, and margin information for a conjunct.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConjunctReport {
    /// Conjunct status: "pass", "partial", "fail", or "insufficient_data".
    pub status: String,

    /// Evidence from upstream sources contributing to this conjunct.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<EvidenceReport>,

    /// Min/max margin information (used by consistency check).
    pub margins: Option<MarginReport>,
}

/// Alias for backward compatibility.
pub type ConjunctOutput = ConjunctReport;

/// Evidence report with threshold comparison information.
///
/// Wraps Evidence with computed threshold and floor comparisons.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceReport {
    /// Source identifier (e.g., "arc-agi-3", "metr-80pct-time-horizon").
    pub source: String,

    /// Measurement identifier within the source.
    pub measurement: String,

    /// Measurement value (fraction, hours, or other).
    pub value: serde_json::Value,

    /// Threshold value for this source (if known).
    pub threshold: Option<f64>,

    /// Floor value for this source (if known).
    pub floor: Option<f64>,

    /// Whether value passes threshold.
    pub passes_threshold: Option<bool>,

    /// Whether value is below floor.
    pub below_floor: Option<bool>,

    /// Reliability percentile of the measurement.
    pub reliability_percentile: u8,

    /// Provenance metadata for the measurement.
    pub provenance: ProvenanceReport,
}

/// Alias for backward compatibility.
pub type EvidenceOutput = EvidenceReport;

/// Provenance metadata for an evidence measurement.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProvenanceReport {
    /// Source URL or API endpoint.
    pub source_url: String,

    /// ISO 8601 timestamp when data was fetched.
    pub fetch_timestamp: String,

    /// Source version or dataset version (if applicable).
    pub source_version: Option<String>,

    /// The raw value as ingested (before parsing/validation).
    pub raw_value: String,
}

/// Alias for backward compatibility.
pub type ProvenanceOutput = ProvenanceReport;

/// Margin information for a conjunct's evidence.
///
/// Used by consistency check to validate margin variance.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarginReport {
    /// Minimum margin (value / threshold) across sources.
    pub min: f64,

    /// Maximum margin (value / threshold) across sources.
    pub max: f64,
}

/// Alias for backward compatibility.
pub type MarginOutput = MarginReport;

/// Consistency check result.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConsistencyCheckOutput {
    /// Check status: "pass" or "fail".
    pub status: String,

    /// Which sub-rules failed (e.g., "margin_variance_ratio").
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub failed_rules: Vec<String>,

    /// Human-readable detail on why check failed.
    pub detail: Option<String>,
}

/// Generate the JSON schema for the AGI/4 verdict output.
///
/// Returns the complete JSON schema that describes the structure of VerdictOutput.
/// This schema is committed to the repository and validated in CI to detect drift.
pub fn generate_schema() -> schemars::schema::RootSchema {
    schemars::schema_for!(VerdictOutput)
}

/// Serialize the JSON schema to a pretty-printed JSON string.
///
/// Used for both CLI output and schema validation in CI.
pub fn schema_json_string() -> Result<String, serde_json::Error> {
    let schema = generate_schema();
    serde_json::to_string_pretty(&schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_output_serialize_deserialize() {
        let output = VerdictOutput {
            spec_version: "0.1.0".to_string(),
            runner_version: "0.1.0".to_string(),
            run_timestamp: "2026-05-26T00:00:00Z".to_string(),
            model: ModelMetadata {
                id: "test-model".to_string(),
                provider: Some("test-lab".to_string()),
                version_or_date: Some("2026-05-26".to_string()),
            },
            conjuncts: ConjunctsOutput {
                generality: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                economic_substitutability: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                environmental_transfer: ConjunctReport {
                    status: "partial".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                autonomous_agency: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
            },
            consistency_check: ConsistencyCheckOutput {
                status: "pass".to_string(),
                failed_rules: vec![],
                detail: None,
            },
            verdict: "not_attested".to_string(),
            verdict_reasons: vec!["environmental_transfer".to_string()],
            known_gaps_acknowledged: vec!["nes_underspecified".to_string()],
        };

        // Serialize to JSON
        let json = serde_json::to_string(&output).expect("should serialize");
        assert!(!json.is_empty());

        // Deserialize back
        let deserialized: VerdictOutput = serde_json::from_str(&json).expect("should deserialize");

        // Verify round-trip
        assert_eq!(deserialized.spec_version, output.spec_version);
        assert_eq!(deserialized.model.id, output.model.id);
        assert_eq!(deserialized.verdict, output.verdict);
        assert_eq!(deserialized.verdict_reasons.len(), 1);
    }

    #[test]
    fn conjunct_report_serialize() {
        let report = ConjunctReport {
            status: "pass".to_string(),
            evidence: vec![],
            margins: Some(MarginReport {
                min: 0.85,
                max: 0.95,
            }),
        };

        let json = serde_json::to_string(&report).expect("should serialize");
        assert!(json.contains("\"status\":\"pass\""));
        assert!(json.contains("\"min\":0.85"));
    }

    #[test]
    fn evidence_report_with_provenance() {
        let evidence = EvidenceReport {
            source: "arc-agi-3".to_string(),
            measurement: "interactive-private-pass".to_string(),
            value: serde_json::json!(0.75),
            threshold: Some(0.50),
            floor: Some(0.05),
            passes_threshold: Some(true),
            below_floor: Some(false),
            reliability_percentile: 80,
            provenance: ProvenanceReport {
                source_url: "https://arcprize.org".to_string(),
                fetch_timestamp: "2026-05-26T00:00:00Z".to_string(),
                source_version: Some("v1.0".to_string()),
                raw_value: "0.75".to_string(),
            },
        };

        let json = serde_json::to_string(&evidence).expect("should serialize");
        let deserialized: EvidenceReport = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(deserialized.source, "arc-agi-3");
        assert_eq!(deserialized.passes_threshold, Some(true));
        assert_eq!(deserialized.provenance.source_url, "https://arcprize.org");
    }

    #[test]
    fn model_metadata_with_optional_fields() {
        let model = ModelMetadata {
            id: "model-v1".to_string(),
            provider: None,
            version_or_date: None,
        };

        let json = serde_json::to_string(&model).expect("should serialize");
        assert!(json.contains("\"id\":\"model-v1\""));

        let deserialized: ModelMetadata = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(deserialized.id, "model-v1");
        assert!(deserialized.provider.is_none());
    }

    #[test]
    fn margin_report_serialize() {
        let margin = MarginReport {
            min: 0.12,
            max: 2.98,
        };

        let json = serde_json::to_string(&margin).expect("should serialize");
        let deserialized: MarginReport = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(deserialized.min, 0.12);
        assert_eq!(deserialized.max, 2.98);
    }

    #[test]
    fn consistency_check_output_serialize() {
        let check = ConsistencyCheckOutput {
            status: "fail".to_string(),
            failed_rules: vec!["margin_variance_ratio".to_string()],
            detail: Some("min/max ratio = 0.12, below required 0.5".to_string()),
        };

        let json = serde_json::to_string(&check).expect("should serialize");
        assert!(json.contains("margin_variance_ratio"));
    }

    #[test]
    fn conjuncts_output_all_variants() {
        let conjuncts = ConjunctsOutput {
            generality: ConjunctReport {
                status: "pass".to_string(),
                evidence: vec![],
                margins: None,
            },
            economic_substitutability: ConjunctReport {
                status: "fail".to_string(),
                evidence: vec![],
                margins: None,
            },
            environmental_transfer: ConjunctReport {
                status: "partial".to_string(),
                evidence: vec![],
                margins: None,
            },
            autonomous_agency: ConjunctReport {
                status: "insufficient_data".to_string(),
                evidence: vec![],
                margins: None,
            },
        };

        let json = serde_json::to_string(&conjuncts).expect("should serialize");
        let deserialized: ConjunctsOutput =
            serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(deserialized.generality.status, "pass");
        assert_eq!(deserialized.economic_substitutability.status, "fail");
        assert_eq!(deserialized.environmental_transfer.status, "partial");
        assert_eq!(deserialized.autonomous_agency.status, "insufficient_data");
    }

    #[test]
    fn json_schema_generation() {
        let schema = schemars::schema_for!(VerdictOutput);
        assert!(schema.schema.metadata.is_some());

        // Verify it can be serialized to JSON schema
        let schema_json = serde_json::to_string(&schema).expect("should serialize schema");
        assert!(!schema_json.is_empty());
    }

    #[test]
    fn json_schema_for_conjunct_report() {
        let schema = schemars::schema_for!(ConjunctReport);
        let schema_json = serde_json::to_string(&schema).expect("should serialize schema");
        assert!(schema_json.contains("status"));
        assert!(schema_json.contains("evidence"));
    }

    #[test]
    fn skip_serializing_if_empty() {
        let output = VerdictOutput {
            spec_version: "0.1.0".to_string(),
            runner_version: "0.1.0".to_string(),
            run_timestamp: "2026-05-26T00:00:00Z".to_string(),
            model: ModelMetadata {
                id: "test".to_string(),
                provider: None,
                version_or_date: None,
            },
            conjuncts: ConjunctsOutput {
                generality: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                economic_substitutability: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                environmental_transfer: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                autonomous_agency: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
            },
            consistency_check: ConsistencyCheckOutput {
                status: "pass".to_string(),
                failed_rules: vec![],
                detail: None,
            },
            verdict: "attested".to_string(),
            verdict_reasons: vec![],
            known_gaps_acknowledged: vec![],
        };

        let json = serde_json::to_string(&output).expect("should serialize");
        // Empty vecs should not be serialized
        assert!(!json.contains("\"verdict_reasons\":[]"));
        assert!(!json.contains("\"known_gaps_acknowledged\":[]"));
    }

    #[test]
    fn schema_drift_check() {
        // Load the committed schema file
        let committed_schema_str = include_str!("../../../../schema/agi4-output-v0.1.0.json");
        let committed_schema: serde_json::Value = serde_json::from_str(committed_schema_str)
            .expect("committed schema should be valid JSON");

        // Generate schema from current code
        let generated_schema = schemars::schema_for!(VerdictOutput);
        let generated_json = serde_json::to_value(&generated_schema)
            .expect("generated schema should serialize to JSON");

        // Compare: if they don't match, schema has drifted
        if committed_schema != generated_json {
            // For debugging, show the diff
            let committed_pretty =
                serde_json::to_string_pretty(&committed_schema).unwrap_or_default();
            let generated_pretty =
                serde_json::to_string_pretty(&generated_json).unwrap_or_default();

            panic!(
                "Schema drift detected!\n\nCommitted schema:\n{}\n\nGenerated schema:\n{}\n\n\
                 To fix, run: `cargo run -p agi4 -- schema > schema/agi4-output-v0.1.0.json`",
                committed_pretty, generated_pretty
            );
        }
    }
}
