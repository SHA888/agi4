//! RE-Bench adapter for autonomous agency conjunct.
//!
//! Ingests AI research engineering task success rate from the RE-Bench benchmark.
//! Returns a single Fraction value representing task success performance.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// RE-Bench benchmark data: AI research engineering task success rate.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReBenchRaw {
    /// Task success rate as a fraction (0.0 to 1.0).
    pub task_success_rate: f64,
}

/// Error type for RE-Bench adapter operations.
#[derive(Debug, Clone)]
pub enum ReBenchError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds task success rate).
    ValidationError(String),
}

impl fmt::Display for ReBenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "RE-Bench parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "RE-Bench validation error: {}", msg),
        }
    }
}

impl Error for ReBenchError {}

/// RE-Bench adapter for the task success rate measurement.
pub struct ReBenchAdapter {
    /// Canonical RE-Bench endpoint (METR).
    endpoint: Url,
}

impl ReBenchAdapter {
    /// Create a new RE-Bench adapter with the canonical endpoint.
    pub fn new() -> Result<Self, ReBenchError> {
        let endpoint = Url::parse("https://re-bench.metr.org/api/results")
            .map_err(|e| ReBenchError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create a RE-Bench adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for ReBenchAdapter {
    fn default() -> Self {
        Self::new().expect("default RE-Bench endpoint should be valid")
    }
}

impl Source for ReBenchAdapter {
    type Raw = ReBenchRaw;
    type Error = ReBenchError;

    fn id(&self) -> SourceId {
        SourceId::new("re-bench")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<ReBenchRaw>(raw)
            .map_err(|e| ReBenchError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Validate and construct BoundedFraction
        let task_success_rate = BoundedFraction::new(raw.task_success_rate).map_err(|e| {
            ReBenchError::ValidationError(format!("invalid task success rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new("task-success-rate"),
            value: SourceValue::Fraction(task_success_rate),
            reliability_percentile: 80, // Per SPEC §2.4
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("re-bench-v1".to_string()),
                raw_value: raw.task_success_rate.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn re_bench_adapter_new() {
        let adapter = ReBenchAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "re-bench");
        assert!(adapter.endpoint().as_str().contains("metr"));
    }

    #[test]
    fn re_bench_adapter_default() {
        let adapter = ReBenchAdapter::default();
        assert_eq!(adapter.id().as_str(), "re-bench");
    }

    #[test]
    fn re_bench_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/re-bench").unwrap();
        let adapter = ReBenchAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn re_bench_parse_valid_json() {
        let adapter = ReBenchAdapter::default();
        let raw_json = r#"{"task_success_rate": 0.68}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let re_bench_raw = result.unwrap();
        assert_eq!(re_bench_raw.task_success_rate, 0.68);
    }

    #[test]
    fn re_bench_parse_invalid_json() {
        let adapter = ReBenchAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(ReBenchError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn re_bench_parse_malformed_json() {
        let adapter = ReBenchAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn re_bench_to_evidence_valid() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: 0.68,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(
            evidence_vec.len(),
            1,
            "RE-Bench produces one evidence entry"
        );

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "re-bench");
        assert_eq!(evidence.measurement.as_str(), "task-success-rate");
        assert_eq!(evidence.reliability_percentile, 80);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.68);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn re_bench_to_evidence_zero_rate() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn re_bench_to_evidence_maximum_rate() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn re_bench_to_evidence_out_of_bounds_high() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ReBenchError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn re_bench_to_evidence_out_of_bounds_low() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ReBenchError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn re_bench_to_evidence_provenance() {
        let adapter = ReBenchAdapter::default();
        let raw = ReBenchRaw {
            task_success_rate: 0.68,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(evidence.provenance.source_url.as_str().contains("metr"));
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "re-bench-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.68");
    }

    #[test]
    fn re_bench_round_trip() {
        let adapter = ReBenchAdapter::default();
        let raw_json = r#"{"task_success_rate": 0.68}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let re_bench_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(re_bench_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "re-bench");
        assert_eq!(evidence.reliability_percentile, 80);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.68),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn re_bench_error_display() {
        let err1 = ReBenchError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = ReBenchError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
