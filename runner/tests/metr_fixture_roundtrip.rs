//! Integration test: METR adapter round-trip with frozen fixture.
//!
//! Verifies the complete flow: frozen JSON fixture → parse → to_evidence → valid structure

use agi4_adapters::{InMemoryFetcher, MetrAdapter, ModelId, Source};
use url::Url;

#[test]
fn metr_fixture_roundtrip() {
    // Load fixture
    let fixture_json = include_str!("../tests/fixtures/metr/time-horizon-168h.json");

    // Create adapter
    let adapter = MetrAdapter::default();

    // Set up fetcher with fixture
    let mut fetcher = InMemoryFetcher::new();
    fetcher.insert(adapter.endpoint().as_str(), fixture_json);

    // Simulate fetch: in real usage, the fetcher would provide the JSON
    let fetched_data = fetcher
        .fetch(adapter.endpoint())
        .expect("fixture should be available");

    // Parse: raw string → MetrRaw
    let metr_raw = adapter.parse(&fetched_data).expect("should parse fixture");

    // Verify parsed value
    assert_eq!(metr_raw.value, 168.0);

    // Convert: MetrRaw → Vec<Evidence>
    let model = ModelId::new("example-model-v1");
    let evidence_vec = adapter
        .to_evidence(metr_raw, &model)
        .expect("should convert to evidence");

    // Verify evidence structure
    assert_eq!(evidence_vec.len(), 1, "METR produces one evidence entry");

    let evidence = &evidence_vec[0];

    // Verify metadata
    assert_eq!(
        evidence.source.as_str(),
        "metr-80pct-time-horizon",
        "source ID should match"
    );
    assert_eq!(
        evidence.measurement.as_str(),
        "80pct-time-horizon",
        "measurement should be time-horizon"
    );
    assert_eq!(
        evidence.reliability_percentile, 80,
        "reliability percentile is 80 by SPEC definition"
    );

    // Verify value type and bounds
    use agi4_core::evidence::SourceValue;
    match &evidence.value {
        SourceValue::Hours(hours) => {
            assert_eq!(hours.value(), 168.0, "value should be 168 hours");
        }
        _ => panic!("expected Hours variant"),
    }

    // Verify provenance
    assert!(!evidence.provenance.raw_value.is_empty());
    assert!(evidence.provenance.source_version.is_some());
    assert!(evidence.provenance.source_url.as_str().contains("metr.org"));
}

#[test]
fn metr_fixture_multiple_models() {
    // Verify that the same fixture can be used for multiple models
    let fixture_json = include_str!("../tests/fixtures/metr/time-horizon-168h.json");

    let adapter = MetrAdapter::default();

    let models = vec![
        ModelId::new("model-a"),
        ModelId::new("model-b"),
        ModelId::new("model-c"),
    ];

    for model in models {
        let metr_raw = adapter
            .parse(fixture_json)
            .expect("should parse fixture for all models");

        let evidence_vec = adapter
            .to_evidence(metr_raw, &model)
            .expect("should convert for all models");

        assert_eq!(evidence_vec.len(), 1);
        let evidence = &evidence_vec[0];

        // Value should be the same for all models (from the same fixture)
        use agi4_core::evidence::SourceValue;
        match &evidence.value {
            SourceValue::Hours(hours) => assert_eq!(hours.value(), 168.0),
            _ => panic!("expected Hours"),
        }
    }
}
