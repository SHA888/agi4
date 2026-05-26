//! Verdict function and verdict enumeration.
//!
//! The verdict is the load-bearing output of the runner.
//! This function is pure and total: same inputs always produce same outputs,
//! with no panics on any valid input.

use serde::{Deserialize, Serialize};

/// The top-level verdict from the runner.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "verdict")]
pub enum Verdict {
    #[serde(rename = "attested")]
    Attested,
    #[serde(rename = "not_attested")]
    NotAttested {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        reasons: Vec<String>,
    },
    #[serde(rename = "insufficient_data")]
    InsufficientData {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        missing: Vec<String>,
    },
}

impl Verdict {
    /// Create an attested verdict.
    pub fn attested() -> Self {
        Verdict::Attested
    }

    /// Create a not_attested verdict with reason(s).
    pub fn not_attested(reasons: Vec<&str>) -> Self {
        Verdict::NotAttested {
            reasons: reasons.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Create an insufficient_data verdict.
    pub fn insufficient_data(missing: Vec<&str>) -> Self {
        Verdict::InsufficientData {
            missing: missing.iter().map(|s| s.to_string()).collect(),
        }
    }
}
