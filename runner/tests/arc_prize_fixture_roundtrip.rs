//! Integration test: ARC Prize adapter round-trip with frozen fixture.
//!
//! Verifies the complete flow: frozen JSON fixture → parse → to_evidence → valid structure
//! Tests both ARC-AGI-2 and ARC-AGI-3 measurements.

use agi4_adapters::{ArcPrizeAdapter, InMemoryFetcher, ModelId, Source};
use agi4_core::evidence::SourceValue;
use url::Url;

#[test]
fn arc_prize_fixture_roundtrip() {
    // Load fixture
    let fixture_json = include_str!("../tests/fixtures/arc-prize/leaderboard-example.json");

    // Create adapter
    let adapter = ArcPrizeAdapter::default();

    // Set up fetcher with fixture
    let mut fetcher = InMemoryFetcher::new();
    fetcher.insert(adapter.endpoint().as_str(), fixture_json);

    // Simulate fetch
    let fetched_data = fetcher
        .fetch(adapter.endpoint())
        .expect("fixture should be available");

    // Parse: raw string → ArcPrizeRaw
    let arc_raw = adapter.parse(&fetched_data).expect("should parse fixture");

    // Verify parsed values
    assert_eq!(arc_raw.arc_agi_2.pass_rate, 0.87);
    assert_eq!(arc_raw.arc_agi_3.pass_rate, 0.52);

    // Convert: ArcPrizeRaw → Vec<Evidence>
    let model = ModelId::new("example-model-v1");
    let evidence_vec = adapter
        .to_evidence(arc_raw, &model)
        .expect("should convert to evidence");

    // Verify evidence count: should emit 2 entries (ARC-AGI-2 and ARC-AGI-3)
    assert_eq!(evidence_vec.len(), 2, "ARC Prize adapter produces two evidence entries");

    // Find and verify ARC-AGI-2 evidence
    let arc_agi_2_ev = evidence_vec
        .iter()
        .find(|e| e.source.as_str() == "arc-agi-2")
        .expect("should have arc-agi-2 evidence");

    assert_eq!(
        arc_agi_2_ev.measurement.as_str(),
        "pass@1-private-split",
        "ARC-AGI-2 measurement ID should be pass@1-private-split"
    );
    assert_eq!(
        arc_agi_2_ev.reliability_percentile, 95,
        "ARC-AGI-2 reliability percentile is 95 per SPEC §2.1"
    );

    match &arc_agi_2_ev.value {
        SourceValue::Fraction(frac) => {
            assert_eq!(frac.value(), 0.87, "ARC-AGI-2 value should be 0.87");
        }
        _ => panic!("expected Fraction variant for ARC-AGI-2"),
    }

    // Find and verify ARC-AGI-3 evidence
    let arc_agi_3_ev = evidence_vec
        .iter()
        .find(|e| e.source.as_str() == "arc-agi-3")
        .expect("should have arc-agi-3 evidence");

    assert_eq!(
        arc_agi_3_ev.measurement.as_str(),
        "pass@1-interactive-private",
        "ARC-AGI-3 measurement ID should be pass@1-interactive-private"
    );
    assert_eq!(
        arc_agi_3_ev.reliability_percentile, 80,
        "ARC-AGI-3 reliability percentile is 80 per SPEC §2.1"
    );

    match &arc_agi_3_ev.value {
        SourceValue::Fraction(frac) => {
            assert_eq!(frac.value(), 0.52, "ARC-AGI-3 value should be 0.52");
        }
        _ => panic!("expected Fraction variant for ARC-AGI-3"),
    }

    // Verify provenance on both
    for evidence in &evidence_vec {
        assert!(!evidence.provenance.raw_value.is_empty());
        assert!(evidence.provenance.source_version.is_some());
        assert!(evidence.provenance.source_url.as_str().contains("arc"));
    }
}

#[test]
fn arc_prize_fixture_multiple_models() {
    // Verify that the same fixture can be used for multiple models
    let fixture_json = include_str!("../tests/fixtures/arc-prize/leaderboard-example.json");

    let adapter = ArcPrizeAdapter::default();

    let models = vec![
        ModelId::new("model-a"),
        ModelId::new("model-b"),
        ModelId::new("model-c"),
    ];

    for model in models {
        let arc_raw = adapter
            .parse(fixture_json)
            .expect("should parse fixture for all models");

        let evidence_vec = adapter
            .to_evidence(arc_raw, &model)
            .expect("should convert for all models");

        assert_eq!(evidence_vec.len(), 2);

        // Both evidence entries should have consistent values regardless of model
        let arc_agi_2_ev = evidence_vec
            .iter()
            .find(|e| e.source.as_str() == "arc-agi-2")
            .unwrap();

        match &arc_agi_2_ev.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.87),
            _ => panic!("expected Fraction"),
        }
    }
}

#[test]
fn arc_prize_fixture_invalid_json() {
    let adapter = ArcPrizeAdapter::default();
    let invalid_json = "not valid json";
    let result = adapter.parse(invalid_json);
    assert!(result.is_err());
}

#[test]
fn arc_prize_fixture_out_of_bounds() {
    let adapter = ArcPrizeAdapter::default();
    let out_of_bounds_json = r#"{"arc_agi_2": {"pass_rate": 1.5}, "arc_agi_3": {"pass_rate": 0.5}}"#;
    let result = adapter.parse(out_of_bounds_json);

    // Parse should succeed (it's valid JSON)
    let arc_raw = result.expect("should parse");

    // But to_evidence should fail due to bounds validation
    let model = ModelId::new("test-model");
    let evidence_result = adapter.to_evidence(arc_raw, &model);
    assert!(evidence_result.is_err());
}
