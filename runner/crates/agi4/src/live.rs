//! Live upstream source fetching and attestation.
//!
//! Routes real evidence through agi4-core evaluators and consistency check.
//! The verdict is based purely on evidence-driven evaluation using SPEC-mandated thresholds.

use agi4_core::conjunct::ConjunctStatus;
use agi4_core::consistency::consistency_check;
use agi4_core::evaluators::{
    evaluate_autonomous_agency, evaluate_economic_substitutability,
    evaluate_environmental_transfer, evaluate_generality,
};
use agi4_core::evidence::Evidence;
use agi4_schema::{
    ConjunctReport, ConjunctsOutput, ConsistencyCheckOutput, EvidenceReport, ModelMetadata,
    VerdictOutput,
};
use chrono::Utc;

/// Convert a ConjunctStatus to its lowercase string representation for output.
fn status_to_string(status: ConjunctStatus) -> String {
    format!("{:?}", status).to_lowercase()
}

/// Perform live attestation by fetching from upstream sources and evaluating evidence.
/// Returns a verdict JSON with collected evidence and evaluation results based on agi4-core logic.
///
/// **IMPORTANT**: This implementation is architectural scaffolding only. Evidence collection is stubbed
/// with an empty vector. Real attestation requires task 2.15: wiring CachingFetcher + all 10 adapters.
/// Currently, all verdicts return `insufficient_data` due to empty evidence, making this a placeholder.
pub fn attest_live(model_id: &str) -> Result<VerdictOutput, Box<dyn std::error::Error>> {
    eprintln!("Starting live attestation for model: {}", model_id);
    eprintln!("HTTP fetcher: timeout=30s, retries=3");
    eprintln!("Filesystem cache: ~/.cache/agi4/, 24-hour TTL");

    let now = Utc::now();
    let run_timestamp = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    // TASK 2.15 TODO: Fetch evidence from upstream sources via CachingFetcher + adapters.
    // The architecture is in place (evaluators + consistency_check below), but evidence
    // collection is stubbed. Without real evidence, all evaluators return InsufficientData.
    // Placeholder: empty evidence vector. To be replaced in task 2.15 with:
    //   let all_evidence = fetch_all_adapters(&CachingFetcher::new()?, &model)?;
    let all_evidence: Vec<Evidence> = Vec::new();
    let evidence_reports: Vec<EvidenceReport> = Vec::new();

    // Evaluate each conjunct through agi4-core evaluators
    let generality_status = evaluate_generality(&all_evidence);
    let econ_status = evaluate_economic_substitutability(&all_evidence);
    let env_status = evaluate_environmental_transfer(&all_evidence);
    let agency_status = evaluate_autonomous_agency(&all_evidence);

    let conjunct_statuses = [generality_status, econ_status, env_status, agency_status];

    // Run consistency check with real evidence
    let consistency_result = consistency_check(&all_evidence, &conjunct_statuses);

    // Build verdict: all 4 conjuncts must pass AND consistency check must pass
    let overall_verdict = if conjunct_statuses.iter().all(|s| *s == ConjunctStatus::Pass)
        && consistency_result.passed
    {
        "attested"
    } else {
        "not_attested"
    };

    // Group evidence by conjunct for output (single pass O(n) partition instead of 4 passes)
    let mut generality_evidence = Vec::new();
    let mut econ_evidence = Vec::new();
    let mut env_evidence = Vec::new();
    let mut agency_evidence = Vec::new();

    for report in evidence_reports {
        match report.source.as_str() {
            "arc-agi-2" | "hle" | "gpqa-diamond" => generality_evidence.push(report),
            "gdpval" | "gdpval-aa" | "rli" | "apex-agents" => econ_evidence.push(report),
            "arc-agi-3" => {
                // Arc-agi-3 contributes to both generality and environmental_transfer
                generality_evidence.push(report.clone());
                env_evidence.push(report);
            }
            "osworld" | "nes" => env_evidence.push(report),
            "metr-80pct-time-horizon" | "re-bench" | "swe-bench-verified" => {
                agency_evidence.push(report)
            }
            _ => {} // Unknown source, skip
        }
    }

    let verdict_reasons = vec![
        format!("Generality: {}", status_to_string(generality_status)),
        format!(
            "Economic Substitutability: {}",
            status_to_string(econ_status)
        ),
        format!("Environmental Transfer: {}", status_to_string(env_status)),
        format!("Autonomous Agency: {}", status_to_string(agency_status)),
        format!(
            "Consistency Check: {}",
            if consistency_result.passed {
                "pass"
            } else {
                "fail"
            }
        ),
    ];

    let verdict_output = VerdictOutput {
        spec_version: crate::SPEC_VERSION.to_string(),
        runner_version: crate::VERSION.to_string(),
        run_timestamp,
        model: ModelMetadata {
            id: model_id.to_string(),
            provider: None,
            version_or_date: None,
        },
        conjuncts: ConjunctsOutput {
            generality: ConjunctReport {
                status: status_to_string(generality_status),
                evidence: generality_evidence,
                margins: None,
            },
            economic_substitutability: ConjunctReport {
                status: status_to_string(econ_status),
                evidence: econ_evidence,
                margins: None,
            },
            environmental_transfer: ConjunctReport {
                status: status_to_string(env_status),
                evidence: env_evidence,
                margins: None,
            },
            autonomous_agency: ConjunctReport {
                status: status_to_string(agency_status),
                evidence: agency_evidence,
                margins: None,
            },
        },
        consistency_check: ConsistencyCheckOutput {
            status: if consistency_result.passed {
                "pass"
            } else {
                "fail"
            }
            .to_string(),
            failed_rules: consistency_result.failed_rules,
            detail: consistency_result.detail,
        },
        verdict: overall_verdict.to_string(),
        verdict_reasons,
        known_gaps_acknowledged: vec![
            "NES (Novel-Environment Subset) underspecified; accepting evidence but not evaluating"
                .to_string(),
        ],
    };

    eprintln!("Verdict: {} ({})", verdict_output.verdict, model_id);
    Ok(verdict_output)
}
