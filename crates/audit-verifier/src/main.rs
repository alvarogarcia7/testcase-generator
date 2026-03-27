use anyhow::{Context, Result};
use audit_verifier::signing;
use clap::Parser;
use serde::{Deserialize, Serialize};
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

    /// Path to P-521 private key PEM file (optional, generates new key if not provided)
    #[arg(short = 'k', long, value_name = "PATH")]
    private_key: Option<PathBuf>,

    /// Path to save generated private key (only used when no key is provided)
    #[arg(long, value_name = "PATH")]
    save_key: Option<PathBuf>,

    /// Key identifier to include in signature output
    #[arg(long, value_name = "ID", default_value = "audit-verifier")]
    key_id: String,

    /// Output file for signed audit verification results (JSON format)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ExecutionLogEntry {
    #[serde(default)]
    source_yaml_sha256: Option<String>,
}

#[derive(Debug, Serialize)]
struct VerificationResult {
    computed_hash: String,
    total_entries: usize,
    hash_mismatches: usize,
    missing_hash_fields: usize,
    verification_passed: bool,
}

#[derive(Debug, Serialize)]
struct SignedAuditOutput {
    verification_result: VerificationResult,
    execution_log_sha256: String,
    signature: String,
    public_key: String,
    key_id: String,
    timestamp: String,
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
    } else {
        println!("\n✗ Verification failed");
    }

    // Compute SHA-256 hash of the execution log content
    let mut log_hasher = Sha256::new();
    log_hasher.update(log_content.as_bytes());
    let execution_log_hash = format!("{:x}", log_hasher.finalize());

    // Create verification result
    let verification_result = VerificationResult {
        computed_hash: computed_hash.clone(),
        total_entries: log_entries.len(),
        hash_mismatches: mismatch_count,
        missing_hash_fields: missing_hash_count,
        verification_passed: all_match,
    };

    // Load or generate private key
    let private_key = if let Some(key_path) = &cli.private_key {
        println!("\nLoading private key from: {}", key_path.display());
        signing::load_private_key(key_path).context("Failed to load private key")?
    } else {
        println!("\nGenerating new P-521 private key...");
        let key = signing::generate_private_key();

        if let Some(save_path) = &cli.save_key {
            println!("Saving private key to: {}", save_path.display());
            signing::save_private_key(&key, save_path).context("Failed to save private key")?;
        }

        key
    };

    // Get public key
    let public_key = signing::get_public_key(&private_key);
    let public_key_pem = signing::public_key_to_pem(&public_key);

    // Create canonical representation of verification result for signing
    let verification_json = serde_json::to_string(&verification_result)
        .context("Failed to serialize verification result")?;

    // Compute SHA-256 hash of the verification result
    let mut sig_hasher = Sha256::new();
    sig_hasher.update(verification_json.as_bytes());
    let message_hash = sig_hasher.finalize();

    // Sign the hash
    let signature = signing::sign_message(&private_key, &message_hash);
    let signature_hex = hex::encode(signature);

    // Create signed output
    let signed_output = SignedAuditOutput {
        verification_result,
        execution_log_sha256: execution_log_hash,
        signature: signature_hex,
        public_key: public_key_pem,
        key_id: cli.key_id.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Output the signed result
    let output_json = serde_json::to_string_pretty(&signed_output)
        .context("Failed to serialize signed output")?;

    if let Some(output_path) = &cli.output {
        fs::write(output_path, &output_json).context(format!(
            "Failed to write output to: {}",
            output_path.display()
        ))?;
        println!(
            "\n✓ Signed audit verification written to: {}",
            output_path.display()
        );
    } else {
        println!("\n--- Signed Audit Verification Output ---");
        println!("{}", output_json);
    }

    if all_match {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
