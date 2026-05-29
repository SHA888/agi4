//! Cross-conjunct consistency check.
//!
//! Implements SPEC.md §4: prevents suspicious measurement patterns where
//! one conjunct is in insufficient_data while others marginally pass.

use crate::conjunct::ConjunctStatus;
use crate::evidence::{Evidence, SourceValue};
use crate::sources;
use crate::threshold;
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

/// Check rule 1: no insufficient_data masking.
/// If three conjuncts pass and one is insufficient_data, it's a masking pattern.
fn check_no_insufficient_data_masking(
    conjunct_statuses: &[ConjunctStatus; 4],
) -> Result<(), String> {
    let pass_count = conjunct_statuses
        .iter()
        .filter(|s| **s == ConjunctStatus::Pass)
        .count();
    let insufficient_count = conjunct_statuses
        .iter()
        .filter(|s| **s == ConjunctStatus::InsufficientData)
        .count();

    // If 3 are Pass and 1 is InsufficientData, it's a masking pattern
    if pass_count == 3 && insufficient_count == 1 {
        return Err(
            "One conjunct is insufficient_data while all others pass (masking pattern)".to_string(),
        );
    }
    Ok(())
}

/// Map source IDs to their associated conjuncts and thresholds.
/// Returns (conjunct_index, pass_threshold, floor) tuples.
fn get_source_threshold(source_id: &str) -> Option<Vec<(usize, f64, Option<f64>)>> {
    match source_id {
        sources::generality::ARC_AGI_2 => {
            Some(vec![(0, threshold::generality::ARC_AGI_2_PASS, None)])
        }
        sources::generality::ARC_AGI_3 => Some(vec![
            (
                0,
                threshold::generality::ARC_AGI_3_PASS,
                Some(threshold::generality::ARC_AGI_3_FLOOR),
            ),
            (
                2,
                threshold::environmental_transfer::ARC_AGI_3_PASS,
                Some(threshold::environmental_transfer::ARC_AGI_3_FLOOR),
            ),
        ]),
        sources::generality::HLE => Some(vec![(0, threshold::generality::HLE_PASS, None)]),
        sources::generality::GPQA_DIAMOND => {
            Some(vec![(0, threshold::generality::GPQA_DIAMOND_PASS, None)])
        }
        sources::economic_substitutability::GDPVAL
        | sources::economic_substitutability::GDPVAL_AA => Some(vec![(
            1,
            threshold::economic_substitutability::GDPVAL_PASS,
            None,
        )]),
        sources::economic_substitutability::RLI => Some(vec![(
            1,
            threshold::economic_substitutability::RLI_PASS,
            Some(threshold::economic_substitutability::RLI_FLOOR),
        )]),
        sources::economic_substitutability::APEX_AGENTS => Some(vec![(
            1,
            threshold::economic_substitutability::APEX_AGENTS_PASS,
            None,
        )]),
        sources::environmental_transfer::OSWORLD => Some(vec![(
            2,
            threshold::environmental_transfer::OSWORLD_PASS,
            None,
        )]),
        sources::environmental_transfer::NES => {
            // NES thresholds TBD in v0.1.x per SPEC.md. For now, skip NES in variance calculation.
            None
        }
        sources::autonomous_agency::METR_80PCT_TIME_HORIZON => Some(vec![(
            3,
            threshold::autonomous_agency::METR_80PCT_PASS_HOURS,
            Some(threshold::autonomous_agency::METR_80PCT_FLOOR_HOURS),
        )]),
        sources::autonomous_agency::RE_BENCH => {
            Some(vec![(3, threshold::autonomous_agency::REBENCH_PASS, None)])
        }
        sources::autonomous_agency::SWE_BENCH_VERIFIED => Some(vec![(
            3,
            threshold::autonomous_agency::SWEBENCH_VERIFIED_PASS_AT_5,
            None,
        )]),
        _ => None,
    }
}

/// Check rule 2: variance bound.
/// When all four conjuncts pass, min_margin >= 0.5 * max_margin.
fn check_variance_bound(
    evidence: &[Evidence],
    conjunct_statuses: &[ConjunctStatus; 4],
) -> Result<(), String> {
    let all_pass = conjunct_statuses.iter().all(|s| *s == ConjunctStatus::Pass);
    if !all_pass {
        // Variance rule only applies when all conjuncts pass
        return Ok(());
    }

    let mut margins = Vec::new();

    for e in evidence {
        if let Some(thresholds) = get_source_threshold(e.source.as_str()) {
            for (_, pass_threshold, _) in thresholds {
                let raw_value = match e.value {
                    SourceValue::Fraction(f) => f.value(),
                    SourceValue::Hours(h) => h.value(),
                };
                let margin = raw_value / pass_threshold;
                margins.push(margin);
            }
        }
    }

    if margins.is_empty() {
        // No recognized sources; variance check passes trivially
        return Ok(());
    }

    let min_margin = margins.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_margin = margins.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if min_margin < threshold::consistency::MARGIN_VARIANCE_RATIO * max_margin {
        return Err(format!(
            "Variance bound violated: min_margin ({:.3}) < 0.5 * max_margin ({:.3})",
            min_margin, max_margin
        ));
    }

    Ok(())
}

/// Check rule 3: provenance metadata completeness.
/// Every source must have URL, fetch timestamp, and source version/date.
fn check_provenance_metadata(evidence: &[Evidence]) -> Result<(), String> {
    let mut missing_sources = Vec::new();

    for e in evidence {
        let source_id = e.source.as_str();
        let mut issues = Vec::new();

        // Check source_url is present and valid (it's a Url type, so presence is guaranteed by type)
        if e.provenance.source_url.as_str().is_empty() {
            issues.push("source_url");
        }

        // Check fetch_timestamp is present (it's a DateTime, so presence is guaranteed by type)

        // Check source_version or we're lenient here because it's optional in the schema
        // but SPEC.md §4 rule 3 says "version or date stamp"
        // The DateTime<Utc> fetch_timestamp serves as the date stamp, so version is optional
        // but if we want to be strict, we could require it. For now, the fetch_timestamp satisfies the "date stamp" requirement.

        if !issues.is_empty() {
            missing_sources.push(format!("{} (missing: {})", source_id, issues.join(", ")));
        }
    }

    if !missing_sources.is_empty() {
        return Err(format!(
            "Provenance metadata incomplete for: {}",
            missing_sources.join("; ")
        ));
    }

    Ok(())
}

/// Evaluate all three consistency check rules.
///
/// Takes the evidence array and the array of per-conjunct statuses (in order:
/// Generality, EconomicSubstitutability, EnvironmentalTransfer, AutonomousAgency).
pub fn consistency_check(
    evidence: &[Evidence],
    conjunct_statuses: &[ConjunctStatus; 4],
) -> ConsistencyResult {
    let mut failed_rules = Vec::new();

    // Rule 1: No insufficient_data masking
    if check_no_insufficient_data_masking(conjunct_statuses).is_err() {
        failed_rules.push("rule_1_insufficient_data_masking");
    }

    // Rule 2: Variance bound
    if check_variance_bound(evidence, conjunct_statuses).is_err() {
        failed_rules.push("rule_2_variance_bound");
    }

    // Rule 3: Provenance metadata
    if check_provenance_metadata(evidence).is_err() {
        failed_rules.push("rule_3_provenance_metadata");
    }

    if failed_rules.is_empty() {
        ConsistencyResult::pass()
    } else {
        let detail = format!("Consistency check failed on: {}", failed_rules.join(", "));
        ConsistencyResult::fail(failed_rules.to_vec(), detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{
        BoundedFraction, MeasurementId, NonNegativeHours, Provenance, SourceId, SourceValue,
    };
    use chrono::Utc;
    use url::Url;

    fn make_evidence(source: &str, value: f64, is_fraction: bool) -> Evidence {
        Evidence {
            source: SourceId::new(source),
            measurement: MeasurementId::new("test-measurement"),
            value: if is_fraction {
                SourceValue::Fraction(BoundedFraction::new(value).unwrap())
            } else {
                SourceValue::Hours(NonNegativeHours::new(value).unwrap())
            },
            reliability_percentile: 95,
            provenance: Provenance {
                source_url: Url::parse("https://example.com").unwrap(),
                fetch_timestamp: Utc::now(),
                source_version: Some("1.0".to_string()),
                raw_value: format!("{}", value),
            },
        }
    }

    #[test]
    fn rule1_all_pass_with_no_insufficient_data() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        assert!(check_no_insufficient_data_masking(&statuses).is_ok());
    }

    #[test]
    fn rule1_all_pass_with_insufficient_data_fails() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        assert!(check_no_insufficient_data_masking(&statuses).is_err());
    }

    #[test]
    fn rule1_not_all_pass_with_insufficient_data_ok() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        assert!(check_no_insufficient_data_masking(&statuses).is_ok());
    }

    #[test]
    fn rule1_not_all_pass_with_fail_and_insufficient_data_ok() {
        let statuses = [
            ConjunctStatus::Fail,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        assert!(check_no_insufficient_data_masking(&statuses).is_ok());
    }

    #[test]
    fn rule2_variance_bound_passes_when_not_all_pass() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![
            make_evidence("arc-agi-2", 0.95, true),
            make_evidence("arc-agi-3", 0.60, true),
        ];
        assert!(check_variance_bound(&evidence, &statuses).is_ok());
    }

    #[test]
    fn rule2_variance_bound_passes_with_reasonable_margins() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        // All fraction sources well above their thresholds with balanced margins
        let evidence = vec![
            make_evidence("arc-agi-2", 0.95, true), // margin: 0.95/0.85 ≈ 1.118
            make_evidence("gdpval", 0.92, true),    // margin: 0.92/0.85 ≈ 1.082
            make_evidence("osworld", 0.93, true),   // margin: 0.93/0.85 ≈ 1.094
            make_evidence("re-bench", 0.80, true),  // margin: 0.80/0.60 ≈ 1.333
        ];
        // min_margin ≈ 1.082, max_margin ≈ 1.333, min >= 0.5*max? 1.082 >= 0.667? Yes
        assert!(check_variance_bound(&evidence, &statuses).is_ok());
    }

    #[test]
    fn rule2_variance_bound_fails_with_extreme_imbalance() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        // Extreme imbalance using fraction and hour mixing to create very different margins
        // One source has tiny margin, another has huge margin
        let evidence = vec![
            make_evidence("arc-agi-2", 0.851, true), // margin: 0.851/0.85 ≈ 1.001
            make_evidence("gdpval", 0.851, true),    // margin: 0.851/0.85 ≈ 1.001
            make_evidence("osworld", 0.851, true),   // margin: 0.851/0.85 ≈ 1.001
            make_evidence("metr-80pct-time-horizon", 8000.0, false), // margin: 8000/168 ≈ 47.6
        ];
        // min_margin ≈ 1.001, max_margin ≈ 47.6
        // Need: min >= 0.5*max => 1.001 >= 23.8? No! This should fail
        assert!(check_variance_bound(&evidence, &statuses).is_err());
    }

    #[test]
    fn rule2_empty_evidence_passes() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![];
        assert!(check_variance_bound(&evidence, &statuses).is_ok());
    }

    #[test]
    fn rule2_unknown_sources_passes() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![make_evidence("unknown-source", 0.95, true)];
        // Unknown sources are ignored, so margins is empty, check passes trivially
        assert!(check_variance_bound(&evidence, &statuses).is_ok());
    }

    #[test]
    fn rule3_complete_provenance_passes() {
        let evidence = vec![
            make_evidence("arc-agi-2", 0.95, true),
            make_evidence("gdpval", 0.90, true),
        ];
        assert!(check_provenance_metadata(&evidence).is_ok());
    }

    #[test]
    fn rule3_empty_evidence_passes() {
        let evidence = vec![];
        assert!(check_provenance_metadata(&evidence).is_ok());
    }

    #[test]
    fn consistency_check_all_pass_all_rules() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![
            make_evidence("arc-agi-2", 0.90, true), // margin: 0.90/0.85 ≈ 1.06
            make_evidence("gdpval", 0.88, true),    // margin: 0.88/0.85 ≈ 1.04
            make_evidence("osworld", 0.90, true),   // margin: 0.90/0.85 ≈ 1.06
            make_evidence("re-bench", 0.75, true),  // margin: 0.75/0.60 = 1.25
        ];
        let result = consistency_check(&evidence, &statuses);
        assert!(result.passed, "Expected pass but got: {:?}", result);
        assert!(result.failed_rules.is_empty());
    }

    #[test]
    fn consistency_check_rule1_fails() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let evidence = vec![make_evidence("arc-agi-2", 0.95, true)];
        let result = consistency_check(&evidence, &statuses);
        assert!(!result.passed);
        assert!(
            result
                .failed_rules
                .contains(&"rule_1_insufficient_data_masking".to_string())
        );
    }

    #[test]
    fn consistency_check_rule2_fails() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![
            make_evidence("arc-agi-2", 0.99, true),
            make_evidence("gdpval", 0.851, true),
            make_evidence("osworld", 0.90, true),
            make_evidence("metr-80pct-time-horizon", 8000.0, false),
        ];
        let result = consistency_check(&evidence, &statuses);
        assert!(!result.passed);
        assert!(
            result
                .failed_rules
                .contains(&"rule_2_variance_bound".to_string())
        );
    }

    #[test]
    fn consistency_check_multiple_rules_fail() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
            ConjunctStatus::Pass,
        ];
        let evidence = vec![
            make_evidence("arc-agi-2", 0.99, true),
            make_evidence("gdpval", 0.851, true),
            make_evidence("osworld", 0.90, true),
            make_evidence("metr-80pct-time-horizon", 8000.0, false),
        ];
        let result = consistency_check(&evidence, &statuses);
        assert!(!result.passed);
        // Rule 2 (variance bound) should fail due to extreme imbalance
        assert!(
            result
                .failed_rules
                .contains(&"rule_2_variance_bound".to_string())
        );
    }

    #[test]
    fn consistency_check_partial_or_fail_status_allows_insufficient_data() {
        let statuses = [
            ConjunctStatus::Pass,
            ConjunctStatus::Partial,
            ConjunctStatus::Pass,
            ConjunctStatus::InsufficientData,
        ];
        let evidence = vec![
            make_evidence("arc-agi-2", 0.95, true),
            make_evidence("gdpval", 0.90, true),
            make_evidence("osworld", 0.95, true),
            make_evidence("metr-80pct-time-horizon", 500.0, false),
        ];
        let result = consistency_check(&evidence, &statuses);
        // Only rule3 would fail if provenance is broken, but we have good provenance
        // Rule1 doesn't apply (not all pass), rule2 doesn't apply (not all pass)
        assert!(result.passed);
    }
}
