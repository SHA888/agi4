//! RLI (Remote Labor Index) adapter for economic substitutability conjunct.
//!
//! Ingests completion rate at expert-comparable quality from the Remote Labor Index.
//! Returns a single Fraction value representing performance on labor tasks.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// RLI benchmark data: completion rate at expert-comparable quality.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RliRaw {
    /// Completion rate as a fraction (0.0 to 1.0).
    pub completion_rate: f64,
}

/// Error type for RLI adapter operations.
#[derive(Debug, Clone)]
pub enum RliError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds completion rate).
    ValidationError(String),
}

impl fmt::Display for RliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "RLI parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "RLI validation error: {}", msg),
        }
    }
}

impl Error for RliError {}

/// RLI adapter for the completion rate measurement.
pub struct RliAdapter {
    /// Canonical RLI endpoint (Scale AI / METR).
    endpoint: Url,
}

impl RliAdapter {
    /// Create a new RLI adapter with the canonical endpoint.
    pub fn new() -> Result<Self, RliError> {
        let endpoint = Url::parse("https://remoteindex.scale.com/api/rli")
            .map_err(|e| RliError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create an RLI adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for RliAdapter {
    fn default() -> Self {
        Self::new().expect("default RLI endpoint should be valid")
    }
}

impl Source for RliAdapter {
    type Raw = RliRaw;
    type Error = RliError;

    fn id(&self) -> SourceId {
        SourceId::new("rli")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<RliRaw>(raw)
            .map_err(|e| RliError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let completion_rate = BoundedFraction::new(raw.completion_rate).map_err(|e| {
            RliError::ValidationError(format!("invalid completion rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("completion-rate"),
            value: SourceValue::Fraction(completion_rate),
            reliability_percentile: 95, // Per SPEC §2.2
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("rli-v1".to_string()),
                raw_value: raw.completion_rate.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rli_adapter_new() {
        let adapter = RliAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "rli");
        assert!(adapter.endpoint().as_str().contains("scale"));
    }

    #[test]
    fn rli_adapter_default() {
        let adapter = RliAdapter::default();
        assert_eq!(adapter.id().as_str(), "rli");
    }

    #[test]
    fn rli_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/rli").unwrap();
        let adapter = RliAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn rli_parse_valid_json() {
        let adapter = RliAdapter::default();
        let raw_json = r#"{"completion_rate": 0.72}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let rli_raw = result.unwrap();
        assert_eq!(rli_raw.completion_rate, 0.72);
    }

    #[test]
    fn rli_parse_invalid_json() {
        let adapter = RliAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(RliError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn rli_parse_malformed_json() {
        let adapter = RliAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn rli_to_evidence_valid() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: 0.72,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1, "RLI produces one evidence entry");

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "rli");
        assert_eq!(evidence.measurement.as_str(), "completion-rate");
        assert_eq!(evidence.reliability_percentile, 95);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.72);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn rli_to_evidence_zero_rate() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn rli_to_evidence_maximum_rate() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn rli_to_evidence_out_of_bounds_high() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(RliError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn rli_to_evidence_out_of_bounds_low() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(RliError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn rli_to_evidence_provenance() {
        let adapter = RliAdapter::default();
        let raw = RliRaw {
            completion_rate: 0.72,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("scale"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "rli-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.72");
    }

    #[test]
    fn rli_round_trip() {
        let adapter = RliAdapter::default();
        let raw_json = r#"{"completion_rate": 0.72}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let rli_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(rli_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "rli");
        assert_eq!(evidence.reliability_percentile, 95);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.72),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn rli_error_display() {
        let err1 = RliError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = RliError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
