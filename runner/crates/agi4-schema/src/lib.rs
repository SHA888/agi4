//! JSON schema and serialization types for AGI/4 verdicts.
//!
//! This crate defines the output types that serialize to JSON conforming to
//! SPEC.md §7 provenance requirements and ARCHITECTURE.md §7 schema.
//! JSON schema is exported and validated against committed schema files in CI.

use serde::{Deserialize, Serialize};

/// The top-level verdict output JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerdictOutput {
    pub spec_version: String,
    pub runner_version: String,
    pub run_timestamp: String,
    pub model: ModelMetadata,
    pub conjuncts: ConjunctsOutput,
    pub consistency_check: ConsistencyCheckOutput,
    pub verdict: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub verdict_reasons: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub known_gaps_acknowledged: Vec<String>,
}

/// Model identification metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub id: String,
    pub provider: Option<String>,
    pub version_or_date: Option<String>,
}

/// Output for all four conjuncts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctsOutput {
    pub generality: ConjunctOutput,
    pub economic_substitutability: ConjunctOutput,
    pub environmental_transfer: ConjunctOutput,
    pub autonomous_agency: ConjunctOutput,
}

/// Output for a single conjunct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctOutput {
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<EvidenceOutput>,
    pub margins: Option<MarginOutput>,
}

/// Evidence output with threshold comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceOutput {
    pub source: String,
    pub measurement: String,
    pub value: serde_json::Value,
    pub threshold: Option<f64>,
    pub floor: Option<f64>,
    pub passes_threshold: Option<bool>,
    pub below_floor: Option<bool>,
    pub reliability_percentile: u8,
    pub provenance: ProvenanceOutput,
}

/// Provenance metadata output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceOutput {
    pub source_url: String,
    pub fetch_timestamp: String,
    pub source_version: Option<String>,
    pub raw_value: String,
}

/// Margin information for consistency check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginOutput {
    pub min: f64,
    pub max: f64,
}

/// Consistency check output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckOutput {
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failed_rules: Vec<String>,
    pub detail: Option<String>,
}
