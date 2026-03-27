use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "audit-verifier")]
#[command(version)]
#[command(about = "Verify test case YAML hash against execution log entries")]
struct Cli {
    /// Path to test case YAML file
    #[arg(short, long, value_name = "PATH")]
    yaml: PathBuf,

    /// Path to execution log JSON file
    #[arg(short, long, value_name = "PATH")]
    log: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ExecutionLogEntry {
    #[serde(default)]
    source_yaml_sha256: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate that files exist
    if !cli.yaml.exists() {
        anyhow::bail!("YAML file does not exist: {}", cli.yaml.display());
    }
    if !cli.log.exists() {
        anyhow::bail!("Log file does not exist: {}", cli.log.display());
    }

    // Compute SHA-256 hash of the YAML file
    let yaml_bytes =
        fs::read(&cli.yaml).context(format!("Failed to read YAML file: {}", cli.yaml.display()))?;

    let mut hasher = Sha256::new();
    hasher.update(&yaml_bytes);
    let computed_hash = format!("{:x}", hasher.finalize());

    println!("Computed SHA-256 hash of YAML file: {}", computed_hash);

    // Parse execution log JSON
    let log_content = fs::read_to_string(&cli.log)
        .context(format!("Failed to read log file: {}", cli.log.display()))?;

    let log_entries: Vec<ExecutionLogEntry> = serde_json::from_str(&log_content).context(
        format!("Failed to parse log file as JSON: {}", cli.log.display()),
    )?;

    if log_entries.is_empty() {
        println!("WARNING: Execution log is empty");
        std::process::exit(1);
    }

    // Extract all source_yaml_sha256 fields and compare
    let mut all_match = true;
    let mut missing_hash_count = 0;
    let mut mismatch_count = 0;

    for (index, entry) in log_entries.iter().enumerate() {
        match &entry.source_yaml_sha256 {
            Some(hash) => {
                if hash != &computed_hash {
                    eprintln!(
                        "ERROR: Hash mismatch at entry {}: expected '{}', got '{}'",
                        index + 1,
                        computed_hash,
                        hash
                    );
                    all_match = false;
                    mismatch_count += 1;
                }
            }
            None => {
                eprintln!(
                    "WARNING: Missing source_yaml_sha256 field at entry {}",
                    index + 1
                );
                all_match = false;
                missing_hash_count += 1;
            }
        }
    }

    // Print summary
    println!("\nVerification Summary:");
    println!("  Total entries: {}", log_entries.len());
    println!("  Hash mismatches: {}", mismatch_count);
    println!("  Missing hash fields: {}", missing_hash_count);

    if all_match {
        println!("\n✓ All hashes match and no missing hash fields");
        std::process::exit(0);
    } else {
        println!("\n✗ Verification failed");
        std::process::exit(1);
    }
}
