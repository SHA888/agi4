//! Markdown report rendering for AGI/4 verdicts.
//!
//! Converts VerdictOutput JSON into human-readable Markdown with
//! provenance links, evidence tables, and per-conjunct sections.

use agi4_schema::{ConjunctReport, VerdictOutput};

/// Render a verdict as Markdown.
///
/// Produces a complete human-readable report including:
/// - Verdict summary with model metadata
/// - Per-conjunct evaluation with evidence tables
/// - Margin analysis and consistency check results
/// - Known gaps acknowledgments
pub fn render(verdict: &VerdictOutput) -> String {
    let mut output = String::new();

    output.push_str("# AGI/4 Attestation Verdict\n\n");

    render_metadata(&mut output, verdict);
    render_conjuncts(&mut output, verdict);
    render_consistency_check(&mut output, verdict);
    render_verdict_summary(&mut output, verdict);
    render_known_gaps(&mut output, verdict);

    output
}

fn render_metadata(output: &mut String, verdict: &VerdictOutput) {
    output.push_str("## Evaluation Metadata\n\n");
    output.push_str(&format!("**Model:** {}\n", verdict.model.id));

    if let Some(provider) = &verdict.model.provider {
        output.push_str(&format!("**Provider:** {}\n", provider));
    }

    if let Some(version) = &verdict.model.version_or_date {
        output.push_str(&format!("**Version/Date:** {}\n", version));
    }

    output.push_str(&format!(
        "**Specification Version:** {}\n",
        verdict.spec_version
    ));
    output.push_str(&format!("**Runner Version:** {}\n", verdict.runner_version));
    output.push_str(&format!("**Run Timestamp:** {}\n", verdict.run_timestamp));
    output.push('\n');
}

fn render_conjuncts(output: &mut String, verdict: &VerdictOutput) {
    output.push_str("## Per-Conjunct Evaluation\n\n");

    output.push_str(&render_conjunct_section(
        "Generality",
        &verdict.conjuncts.generality,
    ));
    output.push_str(&render_conjunct_section(
        "Economic Substitutability",
        &verdict.conjuncts.economic_substitutability,
    ));
    output.push_str(&render_conjunct_section(
        "Environmental Transfer",
        &verdict.conjuncts.environmental_transfer,
    ));
    output.push_str(&render_conjunct_section(
        "Autonomous Agency",
        &verdict.conjuncts.autonomous_agency,
    ));
}

fn render_conjunct_section(name: &str, conjunct: &ConjunctReport) -> String {
    let mut section = String::new();

    section.push_str(&format!("### {}\n\n", name));
    section.push_str(&format!("**Status:** `{}`\n\n", conjunct.status));

    if !conjunct.evidence.is_empty() {
        section.push_str("#### Evidence\n\n");
        section.push_str("| Source | Measurement | Value | Threshold | Passes |\n");
        section.push_str("|--------|-------------|-------|-----------|--------|\n");

        for evidence in &conjunct.evidence {
            let passes = evidence
                .passes_threshold
                .map(|p| if p { "✓" } else { "✗" })
                .unwrap_or("—");
            let threshold = evidence
                .threshold
                .map(|t| format!("{:.2}", t))
                .unwrap_or_else(|| "—".to_string());

            section.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                evidence.source, evidence.measurement, evidence.value, threshold, passes
            ));
        }

        section.push('\n');

        section.push_str("#### Evidence Provenance\n\n");
        for evidence in &conjunct.evidence {
            section.push_str(&format!(
                "**{}:** [{}]({})\n",
                evidence.source, evidence.measurement, evidence.provenance.source_url
            ));
        }
        section.push('\n');
    }

    if let Some(margins) = &conjunct.margins {
        section.push_str(&format!(
            "#### Margins\n\n- **Min:** {:.2}\n- **Max:** {:.2}\n\n",
            margins.min, margins.max
        ));
    }

    section
}

fn render_consistency_check(output: &mut String, verdict: &VerdictOutput) {
    output.push_str("## Consistency Check\n\n");
    output.push_str(&format!(
        "**Status:** `{}`\n\n",
        verdict.consistency_check.status
    ));

    if !verdict.consistency_check.failed_rules.is_empty() {
        output.push_str("**Failed Rules:**\n\n");
        for rule in &verdict.consistency_check.failed_rules {
            output.push_str(&format!("- {}\n", rule));
        }
        output.push('\n');
    }

    if let Some(detail) = &verdict.consistency_check.detail {
        output.push_str(&format!("**Detail:** {}\n\n", detail));
    }
}

fn render_verdict_summary(output: &mut String, verdict: &VerdictOutput) {
    output.push_str("## Verdict\n\n");
    output.push_str(&format!(
        "**Result:** `{}`\n\n",
        verdict.verdict.to_uppercase()
    ));

    if !verdict.verdict_reasons.is_empty() {
        output.push_str("**Reasons:**\n\n");
        for reason in &verdict.verdict_reasons {
            output.push_str(&format!("- {}\n", reason));
        }
        output.push('\n');
    }
}

fn render_known_gaps(output: &mut String, verdict: &VerdictOutput) {
    if !verdict.known_gaps_acknowledged.is_empty() {
        output.push_str("## Known Gaps Acknowledged\n\n");
        for gap in &verdict.known_gaps_acknowledged {
            output.push_str(&format!("- {}\n", gap));
        }
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agi4_schema::{
        ConjunctReport, ConjunctsOutput, ConsistencyCheckOutput, ModelMetadata, VerdictOutput,
    };

    fn create_test_verdict() -> VerdictOutput {
        VerdictOutput {
            spec_version: "0.1.0".to_string(),
            runner_version: "0.1.0".to_string(),
            run_timestamp: "2026-05-26T00:00:00Z".to_string(),
            model: ModelMetadata {
                id: "test-model".to_string(),
                provider: Some("test-lab".to_string()),
                version_or_date: Some("2026-05-26".to_string()),
            },
            conjuncts: ConjunctsOutput {
                generality: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                economic_substitutability: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                environmental_transfer: ConjunctReport {
                    status: "partial".to_string(),
                    evidence: vec![],
                    margins: None,
                },
                autonomous_agency: ConjunctReport {
                    status: "pass".to_string(),
                    evidence: vec![],
                    margins: None,
                },
            },
            consistency_check: ConsistencyCheckOutput {
                status: "pass".to_string(),
                failed_rules: vec![],
                detail: None,
            },
            verdict: "not_attested".to_string(),
            verdict_reasons: vec!["environmental_transfer".to_string()],
            known_gaps_acknowledged: vec!["nes_underspecified".to_string()],
        }
    }

    #[test]
    fn render_produces_markdown() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(!markdown.is_empty());
        assert!(markdown.contains("# AGI/4 Attestation Verdict"));
        assert!(markdown.contains("test-model"));
    }

    #[test]
    fn render_includes_metadata() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("## Evaluation Metadata"));
        assert!(markdown.contains("**Model:** test-model"));
        assert!(markdown.contains("**Provider:** test-lab"));
        assert!(markdown.contains("**Version/Date:** 2026-05-26"));
        assert!(markdown.contains("**Specification Version:** 0.1.0"));
        assert!(markdown.contains("**Runner Version:** 0.1.0"));
        assert!(markdown.contains("**Run Timestamp:** 2026-05-26T00:00:00Z"));
    }

    #[test]
    fn render_includes_conjuncts() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("## Per-Conjunct Evaluation"));
        assert!(markdown.contains("### Generality"));
        assert!(markdown.contains("### Economic Substitutability"));
        assert!(markdown.contains("### Environmental Transfer"));
        assert!(markdown.contains("### Autonomous Agency"));
    }

    #[test]
    fn render_includes_conjunct_status() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("`pass`"));
        assert!(markdown.contains("`partial`"));
    }

    #[test]
    fn render_includes_consistency_check() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("## Consistency Check"));
        assert!(markdown.contains("**Status:**"));
    }

    #[test]
    fn render_includes_verdict_summary() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("## Verdict"));
        assert!(markdown.contains("**Result:**"));
        assert!(markdown.contains("NOT_ATTESTED"));
    }

    #[test]
    fn render_includes_verdict_reasons() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("**Reasons:**"));
        assert!(markdown.contains("environmental_transfer"));
    }

    #[test]
    fn render_includes_known_gaps() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        assert!(markdown.contains("## Known Gaps Acknowledged"));
        assert!(markdown.contains("nes_underspecified"));
    }

    #[test]
    fn render_snapshot_test() {
        let verdict = create_test_verdict();
        let markdown = render(&verdict);

        let expected = r#"# AGI/4 Attestation Verdict

## Evaluation Metadata

**Model:** test-model
**Provider:** test-lab
**Version/Date:** 2026-05-26
**Specification Version:** 0.1.0
**Runner Version:** 0.1.0
**Run Timestamp:** 2026-05-26T00:00:00Z

## Per-Conjunct Evaluation

### Generality

**Status:** `pass`

### Economic Substitutability

**Status:** `pass`

### Environmental Transfer

**Status:** `partial`

### Autonomous Agency

**Status:** `pass`

## Consistency Check

**Status:** `pass`

## Verdict

**Result:** `NOT_ATTESTED`

**Reasons:**

- environmental_transfer

## Known Gaps Acknowledged

- nes_underspecified

"#;

        assert_eq!(markdown.trim(), expected.trim());
    }
}
