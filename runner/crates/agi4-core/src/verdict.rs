//! Verdict function and verdict enumeration.
//!
//! The verdict is the load-bearing output of the runner.
//! This function is pure and total: same inputs always produce same outputs,
//! with no panics on any valid input.

use crate::conjunct::ConjunctStatus;
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

/// Compute the verdict from four conjunct statuses and consistency check result.
///
/// Takes conjunct statuses in order: Generality, EconomicSubstitutability,
/// EnvironmentalTransfer, AutonomousAgency.
///
/// Returns a verdict following SPEC.md §5 rules:
/// - All four pass + consistency pass → Attested
/// - All four pass + consistency fail → NotAttested
/// - Any fail → NotAttested (fail dominates)
/// - Any partial or fail (except isolated insufficient_data) → NotAttested
/// - Any insufficient_data with no fail → InsufficientData
pub fn verdict(conjunct_statuses: &[ConjunctStatus; 4], consistency_passed: bool) -> Verdict {
    let conjuncts = [
        ("generality", conjunct_statuses[0]),
        ("economic_substitutability", conjunct_statuses[1]),
        ("environmental_transfer", conjunct_statuses[2]),
        ("autonomous_agency", conjunct_statuses[3]),
    ];

    // Rule: Any fail → not_attested (fail dominates all other statuses)
    let fail_count = conjuncts
        .iter()
        .filter(|(_, s)| *s == ConjunctStatus::Fail)
        .count();
    if fail_count > 0 {
        let failed: Vec<&str> = conjuncts
            .iter()
            .filter(|(_, s)| *s == ConjunctStatus::Fail)
            .map(|(name, _)| *name)
            .collect();
        return Verdict::not_attested(failed);
    }

    // Rule: Any partial → not_attested
    let partial_count = conjuncts
        .iter()
        .filter(|(_, s)| *s == ConjunctStatus::Partial)
        .count();
    if partial_count > 0 {
        let partialed: Vec<&str> = conjuncts
            .iter()
            .filter(|(_, s)| *s == ConjunctStatus::Partial)
            .map(|(name, _)| *name)
            .collect();
        return Verdict::not_attested(partialed);
    }

    // At this point: all statuses are either Pass or InsufficientData
    let pass_count = conjuncts
        .iter()
        .filter(|(_, s)| *s == ConjunctStatus::Pass)
        .count();
    let insufficient_count = conjuncts
        .iter()
        .filter(|(_, s)| *s == ConjunctStatus::InsufficientData)
        .count();

    // Rule: If any insufficient_data exists (and no fail or partial), verdict is insufficient_data
    if insufficient_count > 0 {
        let insufficient: Vec<&str> = conjuncts
            .iter()
            .filter(|(_, s)| *s == ConjunctStatus::InsufficientData)
            .map(|(name, _)| *name)
            .collect();
        return Verdict::insufficient_data(insufficient);
    }

    // All four must be Pass if we reach here
    if pass_count == 4 {
        if consistency_passed {
            return Verdict::attested();
        } else {
            return Verdict::not_attested(vec!["consistency_check"]);
        }
    }

    // This branch should never be reached given the above logic
    // (all statuses are either Pass or InsufficientData, and we already handled insufficient_data above)
    Verdict::not_attested(vec!["unknown_state"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_all_pass_consistency_pass() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let v = verdict(&statuses, true);
        assert!(matches!(v, Verdict::Attested));
    }

    #[test]
    fn verdict_all_pass_consistency_fail() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let v = verdict(&statuses, false);
        assert!(matches!(v, Verdict::NotAttested { .. }));
    }

    #[test]
    fn verdict_any_fail() {
        let statuses = [
            ConjunctStatus::Fail,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let v = verdict(&statuses, true);
        assert!(matches!(v, Verdict::NotAttested { .. }));
    }

    #[test]
    fn verdict_any_partial() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let v = verdict(&statuses, true);
        assert!(matches!(v, Verdict::NotAttested { .. }));
    }

    #[test]
    fn verdict_any_insufficient_data_no_fail() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let v = verdict(&statuses, true);
        assert!(matches!(v, Verdict::InsufficientData { .. }));
    }

    #[test]
    fn verdict_fail_dominates_insufficient_data() {
        let statuses = [
            ConjunctStatus::Fail,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let v = verdict(&statuses, true);
        // Fail dominates insufficient_data
        assert!(matches!(v, Verdict::NotAttested { .. }));
    }

    #[test]
    fn verdict_partial_dominates_insufficient_data() {
        let statuses = [
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let v = verdict(&statuses, true);
        // Partial dominates insufficient_data
        assert!(matches!(v, Verdict::NotAttested { .. }));
    }

    #[test]
    fn verdict_exhaustive_512_cases() {
        let statuses_list = vec![
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Fail,
            ConjunctStatus::InsufficientData,
        ];

        // Test all 4^4 = 256 combinations for each consistency check state (2 states)
        // Total: 512 combinations
        let mut case_count = 0;
        for g in &statuses_list {
            for e in &statuses_list {
                for env in &statuses_list {
                    for a in &statuses_list {
                        for consistency in &[true, false] {
                            let statuses = [*g, *e, *env, *a];
                            let _v = verdict(&statuses, *consistency);
                            // If we reach here without panic, the verdict function is total
                            case_count += 1;
                        }
                    }
                }
            }
        }

        assert_eq!(case_count, 512);
    }

    #[test]
    fn verdict_reasons_match_failed_conjuncts() {
        let statuses = [
            ConjunctStatus::Fail,
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let v = verdict(&statuses, true);
        match v {
            Verdict::NotAttested { reasons } => {
                // Fail has higher priority, so only generality should be in reasons
                assert!(!reasons.is_empty());
                assert!(reasons.contains(&"generality".to_string()));
            }
            _ => panic!("Expected NotAttested"),
        }
    }

    #[test]
    fn verdict_insufficient_data_reasons() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let v = verdict(&statuses, true);
        match v {
            Verdict::InsufficientData { missing } => {
                assert_eq!(missing.len(), 2);
                assert!(missing.contains(&"economic_substitutability".to_string()));
                assert!(missing.contains(&"autonomous_agency".to_string()));
            }
            _ => panic!("Expected InsufficientData"),
        }
    }
}
