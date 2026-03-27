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
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
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

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=debug)
    #[arg(short, long)]
    verbose: bool,
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

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

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

    let computed_hash = compute_yaml_sha256(&yaml_bytes);

    log::info!("Computed SHA-256 hash of YAML file: {}", computed_hash);

    // Parse execution log JSON
    let log_content = fs::read_to_string(&cli.log)
        .context(format!("Failed to read log file: {}", cli.log.display()))?;

    let log_entries = parse_execution_log(&log_content)?;

    if log_entries.is_empty() {
        log::warn!("Execution log is empty");
        std::process::exit(1);
    }

    // Extract all source_yaml_sha256 fields and compare
    let result = verify_hashes(&computed_hash, &log_entries);

    // Report individual errors
    for (index, entry) in log_entries.iter().enumerate() {
        match &entry.source_yaml_sha256 {
            Some(hash) => {
                if hash != &computed_hash {
                    log::error!(
                        "Hash mismatch at entry {}: expected '{}', got '{}'",
                        index + 1,
                        computed_hash,
                        hash
                    );
                }
            }
            None => {
                log::warn!("Missing source_yaml_sha256 field at entry {}", index + 1);
            }
        }
    }

    // Print summary
    log::info!("Verification Summary:");
    log::info!("  Total entries: {}", log_entries.len());
    log::info!("  Hash mismatches: {}", result.mismatch_count);
    log::info!("  Missing hash fields: {}", result.missing_hash_count);

    if result.all_match {
        log::info!("✓ All hashes match and no missing hash fields");
    } else {
        log::error!("✗ Verification failed");
    }

    // Compute SHA-256 hash of the execution log content
    let mut log_hasher = Sha256::new();
    log_hasher.update(log_content.as_bytes());
    let execution_log_hash = format!("{:x}", log_hasher.finalize());

    // Create verification result
    let verification_result = VerificationResult {
        computed_hash: computed_hash.clone(),
        total_entries: log_entries.len(),
        hash_mismatches: result.mismatch_count,
        missing_hash_fields: result.missing_hash_count,
        verification_passed: result.all_match,
    };

    // Load or generate private key
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
        log::info!(
            "✓ Signed audit verification written to: {}",
            output_path.display()
        );
    } else {
        log::info!("--- Signed Audit Verification Output ---");
        log::info!("{}", output_json);
    }

    if result.all_match {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn compute_yaml_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

fn parse_execution_log(json_content: &str) -> Result<Vec<ExecutionLogEntry>> {
    serde_json::from_str(json_content).context("Failed to parse execution log JSON")
}

#[derive(Debug, PartialEq)]
struct HashVerificationResult {
    all_match: bool,
    mismatch_count: usize,
    missing_hash_count: usize,
}

fn verify_hashes(computed_hash: &str, log_entries: &[ExecutionLogEntry]) -> HashVerificationResult {
    let mut all_match = true;
    let mut missing_hash_count = 0;
    let mut mismatch_count = 0;

    for entry in log_entries {
        match &entry.source_yaml_sha256 {
            Some(hash) => {
                if hash != computed_hash {
                    all_match = false;
                    mismatch_count += 1;
                }
            }
            None => {
                all_match = false;
                missing_hash_count += 1;
            }
        }
    }

    HashVerificationResult {
        all_match,
        mismatch_count,
        missing_hash_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_yaml_sha256_empty_input() {
        let input = b"";
        let hash = compute_yaml_sha256(input);
        // SHA-256 of empty string
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_compute_yaml_sha256_simple_text() {
        let input = b"hello world";
        let hash = compute_yaml_sha256(input);
        // SHA-256 of "hello world"
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_compute_yaml_sha256_yaml_content() {
        let input = b"name: test\nsteps:\n  - command: echo hello\n";
        let hash = compute_yaml_sha256(input);
        // SHA-256 of the YAML content
        assert_eq!(
            hash,
            "51fe73076eb63f0cc4a0b1e98d81c8fbd4413e081943d4d9fc4253b6b61a9121"
        );
    }

    #[test]
    fn test_compute_yaml_sha256_known_hash() {
        // Test with a known hash value
        let input = b"test";
        let hash = compute_yaml_sha256(input);
        // SHA-256 of "test"
        assert_eq!(
            hash,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn test_compute_yaml_sha256_deterministic() {
        let input = b"deterministic test";
        let hash1 = compute_yaml_sha256(input);
        let hash2 = compute_yaml_sha256(input);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_parse_execution_log_empty_array() {
        let json = "[]";
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_parse_execution_log_single_entry_with_hash() {
        let json = r#"[{"source_yaml_sha256": "abc123"}]"#;
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_yaml_sha256.as_deref(), Some("abc123"));
    }

    #[test]
    fn test_parse_execution_log_single_entry_without_hash() {
        let json = r#"[{"other_field": "value"}]"#;
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_yaml_sha256, None);
    }

    #[test]
    fn test_parse_execution_log_multiple_entries_all_with_hash() {
        let json = r#"[
            {"source_yaml_sha256": "hash1"},
            {"source_yaml_sha256": "hash2"},
            {"source_yaml_sha256": "hash3"}
        ]"#;
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].source_yaml_sha256.as_deref(), Some("hash1"));
        assert_eq!(entries[1].source_yaml_sha256.as_deref(), Some("hash2"));
        assert_eq!(entries[2].source_yaml_sha256.as_deref(), Some("hash3"));
    }

    #[test]
    fn test_parse_execution_log_multiple_entries_mixed() {
        let json = r#"[
            {"source_yaml_sha256": "hash1"},
            {"other_field": "value"},
            {"source_yaml_sha256": "hash3"}
        ]"#;
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].source_yaml_sha256.as_deref(), Some("hash1"));
        assert_eq!(entries[1].source_yaml_sha256, None);
        assert_eq!(entries[2].source_yaml_sha256.as_deref(), Some("hash3"));
    }

    #[test]
    fn test_parse_execution_log_null_hash() {
        let json = r#"[{"source_yaml_sha256": null}]"#;
        let result = parse_execution_log(json);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_yaml_sha256, None);
    }

    #[test]
    fn test_parse_execution_log_invalid_json() {
        let json = "not valid json";
        let result = parse_execution_log(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_execution_log_non_array_json() {
        let json = r#"{"source_yaml_sha256": "hash"}"#;
        let result = parse_execution_log(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_hashes_all_match() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_verify_hashes_single_mismatch() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("def456".to_string()),
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 1);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_verify_hashes_multiple_mismatches() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: Some("wrong1".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("wrong2".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("wrong3".to_string()),
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 3);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_verify_hashes_missing_hash() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: None,
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 1);
    }

    #[test]
    fn test_verify_hashes_multiple_missing_hashes() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: None,
            },
            ExecutionLogEntry {
                source_yaml_sha256: None,
            },
            ExecutionLogEntry {
                source_yaml_sha256: None,
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 3);
    }

    #[test]
    fn test_verify_hashes_mixed_errors() {
        let entries = vec![
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("wrong".to_string()),
            },
            ExecutionLogEntry {
                source_yaml_sha256: None,
            },
            ExecutionLogEntry {
                source_yaml_sha256: Some("abc123".to_string()),
            },
        ];
        let result = verify_hashes("abc123", &entries);
        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 1);
        assert_eq!(result.missing_hash_count, 1);
    }

    #[test]
    fn test_verify_hashes_empty_entries() {
        let entries: Vec<ExecutionLogEntry> = vec![];
        let result = verify_hashes("abc123", &entries);
        assert!(result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_verify_hashes_empty_computed_hash() {
        let entries = vec![ExecutionLogEntry {
            source_yaml_sha256: Some("".to_string()),
        }];
        let result = verify_hashes("", &entries);
        assert!(result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_integration_all_match() {
        let yaml_content = b"test: data";
        let computed_hash = compute_yaml_sha256(yaml_content);

        let log_json = format!(
            r#"[
                {{"source_yaml_sha256": "{}"}},
                {{"source_yaml_sha256": "{}"}}
            ]"#,
            computed_hash, computed_hash
        );

        let entries = parse_execution_log(&log_json).unwrap();
        let result = verify_hashes(&computed_hash, &entries);

        assert!(result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_integration_with_mismatch() {
        let yaml_content = b"test: data";
        let computed_hash = compute_yaml_sha256(yaml_content);

        let log_json = format!(
            r#"[
                {{"source_yaml_sha256": "{}"}},
                {{"source_yaml_sha256": "wrong_hash"}}
            ]"#,
            computed_hash
        );

        let entries = parse_execution_log(&log_json).unwrap();
        let result = verify_hashes(&computed_hash, &entries);

        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 1);
        assert_eq!(result.missing_hash_count, 0);
    }

    #[test]
    fn test_integration_with_missing_hash() {
        let yaml_content = b"test: data";
        let computed_hash = compute_yaml_sha256(yaml_content);

        let log_json = format!(
            r#"[
                {{"source_yaml_sha256": "{}"}},
                {{"other_field": "value"}}
            ]"#,
            computed_hash
        );

        let entries = parse_execution_log(&log_json).unwrap();
        let result = verify_hashes(&computed_hash, &entries);

        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 0);
        assert_eq!(result.missing_hash_count, 1);
    }

    #[test]
    fn test_integration_mixed_scenarios() {
        let yaml_content = b"name: test\nvalue: 123";
        let computed_hash = compute_yaml_sha256(yaml_content);

        let log_json = format!(
            r#"[
                {{"source_yaml_sha256": "{}"}},
                {{"source_yaml_sha256": "mismatch1"}},
                {{"other": "field"}},
                {{"source_yaml_sha256": "{}"}},
                {{"source_yaml_sha256": "mismatch2"}}
            ]"#,
            computed_hash, computed_hash
        );

        let entries = parse_execution_log(&log_json).unwrap();
        let result = verify_hashes(&computed_hash, &entries);

        assert!(!result.all_match);
        assert_eq!(result.mismatch_count, 2);
        assert_eq!(result.missing_hash_count, 1);
    }
}
