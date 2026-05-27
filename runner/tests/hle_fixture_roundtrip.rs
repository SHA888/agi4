//! Integration test: HLE adapter round-trip with frozen fixture.
//!
//! Verifies the complete flow: frozen JSON fixture → parse → to_evidence → valid structure

use agi4_adapters::{HleAdapter, InMemoryFetcher, ModelId, Source};
use agi4_core::evidence::SourceValue;
use url::Url;

#[test]
fn hle_fixture_roundtrip() {
    // Load fixture
    let fixture_json = include_str!("../tests/fixtures/hle/overall-accuracy-0.82.json");

    // Create adapter
    let adapter = HleAdapter::default();

    // Set up fetcher with fixture
    let mut fetcher = InMemoryFetcher::new();
    fetcher.insert(adapter.endpoint().as_str(), fixture_json);

    // Simulate fetch
    let fetched_data = fetcher
        .fetch(adapter.endpoint())
        .expect("fixture should be available");

    // Parse: raw string → HleRaw
    let hle_raw = adapter.parse(&fetched_data).expect("should parse fixture");

    // Verify parsed value
    assert_eq!(hle_raw.overall_accuracy, 0.82);

    // Convert: HleRaw → Vec<Evidence>
    let model = ModelId::new("example-model-v1");
    let evidence_vec = adapter
        .to_evidence(hle_raw, &model)
        .expect("should convert to evidence");

    // Verify evidence count: should emit 1 entry for HLE
    assert_eq!(evidence_vec.len(), 1, "HLE adapter produces one evidence entry");

    let evidence = &evidence_vec[0];

    // Verify metadata
    assert_eq!(evidence.source.as_str(), "hle", "source ID should be hle");
    assert_eq!(
        evidence.measurement.as_str(),
        "overall-accuracy",
        "measurement should be overall-accuracy"
    );
    assert_eq!(
        evidence.reliability_percentile, 95,
        "reliability percentile is 95 per SPEC §2.1"
    );

    // Verify value type and bounds
    match &evidence.value {
        SourceValue::Fraction(frac) => {
            assert_eq!(frac.value(), 0.82, "value should be 0.82");
        }
        _ => panic!("expected Fraction value"),
    }

    // Verify provenance
    assert!(!evidence.provenance.raw_value.is_empty());
    assert!(evidence.provenance.source_version.is_some());
    assert!(evidence.provenance.source_url.as_str().contains("hle") || evidence.provenance.source_url.as_str().contains("cais"));
}

#[test]
fn hle_fixture_multiple_models() {
    // Verify that the same fixture can be used for multiple models
    let fixture_json = include_str!("../tests/fixtures/hle/overall-accuracy-0.82.json");

    let adapter = HleAdapter::default();

    let models = vec![
        ModelId::new("model-a"),
        ModelId::new("model-b"),
        ModelId::new("model-c"),
    ];

    for model in models {
        let hle_raw = adapter
            .parse(fixture_json)
            .expect("should parse fixture for all models");

        let evidence_vec = adapter
            .to_evidence(hle_raw, &model)
            .expect("should convert for all models");

        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];

        // Value should be the same for all models (from the same fixture)
        match &evidence.value {
            SourceValue::Fraction(frac) => assert_eq!(frac.value(), 0.82),
            _ => panic!("expected Fraction"),
        }
    }
}
