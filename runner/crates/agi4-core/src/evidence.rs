//! Evidence types representing upstream benchmark measurements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

/// Stable identifier for an upstream source (e.g., "arc-agi-3", "metr-80pct-time-horizon").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(String);

impl SourceId {
    /// Create a new source identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the source ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier for a measurement within a source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeasurementId(String);

impl MeasurementId {
    /// Create a new measurement identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the measurement ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Bounded fraction in [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundedFraction(f64);

impl BoundedFraction {
    /// Create a bounded fraction, validating that value is in [0.0, 1.0].
    pub fn new(value: f64) -> Result<Self, String> {
        if value.is_nan() {
            return Err("BoundedFraction cannot be NaN".to_string());
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(format!(
                "BoundedFraction must be in [0.0, 1.0], got {}",
                value
            ));
        }
        Ok(Self(value))
    }

    /// Get the underlying f64 value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Non-negative hours with upper bound T_MAX.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NonNegativeHours(f64);

impl NonNegativeHours {
    /// Create a non-negative hour value, validating that it's >= 0.0.
    pub fn new(value: f64) -> Result<Self, String> {
        if value.is_nan() {
            return Err("NonNegativeHours cannot be NaN".to_string());
        }
        if value < 0.0 {
            return Err(format!("NonNegativeHours must be >= 0.0, got {}", value));
        }
        Ok(Self(value))
    }

    /// Get the underlying f64 value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The value of a measurement, bounded to meaningful ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceValue {
    Fraction(BoundedFraction),
    Hours(NonNegativeHours),
}

/// Provenance metadata for a measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub source_url: Url,
    pub fetch_timestamp: DateTime<Utc>,
    pub source_version: Option<String>,
    pub raw_value: String,
}

/// Evidence ingested from one upstream source for one measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub source: SourceId,
    pub measurement: MeasurementId,
    pub value: SourceValue,
    pub reliability_percentile: u8,
    pub provenance: Provenance,
}
