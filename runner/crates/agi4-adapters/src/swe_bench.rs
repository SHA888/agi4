//! SWE-bench Verified pass@5 adapter for autonomous agency conjunct.
//!
//! Ingests pass@5 success rate from the SWE-bench Verified benchmark.
//! Rejects pass@1-only data. Returns a single Fraction value representing
//! pass@5 performance.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// SWE-bench Verified benchmark data: pass@k success rate.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SweBenchRaw {
    /// The value of k in pass@k. Must be >= 5.
    pub pass_at_k: u32,
    /// Success rate as a fraction (0.0 to 1.0).
    pub success_rate: f64,
}

/// Error type for SWE-bench adapter operations.
#[derive(Debug, Clone)]
pub enum SweBenchError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., pass@1-only data, out-of-bounds rate).
    ValidationError(String),
}

impl fmt::Display for SweBenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "SWE-bench parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "SWE-bench validation error: {}", msg),
        }
    }
}

impl Error for SweBenchError {}

/// SWE-bench Verified pass@5 adapter for the success rate measurement.
pub struct SweBenchAdapter {
    /// Canonical SWE-bench endpoint.
    endpoint: Url,
}

impl SweBenchAdapter {
    /// Create a new SWE-bench adapter with the canonical endpoint.
    pub fn new() -> Result<Self, SweBenchError> {
        let endpoint = Url::parse("https://swe-bench.github.io/api/results")
            .map_err(|e| SweBenchError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create a SWE-bench adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for SweBenchAdapter {
    fn default() -> Self {
        Self::new().expect("default SWE-bench endpoint should be valid")
    }
}

impl Source for SweBenchAdapter {
    type Raw = SweBenchRaw;
    type Error = SweBenchError;

    fn id(&self) -> SourceId {
        SourceId::new("swe-bench")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<SweBenchRaw>(raw)
            .map_err(|e| SweBenchError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        // Reject pass@1-only data. Per SPEC §2.4, we require pass@k where k >= 5.
        if raw.pass_at_k < 5 {
            return Err(SweBenchError::ValidationError(format!(
                "pass@{} not acceptable; SPEC requires pass@k where k >= 5",
                raw.pass_at_k
            )));
        }

        // Validate and construct BoundedFraction
        let success_rate = BoundedFraction::new(raw.success_rate).map_err(|e| {
            SweBenchError::ValidationError(format!("invalid success rate value: {}", e))
        })?;

        let evidence = Evidence {
            source: self.id(),
            measurement: MeasurementId::new(format!("pass@{}-rate", raw.pass_at_k)),
            value: SourceValue::Fraction(success_rate),
            reliability_percentile: 80, // Per SPEC §2.4
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("swe-bench-verified-v1".to_string()),
                raw_value: raw.success_rate.to_string(),
            },
        };

        Ok(vec![evidence])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swe_bench_adapter_new() {
        let adapter = SweBenchAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "swe-bench");
        assert!(adapter.endpoint().as_str().contains("swe-bench"));
    }

    #[test]
    fn swe_bench_adapter_default() {
        let adapter = SweBenchAdapter::default();
        assert_eq!(adapter.id().as_str(), "swe-bench");
    }

    #[test]
    fn swe_bench_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/swe-bench").unwrap();
        let adapter = SweBenchAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn swe_bench_parse_valid_json() {
        let adapter = SweBenchAdapter::default();
        let raw_json = r#"{"pass_at_k": 5, "success_rate": 0.91}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let swe_bench_raw = result.unwrap();
        assert_eq!(swe_bench_raw.pass_at_k, 5);
        assert_eq!(swe_bench_raw.success_rate, 0.91);
    }

    #[test]
    fn swe_bench_parse_invalid_json() {
        let adapter = SweBenchAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(SweBenchError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn swe_bench_parse_malformed_json() {
        let adapter = SweBenchAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn swe_bench_to_evidence_valid() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: 0.91,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(
            evidence_vec.len(),
            1,
            "SWE-bench produces one evidence entry"
        );

        let evidence = &evidence_vec[0];

        // Verify metadata
        assert_eq!(evidence.source.as_str(), "swe-bench");
        assert_eq!(evidence.measurement.as_str(), "pass@5-rate");
        assert_eq!(evidence.reliability_percentile, 80);

        // Verify value type and bounds
        match &evidence.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.91);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn swe_bench_to_evidence_rejects_pass_at_1() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 1,
            success_rate: 0.85,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(SweBenchError::ValidationError(msg)) => {
                assert!(msg.contains("pass@1") || msg.contains("k >= 5"));
            }
            _ => panic!("expected ValidationError for pass@1"),
        }
    }

    #[test]
    fn swe_bench_to_evidence_rejects_pass_at_3() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 3,
            success_rate: 0.88,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(SweBenchError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError for pass@3"),
        }
    }

    #[test]
    fn swe_bench_to_evidence_accepts_pass_at_10() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 10,
            success_rate: 0.94,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.measurement.as_str(), "pass@10-rate");
    }

    #[test]
    fn swe_bench_to_evidence_zero_rate() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: 0.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn swe_bench_to_evidence_maximum_rate() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: 1.0,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 1);
    }

    #[test]
    fn swe_bench_to_evidence_out_of_bounds_high() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: 1.5,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(SweBenchError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn swe_bench_to_evidence_out_of_bounds_low() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: -0.1,
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(SweBenchError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn swe_bench_to_evidence_provenance() {
        let adapter = SweBenchAdapter::default();
        let raw = SweBenchRaw {
            pass_at_k: 5,
            success_rate: 0.91,
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();
        let evidence = &evidence_vec[0];

        assert!(
            evidence
                .provenance
                .source_url
                .as_str()
                .contains("swe-bench")
        );
        assert!(evidence.provenance.source_version.is_some());
        assert_eq!(
            evidence.provenance.source_version.as_ref().unwrap(),
            "swe-bench-verified-v1"
        );
        assert_eq!(evidence.provenance.raw_value, "0.91");
    }

    #[test]
    fn swe_bench_round_trip() {
        let adapter = SweBenchAdapter::default();
        let raw_json = r#"{"pass_at_k": 5, "success_rate": 0.91}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let swe_bench_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(swe_bench_raw, &model)
            .expect("should convert");

        // Verify
        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];
        assert_eq!(evidence.source.as_str(), "swe-bench");
        assert_eq!(evidence.reliability_percentile, 80);

        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.91),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn swe_bench_error_display() {
        let err1 = SweBenchError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = SweBenchError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
