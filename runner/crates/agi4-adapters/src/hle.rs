//! HLE (Humanity's Last Exam) adapter for generality conjunct.
//!
//! Ingests overall accuracy from the Humanity's Last Exam benchmark.
//! Returns a single Fraction value representing performance on the evaluation suite.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// HLE benchmark data: overall accuracy on the evaluation suite.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HleRaw {
    /// Overall accuracy as a fraction (0.0 to 1.0).
    pub overall_accuracy: f64,
}

/// Error type for HLE adapter operations.
#[derive(Debug, Clone)]
pub enum HleError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds accuracy).
    ValidationError(String),
}

impl fmt::Display for HleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "HLE parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "HLE validation error: {}", msg),
        }
    }
}

impl Error for HleError {}

/// HLE adapter for the overall accuracy measurement.
pub struct HleAdapter {
    /// Canonical HLE endpoint (CAIS-operated).
    endpoint: Url,
}

impl HleAdapter {
    /// Create a new HLE adapter with the canonical endpoint.
    pub fn new() -> Result<Self, HleError> {
        let endpoint = Url::parse("https://humlastexam.cais.net/api/results")
            .map_err(|e| HleError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create an HLE adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for HleAdapter {
    fn default() -> Self {
        Self::new().expect("default HLE endpoint should be valid")
    }
}

impl Source for HleAdapter {
    type Raw = HleRaw;
    type Error = HleError;

    fn id(&self) -> SourceId {
        SourceId::new("hle")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<HleRaw>(raw)
            .map_err(|e| HleError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let accuracy = BoundedFraction::new(raw.overall_accuracy)
            .map_err(|e| HleError::ValidationError(format!("invalid accuracy value: {}", e)))?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("overall-accuracy"),
            value: SourceValue::Fraction(accuracy),
            reliability_percentile: 95, // Per SPEC §2.1
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("hle-v1".to_string()),
                raw_value: raw.overall_accuracy.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hle_adapter_new() {
        let adapter = HleAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "hle");
        assert!(adapter.endpoint().as_str().contains("cais"));
    }

    #[test]
    fn hle_adapter_default() {
        let adapter = HleAdapter::default();
        assert_eq!(adapter.id().as_str(), "hle");
    }

    #[test]
    fn hle_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/hle").unwrap();
        let adapter = HleAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn hle_parse_valid_json() {
        let adapter = HleAdapter::default();
        let raw_json = r#"{"overall_accuracy": 0.82}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let hle_raw = result.unwrap();
        assert_eq!(hle_raw.overall_accuracy, 0.82);
    }

    #[test]
    fn hle_parse_invalid_json() {
        let adapter = HleAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(HleError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn hle_parse_malformed_json() {
        let adapter = HleAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn hle_to_evidence_valid() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: 0.82,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1, "HLE produces one evidence entry");

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "hle");
        assert_eq!(evidence.measurement.as_str(), "overall-accuracy");
        assert_eq!(evidence.reliability_percentile, 95);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.82);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn hle_to_evidence_zero_accuracy() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn hle_to_evidence_maximum_accuracy() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn hle_to_evidence_out_of_bounds_high() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(HleError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn hle_to_evidence_out_of_bounds_low() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(HleError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn hle_to_evidence_provenance() {
        let adapter = HleAdapter::default();
        let raw = HleRaw {
            overall_accuracy: 0.82,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("cais"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "hle-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.82");
    }

    #[test]
    fn hle_round_trip() {
        let adapter = HleAdapter::default();
        let raw_json = r#"{"overall_accuracy": 0.82}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let hle_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(hle_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "hle");
        assert_eq!(evidence.reliability_percentile, 95);

        match &evidence.value {
            SourceValue::Fraction(h) => assert_eq!(h.value(), 0.82),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn hle_error_display() {
        let err1 = HleError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = HleError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
