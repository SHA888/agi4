//! Cross-conjunct consistency check.
//!
//! Implements SPEC.md §4: prevents suspicious measurement patterns where
//! one conjunct is in insufficient_data while others marginally pass.

use serde::{Deserialize, Serialize};

/// Result of the consistency check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyResult {
    pub passed: bool,
    pub failed_rules: Vec<String>,
    pub detail: Option<String>,
}

impl ConsistencyResult {
    /// Create a passing consistency check.
    pub fn pass() -> Self {
        Self {
            passed: true,
            failed_rules: vec![],
            detail: None,
        }
    }

    /// Create a failing consistency check with reason(s).
    pub fn fail(rules: Vec<&str>, detail: String) -> Self {
        Self {
            passed: false,
            failed_rules: rules.iter().map(|s| s.to_string()).collect(),
            detail: Some(detail),
        }
    }
}
