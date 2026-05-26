//! Markdown report rendering for AGI/4 verdicts.
//!
//! Converts VerdictOutput JSON into human-readable Markdown with
//! provenance links and per-conjunct sections.

use agi4_schema::VerdictOutput;

/// Render a verdict as Markdown.
pub fn render(verdict: &VerdictOutput) -> String {
    let mut output = String::new();

    output.push_str("# AGI/4 Attestation Verdict\n\n");
    output.push_str(&format!("**Model:** {}\n", verdict.model.id));
    output.push_str(&format!(
        "**Specification Version:** {}\n",
        verdict.spec_version
    ));
    output.push_str(&format!("**Runner Version:** {}\n", verdict.runner_version));
    output.push_str(&format!("**Run Timestamp:** {}\n", verdict.run_timestamp));
    output.push_str(&format!(
        "**Verdict:** {}\n\n",
        verdict.verdict.to_uppercase()
    ));

    output.push_str("## Conjuncts\n\n");
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

    output
}

fn render_conjunct_section(name: &str, conjunct: &agi4_schema::ConjunctOutput) -> String {
    format!("### {}\n\n**Status:** {}\n\n", name, conjunct.status)
}
