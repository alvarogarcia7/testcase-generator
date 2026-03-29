use anyhow::{Context, Result};
use audit_verifier::audit_log::AuditLog;
use audit_verifier::audit_signer::SignedAuditLog;
use audit_verifier::signing;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sign-audit-log")]
#[command(version)]
#[command(about = "Sign an audit log file with a private key")]
struct Cli {
    /// Path to the audit log JSON file
    #[arg(short, long, value_name = "PATH")]
    log: PathBuf,

    /// Path to P-521 private key PEM file (optional, generates new key if not provided)
    #[arg(short = 'k', long, value_name = "PATH")]
    private_key: Option<PathBuf>,

    /// Path to save generated private key (only used when no key is provided)
    #[arg(long, value_name = "PATH")]
    save_key: Option<PathBuf>,

    /// Key identifier to include in signature
    #[arg(long, value_name = "ID", default_value = "audit-signer")]
    key_id: String,

    /// Output file for signed audit log (JSON format)
    #[arg(short, long, value_name = "PATH")]
    output: PathBuf,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=debug)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    if !cli.log.exists() {
        anyhow::bail!("Audit log file does not exist: {}", cli.log.display());
    }

    log::info!("Loading audit log from: {}", cli.log.display());
    let audit_log = AuditLog::load_from_file(&cli.log)?;
    log::info!("Loaded audit log with {} entries", audit_log.entries.len());

    let private_key = if let Some(key_path) = &cli.private_key {
        log::info!("Loading private key from: {}", key_path.display());
        signing::load_private_key(key_path).context("Failed to load private key")?
    } else {
        log::info!("Generating new P-521 private key...");
        let key = signing::generate_private_key();

        if let Some(save_path) = &cli.save_key {
            log::info!("Saving private key to: {}", save_path.display());
            signing::save_private_key(&key, save_path).context("Failed to save private key")?;
        }

        key
    };

    log::info!("Signing audit log...");
    let signed_log = SignedAuditLog::sign_log(audit_log, &private_key, cli.key_id)?;

    log::info!("Saving signed audit log to: {}", cli.output.display());
    signed_log.save_to_file(&cli.output)?;

    log::info!("✓ Audit log signed successfully");
    log::info!("  Log hash: {}", signed_log.log_hash);
    log::info!("  Signature: {}...", &signed_log.signature[..16]);
    log::info!("  Signed at: {}", signed_log.signed_at);

    Ok(())
}
