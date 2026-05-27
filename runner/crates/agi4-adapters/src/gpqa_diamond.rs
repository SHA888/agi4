//! GPQA Diamond adapter for generality conjunct.
//!
//! Ingests accuracy from the GPQA Diamond benchmark.
//! Returns a single Fraction value representing performance on the evaluation suite.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// GPQA Diamond benchmark data: accuracy on the evaluation suite.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpqaDiamondRaw {
    /// Accuracy as a fraction (0.0 to 1.0).
    pub accuracy: f64,
}

/// Error type for GPQA Diamond adapter operations.
#[derive(Debug, Clone)]
pub enum GpqaDiamondError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds accuracy).
    ValidationError(String),
}

impl fmt::Display for GpqaDiamondError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "GPQA Diamond parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "GPQA Diamond validation error: {}", msg),
        }
    }
}

impl Error for GpqaDiamondError {}

/// GPQA Diamond adapter for the accuracy measurement.
pub struct GpqaDiamondAdapter {
    /// Canonical GPQA Diamond endpoint.
    endpoint: Url,
}

impl GpqaDiamondAdapter {
    /// Create a new GPQA Diamond adapter with the canonical endpoint.
    pub fn new() -> Result<Self, GpqaDiamondError> {
        let endpoint = Url::parse("https://api.gpqabenchmark.com/diamond/results")
            .map_err(|e| GpqaDiamondError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create a GPQA Diamond adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for GpqaDiamondAdapter {
    fn default() -> Self {
        Self::new().expect("default GPQA Diamond endpoint should be valid")
    }
}

impl Source for GpqaDiamondAdapter {
    type Raw = GpqaDiamondRaw;
    type Error = GpqaDiamondError;

    fn id(&self) -> SourceId {
        SourceId::new("gpqa-diamond")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<GpqaDiamondRaw>(raw)
            .map_err(|e| GpqaDiamondError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let accuracy = BoundedFraction::new(raw.accuracy).map_err(|e| {
            GpqaDiamondError::ValidationError(format!("invalid accuracy value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("accuracy"),
            value: SourceValue::Fraction(accuracy),
            reliability_percentile: 95, // Per SPEC §2.1
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("gpqa-diamond-v1".to_string()),
                raw_value: raw.accuracy.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpqa_diamond_adapter_new() {
        let adapter = GpqaDiamondAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "gpqa-diamond");
        assert!(adapter.endpoint().as_str().contains("gpqa"));
    }

    #[test]
    fn gpqa_diamond_adapter_default() {
        let adapter = GpqaDiamondAdapter::default();
        assert_eq!(adapter.id().as_str(), "gpqa-diamond");
    }

    #[test]
    fn gpqa_diamond_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/gpqa").unwrap();
        let adapter = GpqaDiamondAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn gpqa_diamond_parse_valid_json() {
        let adapter = GpqaDiamondAdapter::default();
        let raw_json = r#"{"accuracy": 0.91}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let gpqa_raw = result.unwrap();
        assert_eq!(gpqa_raw.accuracy, 0.91);
    }

    #[test]
    fn gpqa_diamond_parse_invalid_json() {
        let adapter = GpqaDiamondAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(GpqaDiamondError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn gpqa_diamond_parse_malformed_json() {
        let adapter = GpqaDiamondAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn gpqa_diamond_to_evidence_valid() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: 0.91 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(
            evidence_vec.len(),
            1,
            "GPQA Diamond produces one evidence entry"
        );

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "gpqa-diamond");
        assert_eq!(evidence.measurement.as_str(), "accuracy");
        assert_eq!(evidence.reliability_percentile, 95);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.91);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn gpqa_diamond_to_evidence_zero_accuracy() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: 0.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn gpqa_diamond_to_evidence_maximum_accuracy() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: 1.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn gpqa_diamond_to_evidence_out_of_bounds_high() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: 1.5 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(GpqaDiamondError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn gpqa_diamond_to_evidence_out_of_bounds_low() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: -0.1 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(GpqaDiamondError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn gpqa_diamond_to_evidence_provenance() {
        let adapter = GpqaDiamondAdapter::default();
        let raw = GpqaDiamondRaw { accuracy: 0.91 };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("gpqa"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "gpqa-diamond-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.91");
    }

    #[test]
    fn gpqa_diamond_round_trip() {
        let adapter = GpqaDiamondAdapter::default();
        let raw_json = r#"{"accuracy": 0.91}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let gpqa_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(gpqa_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "gpqa-diamond");
        assert_eq!(evidence.reliability_percentile, 95);

        match &evidence.value {
            SourceValue::Fraction(h) => assert_eq!(h.value(), 0.91),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn gpqa_diamond_error_display() {
        let err1 = GpqaDiamondError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = GpqaDiamondError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
