use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "verify-audit-signature")]
#[command(version)]
#[command(about = "Verify cryptographic signature of audit verification output")]
struct Cli {
    /// Path to signed audit verification JSON file
    #[arg(short, long, value_name = "PATH")]
    input: PathBuf,

    /// Display detailed information about the signature
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        anyhow::bail!("Input file does not exist: {}", cli.input.display());
    }

    log::info!("Verifying signed audit output: {}", cli.input.display());
    log::info!("");

    let is_valid = audit_verifier::verify_signature::verify_signed_audit(&cli.input)
        .context("Failed to verify signature")?;

    if cli.verbose {
        let content = std::fs::read_to_string(&cli.input).context("Failed to read input file")?;
        let signed_output: audit_verifier::verify_signature::SignedAuditOutput =
            serde_json::from_str(&content).context("Failed to parse signed output JSON")?;

        log::info!("=== Audit Information ===");
        log::info!("Key ID: {}", signed_output.key_id);
        log::info!("Timestamp: {}", signed_output.timestamp);
        log::info!(
            "Execution Log SHA-256: {}",
            signed_output.execution_log_sha256
        );
        log::info!("");
        log::info!("=== Verification Result ===");
        log::info!(
            "Computed Hash: {}",
            signed_output.verification_result.computed_hash
        );
        log::info!(
            "Total Entries: {}",
            signed_output.verification_result.total_entries
        );
        log::info!(
            "Hash Mismatches: {}",
            signed_output.verification_result.hash_mismatches
        );
        log::info!(
            "Missing Hash Fields: {}",
            signed_output.verification_result.missing_hash_fields
        );
        log::info!(
            "Verification Passed: {}",
            signed_output.verification_result.verification_passed
        );
        log::info!("");
        log::info!("=== Signature ===");
        log::info!(
            "Signature (first 64 chars): {}...",
            &signed_output.signature[..64.min(signed_output.signature.len())]
        );
        log::info!("");
        log::info!("=== Public Key ===");
        let lines: Vec<&str> = signed_output.public_key.lines().collect();
        if lines.len() > 5 {
            log::info!("{}", lines[0]);
            log::info!("  ... ({} lines total)", lines.len());
            log::info!("{}", lines[lines.len() - 1]);
        } else {
            log::info!("{}", signed_output.public_key);
        }
        log::info!("");
    }

    log::info!("=== Signature Verification Result ===");
    if is_valid {
        log::info!("✓ SIGNATURE VALID");
        log::info!("");
        log::info!("The audit verification output has a valid cryptographic signature.");
        log::info!("The signature was created by the holder of the private key corresponding");
        log::info!("to the public key included in this output.");
        std::process::exit(0);
    } else {
        log::error!("✗ SIGNATURE INVALID");
        log::error!("");
        log::error!("WARNING: The signature verification failed!");
        log::error!("This could indicate:");
        log::error!("  - The audit output has been tampered with");
        log::error!("  - The signature was created with a different key");
        log::error!("  - The file has been corrupted");
        std::process::exit(1);
    }
}
