//! Upstream source adapters for AGI/4 evidence ingestion.
//!
//! Each upstream benchmark source implements the Source trait for pure parsing
//! and evidence conversion. The Fetcher trait handles I/O (HTTP, file, in-memory).
//! Adapters are testable in isolation against frozen JSON fixtures.

pub mod apex_agents;
pub mod arc_prize;
pub mod gdpval;
pub mod gpqa_diamond;
pub mod hle;
pub mod metr;
pub mod osworld;
pub mod re_bench;
pub mod rli;

use agi4_core::evidence::{Evidence, SourceId};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use url::Url;

/// Model identifier for evidence ingestion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelId(pub String);

impl ModelId {
    /// Create a new model identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the model ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Error type for source adaptation operations.
#[derive(Debug, Clone)]
pub struct AdapterError {
    source_id: String,
    message: String,
}

impl AdapterError {
    /// Create a new adapter error.
    pub fn new(source_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            source_id: source_id.into(),
            message: message.into(),
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "adapter error [{}]: {}", self.source_id, self.message)
    }
}

impl Error for AdapterError {}

/// The Source trait: each upstream source implements this.
///
/// Sources are pure: they parse and convert without performing I/O.
/// This allows adapters to be tested against frozen fixtures without network access.
pub trait Source {
    /// The native schema for this source's upstream data.
    type Raw: DeserializeOwned;

    /// Error type for parse/to_evidence failures.
    type Error: Error + Send + Sync + 'static;

    /// Stable identifier for this source (e.g., "arc-agi-3").
    fn id(&self) -> SourceId;

    /// The URL or endpoint this source ingests from.
    fn endpoint(&self) -> &Url;

    /// Parse raw upstream data (JSON string) into the typed schema.
    /// Fails closed on any malformed input.
    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error>;

    /// Convert validated raw data into agi4-core Evidence values.
    /// One source may produce evidence for multiple conjuncts
    /// (e.g., ARC-AGI-3 contributes to both Generality and EnvironmentalTransfer).
    fn to_evidence(&self, raw: Self::Raw, model: &ModelId) -> Result<Vec<Evidence>, Self::Error>;
}

/// Fetcher abstraction for I/O.
///
/// Implementations handle HTTP, file, in-memory, or other fetch strategies.
/// Injected at the CLI layer; adapters use pure Source trait, never Fetcher directly.
pub trait Fetcher {
    /// Error type for fetch failures.
    type Error: Error + Send + Sync + 'static;

    /// Fetch raw data from a URL. Returns the response body as a string.
    fn fetch(&self, url: &Url) -> Result<String, Self::Error>;
}

/// In-memory test fetcher for fixture-based testing.
///
/// Stores frozen upstream data snapshots. Useful for unit testing adapters
/// without network access. Does not perform any I/O; all data is pre-loaded.
#[derive(Debug, Clone)]
pub struct InMemoryFetcher {
    data: HashMap<String, String>,
}

impl InMemoryFetcher {
    /// Create a new empty in-memory fetcher.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Insert fixture data for a URL.
    pub fn insert(&mut self, url: impl Into<String>, data: impl Into<String>) {
        self.data.insert(url.into(), data.into());
    }

    /// Insert multiple fixture entries at once.
    pub fn with_data(mut self, entries: Vec<(String, String)>) -> Self {
        for (url, data) in entries {
            self.data.insert(url, data);
        }
        self
    }
}

impl Default for InMemoryFetcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Error for in-memory fetcher (URL not found in fixtures).
#[derive(Debug, Clone)]
pub struct InMemoryFetcherError {
    url: String,
}

impl fmt::Display for InMemoryFetcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fixture not found for URL: {}", self.url)
    }
}

impl Error for InMemoryFetcherError {}

impl Fetcher for InMemoryFetcher {
    type Error = InMemoryFetcherError;

    fn fetch(&self, url: &Url) -> Result<String, Self::Error> {
        self.data
            .get(url.as_str())
            .cloned()
            .ok_or_else(|| InMemoryFetcherError {
                url: url.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_id_new_and_as_str() {
        let model = ModelId::new("example-model-v1");
        assert_eq!(model.as_str(), "example-model-v1");
    }

    #[test]
    fn model_id_from_string() {
        let s = "test-model".to_string();
        let model = ModelId::new(s);
        assert_eq!(model.as_str(), "test-model");
    }

    #[test]
    fn model_id_equality() {
        let m1 = ModelId::new("model-a");
        let m2 = ModelId::new("model-a");
        let m3 = ModelId::new("model-b");
        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn adapter_error_new_and_message() {
        let err = AdapterError::new("arc-agi-3", "failed to parse JSON");
        assert_eq!(err.message(), "failed to parse JSON");
        assert!(err.to_string().contains("arc-agi-3"));
    }

    #[test]
    fn adapter_error_display() {
        let err = AdapterError::new("metr", "network timeout");
        let display_str = err.to_string();
        assert!(display_str.contains("metr"));
        assert!(display_str.contains("network timeout"));
    }

    #[test]
    fn in_memory_fetcher_new() {
        let fetcher = InMemoryFetcher::new();
        assert!(matches!(
            fetcher.fetch(&Url::parse("http://example.com").unwrap()),
            Err(InMemoryFetcherError { .. })
        ));
    }

    #[test]
    fn in_memory_fetcher_default() {
        let fetcher = InMemoryFetcher::default();
        assert!(matches!(
            fetcher.fetch(&Url::parse("http://example.com").unwrap()),
            Err(InMemoryFetcherError { .. })
        ));
    }

    #[test]
    fn in_memory_fetcher_insert_and_fetch() {
        let mut fetcher = InMemoryFetcher::new();
        let url = "http://example.com/data.json";
        let data = r#"{"value": 42}"#;
        fetcher.insert(url, data);

        let result = fetcher
            .fetch(&Url::parse(url).unwrap())
            .expect("should fetch inserted data");
        assert_eq!(result, data);
    }

    #[test]
    fn in_memory_fetcher_with_data() {
        let entries = vec![
            ("http://arc.org/data".to_string(), "arc data".to_string()),
            ("http://metr.org/data".to_string(), "metr data".to_string()),
        ];
        let fetcher = InMemoryFetcher::new().with_data(entries);

        let url1 = Url::parse("http://arc.org/data").unwrap();
        let result1 = fetcher.fetch(&url1).expect("should fetch arc data");
        assert_eq!(result1, "arc data");

        let url2 = Url::parse("http://metr.org/data").unwrap();
        let result2 = fetcher.fetch(&url2).expect("should fetch metr data");
        assert_eq!(result2, "metr data");
    }

    #[test]
    fn in_memory_fetcher_missing_url_error() {
        let fetcher = InMemoryFetcher::new();
        let url = Url::parse("http://nonexistent.com/data").unwrap();
        let err = fetcher
            .fetch(&url)
            .expect_err("should error for missing URL");
        assert!(err.to_string().contains("nonexistent.com"));
    }

    #[test]
    fn in_memory_fetcher_clone() {
        let mut fetcher1 = InMemoryFetcher::new();
        fetcher1.insert("http://test.com/data", "test data");

        let fetcher2 = fetcher1.clone();
        let url = Url::parse("http://test.com/data").unwrap();
        let result = fetcher2.fetch(&url).expect("clone should have data");
        assert_eq!(result, "test data");
    }

    #[test]
    fn in_memory_fetcher_multiple_urls() {
        let mut fetcher = InMemoryFetcher::new();
        fetcher.insert("http://source-a.com/api", "data-a");
        fetcher.insert("http://source-b.com/api", "data-b");
        fetcher.insert("http://source-c.com/api", "data-c");

        let url_a = Url::parse("http://source-a.com/api").unwrap();
        assert_eq!(fetcher.fetch(&url_a).unwrap(), "data-a");

        let url_b = Url::parse("http://source-b.com/api").unwrap();
        assert_eq!(fetcher.fetch(&url_b).unwrap(), "data-b");

        let url_c = Url::parse("http://source-c.com/api").unwrap();
        assert_eq!(fetcher.fetch(&url_c).unwrap(), "data-c");
    }

    #[test]
    fn in_memory_fetcher_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<InMemoryFetcherError>();
    }

    #[test]
    fn in_memory_fetcher_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<InMemoryFetcher>();
    }
}
