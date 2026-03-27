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

    println!("Verifying signed audit output: {}", cli.input.display());
    println!();

    let is_valid = audit_verifier::verify_signature::verify_signed_audit(&cli.input)
        .context("Failed to verify signature")?;

    if cli.verbose {
        let content = std::fs::read_to_string(&cli.input).context("Failed to read input file")?;
        let signed_output: audit_verifier::verify_signature::SignedAuditOutput =
            serde_json::from_str(&content).context("Failed to parse signed output JSON")?;

        println!("=== Audit Information ===");
        println!("Key ID: {}", signed_output.key_id);
        println!("Timestamp: {}", signed_output.timestamp);
        println!(
            "Execution Log SHA-256: {}",
            signed_output.execution_log_sha256
        );
        println!();
        println!("=== Verification Result ===");
        println!(
            "Computed Hash: {}",
            signed_output.verification_result.computed_hash
        );
        println!(
            "Total Entries: {}",
            signed_output.verification_result.total_entries
        );
        println!(
            "Hash Mismatches: {}",
            signed_output.verification_result.hash_mismatches
        );
        println!(
            "Missing Hash Fields: {}",
            signed_output.verification_result.missing_hash_fields
        );
        println!(
            "Verification Passed: {}",
            signed_output.verification_result.verification_passed
        );
        println!();
        println!("=== Signature ===");
        println!(
            "Signature (first 64 chars): {}...",
            &signed_output.signature[..64.min(signed_output.signature.len())]
        );
        println!();
        println!("=== Public Key ===");
        let lines: Vec<&str> = signed_output.public_key.lines().collect();
        if lines.len() > 5 {
            println!("{}", lines[0]);
            println!("  ... ({} lines total)", lines.len());
            println!("{}", lines[lines.len() - 1]);
        } else {
            println!("{}", signed_output.public_key);
        }
        println!();
    }

    println!("=== Signature Verification Result ===");
    if is_valid {
        println!("✓ SIGNATURE VALID");
        println!();
        println!("The audit verification output has a valid cryptographic signature.");
        println!("The signature was created by the holder of the private key corresponding");
        println!("to the public key included in this output.");
        std::process::exit(0);
    } else {
        println!("✗ SIGNATURE INVALID");
        println!();
        println!("WARNING: The signature verification failed!");
        println!("This could indicate:");
        println!("  - The audit output has been tampered with");
        println!("  - The signature was created with a different key");
        println!("  - The file has been corrupted");
        std::process::exit(1);
    }
}
