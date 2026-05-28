//! OSWorld adapter for environmental transfer conjunct.
//!
//! Ingests task completion rate from the OSWorld benchmark.
//! Returns a single Fraction value representing task completion with no domain-specific scaffolding.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// OSWorld benchmark data: task completion rate with no domain-specific scaffolding.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsworldRaw {
    /// Task completion rate as a fraction (0.0 to 1.0).
    pub task_completion_rate: f64,
}

/// Error type for OSWorld adapter operations.
#[derive(Debug, Clone)]
pub enum OsworldError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds task completion rate).
    ValidationError(String),
}

impl fmt::Display for OsworldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "OSWorld parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "OSWorld validation error: {}", msg),
        }
    }
}

impl Error for OsworldError {}

/// OSWorld adapter for the task completion rate measurement.
pub struct OsworldAdapter {
    /// Canonical OSWorld endpoint.
    endpoint: Url,
}

impl OsworldAdapter {
    /// Create a new OSWorld adapter with the canonical endpoint.
    pub fn new() -> Result<Self, OsworldError> {
        let endpoint = Url::parse("https://osworld.ai/api/results")
            .map_err(|e| OsworldError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create an OSWorld adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for OsworldAdapter {
    fn default() -> Self {
        Self::new().expect("default OSWorld endpoint should be valid")
    }
}

impl Source for OsworldAdapter {
    type Raw = OsworldRaw;
    type Error = OsworldError;

    fn id(&self) -> SourceId {
        SourceId::new("osworld")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<OsworldRaw>(raw)
            .map_err(|e| OsworldError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let task_completion_rate = BoundedFraction::new(raw.task_completion_rate).map_err(|e| {
            OsworldError::ValidationError(format!("invalid task completion rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("task-completion-rate"),
            value: SourceValue::Fraction(task_completion_rate),
            reliability_percentile: 80, // Per SPEC §2.3
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("osworld-v1".to_string()),
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
    fn osworld_adapter_new() {
        let adapter = OsworldAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "osworld");
        assert!(adapter.endpoint().as_str().contains("osworld"));
    }

    #[test]
    fn osworld_adapter_default() {
        let adapter = OsworldAdapter::default();
        assert_eq!(adapter.id().as_str(), "osworld");
    }

    #[test]
    fn osworld_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/osworld").unwrap();
        let adapter = OsworldAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn osworld_parse_valid_json() {
        let adapter = OsworldAdapter::default();
        let raw_json = r#"{"task_completion_rate": 0.88}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let osworld_raw = result.unwrap();
        assert_eq!(osworld_raw.task_completion_rate, 0.88);
    }

    #[test]
    fn osworld_parse_invalid_json() {
        let adapter = OsworldAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(OsworldError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn osworld_parse_malformed_json() {
        let adapter = OsworldAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn osworld_to_evidence_valid() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: 0.88,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1, "OSWorld produces one evidence entry");

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "osworld");
        assert_eq!(evidence.measurement.as_str(), "task-completion-rate");
        assert_eq!(evidence.reliability_percentile, 80);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.88);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn osworld_to_evidence_zero_rate() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn osworld_to_evidence_maximum_rate() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn osworld_to_evidence_out_of_bounds_high() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(OsworldError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn osworld_to_evidence_out_of_bounds_low() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(OsworldError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn osworld_to_evidence_provenance() {
        let adapter = OsworldAdapter::default();
        let raw = OsworldRaw {
            task_completion_rate: 0.88,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("osworld"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "osworld-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.88");
    }

    #[test]
    fn osworld_round_trip() {
        let adapter = OsworldAdapter::default();
        let raw_json = r#"{"task_completion_rate": 0.88}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let osworld_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(osworld_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "osworld");
        assert_eq!(evidence.reliability_percentile, 80);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.88),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn osworld_error_display() {
        let err1 = OsworldError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = OsworldError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
