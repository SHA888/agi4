//! METR time-horizon adapter for autonomous agency conjunct.
//!
//! Ingests the 80%-time horizon metric: the quantile of time required for autonomous task completion.
//! Returns a single Hours value per model.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    Evidence, MeasurementId, NonNegativeHours, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// METR time-horizon measurement: 80%-quantile hours for autonomous task completion.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetrRaw {
    /// The 80%-time horizon value in hours.
    pub value: f64,
}

/// Error type for METR adapter operations.
#[derive(Debug, Clone)]
pub enum MetrError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., negative hours).
    ValidationError(String),
}

impl fmt::Display for MetrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "METR parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "METR validation error: {}", msg),
        }
    }
}

impl Error for MetrError {}

/// METR adapter for the time-horizon measurement.
pub struct MetrAdapter {
    /// Canonical METR API endpoint.
    endpoint: Url,
}

impl MetrAdapter {
    /// Create a new METR adapter with the canonical endpoint.
    pub fn new() -> Result<Self, MetrError> {
        let endpoint = Url::parse("https://metr.org/api/time-horizon")
            .map_err(|e| MetrError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create a METR adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for MetrAdapter {
    fn default() -> Self {
        Self::new().expect("default METR endpoint should be valid")
    }
}

impl Source for MetrAdapter {
    type Raw = MetrRaw;
    type Error = MetrError;

    fn id(&self) -> SourceId {
        SourceId::new("metr-80pct-time-horizon")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<MetrRaw>(raw)
            .map_err(|e| MetrError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct NonNegativeHours
        let hours = NonNegativeHours::new(raw.value)
            .map_err(|e| MetrError::ValidationError(format!("invalid hours value: {}", e)))?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("80pct-time-horizon"),
            value: SourceValue::Hours(hours),
            reliability_percentile: 80,
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("metr-v1".to_string()),
                raw_value: raw.value.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metr_adapter_new() {
        let adapter = MetrAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "metr-80pct-time-horizon");
        assert!(adapter.endpoint().as_str().contains("metr.org"));
    }

    #[test]
    fn metr_adapter_default() {
        let adapter = MetrAdapter::default();
        assert_eq!(adapter.id().as_str(), "metr-80pct-time-horizon");
    }

    #[test]
    fn metr_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/metr").unwrap();
        let adapter = MetrAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn metr_parse_valid_json() {
        let adapter = MetrAdapter::default();
        let raw_json = r#"{"value": 168.0}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let metr_raw = result.unwrap();
        assert_eq!(metr_raw.value, 168.0);
    }

    #[test]
    fn metr_parse_invalid_json() {
        let adapter = MetrAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(MetrError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn metr_parse_malformed_json() {
        let adapter = MetrAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn metr_to_evidence_valid() {
        let adapter = MetrAdapter::default();
        let raw = MetrRaw { value: 168.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);

        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "metr-80pct-time-horizon");
        assert_eq!(evidence.measurement.as_str(), "80pct-time-horizon");
        assert_eq!(evidence.reliability_percentile, 80);

        match &evidence.value {
            SourceValue::Hours(hours) => {
                assert_eq!(hours.value(), 168.0);
            }
            _ => panic!("expected Hours value"),
        }
    }

    #[test]
    fn metr_to_evidence_zero_hours() {
        let adapter = MetrAdapter::default();
        let raw = MetrRaw { value: 0.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn metr_to_evidence_negative_hours() {
        let adapter = MetrAdapter::default();
        let raw = MetrRaw { value: -10.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(MetrError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn metr_to_evidence_large_hours() {
        let adapter = MetrAdapter::default();
        let raw = MetrRaw {
            value: 720.0, // one month
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
    }

    #[test]
    fn metr_to_evidence_provenance() {
        let adapter = MetrAdapter::default();
        let raw = MetrRaw { value: 168.0 };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("metr.org"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "metr-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "168");
    }

    #[test]
    fn metr_round_trip() {
        let adapter = MetrAdapter::default();
        let raw_json = r#"{"value": 168.0}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let metr_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(metr_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "metr-80pct-time-horizon");
        assert_eq!(evidence.reliability_percentile, 80);

        match &evidence.value {
            SourceValue::Hours(h) => assert_eq!(h.value(), 168.0),
            _ => panic!("expected Hours"),
        }
    }

    #[test]
    fn metr_error_display() {
        let err1 = MetrError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = MetrError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
