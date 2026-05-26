//! Evidence types representing upstream benchmark measurements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

/// Stable identifier for an upstream source (e.g., "arc-agi-3", "metr-80pct-time-horizon").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(String);

/// Stable identifier for a measurement within a source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeasurementId(String);

/// Bounded fraction in [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundedFraction(f64);

/// Non-negative hours with upper bound T_MAX.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NonNegativeHours(f64);

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
