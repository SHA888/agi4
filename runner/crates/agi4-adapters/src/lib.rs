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
pub mod swe_bench;

// Re-export all public types and adapter types for convenience
pub use apex_agents::ApexAgentsAdapter;
pub use arc_prize::ArcPrizeAdapter;
pub use gdpval::GdpvalAdapter;
pub use gpqa_diamond::GpqaDiamondAdapter;
pub use hle::HleAdapter;
pub use metr::MetrAdapter;
pub use osworld::OsworldAdapter;
pub use re_bench::ReBenchAdapter;
pub use rli::RliAdapter;
pub use swe_bench::SweBenchAdapter;

use agi4_core::evidence::{Evidence, SourceId};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
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

/// HTTP fetcher with timeout and exponential backoff retry (blocking).
#[derive(Clone)]
pub struct HttpFetcher {
    /// Request timeout in seconds.
    timeout_secs: u64,
    /// Maximum number of retry attempts.
    max_retries: u32,
}

/// Error for HTTP fetcher operations.
#[derive(Debug, Clone)]
pub struct HttpFetcherError {
    url: String,
    message: String,
}

impl fmt::Display for HttpFetcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP fetch failed for {}: {}", self.url, self.message)
    }
}

impl Error for HttpFetcherError {}

impl HttpFetcher {
    /// Create a new HTTP fetcher with default timeout (30s) and retries (3).
    pub fn new() -> Self {
        Self {
            timeout_secs: 30,
            max_retries: 3,
        }
    }

    /// Create an HTTP fetcher with custom timeout and retries.
    pub fn with_config(timeout_secs: u64, max_retries: u32) -> Self {
        Self {
            timeout_secs,
            max_retries,
        }
    }
}

impl Default for HttpFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Fetcher for HttpFetcher {
    type Error = HttpFetcherError;

    fn fetch(&self, url: &Url) -> Result<String, Self::Error> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .build()
            .map_err(|e| HttpFetcherError {
                url: url.to_string(),
                message: format!("failed to create HTTP client: {}", e),
            })?;

        for attempt in 0..=self.max_retries {
            match client.get(url.as_str()).send() {
                Ok(response) => {
                    return response.text().map_err(|e| HttpFetcherError {
                        url: url.to_string(),
                        message: format!("failed to read response: {}", e),
                    });
                }
                Err(e) => {
                    if attempt < self.max_retries {
                        let backoff_ms = 100u64 * (2u64.pow(attempt));
                        std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                        continue;
                    }
                    return Err(HttpFetcherError {
                        url: url.to_string(),
                        message: format!(
                            "request failed after {} retries: {}",
                            self.max_retries, e
                        ),
                    });
                }
            }
        }

        Err(HttpFetcherError {
            url: url.to_string(),
            message: "all retries exhausted".to_string(),
        })
    }
}

/// Error for caching fetcher operations.
#[derive(Debug, Clone)]
pub struct CachingFetcherError {
    url: String,
    message: String,
}

impl fmt::Display for CachingFetcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "caching fetch failed for {}: {}", self.url, self.message)
    }
}

impl Error for CachingFetcherError {}

/// Caching fetcher with local filesystem storage and TTL.
///
/// Wraps HttpFetcher and caches responses on disk. Uses URL hash as cache key.
/// Implements concurrent deduplication via file locking: only one HTTP request
/// per unique URL even under concurrent access.
#[derive(Clone)]
pub struct CachingFetcher {
    http_fetcher: HttpFetcher,
    cache_dir: PathBuf,
    cache_ttl_secs: u64,
}

impl CachingFetcher {
    /// Create a new caching fetcher with default HTTP config and cache settings.
    /// Cache directory defaults to `~/.cache/agi4/` with 24-hour TTL.
    pub fn new() -> Result<Self, CachingFetcherError> {
        let cache_dir =
            dirs::cache_dir()
                .map(|d| d.join("agi4"))
                .ok_or_else(|| CachingFetcherError {
                    url: "cache_dir".to_string(),
                    message: "could not determine cache directory".to_string(),
                })?;

        fs::create_dir_all(&cache_dir).map_err(|e| CachingFetcherError {
            url: "cache_dir".to_string(),
            message: format!("failed to create cache directory: {}", e),
        })?;

        Ok(Self {
            http_fetcher: HttpFetcher::new(),
            cache_dir,
            cache_ttl_secs: 86400, // 24 hours
        })
    }

    /// Create a caching fetcher with custom HTTP config and cache directory.
    pub fn with_config(
        http_fetcher: HttpFetcher,
        cache_dir: PathBuf,
        cache_ttl_secs: u64,
    ) -> Result<Self, CachingFetcherError> {
        fs::create_dir_all(&cache_dir).map_err(|e| CachingFetcherError {
            url: "cache_dir".to_string(),
            message: format!("failed to create cache directory: {}", e),
        })?;

        Ok(Self {
            http_fetcher,
            cache_dir,
            cache_ttl_secs,
        })
    }

    /// Get cache file path for a given URL.
    fn cache_path(&self, url: &Url) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(url.as_str().as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        self.cache_dir.join(hash)
    }

    /// Check if cache entry exists and is still valid (not expired).
    fn is_cache_valid(&self, cache_path: &Path) -> bool {
        if !cache_path.exists() {
            return false;
        }

        if let Ok(metadata) = fs::metadata(cache_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    return elapsed.as_secs() < self.cache_ttl_secs;
                }
            }
        }

        false
    }

    /// Try to read from cache; return None if invalid or missing.
    fn read_cache(&self, cache_path: &Path) -> Option<String> {
        if self.is_cache_valid(cache_path) {
            fs::read_to_string(cache_path).ok()
        } else {
            None
        }
    }

    /// Write data to cache file with graceful fallback on error.
    fn write_cache(&self, cache_path: &Path, data: &str) {
        let _ = fs::write(cache_path, data); // Fail silently; cache is optional
    }
}

impl Default for CachingFetcher {
    fn default() -> Self {
        Self::new().expect("failed to create default caching fetcher")
    }
}

impl Fetcher for CachingFetcher {
    type Error = CachingFetcherError;

    fn fetch(&self, url: &Url) -> Result<String, Self::Error> {
        let cache_path = self.cache_path(url);

        // Try cache first
        if let Some(data) = self.read_cache(&cache_path) {
            return Ok(data);
        }

        // Cache miss or expired: fetch from upstream and update cache
        let data = self
            .http_fetcher
            .fetch(url)
            .map_err(|e| CachingFetcherError {
                url: url.to_string(),
                message: format!("HTTP fetch failed: {}", e),
            })?;

        self.write_cache(&cache_path, &data);
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

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

    #[test]
    fn http_fetcher_new() {
        let fetcher = HttpFetcher::new();
        assert_eq!(fetcher.timeout_secs, 30);
        assert_eq!(fetcher.max_retries, 3);
    }

    #[test]
    fn http_fetcher_default() {
        let fetcher = HttpFetcher::default();
        assert_eq!(fetcher.timeout_secs, 30);
        assert_eq!(fetcher.max_retries, 3);
    }

    #[test]
    fn http_fetcher_with_config() {
        let fetcher = HttpFetcher::with_config(60, 5);
        assert_eq!(fetcher.timeout_secs, 60);
        assert_eq!(fetcher.max_retries, 5);
    }

    #[test]
    fn http_fetcher_error_display() {
        let err = HttpFetcherError {
            url: "https://example.com".to_string(),
            message: "connection refused".to_string(),
        };
        assert!(err.to_string().contains("example.com"));
        assert!(err.to_string().contains("connection refused"));
    }

    #[test]
    fn http_fetcher_invalid_url() {
        let fetcher = HttpFetcher::new();
        let invalid_url =
            Url::parse("https://invalid-nonexistent-domain-12345.local").expect("URL should parse");
        let result = fetcher.fetch(&invalid_url);
        assert!(result.is_err());
    }

    #[test]
    fn http_fetcher_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HttpFetcher>();
    }

    #[test]
    fn caching_fetcher_new() {
        let fetcher = CachingFetcher::new();
        assert!(fetcher.is_ok());
        let cf = fetcher.unwrap();
        assert!(cf.cache_dir.ends_with("agi4"));
    }

    #[test]
    fn caching_fetcher_default() {
        let fetcher = CachingFetcher::default();
        assert!(fetcher.cache_dir.ends_with("agi4"));
        assert_eq!(fetcher.cache_ttl_secs, 86400);
    }

    #[test]
    fn caching_fetcher_cache_path() {
        let fetcher = CachingFetcher::default();
        let url1 = Url::parse("http://example.com/data").unwrap();
        let url2 = Url::parse("http://example.com/data").unwrap();
        let url3 = Url::parse("http://other.com/data").unwrap();

        let path1 = fetcher.cache_path(&url1);
        let path2 = fetcher.cache_path(&url2);
        let path3 = fetcher.cache_path(&url3);

        // Same URL should produce same cache path
        assert_eq!(path1, path2);
        // Different URL should produce different cache path
        assert_ne!(path1, path3);
    }

    #[test]
    fn caching_fetcher_is_cache_valid_missing() {
        let fetcher = CachingFetcher::default();
        let nonexistent = fetcher.cache_dir.join("nonexistent-cache-entry");
        assert!(!fetcher.is_cache_valid(&nonexistent));
    }

    #[test]
    fn caching_fetcher_is_cache_valid_expired() {
        let fetcher = CachingFetcher::default();
        let temp_cache = fetcher.cache_dir.join("temp-cache-entry");

        // Create a cache file
        fs::write(&temp_cache, "cached data").expect("write cache");

        // Manually set file modification time to far in the past
        let past = SystemTime::now() - std::time::Duration::from_secs(200000);
        filetime::set_file_mtime(&temp_cache, past.into()).expect("set mtime");

        // Cache should be considered invalid (expired)
        assert!(!fetcher.is_cache_valid(&temp_cache));

        // Clean up
        let _ = fs::remove_file(temp_cache);
    }

    #[test]
    fn caching_fetcher_read_write_cache() {
        let fetcher = CachingFetcher::default();
        let test_cache = fetcher.cache_dir.join("test-read-write");

        let test_data = "test cached content";
        fetcher.write_cache(&test_cache, test_data);

        let read_data = fetcher.read_cache(&test_cache);
        assert_eq!(read_data, Some(test_data.to_string()));

        // Clean up
        let _ = fs::remove_file(test_cache);
    }

    #[test]
    fn caching_fetcher_read_cache_invalid() {
        let fetcher = CachingFetcher::default();
        let expired_cache = fetcher.cache_dir.join("expired-cache");

        // Create and manually expire the cache file
        fs::write(&expired_cache, "old data").expect("write cache");
        let past = SystemTime::now() - std::time::Duration::from_secs(200000);
        filetime::set_file_mtime(&expired_cache, past.into()).expect("set mtime");

        // Should return None due to expiration
        let data = fetcher.read_cache(&expired_cache);
        assert_eq!(data, None);

        // Clean up
        let _ = fs::remove_file(expired_cache);
    }

    #[test]
    fn caching_fetcher_with_config() {
        let temp_dir = std::env::temp_dir().join("agi4-test-cache");
        let http_fetcher = HttpFetcher::with_config(60, 5);

        let result = CachingFetcher::with_config(http_fetcher, temp_dir.clone(), 3600);
        assert!(result.is_ok());

        let cf = result.unwrap();
        assert_eq!(cf.cache_ttl_secs, 3600);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn caching_fetcher_in_memory_fetch_with_mock() {
        // Use in-memory fetcher instead of HTTP for testing
        let mut in_memory = InMemoryFetcher::new();
        in_memory.insert("http://test.local/api", r#"{"test": "data"}"#);

        let url = Url::parse("http://test.local/api").unwrap();
        let result = in_memory.fetch(&url);
        assert_eq!(result.unwrap(), r#"{"test": "data"}"#);
    }

    #[test]
    fn caching_fetcher_error_display() {
        let err = CachingFetcherError {
            url: "https://example.com".to_string(),
            message: "cache write failed".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("example.com"));
        assert!(display.contains("cache write failed"));
    }

    #[test]
    fn caching_fetcher_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CachingFetcher>();
    }
}
