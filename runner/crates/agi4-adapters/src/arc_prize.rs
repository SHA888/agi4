//! ARC Prize adapter for generality and environmental transfer conjuncts.
//!
//! Ingests pass@1 accuracy scores from ARC-AGI-2 (private split) and ARC-AGI-3
//! (interactive task private split) from the ARC Prize leaderboard.
//! Returns two Evidence values: one per benchmark.

use crate::{ModelId, Source};
use agi4_core::evidence::{
    BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use url::Url;

/// ARC Prize leaderboard data: pass@1 scores for both benchmarks.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcPrizeRaw {
    /// ARC-AGI-2 private split pass@1 score.
    pub arc_agi_2: ArcAgi2Score,
    /// ARC-AGI-3 interactive task private split pass@1 score.
    pub arc_agi_3: ArcAgi3Score,
}

/// ARC-AGI-2 benchmark score.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcAgi2Score {
    /// Pass@1 accuracy on private split (0.0 to 1.0).
    pub pass_rate: f64,
}

/// ARC-AGI-3 benchmark score.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcAgi3Score {
    /// Pass@1 accuracy on interactive task private split (0.0 to 1.0).
    pub pass_rate: f64,
}

/// Error type for ARC Prize adapter operations.
#[derive(Debug, Clone)]
pub enum ArcPrizeError {
    /// JSON parsing failed.
    ParseError(String),
    /// Value validation failed (e.g., out-of-bounds score).
    ValidationError(String),
}

impl fmt::Display for ArcPrizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "ARC Prize parse error: {}", msg),
            Self::ValidationError(msg) => write!(f, "ARC Prize validation error: {}", msg),
        }
    }
}

impl Error for ArcPrizeError {}

/// ARC Prize adapter for both ARC-AGI-2 and ARC-AGI-3 benchmarks.
pub struct ArcPrizeAdapter {
    /// Canonical ARC Prize leaderboard endpoint.
    endpoint: Url,
}

impl ArcPrizeAdapter {
    /// Create a new ARC Prize adapter with the canonical endpoint.
    pub fn new() -> Result<Self, ArcPrizeError> {
        let endpoint = Url::parse("https://arcprize.org/api/leaderboard")
            .map_err(|e| ArcPrizeError::ParseError(format!("invalid endpoint URL: {}", e)))?;
        Ok(Self { endpoint })
    }

    /// Create an ARC Prize adapter with a custom endpoint (for testing).
    pub fn with_endpoint(endpoint: Url) -> Self {
        Self { endpoint }
    }
}

impl Default for ArcPrizeAdapter {
    fn default() -> Self {
        Self::new().expect("default ARC Prize endpoint should be valid")
    }
}

impl Source for ArcPrizeAdapter {
    type Raw = ArcPrizeRaw;
    type Error = ArcPrizeError;

    fn id(&self) -> SourceId {
        // This adapter is a multi-source, but the Source trait expects one id().
        // We emit two Evidence entries with different source IDs in to_evidence().
        // For consistency, we use the first source as the primary ID.
        SourceId::new("arc-prize-leaderboard")
    }

    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error> {
        serde_json::from_str::<ArcPrizeRaw>(raw)
            .map_err(|e| ArcPrizeError::ParseError(format!("failed to deserialize JSON: {}", e)))
    }

    fn to_evidence(&self, raw: Self::Raw, _model: &ModelId) -> Result<Vec<Evidence>, Self::Error> {
        let mut evidence_vec = Vec::new();

        // Validate and emit ARC-AGI-2 evidence
        let arc_agi_2_fraction = BoundedFraction::new(raw.arc_agi_2.pass_rate)
            .map_err(|e| ArcPrizeError::ValidationError(format!("ARC-AGI-2 validation: {}", e)))?;

        let arc_agi_2_evidence = Evidence {
            source: SourceId::new("arc-agi-2"),
            measurement: MeasurementId::new("pass@1-private-split"),
            value: SourceValue::Fraction(arc_agi_2_fraction),
            reliability_percentile: 95, // Per SPEC §2.1
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("arc-prize-v1".to_string()),
                raw_value: raw.arc_agi_2.pass_rate.to_string(),
            },
        };
        evidence_vec.push(arc_agi_2_evidence);

        // Validate and emit ARC-AGI-3 evidence
        let arc_agi_3_fraction = BoundedFraction::new(raw.arc_agi_3.pass_rate)
            .map_err(|e| ArcPrizeError::ValidationError(format!("ARC-AGI-3 validation: {}", e)))?;

        let arc_agi_3_evidence = Evidence {
            source: SourceId::new("arc-agi-3"),
            measurement: MeasurementId::new("pass@1-interactive-private"),
            value: SourceValue::Fraction(arc_agi_3_fraction),
            reliability_percentile: 80, // Per SPEC §2.1 (exploration variance)
            provenance: Provenance {
                source_url: self.endpoint.clone(),
                fetch_timestamp: chrono::Utc::now(),
                source_version: Some("arc-prize-v1".to_string()),
                raw_value: raw.arc_agi_3.pass_rate.to_string(),
            },
        };
        evidence_vec.push(arc_agi_3_evidence);

        Ok(evidence_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_prize_adapter_new() {
        let adapter = ArcPrizeAdapter::new().expect("should create adapter");
        assert_eq!(adapter.id().as_str(), "arc-prize-leaderboard");
        assert!(adapter.endpoint().as_str().contains("arcprize.org"));
    }

    #[test]
    fn arc_prize_adapter_default() {
        let adapter = ArcPrizeAdapter::default();
        assert_eq!(adapter.id().as_str(), "arc-prize-leaderboard");
    }

    #[test]
    fn arc_prize_adapter_with_custom_endpoint() {
        let custom_url = Url::parse("http://localhost:8080/arc").unwrap();
        let adapter = ArcPrizeAdapter::with_endpoint(custom_url.clone());
        assert_eq!(adapter.endpoint(), &custom_url);
    }

    #[test]
    fn arc_prize_parse_valid_json() {
        let adapter = ArcPrizeAdapter::default();
        let raw_json = r#"{"arc_agi_2": {"pass_rate": 0.87}, "arc_agi_3": {"pass_rate": 0.52}}"#;
        let result = adapter.parse(raw_json);
        assert!(result.is_ok());
        let arc_raw = result.unwrap();
        assert_eq!(arc_raw.arc_agi_2.pass_rate, 0.87);
        assert_eq!(arc_raw.arc_agi_3.pass_rate, 0.52);
    }

    #[test]
    fn arc_prize_parse_invalid_json() {
        let adapter = ArcPrizeAdapter::default();
        let invalid_json = r#"{"invalid": "schema"}"#;
        let result = adapter.parse(invalid_json);
        assert!(result.is_err());
        match result {
            Err(ArcPrizeError::ParseError(_)) => {}
            _ => panic!("expected ParseError"),
        }
    }

    #[test]
    fn arc_prize_parse_malformed_json() {
        let adapter = ArcPrizeAdapter::default();
        let malformed = "not valid json";
        let result = adapter.parse(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn arc_prize_to_evidence_valid() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: 0.87 },
            arc_agi_3: ArcAgi3Score { pass_rate: 0.52 },
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 2, "should emit 2 evidence entries");

        // Check ARC-AGI-2
        let arc_agi_2 = &evidence_vec[0];
        assert_eq!(arc_agi_2.source.as_str(), "arc-agi-2");
        assert_eq!(arc_agi_2.measurement.as_str(), "pass@1-private-split");
        assert_eq!(arc_agi_2.reliability_percentile, 95);

        match &arc_agi_2.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.87);
            }
            _ => panic!("expected Fraction value"),
        }

        // Check ARC-AGI-3
        let arc_agi_3 = &evidence_vec[1];
        assert_eq!(arc_agi_3.source.as_str(), "arc-agi-3");
        assert_eq!(arc_agi_3.measurement.as_str(), "pass@1-interactive-private");
        assert_eq!(arc_agi_3.reliability_percentile, 80);

        match &arc_agi_3.value {
            SourceValue::Fraction(frac) => {
                assert_eq!(frac.value(), 0.52);
            }
            _ => panic!("expected Fraction value"),
        }
    }

    #[test]
    fn arc_prize_to_evidence_zero_scores() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: 0.0 },
            arc_agi_3: ArcAgi3Score { pass_rate: 0.0 },
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 2);
    }

    #[test]
    fn arc_prize_to_evidence_maximum_scores() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: 1.0 },
            arc_agi_3: ArcAgi3Score { pass_rate: 1.0 },
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_ok());
        let evidence_vec = result.unwrap();
        assert_eq!(evidence_vec.len(), 2);
    }

    #[test]
    fn arc_prize_to_evidence_out_of_bounds_high() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: 1.5 },
            arc_agi_3: ArcAgi3Score { pass_rate: 0.5 },
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ArcPrizeError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn arc_prize_to_evidence_out_of_bounds_low() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: -0.1 },
            arc_agi_3: ArcAgi3Score { pass_rate: 0.5 },
        };
        let model = ModelId::new("test-model");
        let result = adapter.to_evidence(raw, &model);

        assert!(result.is_err());
        match result {
            Err(ArcPrizeError::ValidationError(_)) => {}
            _ => panic!("expected ValidationError"),
        }
    }

    #[test]
    fn arc_prize_to_evidence_provenance() {
        let adapter = ArcPrizeAdapter::default();
        let raw = ArcPrizeRaw {
            arc_agi_2: ArcAgi2Score { pass_rate: 0.87 },
            arc_agi_3: ArcAgi3Score { pass_rate: 0.52 },
        };
        let model = ModelId::new("test-model");
        let evidence_vec = adapter.to_evidence(raw, &model).unwrap();

        for evidence in &evidence_vec {
            assert!(evidence
                .provenance
                .source_url
                .as_str()
                .contains("arcprize.org"));
            assert!(evidence.provenance.source_version.is_some());
            assert_eq!(
                evidence.provenance.source_version.as_ref().unwrap(),
                "arc-prize-v1"
            );
            assert!(!evidence.provenance.raw_value.is_empty());
        }
    }

    #[test]
    fn arc_prize_round_trip() {
        let adapter = ArcPrizeAdapter::default();
        let raw_json = r#"{"arc_agi_2": {"pass_rate": 0.87}, "arc_agi_3": {"pass_rate": 0.52}}"#;
        let model = ModelId::new("test-model");

        // Parse JSON
        let arc_raw = adapter.parse(raw_json).expect("should parse");

        // Convert to evidence
        let evidence_vec = adapter
            .to_evidence(arc_raw, &model)
            .expect("should convert");

        // Verify count
        assert_eq!(evidence_vec.len(), 2);

        // Verify ARC-AGI-2
        let arc_agi_2 = &evidence_vec[0];
        assert_eq!(arc_agi_2.source.as_str(), "arc-agi-2");
        assert_eq!(arc_agi_2.reliability_percentile, 95);

        match &arc_agi_2.value {
            SourceValue::Fraction(h) => assert_eq!(h.value(), 0.87),
            _ => panic!("expected Fraction"),
        }

        // Verify ARC-AGI-3
        let arc_agi_3 = &evidence_vec[1];
        assert_eq!(arc_agi_3.source.as_str(), "arc-agi-3");
        assert_eq!(arc_agi_3.reliability_percentile, 80);

        match &arc_agi_3.value {
            SourceValue::Fraction(h) => assert_eq!(h.value(), 0.52),
            _ => panic!("expected Fraction"),
        }
    }

    #[test]
    fn arc_prize_error_display() {
        let err1 = ArcPrizeError::ParseError("test error".to_string());
        assert!(err1.to_string().contains("parse error"));

        let err2 = ArcPrizeError::ValidationError("invalid value".to_string());
        assert!(err2.to_string().contains("validation error"));
    }
}
