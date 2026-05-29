//! GDPval (Artificial Analysis) adapter for economic substitutability conjunct.
//!
//! Ingests win+tie rate from the GDPval benchmark.
//! Returns a single Fraction value representing performance vs industry experts.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// GDPval benchmark data: win+tie rate vs industry experts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GdpvalRaw {
    /// Win+tie rate as a fraction (0.0 to 1.0).
    pub win_tie_rate: f64,
}

/// Error type for GDPval adapter operations.
#[derive(Debug, Clone)]
pub enum GdpvalError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds win+tie rate).
    ValidationError(String),
}

impl fmt::Display for GdpvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "GDPval parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "GDPval validation error: {}", msg),
        }
    }
}

impl Error for GdpvalError {}

/// GDPval adapter for the win+tie rate measurement.
pub struct GdpvalAdapter {
    /// Canonical GDPval (Artificial Analysis) endpoint.
    endpoint: Url,
}

impl GdpvalAdapter {
    /// Create a new GDPval adapter with the canonical endpoint.
    pub fn new() -> Result<Self, GdpvalError> {
        let endpoint = Url::parse("https://artificial-analysis.com/api/gdpval")
            .map_err(|e| GdpvalError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create a GDPval adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for GdpvalAdapter {
    fn default() -> Self {
        Self::new().expect("default GDPval endpoint should be valid")
    }
}

impl Source for GdpvalAdapter {
    type Raw = GdpvalRaw;
    type Error = GdpvalError;

    fn id(&self) -> SourceId {
        SourceId::new("gdpval")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<GdpvalRaw>(raw)
            .map_err(|e| GdpvalError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let win_tie_rate = BoundedFraction::new(raw.win_tie_rate).map_err(|e| {
            GdpvalError::ValidationError(format!("invalid win+tie rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("win-tie-rate"),
            value: SourceValue::Fraction(win_tie_rate),
            reliability_percentile: 95, // Per SPEC §2.2
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("gdpval-v1".to_string()),
                raw_value: raw.win_tie_rate.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gdpval_adapter_new() {
        let adapter = GdpvalAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "gdpval");
        assert!(adapter.endpoint().as_str().contains("artificial-analysis"));
    }

    #[test]
    fn gdpval_adapter_default() {
        let adapter = GdpvalAdapter::default();
        assert_eq!(adapter.id().as_str(), "gdpval");
    }

    #[test]
    fn gdpval_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/gdpval").unwrap();
        let adapter = GdpvalAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn gdpval_parse_valid_json() {
        let adapter = GdpvalAdapter::default();
        let raw_json = r#"{"win_tie_rate": 0.87}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let gdpval_raw = result.unwrap();
        assert_eq!(gdpval_raw.win_tie_rate, 0.87);
    }

    #[test]
    fn gdpval_parse_invalid_json() {
        let adapter = GdpvalAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(GdpvalError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn gdpval_parse_malformed_json() {
        let adapter = GdpvalAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn gdpval_to_evidence_valid() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: 0.87 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1, "GDPval produces one evidence entry");

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "gdpval");
        assert_eq!(evidence.measurement.as_str(), "win-tie-rate");
        assert_eq!(evidence.reliability_percentile, 95);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.87);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn gdpval_to_evidence_zero_rate() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: 0.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn gdpval_to_evidence_maximum_rate() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: 1.0 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn gdpval_to_evidence_out_of_bounds_high() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: 1.5 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(GdpvalError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn gdpval_to_evidence_out_of_bounds_low() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: -0.1 };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(GdpvalError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn gdpval_to_evidence_provenance() {
        let adapter = GdpvalAdapter::default();
        let raw = GdpvalRaw { win_tie_rate: 0.87 };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence
            .provenance
            .source_url
            .as_str()
            .contains("artificial-analysis"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "gdpval-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.87");
    }

    #[test]
    fn gdpval_round_trip() {
        let adapter = GdpvalAdapter::default();
        let raw_json = r#"{"win_tie_rate": 0.87}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let gdpval_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(gdpval_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "gdpval");
        assert_eq!(evidence.reliability_percentile, 95);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.87),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn gdpval_error_display() {
        let err1 = GdpvalError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = GdpvalError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
