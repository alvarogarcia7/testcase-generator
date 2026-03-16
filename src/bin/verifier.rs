use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use testcase_manager::BatchVerificationReport;
use testcase_manager::MatchStrategy;
use testcase_manager::TestCaseStorage;
use testcase_manager::TestVerifier;
use testcase_manager::ContainerReportConfig;

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

    /// Enable container YAML output format with enhanced metadata
    #[arg(long)]
    container_format: bool,

    /// Report title (used with --container-format)
    #[arg(long, default_value = "Test Execution Results")]
    title: String,

    /// Project name (used with --container-format)
    #[arg(long, default_value = "Test Case Manager - Verification Results")]
    project: String,

    /// Environment information (used with --container-format)
    #[arg(long)]
    environment: Option<String>,

    /// Platform information (used with --container-format)
    #[arg(long)]
    platform: Option<String>,

    /// Executor information (used with --container-format)
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Validate CLI arguments
    let (mode, log_path, test_case_id, folder_path) = validate_args(&cli)?;

    // Parse match strategy
    let match_strategy = parse_match_strategy(&cli.match_strategy)?;

    // Initialize storage and verifier
    let storage = TestCaseStorage::new(&cli.test_case_dir)
        .context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::with_strategies(storage, match_strategy, match_strategy);

    // Execute appropriate mode
    let report = match mode {
        Mode::SingleFile => {
            // Safe unwraps: validate_args guarantees these are Some when mode is SingleFile
            let log_path = log_path.expect("log_path must be Some for SingleFile mode");
            let test_case_id = test_case_id.expect("test_case_id must be Some for SingleFile mode");
            handle_single_file_mode(&verifier, &log_path, &test_case_id)?
        }
        Mode::FolderDiscovery => {
            // Safe unwrap: validate_args guarantees this is Some when mode is FolderDiscovery
            let folder_path =
                folder_path.expect("folder_path must be Some for FolderDiscovery mode");
            handle_folder_mode(&verifier, &folder_path)?
        }
    };

    // Log summary of failures
    log_verification_errors(&report);

    // Generate output in requested format
    let output_config = OutputConfig {
        container_format: cli.container_format,
        title: &cli.title,
        project: &cli.project,
        environment: cli.environment.as_ref(),
        platform: cli.platform.as_ref(),
        executor: cli.executor.as_ref(),
    };
    let output_content = generate_output(&verifier, &report, &cli.format, output_config)?;

    // Write to file or stdout
    write_output(&output_content, cli.output.as_ref())?;

    // Exit with non-zero code if any tests failed
    if report.failed_test_cases > 0 {
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
    verifier: &TestVerifier,
    log_path: &PathBuf,
    test_case_id: &str,
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

    // Verify
    log::debug!("Verifying test case '{}' against logs", test_case_id);
    let result = verifier.verify_test_case(&test_case, &logs);

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
    verifier: &TestVerifier,
    folder_path: &PathBuf,
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
                        log::debug!("Verifying test case '{}' against logs", test_case_id);

                        let result = verifier.verify_test_case(&test_case, &logs);

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
                                testcase_manager::StepVerificationResultEnum::Fail {
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
                                testcase_manager::StepVerificationResultEnum::NotExecuted {
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

struct OutputConfig<'a> {
    container_format: bool,
    title: &'a str,
    project: &'a str,
    environment: Option<&'a String>,
    platform: Option<&'a String>,
    executor: Option<&'a String>,
}

fn generate_output(
    verifier: &TestVerifier,
    report: &BatchVerificationReport,
    format: &str,
    config: OutputConfig,
) -> Result<String> {
    if config.container_format {
        // Use container format with metadata
        let container_config = ContainerReportConfig {
            title: config.title.to_string(),
            project: config.project.to_string(),
            environment: config.environment.cloned(),
            platform: config.platform.cloned(),
            executor: config.executor.cloned(),
        };
        verifier
            .generate_container_yaml_report(std::slice::from_ref(report), format, container_config)
            .context("Failed to generate container report")
    } else {
        // Use simple batch report format
        match format.to_lowercase().as_str() {
            "yaml" => verifier
                .generate_container_report(report, "yaml")
                .context("Failed to generate YAML report"),
            "json" => verifier
                .generate_container_report(report, "json")
                .context("Failed to generate JSON report"),
            _ => anyhow::bail!("Unsupported format: {}", format),
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
        _ => anyhow::bail!(
            "Invalid match strategy '{}'. Must be one of: exact, regex, contains, precomputed",
            strategy
        ),
    }
}
