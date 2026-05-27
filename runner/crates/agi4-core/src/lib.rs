//! Pure verdict logic for AGI/4 attestation.
//!
//! This crate contains the core types and verdict function with zero external dependencies
//! beyond serialization. The verdict function is pure and total: given valid input, it always
//! returns a verdict with no panics or side effects.

pub mod conjunct;
pub mod consistency;
pub mod evaluators;
pub mod evidence;
pub mod sources;
pub mod threshold;
pub mod verdict;

pub use conjunct::{Conjunct, ConjunctStatus};
pub use evaluators::{
    evaluate_autonomous_agency, evaluate_economic_substitutability,
    evaluate_environmental_transfer, evaluate_generality,
};
pub use evidence::Evidence;
pub use verdict::{Verdict, verdict};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjunct_debug_works() {
        let c = Conjunct::Generality;
        let debug_str = format!("{:?}", c);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Generality"));
    }

    #[test]
    fn conjunct_clone_works() {
        let c1 = Conjunct::EconomicSubstitutability;
        let c2 = c1;
        assert_eq!(c1, c2);
    }

    #[test]
    fn conjunct_serialize_works() {
        let c = Conjunct::EnvironmentalTransfer;
        let json = serde_json::to_string(&c).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: Conjunct = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(c, deserialized);
    }

    #[test]
    fn conjunct_all_variants_serialize() {
        let variants = vec![
            Conjunct::Generality,
            Conjunct::EconomicSubstitutability,
            Conjunct::EnvironmentalTransfer,
            Conjunct::AutonomousAgency,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).expect("should serialize");
            let deserialized: Conjunct = serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn conjunct_status_debug_works() {
        let statuses = vec![
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Fail,
            ConjunctStatus::InsufficientData,
        ];

        for status in statuses {
            let debug_str = format!("{:?}", status);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn conjunct_status_clone_works() {
        let s1 = ConjunctStatus::Pass;
        let s2 = s1;
        assert_eq!(s1, s2);
    }

    #[test]
    fn conjunct_status_serialize_works() {
        let status = ConjunctStatus::Partial;
        let json = serde_json::to_string(&status).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: ConjunctStatus = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(status, deserialized);
    }

    #[test]
    fn evidence_debug_works() {
        use chrono::Utc;
        use evidence::{
            BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
        };
        use url::Url;

        let evidence = Evidence {
            source: SourceId::new("test-source"),
            measurement: MeasurementId::new("test-measurement"),
            value: SourceValue::Fraction(BoundedFraction::new(0.75).unwrap()),
            reliability_percentile: 95,
            provenance: Provenance {
                source_url: Url::parse("https://example.com").unwrap(),
                fetch_timestamp: Utc::now(),
                source_version: Some("1.0".to_string()),
                raw_value: "75.0%".to_string(),
            },
        };

        let debug_str = format!("{:?}", evidence);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn evidence_clone_works() {
        use chrono::Utc;
        use evidence::{
            BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
        };
        use url::Url;

        let evidence = Evidence {
            source: SourceId::new("test-source"),
            measurement: MeasurementId::new("test-measurement"),
            value: SourceValue::Fraction(BoundedFraction::new(0.75).unwrap()),
            reliability_percentile: 95,
            provenance: Provenance {
                source_url: Url::parse("https://example.com").unwrap(),
                fetch_timestamp: Utc::now(),
                source_version: Some("1.0".to_string()),
                raw_value: "75.0%".to_string(),
            },
        };

        let cloned = evidence.clone();
        assert_eq!(evidence.source, cloned.source);
    }

    #[test]
    fn evidence_serialize_works() {
        use chrono::Utc;
        use evidence::{
            BoundedFraction, Evidence, MeasurementId, Provenance, SourceId, SourceValue,
        };
        use url::Url;

        let evidence = Evidence {
            source: SourceId::new("test-source"),
            measurement: MeasurementId::new("test-measurement"),
            value: SourceValue::Fraction(BoundedFraction::new(0.75).unwrap()),
            reliability_percentile: 95,
            provenance: Provenance {
                source_url: Url::parse("https://example.com").unwrap(),
                fetch_timestamp: Utc::now(),
                source_version: Some("1.0".to_string()),
                raw_value: "75.0%".to_string(),
            },
        };

        let json = serde_json::to_string(&evidence).expect("should serialize");
        assert!(!json.is_empty());
        let _deserialized: Evidence = serde_json::from_str(&json).expect("should deserialize");
    }

    #[test]
    fn bounded_fraction_debug_works() {
        use evidence::BoundedFraction;

        let bf = BoundedFraction::new(0.5).unwrap();
        let debug_str = format!("{:?}", bf);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn bounded_fraction_clone_works() {
        use evidence::BoundedFraction;

        let bf1 = BoundedFraction::new(0.5).unwrap();
        let bf2 = bf1;
        assert_eq!(bf1, bf2);
    }

    #[test]
    fn bounded_fraction_serialize_works() {
        use evidence::BoundedFraction;

        let bf = BoundedFraction::new(0.5).unwrap();
        let json = serde_json::to_string(&bf).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: BoundedFraction =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(bf, deserialized);
    }

    #[test]
    fn non_negative_hours_debug_works() {
        use evidence::NonNegativeHours;

        let nnh = NonNegativeHours::new(24.0).unwrap();
        let debug_str = format!("{:?}", nnh);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn non_negative_hours_clone_works() {
        use evidence::NonNegativeHours;

        let nnh1 = NonNegativeHours::new(24.0).unwrap();
        let nnh2 = nnh1;
        assert_eq!(nnh1, nnh2);
    }

    #[test]
    fn non_negative_hours_serialize_works() {
        use evidence::NonNegativeHours;

        let nnh = NonNegativeHours::new(24.0).unwrap();
        let json = serde_json::to_string(&nnh).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: NonNegativeHours =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(nnh, deserialized);
    }

    #[test]
    fn verdict_debug_works() {
        let v = Verdict::attested();
        let debug_str = format!("{:?}", v);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn verdict_clone_works() {
        let v1 = Verdict::not_attested(vec!["reason"]);
        let v2 = v1.clone();
        // Just verify it clones without panic
        let _ = format!("{:?}", v2);
    }

    #[test]
    fn verdict_serialize_works() {
        let v = Verdict::attested();
        let json = serde_json::to_string(&v).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: Verdict = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(format!("{:?}", v), format!("{:?}", deserialized));
    }

    #[test]
    fn consistency_result_debug_works() {
        use consistency::ConsistencyResult;

        let cr = ConsistencyResult::pass();
        let debug_str = format!("{:?}", cr);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn consistency_result_clone_works() {
        use consistency::ConsistencyResult;

        let cr1 = ConsistencyResult::pass();
        let cr2 = cr1.clone();
        assert_eq!(cr1.passed, cr2.passed);
    }

    #[test]
    fn consistency_result_serialize_works() {
        use consistency::ConsistencyResult;

        let cr = ConsistencyResult::pass();
        let json = serde_json::to_string(&cr).expect("should serialize");
        assert!(!json.is_empty());
        let deserialized: ConsistencyResult =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(cr.passed, deserialized.passed);
    }
}
