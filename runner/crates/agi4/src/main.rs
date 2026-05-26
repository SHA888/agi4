use clap::{Parser, Subcommand};

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
        Commands::Attest { model, fixture, live } => {
            if live {
                eprintln!("Error: live attestation not yet wired (v0.1.0 stub)");
                std::process::exit(1);
            }

            if let Some(fixture_path) = fixture {
                println!("Would attest model '{}' using fixture '{}'", model, fixture_path);
            } else {
                eprintln!("Error: either --fixture or --live must be specified");
                std::process::exit(1);
            }
        }
        Commands::Render { input } => {
            println!("Would render verdict from '{}'", input);
        }
        Commands::Schema => {
            println!("Schema command (to be implemented in v0.1.0)");
        }
        Commands::Version => {
            println!("agi4 {}", agi4::VERSION);
            println!("spec version {}", agi4::SPEC_VERSION);
        }
    }
}
