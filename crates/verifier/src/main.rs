use anyhow::{Context, Result};
use clap::Parser;
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use testcase_models::TestCase;
use testcase_storage::TestCaseStorage;
use testcase_verification::{
    BatchVerificationReport, ContainerReport, ContainerReportConfig, MatchStrategy,
    StepVerificationResultEnum, TestCaseVerificationResult, TestExecutionLog, TestVerifier,
};

/// Storage-aware TestVerifier that wraps the core TestVerifier from testcase-verification
/// and provides storage integration
struct StorageTestVerifier {
    storage: TestCaseStorage,
    verifier: TestVerifier,
}

impl StorageTestVerifier {
    /// Create a new verifier with storage and custom match strategies
    fn with_strategies(
        storage: TestCaseStorage,
        result_strategy: MatchStrategy,
        output_strategy: MatchStrategy,
    ) -> Self {
        Self {
            storage,
            verifier: TestVerifier::new(result_strategy, output_strategy),
        }
    }

    /// Get a reference to the test case storage
    fn storage(&self) -> &TestCaseStorage {
        &self.storage
    }

    /// Generate a container report from multiple batch reports
    fn generate_report(
        &self,
        reports: &[BatchVerificationReport],
        format: &str,
        config: ContainerReportConfig,
    ) -> Result<String> {
        self.verifier.generate_report(reports, format, config)
    }

    /// Parse a test execution log file
    fn parse_log_file<P: AsRef<Path>>(&self, log_path: P) -> Result<Vec<TestExecutionLog>> {
        self.verifier.parse_log_file(log_path)
    }

    /// Parse a test execution log file with a specified test case ID
    fn parse_log_file_with_test_case_id<P: AsRef<Path>>(
        &self,
        log_path: P,
        test_case_id: &str,
    ) -> Result<Vec<TestExecutionLog>> {
        self.verifier
            .parse_log_file_with_test_case_id(log_path, test_case_id)
    }

    /// Verify a single test case against execution logs
    fn verify_test_case(
        &self,
        test_case: &TestCase,
        execution_logs: &[TestExecutionLog],
    ) -> TestCaseVerificationResult {
        self.verifier.verify_test_case(test_case, execution_logs)
    }
}

/// Configuration for the verifier report output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct VerifierConfig {
    /// Report title
    title: String,

    /// Project name
    project: String,

    /// Environment information
    environment: Option<String>,

    /// Platform information
    platform: Option<String>,

    /// Executor information
    executor: Option<String>,
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            title: "Test Execution Results".to_string(),
            project: "Test Case Manager - Verification Results".to_string(),
            environment: None,
            platform: None,
            executor: None,
        }
    }
}

impl VerifierConfig {
    /// Load configuration from a YAML file
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content =
            fs::read_to_string(path.as_ref()).context("Failed to read configuration file")?;
        let config: VerifierConfig =
            serde_yaml::from_str(&content).context("Failed to parse configuration file")?;
        Ok(config)
    }

    /// Apply CLI overrides to the configuration
    fn apply_cli_overrides(
        &mut self,
        title: Option<String>,
        project: Option<String>,
        environment: Option<String>,
        platform: Option<String>,
        executor: Option<String>,
    ) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(p) = project {
            self.project = p;
        }
        if environment.is_some() {
            self.environment = environment;
        }
        if platform.is_some() {
            self.platform = platform;
        }
        if executor.is_some() {
            self.executor = executor;
        }
    }
}

#[derive(Parser)]
#[command(name = "verifier")]
#[command(version)]
#[command(about = "Verify test execution logs against test case definitions")]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
struct Cli {
    /// Single-file mode: path to log file
    #[arg(short, long, value_name = "PATH")]
    log: Option<PathBuf>,

    /// Single-file mode: test case ID to verify against
    #[arg(short = 'c', long = "test-case", value_name = "ID")]
    test_case_id: Option<String>,

    /// Folder discovery mode: path to folder containing log files
    #[arg(short = 'f', long, value_name = "PATH")]
    folder: Option<PathBuf>,

    /// Output format (yaml or json)
    #[arg(short = 'F', long, default_value = "yaml", value_name = "FORMAT")]
    format: String,

    /// Output file path (optional, defaults to stdout)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Path to test case storage directory
    #[arg(short = 'd', long, default_value = "testcases", value_name = "DIR")]
    test_case_dir: PathBuf,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=debug)
    #[arg(short, long)]
    verbose: bool,

    /// Path to configuration file (YAML format)
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Report title (overrides config file)
    #[arg(long)]
    title: Option<String>,

    /// Project name (overrides config file)
    #[arg(long)]
    project: Option<String>,

    /// Environment information (overrides config file)
    #[arg(long)]
    environment: Option<String>,

    /// Platform information (overrides config file)
    #[arg(long)]
    platform: Option<String>,

    /// Executor information (overrides config file)
    #[arg(long)]
    executor: Option<String>,

    /// Match strategy for verification (exact, regex, contains, or precomputed)
    #[arg(
        short = 'm',
        long = "match-strategy",
        default_value = "exact",
        value_name = "STRATEGY"
    )]
    match_strategy: String,

    /// Path to JSON schema file for validating output
    #[arg(
        long = "schema",
        default_value = "data/testcase_results_container/schema.json",
        value_name = "PATH"
    )]
    schema_path: PathBuf,

    /// Exit with 0 even if test cases failed (useful for acceptance testing)
    #[arg(long = "success-on-completion")]
    success_on_completion: bool,

    /// Verify source YAML hash from execution logs
    #[arg(long = "verify-source-hash")]
    verify_source_hash: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Validate CLI arguments
    let (mode, log_path, test_case_id, folder_path) = validate_args(&cli)?;

    // Parse match strategy
    let match_strategy = parse_match_strategy(&cli.match_strategy)?;

    // Load configuration
    let config = load_configuration(&cli)?;

    // Initialize storage and verifier
    let storage = TestCaseStorage::new(&cli.test_case_dir)
        .context("Failed to initialize test case storage")?;
    let verifier = StorageTestVerifier::with_strategies(storage, match_strategy, match_strategy);

    // Execute appropriate mode
    let report = match mode {
        Mode::SingleFile => {
            // Safe unwraps: validate_args guarantees these are Some when mode is SingleFile
            let log_path = log_path.expect("log_path must be Some for SingleFile mode");
            let test_case_id = test_case_id.expect("test_case_id must be Some for SingleFile mode");
            handle_single_file_mode(&verifier, &log_path, &test_case_id, cli.verify_source_hash)?
        }
        Mode::FolderDiscovery => {
            // Safe unwrap: validate_args guarantees this is Some when mode is FolderDiscovery
            let folder_path =
                folder_path.expect("folder_path must be Some for FolderDiscovery mode");
            handle_folder_mode(&verifier, &folder_path, cli.verify_source_hash)?
        }
    };

    // Log summary of failures
    log_verification_errors(&report);

    // Generate output in requested format
    let output_content =
        generate_output(&verifier, &report, &cli.format, &config, &cli.schema_path)?;

    // Write to file or stdout
    write_output(&output_content, cli.output.as_ref())?;

    // Exit with non-zero code if any tests failed (unless --success-on-completion is set)
    if !cli.success_on_completion && report.failed_test_cases > 0 {
        std::process::exit(1);
    }

    Ok(())
}

#[derive(Debug)]
enum Mode {
    SingleFile,
    FolderDiscovery,
}

type ValidationResult = (Mode, Option<PathBuf>, Option<String>, Option<PathBuf>);

fn validate_args(cli: &Cli) -> Result<ValidationResult> {
    // Check for single-file mode
    let single_file_mode = cli.log.is_some() && cli.test_case_id.is_some();
    // Check for folder discovery mode
    let folder_mode = cli.folder.is_some();

    // Validate mutually exclusive modes
    if !single_file_mode && !folder_mode {
        anyhow::bail!("Must specify either single-file mode (--log and --test-case) or folder discovery mode (--folder)");
    }

    if single_file_mode && folder_mode {
        anyhow::bail!("Cannot use both single-file mode (--log/--test-case) and folder mode (--folder) simultaneously");
    }

    // Validate format
    let format = cli.format.to_lowercase();
    if format != "yaml" && format != "json" {
        anyhow::bail!("Format must be 'yaml' or 'json', got: {}", format);
    }

    if single_file_mode {
        // Safe to unwrap: single_file_mode is true only if both log and test_case_id are Some
        let log_path = cli
            .log
            .as_ref()
            .expect("log should be Some in single-file mode");
        let test_case_id = cli
            .test_case_id
            .as_ref()
            .expect("test_case_id should be Some in single-file mode");

        if !log_path.exists() {
            anyhow::bail!("Log file does not exist: {}", log_path.display());
        }

        Ok((
            Mode::SingleFile,
            Some(log_path.clone()),
            Some(test_case_id.clone()),
            None,
        ))
    } else {
        // Safe to unwrap: folder_mode is true only if folder is Some
        let folder_path = cli
            .folder
            .as_ref()
            .expect("folder should be Some in folder mode");

        if !folder_path.exists() {
            anyhow::bail!("Folder does not exist: {}", folder_path.display());
        }

        if !folder_path.is_dir() {
            anyhow::bail!("Path is not a directory: {}", folder_path.display());
        }

        Ok((Mode::FolderDiscovery, None, None, Some(folder_path.clone())))
    }
}

fn handle_single_file_mode(
    verifier: &StorageTestVerifier,
    log_path: &PathBuf,
    test_case_id: &str,
    verify_source_hash: bool,
) -> Result<BatchVerificationReport> {
    log::info!("Processing log file: {}", log_path.display());
    log::debug!("Single-file mode: test case ID = '{}'", test_case_id);

    // Parse log file with specified test case ID
    log::debug!("Parsing log file: {}", log_path.display());
    let logs = verifier
        .parse_log_file_with_test_case_id(log_path, test_case_id)
        .context("Failed to parse test execution log")?;

    log::debug!(
        "Successfully parsed log file with {} log entries",
        logs.len()
    );

    // Load test case
    log::debug!("Loading test case definition for ID: '{}'", test_case_id);
    let test_case = verifier
        .storage()
        .load_test_case_by_id(test_case_id)
        .context(format!("Failed to load test case: {}", test_case_id))?;

    log::debug!(
        "Successfully loaded test case '{}' with {} test sequences",
        test_case_id,
        test_case.test_sequences.len()
    );

    // Compute source hash and verify if requested
    let source_hash = compute_test_case_source_hash(verifier.storage(), test_case_id)?;
    if verify_source_hash {
        verify_hash_against_logs(&logs, test_case_id, &source_hash)?;
    }

    // Verify
    log::debug!("Verifying test case '{}' against logs", test_case_id);
    let mut result = verifier.verify_test_case(&test_case, &logs);
    result.source_yaml_sha256 = Some(source_hash);

    log::debug!(
        "Verification result: pass={}, steps={}/{}",
        result.overall_pass,
        result.passed_steps,
        result.total_steps
    );

    // Create batch report with single test case
    log::debug!("Creating batch report with single test case result");
    let mut report = BatchVerificationReport::new();
    report.add_test_case_result(result);

    log::debug!(
        "Batch report created: total={}, passed={}, failed={}",
        report.total_test_cases,
        report.passed_test_cases,
        report.failed_test_cases
    );

    Ok(report)
}

fn handle_folder_mode(
    verifier: &StorageTestVerifier,
    folder_path: &PathBuf,
    verify_source_hash: bool,
) -> Result<BatchVerificationReport> {
    log::debug!(
        "Starting folder discovery mode for: {}",
        folder_path.display()
    );

    // Discover all log files in the folder recursively
    let log_files = discover_log_files(folder_path)?;

    if log_files.is_empty() {
        log::warn!(
            "No execution log files (*_execution_log.json) found in folder: {}",
            folder_path.display()
        );
        log::debug!("Folder discovery completed with no files found");
        return Ok(BatchVerificationReport::new());
    }

    log::info!(
        "Found {} execution log file(s) in {}",
        log_files.len(),
        folder_path.display()
    );

    log::debug!("Discovered log files:");
    for log_file in &log_files {
        log::debug!("  - {}", log_file.display());
    }

    // Process each log file individually for better logging
    let mut report = BatchVerificationReport::new();
    log::debug!("Initialized empty batch verification report");

    for (idx, log_file) in log_files.iter().enumerate() {
        log::info!(
            "Processing log file {}/{}: {}",
            idx + 1,
            log_files.len(),
            log_file.display()
        );

        // Extract test case ID from filename
        let test_case_id = extract_test_case_id_from_filename(log_file);
        log::debug!("Extracted test case ID: '{}' from filename", test_case_id);

        log::debug!("Parsing log file: {}", log_file.display());
        match verifier.parse_log_file(log_file) {
            Ok(logs) => {
                log::debug!(
                    "Successfully parsed log file with {} log entries",
                    logs.len()
                );

                // Load test case
                log::debug!("Loading test case definition for ID: '{}'", test_case_id);
                match verifier.storage().load_test_case_by_id(&test_case_id) {
                    Ok(test_case) => {
                        log::debug!(
                            "Successfully loaded test case '{}' with {} test sequences",
                            test_case_id,
                            test_case.test_sequences.len()
                        );

                        // Compute source hash and verify if requested
                        let source_hash_result = compute_test_case_source_hash(verifier.storage(), &test_case_id);
                        let source_hash = match source_hash_result {
                            Ok(hash) => {
                                if verify_source_hash {
                                    if let Err(e) = verify_hash_against_logs(&logs, &test_case_id, &hash) {
                                        log::error!(
                                            "Source hash verification failed for test case '{}': {}",
                                            test_case_id,
                                            e
                                        );
                                        continue;
                                    }
                                }
                                Some(hash)
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to compute source hash for test case '{}': {}",
                                    test_case_id,
                                    e
                                );
                                None
                            }
                        };

                        log::debug!("Verifying test case '{}' against logs", test_case_id);

                        let mut result = verifier.verify_test_case(&test_case, &logs);
                        result.source_yaml_sha256 = source_hash;

                        log::debug!(
                            "Verification result for '{}': pass={}, steps={}/{}",
                            test_case_id,
                            result.overall_pass,
                            result.passed_steps,
                            result.total_steps
                        );

                        report.add_test_case_result(result);
                        log::debug!("Added test case result to batch report. Current report stats: total={}, passed={}, failed={}",
                                   report.total_test_cases, report.passed_test_cases, report.failed_test_cases);
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to load test case '{}' for log file '{}': {}",
                            test_case_id,
                            log_file.display(),
                            e
                        );
                        log::debug!(
                            "Test case loading failed, skipping verification for this log file"
                        );
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to parse log file '{}': {}", log_file.display(), e);
                log::debug!("Log file parsing failed, skipping this file");
            }
        }
    }

    log::debug!("Folder mode processing complete. Final batch report stats:");
    log::debug!("  Total test cases: {}", report.total_test_cases);
    log::debug!("  Passed test cases: {}", report.passed_test_cases);
    log::debug!("  Failed test cases: {}", report.failed_test_cases);
    log::debug!("  Total steps: {}", report.total_steps);
    log::debug!("  Passed steps: {}", report.passed_steps);
    log::debug!("  Failed steps: {}", report.failed_steps);

    Ok(report)
}

fn discover_log_files(folder_path: &PathBuf) -> Result<Vec<PathBuf>> {
    log::debug!(
        "Beginning recursive log file discovery in: {}",
        folder_path.display()
    );
    let mut log_files = Vec::new();
    discover_log_files_recursive(folder_path, &mut log_files)?;
    log::debug!(
        "Log file discovery complete. Found {} file(s)",
        log_files.len()
    );
    Ok(log_files)
}

fn discover_log_files_recursive(dir: &PathBuf, log_files: &mut Vec<PathBuf>) -> Result<()> {
    log::debug!("Scanning directory: {}", dir.display());

    let entries =
        fs::read_dir(dir).context(format!("Failed to read directory: {}", dir.display()))?;

    let mut file_count = 0;
    let mut dir_count = 0;
    let mut skipped_count = 0;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        // Get metadata to check file type without following symlinks
        let metadata = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Failed to read metadata for '{}': {}", path.display(), e);
                log::debug!("Skipping file due to metadata read error");
                skipped_count += 1;
                continue;
            }
        };

        // Skip symlinks to avoid potential infinite loops
        if metadata.is_symlink() {
            log::debug!("Skipping symlink: {}", path.display());
            skipped_count += 1;
            continue;
        }

        if metadata.is_dir() {
            log::debug!("Found subdirectory: {}", path.display());
            dir_count += 1;
            // Recursively search subdirectories
            discover_log_files_recursive(&path, log_files)?;
        } else if metadata.is_file() {
            file_count += 1;
            // Check if filename matches the pattern *_execution_log.json
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.ends_with("_execution_log.json") {
                    log::debug!("Found matching log file: {}", path.display());
                    log_files.push(path);
                } else {
                    log::debug!("Skipping non-matching file: {}", path.display());
                }
            }
        }
    }

    log::debug!(
        "Directory scan complete for {}: {} files, {} subdirs, {} skipped",
        dir.display(),
        file_count,
        dir_count,
        skipped_count
    );

    Ok(())
}

fn extract_test_case_id_from_filename(log_path: &Path) -> String {
    // Expected format: {test_case_id}_execution_log.json
    let test_case_id = log_path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_suffix("_execution_log"))
        .unwrap_or("UNKNOWN")
        .to_string();

    log::debug!(
        "Extracting test case ID from filename '{}': '{}'",
        log_path.display(),
        test_case_id
    );

    test_case_id
}

fn log_verification_errors(report: &BatchVerificationReport) {
    for test_case in &report.test_cases {
        if !test_case.overall_pass {
            log::error!(
                "Test case '{}' FAILED ({}/{} steps passed)",
                test_case.test_case_id,
                test_case.passed_steps,
                test_case.total_steps
            );

            for sequence in &test_case.sequences {
                if !sequence.all_steps_passed {
                    log::error!(
                        "  Sequence {} '{}' FAILED",
                        sequence.sequence_id,
                        sequence.name
                    );

                    for step_result in &sequence.step_results {
                        if !step_result.is_pass() {
                            match step_result {
                                StepVerificationResultEnum::Fail {
                                    step,
                                    description,
                                    reason,
                                    ..
                                } => {
                                    log::error!(
                                        "    Step {} '{}' FAILED: {}",
                                        step,
                                        description,
                                        reason
                                    );
                                }
                                StepVerificationResultEnum::NotExecuted {
                                    step,
                                    description,
                                    ..
                                } => {
                                    log::error!("    Step {} '{}' NOT EXECUTED", step, description);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn load_configuration(cli: &Cli) -> Result<VerifierConfig> {
    // Start with defaults or load from config file if specified
    let mut config = if let Some(config_path) = &cli.config {
        log::info!("Loading configuration from: {}", config_path.display());
        VerifierConfig::load_from_file(config_path)?
    } else {
        VerifierConfig::default()
    };

    // Apply CLI overrides
    config.apply_cli_overrides(
        cli.title.clone(),
        cli.project.clone(),
        cli.environment.clone(),
        cli.platform.clone(),
        cli.executor.clone(),
    );

    log::debug!(
        "Final configuration: title='{}', project='{}'",
        config.title,
        config.project
    );

    Ok(config)
}

fn generate_output(
    verifier: &StorageTestVerifier,
    report: &BatchVerificationReport,
    format: &str,
    config: &VerifierConfig,
    schema_path: &PathBuf,
) -> Result<String> {
    // Always use container format with metadata
    let container_config = ContainerReportConfig {
        title: config.title.clone(),
        project: config.project.clone(),
        environment: config.environment.clone(),
        platform: config.platform.clone(),
        executor: config.executor.clone(),
    };
    let output = verifier
        .generate_report(std::slice::from_ref(report), format, container_config)
        .context("Failed to generate container report")?;

    // Validate the output against the schema
    validate_output_against_schema(&output, format, schema_path)?;

    Ok(output)
}

/// Validate generated output against the JSON schema
fn validate_output_against_schema(output: &str, format: &str, schema_path: &PathBuf) -> Result<()> {
    // Check if schema file exists
    if !schema_path.exists() {
        anyhow::bail!(
            "Schema file not found at {}. Please ensure the schema file exists or specify a valid path using --schema",
            schema_path.display()
        );
    }

    log::debug!("Loading schema from {}", schema_path.display());
    let schema_content = fs::read_to_string(schema_path).context(format!(
        "Failed to read schema file: {}",
        schema_path.display()
    ))?;

    let schema_json: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse schema file as JSON")?;

    // Compile the schema
    let compiled_schema = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

    // Parse the output based on format
    log::debug!("Validating {} output against schema", format);
    let output_json: serde_json::Value = match format.to_lowercase().as_str() {
        "yaml" => {
            // Parse YAML into ContainerReport, then serialize to JSON for validation
            // This ensures proper handling of externally tagged enums
            let container: ContainerReport = serde_yaml::from_str(output)
                .context("Failed to parse YAML output for validation")?;
            serde_json::to_value(&container)
                .context("Failed to convert YAML to JSON for validation")?
        }
        "json" => {
            // Parse JSON directly
            serde_json::from_str(output).context("Failed to parse JSON output for validation")?
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported format for validation: {}",
                format
            ));
        }
    };

    // Validate against schema
    let validation_result = compiled_schema.validate(&output_json);

    match validation_result {
        Ok(_) => {
            log::info!("✓ Output validation passed: conforms to schema");
            Ok(())
        }
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .map(|e| format!("  - {} at {}", e, e.instance_path))
                .collect();

            log::error!("✗ Output validation failed:");
            for msg in &error_messages {
                log::error!("{}", msg);
            }

            Err(anyhow::anyhow!(
                "Output does not conform to schema:\n{}",
                error_messages.join("\n")
            ))
        }
    }
}

fn write_output(content: &str, output_path: Option<&PathBuf>) -> Result<()> {
    if let Some(path) = output_path {
        fs::write(path, content)
            .context(format!("Failed to write output to {}", path.display()))?;
        log::info!("Report written to {}", path.display());
    } else {
        println!("{}", content);
    }

    Ok(())
}
fn parse_match_strategy(strategy: &str) -> Result<MatchStrategy> {
    match strategy.to_lowercase().as_str() {
        "exact" => Ok(MatchStrategy::Exact),
        "regex" => Ok(MatchStrategy::Regex),
        "contains" => Ok(MatchStrategy::Contains),
        "precomputed" => Ok(MatchStrategy::Precomputed),
        "bash-evaluation" | "bash" => Ok(MatchStrategy::BashEvaluation),
        _ => anyhow::bail!(
            "Invalid match strategy '{}'. Must be one of: exact, regex, contains, precomputed, bash-evaluation",
            strategy
        ),
    }
}

fn compute_test_case_source_hash(storage: &TestCaseStorage, test_case_id: &str) -> Result<String> {
    let yaml_path = find_test_case_file_path(storage, test_case_id)
        .context(format!("Failed to find test case file for ID: {}", test_case_id))?;
    
    log::debug!("Computing SHA-256 hash for test case file: {}", yaml_path.display());
    
    let file_bytes = fs::read(&yaml_path)
        .context(format!("Failed to read test case file: {}", yaml_path.display()))?;
    
    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);
    
    log::debug!("Computed SHA-256 hash for '{}': {}", test_case_id, hash_hex);
    
    Ok(hash_hex)
}

fn find_test_case_file_path(storage: &TestCaseStorage, test_case_id: &str) -> Result<PathBuf> {
    const YAML_EXTENSIONS: &[&str] = &["yaml", "yml"];
    
    let base_path = storage.base_path();
    let id_path = Path::new(test_case_id);
    
    if let Some(ext) = id_path.extension() {
        if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
            let file_path = base_path.join(test_case_id);
            if file_path.exists() {
                return Ok(file_path);
            }
            if id_path.exists() {
                return Ok(id_path.to_path_buf());
            }
        }
    }
    
    for ext in YAML_EXTENSIONS {
        let file_name = format!("{}.{}", test_case_id, ext);
        let file_path = base_path.join(&file_name);
        
        if file_path.exists() {
            return Ok(file_path);
        }
    }
    
    for ext in YAML_EXTENSIONS {
        let file_name = format!("{}.{}", test_case_id, ext);
        if let Ok(entries) = fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(found) = search_recursive(&path, &file_name) {
                        return Ok(found);
                    }
                }
            }
        }
    }
    
    anyhow::bail!("Test case file not found for ID: {}", test_case_id)
}

fn search_recursive(dir: &Path, file_name: &str) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.file_name()?.to_str()? == file_name {
                return Some(path);
            } else if path.is_dir() {
                if let Some(found) = search_recursive(&path, file_name) {
                    return Some(found);
                }
            }
        }
    }
    None
}

fn verify_hash_against_logs(
    logs: &[TestExecutionLog],
    test_case_id: &str,
    expected_hash: &str,
) -> Result<()> {
    log::debug!(
        "Verifying source hash for test case '{}' against {} log entries",
        test_case_id,
        logs.len()
    );

    let mut hash_map: HashMap<String, usize> = HashMap::new();
    let mut missing_count = 0;

    for log in logs {
        if let Some(log_hash) = &log.source_yaml_sha256 {
            *hash_map.entry(log_hash.clone()).or_insert(0) += 1;
        } else {
            missing_count += 1;
        }
    }

    if missing_count > 0 {
        anyhow::bail!(
            "Source hash verification failed for test case '{}': {} log entries are missing source_yaml_sha256 field",
            test_case_id,
            missing_count
        );
    }

    for (log_hash, count) in &hash_map {
        if log_hash != expected_hash {
            anyhow::bail!(
                "Source hash verification failed for test case '{}': expected hash '{}' but {} log entries have hash '{}'",
                test_case_id,
                expected_hash,
                count,
                log_hash
            );
        }
    }

    log::info!(
        "✓ Source hash verification passed for test case '{}': all {} log entries match expected hash",
        test_case_id,
        logs.len()
    );

    Ok(())
}
