use anyhow::{Context, Result};
use clap::Parser;
use p521::ecdsa::{signature::Verifier, Signature};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "verify-audit-log")]
#[command(version)]
#[command(about = "Verify audit log signature given keypair, payload, and signature")]
struct Cli {
    /// Path to private key PEM file (used to derive public key)
    #[arg(short = 'k', long, value_name = "PATH")]
    keypair: PathBuf,

    /// Path to payload file to verify
    #[arg(short, long, value_name = "PATH")]
    payload: PathBuf,

    /// Path to signature file (hex-encoded)
    #[arg(short, long, value_name = "PATH")]
    signature: PathBuf,

    /// Display detailed information
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.keypair.exists() {
        anyhow::bail!("Keypair file does not exist: {}", cli.keypair.display());
    }
    if !cli.payload.exists() {
        anyhow::bail!("Payload file does not exist: {}", cli.payload.display());
    }
    if !cli.signature.exists() {
        anyhow::bail!("Signature file does not exist: {}", cli.signature.display());
    }

    if cli.verbose {
        log::info!("Verifying audit log signature...");
        log::info!("Keypair: {}", cli.keypair.display());
        log::info!("Payload: {}", cli.payload.display());
        log::info!("Signature: {}", cli.signature.display());
        log::info!("");
    }

    let signing_key = audit_verifier::signing::load_private_key(&cli.keypair)
        .context("Failed to load private key")?;

    let verifying_key = audit_verifier::signing::get_public_key(&signing_key);

    let payload_bytes = fs::read(&cli.payload)
        .context(format!("Failed to read payload: {}", cli.payload.display()))?;

    let mut hasher = Sha256::new();
    hasher.update(&payload_bytes);
    let payload_hash = hasher.finalize();

    if cli.verbose {
        log::info!("Payload SHA-256: {:x}", payload_hash);
    }

    let signature_content = fs::read_to_string(&cli.signature).context(format!(
        "Failed to read signature: {}",
        cli.signature.display()
    ))?;
    let signature_hex = signature_content.trim();

    let signature_bytes = hex::decode(signature_hex).context("Failed to decode signature hex")?;

    let signature = Signature::from_slice(&signature_bytes).context("Failed to parse signature")?;

    if cli.verbose {
        log::info!("Signature length: {} bytes", signature_bytes.len());
        log::info!("");
    }

    match verifying_key.verify(&payload_hash, &signature) {
        Ok(_) => {
            log::info!("✓ SIGNATURE VALID");
            if cli.verbose {
                log::info!("");
                log::info!("The audit log signature is valid.");
                log::info!("The signature was created by the holder of the private key");
                log::info!("corresponding to the provided keypair.");
            }
            std::process::exit(0);
        }
        Err(e) => {
            log::error!("✗ SIGNATURE INVALID");
            if cli.verbose {
                log::error!("");
                log::error!("WARNING: The signature verification failed!");
                log::error!("Error: {:?}", e);
                log::error!("");
                log::error!("This could indicate:");
                log::error!("  - The payload has been tampered with");
                log::error!("  - The signature was created with a different key");
                log::error!("  - The signature or payload file has been corrupted");
            }
            std::process::exit(1);
        }
    }
}
