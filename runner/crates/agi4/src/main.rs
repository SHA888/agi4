use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "agi4")]
#[command(about = "AGI/4 specification and reference runner", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Attest a model against the AGI/4 specification
    Attest {
        /// Model identifier
        #[arg(long)]
        model: String,

        /// Path to fixture directory (for v0.1.0)
        #[arg(long)]
        fixture: Option<String>,

        /// Fetch from live upstream sources (stubbed in v0.1.0)
        #[arg(long)]
        live: bool,
    },

    /// Render a verdict JSON to Markdown
    Render {
        /// Path to verdict JSON file
        #[arg(long)]
        input: String,
    },

    /// Print the output JSON schema
    Schema,

    /// Print version information
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Attest {
            model,
            fixture,
            live,
        } => {
            if live {
                // Wire live attestation: fetch from upstream sources concurrently
                // with timeout=30s and retry=3
                match agi4::live::attest_live(&model) {
                    Ok(verdict_json) => {
                        println!("{}", serde_json::to_string_pretty(&verdict_json).unwrap())
                    }
                    Err(e) => {
                        eprintln!("Error during live attestation: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if let Some(fixture_path) = fixture {
                match attest_from_fixture(&model, &fixture_path) {
                    Ok(verdict_json) => println!("{}", verdict_json),
                    Err(e) => {
                        eprintln!("Error during attestation: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: either --fixture or --live must be specified");
                std::process::exit(1);
            }
        }
        Commands::Render { input } => match render_verdict_file(&input) {
            Ok(markdown) => println!("{}", markdown),
            Err(e) => {
                eprintln!("Error rendering verdict: {}", e);
                std::process::exit(1);
            }
        },
        Commands::Schema => match agi4_schema::schema_json_string() {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Error generating schema: {}", e);
                std::process::exit(1);
            }
        },
        Commands::Version => {
            println!("agi4 {}", agi4::VERSION);
            println!("spec version {}", agi4::SPEC_VERSION);
        }
    }
}

fn attest_from_fixture(
    model: &str,
    fixture_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use agi4::consistency_check;
    use agi4::core::ConjunctStatus;
    use agi4::fixtures::load_evidence_from_fixtures;
    use agi4_core::evaluators::{
        evaluate_autonomous_agency, evaluate_economic_substitutability,
        evaluate_environmental_transfer, evaluate_generality,
    };
    use agi4_schema::{
        ConjunctReport, ConjunctsOutput, ConsistencyCheckOutput, EvidenceReport, ModelMetadata,
        VerdictOutput,
    };
    use chrono::Utc;

    let now = Utc::now();
    let run_timestamp = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    // Load evidence from fixture directory
    let all_evidence = load_evidence_from_fixtures(fixture_dir, model)?;

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

    // Convert statuses to strings for output
    fn status_to_string(status: ConjunctStatus) -> String {
        format!("{:?}", status).to_lowercase()
    }

    // For fixture path, report evidence count per conjunct without detailed evidence reports
    // (detailed evidence reporting requires threshold/floor lookup which is out of scope for task 2.15)
    let generality_evidence: Vec<EvidenceReport> = vec![];
    let econ_evidence: Vec<EvidenceReport> = vec![];
    let env_evidence: Vec<EvidenceReport> = vec![];
    let agency_evidence: Vec<EvidenceReport> = vec![];

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
        spec_version: agi4::SPEC_VERSION.to_string(),
        runner_version: agi4::VERSION.to_string(),
        run_timestamp,
        model: ModelMetadata {
            id: model.to_string(),
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
            failed_rules: consistency_result
                .failed_rules
                .iter()
                .map(|r| r.to_string())
                .collect(),
            detail: consistency_result.detail.map(|d| d.to_string()),
        },
        verdict: overall_verdict.to_string(),
        verdict_reasons,
        known_gaps_acknowledged: vec![
            "Fixture-based attestation uses only provided evidence".to_string(),
        ],
    };

    Ok(serde_json::to_string_pretty(&verdict_output)?)
}

fn render_verdict_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(path)?;
    let verdict: agi4_schema::VerdictOutput = serde_json::from_str(&json_str)?;
    Ok(agi4::render_verdict(&verdict))
}
