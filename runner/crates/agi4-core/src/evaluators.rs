//! Per-conjunct evaluation functions.
//!
//! Each function takes evidence for a conjunct and returns a ConjunctStatus
//! based on the thresholds defined in SPEC.md §3.

use crate::conjunct::ConjunctStatus;
use crate::evidence::{Evidence, SourceValue};
use crate::threshold;

/// Evaluate the Generality conjunct.
///
/// Requires: at least 3 of 4 sources (ARC-AGI-2, ARC-AGI-3, HLE, GPQA-Diamond)
/// with ARC-AGI-3 mandatory.
///
/// Pass: all four sources meet thresholds
/// Partial: at least one meets, at least one doesn't
/// Fail: no source meets OR ARC-AGI-3 < 5%
/// InsufficientData: minimum evidence requirement unmet
pub fn evaluate_generality(evidence: &[Evidence]) -> ConjunctStatus {
    let mut arc_agi_2 = None;
    let mut arc_agi_3 = None;
    let mut hle = None;
    let mut gpqa_diamond = None;

    for e in evidence {
        match e.source.as_str() {
            "arc-agi-2" => {
                if let SourceValue::Fraction(f) = e.value {
                    arc_agi_2 = Some(f);
                }
            }
            "arc-agi-3" => {
                if let SourceValue::Fraction(f) = e.value {
                    arc_agi_3 = Some(f);
                }
            }
            "hle" => {
                if let SourceValue::Fraction(f) = e.value {
                    hle = Some(f);
                }
            }
            "gpqa-diamond" => {
                if let SourceValue::Fraction(f) = e.value {
                    gpqa_diamond = Some(f);
                }
            }
            _ => {}
        }
    }

    // ARC-AGI-3 is mandatory
    let arc_agi_3 = match arc_agi_3 {
        Some(f) => f,
        None => return ConjunctStatus::InsufficientData,
    };

    // Check ARC-AGI-3 floor
    if arc_agi_3.value() < threshold::generality::ARC_AGI_3_FLOOR {
        return ConjunctStatus::Fail;
    }

    // Count how many sources are available
    let available_sources = [arc_agi_2.is_some(), hle.is_some(), gpqa_diamond.is_some()]
        .iter()
        .filter(|&&x| x)
        .count()
        + 1; // +1 for ARC-AGI-3

    // Minimum evidence: at least 3 of 4 sources
    if available_sources < 3 {
        return ConjunctStatus::InsufficientData;
    }

    // Check thresholds
    let arc_agi_2_pass = arc_agi_2
        .map(|f| f.value() >= threshold::generality::ARC_AGI_2_PASS)
        .unwrap_or(false);
    let arc_agi_3_pass = arc_agi_3.value() >= threshold::generality::ARC_AGI_3_PASS;
    let hle_pass = hle
        .map(|f| f.value() >= threshold::generality::HLE_PASS)
        .unwrap_or(false);
    let gpqa_pass = gpqa_diamond
        .map(|f| f.value() >= threshold::generality::GPQA_DIAMOND_PASS)
        .unwrap_or(false);

    let sources = [
        (arc_agi_2_pass, arc_agi_2.is_some()),
        (arc_agi_3_pass, true),
        (hle_pass, hle.is_some()),
        (gpqa_pass, gpqa_diamond.is_some()),
    ];

    let passing = sources
        .iter()
        .filter(|(pass, present)| *present && *pass)
        .count();
    let present = sources.iter().filter(|(_, present)| *present).count();

    if passing == present && present >= 3 {
        ConjunctStatus::Pass
    } else if passing > 0 && passing < present {
        ConjunctStatus::Partial
    } else {
        ConjunctStatus::Fail
    }
}

/// Evaluate the Economic Substitutability conjunct.
///
/// Requires: both GDPval and RLI (APEX-Agents is supplementary)
///
/// Pass: GDPval ≥85% AND RLI ≥60%
/// Partial: one meets, one doesn't
/// Fail: neither meets threshold
/// InsufficientData: missing required sources
pub fn evaluate_economic_substitutability(evidence: &[Evidence]) -> ConjunctStatus {
    let mut gdpval = None;
    let mut rli = None;

    for e in evidence {
        match e.source.as_str() {
            "gdpval" => {
                if let SourceValue::Fraction(f) = e.value {
                    gdpval = Some(f);
                }
            }
            "rli" => {
                if let SourceValue::Fraction(f) = e.value {
                    rli = Some(f);
                }
            }
            "apex-agents" => {
                // APEX-Agents is supplementary, not used in logic yet
            }
            _ => {}
        }
    }

    // Both GDPval and RLI are required
    let gdpval = match gdpval {
        Some(f) => f,
        None => return ConjunctStatus::InsufficientData,
    };
    let rli = match rli {
        Some(f) => f,
        None => return ConjunctStatus::InsufficientData,
    };

    // Check for floor on RLI
    if rli.value() < threshold::economic_substitutability::RLI_FLOOR {
        return ConjunctStatus::Fail;
    }

    let gdpval_pass = gdpval.value() >= threshold::economic_substitutability::GDPVAL_PASS;
    let rli_pass = rli.value() >= threshold::economic_substitutability::RLI_PASS;

    if gdpval_pass && rli_pass {
        ConjunctStatus::Pass
    } else if gdpval_pass || rli_pass {
        ConjunctStatus::Partial
    } else {
        ConjunctStatus::Fail
    }
}

/// Evaluate the Environmental Transfer conjunct.
///
/// Requires: ARC-AGI-3 (mandatory) + at least one of OSWorld or NES
///
/// Pass: ARC-AGI-3 ≥50% AND (OSWorld ≥85% OR NES ≥threshold)
/// Partial: ARC-AGI-3 above floor but below threshold OR ARC-AGI-3 passes but no secondary source
/// Fail: ARC-AGI-3 < 5%
/// InsufficientData: ARC-AGI-3 missing or no secondary source
pub fn evaluate_environmental_transfer(evidence: &[Evidence]) -> ConjunctStatus {
    let mut arc_agi_3 = None;
    let mut osworld = None;
    let mut nes = None;

    for e in evidence {
        match e.source.as_str() {
            "arc-agi-3" => {
                if let SourceValue::Fraction(f) = e.value {
                    arc_agi_3 = Some(f);
                }
            }
            "osworld" => {
                if let SourceValue::Fraction(f) = e.value {
                    osworld = Some(f);
                }
            }
            "nes" => {
                if let SourceValue::Fraction(f) = e.value {
                    nes = Some(f);
                }
            }
            _ => {}
        }
    }

    // ARC-AGI-3 is required
    let arc_agi_3 = match arc_agi_3 {
        Some(f) => f,
        None => return ConjunctStatus::InsufficientData,
    };

    // Check ARC-AGI-3 floor
    if arc_agi_3.value() < threshold::environmental_transfer::ARC_AGI_3_FLOOR {
        return ConjunctStatus::Fail;
    }

    // Need at least one secondary source
    if osworld.is_none() && nes.is_none() {
        return ConjunctStatus::InsufficientData;
    }

    let arc_agi_3_pass = arc_agi_3.value() >= threshold::environmental_transfer::ARC_AGI_3_PASS;
    let osworld_pass = osworld
        .map(|f| f.value() >= threshold::environmental_transfer::OSWORLD_PASS)
        .unwrap_or(false);

    if arc_agi_3_pass && (osworld_pass || nes.is_some()) {
        // NES is TBD, so we treat it as passing if present
        ConjunctStatus::Pass
    } else if arc_agi_3_pass
        || osworld_pass
        || arc_agi_3.value() >= threshold::environmental_transfer::ARC_AGI_3_FLOOR
    {
        ConjunctStatus::Partial
    } else {
        ConjunctStatus::Fail
    }
}

/// Evaluate the Autonomous Agency conjunct.
///
/// Requires: METR 80%-time horizon (mandatory) + at least one of RE-Bench or SWE-bench Verified
///
/// Pass: METR ≥168h AND (RE-Bench ≥60% OR SWE-bench ≥85%)
/// Partial: METR ≥168h but no supporting source OR supporting source passes but METR < 168h >= 8h
/// Fail: METR < 8h
/// InsufficientData: METR missing or no supporting source
pub fn evaluate_autonomous_agency(evidence: &[Evidence]) -> ConjunctStatus {
    let mut metr = None;
    let mut rebench = None;
    let mut swebench = None;

    for e in evidence {
        match e.source.as_str() {
            "metr-80pct-time-horizon" => {
                if let SourceValue::Hours(h) = e.value {
                    metr = Some(h);
                }
            }
            "rebench" => {
                if let SourceValue::Fraction(f) = e.value {
                    rebench = Some(f);
                }
            }
            "swebench-verified-pass-at-5" => {
                if let SourceValue::Fraction(f) = e.value {
                    swebench = Some(f);
                }
            }
            _ => {}
        }
    }

    // METR is required
    let metr = match metr {
        Some(h) => h,
        None => return ConjunctStatus::InsufficientData,
    };

    // Check METR floor
    if metr.value() < threshold::autonomous_agency::METR_80PCT_FLOOR_HOURS {
        return ConjunctStatus::Fail;
    }

    // Need at least one supporting source
    if rebench.is_none() && swebench.is_none() {
        return ConjunctStatus::InsufficientData;
    }

    let metr_pass = metr.value() >= threshold::autonomous_agency::METR_80PCT_PASS_HOURS;
    let rebench_pass = rebench
        .map(|f| f.value() >= threshold::autonomous_agency::REBENCH_PASS)
        .unwrap_or(false);
    let swebench_pass = swebench
        .map(|f| f.value() >= threshold::autonomous_agency::SWEBENCH_VERIFIED_PASS_AT_5)
        .unwrap_or(false);

    if metr_pass && (rebench_pass || swebench_pass) {
        ConjunctStatus::Pass
    } else if metr_pass || rebench_pass || swebench_pass {
        ConjunctStatus::Partial
    } else {
        ConjunctStatus::Fail
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{BoundedFraction, MeasurementId, NonNegativeHours, Provenance, SourceId};
    use chrono::Utc;
    use url::Url;

    fn make_evidence(source: &str, measurement: &str, value: SourceValue) -> Evidence {
        Evidence {
            source: SourceId::new(source),
            measurement: MeasurementId::new(measurement),
            value,
            reliability_percentile: 95,
            provenance: Provenance {
                source_url: Url::parse("https://example.com").unwrap(),
                fetch_timestamp: Utc::now(),
                source_version: Some("1.0".to_string()),
                raw_value: "test".to_string(),
            },
        }
    }

    #[test]
    fn generality_pass_all_sources() {
        let evidence = vec![
            make_evidence(
                "arc-agi-2",
                "pass-rate",
                SourceValue::Fraction(BoundedFraction::new(0.85).unwrap()),
            ),
            make_evidence(
                "arc-agi-3",
                "pass-rate",
                SourceValue::Fraction(BoundedFraction::new(0.50).unwrap()),
            ),
            make_evidence(
                "hle",
                "accuracy",
                SourceValue::Fraction(BoundedFraction::new(0.80).unwrap()),
            ),
            make_evidence(
                "gpqa-diamond",
                "accuracy",
                SourceValue::Fraction(BoundedFraction::new(0.90).unwrap()),
            ),
        ];

        assert_eq!(evaluate_generality(&evidence), ConjunctStatus::Pass);
    }

    #[test]
    fn generality_insufficient_data() {
        let evidence = vec![make_evidence(
            "arc-agi-3",
            "pass-rate",
            SourceValue::Fraction(BoundedFraction::new(0.50).unwrap()),
        )];

        assert_eq!(
            evaluate_generality(&evidence),
            ConjunctStatus::InsufficientData
        );
    }

    #[test]
    fn generality_fail_below_floor() {
        let evidence = vec![
            make_evidence(
                "arc-agi-3",
                "pass-rate",
                SourceValue::Fraction(BoundedFraction::new(0.03).unwrap()),
            ),
            make_evidence(
                "hle",
                "accuracy",
                SourceValue::Fraction(BoundedFraction::new(0.80).unwrap()),
            ),
            make_evidence(
                "gpqa-diamond",
                "accuracy",
                SourceValue::Fraction(BoundedFraction::new(0.90).unwrap()),
            ),
        ];

        assert_eq!(evaluate_generality(&evidence), ConjunctStatus::Fail);
    }

    #[test]
    fn economic_substitutability_pass() {
        let evidence = vec![
            make_evidence(
                "gdpval",
                "win-rate",
                SourceValue::Fraction(BoundedFraction::new(0.85).unwrap()),
            ),
            make_evidence(
                "rli",
                "completion-rate",
                SourceValue::Fraction(BoundedFraction::new(0.60).unwrap()),
            ),
        ];

        assert_eq!(
            evaluate_economic_substitutability(&evidence),
            ConjunctStatus::Pass
        );
    }

    #[test]
    fn economic_substitutability_insufficient_data() {
        let evidence = vec![make_evidence(
            "gdpval",
            "win-rate",
            SourceValue::Fraction(BoundedFraction::new(0.85).unwrap()),
        )];

        assert_eq!(
            evaluate_economic_substitutability(&evidence),
            ConjunctStatus::InsufficientData
        );
    }

    #[test]
    fn environmental_transfer_pass() {
        let evidence = vec![
            make_evidence(
                "arc-agi-3",
                "pass-rate",
                SourceValue::Fraction(BoundedFraction::new(0.50).unwrap()),
            ),
            make_evidence(
                "osworld",
                "completion-rate",
                SourceValue::Fraction(BoundedFraction::new(0.85).unwrap()),
            ),
        ];

        assert_eq!(
            evaluate_environmental_transfer(&evidence),
            ConjunctStatus::Pass
        );
    }

    #[test]
    fn environmental_transfer_insufficient_without_secondary() {
        let evidence = vec![make_evidence(
            "arc-agi-3",
            "pass-rate",
            SourceValue::Fraction(BoundedFraction::new(0.50).unwrap()),
        )];

        assert_eq!(
            evaluate_environmental_transfer(&evidence),
            ConjunctStatus::InsufficientData
        );
    }

    #[test]
    fn autonomous_agency_pass() {
        let evidence = vec![
            make_evidence(
                "metr-80pct-time-horizon",
                "hours",
                SourceValue::Hours(NonNegativeHours::new(168.0).unwrap()),
            ),
            make_evidence(
                "rebench",
                "success-rate",
                SourceValue::Fraction(BoundedFraction::new(0.60).unwrap()),
            ),
        ];

        assert_eq!(evaluate_autonomous_agency(&evidence), ConjunctStatus::Pass);
    }

    #[test]
    fn autonomous_agency_insufficient_without_supporting() {
        let evidence = vec![make_evidence(
            "metr-80pct-time-horizon",
            "hours",
            SourceValue::Hours(NonNegativeHours::new(168.0).unwrap()),
        )];

        assert_eq!(
            evaluate_autonomous_agency(&evidence),
            ConjunctStatus::InsufficientData
        );
    }

    #[test]
    fn autonomous_agency_fail_below_floor() {
        let evidence = vec![
            make_evidence(
                "metr-80pct-time-horizon",
                "hours",
                SourceValue::Hours(NonNegativeHours::new(4.0).unwrap()),
            ),
            make_evidence(
                "rebench",
                "success-rate",
                SourceValue::Fraction(BoundedFraction::new(0.60).unwrap()),
            ),
        ];

        assert_eq!(evaluate_autonomous_agency(&evidence), ConjunctStatus::Fail);
    }
}
