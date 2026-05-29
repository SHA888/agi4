//! Fixture-based evidence loading for testing and deterministic attestation.
//!
//! Loads frozen upstream data from a fixture directory, converts through adapters,
//! and produces evidence for verdict evaluation.

use agi4_adapters::{
    ModelId, Source, apex_agents::ApexAgentsAdapter, arc_prize::ArcPrizeAdapter,
    gdpval::GdpvalAdapter, gpqa_diamond::GpqaDiamondAdapter, hle::HleAdapter, metr::MetrAdapter,
    osworld::OsworldAdapter, re_bench::ReBenchAdapter, rli::RliAdapter, swe_bench::SweBenchAdapter,
};
use agi4_core::evidence::Evidence;
use std::fs;
use std::path::Path;

/// Load all evidence from a fixture directory.
///
/// The fixture directory is expected to have subdirectories named after each source,
/// each containing a single JSON file with fixture data. For example:
/// ```text
/// fixtures/
///   metr/
///     time-horizon-168h.json
///   arc-prize/
///     leaderboard-example.json
///   ... (other sources)
/// ```
pub fn load_evidence_from_fixtures(
    fixture_dir: &str,
    model_id: &str,
) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
    let model = ModelId::new(model_id);
    let fixture_path = Path::new(fixture_dir);
    let mut all_evidence = Vec::new();

    if !fixture_path.is_dir() {
        return Err(format!("fixture directory not found: {}", fixture_dir).into());
    }

    // List all subdirectories and attempt to load from each known source
    for entry in fs::read_dir(fixture_path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let dir_name = path.file_name().ok_or("invalid directory name")?;
        let source_name = dir_name.to_string_lossy().to_string();

        // Find the first JSON file in this directory
        let json_file = match fs::read_dir(&path) {
            Ok(entries) => {
                let mut found_file = None;
                for f in entries {
                    let f = f?;
                    let fpath = f.path();
                    if fpath.extension().is_some_and(|ext| ext == "json") {
                        found_file = Some(fpath);
                        break;
                    }
                }
                found_file
            }
            Err(_) => continue,
        };

        let json_file = match json_file {
            Some(f) => f,
            None => continue,
        };

        let json_content = fs::read_to_string(&json_file)?;

        // Load evidence from the appropriate adapter
        let source_evidence = match source_name.as_str() {
            "metr" => {
                let adapter = MetrAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert METR evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse METR fixture: {}", e);
                        continue;
                    }
                }
            }
            "arc-prize" => {
                let adapter = ArcPrizeAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert ARC Prize evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse ARC Prize fixture: {}", e);
                        continue;
                    }
                }
            }
            "hle" => {
                let adapter = HleAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert HLE evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse HLE fixture: {}", e);
                        continue;
                    }
                }
            }
            "gpqa-diamond" => {
                let adapter = GpqaDiamondAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert GPQA Diamond evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse GPQA Diamond fixture: {}", e);
                        continue;
                    }
                }
            }
            "gdpval" => {
                let adapter = GdpvalAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert GDPval evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse GDPval fixture: {}", e);
                        continue;
                    }
                }
            }
            "rli" => {
                let adapter = RliAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert RLI evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse RLI fixture: {}", e);
                        continue;
                    }
                }
            }
            "apex-agents" => {
                let adapter = ApexAgentsAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert APEX-Agents evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse APEX-Agents fixture: {}", e);
                        continue;
                    }
                }
            }
            "osworld" => {
                let adapter = OsworldAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert OSWorld evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse OSWorld fixture: {}", e);
                        continue;
                    }
                }
            }
            "re-bench" => {
                let adapter = ReBenchAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert RE-Bench evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse RE-Bench fixture: {}", e);
                        continue;
                    }
                }
            }
            "swe-bench" => {
                let adapter = SweBenchAdapter::default();
                match adapter.parse(&json_content) {
                    Ok(raw) => match adapter.to_evidence(raw, &model) {
                        Ok(evidence) => evidence,
                        Err(e) => {
                            eprintln!("warning: failed to convert SWE-Bench evidence: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("warning: failed to parse SWE-Bench fixture: {}", e);
                        continue;
                    }
                }
            }
            _ => {
                eprintln!("warning: unknown source directory: {}", source_name);
                continue;
            }
        };

        all_evidence.extend(source_evidence);
    }

    Ok(all_evidence)
}
