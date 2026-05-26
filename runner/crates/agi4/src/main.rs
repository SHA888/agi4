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
                eprintln!("Error: live attestation not yet wired (v0.1.0 stub)");
                std::process::exit(1);
            }

            if let Some(fixture_path) = fixture {
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
    _fixture_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use agi4_schema::{
        ConjunctReport, ConjunctsOutput, ConsistencyCheckOutput, ModelMetadata, VerdictOutput,
    };
    use chrono::Utc;

    let verdict_output = VerdictOutput {
        spec_version: agi4::SPEC_VERSION.to_string(),
        runner_version: agi4::VERSION.to_string(),
        run_timestamp: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        model: ModelMetadata {
            id: model.to_string(),
            provider: None,
            version_or_date: None,
        },
        conjuncts: ConjunctsOutput {
            generality: ConjunctReport {
                status: "insufficient_data".to_string(),
                evidence: vec![],
                margins: None,
            },
            economic_substitutability: ConjunctReport {
                status: "insufficient_data".to_string(),
                evidence: vec![],
                margins: None,
            },
            environmental_transfer: ConjunctReport {
                status: "insufficient_data".to_string(),
                evidence: vec![],
                margins: None,
            },
            autonomous_agency: ConjunctReport {
                status: "insufficient_data".to_string(),
                evidence: vec![],
                margins: None,
            },
        },
        consistency_check: ConsistencyCheckOutput {
            status: "pass".to_string(),
            failed_rules: vec![],
            detail: None,
        },
        verdict: "insufficient_data".to_string(),
        verdict_reasons: vec![
            "generality: insufficient data".to_string(),
            "economic_substitutability: insufficient data".to_string(),
            "environmental_transfer: insufficient data".to_string(),
            "autonomous_agency: insufficient data".to_string(),
        ],
        known_gaps_acknowledged: vec!["All conjuncts require evidence data".to_string()],
    };

    Ok(serde_json::to_string_pretty(&verdict_output)?)
}

fn render_verdict_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(path)?;
    let verdict: agi4_schema::VerdictOutput = serde_json::from_str(&json_str)?;
    Ok(agi4::render_verdict(&verdict))
}
