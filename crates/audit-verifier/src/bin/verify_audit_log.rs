use anyhow::{Context, Result};
use audit_verifier::audit_signer::{SignedAuditLog, SignatureVerificationReport};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "verify-audit-log")]
#[command(version)]
#[command(about = "Verify a signed audit log file")]
struct Cli {
    /// Path to the signed audit log JSON file
    #[arg(value_name = "SIGNED_LOG")]
    signed_log: PathBuf,

    /// Optional output file for verification report (JSON format)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=debug)
    #[arg(short, long)]
    verbose: bool,

    /// Show detailed information about each log entry
    #[arg(short = 'd', long)]
    detailed: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    if !cli.signed_log.exists() {
        anyhow::bail!("Signed audit log file does not exist: {}", cli.signed_log.display());
    }

    log::info!("Loading signed audit log from: {}", cli.signed_log.display());
    let signed_log = SignedAuditLog::load_from_file(&cli.signed_log)?;

    log::info!("Verifying signature...");
    let report = SignatureVerificationReport::verify(&signed_log);

    println!("=== Audit Log Verification Report ===");
    println!();
    println!("Verification Status: {}", if report.is_valid { "✓ VALID" } else { "✗ INVALID" });
    println!("Log Hash Verified:   {}", if report.log_hash_verified { "✓" } else { "✗" });
    println!("Signature Verified:  {}", if report.signature_verified { "✓" } else { "✗" });
    println!();
    println!("Key ID:        {}", report.key_id);
    println!("Signed At:     {}", report.signed_at);
    println!("Verified At:   {}", report.verified_at);
    println!("Total Entries: {}", report.total_entries);
    println!();

    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
        println!();
    }

    if cli.detailed {
        println!("=== Audit Log Details ===");
        println!();
        println!("Version:      {}", signed_log.audit_log.version);
        println!("Created At:   {}", signed_log.audit_log.created_at);
        println!("Last Updated: {}", signed_log.audit_log.last_updated);
        println!();
        println!("Entries:");
        for (i, entry) in signed_log.audit_log.entries.iter().enumerate() {
            println!("  {}. [{:?}] {:?}", i + 1, entry.status, entry.operation);
            println!("     Timestamp: {}", entry.timestamp);
            if let Some(user) = &entry.user {
                println!("     User: {}", user);
            }
            if let Some(hostname) = &entry.hostname {
                println!("     Host: {}", hostname);
            }
            if !entry.input_files.is_empty() {
                println!("     Input files: {}", entry.input_files.len());
            }
            if !entry.output_files.is_empty() {
                println!("     Output files: {}", entry.output_files.len());
            }
            if let Some(duration) = entry.duration_ms {
                println!("     Duration: {}ms", duration);
            }
            if let Some(error) = &entry.error_message {
                println!("     Error: {}", error);
            }
            println!();
        }
    }

    if let Some(output_path) = &cli.output {
        let report_json = serde_json::to_string_pretty(&report)
            .context("Failed to serialize verification report")?;
        fs::write(output_path, report_json)
            .context(format!("Failed to write report to: {}", output_path.display()))?;
        log::info!("Verification report written to: {}", output_path.display());
    }

    if report.is_valid {
        log::info!("✓ Audit log verification successful");
        std::process::exit(0);
    } else {
        log::error!("✗ Audit log verification failed");
        std::process::exit(1);
    }
}
