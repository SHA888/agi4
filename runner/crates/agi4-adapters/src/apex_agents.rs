//! APEX-Agents adapter for economic substitutability conjunct.
//!
//! Ingests task completion rate from the APEX-Agents benchmark.
//! Returns a single Fraction value representing task completion performance.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// APEX-Agents benchmark data: task completion rate.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApexAgentsRaw {
    /// Task completion rate as a fraction (0.0 to 1.0).
    pub task_completion_rate: f64,
}

/// Error type for APEX-Agents adapter operations.
#[derive(Debug, Clone)]
pub enum ApexAgentsError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds task completion rate).
    ValidationError(String),
}

impl fmt::Display for ApexAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "APEX-Agents parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "APEX-Agents validation error: {}", msg),
        }
    }
}

impl Error for ApexAgentsError {}

/// APEX-Agents adapter for the task completion rate measurement.
pub struct ApexAgentsAdapter {
    /// Canonical APEX-Agents endpoint.
    endpoint: Url,
}

impl ApexAgentsAdapter {
    /// Create a new APEX-Agents adapter with the canonical endpoint.
    pub fn new() -> Result<Self, ApexAgentsError> {
        let endpoint = Url::parse("https://apexagents.ai/api/results")
            .map_err(|e| ApexAgentsError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create an APEX-Agents adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for ApexAgentsAdapter {
    fn default() -> Self {
        Self::new().expect("default APEX-Agents endpoint should be valid")
    }
}

impl Source for ApexAgentsAdapter {
    type Raw = ApexAgentsRaw;
    type Error = ApexAgentsError;

    fn id(&self) -> SourceId {
        SourceId::new("apex-agents")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<ApexAgentsRaw>(raw)
            .map_err(|e| ApexAgentsError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let task_completion_rate = BoundedFraction::new(raw.task_completion_rate).map_err(|e| {
            ApexAgentsError::ValidationError(format!("invalid task completion rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("task-completion-rate"),
            value: SourceValue::Fraction(task_completion_rate),
            reliability_percentile: 95, // Per SPEC §2.2
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("apex-agents-v1".to_string()),
                raw_value: raw.task_completion_rate.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apex_agents_adapter_new() {
        let adapter = ApexAgentsAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "apex-agents");
        assert!(adapter.endpoint().as_str().contains("apexagents"));
    }

    #[test]
    fn apex_agents_adapter_default() {
        let adapter = ApexAgentsAdapter::default();
        assert_eq!(adapter.id().as_str(), "apex-agents");
    }

    #[test]
    fn apex_agents_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/apex").unwrap();
        let adapter = ApexAgentsAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn apex_agents_parse_valid_json() {
        let adapter = ApexAgentsAdapter::default();
        let raw_json = r#"{"task_completion_rate": 0.78}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let apex_raw = result.unwrap();
        assert_eq!(apex_raw.task_completion_rate, 0.78);
    }

    #[test]
    fn apex_agents_parse_invalid_json() {
        let adapter = ApexAgentsAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(ApexAgentsError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn apex_agents_parse_malformed_json() {
        let adapter = ApexAgentsAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn apex_agents_to_evidence_valid() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: 0.78,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(
            evidence_vec.len(),
            1,
            "APEX-Agents produces one evidence entry"
        );

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "apex-agents");
        assert_eq!(evidence.measurement.as_str(), "task-completion-rate");
        assert_eq!(evidence.reliability_percentile, 95);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.78);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn apex_agents_to_evidence_zero_rate() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn apex_agents_to_evidence_maximum_rate() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn apex_agents_to_evidence_out_of_bounds_high() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ApexAgentsError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn apex_agents_to_evidence_out_of_bounds_low() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ApexAgentsError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn apex_agents_to_evidence_provenance() {
        let adapter = ApexAgentsAdapter::default();
        let raw = ApexAgentsRaw {
            task_completion_rate: 0.78,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence
            .provenance
            .source_url
            .as_str()
            .contains("apexagents"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "apex-agents-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.78");
    }

    #[test]
    fn apex_agents_round_trip() {
        let adapter = ApexAgentsAdapter::default();
        let raw_json = r#"{"task_completion_rate": 0.78}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let apex_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(apex_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "apex-agents");
        assert_eq!(evidence.reliability_percentile, 95);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.78),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn apex_agents_error_display() {
        let err1 = ApexAgentsError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = ApexAgentsError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
